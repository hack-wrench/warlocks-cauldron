use crate::random::*;

/// Methods collection for generating a random choice from items in a sequence
pub struct Choice;

impl Choice {
    /// Get a randomly-chosen item from iter
    pub fn get<T, V: Iterator<Item = T>>(iter: V) -> T {
        get_random_element(iter)
    }

    /// Get a bool with a probability p of being true.
    pub fn prob(p: f64) -> bool {
        rand_bool(p)
    }

    /// Generate a randomly-chosen sequence or bare element from a sequence
    pub fn pick<'a, T>(items: &'a Vec<T>, length: usize) -> Vec<&'a T> {
        get_random_elements(items.iter(), length)
    }

    /// Generate a randomly-chosen unique sequence or bare element from a sequence
    pub fn pick_unique<'a, T: std::cmp::Eq>(items: &'a Vec<T>, length: usize) -> Vec<&'a T> {
        let mut output = vec![];

        let mut uniques: Vec<&T> = vec![];
        for item in items.iter() {
            if !uniques.contains(&item) {
                uniques.push(item);
            }
        }

        loop {
            if uniques.is_empty() {
            break 
            }

            output.push(
                uniques.remove(randint(0, uniques.len() - 1))
            );

            if output.len() == length {
                break
            }
        }

        output
    }
}

