use std::collections::HashMap;
use crate::core::services::loader::LoaderWords;

pub struct LoaderWordsWithoutAccents {
    map_char: HashMap<char, char>
}

impl LoaderWords<Vec<String>> for LoaderWordsWithoutAccents {
    fn load_words(&self) -> Vec<String> {
        // vec![]
        let data = include_str!("../../../data/french.txt");
        data
            .split("\n")
            .filter(|e| Self::has_invalid_char(*e))
            .map(|word| self.replace_accents(word))
            .map(|e| e.to_string())
            .collect::<Vec<_>>()
    }
}

impl LoaderWordsWithoutAccents {

    pub fn new() -> Self {
        Self {
            map_char: HashMap::from([
                ('é', 'e'),
                ('ê', 'e'),
                ('è', 'e'),
                ('ë', 'e'),
                ('ù', 'u'),
                ('û', 'u'),
                ('ü', 'u'),
                ('î', 'i'),
                ('ï', 'i'),
                ('à', 'a'),
                ('ç', 'c'),
            ])
        }
    }

    pub fn has_invalid_char(word: &str) -> bool {
        word
            .contains(|caracter| caracter == '-')
    }

    pub fn replace_accents(&self, word: &str) -> String {
        word
            .chars()
            .into_iter()
            .map(|caracter| {
                self.map_char
                    .get(&caracter.clone())
                    .map(|char| char.clone())
                    .unwrap_or(caracter)
            })
            .collect::<String>()
    }
}