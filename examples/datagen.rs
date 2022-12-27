use warlocks_cauldron::*;

fn main() {
    let address = Address(Locale::EN);
    println!("Address: {}", address.full_address());

    println!("IMEI: {}", Code::imei());

    println!("Mnemonic Phrase: {}", Cryptographic::mnemonic_phrase());
}
