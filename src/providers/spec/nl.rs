use super::super::dependencies::*;


/// Methods collection provides special data for Netherlands (nl)
pub struct NetherlandsSpecProvider;

impl NetherlandsSpecProvider {
    fn is_valid_bsn(number: u32) -> bool {
        let mut total: i32 = 0;
        let mut multiplier: i32 = 9;

        for char in number.to_string().chars() {
            multiplier = match multiplier {
                1 => -multiplier,
                other => other,
            };

            total += char.to_digit(10).unwrap() as i32 * multiplier;
            multiplier -= 1;
        }

        return total % 11 == 0
    }

    /// Generate a random, but valid ``Burgerservicenummer``
    pub fn bsn() -> u32 {
        let (a, b) = (100000000, 999999999);

        loop {
            let sample = randint(a, b);
            if Self::is_valid_bsn(sample) {
                return sample;
            }
        }
    }

    /// Generate a random, but valid ``Burgerservicenummer`` | *An allias for .bsn()* 
    pub fn burgerservicenummer() -> u32 {
        Self::bsn()
    }
}
