use std::fs::File;
use std::io::{BufRead, BufReader};

pub(crate) fn load_words() -> Vec<String> {
    let data = include_str!("../../data/french.txt");
    data
        .split("\n")
        .map(|e| e.to_string())
        .collect::<Vec<_>>()
}