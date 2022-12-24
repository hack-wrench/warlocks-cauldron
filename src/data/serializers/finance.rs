use super::dependencies::*;

#[derive(Serialize, Deserialize, Clone)]
pub struct Finance {
    pub company: Company,

    #[serde(rename = "currency-code")]
    pub currency_code: String,

    #[serde(rename = "price-format", default)]
    pub price_format: String,

    #[serde(rename = "numeric-decimal", default)]
    pub numeric_decimal: String,

    #[serde(rename = "numeric-thousands", default)]
    pub numeric_thousands: String,

    #[serde(rename = "numeric-frac-digits", default)]
    pub numeric_frac_digits: u8,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Company {
    pub name: Vec<String>,

    #[serde(rename = "type")]
    pub _type: Type,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Type {
    pub abbr: Vec<String>,
    pub title: Vec<String>,
}
