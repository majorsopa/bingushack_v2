use darling::FromDeriveInput;
use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};
use macro_common::*;






#[derive(FromDeriveInput, Default)]
#[darling(attributes(bingus_module), default)]
struct Opts {
    name: StringHelper,
    tick_method: FnHelper,
    on_enable_method: FnHelper,
    on_disable_method: FnHelper,
    on_load_method: FnHelper,
    on_unload_method: FnHelper,

    settings: Vec<BingusS>,
}

#[proc_macro_derive(BingusModuleTrait, attributes(bingus_module))]
pub fn derive_bingus_module(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    let opts = Opts::from_derive_input(&input).expect("error with macro derive inputs");
    let DeriveInput { ident, .. } = input;


    // todo: settings
    let name = opts.name.inner;

    let get_name = quote! {
        fn get_name(&self) -> &'static str {
            #name
        }
    };


    let defaults = {
        let defaults_args = quote! {&mut self, _env: JNIEnv, _mappings_manager: Rc<MappingsManager>};

        let tick_method = {
            let matched = opts.tick_method.inner;
            quote! {
                fn tick(#defaults_args) {
                    #matched;
                }
            }
        };
        let on_enable_method = {
            let matched = opts.on_enable_method.inner;
            quote! {
                fn on_enable(#defaults_args) {
                    #matched;
                }
            }
        };
        let on_disable_method = {
            let matched = opts.on_disable_method.inner;
            quote! {
                fn on_disable(#defaults_args) {
                    #matched;
                }
            }
        };
        let on_load_method = {
            let matched = opts.on_load_method.inner;
            quote! {
                fn on_load(#defaults_args) {
                    #matched;
                }
            }
        };
        let on_unload_method = {
            let matched = opts.on_unload_method.inner;
            quote! {
                fn on_unload(#defaults_args) {
                    #matched;
                }
            }
        };

        let defaults = quote! {
            #tick_method
            #on_enable_method
            #on_disable_method
            #on_load_method
            #on_unload_method
        };

        defaults
    };

    let output = quote! {
        impl BingusModuleTrait for #ident {
            #get_name
            #defaults
        }
    };

    output.into()
}
