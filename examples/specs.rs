use warlocks_cauldron::*;


fn main() {
    println!("Denmark CPR: {}", DenmarkSpecProvider::cpr());

    println!("Fedex code: {}", USASpecProvider::tracking_number(TrackingService::FEDEX));
    println!("SSN code: {}",  USASpecProvider::ssn());

    println!("Italy fiscal code: {}", ItalySpecProvider::fiscal_code(Some(Gender::MALE)));

    println!("Netherlands BSN: {}", NetherlandsSpecProvider::bsn());

    println!("Poland NIP: {}",   PolandSpecProvider::nip());
    println!("Poland PESEL: {}", PolandSpecProvider::pesel(None, None));
    println!("Poland REGON: {}", PolandSpecProvider::regon());

    println!("Brazil CPF: {}",  BrazilSpecProvider::cpf(true));
    println!("Brazil CNPJ: {}", BrazilSpecProvider::cnpj(true));

    println!("Russian sentence: {}",   RussiaSpecProvider::generate_sentence());
    println!("Russian patronymic: {}", RussiaSpecProvider::patronymic(Some(Gender::FEMALE)));
    println!("Russian seriens and number: {}", RussiaSpecProvider::series_and_number(None));
    println!("Russian SNILS: {}", RussiaSpecProvider::snils());
    println!("Russian OGRN: {}", RussiaSpecProvider::ogrn());
    println!("Russian BIC: {}", RussiaSpecProvider::bic());
    println!("Russian KPP: {}", RussiaSpecProvider::kpp());
}
