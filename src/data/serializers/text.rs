use super::dependencies::*;


#[derive(Serialize, Deserialize)]
pub struct Text {
    pub alphabet: Alphabet,
    pub answers: Vec<String>,
    pub color: Vec<String>,
    pub level: Vec<String>,
    pub quotes: Vec<String>,
    pub text: Vec<String>,
    pub words: Words,
}

#[derive(Serialize, Deserialize)]
pub struct Alphabet {
    pub uppercase: Vec<String>,
    pub lowercase: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Words {
    pub bad: Vec<String>,
    pub normal: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Street {
    pub name: Vec<String>,
    pub suffix: Vec<String>,
}
