extern crate proc_macro2;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};
use syn::spanned::Spanned;
use proc_macro2::{Ident, Span};

#[proc_macro_derive(ToVec, attributes(to_vec))]
pub fn to_vec_derive(input: TokenStream) -> TokenStream {
    let mut insert_tokens = vec![];
    let parsed_input: DeriveInput = parse_macro_input!(input);
    let struct_name = parsed_input.ident;
    match parsed_input.data {
        Data::Struct(s) => {
            if let Fields::Named(name_fields) = s.fields {
                let a = name_fields.named;
                for field in a {
                    let comment = get_filed_attr(&field, "comment");
                    let field_type = get_filed_attr(&field, "field_type");
                    let ty = get_filed_type(&field.ty);
                    let field = field.ident.as_ref().unwrap();
                    let mut comment_val = &Ident::new("_", Span::call_site());
                    let mut field_type_val = &Ident::new("_", Span::call_site());
                    if comment.is_some() {
                        comment_val = comment.as_ref().unwrap();
                    }
                    if field_type.is_some() {
                        field_type_val = field_type.as_ref().unwrap();
                    }
                    let insert_token;
                    if ty == "Option" {
                        insert_token = quote! {
                        let mut map = HashMap::new();
                        map.insert(String::from("key"), Value::from(stringify!(#field)));
                        let v:Value = serde_json::to_value(self.#field.as_ref()).unwrap();
                        map.insert(String::from("value"), Value::from(v));
                        if stringify!(#comment_val) != "_" {
                            map.insert(String::from("comment"), Value::from(stringify!(#comment_val)));
                        }
                        if stringify!(#field_type_val) != "_" {
                            map.insert(String::from("field_type"), Value::from(stringify!(#field_type_val)));
                        }
                        array.push(map);
                    };
                    } else {
                        insert_token = quote! {
                        let mut map = HashMap::new();
                        map.insert(String::from("key"), Value::from(stringify!(#field)));
                        map.insert(String::from("value"), Value::from(self.#field.to_owned()));
                        if stringify!(#comment_val) != "_" {
                            map.insert(String::from("comment"), Value::from(stringify!(#comment_val)));
                        }
                        if stringify!(#field_type_val) != "_" {
                            map.insert(String::from("field_type"), Value::from(stringify!(#field_type_val)));
                        }
                        array.push(map);
                    };
                    }
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

fn get_filed_attr(field: &syn::Field, sub_attr: &str) -> Option<syn::Ident> {
    for attr in field.attrs.iter() {
        if let Ok(syn::Meta::List(syn::MetaList {
                                      ref path,
                                      ref nested,
                                      ..
                                  })) = attr.parse_meta()

        {
            for seg in path.segments.iter() {
                if seg.ident == "to_vec" {
                    for nest in nested {
                        if let syn::NestedMeta::Meta(syn::Meta::NameValue(kv)) = nest {
                            if kv.path.is_ident(sub_attr) {
                                if let syn::Lit::Str(ref ident_str) = &kv.lit {
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
    }
    None
}

fn get_filed_type(ty: &syn::Type) -> String {
    if let syn::Type::Path(syn::TypePath {
                               ref path,
                               ..
                           }) = ty {
        if let Some(seg) = path.segments.last() {
            // eprintln!("{:#?}", &seg.ident.to_string());
            return seg.ident.to_string();
            // if seg.ident == outer_ident_name {
            //     if let syn::PathArguments::AngleBracketed(
            //         syn::AngleBracketedGenericArguments {
            //             args,
            //             ..
            //         }) = &seg.arguments {
            //         if let Some(syn::GenericArgument::Type(inner_type)) = args.first() {
            //             return Some(inner_type);
            //         }
            //     }
            // }
        }
    }
    String::new()
}