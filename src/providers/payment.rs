use regex::Regex;

use crate::Person;

use super::dependencies::*;


#[derive(Debug)]
pub struct CreditCard {
    pub number: String,
    pub expiration_date: String,
    pub owner: String,
    pub cvv: String,
}

/// Methods collection for generating data related to payments
pub struct Payment;

impl Payment {
    /// Generate a random CID
    pub fn cid() -> String {
        format!("{:.4}", randint(1, 9999))
    }

    /// Generate a random PayPal account
    pub fn paypal() -> String {
        Person::email(None, false)
    }

    /// Generate a random bitcoin address
    pub fn bitcoin_address() -> String {
        let type_ = get_random_element(vec!["1", "3"].into_iter());
        let characters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
        let address = get_random_elements(characters.chars(), 33);
        format!("{type_}{}", address.iter().join(""))
    }

    /// Generate a random Ethereum address
    pub fn ethereum_address() -> String {
        format!("0x{}", hex::encode(urandom(20)))
    }

    /// Generate a random credit card number
    pub fn credit_card_network() -> &'static str {
        get_random_element(CREDIT_CARD_NETWORKS.iter())
    }

    /// Generate a random credit card number
    pub fn credit_card_number(card_type: Option<CardType>) -> String {
        let card_type = validate_variant(card_type, None);

        let mut regex = r"(\d{4})(\d{4})(\d{4})(\d{4})";
        let mut length = 16;
        let mut groups = 4;

        let start_number = match card_type {
            CardType::VISA => randint(4000, 4999),
            CardType::MASTER_CARD => get_random_element(vec![randint(2221, 2720), randint(5100, 5599)].into_iter()),
            CardType::AMERICAN_EXPRESS => {
                regex = r"(\d{4})(\d{6})(\d{5})";
                length = 15;
                groups = 3;
                get_random_element(vec![34, 37].into_iter())
            },
            _ => panic!("No valid card type!"),
        };

        let mut str_num = start_number.to_string();
        while str_num.len() <= length {
            str_num += &get_random_element("0123456789".chars()).to_string()
        } 

        str_num.push_str(&luhn::checksum(str_num.as_bytes()).to_string());

        let capt = Regex::new(regex).unwrap().captures_iter(&str_num)
            .next().unwrap();

        (1..=groups).map(|i| capt.get(i).unwrap().as_str()).join(" ")
    }

    /// Generate a random expiration date for credit card
    ///
    /// Arguments:
    /// * `minimum` -  Date of issue
    /// * `maximum` - Maximum of expiration date
    pub fn credit_card_expiration_date(minimum: i32, maximum: i32) -> String {
        format!("{:02}/{}", randint(1, 12), randint(minimum, maximum))
    }

    /// Generate a random CVV
    pub fn cvv() -> String {
        format!("{:03}", randint(1, 999))
    }

    /// Generate full credit card
    pub fn credit_card() -> CreditCard {
        CreditCard {
            number: Self::credit_card_number(None),
            expiration_date: Self::credit_card_expiration_date(16, 25),
            owner: Person(Locale::EN).full_name(None, false).to_uppercase(),
            cvv: Self::cvv(),
        }
    }
}
