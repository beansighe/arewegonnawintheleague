use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use askama::Template;
use gonnawintheleague as league;
use serde::Deserialize;
use std::sync::{Arc, Mutex};
use std::thread;

//cast using as f32 to use as divisor
const NUM_SIMULATIONS: i32 = 4000;
const NUM_THREADS: u32 = 4;

struct AppStateWithData {
    standings: league::LeagueTable,
    fixtures: Vec<league::Match>,
}
#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    results: Option<&'a (i32, (f32, i32), String)>,
}

#[derive(Deserialize)]
struct FormData {
    team: String,
    rank: i32,
}

async fn index() -> impl Responder {
    let blank_template = IndexTemplate { results: None };
    HttpResponse::Ok()
        .content_type("text/html")
        .body(blank_template.render().unwrap())
}

async fn submit(form: web::Form<FormData>, data: web::Data<AppStateWithData>) -> impl Responder {
    let team = form.team.clone();
    let rank = form.rank;
    let (standings, fixtures) = (&data.standings, &data.fixtures);
    let computed_results = (
        rank,
        calculate_results(&team, rank, standings, fixtures),
        team,
    );
    let results_template = IndexTemplate {
        results: Some(&computed_results),
    };
    HttpResponse::Ok()
        .content_type("text/html")
        .body(results_template.render().unwrap())
}

pub fn calculate_results(
    target_team: &str,
    target_rank: i32,
    standings: &league::LeagueTable,
    fixtures: &Vec<league::Match>,
) -> (f32, i32) {
    // running tally instantiated as Arc holding Mutex to allow all threads to modify
    let final_count = Arc::new(Mutex::new(0));
    //let min_wins = Arc::new(Mutex::new(0));
    let total_wins = Arc::new(Mutex::new(0));
    //let target_count = Arc::new(Mutex::new(0));

    // spawn threads
    thread::scope(|s| {
        for _i in 0..NUM_THREADS {
            s.spawn(|| {
                let mut count = 0;
                //let mut curr_min = 38;
                let mut thread_wins = 0;
                //let mut target_count_thread = 0;
                for _j in 0..NUM_SIMULATIONS {
                    // if the target team achieves the target rank or better, add to the success tally
                    let (rank, wins) = league::run_simulation(target_team, standings, fixtures);
                    if rank <= target_rank {
                        count += 1;
                        thread_wins += wins;
                        //target_count_thread += 1;
                    }
                    /*if wins < curr_min {
                        curr_min = wins;
                    }
                    */
                    // }
                }
                // access mutex to add this threads' count to the running total
                let mut final_count = final_count.lock().unwrap();
                *final_count += count;
                //let mut min_wins = min_wins.lock().unwrap();
                //*min_wins = curr_min;
                let mut total_wins = total_wins.lock().unwrap();
                *total_wins += thread_wins;
                //let mut target_count = target_count.lock().unwrap();
                //*target_count += target_count_thread;
            });
        }
    });

    // access final count mutex
    let final_count = final_count.lock().unwrap();
    //let min_wins = min_wins.lock().unwrap();
    let total_wins = total_wins.lock().unwrap();
    //let target_count = target_count.lock().unwrap();

    // calculate probability of success as total successes over total number of simulations * 100 to report as percent
    if *final_count > 0 {
        (
            *final_count as f32 / (NUM_SIMULATIONS as f32 * NUM_THREADS as f32) * 100.0,
            *total_wins / *final_count,
        )
    } else {
        (0.0, 0)
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // read in data
    let mut fixture_list = Vec::<league::Match>::new();
    let mut current_table = league::LeagueTable::new();
    league::read_standings(&mut current_table);
    league::read_fixtures(&mut fixture_list);
    let state_data = web::Data::new(AppStateWithData {
        standings: current_table,
        fixtures: fixture_list,
    });

    HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(index))
            .app_data(state_data.clone())
            .route("/submit", web::post().to(submit))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
