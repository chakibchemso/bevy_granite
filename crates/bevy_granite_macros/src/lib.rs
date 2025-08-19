use once_cell::sync::Lazy;
use proc_macro::TokenStream;
use quote::quote;
use std::sync::Mutex;
use syn::{parse_macro_input, DeriveInput};

static REGISTERED_COMPONENTS: Lazy<Mutex<Vec<(String, bool)>>> =
    Lazy::new(|| Mutex::new(Vec::new()));

use std::sync::atomic::{AtomicBool, Ordering};

static IMPORTS_ADDED: AtomicBool = AtomicBool::new(false);

#[proc_macro_attribute]
pub fn granite_component(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;
    let name_str = name.to_string();
    println!("MACRO: Registering component: {}", name_str);
    let attr_str = attr.to_string();
    let include_default = attr_str.contains("default");
    let is_hidden = attr_str.contains("ui_hidden");
    REGISTERED_COMPONENTS
        .lock()
        .unwrap()
        .push((name_str.clone(), is_hidden));
    let derives = if include_default {
        quote! {
            #[derive(Reflect, Serialize, Deserialize, Debug, Clone, Component, PartialEq)]
        }
    } else {
        quote! {
            #[derive(Reflect, Serialize, Deserialize, Debug, Clone, Component, Default, PartialEq)]
        }
    };
    // Only add imports on the first use
    let needs_imports = !IMPORTS_ADDED.swap(true, Ordering::Relaxed);
    let imports = if needs_imports {
        quote! {
            #[warn(unused_imports)]
            use bevy::prelude::{Component,ReflectFromReflect, ReflectDefault, ReflectDeserialize, ReflectSerialize, ReflectComponent};
            #[warn(unused_imports)]
            use bevy::reflect::{Reflect, FromReflect};
            #[warn(unused_imports)]
            use serde::{Serialize, Deserialize};
        }
    } else {
        quote! {}
    };
    let expanded = quote! {
        #imports
        #derives
        #[reflect(Component, Serialize, Deserialize, Default, FromReflect)]
        #input
    };
    TokenStream::from(expanded)
}

#[proc_macro]
pub fn register_editor_components(input: TokenStream) -> TokenStream {
    let app_name = if input.is_empty() {
        quote!(app)
    } else {
        let parsed = parse_macro_input!(input as syn::Ident);
        quote!(#parsed)
    };

    let components = REGISTERED_COMPONENTS.lock().unwrap();
    let tokens = components.iter().map(|(name, is_hidden)| {
        let ident = syn::Ident::new(name, proc_macro2::Span::call_site());

        if *is_hidden {
            quote! {
                #app_name.register_type::<#ident>();
            }
        } else {
            quote! {
                #app_name.register_type::<#ident>();
                #app_name.register_type_data::<#ident, bevy_granite::prelude::BridgeTag>();
            }
        }
    });

    let expanded = quote! {
        {
            #(#tokens)*
        }
    };
    TokenStream::from(expanded)
}
