use darling::FromMeta;
use quote::__private::TokenStream as TokenStream2;


#[derive(Default)]
pub struct StringHelper {
    pub inner: &'static str,
}

impl FromMeta for StringHelper {
    fn from_string(value: &str) -> Result<Self, darling::Error> {
        let value = unsafe { std::mem::transmute::<&str, &'static str>(value) };
        Ok(StringHelper { inner: value })
    }
}

#[derive(Default)]
pub struct FnHelper {
    pub inner: TokenStream2,
}

impl FromMeta for FnHelper {
    /*fn from_string(value: &str) -> Result<Self, darling::Error> {
        let value: TokenStream = value.parse().unwrap();
        Ok(FnPointerHelper { inner: value.into() })
    }*/
    fn from_value(value: &syn::Lit) -> darling::Result<Self> {
        let value = match value {
            syn::Lit::Str(lit_str) => lit_str.value(),
            _ => panic!("expected an actual string literal"),
        };
        let value: TokenStream2 = value.parse().unwrap();
        Ok(FnHelper { inner: value.into() })
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
