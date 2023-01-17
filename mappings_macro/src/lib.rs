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