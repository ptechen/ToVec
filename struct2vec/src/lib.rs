use serde_json::Value;
use std::collections::HashMap;

#[cfg(feature = "struct2vec_derive")]
#[allow(unused_imports)]
#[macro_use]
extern crate struct2vec_derive;
#[cfg(feature = "struct2vec_derive")]
#[doc(hidden)]
pub use struct2vec_derive::*;

pub trait ToVec {
    fn to_vec(&self) -> Vec<HashMap<String, Value>>;
}
