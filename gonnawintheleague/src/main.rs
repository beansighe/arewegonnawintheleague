//! This is a web app which attempts to provide the user with a
//! measure by which to gauge the potential outcome of the season
//! of a specific Premier League team by calculating a percent chance
//! of achieving a specific rank of better by the end of the season
//! given current standings and the remaining fixtures using a
//! Monte Carlo simulation.

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use askama::Template;
use gonnawintheleague as league;
use serde::Deserialize;
use std::sync::{Arc, Mutex};
use std::thread;

const NUM_SIMULATIONS: i32 = 4000;
const NUM_THREADS: u32 = 4;

/// This structure holds the current data
/// which will serve as the starting point
/// and the basis for the Monte Carlo simulation.
///
/// Treating it as app state data allows us
/// to only read the data and construct the structures once
struct AppStateWithData {
    standings: league::LeagueTable,
    fixtures: Vec<league::Match>,
}
#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    results: Option<&'a (i32, f32, String)>,
}

#[derive(Deserialize)]
struct FormData {
    team: String,
    rank: i32,
}

/// implements the landing page before any calculations have been done
async fn index() -> impl Responder {
    let blank_template = IndexTemplate { results: None };
    HttpResponse::Ok()
        .content_type("text/html")
        .body(blank_template.render().unwrap())
}

/// handles form processing, capturing and displaying of results
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
) -> f32 {
    // running tally instantiated as Arc holding Mutex to allow all threads to modify
    let final_count = Arc::new(Mutex::new(0));

    // spawn threads
    thread::scope(|s| {
        for _i in 0..NUM_THREADS {
            s.spawn(|| {
                let mut count = 0;
                for _j in 0..NUM_SIMULATIONS {
                    // if the target team achieves the target rank or better, add to the success tally
                    if league::run_simulation(target_team, standings, fixtures) <= target_rank {
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
    *useable_count as f32 / (NUM_SIMULATIONS as f32 * NUM_THREADS as f32) * 100.0
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
