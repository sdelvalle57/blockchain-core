use actix_web::{get, post, HttpResponse, Responder, web};
use serde::{Deserialize, Serialize};
use serde_json;

use crate::db_handler;
use crate::common::validator::{self, Message, validate_signature};

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
        }
    }
    
    HttpResponse::Ok().body(format!("message"))
}
