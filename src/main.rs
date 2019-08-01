#[macro_use]
extern crate log;

use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;

fn healthz(req: HttpRequest) -> impl Responder {
    let path:&str = req.path();
    format!("we are good on {}.", path)
}

fn next_sucker(
    state: web::Data<Mutex<Vec<String>>>, _req: HttpRequest) -> HttpResponse {
    let mut suckers = state.lock().unwrap();
    let next = suckers.get(0).cloned();
    suckers.rotate_left(1);
    match next {
        Some(person) => HttpResponse::Ok().body(
            format!("Next Sucker is : {}\n", person)),
        None => HttpResponse::InternalServerError().body(
            format!("Next Sucker is : Robert K")),
    }
}

fn main() {
    env_logger::init();
    info!("[main] this is an info log msg");

    let people = web::Data::new(
        Mutex::new(
            vec!["A".to_string(),
                 "B".to_string(),
                 "C".to_string(),
                 "D".to_string()]));
    
    HttpServer::new(move || {
        App::new()
            .register_data(people.clone())
            .route("/healthz", web::get().to(healthz))
            .service(web::resource("/next").to(next_sucker))
    })
        .bind("127.0.0.1:28000")
        .expect("Can not bind to port 28000")
        .run()
        .unwrap()
}

