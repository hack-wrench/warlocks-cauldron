use std::iter::zip;

use chrono::{NaiveDate, Datelike};

use super::super::dependencies::*;
use crate::Datetime;


pub struct PolandSpecProvider;

impl PolandSpecProvider {
    /// Generate random valid 10-digit NIP
    pub fn nip() -> String {
        let mut nip_digits: Vec<u32> = randint(101, 999).to_string().chars()
            .map(|c| c.to_digit(10).unwrap())
            .chain(randints(0, 9, 6).into_iter())
            .collect();

        let nip_coefficients = vec![6, 5, 7, 2, 3, 4, 5, 6, 7];

        let sum_v: u32 = zip(&nip_digits, &nip_coefficients).map(|(d, c)| d * c).sum();

        let checksum_digit = sum_v % 11;
        if checksum_digit > 9 {
            return Self::nip();
        }

        nip_digits.push(checksum_digit);

        nip_digits.into_iter().join("")
    }

    /// Generate random 11-digit PESEL
    pub fn pesel(birth_date: Option<NaiveDate>, gender: Option<Gender>) -> String {
        let birth_date = birth_date.unwrap_or_else(|| Datetime::date(1940, 2023));

        let year = birth_date.year();
        let mut month = birth_date.month();
        let day = birth_date.day();

        if 1800 <= year && year <= 1899 {
            month += 80;
        } else if 2000 <= year && year <= 2099 {
            month += 20;
        } else if 2100 <= year && year <= 2199 {
            month += 40
        } else if 2200 <= year && year <= 2299 {
            month += 60
        }

        let series_number = randint(0, 999);
        let mut pesel_digits: Vec<u32> = format!("{year:02}{month:02}{day:02}{series_number:03}")
            .chars().map(|c| c.to_digit(10).unwrap()).collect();

        let gender_digit = get_random_element(match validate_variant(gender, None) {
            Gender::MALE => vec![1, 3, 5, 7, 9].into_iter(),
            Gender::FEMALE => vec![0, 2, 4, 6, 8].into_iter(),
            _ => panic!("This gender doesnt accept!"),
        });

        pesel_digits.push(gender_digit);

        let pesel_coeffs = vec![9, 7, 3, 1, 9, 7, 3, 1, 9, 7];

        let sum_v: u32 = zip(&pesel_digits, &pesel_coeffs).map(|(d, c)| d * c).sum();

        let checksum_digit = sum_v % 10;
        pesel_digits.push(checksum_digit);

        pesel_digits.into_iter().join("")
    }

    /// Generate random valid 9-digit REGON
    pub fn regon() -> String {
        let mut regon_digits = randints(0, 9, 8);
        let regon_coeffs = vec![8, 9, 2, 3, 4, 5, 6, 7];

        let sum_v: u32 = zip(&regon_digits, &regon_coeffs).map(|(d, c)| d * c).sum();

        let mut checksum_digit = sum_v % 11;
        if checksum_digit > 9 {
            checksum_digit = 0;
        }

        regon_digits.push(checksum_digit);

        regon_digits.into_iter().join("")
    }
}
