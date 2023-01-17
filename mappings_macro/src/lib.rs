#[macro_export]
macro_rules! obf_mappings {
    (field $key_name:literal, $ob_name:literal, $sig:literal, $is_static:literal) => {{
        __internal_cm.add_field(
            $key_name.to_string(),
            $ob_name.to_string(),
            $sig.to_string(),
            $is_static,
        );
    }};

    (method $key_name:literal, $ob_name:literal, $sig:literal, $is_static:literal) => {{
        __internal_cm.add_method(
            $key_name.to_string(),
            $ob_name.to_string(),
            $sig.to_string(),
            $is_static,
        );
    }};
}


#[macro_export]
macro_rules! add_mapping {
    (
        $class_name:literal,            // the easy-to-use name of the class
        $class_path:literal,            // path to the class or the obfuscated class name
        $fields_and_methods:block       // the fields and methods of the class (using the `obf_mappings!` macro)
    ) => {{
        let mut __internal_cm = ClassMapping::new_from_class(__internal_jni_env.find_class($class_path).unwrap());
        $fields_and_methods;
        __internal_mappings_manager.insert($class_name.to_string(), __internal_cm);
    }};
}

// put it all in here so the variables work
#[macro_export]
macro_rules! mappings_block {
    ($jni_env:ident, $mappings_to_add:block) => {{
        let mut __internal_mappings_manager = MappingsManager::default();
        let __internal_jni_env = $jni_env;

        $mappings_to_add;

        __internal_mappings_manager
    }};
}



// `let method_output: Result<JValue<'_>> = call_method_or_get_field!(..., &[]);`
// `let field_output: Result<JValue<'_>> = call_method_or_get_field!(...);`
#[macro_export]
macro_rules! call_method_or_get_field {
    // for fields
    ($env:expr, $cm:expr, $field_name:literal, $is_static:literal) => {{
        let field = $cm.get_field($field_name, $is_static).unwrap();

        if $is_static {
            $env.get_static_field(
                $cm.get_class(),
                field.get_name(),
                field.get_sig(),
            )
        } else {
            $env.get_field(
                $cm.get_object().unwrap(),
                field.get_name(),
                field.get_sig(),
            )
        }
    }};

    // for methods
    ($env:expr, $cm:expr, $method_name:literal, $is_static:literal, $method_args:expr) => {{
        let method = $cm.get_field($field_name, $is_static).unwrap();

        if $is_static {
            $env.call_static_method(
                $cm.get_class(),
                method.get_name(),
                method.get_sig(),
                $method_args,
            );
        } else {
            $env.call_method(
                $cm.get_object().unwrap(),
                method.get_name(),
                method.get_sig(),
                $method_args,
            );
        }
    }};
}

// put all uses of the above macro in here so the variables work
#[macro_export]
macro_rules! use_jni_env_block {
    ($jni_env:ident, $block:block) => {{
        let __internal_jni_env = $jni_env;
        $block;
    }};
}