// hashmaps3.rs

// A list of scores (one per line) of a soccer match is given. Each line
// is of the form :
// <team_1_name>,<team_2_name>,<team_1_goals>,<team_2_goals>
// Example: England,France,4,2 (England scored 4 goals, France 2).

// You have to build a scores table containing the name of the team, goals
// the team scored, and goals the team conceded. One approach to build
// the scores table is to use a Hashmap. The solution is partially
// written to use a Hashmap, complete it to pass the test.

// Make me pass the tests!

// Execute `rustlings hint hashmaps3` or use the `hint` watch subcommand for a hint.

use std::collections::HashMap;

// A structure to store team name and its goal details.
struct Team {
    name: String,
    goals_scored: u8,
    goals_conceded: u8,
}

struct Scores(HashMap<String, Team>);
impl Scores {
    fn new() -> Self {
        Scores(HashMap::new())
    }

    fn add(&mut self, name: String, scored: u8, conceded: u8) {
        match self.0.get_mut(&name) {
            Some(scores) => {
                scores.goals_scored += scored;
                scores.goals_conceded += conceded;
            }
            None => {
                let scores = Team {
                    name: name.to_owned(),
                    goals_scored: scored,
                    goals_conceded: conceded,
                };

                self.0.insert(name.to_owned(), scores);
            }
        }
        // if let Some(scores) = self.0.get_mut(&name) {
        //     scores.goals_scored += scored;
        //     scores.goals_conceded += conceded;
        // } else {
        //     let scores = Team {
        //         name: name.to_owned(),
        //         goals_scored: scored,
        //         goals_conceded: conceded,
        //     };

        //     self.0.insert(name.to_owned(), scores);
        // }
    }
}

fn build_scores_table(results: String) -> HashMap<String, Team> {
    // The name of the team is the key and its associated struct is the value.
    let mut scores = Scores::new();

    for r in results.lines() {
        let v: Vec<&str> = r.split(',').collect();
        let team_1_name = v[0].to_string();
        let team_1_score: u8 = v[2].parse().unwrap();
        let team_2_name = v[1].to_string();
        let team_2_score: u8 = v[3].parse().unwrap();

        scores.add(team_1_name, team_1_score, team_2_score);
        scores.add(team_2_name, team_2_score, team_1_score);
    }

    scores.0
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
}
