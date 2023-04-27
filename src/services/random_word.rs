use rand::Rng;
use crate::core::services::words::WordsContainer;

pub trait RandomWord: WordsContainer {
    fn random_word(&self) -> String {
        let taille = self.get_words().len() as u32;

        let mut rnd = rand::thread_rng();
        let random_index = rnd.gen_range(0..taille);

        self.get_words().iter().nth(random_index as usize)
            .map(|v| v.to_string())
            .unwrap_or("".to_string())
    }
}