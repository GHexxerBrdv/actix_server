use actix_web::{HttpResponse, Responder, get, web};
use std::sync::Mutex;

pub struct AppState {
    pub app_name: String,
}

pub struct MutData {
    pub counter: Mutex<u32>,
}

#[get("/appstate")]
pub async fn appstate(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().body(format!("App Name: {}", data.app_name))
}

pub async fn mutdata(data: web::Data<MutData>) -> String {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;
    format!("Counter: {}", counter)
}
