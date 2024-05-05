// threads2.rs
//
// Building on the last exercise, we want all of the threads to complete their
// work but this time the spawned threads need to be in charge of updating a
// shared value: JobStatus.jobs_completed
//
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a
// hint.

use std::ops::DerefMut;
use std::sync::{Arc, Mutex, MutexGuard, Condvar};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed_count: u32,
}

fn main() {
    // example: spawn 10 threads each updating a Mutex protected struct value
    // s.t the struct int field initialized at 0 getting to 10 in the end 
    let ref_arc_status: Arc<Mutex<JobStatus>> = Arc::new(Mutex::new(JobStatus {jobs_completed_count: 0}));
    let mut handles = vec![];
    
    for _ in 0..10 {
        let ref_arc_cloned_status: Arc<Mutex<JobStatus>> = Arc::clone(&ref_arc_status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(25));

            // would cause temporary value dropped while borrowed error: https://doc.rust-lang.org/error_codes/E0716.html ?!
            // let ref_mut_job_status: &mut JobStatus = ref_arc_cloned_status.lock().unwrap().deref_mut();

            let mut tmp_mutex_guard_job_status: MutexGuard<'_, JobStatus>= ref_arc_cloned_status.lock().unwrap();
            // the only way to access the inner value of a MutexGuard is via Deref or DerefMut implementations:
            // https://doc.rust-lang.org/std/sync/struct.MutexGuard.html#
            let ref_mut_job_status: &mut JobStatus = &mut tmp_mutex_guard_job_status.deref_mut();

            ref_mut_job_status.jobs_completed_count = ref_mut_job_status.jobs_completed_count + 1;
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }

    println!("jobs completed {}", ref_arc_status.lock().unwrap().jobs_completed_count);


    // ditto example: spawn 10 threads with each incrementing a Mutex guard counter
    // counter value initialised at 0 to result in the final value to be 10
    let counter_mutex: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
    let mut handles_vec: Vec<thread::JoinHandle<()>> = vec![];
    
    for _ in 0..10 {
        let counter_mutex_cloned: Arc<Mutex<i32>> = Arc::clone(&counter_mutex);
        let thread_handle: thread::JoinHandle<()> = thread::spawn(move || -> () {
            let mut counter: MutexGuard<'_, i32> = counter_mutex_cloned.lock().unwrap();
            *counter += 1;
        });
        handles_vec.push(thread_handle);
    }
    
    for handle in handles_vec {
        handle.join().unwrap();
    }
    
    let counter_result: i32 = *counter_mutex.lock().unwrap();
    
    println!("Final counter result {}", counter_result);

    // example: use Condvar with Mutex to coordinate workload between threads
    {
        // the common pattern is to pair a Condvar with a Mutex for signalling
        // change to the Mutex made in one thread that is intended for
        // another otherwise dormant thread to pick up and process
        let pair: Arc<(Mutex<bool>, Condvar)> = Arc::new((Mutex::new(false), Condvar::new()));
        let pair2 = Arc::clone(&pair);

        // spawn a new thread, acquire the Mutex lock to make change to it
        // and notify the change via Condvar in the end
        thread::spawn(move|| {
            let ref_pair: &(Mutex<bool>, Condvar) = &*pair2;
            
            let lock: &Mutex<bool> = &ref_pair.0;
            let mut up: MutexGuard<'_, bool> = lock.lock().unwrap();
            *up = true;
            
            let cvar = &ref_pair.1;
            cvar.notify_one();
        });

        // the other side also need to acquire the lock to check for the
        // expected signal in the Mutex. When the expected signal is
        // not present yet, wait() method puts this thread to sleep to
        // be later woken up to check again (hopefully then to get the
        // expected signal s.t this thread can proceed with that,
        // signal but also likely to witness spurious wakeup)
        let (lock, cvar) = &*pair;
        let mut up: MutexGuard<'_, bool> = lock.lock().unwrap();
        while !*up {
            up = cvar.wait(up).unwrap();
        }

        // finally "up" should be true
        assert!(*up);
    }
}
