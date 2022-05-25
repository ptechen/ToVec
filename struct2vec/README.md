# ToVec
Rust structure to slice.

# example one:
    use struct2vec::ToVec;
    use struct2vec_derive::ToVec;
    use std::collections::HashMap;
    #[derive(ToVec, Debug, Clone, Deserialize, Serialize)]
    struct Custom {
        #[to_vec(comment = "用户名", field_type = "字段类型")]
        name: String,
    }

    let custom = Custom{name:String::from("123")};
    custom.to_vec()

# example two:
    use struct2vec::ToVec;
    use struct2vec_derive::ToVec;
    use std::collections::HashMap;
    use from_value_derive::From;
    #[derive(ToVec, Debug, Clone, Deserialize, Serialize)]
    struct Custom {
        #[to_vec(comment = "用户名", field_type = "字段类型")]
        name: String,
        data: Data
    }

    #[derive(From, Debug, Clone, Deserialize, Serialize)]
    struct Data {
        key: String
    }
    
    let custom = Custom{name:String::from("123")};
    custom.to_vec()
