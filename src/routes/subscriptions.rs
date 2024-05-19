use actix_web::web;
use actix_web::HttpResponse;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(form: web::Form<FormData>) -> HttpResponse {
    println!("{} {}", form.name, form.email);
    HttpResponse::Ok().finish()
}
