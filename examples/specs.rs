use warlocks_cauldron::*;


fn main() {
    println!("Denmark CPR: {}", DenmarkSpecProvider::cpr());

    println!("Fedex code: {}", USASpecProvider::tracking_number(TrackingService::FEDEX));
    println!("SSN code: {}", USASpecProvider::ssn());
}
