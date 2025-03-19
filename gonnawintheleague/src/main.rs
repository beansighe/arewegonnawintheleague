use gonnawintheleague as league;
use std::thread;
use std::sync::{Arc, Mutex};

//cast using as f32 to use as divisor
const NUM_SIMULATIONS: i32 = 4000;
const NUM_THREADS: u32 = 4;

fn main() {
    // STEPS
    // read in target team, target rank
    let target_team = "Brighton";
    let target_rank = 7;

    // read in data
    let mut fixture_list = Vec::<league::Match>::new();
    let mut current_table = league::LeagueTable::new();
    league::read_standings(&mut current_table);
    league::read_fixtures(&mut fixture_list);


    let final_count = Arc::new(Mutex::new(0));

    thread::scope(|s| {
        for _i in 0..NUM_THREADS {
            s.spawn(|| {
                let mut count = 0;
                for _j in 0..NUM_SIMULATIONS {
                    if league::run_simulation(target_team, &current_table, &fixture_list)
                        <= target_rank
                    {
                        count += 1;
                    }
                }
                let mut final_count = final_count.lock().unwrap();
                *final_count += count;
            });
        }
    });
    

    let useable_count = final_count.lock().unwrap();
    let outcome =  *useable_count as f32 / (NUM_SIMULATIONS as f32 * NUM_THREADS as f32) * 100.0;

    println!(
        "Percent chance {} achieves rank {} or better: {}%",
        target_team, target_rank, outcome
    );

}
