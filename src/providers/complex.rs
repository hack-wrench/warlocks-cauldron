use super::{
    dependencies::*,
    *,
};


pub struct ComplexProvider {
    pub address: Address,
    pub code: Code,
    pub cryptographic: Cryptographic,
    pub date: Datetime,
    pub text: Text,
}

impl ComplexProvider {
    pub fn new(locale: Locale) -> Self {
        Self {
            address: Address(locale.clone()),
            code: Code,
            cryptographic: Cryptographic,
            date: Datetime(locale.clone()),
            text: Text(locale.clone()),
        }
    }
}
