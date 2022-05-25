use serde_json::Value;
use std::collections::HashMap;

#[cfg(feature = "struct2vec_derive")]
pub use struct2vec_derive::ToVec;

pub trait ToVec {
    fn to_vec(&self) -> Vec<HashMap<String, Value>>;
}
