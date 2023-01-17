use std::{collections::HashMap, cell::RefCell};

use jni::objects::{JClass, JObject};

use super::sig_holder::SigHolder;

#[derive(Debug)]
pub struct ClassMapping<'a> {
    class: RefCell<JClass<'a>>,
    object: RefCell<Option<JObject<'a>>>,

    fields: HashMap<&'static str, SigHolder>,
    static_fields: HashMap<&'static str, SigHolder>,

    methods: HashMap<&'static str, SigHolder>,
    static_methods: HashMap<&'static str, SigHolder>,
}

impl<'a> ClassMapping<'a> {
    pub fn new_from_class(class: JClass<'a>) -> Self {
        Self {
            class: RefCell::new(class),
            object: RefCell::new(None),

            fields: HashMap::new(),
            static_fields: HashMap::new(),

            methods: HashMap::new(),
            static_methods: HashMap::new(),
        }
    }

    pub fn apply_object(&self, object: JObject<'a>) {
        *self.object.borrow_mut() = Some(object);
    }

    pub fn get_field(&self, name: &str, is_static: bool) -> Option<&SigHolder> {
        if is_static {
            self.static_fields.get(name)
        } else {
            self.fields.get(name)
        }
    }

    pub fn add_field(&mut self, name: &'static str, sig: SigHolder, is_static: bool) {
        if is_static {
            self.static_fields.insert(name, sig);
        } else {
            self.fields.insert(name, sig);
        }
    }

    pub fn get_method(&self, name: &str, is_static: bool) -> Option<&SigHolder> {
        if is_static {
            self.static_methods.get(name)
        } else {
            self.methods.get(name)
        }
    }

    pub fn add_method(&mut self, name: &'static str, sig: SigHolder, is_static: bool) {
        if is_static {
            self.static_methods.insert(name, sig);
        } else {
            self.methods.insert(name, sig);
        }
    }

    pub fn get_class(&self) -> JClass<'a> {
        *self.class.borrow()
    }

    pub fn get_object(&self) -> Option<JObject<'a>> {
        *self.object.borrow()
    }
}
