use super::dependencies::*;


#[derive(Serialize, Deserialize)]
pub struct Person {
    pub academic_degree: Vec<String>,
    pub gender: Vec<String>,
    pub language: Vec<String>,
    pub names: Names,

    pub nationality: Value,

    pub occupation: Vec<String>,
    pub political_views: Vec<String>,

    pub surnames: Value,

    pub title: Title,
    pub university: Vec<String>,
    pub views_on: Vec<String>,
    pub worldview: Vec<String>,
    pub telephone_fmt: Vec<String>,
}


#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct NationalityMap {
    pub female: Vec<String>,
    pub male: Vec<String>,
}

#[derive(PartialEq, Debug)]
pub enum NationalityOption {
    Map(NationalityMap),
    Sequence(Vec<String>),
    None,
}


#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct SurnamesMap {
    pub female: Vec<String>,
    pub male: Vec<String>,
}

#[derive(PartialEq, Debug)]
pub enum SurnamesOption {
    Map(SurnamesMap),
    Sequence(Vec<String>),
    None,
}



impl Person {
    pub fn get_nationality(&self) -> NationalityOption {
        if let Value::Array(_) = &self.nationality {
            return NationalityOption::Sequence(serde_json::from_value(self.nationality.clone()).unwrap());
        } else if let Value::Object(_) = &self.nationality {
            return NationalityOption::Map(serde_json::from_value(self.nationality.clone()).unwrap());
        }

        NationalityOption::None
    }

    pub fn get_surnames(&self) -> SurnamesOption {
        if let Value::Array(_) = &self.surnames {
            return SurnamesOption::Sequence(serde_json::from_value(self.surnames.clone()).unwrap());
        } else if let Value::Object(_) = &self.surnames {
            return SurnamesOption::Map(serde_json::from_value(self.surnames.clone()).unwrap());
        }

        SurnamesOption::None
    }
}

#[derive(Serialize, Deserialize)]
pub struct Names {
    pub female: Vec<String>,
    pub male: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Title {
    pub female: Female,
    pub male: Male,
}

#[derive(Serialize, Deserialize)]
pub struct Female {
    pub typical: Vec<String>,
    pub academic: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Male {
    pub typical: Vec<String>,
    pub academic: Vec<String>,
}
