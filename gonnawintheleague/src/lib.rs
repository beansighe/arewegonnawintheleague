//! Library of functions and typedefs to support program arewegonnawintheleague

use std::collections::HashMap;

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

#[derive(Debug, Default, Clone)]
pub struct LeagueTable(HashMap<String, Team>);

impl LeagueTable {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn print_table(&self) {
        println!("Rank\tTeam");
        let mut i = 1;
        /*
        for key in self.0.keys() {
        */
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

    pub fn update(&mut self, latest_match: Match) {
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
}

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

        league_table.0.entry("Arsenal".to_string()).and_modify(|team| team.pts = 70);
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
        league_table.update(new_match);

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
        league_table.update(second_match);

        assert_eq!(71, league_table.0.get("Liverpool").unwrap().pts);
        assert_eq!(42, league_table.0.get("Liverpool").unwrap().goal_diff);
        assert_eq!(30, league_table.0.get("Liverpool").unwrap().matches_played);

        assert_eq!(28, league_table.0.get("Arsenal").unwrap().pts);
        assert_eq!(26, league_table.0.get("Arsenal").unwrap().goal_diff);
        assert_eq!(29, league_table.0.get("Arsenal").unwrap().matches_played);

    }
}
