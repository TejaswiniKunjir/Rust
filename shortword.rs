fn shortest_word(sentence: &str) -> Option<&str> {
    sentence
        .split_whitespace()
        .min_by_key(|word| word.len())
}

fn main() {
    let sentence = "hi goog morning";
    match shortest_word(sentence) {
        Some(shortest) => println!("The shortest word is: {}", shortest),
        None => println!("No words found in the sentence"),
    }
}