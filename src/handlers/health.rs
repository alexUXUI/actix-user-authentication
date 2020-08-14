use actix_web::{ Responder, get, HttpResponse };

#[get("/ping")]
pub async fn status() -> impl Responder {
    HttpResponse::Ok().body("pong 🏓")
}