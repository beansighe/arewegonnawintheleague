use gonnawintheleague as league;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread;

//cast using as f32 to use as divisor
const NUM_SIMULATIONS: i32 = 4000;
const NUM_THREADS: u32 = 4;

fn main() {
    // STEPS
    // read in target team, target rank
    fn read_from_user() -> (String, i32) {
        let mut user_team = String::new();
        let mut user_rank_input = String::new();
        let mut user_rank = 0;

        //read in target team name from user input
        print!("Desired team name: ");
        let _ = std::io::stdout().flush();
        match std::io::stdin().read_line(&mut user_team) {
            Ok(_len) => (),
            Err(error) => println!("error reading input: {error:?}"),
        }

        // trim off newline
        match user_team.pop() {
            Some(_char) => (),
            None => println!("please provide valid team name"),
        }

        //read in target rank from user input
        print!("Desired rank: ");
        let _ = std::io::stdout().flush();
        match std::io::stdin().read_line(&mut user_rank_input) {
            Ok(_len) => (),
            Err(error) => println!("error reading input: {error:?}"),
        }
        //convert rank input to int
        if user_rank_input.is_ascii() {
            match user_rank_input.trim().parse::<i32>() {
                Ok(integer) => user_rank = integer,
                Err(error) => println!("input could not be parsed as int: {error:?}"),
            }
        }

        (user_team, user_rank)
    }

    // hardcoded team and rank for testing
    //let target_team = "Brighton";
    //let target_rank = 7;

    let (target_team, target_rank) = read_from_user();

    // read in data
    let mut fixture_list = Vec::<league::Match>::new();
    let mut current_table = league::LeagueTable::new();
    league::read_standings(&mut current_table);
    league::read_fixtures(&mut fixture_list);

    // running tally instantiated as Arc holding Mutex to allow all threads to modify
    let final_count = Arc::new(Mutex::new(0));

    // spawn threads
    thread::scope(|s| {
        for _i in 0..NUM_THREADS {
            s.spawn(|| {
                let mut count = 0;
                for _j in 0..NUM_SIMULATIONS {
                    // if the target team achieves the target rank or better, add to the success tally
                    if league::run_simulation(&target_team, &current_table, &fixture_list)
                        <= target_rank
                    {
                        count += 1;
                    }
                }
                // access mutex to add this threads' count to the running total
                let mut final_count = final_count.lock().unwrap();
                *final_count += count;
            });
        }
    });

    // access final count mutex
    let useable_count = final_count.lock().unwrap();

    // calculate probability of success as total successes over total number of simulations * 100 to report as percent
    let outcome = *useable_count as f32 / (NUM_SIMULATIONS as f32 * NUM_THREADS as f32) * 100.0;

    println!(
        "Percent chance {} achieves rank {} or better: {}%",
        target_team, target_rank, outcome
    );
}
