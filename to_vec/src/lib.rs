use std::collections::HashMap;
use serde_json::Value;

pub trait ToVec {
    fn to_vec(&self) -> Vec<HashMap<String, Value>>;
}