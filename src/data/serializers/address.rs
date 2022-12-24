use super::dependencies::*;


#[derive(Serialize, Deserialize, Clone)]
pub struct Address {
    pub address_fmt: String,
    pub city: Vec<String>,

    #[serde(default)]
    pub continent: Vec<String>,

    pub country: Country,
    pub postal_code_fmt: String,
    pub state: State,
    pub street: Street,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Country {
    pub current_locale: String,

    #[serde(default)]
    pub name: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct State {
    pub abbr: Vec<String>,
    pub name: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Street {
    pub name: Vec<String>,

    #[serde(default)]
    pub suffix: Vec<String>,
}
