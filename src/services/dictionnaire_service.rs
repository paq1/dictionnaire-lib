use crate::core::services::loader::LoaderWords;
use crate::services::random_word::RandomWord;
use crate::core::services::words::WordsContainer;

pub struct DictionnaireService {
    pub words: Vec<String>,
}

impl WordsContainer for DictionnaireService {
    fn get_words(&self) -> &Vec<String> {
        &self.words
    }
}

impl RandomWord for DictionnaireService {}

impl DictionnaireService {
    pub fn new(loader: Box<dyn LoaderWords<Vec<String>>>) -> Self {
        Self {
            words: loader.load_words(),
        }
    }
}
