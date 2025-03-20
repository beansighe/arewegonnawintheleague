use gonnawintheleague as league;
use std::sync::{Arc, Mutex};
use std::thread;
use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use askama::Template;
use serde::Deserialize;

//cast using as f32 to use as divisor
const NUM_SIMULATIONS: i32 = 4000;
const NUM_THREADS: u32 = 4;

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

async fn index() -> impl Responder {
    let blank_template = IndexTemplate { results: None};
    HttpResponse::Ok()
        .content_type("text/html")
        .body(blank_template.render().unwrap())

}

async fn submit(form: web::Form<FormData>) -> impl Responder {
    let team = form.team.clone();
    let rank = form.rank;
    let computed_results = (rank, 49.8, team);
    let results_template = IndexTemplate { results: Some(&computed_results) };
    HttpResponse::Ok()
        .content_type("text/html")
        .body(results_template.render().unwrap())
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .route("/", web::get().to(index))
        .route("/submit", web::post().to(submit))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
    
    // STEPS
    // read in target team, target rank

    // hardcoded team and rank for testing
    /*let target_team = "Brighton";
    let target_rank = 7;


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
    */
}
