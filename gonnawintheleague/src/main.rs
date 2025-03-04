use gonnawintheleague::LeagueTable;

fn main() {
    //data needed:
    //  - current table standings
    //      - team name
    //      - points
    //      - goals for
    //      - goals against
    //      - gd
    //  - remaining fixture list
    //      - home team
    //      - away team

    fn simulate_match(home_team: String, away_team: String) {
        //get random number b/t 0,1, if <.5 assign to away team first
        //get random number b/t 0-9, assign to first team
        //get random number b/t 0-9, assign to second team

        // add goals for and against for each team
        /*let result = home_score - away_score;
        if result == 0 {
            home_points += 1;
            away_points += 1;
            } else if result > 0 {
            home_points += 3;
            } else {
            away_points += 3;
        }*/
    }

    fn simulate_season() {
        /*for match in remaining_match_list {
          simulate_match(match.home_team, match.away_team);
        }*/
    }
    let mut league_table = LeagueTable::new();
    league_table.add_team("Liverpool".to_string(), 67, 66, 26, 28);
    //league_table.add_team("Arsenal".to_string(), 27, 51, 23, 27);
    league_table.print_table();
    // track how often team lands at given spot or above
    // divide by n (number of simulations)
}
