use super::dependencies::*;


#[derive(Serialize, Deserialize, Clone)]
pub struct Person {
    pub academic_degree: Vec<String>,
    pub gender: Vec<String>,
    pub language: Vec<String>,
    pub names: Names,

    #[serde(rename = "nationality")]
    pub seq_nationality: Option<Vec<String>>,

    #[serde(rename = "nationality")]
    pub map_nationality: Option<Vec<String>>,

    pub occupation: Vec<String>,
    pub political_views: Vec<String>,
    pub surnames: Vec<String>,
    pub title: Title,
    pub university: Vec<String>,
    pub views_on: Vec<String>,
    pub worldview: Vec<String>,
    pub telephone_fmt: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Names {
    pub female: Vec<String>,
    pub male: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Title {
    pub female: Female,
    pub male: Male,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Female {
    pub typical: Vec<String>,
    pub academic: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Male {
    pub typical: Vec<String>,
    pub academic: Vec<String>,
}
