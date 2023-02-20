# 🧙‍♀️ Warlock's Cauldron
🦀 Fake Data Generator written in Rust - fully inspired by https://mimesis.name 🐍

## Installation
All localizations are enabled by default feature, you can specify localizations in features!
```toml
[dependencies.warlocks-cauldron]
version = "0.26.9"
# git = "https://github.com/hack-wrench/warlocks-cauldron"
# features = ["en"] # For example to use only english localization
```

## Supported languages
There are currently 26 languages available: `cs, da, de, el, en, es, et, fa, fi, fr, hu, is, it, ja, kk, ko, nl, no, pl, pt, ru, sk, sv, tr, uk, zh`

## Supported providers
There are currently 18 providers available: `Address, Choice, Code, Cryptographic, Date, Development, File, Finance, Food, Hardware, Internet, Numeric, Path, Payment, Person, Science, Text, Transport`

## Examples
Visit [`/examples`](https://github.com/hack-wrench/warlocks-cauldron/tree/main/examples) for detailed examples. In the process of development it was decided to make the workflow as close to [mimesis](https://mimesis.name) as possible, most of the methods and namespace were taken from there.
``` rust
use warlocks_cauldron::*;

fn main() {
    // A common option for most providers
    let complex = ComplexProvider::new(&Locale::EN);
    println!("Person: {}", complex.person.full_name(None, false));
    println!("Telephone: {}", complex.person.telephone(None));
    println!("Address: {}", complex.address.full_address());
    println!("Birthday: {}", Datetime::date(1940, 2000));
    println!("Weight: {} kg", Person::weight(30, 90));
    println!("Height: {} m", Person::height(1.5, 2.0));

    // But you can also use single providers
    let russian_person = Person(&Locale::RU);
    println!("Their Russian friend: {}", russian_person.full_name(Some(Gender::MALE), false));
}
```


## License
This project is licensed under the [GPL-3.0 license](https://github.com/hack-wrench/warlocks-cauldron/blob/main/LICENSE)
