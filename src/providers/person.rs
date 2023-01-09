use std::str::FromStr;

use sha2::{Sha256, Digest};
use regex::Regex;

use crate::data::serializers::{SurnamesOption, NationalityOption};
use super::dependencies::*;


pub enum SexType<'a> { ISO5218(u8), String(&'a str) }

/// Struct for generating personal data
pub struct Person(pub Locale);

impl Person {
    /// Private. Return global parsed data from own locale
    fn data(&self) -> &ParsedData { self.0.get_data() }

    /// Get a random integer value
    pub fn age(minimum: i32, maximum: i32) -> i32 {
        randint(minimum, maximum)
    }

    /// Get a random integer value
    pub fn work_experience(minimum: i32, maximum: i32) -> i32 {
        randint(minimum, maximum)
    }

    /// Generate a random name
    pub fn name(&self, gender: Option<Gender>) -> &str {
        match validate_variant(gender, None) {
            Gender::MALE => get_random_element(self.data().person.names.male.iter()),
            Gender::FEMALE => get_random_element(self.data().person.names.female.iter()),
            _ => panic!("Validation error!"),
        }
    }

    /// Generate a random first name | *An allias for .name()*
    pub fn first_name(&self, gender: Option<Gender>) -> &str {
        self.name(gender)
    }
    
    /// Generate a random surname | *An allias for .name()*
    pub fn surname(&self, gender: Option<Gender>) -> String {
        match self.data().person.get_surnames() {
            SurnamesOption::Map(m) => match validate_variant(gender, None) {
                Gender::MALE => get_random_element(m.male.into_iter()),
                Gender::FEMALE => get_random_element(m.female.into_iter()),
                _ => panic!("Validation surnames error!"),
            },
            SurnamesOption::Sequence(seq) => get_random_element(seq.into_iter()),
            SurnamesOption::None => panic!("There is no surnames exists!"),
        }
    }

    /// Generate a random last name | *An allias for .surname()*
    pub fn last_name(&self, gender: Option<Gender>) -> String {
        self.surname(gender)
    }

    /// Generate a random title for name
    pub fn title(&self, gender: Option<Gender>, title_type: Option<TitleType>) -> &str {
        get_random_element(match (validate_variant(gender, None), validate_variant(title_type, None)) {
            (Gender::MALE, TitleType::TYPICAL) => self.data().person.title.male.typical.iter(),
            (Gender::MALE, TitleType::ACADEMIC) => self.data().person.title.male.academic.iter(),
            (Gender::FEMALE, TitleType::TYPICAL) => self.data().person.title.female.typical.iter(),
            (Gender::FEMALE, TitleType::ACADEMIC) => self.data().person.title.female.academic.iter(),
            _ => panic!("Validation error!"),
        })
    }

    /// Generate a random full name
    ///
    /// ex: name surname
    pub fn full_name(&self, gender: Option<Gender>, reverse: bool) -> String {
        match reverse {
            true => format!("{} {}", self.name(gender.clone()), self.surname(gender.clone())),
            false => format!("{} {}",self.surname(gender.clone()), self.name(gender.clone())),
        }
    }

    /// You can create many different usernames using masks:
    ///
    /// - **C** stands for capitalized username.
    /// - **U** stands for uppercase username.
    /// - **l** stands for lowercase username.
    /// - **d** stands for digits in username.
    ///
    /// You can also use symbols to separate the different part of the username: **.** **_** **-**
    pub fn username(mask: Option<&str>, drange: Option<(u32, u32)>) -> String {
        let mask = match mask {
            Some(m) => m,
            None => "l_d",
        };

        let drange = match drange {
            Some(dr) => dr,
            None => (1800, 2100),
        };

        let mut output = String::new();
        for tag in Regex::new(r"[CUld.\-_]").unwrap().find_iter(mask) {
            let username = get_random_element(USERNAMES.iter());
            output.push_str(&match tag.as_str() {
                "C" => username[..1].to_uppercase() + &username[1..],
                "U" => username.to_uppercase(),
                "l" => username.to_lowercase(),
                "d" => randint(drange.0, drange.1).to_string(),
                other => String::from_str(other).unwrap(),
            });
        }

        output
    }

    /// Generate a password or hash of password
    pub fn password(length: usize, hashed: bool) -> String {
        let characters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!\"#$%&\'()*+,-./:;<=>?@[\\]^_`{|}~";
        let password = get_random_elements(characters.chars(), length).iter().join("");

        match hashed {
            true => format!("{:x}", Sha256::new_with_prefix(password.as_bytes()).finalize()),
            false => password,
        }
    }

    /// Generate a random email
    ///
    /// Arguments:
    /// * `domains` - List of custom domains for emails
    /// * `unique` - Makes email addresses unique
    pub fn email(domains: Option<Vec<&'static str>>, unique: bool) -> String {
        let domain = match domains {
            Some(d) => {
                let domain = get_random_element(d.into_iter());
                match domain.starts_with('@') {
                    true => domain.to_string(),
                    false => format!("@{domain}"),
                }
            },
            None => get_random_element(EMAIL_DOMAINS.iter()).to_string(),
        };

        let name = match unique {
            true => randstr(unique, 13),
            false => Self::username(Some("ld"), Some((1800, 2100)))
        };

        format!("{name}{domain}")
    }

    /// Get a random gender
    /// 
    /// Arguments:
    /// * `iso5218` - Codes for the representation of human sexes is an international standard (0 - not known, 1 - male, 2 - female, 9 - not applicable)
    /// * `symbol` - Symbol of gender
    pub fn gender(&self, iso_5218: bool, symbol: bool) -> SexType {
        if iso_5218 {
            return SexType::ISO5218(get_random_element(vec![1, 2, 3, 9].into_iter()));
        }

        if symbol {
            return SexType::String(get_random_element(GENDER_SYMBOLS.iter()));
        }

        SexType::String(get_random_element(self.data().person.gender.iter()))
    }


    /// Get a random sex | *An allias for .gender()*
    pub fn sex(&self, iso_5218: bool, symbol: bool) -> SexType {
        self.gender(iso_5218, symbol)
    }

    /// Generate a random height
    pub fn height(minimum: f32, maximum: f32) -> String {
        format!("{:.2}", uniform(minimum, maximum))
    }

    /// Generate a random weight
    pub fn weight(minimum: i32, maximum: i32) -> i32 {
        randint(minimum, maximum)
    }

    /// Get a random blood type
    pub fn blood_type() -> &'static str {
        get_random_element(BLOOD_GROUPS.iter())
    }

    /// Get a random job
    pub fn occupation(&self) -> &str {
        get_random_element(self.data().person.occupation.iter())
    }

    /// Get a random political views
    pub fn political_views(&self) -> &str {
        get_random_element(self.data().person.political_views.iter())
    }

    /// Get a random political views
    pub fn worldview(&self) -> &str {
        get_random_element(self.data().person.worldview.iter())
    }

    /// Get a random political views
    pub fn views_on(&self) -> &str {
        get_random_element(self.data().person.views_on.iter())
    }

    /// Generate a random nationality | *An allias for .name()*
    pub fn nationality(&self, gender: Option<Gender>) -> String {
        match self.data().person.get_nationality() {
            NationalityOption::Map(m) => match validate_variant(gender, None) {
                Gender::MALE => get_random_element(m.male.into_iter()),
                Gender::FEMALE => get_random_element(m.female.into_iter()),
                _ => panic!("Validation nationality error!"),
            },
            NationalityOption::Sequence(seq) => get_random_element(seq.into_iter()),
            NationalityOption::None => panic!("There is no nationality exists!"),
        }
    }

    /// Get a random university
    pub fn university(&self) -> &str {
        get_random_element(self.data().person.university.iter())
    }

    /// Get a random academic degree
    pub fn academic_degree(&self) -> &str {
        get_random_element(self.data().person.academic_degree.iter())
    }

    /// Get a random language
    pub fn language(&self) -> &str {
        get_random_element(self.data().person.language.iter())
    }

    /// Generate a random phone number
    /// 
    /// Arguments:
    /// * `mask` - The mask. Here ``@`` is a placeholder for characters and ``#`` is placeholder for digits
    pub fn telephone(&self, mask: Option<&str>) -> String {
        let mask = match mask {
            None => get_random_element(self.data().person.telephone_fmt.iter()),
            Some(m) => m,
        };

        custom_code(mask, '@', '#')
    }

    /// Generate a random identifier by mask
    /// 
    /// Arguments:
    /// * `mask` - The mask. Here ``@`` is a placeholder for characters and ``#`` is placeholder for digits
    pub fn identifier(&self, mask: Option<&str>) -> String {
        let mask = match mask {
            None => "##-##/##",
            Some(m) => m,
        };

        custom_code(mask, '@', '#')
    }
}
