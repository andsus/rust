pub fn reverse(input: &str) -> String {
    
    // Use iterator of chars() directly per Harrison suggestion :)
    input.chars().rev().collect()
}
