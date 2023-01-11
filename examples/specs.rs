use warlocks_cauldron::*;


fn main() {
    println!("Denmark CPR: {}", DenmarkSpecProvider::cpr());

    println!("Fedex code: {}", USASpecProvider::tracking_number(TrackingService::FEDEX));
    println!("SSN code: {}", USASpecProvider::ssn());

    println!("Italy fiscal code: {}", ItalySpecProvider::fiscal_code(Some(Gender::MALE)));

    println!("Netherlands BSN: {}", NetherlandsSpecProvider::bsn());

    println!("Poland NIP: {}", PolandSpecProvider::nip());
    println!("Poland PESEL: {}", PolandSpecProvider::pesel(None, None));
    println!("Poland REGON: {}", PolandSpecProvider::regon());
}
