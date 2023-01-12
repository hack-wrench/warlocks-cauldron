use super::super::dependencies::*;


/// Methods collection provides special data for Italy (it)
pub struct ItalySpecProvider;

impl ItalySpecProvider {
    /// Return a random fiscal code
    pub fn fiscal_code(gender: Option<Gender>) -> String {
        let mut code = generate_string("ABCDEFGHIJKLMNOPQRSTUVWXYZ", 6);
        code.push_str(&custom_code("##", '@', '#'));

        let fiscal_code = crate::data::parsed::IT.builtin.get("fiscal_code").unwrap()
            .as_object().unwrap();
        
        code.push_str(
            get_random_element(fiscal_code
                .get("month_codes").unwrap()
                .as_array().unwrap()
                .into_iter().map(|c| c.as_str().unwrap())
            )
        );

        let mut birth_day = randint(101, 131);
        if validate_variant(gender, None) == Gender::FEMALE {
            birth_day += 40;
        }

        code.push_str(&birth_day.to_string()[1..]);

        code.push_str(
            get_random_element(fiscal_code
                .get("city_letters").unwrap()
                .as_array().unwrap()
                .into_iter().map(|c| c.as_str().unwrap())
            )
        );

        code.push_str(&custom_code("###@", '#', '@'));

        code
    }
}
