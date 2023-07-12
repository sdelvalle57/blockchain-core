use actix_cors::Cors;
use actix_web::web::Data;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::{io, sync::Arc};

mod handler;
mod schema;

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
pub async fn start_server() -> io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let schema = Arc::new(schema::create_schema());

    log::info!("starting HTTP server on port 7878");
    log::info!("GraphiQL playground: http://localhost:7878/graphiql");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .supports_credentials();
        App::new()
            .wrap(cors)
            .app_data(Data::from(schema.clone()))
            .service(handler::hello)
            .service(handler::echo)
            .service(handler::graphql)
            .service(handler::graphql_playground)
            .route("/hey", web::get().to(manual_hello))
    })
    .workers(5)
    .bind(("127.0.0.1", 7878))
    .expect("unable to start server")
    .run()
    .await
}
