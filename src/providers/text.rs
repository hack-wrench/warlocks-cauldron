use super::dependencies::*;


/// A struct for generating text data
pub struct Text(pub Locale);

impl Text {
    /// Private. Return global parsed data from own locale
    fn data(&self) -> &ParsedData { self.0.get_data() }
    

    /// Get an alphabet for current locale
    /// 
    /// return example: Alphabet
    ///
    /// # Arguments
    /// * `lower_case` - Return alphabet in lower case
    pub fn alphabet(&self, lower_case: bool) -> &Vec<String> {
        match lower_case {
            true => &self.data().text.alphabet.lowercase,
            false => &self.data().text.alphabet.uppercase,
        }
    }

    /// Get an alphabet for current locale
    /// 
    /// return example: critical
    pub fn level(&self) -> &str {
        get_random_element(self.data().text.level.iter())
    }

    /// Generate the text
    /// 
    /// return example: 
    /// 
    /// # Arguments
    /// * `quantity` - Quantity of sentences.
    pub fn text(&self, quantity: usize) -> String {
        get_random_elements(self.data().text.text.iter(), quantity)
            .iter().join(" ")
    }

    /// Get a random sentence from text
    /// 
    /// return example: Sentence
    pub fn sentence(&self) -> String {
        self.text(1)
    }

    /// Get a random title
    /// 
    /// return example: The title
    pub fn title(&self) -> String {
        self.text(1)
    }
    
    /// Generate a vec of random words
    /// 
    /// return example: vec!\[science, network, god, octopus, love\]
    /// 
    /// # Arguments
    /// * `quantity` - Quantity of words
    pub fn words(&self, quantity: usize) -> Vec<&String> {
        get_random_elements(self.data().text.words.normal.iter(), quantity)
    }

    /// Get a random word
    /// 
    /// return example: Science
    pub fn word(&self) -> &str {
        self.words(1).first().unwrap()
    }

    /// Get a random swear word
    /// 
    /// return example: Damn
    pub fn swear_word(&self) -> &str {
        get_random_element(self.data().text.words.bad.iter())
    }

    /// Get a random quote from movie
    /// 
    /// return example:  Bond... James Bond
    pub fn quote(&self) -> &str {
        get_random_element(self.data().text.quotes.iter())
    }

    /// Get a random quote from movie
    /// 
    /// return example: No
    pub fn answer(&self) -> &str {
        get_random_element(self.data().text.answers.iter())
    }

    /// Get a random name of color
    /// 
    /// return example: Red
    pub fn color(&self) -> &str {
        get_random_element(self.data().text.color.iter())
    }

    /// Generate a random hex color
    /// 
    /// return example: #d8346b
    /// 
    /// # Arguments
    /// * `safe` - Get safe Flat UI hex color
    pub fn hex_color(safe: bool) -> String {
        match safe {
            true => get_random_element(SAFE_COLORS.iter()).to_string(),
            false => format!("#{:#06x}", randint(0x000000, 0xFFFFFF)),
        }
    }

    /// Generate a random rgb color tuple
    /// 
    /// return example: (252, 85, 32)
    ///
    /// # Arguments
    /// * `safe` - Get safe RGB tuple
    pub fn rgb_color(safe: bool) -> (u8, u8, u8) {
        let color = Self::hex_color(safe).replacen("#", "", 1);
        (
            u8::from_str_radix(&color[0..2], 16).expect("Cant convert RGB hex to u8 tuple!"),
            u8::from_str_radix(&color[2..4], 16).expect("Cant convert RGB hex to u8 tuple!"),
            u8::from_str_radix(&color[4..6], 16).expect("Cant convert RGB hex to u8 tuple!"),
        )
    }
}
