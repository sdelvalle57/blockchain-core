use actix_web::{get, post, HttpResponse, Responder, web, route};
use juniper::http::GraphQLRequest;
use juniper::http::graphiql::graphiql_source;
use serde::{Deserialize};
use serde_json;
use actix_web_lab::respond::Html;

use crate::db_handler;
use crate::common::validator::{Message, validate_signature};

use crate::server::schema::Schema;


#[derive(Deserialize)]
pub struct RpcCall {
    sender: String,
    message: Vec<String>,
    signature: String
}


impl TryFrom<RpcCall> for Message {
    type Error = serde_json::Error;
    fn try_from(value: RpcCall) -> Result<Self, Self::Error> {
        let json_value = serde_json::to_value(value.message)?;
        Ok(
            Message { 
                message: json_value.to_string(), 
                signature: value.signature, 
                sender: value.sender
            }
        )
    }
}


#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    println!("{}", req_body);
    HttpResponse::Ok().body(req_body)
}

#[post("/init_blockchain")]
pub async fn init_blockchain(form: web::Form<RpcCall>) -> HttpResponse {
    println!("Init blockchain");
    let db_count = db_handler::get_count();
    if db_count > 0 {
        HttpResponse::MethodNotAllowed().body("Blockchain already initialized");
    }

    match Message::try_from(form.into_inner()) {
        Err(err) => return HttpResponse::BadRequest().body(err.to_string()),
        Ok(message) => {
            if let Err(err) = validate_signature(&message) {
                return HttpResponse::BadRequest().body(err.to_string())
            }
            //TODO: init blockchain
        }
    }
    
    HttpResponse::Ok().body(format!("message"))
}


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
