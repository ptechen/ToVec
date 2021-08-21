# ToVec
Rust structure to slice.
    
# example:
    use to_vec::ToVec;
    use to_vec_derive::ToVec;
    use std::collections::HashMap;
    #[derive(ToVec, Debug, Clone, Deserialize, Serialize)]
    pub struct Custom {
        name: String,
    }

    let custom = Custom{name:String::from("123")};
    custom.to_vec()