use std::collections::HashMap;

use jni::JNIEnv;

use super::class_mapping::ClassMapping;
use super::SigHolder;

#[derive(Debug, Default)]
pub struct MappingsManager<'a> {
    mappings: HashMap<&'static str, ClassMapping<'a>>
}

impl MappingsManager<'_> {
    pub fn new(env: JNIEnv) -> Self {
        macro_rules! adds {
            ($cm:ident) => {
                #[allow(unused_macros)]
                macro_rules! add_field {
                    ($key_name:literal, $ob_name:literal, $sig:literal, $is_static:literal) => {
                        let sig = SigHolder::new($ob_name, $sig);
                        $cm.add_field(
                            $key_name,
                            sig,
                            $is_static,
                        )
                    };
                }

                #[allow(unused_macros)]
                macro_rules! add_method {
                    ($key_name:literal, $ob_name:literal, $sig:literal, $is_static:literal) => {
                        let sig = SigHolder::new($ob_name, $sig);
                        $cm.add_method(
                            $key_name,
                            sig,
                            $is_static,
                        )
                    };
                }
            };
        }
        // macro for making a class mapping
        macro_rules! add_mapping {
            (
                $new_self:ident,
                $class_name:literal,            // the easy-to-use name of the class
                $class_path:literal,            // path to the class or the obfuscated class name
                $fields_and_methods:block       // the fields and methods of the class (using the `add_field_or_method!` macro)
            ) => {{
                let mut cm = ClassMapping::new_from_class(env.find_class($class_path).unwrap());

                adds!(cm);
                $fields_and_methods

                $new_self.mappings.insert($class_name, cm);
            }}
        }

        let mut new_self = MappingsManager::default();



        let mut mappings_manager = MappingsManager::default();

        add_mapping!(new_self, "MinecraftClient", "ejf", {
            add_field!("player", "t", "Lfcz;", false);
        });

        mappings_manager
    }

    pub fn get_class(&self, name: &str) -> Option<&ClassMapping> {
        self.mappings.get(name)
    }
}
