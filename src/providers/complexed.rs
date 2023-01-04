use super::{
    dependencies::*,
    *,
};


pub struct ComplexProvider {
    pub address: Address,
    pub date: Datetime,
    pub finance: Finance,
    pub food: Food,
    pub text: Text,
}

impl ComplexProvider {
    pub fn new(locale: Locale) -> Self {
        Self {
            address: Address(locale.clone()),
            date: Datetime(locale.clone()),
            finance: Finance(locale.clone()),
            food: Food(locale.clone()),
            text: Text(locale.clone()),
        }
    }
}
