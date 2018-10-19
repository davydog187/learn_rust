const VOWELS: [char; 6] = ['a', 'e', 'i', 'o', 'u', 'y'];

pub fn translate(word: &str) -> String {
    let mut chars = word.chars();

    if VOWELS.contains(&first) {
        format!("{} hay", rest.to_owned())
    } else {
        rest.to_owned() + first
    }
}
