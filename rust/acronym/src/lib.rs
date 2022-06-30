pub fn abbreviate(phrase: &str) -> String {
    let mut acronym = String::new();

    let splittable = |c: char| c.is_whitespace() || (c.is_ascii_punctuation() && c != '\'');
    let words: Vec<&str> = phrase.split(splittable).filter(|x| !x.is_empty()).collect();

    for word in words {
        let capitalized_chars: Vec<char> = word.chars().filter(|c| c.is_uppercase()).collect();
        if capitalized_chars.is_empty() || (capitalized_chars.len() == word.len()) {
            // All capitalized, just get the first one
            acronym.push(word.chars().next().unwrap().to_ascii_uppercase());
        } else {
            for char in capitalized_chars {
                acronym.push(char);
            }
        }
    }

    acronym
}
