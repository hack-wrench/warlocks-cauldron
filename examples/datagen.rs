use warlocks_cauldron::{Address, Locale};

fn main() {
    let address = Address(Locale::RU);
    println!("{}", address.street_name())
}
