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
        let method = $cm.get_method($method_name, $is_static).unwrap();

        if $is_static {
            $env.call_static_method(
                $cm.get_class(),
                method.get_name(),
                method.get_sig(),
                $method_args,
            )
        } else {
            $env.call_method(
                $cm.get_object().unwrap(),
                method.get_name(),
                method.get_sig(),
                $method_args,
            )
        }
    }};
}

// puts a jni JObject into a ClassMapping
// `apply_object!(ClassMapping, JObject);`
#[macro_export]
macro_rules! apply_object {
    ($to_apply_cm:ident, $object_to_apply:expr) => {
        $to_apply_cm.apply_object($object_to_apply)
    };
}
