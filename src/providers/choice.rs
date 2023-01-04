use super::dependencies::*;


pub fn choice<'a, T>(items: &'a Vec<T>, length: usize) -> Vec<&'a T> {
    get_random_elements(items.iter(), length)
}

pub fn choice_unique<'a, T: std::cmp::Eq>(items: &'a Vec<T>, length: usize) -> Vec<&'a T> {
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
            uniques.remove(rand_usize(0, uniques.len() - 1))
        );

        if output.len() == length {
            break
        }
    }

    output
}
