//! Library of functions and typedefs to support program arewegonnawintheleague.
//! 
//! This library contains structures and supports for managing standings and
//! match data for starting input and simulation results, running simulations,
//! and reading data in from json files (in place of API calls, for now)
//! 

use rand::distr::weighted::WeightedIndex;
use rand::prelude::*;
use relative_path::RelativePath;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::env::current_dir;
use std::fs::File;
use std::io::BufReader;


const NUM_POSSIBLE_GOALS: [i32; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
const HOME_WEIGHTS: [f32; 8] = [18.8, 30.3, 24.8, 14.3, 7.0, 3.1, 1.2, 0.5];
const AWAY_WEIGHTS: [f32; 8] = [33.8, 36.2, 19.3, 7.4, 2.3, 0.7, 0.2, 0.1];
const FIXTURES_PATH: &str = "/data/fixtures_list.json";
const STANDINGS_PATH: &str = "/data/standings.json";

// Structures for managing data within simulations
//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Stores individual team data to be held within the league table structure
#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct Team {
    name: String,
    pts: u32,
    goal_diff: i32,
}

impl Team {
    /// Create a new team based on raw data
    pub fn new(name: String, pts: u32, goal_diff: i32) -> Self {
        Self {
            name,
            pts,
            goal_diff,
        }
    }

    /// Updates pts based on passed match outcome data
    /// to reflect effect of simulated match on team's
    /// table standing
    pub fn update(&mut self, match_goal_diff: i32) {
        self.goal_diff += match_goal_diff;
        match match_goal_diff.cmp(&0) {
            Ordering::Equal => self.pts += 1,
            Ordering::Greater => self.pts += 3,
            Ordering::Less => (),
        }
    }
}

/// Stores match data to be used in simulation
/// 
/// Home and away affects the distribution used in
/// simulating the scores as well as how the match goal
/// differential is passed to the corresponding Team's
/// update function
#[derive(Debug, Default, Clone)]
pub struct Match {
    home: String,
    away: String,
}

impl Match {
    /// create an empty Match
    pub fn new() -> Self {
        Self::default()
    }

    /// create a Match using provided data
    pub fn from(home: &str, away: &str) -> Self {
        Self {
            home: home.to_string(),
            away: away.to_string(),
        }
    }
}

/// Structure for storing current standings as well as 
/// standings generated through a simulation
#[derive(Debug, Default, Clone)]
pub struct LeagueTable(HashMap<String, Team>);

impl LeagueTable {
    /// create an empty LeagueTable
    pub fn new() -> Self {
        Self::default()
    }

    /// Function to print an ordered league table to stdout
    /// 
    /// Used in unit testing
    pub fn print_table(&self) {
        println!("Rank\tTeam\t\t\tPoints\t GD");
        let mut i = 1;
        let mut print_vector: Vec<&Team> = self.0.values().collect();
        print_vector.sort_by(|x, y| {
            y.pts
                .cmp(&x.pts)
                .then_with(|| y.goal_diff.cmp(&x.goal_diff))
        });
        for team in print_vector {
            println!(
                "{}\t{:<10}\t\t{:>5}\t{:>3}",
                i, team.name, team.pts, team.goal_diff
            );
            i += 1;
        }
    }

    /// Function to add to the table using raw data
    pub fn add_team(&mut self, name: String, pts: u32, goals_diff: i32) {
        self.0
            .entry(name.clone())
            .insert_entry(Team::new(name.clone(), pts, goals_diff));
    }

    /// Function to add to the table using an externally instantiated Team struct
    pub fn add_team_struct(&mut self, name: String, team: Team) {
        self.0.entry(name.clone()).insert_entry(team);
    }

    /// Function to update the data of the designated teams stored within the
    /// LeagueTable based on simulated match data
    /// 
    /// The goal differential is calculated once and passed as is to the home
    /// team and multiplied by negative 1 to the away team
    pub fn update(&mut self, latest_match: &Match, home_goals: i32, away_goals: i32) {
        let goal_diff = home_goals - away_goals;
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
    /// Returns the rank achieved in a single simulation by the team
    /// whose name matches the passed &str
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


// Structures for simulation running and data tracking
//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Simulates outcomes in all matches in the list of matches remaining in the season and 
/// returns the rank achieved by the target team
/// 
/// The weights used in the distribution model for the Monte Carlo simulation 
/// were calculated based on data from the following source:
///    <https://fivethirtyeight.com/features/in-126-years-english-football-has-seen-13475-nil-nil-draws/>
/// itself based on data collected by James Curley: <https://github.com/jalapic/engsoccerdata>
/// 
/// This simulation is based on overall historical data on the average number of 
/// goals scored by home or away teams in the top four tiers of English Football League play.
/// It does not take into account recent form or historical results between specific teams.
pub fn run_simulation(
    target_team: &str,
    current_table: &LeagueTable,
    match_list: &Vec<Match>,
) -> i32 {
    let mut simulated_table = current_table.clone();
    let home_dist = WeightedIndex::new(HOME_WEIGHTS).unwrap();
    let away_dist = WeightedIndex::new(AWAY_WEIGHTS).unwrap();
    let mut rng = rand::rng();

    for game in match_list {
        let home_goals = NUM_POSSIBLE_GOALS[home_dist.sample(&mut rng)];
        let away_goals = NUM_POSSIBLE_GOALS[away_dist.sample(&mut rng)];
        simulated_table.update(game, home_goals, away_goals);
    }

    simulated_table.find_final_rank(target_team)
}

//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Reading in data from files

/// Function to read in a list of the remaining fixtures in the Premier League season
/// from a json file and store the result in a vector
/// 
/// Json should take form of an array of objects, each containing two string literals
/// labeled "home" and "away" as appropriate
pub fn read_fixtures(fixture_list: &mut Vec<Match>) {
    let root_dir =
        current_dir().expect("should only be run in valid directory with appropriate permissions");
    let fixtures_relative = RelativePath::new(FIXTURES_PATH);
    let fixtures_full_path = fixtures_relative.to_path(&root_dir);
    println!("fixtures path: {fixtures_full_path:?}");
    let file = File::open(fixtures_full_path).expect("file should open if path constant valid");
    let reader = BufReader::new(file);
    let fixtures: Result<Value> = serde_json::from_reader(reader);
    match fixtures {
        Ok(list) => {
            for i in 0..379 {
                let catch = list.get(i);
                match catch {
                    None => break,
                    Some(entry) => {
                        fixture_list.push(Match::from(
                            entry["home"].as_str().unwrap(),
                            entry["away"].as_str().unwrap(),
                        ));
                    }
                }
            }
        }
        Err(error) => println!("error reading file: {error:?}"),
    }
}

/// Function to read in the current standings in the Premier League from
/// a json file and store in a LeagueTable struct
/// 
/// Json file should take the form of an array of objects, each of which
/// must take the form of a Team struct in order to be read
pub fn read_standings(current_table: &mut LeagueTable) {
    let root_dir =
        current_dir().expect("should only be run in valid directory with appropriate permissions");
    let standings_relative = RelativePath::new(STANDINGS_PATH);
    let standings_full_path = standings_relative.to_path(&root_dir);
    println!("standings full path: {standings_full_path:?}");
    let file = File::open(standings_full_path).expect("file should open if path constant valid");
    let reader = BufReader::new(file);
    let standings_data: [Team; 20] =
        serde_json::from_reader(reader).expect("data should be correctly formatted");
    for team in standings_data {
        current_table.add_team_struct(team.name.to_string(), team.clone());
    }
}

//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Unit Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_one_team() {
        let mut league_table = LeagueTable::new();
        league_table.add_team("Liverpool".to_string(), 67, 40);
        assert_ne!(league_table.0.get("Liverpool"), None);
        assert_eq!("Liverpool", league_table.0.get("Liverpool").unwrap().name);
    }

    #[test]
    fn print_league_table() {
        let mut league_table = LeagueTable::new();
        league_table.add_team("Liverpool".to_string(), 67, 40);
        league_table.add_team("Arsenal".to_string(), 27, 28);
        league_table.print_table();
    }

    #[test]
    fn print_reranked_league_table() {
        let mut league_table = LeagueTable::new();
        league_table.add_team("Liverpool".to_string(), 67, 40);
        league_table.add_team("Arsenal".to_string(), 27, 28);
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
        league_table.add_team("Liverpool".to_string(), 67, 40);
        assert_ne!(league_table.0.get("Liverpool"), None);
        assert_eq!(67, league_table.0.get("Liverpool").unwrap().pts);
        assert_eq!(40, league_table.0.get("Liverpool").unwrap().goal_diff);
    }

    #[test]
    fn update_with_match_data() {
        let new_match = Match {
            home: "Liverpool".to_string(),
            away: "Arsenal".to_string(),
        };
        let mut league_table = LeagueTable::new();
        league_table.add_team("Liverpool".to_string(), 67, 40);
        league_table.add_team("Arsenal".to_string(), 27, 26);
        league_table.update(&new_match, 2, 0);

        assert_eq!(70, league_table.0.get("Liverpool").unwrap().pts);
        assert_eq!(42, league_table.0.get("Liverpool").unwrap().goal_diff);

        assert_eq!(27, league_table.0.get("Arsenal").unwrap().pts);
        assert_eq!(24, league_table.0.get("Arsenal").unwrap().goal_diff);

        let second_match = Match {
            home: "Liverpool".to_string(),
            away: "Arsenal".to_string(),
        };
        league_table.update(&second_match, 2, 2);

        assert_eq!(71, league_table.0.get("Liverpool").unwrap().pts);
        assert_eq!(42, league_table.0.get("Liverpool").unwrap().goal_diff);

        assert_eq!(28, league_table.0.get("Arsenal").unwrap().pts);
        assert_eq!(24, league_table.0.get("Arsenal").unwrap().goal_diff);
    }

    #[test]
    fn get_final_ranking() {
        let mut league_table = LeagueTable::new();
        league_table.add_team("Liverpool".to_string(), 67, 40);
        league_table.add_team("Arsenal".to_string(), 54, 28);

        let liverpool_rank = league_table.find_final_rank("Liverpool");
        let arsenal_rank = league_table.find_final_rank("Arsenal");

        assert_eq!(1, liverpool_rank);
        assert_eq!(2, arsenal_rank);
    }

    #[test]
    fn small_simulation() {
        let mut league_table = LeagueTable::new();
        league_table.add_team("Liverpool".to_string(), 67, 40);
        league_table.add_team("Arsenal".to_string(), 54, 28);
        league_table.add_team("Nottingham Forest".to_string(), 48, 18);
        league_table.add_team("Manchester City".to_string(), 47, 16);

        let mut matches = vec![
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
        let mut count = 0.0;
        for _x in 1..50 {
            if run_simulation(&target, &mut league_table, &mut matches) <= 1 {
                count += 1.0;
            }
        }

        println!("{} {}%", target, count / 50.0 * 100.0);
    }

    #[test]
    fn read_in_table() {
        let mut new_league_table = LeagueTable::new();
        read_standings(&mut new_league_table);
        new_league_table.print_table();
    }

    #[test]
    fn read_in_fixture_list() {
        let mut fixtures_list = Vec::<Match>::new();
        read_fixtures(&mut fixtures_list);
        println!("Fixtures\n{fixtures_list:?}");
    }

    #[test]
    fn full_threadless_sim_test() {
        let mut fixtures = Vec::<Match>::new();
        let mut current_table = LeagueTable::new();
        read_standings(&mut current_table);
        read_fixtures(&mut fixtures);
        let target_team = "Brighton".to_string();
        let rank = 7;
        let mut count = 0.0;
        for _i in 1..50 {
            if run_simulation(&target_team, &mut current_table, &mut fixtures) <= rank {
                count += 1.0;
            }
        }
        println!(
            "Percent chance {} finishes at or above rank {}: {}%",
            target_team,
            rank,
            count / 50.0 * 100.0
        );
    }
}
