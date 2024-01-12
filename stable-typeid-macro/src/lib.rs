use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Ident, Type, Visibility};
mod sort;

fn hash(s: &str) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    hasher.finish()
}
#[proc_macro_attribute]
pub fn sort_struct(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // 解析为结构体
    let input = parse_macro_input!(item as DeriveInput);
    // struct名称
    let name = input.ident;
    // struct vis
    let main_vis = input.vis;
    // struct generic
    let main_generic = input.generics;
    // struct字段
    let fields = match input.data {
        Data::Struct(data) => match data.fields {
            Fields::Named(fields) => fields,
            _ => panic!("Expected named fields"),
        },
        _ => panic!("Expected a struct"),
    };
    let mut field_vec = Vec::new();
    let mut viss: Vec<Visibility> = Vec::new();
    for field in fields.named {
        let ident = field.ident.unwrap();
        let ty = field.ty;
        let vis = field.vis;
        field_vec.push((ident, ty));
        viss.push(vis);
    }
    // 排序
    field_vec.sort_by_key(|(ident, _)| hash(&ident.to_string()));
    let mut idents: Vec<Ident> = Vec::new();
    let mut tys: Vec<Type> = Vec::new();
    for field in field_vec {
        idents.push(field.0);
        tys.push(field.1);
    }

    let expanded = quote! {
        #main_vis struct #name #main_generic {
            #(#viss #idents: #tys),*
        }
    };
    TokenStream::from(expanded)
}

#[proc_macro_derive(StableID)]
pub fn stable_id(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let fields = match input.data {
        Data::Struct(data) => match data.fields {
            Fields::Named(fields) => fields,
            _ => panic!("Expected named fields"),
        },
        _ => panic!("Expected a struct"),
    };
    let mut field_strings: Vec<String> = Vec::new();
    for field in fields.named {
        let ident = field.ident.unwrap().to_string();
        let ty = field.ty;
        let ty = quote! {#ty};
        let ty = ty.to_string();
        field_strings.push(format!("{}${};", ident, ty));
    }
    let hash = hash(&field_strings.join(""));
    let expanded = quote! {
        impl StableAny for #name {
            fn type_id(&self) -> &'static StableId where Self: Sized {
                &StableId(#hash)
            }
        }

        impl StableID for #name {
            const _STABLE_ID: &'static StableId = &StableId(#hash);
        }
    };
    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn stable_type(attr: TokenStream, item: TokenStream) -> TokenStream {
    let sort = sort_struct(attr, item);
    let stable = stable_id(sort.clone());
    let sort = proc_macro2::TokenStream::from(sort);
    let stable = proc_macro2::TokenStream::from(stable);
    let expanded = quote! {
        #sort
        #stable
    };
    TokenStream::from(expanded)
}
