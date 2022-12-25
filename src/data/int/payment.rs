use super::dependencies::*;


#[cfg(feature = "payment")]
lazy_static! {
    pub static ref CREDIT_CARD_NETWORKS: List = list! [
        "Visa",
        "MasterCard",
        "Chase",
        "American Express",
        "Discover"
    ];
}
