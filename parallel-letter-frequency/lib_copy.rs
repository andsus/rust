use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;

const THREAD_MAX: usize = 32;
pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    if worker_count < 2 || input.len() / worker_count < THREAD_MAX {
          return single_thread(input);
    }
    // setup multiple producer single consumer channel
    let (tx,rx) = mpsc::channel();
    let size = input.len();
    let each = size / worker_count;

    for i in 0..worker_count {
        let right = i * each;
        let slice: Vec<String> = input[right..right + each].iter()
            .map(|s| String::from(*s)).collect();
        let clone = tx.clone();
        // let new spawn own the tx - move
        thread::spawn(move || {
            let mut map = HashMap::new();
            for line in slice {
                for ch in line.chars() {
                    push_map(&mut map, ch, 1);
                }
            }
            clone.send(map).unwrap_or_default();
        });
    }
    drop(tx);
    let mut map = HashMap::new();
    // receiving multiple map
    for j in rx {
        reduce(&mut map, &j);
    }
    map

    // unimplemented!(
    //     "Count the frequency of letters in the given input '{:?}'. Ensure that you are using {} to process the input.",
    //     input,
    //     match worker_count {
    //         1 => "1 worker".to_string(),
    //         _ => format!("{} workers", worker_count),
    //     }
    // );
}

fn single_thread(input: &[&str]) -> HashMap<char,usize> {
    let mut map = HashMap::new();
    for line in input.iter() {
        for ch in line.chars() {
            push_map(&mut map, ch, 1)
        }
    }
    map
}

fn push_map(item: &mut HashMap<char, usize>, key: char, value: usize ) {
    if key.is_numeric() || key.is_ascii_punctuation() {
        return;
    }
    let key = key.to_ascii_lowercase();
    if item.contains_key(&key) {
        *item.get_mut(&key).unwrap() += value;
    } else {
        item.insert(key,value);
    }
}

fn reduce(first: &mut HashMap<char,usize>, last: &HashMap<char,usize> ) {
    for (key,value) in last.iter() {
        push_map(first, *key, *value);
    }
}