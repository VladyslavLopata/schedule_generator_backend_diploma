use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Request {
    email: String,
    password: String,
}

#[derive(Serialize)]
pub struct Response {
    #[serde(rename = "type")]
    tp: String,
    result: bool,
}

#[post("/login")]
pub async fn login(request: web::Json<Request>) -> impl Responder {
    if request.email.len() > 0 && request.password.len() > 0 {
        return HttpResponse::Ok().json(Response {
            result: true,
            tp: String::from("admin"),
        });
    }
    HttpResponse::Ok().json(Response {
        result: false,
        tp: String::new(),
    })
}
