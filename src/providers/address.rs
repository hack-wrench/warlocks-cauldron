use super::dependencies::*;


pub enum DDType {
    lt, lg,
}

#[derive(Debug)]
pub enum FloatNumber {
    DMS(String),
    Raw(f32),
}

impl std::fmt::Display for FloatNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            FloatNumber::DMS(dms) => dms.to_string(),
            FloatNumber::Raw(raw) => raw.to_string(),
        })
    }
}

#[derive(Debug)]
pub enum Coordinates {
    DMS(String, String),
    Raw(f32, f32),
}

impl std::fmt::Display for Coordinates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let coordiantes = match self {
            Coordinates::DMS(lng, lat) => (lng.to_string(), lat.to_string()),
            Coordinates::Raw(lng, lat) => (lng.to_string(), lat.to_string()),
        };

        write!(f, "({}, {})", coordiantes.0, coordiantes.1) 
    }
}


/// Struct for generate fake address data.
///
/// This struct provides all the data related to geographical location.
pub struct Address<'a>(pub &'a Locale);

impl<'a> Address<'a> {
    /// Private. Return global parsed data from own locale
    fn data(&self) -> &ParsedData { self.0.get_data() }

    /// Private. Convert decimal number to DMS format
    /// 
    /// # Arguments
    /// * `num` - Decimal number
    /// * `dd_type` - Type of number
    fn dd_to_dms(num: f32, dd_type: DDType) -> String {
        let direction = match dd_type {
            DDType::lg => if num < 0.0 { "W" } else { "E" },
            DDType::lt => if num < 0.0 { "S" } else { "N" },
        };

        let num = num.abs();
        let degrees = num.floor();
        let part = num - degrees;
        let minutes = (part * 60.0).floor();
        let seconds = 3600.0 * part - 60.0 * minutes;
        format!("{degrees}º{minutes}'{seconds:.3}\"{direction}")
    }

    /// Get float number
    /// 
    /// # Arguments
    /// * `key` - DDType enum
    /// * `dms` - Use DMS format
    fn get_fs(key: DDType, dms: bool) -> FloatNumber {
        let rng = match key {
            DDType::lt => (-90.0, 90.0),
            DDType::lg => (-180.0, 180.0),
        };

        let result = uniform(rng.0, rng.1);

        match dms {
            true => FloatNumber::DMS(Self::dd_to_dms(result, key)),
            false => FloatNumber::Raw(result),
        }
    }

    /// Generate a random street number
    /// 
    /// return example: 666
    pub fn street_number(&self) -> i32 {
        rand::thread_rng().gen_range(0..1400)
    }

    /// Get a random street name
    /// 
    /// return example: Lovecraft
    pub fn street_name(&self) -> &str {
        get_random_element(self.data().address.street.name.iter())
    }

    /// Get a random street suffix
    /// 
    /// return example: Hill
    pub fn street_suffix(&self) -> &str {
        get_random_element(self.data().address.street.suffix.iter())
    }

    /// Generate a random local address | *An allias for .local_address()* 
    ///  for compatibility with mimesis using
    #[deprecated(since = "0.0.0", note = "use .local_address()")]
    pub fn address(&self) -> String {
        self.local_address()
    }

    /// Generate a random local address
    ///
    /// return example: 666 Lovecraft Avenue
    pub fn local_address(&self) -> String {
        let data = self.data();

        let format = &data.address.address_fmt;

        let st_num = &self.street_number().to_string();
        let st_name = self.street_name();
        
        if SHORTENED_ADDRESS_FMT.contains(&data.lang_code) {
            return format
                .replace("{st_num}", st_num)
                .replace("{st_name}", st_name);
        }

        if data.lang_code.eq("ja") {
            return format
                .replacen("{}", self.city(), 1)
                .replacen("{}", &randint(0, 100).to_string(), 1)
                .replacen("{}", &randint(0, 100).to_string(), 1)
                .replacen("{}", &randint(0, 100).to_string(), 1);
        }

        format
            .replace("{st_num}", st_num)
            .replace("{st_name}", st_name)
            .replace("{st_sfx}", &self.street_suffix())
    }

    /// Generate a random address including country name and state
    /// 
    /// return example: United States, Massachusetts, Innsmouth, 666 Lovecraft Avenue
    pub fn full_address(&self) -> String {
        format!("{}, {}, {}, {}", self.country(true), self.state(false), self.city(), self.local_address())
    }

    /// Get a random continent name or continent code
    /// 
    /// 
    /// return example: NA
    ///
    /// # Arguments
    /// * `code` - Return code of continent
    pub fn continent(&self, code: bool) -> &str {
        get_random_element(match code {
            true => self.data().address.continent.iter(),
            false => CONTINENT_CODES.iter(),
        })
    }

    /// Get a random calling code of random country
    ///  
    /// return example: US
    ///
    /// # Arguments
    /// * `code` - CountryCode enum
    pub fn country_code(&self, code: Option<CountryCode>) -> Option<&str> {
        match COUNTRY_CODES.get(validate_enum(code, None)) {
            Some(cc) => Some(get_random_element(cc.iter())),
            None => None,
        }
    }

    /// Get the country of the current locale
    ///  
    /// return example: United States
    ///
    /// # Arguments
    /// * `current_locale` - Get country name by current locale
    pub fn country(&self, current_locale: bool) -> &str {
        match current_locale {
            false => get_random_element(self.data().address.country.name.iter()),
            true => &self.data().address.country.current_locale,
        }
    }

    /// Get a random administrative district of country
    /// 
    /// return example: Massachusetts
    ///
    /// # Arguments
    /// * `abbr` - Return ISO 3166-2 code
    pub fn state(&self, abbr: bool) -> &str {
        get_random_element(
            match abbr {
                true => self.data().address.state.abbr.iter(),
                false => self.data().address.state.name.iter(),
            }
        )
    }

    /// Get a random region | *An allias for .state()*
    /// 
    /// return example: Massachusetts
    ///
    /// # Arguments
    /// * `abbr` - Return ISO 3166-2 code
    pub fn region(&self, abbr: bool) -> &str {
        self.state(abbr)
    }

    /// Get a random province | *An allias for .state()*
    /// 
    /// return example: Massachusetts
    ///
    /// # Arguments
    /// * `abbr` - Return ISO 3166-2 code
    pub fn province(&self, abbr: bool) -> &str {
        self.state(abbr)
    }

    /// Get a random region | *An allias for .state()*
    /// 
    /// return example: Massachusetts
    ///
    /// # Arguments
    /// * `abbr` - Return ISO 3166-2 code
    pub fn federal_subject(&self, abbr: bool) -> &str {
        self.state(abbr)
    }

    /// Get a random prefecture | *An allias for .state()*
    /// 
    /// return example: Massachusetts
    ///
    /// # Arguments
    /// * `abbr` - Return ISO 3166-2 code
    pub fn prefecture(&self, abbr: bool) -> &str {
        self.state(abbr)
    }

    /// Get a random city
    /// 
    /// return example: Innsmouth
    pub fn city(&self) -> &str {
        get_random_element(self.data().address.city.iter())
    }

    /// Generate a postal code for current locale
    ///
    /// return example: 66613
    pub fn postal_code(&self) -> String {
        custom_code(&self.data().address.postal_code_fmt, '@', '#')
    }

    /// Generate a zip code | *An allias for .postal_code()*
    ///
    /// return example: 66613
    pub fn zip_code(&self) -> String {
        self.postal_code()
    }

    /// Get a random calling code of random country
    ///
    /// return example: +666
    pub fn calling_code(&self) -> &str {
        get_random_element(CALLING_CODES.iter())
    }

    /// Generate a random value of latitude
    /// 
    /// return example: 41º14'0.000"N
    ///
    /// # Arguments
    /// * `abbr` - Use DMS format
    pub fn latitude(dms: bool) -> FloatNumber {
        Self::get_fs(DDType::lt, dms)
    }

    /// Generate a random value of longitude
    /// 
    /// return example: 69º56'0.000"W
    /// 
    /// # Arguments
    /// * `abbr` - Use DMS format
    pub fn longitude(dms: bool) -> FloatNumber {
        Self::get_fs(DDType::lg, dms)
    }

    /// Generate random geo coordinates
    /// 
    /// return example: Coordinates::DMS("41º14'0.000"N", "69º56'0.000"W")
    ///
    /// # Arguments
    /// * `abbr` - Use DMS format
    pub fn coordinates(dms: bool) -> Coordinates {
        match (dms, Self::latitude(dms), Self::longitude(dms)) {
            (true, FloatNumber::DMS(lat), FloatNumber::DMS(lng)) => Coordinates::DMS(lat, lng),
            (false, FloatNumber::Raw(lat), FloatNumber::Raw(lng)) => Coordinates::Raw(lat, lng),
            _ => panic!("In theory, it shouldn't break :D"),
        }
    }
}
