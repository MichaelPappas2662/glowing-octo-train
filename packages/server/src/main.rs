use actix_web::{web, App, HttpResponse, HttpServer, Responder};

mod models;
mod schema;

async fn greet() -> impl Responder {
    HttpResponse::Ok().body("Hello from Rust Server!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server on http://127.0.0.1:3030");
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
    })
    .bind("127.0.0.1:3030")?
    .run()
    .await
}
