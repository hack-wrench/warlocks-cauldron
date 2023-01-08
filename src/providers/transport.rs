use super::dependencies::*;


/// Methods collection for generating data related to transports
pub struct Transport;

impl Transport {
    /// Get a random car manufacturer
    pub fn manufacturer() -> &'static str {
        get_random_element(MANUFACTURERS.iter())
    }

    /// Get a random vehicle
    pub fn car() -> &'static str {
        get_random_element(CARS.iter())
    }

    /// Generate a dummy airplane model
    pub fn airplane() -> &'static str {
        get_random_element(AIRPLANES.iter())
    }

    /// Get vehicle registration code of country
    /// 
    /// # Arguments:
    /// * `locale` - Registration code for locale (country)
    pub fn vehicle_registration_code(locale: Option<Locale>) -> &'static str {
        match locale {
            Some(locale) => &VRC_BY_LOCALES.get(locale.get_data().lang_code.as_str()).unwrap(),
            None => get_random_element(VR_CODES.iter())
        }
    }
}
