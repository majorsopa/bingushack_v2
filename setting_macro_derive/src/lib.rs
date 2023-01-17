use darling::FromDeriveInput;
use macro_common::*;
use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};



#[derive(FromDeriveInput)]
#[darling(attributes(bingus_setting))]
struct Opts {
    setting_type: TypeHelper,
}

#[proc_macro_derive(BingusSettingTrait, attributes(bingus_setting))]
pub fn derive_bingus_setting(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    let opts = Opts::from_derive_input(&input).expect("error with macro derive inputs");
    let DeriveInput { ident, .. } = input;


    let setting_type = opts.setting_type.inner;

    let get_value = quote! {
        fn get_value(&self) -> #setting_type {
            self.0
        }
    };

    let get_value_mut = quote! {
        fn get_value_mut(&mut self) -> &mut #setting_type {
            &mut self.0
        }
    };

    let output = quote! {
        impl BingusSettingTrait<#setting_type> for #ident {
            #get_value
            #get_value_mut
        }
    };

    output.into()
}