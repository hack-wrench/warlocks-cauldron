use super::super::dependencies::*;


rust_enum! {
    pub enum TrackingService(&'static [&'static str]) {
        USPS = &[
            "#### #### #### #### ####",
            "@@ ### ### ### US",
        ],

        FEDEX = &[
            "#### #### ####",
            "#### #### #### ###",
        ],

        UPS = &[
            "1Z@####@##########",
        ],
    }
}


/// Methods collection provides special data for USA (en)
pub struct USASpecProvider;

impl USASpecProvider {
    /// Generate random tracking number
    pub fn tracking_number(service: TrackingService) -> String {
        let mask = get_random_element(service.value().iter());
        custom_code(mask, '@', '#')
    }

    /// Generate a random, but valid SSN
    pub fn ssn() -> String {
        let area = match randint(1, 899) {
            666 => 665,
            other => other,
        };

        format!("{area:03}-{:02}-{:04}", randint(1, 99), randint(1, 9999))
    }
}
