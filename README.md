# ToVec
Rust structure to slice.
    
# example:
    use struct2vec::ToVec;
    use struct2vec_derive::ToVec;
    use std::collections::HashMap;
    #[derive(ToVec, Debug, Clone, Deserialize, Serialize)]
    pub struct Custom {
        name: String,
    }

    let custom = Custom{name:String::from("123")};
    custom.to_vec()