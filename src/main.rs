use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use std::sync::Mutex;

struct AppStateWithCounter {
    counter: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads
}

async fn index(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap(); // <- get counter's MutexGuard
    *counter += 1; // <- access counter inside MutexGuard

    format!("Request number: {}", counter) // <- response with count
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello actix-web!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

/// extract path info from "/users/{user_id}/{friend}" url
/// {user_id} - deserializes to a u32
/// {friend} - deserializes to a String
#[get("/users/{user_id}/{friend}")] // <- define path parameters
async fn welcome(web::Path((user_id, friend)): web::Path<(u32, String)>) -> Result<String> {
    Ok(format!("Welcome {}, user_id {}!", friend, user_id))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            .service(
                web::scope("/counter")
                    .app_data(counter.clone())
                    .route("", web::get().to(index)),
            )
            .service(welcome)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
