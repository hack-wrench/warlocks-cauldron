use warlocks_cauldron::*;

fn main() {
    let some_pool = RandomPool::new(vec![
        "TEST", "test",  "TeSt", "tEsT",
    ]);

    println!("Something any: {}", some_pool.get());

    let address_pool = RandomPool::new(vec![
        Address(&Locale::EN), Address(&Locale::RU),
    ]);

    println!("Random address: {}", address_pool.get().city());

    let complex_pool = RandomPool::new(vec![
        ComplexProvider::new(&Locale::EN), ComplexProvider::new(&Locale::RU),
    ]);

    println!("Random complex provider: {}", complex_pool.get().person.name(Some(Gender::MALE)))
}
