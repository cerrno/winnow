#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn winnow(input: &str, window: u32) -> Vec<(u64, u64)> {
    let input = clean(input);
    ngram(input.chars(), window)
        .map(|x| x.iter().collect::<String>())
        .map(make_pair)
        .collect()
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
fn ngram<I>(mut f: I, n: u32) -> impl Iterator<Item = Vec<I::Item>>
where
    I: Iterator + Clone,
{
    let mut v = vec![];
    let mut b = f.clone();
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
    v.into_iter()
}

fn make_pair<T: Hash>(input: T) -> (u64, u64) {
    (hash(&input), 0)
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
        let ngrams = ngram(input.chars(), 4);
        assert_eq!(
            ngrams
                .map(|x| x.into_iter().collect::<String>())
                .collect::<Vec<String>>(),
            vec!["abcd", "bcde", "cdef", "defg", "efgh"]
        );

        let input = "A do run run run";
        let input = clean(input);
        let ngrams = ngram(input.chars(), 5);
        assert_eq!(
            ngrams
                .map(|x| x.into_iter().collect::<String>())
                .collect::<Vec<String>>(),
            vec!["adoru", "dorun", "orunr", "runru", "unrun", "nrunr", "runru", "unrun"]
        );
    }

    #[test]
    fn test_basic() {
        let input = "A do run run run, a do run run";
        println!("{:?}", winnow(input, 5));
    }
}
