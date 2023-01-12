use super::super::{Local, Datelike, dependencies::*};


/// Methods collection provides special data for Ukraine (uk)
pub struct UkraineSpecProvider;

impl UkraineSpecProvider {
    /// Generate random patronymic name
    pub fn patronymic(gender: Option<Gender>) -> String {
        let patronymics = crate::data::parsed::UK.builtin.get("patronymic").unwrap()
            .as_object().unwrap();

        let gender = validate_enum(gender, None);

        get_random_element(patronymics.get(gender).expect("Cannot find a patronymic with the given gender!")
            .as_array().unwrap().into_iter()
                .map(|i| i.as_str())).unwrap()
                    .to_string()
    }
}
