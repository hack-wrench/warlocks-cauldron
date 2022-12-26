use warlocks_cauldron::{Address, Locale};

fn main() {
    let address = Address(Locale::EN);
    println!("Address:\n{}\n", address.full_address());
}
