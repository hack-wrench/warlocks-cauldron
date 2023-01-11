use warlocks_cauldron::*;


fn main() {
    println!("Denmark CPR: {}", DenmarkSpecProvider::cpr());

    println!("Fedex code: {}", USASpecProvider::tracking_number(TrackingService::FEDEX));
    println!("SSN code: {}", USASpecProvider::ssn());

    println!("Italy fiscal code: {}", ItalySpecProvider::fiscal_code(Some(Gender::MALE)));

    println!("Netherlands BSN: {}", NetherlandsSpecProvider::bsn());
}
