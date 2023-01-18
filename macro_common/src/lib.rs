use darling::FromMeta;
use quote::{
    __private::TokenStream as TokenStream2
};



#[derive(Default)]
pub struct StringHelper {
    pub inner: String,
}

impl FromMeta for StringHelper {
    fn from_string(value: &str) -> Result<Self, darling::Error> {
        Ok(StringHelper { inner: value.to_string() })
    }
}

#[derive(Default)]
pub struct FnHelper {
    pub inner: TokenStream2,
}

impl FromMeta for FnHelper {
    fn from_value(value: &syn::Lit) -> darling::Result<Self> {
        let value = match value {
            syn::Lit::Str(lit_str) => lit_str.value(),
            _ => panic!("expected an actual string literal"),
        };
        let value: TokenStream2 = value.parse().unwrap();
        Ok(FnHelper { inner: value })
    }
}

pub struct TypeHelper {
    pub inner: syn::Type,
}

impl FromMeta for TypeHelper {
    fn from_value(value: &syn::Lit) -> darling::Result<Self> {
        let value = match value {
            syn::Lit::Str(lit_str) => lit_str.value(),
            _ => panic!("expected an actual type"),
        };
        let value: TokenStream2 = value.parse().unwrap();
        let value: syn::Type = syn::parse2(value).unwrap();
        Ok(TypeHelper { inner: value })
    }
}



#[derive(Default)]
pub struct SettingsListHelper {
    pub inner: Vec<FnHelper>,
}

impl FromMeta for SettingsListHelper {
    fn from_value(value: &syn::Lit) -> darling::Result<Self> {
        let value_string = match value {
            syn::Lit::Str(lit_str) => lit_str.value(),
            _ => panic!("expected an actual string"),
        };

        // parse it like `[a, b, c]`
        let mut inner_vec = Vec::new();
        let mut value_string = value_string.trim();
        if value_string.starts_with('[') {
            value_string = &value_string[1..];
        } else {
            panic!("expected a list");
        }
        if value_string.ends_with(']') {
            value_string = &value_string[..value_string.len() - 1];
        } else {
            panic!("expected a closing bracket");
        }
        for value in value_string.split(',') {
            let value = value.trim();
            inner_vec.push(FnHelper { inner: value.parse().unwrap() });
        }

        Ok(SettingsListHelper { inner: inner_vec })
    }
}
