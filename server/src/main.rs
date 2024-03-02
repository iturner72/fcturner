use actix_cors::Cors;
use actix_files as fs;
use actix_web::{web, App, HttpResponse, Responder, HttpServer, http::header, middleware::Logger};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref VOTES: Mutex<Vec<String>> = Mutex::new(vec![]);
}

#[derive(Serialize, Deserialize)]
struct Vote {
    option: String,
}

async fn serve_poll() -> impl Responder {
    HttpResponse::Ok().json(vec!["Green", "Purple", "Red", "Blue"])
}

async fn handle_vote(body: web::Json<Vote>) -> impl Responder {
    let vote = body.into_inner();
    let mut votes = VOTES.lock().unwrap();
    votes.push(vote.option);

    HttpResponse::Ok().json(serde_json::json!({"message": "Vote recorded"}))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("https://d330-192-145-119-230.ngrok-free.app")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT, header::CONTENT_TYPE])
            .max_age(3600);

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .service(web::resource("/poll")
                .route(web::get().to(serve_poll))
                .route(web::post().to(handle_vote))
            )

            .service(fs::Files::new("/static", "./public").show_files_listing())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
