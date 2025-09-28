use actix_web::{HttpResponse, Responder, get};

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world")
}

#[get("/name")]
pub async fn get_name() -> impl Responder {
    HttpResponse::Ok().body("Gaurang Bharadava")
}

pub async fn echo() -> impl Responder {
    HttpResponse::Ok().body("Echo")
}

pub async fn prefix_path_one() -> impl Responder {
    HttpResponse::Ok().body("Prefix Path One")
}

pub async fn prefix_path_two() -> impl Responder {
    HttpResponse::Ok().body("Prefix Path Two")
}
