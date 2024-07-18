use actix_web::{App, get, HttpResponse, HttpServer, Responder};
use std::env;


fn address() -> Result<String, env::VarError> {
    env::var("ADDRESS")
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let addr = match address() {
        Ok(addr) => addr,
        Err(e) => {
            eprintln!("Environment variable ADDRESS is not set: {}", e);
            std::process::exit(1); // Exit the program with an error code
        }
    };

    HttpServer::new(|| {
        App::new()
            .service(hello)
    })
    .bind(addr)?
    .run()
    .await
}