use super::database::Database;
use super::error_responses;
use super::mime;
use super::models::blockchain::Blockchain;

use actix_web::{get, http, post, web, App, HttpResponse, HttpServer, Responder};
use serde_json::Value;

#[derive(Debug)]
struct AppState(String);

pub async fn listen() -> std::io::Result<()> {
    let port = 8080;
    println!("listening on {}", port);

    let connection = Database::connect().expect("could not connect to database");

    let app_state = web::Data::new(AppState("Kamaal".to_string()));
    let app = move || {
        App::new()
            .app_data(app_state.clone())
            .service(hello)
            .service(get_blocks)
            .service(mine_blocks)
    };

    let server = HttpServer::new(app).bind(("127.0.0.1", port));
    server?.run().await
}

#[get("/")]
async fn hello(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok()
        .append_header((http::header::CONTENT_TYPE, mime::APPLICATION_JSON))
        .body(format!("{{\"hello\": \"{}\"}}", data.0))
}

#[get("/blocks")]
async fn get_blocks() -> impl Responder {
    let blockchain = Blockchain::new();
    let blocks = serde_json::to_string(&blockchain.blocks()).unwrap();

    HttpResponse::Ok()
        .append_header((http::header::CONTENT_TYPE, mime::APPLICATION_JSON))
        .body(blocks)
}

#[post("/blocks")]
async fn mine_blocks(request_body: String) -> impl Responder {
    if request_body.len() < 2 {
        println!("error: invalid payload");
        return error_responses::bad_request();
    }

    let request_body: Value = match serde_json::from_str(&request_body) {
        Err(_) => {
            println!("error: invalid payload");
            return error_responses::bad_request();
        }
        Ok(value) => value,
    };

    let data = match request_body.get("data") {
        None => {
            println!("error: invalid payload");
            return error_responses::bad_request();
        }
        Some(value) => match value.as_str() {
            None => {
                println!("error: invalid payload");
                return error_responses::bad_request();
            }
            Some(value) => value,
        },
    }
    .to_string();

    let mut blockchain = Blockchain::new();
    match blockchain.generate_next_block(data) {
        Err(err) => {
            println!("error: {}", err);
            return error_responses::bad_request();
        }
        Ok(()) => (),
    };

    HttpResponse::NoContent().body("")
}