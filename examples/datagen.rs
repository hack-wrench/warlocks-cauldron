use warlocks_cauldron::*;

fn main() {
    println!("Address: {}", Address(Locale::EN).full_address());

    println!("IMEI: {}", Code::imei());

    println!("Mnemonic Phrase: {}", Cryptographic::mnemonic_phrase());

    println!("Datetime: {}", Datetime::datetime(1984, 2077));

    println!("Words: {:?}", Text(Locale::EN).words(5));

    println!("Filename: {}", File::file_name(None));

    println!("Mac: {}", Internet::mac());

    println!("Company: {}", Finance(Locale::EN).company());
}
