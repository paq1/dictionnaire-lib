use crate::core::services::loader::LoaderWords;

pub struct LoaderWordsWithoutAccents {}

impl LoaderWords<Vec<String>> for LoaderWordsWithoutAccents {
    fn load_words(&self) -> Vec<String> {
        // vec![]
        let data = include_str!("../../../data/french.txt");
        data
            .split("\n")
            .filter(|e| Self::has_invalid_char(*e))
            .map(|e| e.to_string())
            .collect::<Vec<_>>()
    }
}

impl LoaderWordsWithoutAccents {
    pub fn has_invalid_char(word: &str) -> bool {
        word
            .contains(|caracter| caracter < 'a' || caracter > 'z')
    }
}