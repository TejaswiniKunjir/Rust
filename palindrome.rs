fn is_palindrome(s: &str) -> bool {
    let mut reversed = String::new();
    for c in s.chars().rev() {
        reversed.push(c);
    }
    s == reversed
}

fn main() {
    let s = "madam";
    let is_palindrome = is_palindrome(s);
    println!("Is '{}' a palindrome? {}", s, is_palindrome);
}