use to_vec_derive::ToVec;
use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse};
use serde_derive::{ Deserialize, Serialize};
use serde_json::{Value, Map};
use to_vec::ToVec;
use std::collections::HashMap;

#[derive(ToVec, Debug, Clone, Deserialize, Serialize)]
pub struct User {
    id: usize,
    name: String,
    active: bool,
    array: Vec<String>,
    st: Vec<Custom>,
}

#[derive(ToVec, Debug, Clone, Deserialize, Serialize)]
pub struct Custom
{
    name: String,
}

impl From<Custom> for Value{
    fn from(params: Custom) -> Self {
        let mut map = Map::new();
        map.insert("name".to_string(), Value::from(params.name));
        Value::Object(map)
    }
}

async fn greet(_req: HttpRequest) -> impl Responder {
    let st = Custom{name:String::from("123")};
    let name = User{id: 1, name: String::from("123"), active: true, array: vec![String::from("321")], st: vec![st]};
    HttpResponse::Ok().json(name.to_vec())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}