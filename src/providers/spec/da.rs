use chrono::Datelike;

use super::super::{Choice, Datetime, dependencies::*};
use std::iter::zip;


pub struct DenmarkSpecProvider;

impl DenmarkSpecProvider {
    fn calculate_century_selector(year: i32) -> i32 {
        if 1858 <= year && year < 1900 {
            return randint(5, 8);
        }

        if 1900 <= year && year < 1937 {
            return randint(0, 3);
        }

        if 1937 <= year && year < 2000 {
            return get_random_element(vec![4, 9].into_iter());
        }

        if 2000 <= year && year < 2037 {
            return randint(4, 9);
        }

        panic!("Invalid year!")
    }

    /// 
    fn calculate_checksum(cpr_nr_no_checksum: String) -> u32 {
        let cpr_digits: Vec<u32> = cpr_nr_no_checksum.chars()
            .into_iter().map(|c| c.to_digit(10).unwrap()).collect();
        
        let checksum_factors = vec![4, 3, 2, 7, 6, 5, 4, 3, 2];

        let remainder: u32 = zip(cpr_digits, checksum_factors)
            .into_iter().map(|(d, c)| d * c).sum();

        match remainder % 11 {
            0 => 0,
            other => 11 - other,
        }
    }

    /// Generate a serial number and checksum from cpr_century
    fn generate_serial_checksum(cpr_century: &str) -> (String, u32) {
        let serial_number = format!("{:02}", randint(0, 99));
        let cpr_nr_no_checksum  = format!("{cpr_century}{serial_number}");
        let checksum = Self::calculate_checksum(cpr_nr_no_checksum);

        if checksum == 10 {
            Self::generate_serial_checksum(cpr_century)
        } else {
            (serial_number, checksum)
        }
    }

    /// Generate a random CPR number (Central Person Registry)
    pub fn cpr() -> String {
        let date = Datetime::date(1858, 2023);
        let cpr_date = date.format("%d%m%y").to_string();
        let century_selector = Self::calculate_century_selector(date.year());
        let cpr_century = format!("{cpr_date}{century_selector}");
        let (serial_number, checksum) = Self::generate_serial_checksum(&cpr_century);
        format!("{cpr_century}{serial_number}{checksum}")
    }
}
