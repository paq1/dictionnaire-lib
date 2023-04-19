use std::fs::File;
use std::io::{BufRead, BufReader};

pub(crate) fn load_words() -> Vec<String> {
    File::open("./data/french.txt")
        .map(|file| {
            let reader = BufReader::new(file);

            reader
                .lines()
                .into_iter()
                .map(|line| {
                    line.unwrap_or("".to_string())
                })
                .filter(|word| !word.trim().is_empty())
                .collect::<Vec<_>>()
        })
        .unwrap_or(vec![])
}