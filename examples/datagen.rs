use warlocks_cauldron::{Locale, Address, Code};

fn main() {
    let address = Address(Locale::EN);
    println!("Address: {}", address.full_address());

    println!("IMEI: {}", Code::imei());
}
