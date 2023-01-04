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

    println!("Drink: {}", Food(Locale::EN).drink());

    println!("Manufacturer: {}", Hardware::manufacturer());

    println!("Choice: {:?}", choice(&vec!["a", "b", "c"], 5));
    println!("Unique choice: {:?}", choice_unique(&vec!["a", "a", "b", "c"], 5));
}
