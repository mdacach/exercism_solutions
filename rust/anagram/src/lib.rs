use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word_lowercase = word.to_lowercase();
    let word_sorted = get_sorted(&word_lowercase);

    possible_anagrams
        .iter()
        .filter(|candidate| {
            let candidate = candidate.to_lowercase();
            // Length between cases may differ in some cases, so we compare
            // them both lowercase
            if candidate.len() != word_lowercase.len() {
                return false;
            }
            if candidate == word_lowercase {
                return false;
            }
            get_sorted(&candidate) == word_sorted
        })
        .copied()
        .collect()
}

fn get_sorted(word: &str) -> Vec<char> {
    let mut as_chars: Vec<char> = word.chars().collect();
    as_chars.sort_unstable();

    as_chars
}
