use warlocks_cauldron::*;

fn main() {
    let complex = ComplexProvider::new(Locale::EN);
    println!("Person: {}", complex.person.full_name(None, false));
    println!("Telephone: {}", complex.person.telephone(None));
    println!("Address: {}", complex.address.full_address());
    println!("Birthday: {}", Datetime::date(1940, 2000));
    println!("Weight: {} kg", Person::weight(30, 90));
    println!("Height: {} m", Person::height(1.5, 2.0));
}
