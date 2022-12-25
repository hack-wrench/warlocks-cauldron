use super::dependencies::*;


#[derive(Serialize, Deserialize, Clone)]
pub struct Datetime {
    pub day: Day,
    pub formats: Formats,
    pub month: Month,
    pub periodicity: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Day {
    pub abbr: Vec<String>,
    pub name: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Formats {
    pub date: String,
    pub time: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Month {
    pub abbr: Vec<String>,
    pub name: Vec<String>,
}
