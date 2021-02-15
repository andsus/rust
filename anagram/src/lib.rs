use std::collections::{HashSet, HashMap};
use std::iter::FromIterator;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    // character count map
    let char_map : HashMap<char, usize> = HashMap::new();
    let char_count_map: HashMap<char, usize> = word.to_lowercase().chars().fold( char_map, |mut chrs, c|{
        *chrs.entry(c).or_default() += 1;
        chrs

    } ) ;
    

    HashSet::from_iter( 
        possible_anagrams.iter().filter( |w| {
            let w = w.to_lowercase();
            w.len() == word.len() && w != word.to_lowercase() &&  { // 
                w.chars().fold(HashMap::new(), |mut chrs,c| {
                    *chrs.entry(c).or_default() += 1;
                    chrs
                }) == char_count_map
            }
        })
        .cloned(),

    )
        

}
