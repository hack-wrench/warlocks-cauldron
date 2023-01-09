use super::{
    dependencies::*,
    *,
};

/// Collection of every provider, which uses locales
pub struct ComplexProvider {
    pub address: Address,
    pub date: Datetime,
    pub finance: Finance,
    pub food: Food,
    pub person: Person,
    pub text: Text,
}

impl ComplexProvider {
    pub fn new(locale: Locale) -> Self {
        Self {
            address: Address(locale.clone()),
            date: Datetime(locale.clone()),
            finance: Finance(locale.clone()),
            food: Food(locale.clone()),
            person: Person(locale.clone()),
            text: Text(locale.clone()),
        }
    }
}
