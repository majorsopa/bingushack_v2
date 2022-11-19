#[derive(Debug, Clone, Copy)]
pub struct SigHolder {
    name: &'static str,
    sig: &'static str,
}

impl SigHolder {
    pub fn new(name: &'static str, sig: &'static str) -> Self {
        Self { name, sig }
    }

    pub fn get_name(&self) -> &'static str {
        self.name
    }

    pub fn get_sig(&self) -> &'static str {
        self.sig
    }
}
