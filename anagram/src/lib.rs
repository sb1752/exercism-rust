use std::collections::HashSet;

pub fn anagrams_for<'a>(target: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let mut anagrams = HashSet::new();

    let unsorted_target: Vec<char> = target.to_lowercase().chars().collect();
    let mut sorted_target = unsorted_target.clone();
    sorted_target.sort_unstable();

    for word in possible_anagrams {
        let mut word_chars: Vec<char> = word.to_lowercase().chars().collect();

        if word_chars.len() == unsorted_target.len() && word_chars != unsorted_target {
            word_chars.sort_unstable();
            if word_chars == sorted_target {
                anagrams.insert(*word);
            }
        }
    }

    anagrams
}
