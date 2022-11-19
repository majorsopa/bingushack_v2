use std::collections::HashMap;

use super::class_mapping::ClassMapping;

#[derive(Debug, Default)]
pub struct MappingsManager<'a> {
    mappings: HashMap<&'static str, ClassMapping<'a>>
}

impl MappingsManager<'_> {
    pub fn get_class(&self, name: &str) -> Option<&ClassMapping> {
        self.mappings.get(name)
    }
}
