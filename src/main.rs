use actix_web::{post, HttpResponse, HttpServer, Responder};

pub mod models;
pub mod schema;
pub mod sign_up;

#[post("/login")]
pub async fn login_student() -> impl Responder {
    HttpResponse::Ok().body("Login successfull")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {});
}
