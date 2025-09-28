use actix_web::{guard, web, App, HttpResponse, HttpServer};
use std::sync::Mutex;

mod routes;
mod routes2;
use routes::*;
use routes2::*;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // for routes 2 for mutable data
    let counter = web::Data::new(MutData {
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        App::new()
            .service(hello)
            .service(get_name)
            .route("/echo", web::get().to(echo))
            .service(
                web::scope("/app")
                    .route("/index.html", web::get().to(prefix_path_one))
                    .route("/ok.html", web::get().to(prefix_path_two)),
            )
            .app_data(web::Data::new(AppState {
                app_name: String::from("Actix Web"),
            }))
            .service(appstate)
            .app_data(counter.clone())
            .route("/mutdata", web::get().to(mutdata))
            .service(
                web::scope("/guard")
                    .guard(guard::Host("www.rust-lang.org"))
                    .route("", web::to(|| async {
                        HttpResponse::Ok().body("www")
                    }))
            )
            .service(
                web::scope("/second")
                    .guard(guard::Host("users.rust-lang.org"))
                    .route("", web::to(|| async {
                        HttpResponse::Ok().body("user")
                    }))
            )
            .route("/", web::to(HttpResponse::Ok))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
