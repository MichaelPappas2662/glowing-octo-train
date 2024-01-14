use actix_web::{web, App, HttpServer, HttpResponse, Responder, http::StatusCode};
use jsonwebtoken::{encode, Header, EncodingKey, decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    // Claims here
}

async fn login() -> impl Responder {
    // Handle login, generate JWT
}

async fn oauth_callback(query: web::Query<OAuthQuery>) -> impl Responder {
    // Handle OAuth callback, exchange code for tokens, generate JWT
}

async fn protected_route() -> impl Responder {
    // This route requires JWT validation
}

fn main() {
    HttpServer::new(|| {
        App::new()
            .route("/login", web::get().to(login))
            .route("/oauth/callback", web::get().to(oauth_callback))
            .route("/protected", web::get().to(protected_route))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
