use darling::FromDeriveInput;
use proc_macro::{self, TokenStream};
use quote::{quote};
use syn::{parse_macro_input, DeriveInput, parse::Parser};
use macro_common::*;






#[derive(FromDeriveInput, Default)]
#[darling(attributes(bingus_module), default)]
struct Opts {
    name: StringHelper,
    tick_method: FnHelper,
    render_method: FnHelper,
    on_enable_method: FnHelper,
    on_disable_method: FnHelper,
    on_load_method: FnHelper,
    on_unload_method: FnHelper,
    settings_list_fields: SettingsListHelper,
}

#[proc_macro_derive(BingusModuleTrait, attributes(bingus_module))]
pub fn derive_bingus_module(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    let opts = Opts::from_derive_input(&input).expect("error with macro derive inputs");
    let DeriveInput { ident, .. } = input;


    let init = quote! {
        fn init(&mut self, jni_env: JNIEnv<'static>, mappings_manager: &mut Arc<MappingsManager<'static>>) {
            let jni_env: *mut JNIEnv<'static> = Box::into_raw(Box::new(jni_env));
            self.__env = Some(AtomicPtr::new(jni_env));
            let mappings_manager: *mut MappingsManager<'static> = unsafe { Arc::get_mut_unchecked(mappings_manager) };  // bruh
            self.__mappings_manager = Some(Arc::new(AtomicPtr::new(mappings_manager)));
        }
    };

    let name = opts.name.inner;

    let get_name = quote! {
        fn get_name(&self) -> &'static str {
            #name
        }
    };

    let defaults_args = quote! {&mut self};

    let toggle_method= quote! {
        fn toggle(#defaults_args) {
            let new_val = !*self.__enabled_bool_setting.0.get_bool();
            *self.__enabled_bool_setting.0.get_bool() = new_val;
        }
    };

    let unwrapped = quote! {
        let _env = unsafe {
            let matched = match self.__env {
                Some(ref env) => env,
                None => return,
            };
            *matched.load(std::sync::atomic::Ordering::Relaxed)
        };
        let _mappings_manager = unsafe {
            let matched = match self.__mappings_manager {
                Some(ref mappings_manager) => mappings_manager,
                None => return,
            };
            let loaded = matched.load(std::sync::atomic::Ordering::Relaxed);
            loaded.as_ref().unwrap()
        };
    };

    let defaults = {
        let tick_method = {
            let matched = opts.tick_method.inner;
            quote! {
                fn tick(#defaults_args) {
                    #unwrapped;
                    #matched;
                }
            }
        };
        let render_method = {
            let matched = opts.render_method.inner;
            quote! {
                fn render(&mut self) {
                    #unwrapped;
                    *PROJECTION_MATRIX.lock().unwrap() = Some(get_matrix_16(_env, get_projection_class_mapping(_env, _mappings_manager)));
                    *MODELVIEW_MATRIX.lock().unwrap() = Some(get_matrix_16(_env, get_modelview_class_mapping(_env, _mappings_manager)));
                    #matched;
                }
            }
        };
        let on_enable_method = {
            let matched = opts.on_enable_method.inner;
            quote! {
                fn on_enable(#defaults_args) {
                    #unwrapped;
                    #matched;
                }
            }
        };
        let on_disable_method = {
            let matched = opts.on_disable_method.inner;
            quote! {
                fn on_disable(#defaults_args) {
                    #unwrapped;
                    #matched;
                }
            }
        };
        let on_load_method = {
            let matched = opts.on_load_method.inner;
            quote! {
                fn on_load(#defaults_args) {
                    #unwrapped;
                    #matched;
                }
            }
        };
        let on_unload_method = {
            let matched = opts.on_unload_method.inner;
            quote! {
                fn on_unload(#defaults_args) {
                    #unwrapped;
                    #matched;
                }
            }
        };

        let defaults = quote! {
            #init
            #tick_method
            #render_method
            #on_enable_method
            #on_disable_method
            #on_load_method
            #on_unload_method
        };

        defaults
    };

    let settings = {
        let mut settings_list = quote! {};
        let mut mut_settings_list_with_names = quote! {};

        let settings_list_fields = opts.settings_list_fields.inner.into_iter().map(|x| x.inner).collect::<Vec<_>>();

        for setting in settings_list_fields {
            settings_list.extend(quote! {self.#setting,});
            mut_settings_list_with_names.extend(quote! {{
                let name = self.#setting.1;
                let range = self.#setting.2;
                (&mut self.#setting.0, name, range)
            },});
        }

        

        {
            quote!{
                fn get_enabled(&mut self) -> (&mut BingusSetting, &'static str, Option<[f32; 2]>) {
                    let name = self.__enabled_bool_setting.1;
                    let range = self.__enabled_bool_setting.2;
                    (&mut self.__enabled_bool_setting.0, name, range)
                }

                fn get_keybind(&mut self) -> (&mut BingusSetting, &'static str, Option<[f32; 2]>) {
                    let name = self.__keybind_setting.1;
                    let range = self.__keybind_setting.2;
                    (&mut self.__keybind_setting.0, name, range)
                }

                fn get_settings(&mut self) -> Vec<(&mut BingusSetting, &'static str, Option<[f32; 2]>)> {
                    vec![#mut_settings_list_with_names]
                }
            }
        }
    };

    let output = quote! {
        impl BingusModuleTrait for #ident {
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
                                __enabled_bool_setting: (BingusSetting, &'static str, Option<[f32; 2]>)
                            }
                        ).unwrap());
                    fields
                        .named
                        .push(syn::Field::parse_named.parse2(
                            quote! {
                                __keybind_setting: (BingusSetting, &'static str, Option<[f32; 2]>)
                            }
                        ).unwrap());
                    fields
                        .named
                        .push(syn::Field::parse_named.parse2(
                            quote! {
                                __env: Option<AtomicPtr<JNIEnv<'static>>>
                            }
                        ).unwrap());
                    fields
                        .named
                        .push(syn::Field::parse_named.parse2(
                            quote! {
                                __mappings_manager: Option<Arc<AtomicPtr<MappingsManager<'static>>>>
                            }
                        ).unwrap());
                }   
                _ => {
                    
                }
            }
            
            quote! {
                #ast
            }.into()
        }
        _ => panic!("`add_bingus_fields` has to be used with structs "),
    }
}
