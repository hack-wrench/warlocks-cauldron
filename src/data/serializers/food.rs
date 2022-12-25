use super::dependencies::*;


#[derive(Serialize, Deserialize, Clone)]
pub struct Food {
    pub dishes: Vec<String>,
    pub drinks: Vec<String>,
    pub fruits: Vec<String>,
    pub spices: Vec<String>,
    pub vegetables: Vec<String>,
}
