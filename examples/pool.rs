use warlocks_cauldron::*;

fn main() {
    let address_pool = RandomPool::new(vec![
        Address(Locale::EN), Address(Locale::RU),
    ]);

    println!("City: {}", address_pool.get().city());

    let complex_pool = RandomPool::new(vec![
        ComplexProvider::new(Locale::EN), ComplexProvider::new(Locale::RU),
    ]);

    println!("Words: {}", complex_pool.get().text.word())
}