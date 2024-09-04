use actix_web::{post, web::Json, App, HttpResponse, HttpServer, Responder};
use models::{Login, StudentInit};
use sign_up::{first_years_init::first_years_init, get_data::get_data};

pub mod models;
pub mod schema;
pub mod sign_up;

#[post("/login")]
pub async fn login_student(data: Json<Login>) -> impl Responder {
    let reg_no = data.reg_no.clone().to_ascii_uppercase();
    let password = data.password.clone();
    println!("{}: {}", reg_no, password);
    let result = get_data(reg_no, password);
    HttpResponse::Ok().body(result)
}
#[post("/signUp")]
pub async fn sign_up_student(data: Json<StudentInit>) -> impl Responder {
    let reg_no = data.reg_no.clone();
    let password = data.password.clone();
    let first_name = data.first_name.clone();
    let last_name = data.last_name.clone();
    let middle_name = data.middle_name.clone();
    let year_of_study = data.year_of_study;
    let semester = data.semester;
    let programme = data.programme.clone();
    let course = data.course.clone();
    let admission_year = data.admission_year;
    let result = first_years_init(
        reg_no,
        password,
        first_name,
        last_name,
        middle_name,
        year_of_study,
        semester,
        programme,
        course,
        admission_year,
    );
    HttpResponse::Ok().body(result)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(login_student).service(sign_up_student))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
