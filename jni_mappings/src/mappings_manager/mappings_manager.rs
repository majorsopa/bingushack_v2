use std::collections::HashMap;

use jni::JNIEnv;
use mappings_macro::mappings_block;

use super::class_mapping::ClassMapping;

#[derive(Debug, Default)]
pub struct MappingsManager<'a> {
    mappings: HashMap<&'static str, ClassMapping<'a>>
}

impl MappingsManager<'_> {
    pub fn new(env: JNIEnv) -> Self {
        mappings_block! {
            env, 
            {}
        }
    }

    pub fn get_class(&self, name: &str) -> Option<&ClassMapping> {
        self.mappings.get(name)
    }
}
