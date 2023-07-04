// iterators2.rs
//
// In this exercise, you'll learn some of the unique advantages that iterators
// can offer. Follow the steps to complete the exercise.
//
// Execute `rustlings hint iterators2` or use the `hint` watch subcommand for a
// hint.

// Step 1.
// Complete the `capitalize_first` function.
// "hello" -> "Hello"
pub fn capitalize_first(input: &str) -> String {
    let mut buf = String::with_capacity(input.len());
    
    // I used a match statement here first, checking for whitespace
    // but it ended up making no sense returning a space in the first
    // place, so I just trimmed...
    //  But then it won't pass the iter to string test :D
    //
    //  This is a "solution" made without watching the hints...

    for (idx, character) in input.chars().enumerate() {
        let matching: char = 
            match idx {
                0 => character.to_ascii_uppercase(),
                _ if character.is_whitespace() => character,
                _ => character
                };
        buf.push(matching)
    };

    buf
}

// Step 2.
// Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    let output = words
        .into_iter()
        .map(|word| capitalize_first(word))            
        .collect();

    output
}

// Step 3.
// Apply the `capitalize_first` function again to a slice of string slices.
// Return a single string.
// ["hello", " ", "world"] -> "Hello World"
pub fn capitalize_words_string(words: &[&str]) -> String {
    words
        .into_iter()
        .map(|word| capitalize_first(word))
        .collect()

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
