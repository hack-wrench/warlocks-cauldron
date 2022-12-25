use super::dependecies::*;

pub struct Address(pub Locale);

impl Address {
    fn data(&self) -> &ParsedData { self.0.get_data() }

    pub fn street_number(&self) -> i32 {
        rand::thread_rng().gen_range(0..1400)
    }

    pub fn street_name(&self) -> &String {
        get_random_element(self.data().address.street.name.iter())
    }
}
