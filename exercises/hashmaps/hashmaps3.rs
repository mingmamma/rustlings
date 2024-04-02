// hashmaps3.rs
//
// A list of scores (one per line) of a soccer match is given. Each line is of
// the form : "<team_1_name>,<team_2_name>,<team_1_goals>,<team_2_goals>"
// Example: England,France,4,2 (England scored 4 goals, France 2).
//
// You have to build a scores table containing the name of the team, goals the
// team scored, and goals the team conceded. One approach to build the scores
// table is to use a Hashmap. The solution is partially written to use a
// Hashmap, complete it to pass the test.
//
// Make me pass the tests!
//
// Execute `rustlings hint hashmaps3` or use the `hint` watch subcommand for a
// hint.

use std::collections::{HashMap, HashSet};

// A structure to store the goal details of a team.
struct Team {
    goals_scored: u8,
    goals_conceded: u8,
}

fn build_scores_table(results: String) -> HashMap<String, Team> {
    // The name of the team is the key and its associated struct is the value.
    let mut scores: HashMap<String, Team> = HashMap::new();

    // parsing utility to iterate multilines of raw String data and extract info
    // for example
    // "England,France,4,2\nFrance,Italy,3,1\nPoland,Spain,2,0\nGermany,England,2,1\n" is four lines of data
    // where the two names and scores of interested are extracted from each line s.t. the first line, from example, yields
    // team_1_name: England, team_1_score: 4, team_2_name: France, team_2_score: France
    for r in results.lines() {
        let v: Vec<&str> = r.split(',').collect();
        let team_1_name = v[0].to_string();
        let team_1_score: u8 = v[2].parse().unwrap();
        let team_2_name = v[1].to_string();
        let team_2_score: u8 = v[3].parse().unwrap();
        
        // Populate the scores table with details extracted from the
        // current line. Keep in mind that goals scored by team_1
        // will be the number of goals conceded from team_2, and similarly
        // goals scored by team_2 will be the number of goals conceded by team_1.
        
        // the following pattern of "updating the old value by increment if exist, or initialize a new unit value" is
        // analogous to the pattern examplified by the classic word count example 
        // https://doc.rust-lang.org/book/ch08-03-hash-maps.html#updating-a-value-based-on-the-old-value
        
        let team_1: &mut Team = scores.entry(team_1_name).or_insert(Team{goals_scored: 0, goals_conceded: 0});
        

        // the value of type &mut Team is functionally equivalent to type mut Team s.t. the += operator works?!
        // being functionally equivalent specifically means the derefencing &mut Team to mut Team is not needed to work,
        // as the following, noting the extra set of parentheses, due to the precedence of operator rule
        
        // (*team_1).goals_scored += team_1_score;
        team_1.goals_scored += team_1_score;
        team_1.goals_conceded += team_2_score;

        let team_2 = scores.entry(team_2_name).or_insert(Team{goals_scored: 0, goals_conceded: 0});
        team_2.goals_scored += team_2_score;
        team_2.goals_conceded += team_1_score;
    }
    scores
}

// Rust does not have a Scala comparable toSet method in Vector. User defined solutions:
//https://stackoverflow.com/questions/39803237/build-hashset-from-a-vector-in-rust#39804262
fn vec_to_iter(vec: Vec<&String>) -> HashSet<&String> {
    HashSet::from_iter(vec)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_results() -> String {
        let results = "".to_string()
            + "England,France,4,2\n"
            + "France,Italy,3,1\n"
            + "Poland,Spain,2,0\n"
            + "Germany,England,2,1\n";
        results
    }

    #[test]
    fn build_scores() {
        let scores = build_scores_table(get_results());

        let mut keys: Vec<&String> = scores.keys().collect();
        keys.sort();
        assert_eq!(
            keys,
            vec!["England", "France", "Germany", "Italy", "Poland", "Spain"]
        );

        // Asserting with HashSet to ascertain the values instead of a sorted Vector
        // Noting the flexibility of collect method that can accomodate various return container type?!
        // Noting the inflexilibity that the element type of the container is &String, what's the restriction here?!
        let keys_2_set: HashSet<&String> = scores.keys().collect();

        // As a consequence of the element type of &String, initializing some &String type data for assertion
        let England = &String::from("England");
        let France = &String::from("France");
        let Germany = &String::from("Germany");
        let Italy = &String::from("Italy");
        let Poland = &String::from("Poland");
        let Spain = &String::from("Spain");

        let expected_hashset = HashSet::from([England, France, Germany, Italy, Poland, Spain]);
        assert_eq!(
            keys_2_set,
            expected_hashset
        )
    }

    #[test]
    fn validate_team_score_1() {
        let scores = build_scores_table(get_results());
        let team = scores.get("England").unwrap();
        assert_eq!(team.goals_scored, 5);
        assert_eq!(team.goals_conceded, 4);
    }

    #[test]
    fn validate_team_score_2() {
        let scores = build_scores_table(get_results());
        let team = scores.get("Spain").unwrap();
        assert_eq!(team.goals_scored, 0);
        assert_eq!(team.goals_conceded, 2);
    }

    #[test]
    fn validate_team_score_3() {
        let scores = build_scores_table(get_results());
        let team = scores.get("France").unwrap();
        assert_eq!(team.goals_scored, 5);
        assert_eq!(team.goals_conceded, 5);        
    }
}
