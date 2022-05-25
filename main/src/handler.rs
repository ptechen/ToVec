use axum::{routing::get, Json, Router};
use from_value_derive::From;
use serde_derive::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::collections::HashMap;
use struct2vec::ToVec;

pub fn app_router() -> Router {
    let app = Router::new()
        .route("/", get(index))
        .route("/to_vec", get(index_to_vec));
    app
}

#[derive(ToVec, Debug, Clone, Deserialize, Serialize)]
pub struct User {
    #[to_vec(comment = "ID")]
    id: usize,
    #[to_vec(comment = "用户名")]
    name: String,
    #[to_vec(comment = "收益", field_type = "金额")]
    profit: u8,
    active: bool,
    array: Vec<String>,
    #[to_vec(comment = "数据")]
    data: Vec<Custom>,
}

#[derive(From, Debug, Clone, Deserialize, Serialize)]
pub struct Custom {
    name: String,
}

pub async fn index() -> Json<User> {
    let st = Custom {
        name: String::from("123"),
    };
    let data = User {
        id: 1,
        name: String::from("123"),
        profit: 0,
        active: true,
        array: vec![String::from("321")],
        data: vec![st],
    };
    Json(data)
}

pub async fn index_to_vec() -> Json<Vec<HashMap<String, Value>>> {
    let st = Custom {
        name: "".to_string(),
    };
    let data = User {
        id: 1,
        name: String::from("123"),
        profit: 0,
        active: true,
        array: vec![String::from("321")],
        data: vec![st],
    };
    let data = data.to_vec();
    Json(data)
}
