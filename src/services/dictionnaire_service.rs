use rand::Rng;
use crate::services::loader::load_words;

pub struct DictionnaireService {
    pub words: Vec<String>
}

impl DictionnaireService {
    pub fn new() -> Self {
        Self {
            words: load_words()
        }
    }

    pub fn random_word(&self) -> String {
        let taille = self.words.len() as u32;

        let mut rnd = rand::thread_rng();
        let random_index = rnd.gen_range(0..taille);

        self.words.iter().nth(random_index as usize)
            .map(|v| v.to_string())
            .unwrap_or("".to_string())
    }
}
