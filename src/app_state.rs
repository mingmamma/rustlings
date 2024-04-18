use anyhow::{bail, Context, Result};
use crossterm::{
    style::Stylize,
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use std::{
    fs::{self, File},
    io::{Read, StdoutLock, Write},
    path::Path,
    process::{Command, Stdio},
};

use crate::{
    embedded::{WriteStrategy, EMBEDDED_FILES},
    exercise::Exercise,
    info_file::ExerciseInfo,
};

const STATE_FILE_NAME: &str = ".rustlings-state.txt";
const BAD_INDEX_ERR: &str = "The current exercise index is higher than the number of exercises";

#[must_use]
pub enum ExercisesProgress {
    AllDone,
    Pending,
}

pub enum StateFileStatus {
    Read,
    NotRead,
}

pub struct AppState {
    current_exercise_ind: usize,
    exercises: Vec<Exercise>,
    n_done: u16,
    final_message: String,
    file_buf: Vec<u8>,
    official_exercises: bool,
}

impl AppState {
    fn update_from_file(&mut self) -> StateFileStatus {
        self.file_buf.clear();
        self.n_done = 0;

        if File::open(STATE_FILE_NAME)
            .and_then(|mut file| file.read_to_end(&mut self.file_buf))
            .is_err()
        {
            return StateFileStatus::NotRead;
        }

        // See `Self::write` for more information about the file format.
        let mut lines = self.file_buf.split(|c| *c == b'\n');
        let Some(current_exercise_name) = lines.next() else {
            return StateFileStatus::NotRead;
        };

        if current_exercise_name.is_empty() || lines.next().is_none() {
            return StateFileStatus::NotRead;
        }

        let mut done_exercises = hashbrown::HashSet::with_capacity(self.exercises.len());

        for done_exerise_name in lines {
            if done_exerise_name.is_empty() {
                break;
            }
            done_exercises.insert(done_exerise_name);
        }

        for (ind, exercise) in self.exercises.iter_mut().enumerate() {
            if done_exercises.contains(exercise.name.as_bytes()) {
                exercise.done = true;
                self.n_done += 1;
            }

            if exercise.name.as_bytes() == current_exercise_name {
                self.current_exercise_ind = ind;
            }
        }

        StateFileStatus::Read
    }

    pub fn new(
        exercise_infos: Vec<ExerciseInfo>,
        final_message: String,
    ) -> (Self, StateFileStatus) {
        let exercises = exercise_infos
            .into_iter()
            .map(|mut exercise_info| {
                // Leaking to be able to borrow in the watch mode `Table`.
                // Leaking is not a problem because the `AppState` instance lives until
                // the end of the program.
                let path = exercise_info.path().leak();

                exercise_info.name.shrink_to_fit();
                let name = exercise_info.name.leak();

                let hint = exercise_info.hint.trim().to_owned();

                Exercise {
                    name,
                    path,
                    mode: exercise_info.mode,
                    hint,
                    done: false,
                }
            })
            .collect::<Vec<_>>();

        let mut slf = Self {
            current_exercise_ind: 0,
            exercises,
            n_done: 0,
            final_message,
            file_buf: Vec::with_capacity(2048),
            official_exercises: !Path::new("info.toml").exists(),
        };

        let state_file_status = slf.update_from_file();

        (slf, state_file_status)
    }

    #[inline]
    pub fn current_exercise_ind(&self) -> usize {
        self.current_exercise_ind
    }

    #[inline]
    pub fn exercises(&self) -> &[Exercise] {
        &self.exercises
    }

    #[inline]
    pub fn n_done(&self) -> u16 {
        self.n_done
    }

    #[inline]
    pub fn current_exercise(&self) -> &Exercise {
        &self.exercises[self.current_exercise_ind]
    }

    pub fn set_current_exercise_ind(&mut self, ind: usize) -> Result<()> {
        if ind >= self.exercises.len() {
            bail!(BAD_INDEX_ERR);
        }

        self.current_exercise_ind = ind;

        self.write()
    }

    pub fn set_current_exercise_by_name(&mut self, name: &str) -> Result<()> {
        // O(N) is fine since this method is used only once until the program exits.
        // Building a hashmap would have more overhead.
        self.current_exercise_ind = self
            .exercises
            .iter()
            .position(|exercise| exercise.name == name)
            .with_context(|| format!("No exercise found for '{name}'!"))?;

        self.write()
    }

    pub fn set_pending(&mut self, ind: usize) -> Result<()> {
        let exercise = self.exercises.get_mut(ind).context(BAD_INDEX_ERR)?;

        if exercise.done {
            exercise.done = false;
            self.n_done -= 1;
            self.write()?;
        }

        Ok(())
    }

    fn reset_path(&self, path: &str) -> Result<()> {
        if self.official_exercises {
            return EMBEDDED_FILES
                .write_exercise_to_disk(path, WriteStrategy::Overwrite)
                .with_context(|| format!("Failed to reset the exercise {path}"));
        }

        let output = Command::new("git")
            .arg("stash")
            .arg("push")
            .arg("--")
            .arg(path)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .output()
            .with_context(|| format!("Failed to run `git stash push -- {path}`"))?;

        if !output.status.success() {
            bail!(
                "`git stash push -- {path}` didn't run successfully: {}",
                String::from_utf8_lossy(&output.stderr),
            );
        }

        Ok(())
    }

    pub fn reset_current_exercise(&mut self) -> Result<&'static str> {
        let path = self.current_exercise().path;
        self.set_pending(self.current_exercise_ind)?;
        self.reset_path(path)?;

        Ok(path)
    }

    pub fn reset_exercise_by_ind(&mut self, exercise_ind: usize) -> Result<&'static str> {
        if exercise_ind >= self.exercises.len() {
            bail!(BAD_INDEX_ERR);
        }

        let path = self.exercises[exercise_ind].path;
        self.set_pending(exercise_ind)?;
        self.reset_path(path)?;

        Ok(path)
    }

    fn next_pending_exercise_ind(&self) -> Option<usize> {
        if self.current_exercise_ind == self.exercises.len() - 1 {
            // The last exercise is done.
            // Search for exercises not done from the start.
            return self.exercises[..self.current_exercise_ind]
                .iter()
                .position(|exercise| !exercise.done);
        }

        // The done exercise isn't the last one.
        // Search for a pending exercise after the current one and then from the start.
        match self.exercises[self.current_exercise_ind + 1..]
            .iter()
            .position(|exercise| !exercise.done)
        {
            Some(ind) => Some(self.current_exercise_ind + 1 + ind),
            None => self.exercises[..self.current_exercise_ind]
                .iter()
                .position(|exercise| !exercise.done),
        }
    }

    pub fn done_current_exercise(&mut self, writer: &mut StdoutLock) -> Result<ExercisesProgress> {
        let exercise = &mut self.exercises[self.current_exercise_ind];
        if !exercise.done {
            exercise.done = true;
            self.n_done += 1;
        }

        let Some(ind) = self.next_pending_exercise_ind() else {
            writer.write_all(RERUNNING_ALL_EXERCISES_MSG)?;

            for (exercise_ind, exercise) in self.exercises().iter().enumerate() {
                writer.write_fmt(format_args!("Running {exercise} ... "))?;
                writer.flush()?;

                if !exercise.run()?.status.success() {
                    writer.write_fmt(format_args!("{}\n\n", "FAILED".red()))?;

                    self.current_exercise_ind = exercise_ind;

                    // No check if the exercise is done before setting it to pending
                    // because no pending exercise was found.
                    self.exercises[exercise_ind].done = false;
                    self.n_done -= 1;

                    self.write()?;

                    return Ok(ExercisesProgress::Pending);
                }

                writer.write_fmt(format_args!("{}\n", "ok".green()))?;
            }

            writer.execute(Clear(ClearType::All))?;
            writer.write_all(FENISH_LINE.as_bytes())?;

            let final_message = self.final_message.trim();
            if !final_message.is_empty() {
                writer.write_all(final_message.as_bytes())?;
                writer.write_all(b"\n")?;
            }

            return Ok(ExercisesProgress::AllDone);
        };

        self.set_current_exercise_ind(ind)?;

        Ok(ExercisesProgress::Pending)
    }

    // Write the state file.
    // The file's format is very simple:
    // - The first line is the name of the current exercise. It must end with `\n` even if there
    // are no done exercises.
    // - The second line is an empty line.
    // - All remaining lines are the names of done exercises.
    fn write(&mut self) -> Result<()> {
        self.file_buf.clear();

        self.file_buf
            .extend_from_slice(self.current_exercise().name.as_bytes());
        self.file_buf.push(b'\n');

        for exercise in &self.exercises {
            if exercise.done {
                self.file_buf.push(b'\n');
                self.file_buf.extend_from_slice(exercise.name.as_bytes());
            }
        }

        fs::write(STATE_FILE_NAME, &self.file_buf)
            .with_context(|| format!("Failed to write the state file {STATE_FILE_NAME}"))?;

        Ok(())
    }
}

const RERUNNING_ALL_EXERCISES_MSG: &[u8] = b"
All exercises seem to be done.
Recompiling and running all exercises to make sure that all of them are actually done.

";

const FENISH_LINE: &str = "+----------------------------------------------------+
|          You made it to the Fe-nish line!          |
+--------------------------  ------------------------+
                           \\/\x1b[31m
     ▒▒          ▒▒▒▒▒▒▒▒      ▒▒▒▒▒▒▒▒          ▒▒
   ▒▒▒▒  ▒▒    ▒▒        ▒▒  ▒▒        ▒▒    ▒▒  ▒▒▒▒
   ▒▒▒▒  ▒▒  ▒▒            ▒▒            ▒▒  ▒▒  ▒▒▒▒
 ░░▒▒▒▒░░▒▒  ▒▒            ▒▒            ▒▒  ▒▒░░▒▒▒▒
   ▓▓▓▓▓▓▓▓  ▓▓      ▓▓██  ▓▓  ▓▓██      ▓▓  ▓▓▓▓▓▓▓▓
     ▒▒▒▒    ▒▒      ████  ▒▒  ████      ▒▒░░  ▒▒▒▒
       ▒▒  ▒▒▒▒▒▒        ▒▒▒▒▒▒        ▒▒▒▒▒▒  ▒▒
         ▒▒▒▒▒▒▒▒▒▒▓▓▓▓▓▓▒▒▒▒▒▒▒▒▓▓▓▓▓▓▒▒▒▒▒▒▒▒
           ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
             ▒▒▒▒▒▒▒▒▒▒██▒▒▒▒▒▒██▒▒▒▒▒▒▒▒▒▒
           ▒▒  ▒▒▒▒▒▒▒▒▒▒██████▒▒▒▒▒▒▒▒▒▒  ▒▒
         ▒▒    ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒    ▒▒
       ▒▒    ▒▒    ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒    ▒▒    ▒▒
       ▒▒  ▒▒    ▒▒                  ▒▒    ▒▒  ▒▒
           ▒▒  ▒▒                      ▒▒  ▒▒\x1b[0m

";