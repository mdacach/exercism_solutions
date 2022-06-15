use std::collections::HashSet;

fn sort_word_case_insensitive(word: &str) -> String {
    let word = word.to_lowercase();
    let mut as_chars: Vec<char> = word.chars().collect();
    as_chars.sort_unstable();

    as_chars.iter().collect()
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut sorted_word_list = Vec::new();
    for w in possible_anagrams {
        sorted_word_list.push(sort_word_case_insensitive(w));
    }

    let target_word = sort_word_case_insensitive(word);

    let lowercase_input_word = word.to_lowercase();
    let mut results = HashSet::new();
    for (sorted_word, original_word) in sorted_word_list.iter().zip(possible_anagrams) {
        let is_anagram = *sorted_word == target_word;
        let same_as_input = original_word.to_lowercase() == lowercase_input_word;
        if is_anagram && !same_as_input {
            results.insert(*original_word);
        }
    }

    results
}
