#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn winnow(input: &str, window: u32) -> Vec<(u64, u64)> {
    let input = clean(input);
    ngram(&input, window).iter().map(make_pair).collect()
}

fn clean(input: &str) -> String {
    input
        .chars()
        .filter(|c| c.is_alphabetic())
        .map(|c| c.to_lowercase())
        .flatten()
        .collect()
}

// 4-gram example:
// abcdefgh
// abcd bcde cdef defg efgh
fn ngram(input: &str, n: u32) -> Vec<String> {
    let mut v = vec![];
    let mut f = input.chars();
    let mut b = input.chars();
    for _ in 0..n {
        assert!(b.next().is_some(), "Input size greater than N");
    }
    loop {
        let ngram = f.clone().take(n as usize).collect();
        v.push(ngram);
        f.next();
        if b.next().is_none() {
            break;
        }
    }
    v
}

fn make_pair<T: Hash>(input: &T) -> (u64, u64) {
    (hash(input), 0)
}

fn hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
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
    fn test_ngram() {
        let input = "abcdefgh";
        let ngrams = ngram(&input, 4);
        assert_eq!(ngrams, vec!["abcd", "bcde", "cdef", "defg", "efgh"]);

        let input = "A do run run run";
        let input = clean(input);
        let ngrams = ngram(&input, 5);
        assert_eq!(ngrams, vec!["adoru", "dorun", "orunr", "runru", "unrun",
            "nrunr", "runru", "unrun"]);
    }

    #[test]
    fn test_basic() {
        let input = "A do run run run, a do run run";
        println!("{:?}", winnow(input, 5));
        assert!(false);
    }
}
