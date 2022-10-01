#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    
    let magazine_words = magazine.iter().fold(HashMap::new(), |mut words, word| {
        *words.entry(word).or_insert(0) += 1;
        words
    });

    let note_words = note.iter().fold(HashMap::new(), |mut words, word| {
        *words.entry(word).or_insert(0) += 1;
        words
    });

    note_words
        .iter()
        .all(&|(w, count)| magazine_words.get(w).unwrap_or(&0) >= count)
    
    
    // let mut words = HashMap::new();

    // for word in magazine {
    //     let entry = words.entry(word).or_insert(0);
    //     *entry += 1;
    // }

    // for word in note {
    //     let entry = words.entry(word).or_insert(0);
        
    //     if *entry == 0 {
    //         return false;
    //     };
    //     *entry -= 1;
    // }

    // true

    
}
