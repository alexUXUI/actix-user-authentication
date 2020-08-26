use actix_web::{ Responder, get, HttpResponse };

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String
}

#[get("/ping")]
pub async fn status() -> impl Responder {
    HttpResponse::Ok().json(HealthResponse {
        status: String::from("pong!")
    })
}