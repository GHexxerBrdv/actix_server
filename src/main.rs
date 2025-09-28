use actix_web::{
    App, HttpRequest, HttpResponse, HttpServer, Responder, Result, error, get,
    http::{StatusCode, header::ContentType},
    post, web,
};
use derive_more::derive::{Display, Error};
use futures::{future::ok, stream::once};
use serde::Deserialize;
use std::io::Error;

#[derive(Debug, Deserialize)]
struct Info {
    user_id: u32,
    name: String,
}

#[derive(Deserialize)]
struct QueryInfo {
    username: String,
}

#[get("/users/{user_id}/{friend}")]
async fn index(path: web::Path<(u32, String)>) -> Result<String> {
    let (user_id, friend) = path.into_inner();
    Ok(format!("Welcome {}, user_id {}", friend, user_id))
}

#[get("/users/struct/{user_id}/{name}")]
async fn get_name(path: web::Path<Info>) -> Result<String> {
    Ok(format!("Welcome {}, user_id {}", path.name, path.user_id))
}

#[get("/query")]
async fn get_query(info: web::Query<QueryInfo>) -> impl Responder {
    HttpResponse::Ok().body(format!("welcome {}", info.username))
}

#[post("/json")]
async fn get_json(info: web::Json<Info>) -> impl Responder {
    HttpResponse::Ok().body(format!("welcome {} user id {}", info.name, info.user_id))
}

#[get("/greet")]
async fn greet() -> impl Responder {
    HttpResponse::Ok().body("Hello from gaurang")
}

#[get("/handler/one")]
async fn handler1(_req: HttpRequest) -> &'static str {
    "Hello from gaurang"
}

#[get("/stream")]
async fn stream_data() -> HttpResponse {
    let body = once(ok::<_, Error>(web::Bytes::from_static(b"test")));

    HttpResponse::Ok()
        .content_type("application/json")
        .streaming(body)
}

// -- custom error handling --

#[derive(Debug, Display, Error)]
enum MyError {
    #[display("internal error")]
    InternalError,
    #[display("bad request")]
    BadClientData,
    #[display("timeout")]
    Timeout,
}

impl error::ResponseError for MyError {
    
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }
    
    fn status_code(&self) -> StatusCode {
        match *self {
            MyError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::BadClientData => StatusCode::BAD_REQUEST,
            MyError::Timeout => StatusCode::GATEWAY_TIMEOUT,
        }
    }
}

#[get("/customerror")]
async fn custom_error() -> Result<&'static str, MyError> {
    Err(MyError::Timeout)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(greet)
            .service(get_name)
            .service(get_query)
            .service(get_json)
            .service(stream_data)
            .service(custom_error)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
