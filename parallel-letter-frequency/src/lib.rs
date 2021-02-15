use std::collections::HashMap;
use rayon::prelude::*;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    input.par_chunks(worker_count) // using rayon slice par_chunks
        .map(LetterFrequency::from_lines)
        .reduce(LetterFrequency::new, LetterFrequency::merge)
        .0

}


pub struct LetterFrequency(HashMap<char, usize>);

impl LetterFrequency {
    fn new() -> Self { Self(HashMap::new()) }
    fn from_lines(line: &[&str]) -> LetterFrequency {
        line.iter()
            .flat_map(|s| s.chars())
            .map(|c| c.to_ascii_lowercase())
            .filter(|c| c.is_lowercase())
            .fold(LetterFrequency::new(), |freq_map, key| freq_map.increment_count(key,1))
    }

    fn increment_count(mut self, key: char, value: usize) -> Self {
        *self.0.entry(key).or_insert(0) += value;
        self
    }
    fn merge(self, fmap2: Self) -> Self {
        fmap2.0
            .into_iter()
            .fold(self, |freq_map, (key,value)| freq_map.increment_count(key,value))
    }
}