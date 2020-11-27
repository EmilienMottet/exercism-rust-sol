use std::collections::HashSet;
use std::iter::FromIterator;
use std::iter::Iterator;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let sorted_word = compute_str(word);
    let filtered = possible_anagrams.into_iter().filter(|&&w| {
        compute_str(&w).eq(&sorted_word) && !word.to_lowercase().eq(&w.to_lowercase())
    });
    filtered.cloned().collect()
}

fn compute_str(word: &str) -> String {
    let mut sorted: Vec<char> = word.to_lowercase().chars().collect();
    sorted.sort_by(|a, b| b.cmp(a));
    String::from_iter(sorted)
}
