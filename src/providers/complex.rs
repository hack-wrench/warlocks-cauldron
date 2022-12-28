use super::{
    dependencies::*,
    *,
};


pub struct ComplexProvider {
    #[cfg(feature = "address")]
    pub address: Address,

    #[cfg(feature = "code")]
    pub code: Code,

    #[cfg(feature = "cryptographic")]
    pub cryptographic: Cryptographic,

    #[cfg(feature = "date")]
    pub date: Datetime,


    #[cfg(feature = "text")]
    pub text: Text,
}

impl ComplexProvider {
    pub fn new(locale: Locale) -> Self {
        Self {
            #[cfg(feature = "address")]
            address: Address(locale.clone()),

            #[cfg(feature = "code")]
            code: Code,

            #[cfg(feature = "cryptographic")]
            cryptographic: Cryptographic,

            #[cfg(feature = "date")]
            date: Datetime(locale.clone()),


            #[cfg(feature = "text")]
            text: Text(locale.clone()),
        }
    }
}
