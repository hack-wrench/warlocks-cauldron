use std::{
    collections::{
        hash_map::DefaultHasher,
        HashMap,
    },
    hash::{Hash, Hasher},
    cmp::Eq, sync::Mutex,
};

use num::Complex;
use counter::Counter;

use super::dependencies::*;


lazy_static! {
    static ref INCREMENTS: Mutex<Counter<u64>> = Mutex::new(Counter::new());
}

/// Methods collection for generate any numeric values
pub struct Numeric;

impl Numeric {
    /// Generate incremental number
    /// 
    /// Each call of this method returns an incrementing number (with the step of +1)
    pub fn increment<S: Hash>(some: S) -> usize {
        let mut hasher = DefaultHasher::new();
        some.hash(&mut hasher);
        INCREMENTS.lock().unwrap()
            .entry(hasher.finish())
            .and_modify(|counter| *counter += 1)
            .or_insert(1)
            .to_owned()
    }

    /// Reset incremental number
    /// 
    /// Each call of this method reset linked increment value
    pub fn reset_increment<S: Hash>(some: S) {
        let mut hasher = DefaultHasher::new();
        some.hash(&mut hasher);
        INCREMENTS.lock().unwrap()
            .entry(hasher.finish())
            .and_modify(|counter| *counter = 0);
    }


    /// Reset all incremental number
    /// 
    /// Each call of this method reset all counter
    pub fn reset_increments() {
        *INCREMENTS.lock().unwrap() = Counter::new();
    }

    /// Generate random float number in range [start, end]
    pub fn float_number(start: f32, end: f32) -> f32 {
        uniform(start, end)
    }

    /// Generate a vector of random float numbers
    pub fn floats(start: f32, end: f32, n: usize) -> Vec<f32> {
        (0..n).map(|_| Self::float_number(start, end)).collect()
    }

    /// Generate random integer from start to end
    ///
    /// Integers can be negative or positive numbers
    pub fn integer_number(start: i32, end: i32) -> i32 {
        randint(start, end)
    }
    
    /// Generate a vector of random integers
    ///
    /// Integers can be negative or positive numbers
    pub fn integers(start: i32, end: i32, n: usize) -> Vec<i32> {
        (0..n).map(|_| Self::integer_number(start, end)).collect()
    }

    /// Generate a random complex int number
    pub fn complex_int_number(start_real: i32, end_real: i32, start_imag: i32, end_imag: i32) -> Complex<i32> {
        Complex::new(randint(start_real, end_real), randint(start_imag, end_imag))
    }

    /// Generate a list of random complex int numbers
    pub fn int_complexes(start_real: i32, end_real: i32, start_imag: i32, end_imag: i32, n: usize) -> Vec<Complex<i32>> {
        (0..n).map(|_| Self::complex_int_number(start_real, end_real, start_imag, end_imag)).collect()
    }

    /// Generate a random complex float number
    pub fn complex_float_number(start_real: f32, end_real: f32, start_imag: f32, end_imag: f32) -> Complex<f32> {
        Complex::new(uniform(start_real, end_real), uniform(start_imag, end_imag))
    }

    /// Generate a list of random complex float numbers
    pub fn float_complexes(start_real: f32, end_real: f32, start_imag: f32, end_imag: f32, n: usize) -> Vec<Complex<f32>> {
        (0..n).map(|_| Self::complex_float_number(start_real, end_real, start_imag, end_imag)).collect()
    }
}
