//! Library of functions and typedefs to support program arewegonnawintheleague

use rand::distr::weighted::WeightedIndex;
use rand::prelude::*;
use std::collections::HashMap;

//source for distribution calcuation:
//    https://fivethirtyeight.com/features/in-126-years-english-football-has-seen-13475-nil-nil-draws/

const NUM_POSSIBLE_GOALS: [i32; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
const HOME_WEIGHTS: [f32; 8] = [18.8, 30.3, 24.8, 14.3, 7.0, 3.1, 1.2, 0.5];
const AWAY_WEIGHTS: [f32; 8] = [33.8, 36.2, 19.3, 7.4, 2.3, 0.7, 0.2, 0.1];

// Structures for managing data within simulations
//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// struct to store individual team data
// held within the league table structure
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Team {
    name: String,
    pts: u32,
    goal_diff: i32,
    matches_played: u32,
}

impl Team {
    pub fn new(name: String, pts: u32, goal_diff: i32, matches_played: u32) -> Self {
        Self {
            name,
            pts,
            goal_diff,
            matches_played,
        }
    }

    pub fn update(&mut self, match_goal_diff: i32) {
        self.matches_played += 1;
        self.goal_diff += match_goal_diff;
        if match_goal_diff == 0 {
            self.pts += 1;
        } else if match_goal_diff > 0 {
            self.pts += 3;
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Match {
    home: String,
    away: String,
    home_goals: i32,
    away_goals: i32,
}

impl Match {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from(home: &str, away: &str) -> Self {
        Self {
            home: home.to_string(),
            away: away.to_string(),
            home_goals: 0,
            away_goals: 0,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct LeagueTable(HashMap<String, Team>);

impl LeagueTable {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn print_table(&self) {
        println!("Rank\tTeam");
        let mut i = 1;
        let mut print_vector: Vec<&Team> = self.0.values().collect();
        print_vector.sort_by(|x, y| {
            y.pts
                .cmp(&x.pts)
                .then_with(|| y.goal_diff.cmp(&x.goal_diff))
        });
        for team in print_vector {
            print!("{}\t{}\n", i, team.name);
            i += 1;
        }
    }

    pub fn add_team(
        &mut self,
        name: String,
        pts: u32,
        goals_for: i32,
        goals_against: i32,
        matches_played: u32,
    ) {
        self.0.entry(name.clone()).insert_entry(Team::new(
            name.clone(),
            pts,
            goals_for - goals_against,
            matches_played,
        ));
    }

    pub fn update(&mut self, latest_match: &Match) {
        let goal_diff = latest_match.home_goals - latest_match.away_goals;
        self.0
            .get_mut(&latest_match.home)
            .unwrap()
            .update(goal_diff);
        self.0
            .get_mut(&latest_match.away)
            .unwrap()
            .update(-goal_diff);
    }

    // could we do this more efficiently?
    pub fn find_final_rank(&mut self, desired_team: &str) -> i32 {
        let mut i = 1;
        let mut ordered_vector: Vec<&Team> = self.0.values().collect();
        ordered_vector.sort_by(|x, y| {
            y.pts
                .cmp(&x.pts)
                .then_with(|| y.goal_diff.cmp(&x.goal_diff))
        });
        for team in ordered_vector {
            if team.name == desired_team {
                break;
            } else {
                i += 1;
            }
        }

        i
    }
}

//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Structures for simulation running and data tracking

pub fn run_simulation(
    target_team: &String,
    current_table: &mut LeagueTable,
    match_list: &mut [Match],
) -> i32 {
    let mut simulated_table = current_table.clone();
    let home_dist = WeightedIndex::new(&HOME_WEIGHTS).unwrap();
    let away_dist = WeightedIndex::new(&AWAY_WEIGHTS).unwrap();
    let mut rng = rand::rng();

    for game in match_list {
        game.home_goals = NUM_POSSIBLE_GOALS[home_dist.sample(&mut rng)];
        game.away_goals = NUM_POSSIBLE_GOALS[away_dist.sample(&mut rng)];
        simulated_table.update(game);
    }

    simulated_table.find_final_rank(target_team)
}

//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Unit Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_one_team() {
        let mut league_table = LeagueTable::new();
        league_table.add_team("Liverpool".to_string(), 67, 66, 26, 28);
        assert_ne!(league_table.0.get("Liverpool"), None);
        assert_eq!("Liverpool", league_table.0.get("Liverpool").unwrap().name);
    }

    #[test]
    fn print_league_table() {
        let mut league_table = LeagueTable::new();
        league_table.add_team("Liverpool".to_string(), 67, 66, 26, 28);
        league_table.add_team("Arsenal".to_string(), 27, 51, 23, 27);
        league_table.print_table();
    }

    #[test]
    fn print_reranked_league_table() {
        let mut league_table = LeagueTable::new();
        league_table.add_team("Liverpool".to_string(), 67, 66, 26, 28);
        league_table.add_team("Arsenal".to_string(), 27, 51, 23, 27);
        league_table.print_table();

        league_table
            .0
            .entry("Arsenal".to_string())
            .and_modify(|team| team.pts = 70);
        league_table.print_table();
    }

    #[test]
    fn manually_update_team_data() {
        let mut league_table = LeagueTable::new();
        league_table.add_team("Liverpool".to_string(), 67, 66, 26, 28);
        assert_ne!(league_table.0.get("Liverpool"), None);
        assert_eq!(67, league_table.0.get("Liverpool").unwrap().pts);
        assert_eq!(40, league_table.0.get("Liverpool").unwrap().goal_diff);
        assert_eq!(28, league_table.0.get("Liverpool").unwrap().matches_played);
    }

    #[test]
    fn update_with_match_data() {
        let new_match = Match {
            home: "Liverpool".to_string(),
            away: "Arsenal".to_string(),
            home_goals: 2,
            away_goals: 0,
        };
        let mut league_table = LeagueTable::new();
        league_table.add_team("Liverpool".to_string(), 67, 66, 26, 28);
        league_table.add_team("Arsenal".to_string(), 27, 51, 23, 27);
        league_table.update(&new_match);

        assert_eq!(70, league_table.0.get("Liverpool").unwrap().pts);
        assert_eq!(42, league_table.0.get("Liverpool").unwrap().goal_diff);
        assert_eq!(29, league_table.0.get("Liverpool").unwrap().matches_played);

        assert_eq!(27, league_table.0.get("Arsenal").unwrap().pts);
        assert_eq!(26, league_table.0.get("Arsenal").unwrap().goal_diff);
        assert_eq!(28, league_table.0.get("Arsenal").unwrap().matches_played);

        let second_match = Match {
            home: "Liverpool".to_string(),
            away: "Arsenal".to_string(),
            home_goals: 2,
            away_goals: 2,
        };
        league_table.update(&second_match);

        assert_eq!(71, league_table.0.get("Liverpool").unwrap().pts);
        assert_eq!(42, league_table.0.get("Liverpool").unwrap().goal_diff);
        assert_eq!(30, league_table.0.get("Liverpool").unwrap().matches_played);

        assert_eq!(28, league_table.0.get("Arsenal").unwrap().pts);
        assert_eq!(26, league_table.0.get("Arsenal").unwrap().goal_diff);
        assert_eq!(29, league_table.0.get("Arsenal").unwrap().matches_played);
    }

    #[test]
    fn get_final_ranking() {
        let mut league_table = LeagueTable::new();
        league_table.add_team("Liverpool".to_string(), 67, 66, 26, 28);
        league_table.add_team("Arsenal".to_string(), 54, 51, 23, 27);

        let liverpool_rank = league_table.find_final_rank("Liverpool");
        let arsenal_rank = league_table.find_final_rank("Arsenal");

        assert_eq!(1, liverpool_rank);
        assert_eq!(2, arsenal_rank);
    }

    #[test]
    fn small_simulation() {
        let mut league_table = LeagueTable::new();
        league_table.add_team("Liverpool".to_string(), 67, 66, 26, 28);
        league_table.add_team("Arsenal".to_string(), 54, 51, 23, 27);
        league_table.add_team("Nottingham Forest".to_string(), 48, 44, 26, 27);
        league_table.add_team("Manchester City".to_string(), 47, 53, 37, 27);

        let mut matches = [
            Match::from("Liverpool", "Arsenal"),
            Match::from("Liverpool", "Nottingham Forest"),
            Match::from("Liverpool", "Manchester City"),
            Match::from("Arsenal", "Liverpool"),
            Match::from("Arsenal", "Nottingham Forest"),
            Match::from("Arsenal", "Manchester City"),
            Match::from("Nottingham Forest", "Liverpool"),
            Match::from("Nottingham Forest", "Arsenal"),
            Match::from("Nottingham Forest", "Manchester City"),
            Match::from("Manchester City", "Liverpool"),
            Match::from("Manchester City", "Arsenal"),
            Match::from("Manchester City", "Nottingham Forest"),
        ];

        let target = "Arsenal".to_string();
        let mut count= 0.0;
        for _x in 1..50 {
            if run_simulation(&target, &mut league_table, &mut matches) <= 2 {
                count += 1.0;
            }
        }

        println!("{} {}%", target, count / 50.0 * 100.0);
    }
}
