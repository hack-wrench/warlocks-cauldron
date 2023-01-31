use super::{
    dependencies::*,
    *,
};

/// Collection of every provider, which uses locales
pub struct ComplexProvider<'a> {
    pub address: Address<'a>,
    pub date: Datetime<'a>,
    pub finance: Finance<'a>,
    pub food: Food<'a>,
    pub person: Person<'a>,
    pub text: Text<'a>,
}

impl<'a> ComplexProvider<'a> {
    pub fn new(locale: &'a Locale) -> Self {
        Self {
            address: Address(locale),
            date: Datetime(locale),
            finance: Finance(locale),
            food: Food(locale),
            person: Person(locale),
            text: Text(locale),
        }
    }
}
