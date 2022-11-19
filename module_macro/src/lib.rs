use darling::FromDeriveInput;
use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};


#[derive(FromDeriveInput)]
#[darling(attributes(bingus_module))]
struct Opts {
    name: String,
}

#[proc_macro_derive(BingusModule, attributes(bingus_module))]
pub fn derive_bingus_module(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    let opts = Opts::from_derive_input(&input).expect("error with macro derive inputs");
    let DeriveInput { ident, .. } = input;

    // todo: settings
    let name = opts.name;

    let get_name = quote! {
        fn get_name() -> String {
            #name
        }
    };


    let defaults = {
        let defaults_args = quote! {&mut self, _env: JNIEnv, _mappings_manager: Rc<MappingsManager>};

        let defaults = quote! {
            fn tick(#defaults_args) {}
            fn on_enable(#defaults_args) {}
            fn on_disable(#defaults_args) {}
            fn on_load(#defaults_args) {}
            fn on_unload(#defaults_args) {}
        };

        defaults
    };

    let output = quote! {
        impl BingusModule for #ident {
            #get_name
            #defaults
        }
    };

    output.into()
}
