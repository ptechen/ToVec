#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};
use syn::spanned::Spanned;

#[proc_macro_derive(ToVec,attributes(to_vec))]
pub fn to_vec_derive(input: TokenStream) -> TokenStream {
    let mut insert_tokens = vec![];
    let parsed_input: DeriveInput = parse_macro_input!(input);
    let struct_name = parsed_input.ident;
    match parsed_input.data {
        Data::Struct(s) => {
            if let Fields::Named(name_fields) = s.fields {
                let a = name_fields.named;
                for i in a {
                    let field = i.ident.as_ref().unwrap();
                    let comment = get_filed_attr(&i);
                    let mut comment_val = field;
                    if comment.is_some() {
                        comment_val = comment.as_ref().unwrap();
                    }
                    let insert_token = quote! {
                        let mut map = HashMap::new();
                        map.insert(String::from("key"), Value::from(stringify!(#field)));
                        map.insert(String::from("value"), Value::from(self.#field.to_owned()));
                        if stringify!(#comment_val) != stringify!(#field) {
                            map.insert(String::from("comment"), Value::from(stringify!(#comment_val)));
                        }
                        array.push(map);
                    };
                    insert_tokens.push(insert_token);
                }
            }
        }
        other => { panic!("ToVec is not yet implemented for: {:?} ", other) }
    }
    let tokens = quote! {
        impl ToVec for #struct_name {
            fn to_vec(&self) -> Vec<HashMap<String, Value>> {
                let mut array = vec![];
                #(#insert_tokens)*
                array
            }
        }
    };
    proc_macro::TokenStream::from(tokens)
}

fn get_filed_attr(field: &syn::Field) ->Option<syn::Ident> {
    for attr in field.attrs.iter() {
        if let Ok(syn::Meta::List(syn::MetaList {
            ref path,
            ref nested,
            ..
        })) = attr.parse_meta()

        {
            if let Some(p) = path.segments.first() {
                if p.ident == "to_vec" {
                    if let Some(syn::NestedMeta::Meta(syn::Meta::NameValue(kv))) = nested.first(){
                        if kv.path.is_ident("comment") {
                            if let syn::Lit::Str(ref ident_str) = kv.lit {
                                return Some(syn::Ident::new(
                                    ident_str.value().as_str(),
                                    attr.span(),
                                ));
                            }
                        }
                    }
                }
            }
        }
    }
    None
}