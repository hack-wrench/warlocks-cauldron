use super::dependencies::*;


/// A struct for generating finance data
pub struct Finance<'a>(pub &'a Locale);

impl<'a> Finance<'a> {
    /// Private. Return global parsed data from own locale
    fn data(&self) -> &ParsedData { self.0.get_data() }

    /// Get a random company name
    ///
    /// return example: Some Company He.Re.
    pub fn company(&self) -> &str {
        get_random_element(self.data().finance.company.name.iter())
    }

    /// Get a random type of business entity
    ///
    /// return example: Corp. OR Coproration
    pub fn company_type(&self, abbr: bool) -> &str {
        get_random_element(match abbr {
            true => self.data().finance.company._type.abbr.iter(),
            false => self.data().finance.company._type.title.iter(),
        })
    }

    /// Get code of the currency for current locale
    ///
    /// return example: USD
    /// 
    /// # Arguments
    /// * `allow_random` - Get a random ISO code
    pub fn currency_iso_code(&self, allow_random: bool) -> &str {
        match allow_random {
            true => get_random_element(CURRENCY_ISO_CODES.iter()),
            false => &self.data().finance.currency_code,
        }
    }

    /// Get a cryptocurrency symbol
    /// 
    /// return example: BTC
    pub fn cryptocurrency_iso_code() -> &'static str {
        get_random_element(CRYPTOCURRENCY_ISO_CODES.iter())
    }

    /// Get a currency symbol for current locale
    /// 
    /// return example: $
    pub fn currency_symbol(&self) -> &str {
        CURRENCY_SYMBOLS.get(self.data().lang_code.as_str()).unwrap_or_else(|| CURRENCY_SYMBOLS.get("default").unwrap())
    }

    /// Get a cryptocurrency symbol
    /// 
    /// return example: ₿
    pub fn cryptocurrency_symbol() -> &'static str {
        get_random_element(CRYPTOCURRENCY_SYMBOLS.iter())
    }


    /// Generate random price
    /// 
    /// return example: 6.66
    /// 
    /// # Arguments
    /// * `minimum` - minimum value of price
    /// * `maximum` - maximum value of price
    pub fn price(minimum: f32, maximum: f32) -> f32 {
        uniform(minimum, maximum)
    }

    /// Return random stock ticker
    /// 
    /// return example: ABC
    pub fn stock_ticker() -> &'static str {
        get_random_element(STOCK_TICKERS.iter())
    }

    /// Return stock name
    /// 
    /// return example: 1-800 FLOWERS.COM
    pub fn stock_name() -> &'static str {
        get_random_element(STOCK_NAMES.iter())
    }

    /// Returns stock exchange name
    /// 
    /// return example: NASDAQ
    pub fn stock_exchange() -> &'static str {
        get_random_element(STOCK_EXCHANGES.iter())
    }
}
