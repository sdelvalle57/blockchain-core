
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
mod handler;


async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
pub async fn start_server() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(handler::hello)
            .service(handler::echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 7878))
    .expect("unable to start server")
    .run()
    .await
}

