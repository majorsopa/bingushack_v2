use darling::FromDeriveInput;
use proc_macro::{self, TokenStream};
use quote::{quote, TokenStreamExt};
use syn::{parse_macro_input, DeriveInput, parse::Parser};
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
    settings_list_field_names: SettingsListHelper,
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

    let toggle_method= quote! {
        fn toggle(&mut self, _env: JNIEnv, _mappings_manager: Rc<MappingsManager>) {
            let new_val = !self.__enabled_bool_setting.get_value();
            *self.__enabled_bool_setting.get_value_mut() = new_val;
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

    let settings = {
        let get = quote! {&self};

        let mut settings_list = quote! {};

        let settings_list_field_names = opts.settings_list_field_names.inner.into_iter().map(|x| x.inner).collect::<Vec<_>>();

        for setting in settings_list_field_names {
            settings_list.extend(quote! {self.#setting,});
        }

        let settings = {
            quote!{
                fn get_settings(#get) -> Vec<BingusSetting> {
                    vec![#settings_list]
                }
            }
        };

        settings
    };

    let output = quote! {
        impl<T> BingusModuleTrait<T> for #ident {
            #get_name
            #toggle_method
            #defaults
            #settings
        }
    };

    output.into()
}

#[proc_macro_attribute]
pub fn add_bingus_fields(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(input as DeriveInput);
    match &mut ast.data {
        syn::Data::Struct(ref mut struct_data) => {
            match &mut struct_data.fields {
                syn::Fields::Named(fields) => {
                    fields
                        .named
                        .push(syn::Field::parse_named.parse2(
                            quote! {
                                __enabled_bool_setting: BingusSetting
                            }
                        ).unwrap());
                }   
                _ => {
                    ()
                }
            }              
            
            return quote! {
                #ast
            }.into();
        }
        _ => panic!("`add_bingus_fields` has to be used with structs "),
    }
}
