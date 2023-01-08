use warlocks_cauldron::*;


fn main() {
    println!("Address: {}", Address(Locale::EN).full_address());

    println!("IMEI: {}", Code::imei());

    println!("Mnemonic Phrase: {}", Cryptographic::mnemonic_phrase());

    // start, end
    println!("Datetime: {}", Datetime::datetime(1984, 2077));

    // words count
    println!("Words: {:?}", Text(Locale::EN).words(5));

    println!("Filename: {}", File::file_name(None));

    // scheme, tld, subdomains
    println!("DSN: {}", Development::dsn(Some(DSNType::POSTGRES), Some(TLDType::CCTLD), Some(vec!["shop", "admin"])));

    println!("Mac: {}", Internet::mac());

    println!("Company: {}", Finance(Locale::EN).company());

    println!("Drink: {}", Food(Locale::EN).drink());

    println!("Manufacturer: {}", Hardware::manufacturer());

    // sequence, length
    println!("Choice: {:?}", Choice::pick(&vec!["a", "b", "c"], 5));
    println!("Unique choice: {:?}", Choice::pick_unique(&vec!["a", "a", "b", "c"], 5));

    // anything hashable object
    println!("Increment a=1: {}", Numeric::increment("a"));
    println!("Increment a+1: {}", Numeric::increment("a"));
    println!("Increment 1=1: {}", Numeric::increment(1));
    println!("Increment a+1: {}", Numeric::increment("a"));

    println!("Project path: {}", Path::new(PlatformType::detect()).project_dir());

    // None or Some(locale) for random or locale get transport code
    println!("USA transport code: {}", Transport::vehicle_registration_code(Some(Locale::EN)));

    // length of sequence
    println!("DNA sequence: {}", Science::dna_sequence(10));
}
