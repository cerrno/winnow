#![warn(
    unreachable_pub,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    rust_2018_idioms,
    missing_debug_implementations
)]

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn winnow(input: &str, window: u32) -> Vec<(u64, u64)> {
    let input = clean(input);
    let ngram_hash_iter = ngram(input.chars(), window)
        .map(|x| x.iter().collect::<String>())
        .map(|x| hash(&x));
    // lul fix this
    let hashes = ngram_hash_iter.collect::<Vec<u64>>();
    //for i in ngram(hashes.iter(), window).collect::<Vec<Vec<_>>>() {
    //    println!("{:?}", i);
    //}
    ngram(hashes.into_iter(), window)
        .enumerate()
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

/// Construct an n-gram from an iterator
///
/// 4-gram example:
/// abcdefgh => abcd bcde cdef defg efgh
///
/// ```
/// # use winnow::ngram;
///
/// let input = "abcdefgh";
/// let ngrams = ngram(input.chars(), 4);
/// assert_eq!(
///     ngrams
///         .map(|x| x.into_iter().collect::<String>())
///         .collect::<Vec<String>>(),
///     vec!["abcd", "bcde", "cdef", "defg", "efgh"]
/// );
/// ```
pub fn ngram<I>(mut f: I, n: u32) -> impl Iterator<Item = Vec<I::Item>>
where
    I: Iterator + Clone,
{
    let mut v = vec![];
    let mut b = f.clone();
    for _ in 0..n {
        assert!(b.next().is_some(), "Input size smaller than N");
    }
    loop {
        let ngram = f.clone().take(n as usize).collect();
        v.push(ngram);
        f.next();
        if b.next().is_none() {
            break;
        }
    }
    // fix to yield vals instead of making vec and returning iterator
    v.into_iter()
}

// select smallest hash out of vec
// select rightmost in case of tie
fn make_pair(input: (usize, Vec<u64>)) -> (u64, u64) {
    let mut min = input.1.get(0).unwrap();
    let mut idx = 0;
    for (i, v) in input.1.iter().enumerate() {
        if v < min {
            min = v;
            idx = i;
        }
    }
    (*min, (input.0 + idx) as u64)
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
        let input = clean("A do run run run");
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
        // println!("{:?}", winnow(input, 5));
        let output = winnow(input, 5);
        let expexted: Vec<(u64, u64)> = vec![
            (4020085029674966483, 3),
            (1468765096528618582, 5),
            (1468765096528618582, 5),
            (1468765096528618582, 5),
            (1468765096528618582, 5),
            (1468765096528618582, 5),
            (2165872647979677269, 8),
            (2165872647979677269, 8),
            (2165872647979677269, 8),
            (2880295526655702587, 9),
            (7536710649711940037, 12),
            (4020085029674966483, 15),
            (4020085029674966483, 15),
        ];
        assert_eq!(output, expexted);
    }
}
