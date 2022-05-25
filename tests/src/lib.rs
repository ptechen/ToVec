#[cfg(test)]
mod tests {
    use crate::{Custom, Custom2, Data};
    use struct2vec::ToVec;
    #[test]
    fn it_works() {
        let custom = Custom {
            name: String::from("123"),
        };
        println!("{:?}", custom.to_vec());

        let custom = Custom2 {
            name: String::from("123"),
            data: Data {
                key: "321".to_string(),
            },
        };
        println!("{:?}", custom.to_vec());
    }
}

use from_value_derive::From;
use serde_json::value::Value;
use serde_json::Map;
use std::collections::HashMap;
use struct2vec::ToVec;
use struct2vec_derive::ToVec;

#[derive(ToVec, Debug, Clone)]
struct Custom {
    #[to_vec(comment = "用户名", field_type = "字段类型")]
    name: String,
}

#[derive(ToVec, Debug, Clone)]
struct Custom2 {
    #[to_vec(comment = "用户名", field_type = "字段类型")]
    name: String,
    data: Data,
}

#[derive(From, Debug, Clone)]
struct Data {
    key: String,
}
