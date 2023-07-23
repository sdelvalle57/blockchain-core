use actix_web::{get, HttpResponse, Responder, web, route};
use juniper::http::GraphQLRequest;
use juniper::http::graphiql::graphiql_source;
use actix_web_lab::respond::Html;
use crate::server::schema::Schema;


/// GraphiQL playground UI
/// check example here https://github.com/actix/examples/blob/master/graphql/juniper/README.md
#[get("/graphiql")]
pub async fn graphql_playground() -> impl Responder {
    Html(graphiql_source("/graphql", None))
}

/// GraphQL endpoint
#[route("/graphql", method = "GET", method = "POST")]
pub async fn graphql(st: web::Data<Schema>, data: web::Json<GraphQLRequest>) -> impl Responder {
    let user = data.execute(&st, &()).await;
    HttpResponse::Ok().json(user)
}
