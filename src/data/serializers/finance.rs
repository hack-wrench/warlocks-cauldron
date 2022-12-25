use super::dependencies::*;

#[derive(Serialize, Deserialize)]
pub struct Finance {
    pub company: Company,

    #[serde(rename = "currency-code")]
    pub currency_code: String,

    #[serde(rename = "price-format")]
    pub price_format: Option<String>,

    #[serde(rename = "numeric-decimal")]
    pub numeric_decimal: Option<String>,

    #[serde(rename = "numeric-thousands")]
    pub numeric_thousands: Option<String>,

    #[serde(rename = "numeric-frac-digits")]
    pub numeric_frac_digits: Option<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct Company {
    pub name: Vec<String>,

    #[serde(rename = "type")]
    pub _type: Type,
}

#[derive(Serialize, Deserialize)]
pub struct Type {
    pub abbr: Vec<String>,
    pub title: Vec<String>,
}
