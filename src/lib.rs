#![allow(dead_code)]
#![allow(unused_variables)]

pub(crate) fn clean(input: &str) -> String {
    input
        .chars()
        .filter(|c| c.is_alphabetic())
        .map(|c| c.to_lowercase())
        .flatten()
        .collect()
}

pub(crate) fn ngram(input: &str, n: u32) -> Vec<String> {
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean() {
        let input = "A do run run run, a do run run";
        let input = clean(input);
        assert!(input == "adorunrunrunadorunrun");
    }

    #[test]
    fn test_basic() {
        let input = "A do run run run, a do run run";
        let input = clean(input);
        let ngrams = ngram(&input, 5);
    }
}
