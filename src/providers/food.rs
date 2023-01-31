use super::dependencies::*;


/// Struct for generating data related to food
pub struct Food<'a>(pub &'a Locale);

impl<'a> Food<'a> {
    /// Private. Return global parsed data from own locale
    fn data(&self) -> &ParsedData { self.0.get_data() }

    /// Get a random spices or herbs
    pub fn vegetable(&self) -> &str {
        get_random_element(self.data().food.vegetables.iter())
    }

    /// Get a random fruit or berry
    pub fn fruit(&self) -> &str {
        get_random_element(self.data().food.fruits.iter())
    }

    /// Get a random dish
    pub fn dish(&self) -> &str {
        get_random_element(self.data().food.dishes.iter())
    }

    /// Get a random spices or herbs
    pub fn spices(&self) -> &str {
        get_random_element(self.data().food.spices.iter())
    }
    
    /// Get a random drink
    pub fn drink(&self) -> &str {
        get_random_element(self.data().food.drinks.iter())
    }
}
