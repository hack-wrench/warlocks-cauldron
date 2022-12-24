use itertools::Itertools;
use rand::prelude::*;


pub fn randints(amount: usize, a: u32, b: u32) -> Vec<u32> {
    Vec::with_capacity(amount).into_iter()
        .map(|_: u32| rand::thread_rng().gen_range(a..b))
        .collect()
}

pub fn urandom(size: usize) -> Vec<u8> {
    Vec::with_capacity(size).into_iter()
        .map(|_: u8| rand::thread_rng().gen_range(0..8))
        .collect()
}

pub fn generate_string(str_seq: String, length: usize) -> String {
    Vec::with_capacity(length).into_iter()
        .map(|_: u8| get_random_element(str_seq.chars()).to_string())
        .collect::<Vec<String>>()
        .join("")
}


// e.g mask = "@###", char = "@", digit = "#")
pub fn custom_code(mask: String, char: char, digit: char) -> String {
    mask.chars().filter_map(|c| {
        if c == char {
            return Some(get_random_element(
                "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars()
            ).to_string())
        } else if c == digit {
            return Some(rand::thread_rng().gen_range(0..8).to_string())
        }

        None
    }).join("")
}

pub fn uniform(a: f32, b: f32) -> f32 {
    rand::random::<f32>() * a + (b - a)
}

pub fn randstr(unique: bool, length: usize) -> String {
    if unique {
        uuid::Uuid::new_v4().to_string()
    } else {
        Vec::with_capacity(length).into_iter()
            .map(|_: u8| get_random_element(
                "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars()
            ).to_string())
            .collect::<Vec<String>>().join("")
    }
}

pub fn get_random_element<T, V: Iterator<Item = T>>(vec: V) -> T {
    vec.choose(&mut rand::thread_rng()).unwrap()
}
