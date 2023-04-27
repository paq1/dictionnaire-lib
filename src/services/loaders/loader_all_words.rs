use crate::core::services::loader::LoaderWords;

pub struct LoaderAllWords {}

impl LoaderWords<Vec<String>> for LoaderAllWords {
    fn load_words(&self) -> Vec<String> {
        let data = include_str!("../../../data/french.txt");
        data
            .split("\n")
            .map(|e| e.to_string())
            .collect::<Vec<_>>()
    }
}