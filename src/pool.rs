use crate::random::get_random_element;

pub struct RandomPool<T>(Vec<T>);

impl<T> RandomPool<T> {
    pub fn new(vec: Vec<T>) -> Self {
        Self(vec)
    }

    pub fn get(&self) -> &T {
        get_random_element(self.0.iter())
    }

    pub fn get_mut(&mut self) -> &mut T {
        get_random_element(self.0.iter_mut())
    }
}
