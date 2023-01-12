use std::iter::zip;

use crate::data::parsed::RU;

use super::super::{Local, Datelike, dependencies::*};


lazy_static! {
    static ref TAX_CODES: Vec<&'static str> = vec![
        "7700", "7800", "5000", "0100",
        "0200", "0300", "0500", "0600",
        "0700", "0800", "0900", "1000",
        "1100", "1200", "1300", "1400",
        "1500", "1600", "1700", "1800",
        "1900", "2000", "2100", "2200",
        "2300", "2400", "2500", "2600",
        "2700", "2800", "2900", "3000",
        "3100", "3200", "3300", "3400",
        "3500", "3600", "3700", "3800",
        "3900", "4000", "4100", "4900",
        "5100", "5200", "5300", "5400",
        "5500", "5600", "5700", "5800",
        "5900", "6000", "6100", "6200",
        "6300", "6400", "6500", "6600",
        "6700", "6800", "6900", "7000",
        "7100", "7200", "7300", "7400",
        "7500", "7600", "7900", "8600",
        "8700", "8900", "9100", "9200",
        "9800", "9900", "9901", "9951",
        "9952", "9953", "9954", "9955",
        "9956", "9957", "9958", "9959",
        "9961", "9962", "9965", "9966",
        "9971", "9972", "9973", "9974",
        "9975", "9976", "9977", "9979",
        "9998",
    ];
}


/// Methods collection provides special data for Russia (ru)
pub struct RussiaSpecProvider;

impl RussiaSpecProvider {
    /// Generate sentence from the parts.
    pub fn generate_sentence() -> String {
        let sentence = RU.builtin.get("sentence").unwrap()
            .as_object().unwrap();

        vec!["head", "p1", "p2", "tail"].into_iter()
            .map(|k| get_random_element(
                sentence.get(k).unwrap().as_array().unwrap().into_iter()
                    .map(|i| i.as_str().unwrap())))
                        .join(" ")
    }

    /// Generate random patronymic name
    pub fn patronymic(gender: Option<Gender>) -> String {
        let patronymics = RU.builtin.get("patronymic").unwrap()
            .as_object().unwrap();

        let gender = validate_enum(gender, None);

        get_random_element(patronymics.get(gender).unwrap()
            .as_array().unwrap().into_iter()
                .map(|i| i.as_str())).unwrap()
                    .to_string()
    }

    /// Generate random series of passport
    pub fn passport_series(year: Option<i32>) -> String {
        let year = year.unwrap_or_else(|| randint(10, Local::now().year().to_string()[2..].parse().unwrap()));
        let region = randint(1, 99);
        format!("{region:02} {year}")
    }

    /// Generate random passport number
    pub fn passport_number() -> i32 {
        randint(100000, 999999)
    }

    /// Generate a random passport number and series
    pub fn series_and_number(year: Option<i32>) -> String {
        format!("{} {}", Self::passport_series(year), Self::passport_number())
    }

    /// Generate snils with special algorithm
    pub fn snils() -> String {
        let number = randints(0, 9, 9);
        let control_code = (9..0).map(|i| number.get(9 - i).unwrap() * i).sum();
        let code = number.into_iter().join("");

        format!("{code}{:02}", if [100, 101].contains(&control_code) {
            0
        } else if control_code < 100 {
            control_code
        } else { // > 101
            match control_code % 101 { 
                100 => 0,
                other => other,
            }
        })
    }

    fn control_sum(nums: &Vec<i32>, t: &str) -> i32 {
        let digits = match t {
            "n2" => vec![7, 2, 4, 10, 3, 5, 9, 4, 6, 8],
            "n1" => vec![3, 7, 2, 4, 10, 3, 5, 9, 4, 6, 8],
            _ => panic!("Invalid t arg!"),
        };

        zip(digits, nums).map(|(d, n)| d * n).sum::<i32>() % 11 % 10
    }

    /// Generate random, but valid ``INN``
    pub fn inn() -> String {
        let mut numbers: Vec<i32> = randints(1, 9, 1).into_iter()
            .chain(randints(0, 9, 9).into_iter())
            .collect();
        
        numbers.push(Self::control_sum(&numbers, "n2"));
        numbers.push(Self::control_sum(&numbers, "n2"));
        numbers.into_iter().join("")
    }

    /// Generate random, but valid ``OGRN``
    pub fn ogrn() -> String {
        let numbers: Vec<i32> = randints(1, 9, 1).into_iter()
            .chain(randints(0, 9, 11).into_iter())
            .collect();
        
        let ogrn: u64 = numbers.into_iter().join("").parse().unwrap();
        let checksum = ogrn % 11 % 10;

        format!("{ogrn}{checksum}")
    }

    /// Generate random ``BIC`` (Bank ID Code)
    pub fn bic() -> String {
        let country_code = "04";
        let code = randint(1, 10);
        let bank_number = randint(0, 99);
        let bank_office = randint(50, 999);

        format!("{:02}{:02}{:02}{:03}", country_code, code, bank_number, bank_office)
    }

    /// Generate random ``KPP``
    pub fn kpp() -> String {
        let tax_code = get_random_element(TAX_CODES.iter());

        let reg_code = randint(1, 99);
        let reg_number = randint(1, 999);
        format!("{tax_code:04}{reg_code:02}{reg_number:03}")
    }
}
