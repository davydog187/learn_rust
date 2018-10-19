/**
 * Convert strings to pig latin.
 * The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.”
 * Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”).
 * Keep in mind the details about UTF-8 encoding!
 */
pub fn translate(word: &str) -> String {
    let mut chars = word.chars();

    let first_char = match chars.next() {
        Some(letter) => letter,
        None => return String::new(),
    };

    match first_char {
        'a' | 'e' | 'i' | 'o' | 'u' => format!("{}-hay", word),
        _ => format!("{}-{}ay", chars.as_str(), first_char),
    }
}
