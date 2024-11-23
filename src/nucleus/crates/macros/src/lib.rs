use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(StateTrait)]
pub fn derive_state_trait(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let ident = input.ident;

    // TODO: maybe enforce that the fields should be of type State<T> or SharedState<T>
    // if let syn::Data::Struct(data_struct) = input.data {
    //     if let syn::Fields::Named(named_fields) = data_struct.fields {
    //         for field in named_fields.named {
    //             println!("fields: {:?}", field.ty);
    //         }
    //     }
    // }

    let expanded = quote! {
        impl StateTrait for #ident {
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}

#[proc_macro_derive(SharedStateTrait)]
pub fn derive_shared_state_trait(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let ident = input.ident;

    // TODO: maybe enforce that the fields should be of type State<T> or SharedState<T>
    // if let syn::Data::Struct(data_struct) = input.data {
    //     if let syn::Fields::Named(named_fields) = data_struct.fields {
    //         for field in named_fields.named {
    //             println!("fields: {:?}", field.ty);
    //         }
    //     }
    // }

    let expanded = quote! {
        impl SharedStateTrait for #ident {
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}
