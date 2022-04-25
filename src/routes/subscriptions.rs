use actix_web::{web, Responder};

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(form: web::Form<FormData>) -> impl Responder {
    format!("Email is {} and username is {}", form.email, form.name)
    // HttpResponse::Ok().finish()
}
