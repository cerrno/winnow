use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::{Hash, Hasher};
use unidiff::{Line, PatchSet};

#[derive(Debug, PartialEq)]
pub struct Fingerprint {
    hash: u64,
    commit: String, // u64,
    file: String,
    line: usize,
}

pub fn winnow(commit: PatchSet, commit_hash: &str) -> Vec<Fingerprint> {
    let mut hash_line_file = vec![];
    for patchfile in commit {
        let mut hash_and_line = vec![];
        let file = &patchfile.target_file.clone();
        for hunk in patchfile {
            for line in hunk.target_lines() {
                hash_and_line.append(&mut winnow_line(line));
            }
        }
        let mut v = add_file(hash_and_line, file);
        hash_line_file.append(&mut v);
    }
    make_fingerprints(hash_line_file, commit_hash)
}

fn winnow_line(line: Line) -> Vec<(u64, usize)> {
    let n = line.target_line_no.unwrap(); // should have target_line_no since it came from target_lines
    let hashes = winnow_str(&line.value, 5);
    hashes.into_iter().map(|h| (h, n)).collect()
}

fn winnow_str(input: &str, window: u32) -> Vec<u64> {
    let input = clean(input);
    let ngram_hash_iter = ngram(input.chars(), window)
        .map(|x| x.iter().collect::<String>())
        .map(|x| hash(&x));
    // lul fix this
    let hashes = ngram_hash_iter.collect::<Vec<u64>>();
    ngram(hashes.into_iter(), window).map(select_hash).collect()
}

pub fn parse_patch(path: &str) -> Vec<Fingerprint> {
    let patch = fs::read_to_string(path).unwrap();
    let mut patchset = PatchSet::new();
    if let Err(e) = patchset.parse(&patch) {
        println!("{:?}", e);
        return vec![];
    }
    winnow(patchset, &patch)
}

fn add_file(incompletes: Vec<(u64, usize)>, file: &str) -> Vec<(u64, usize, String)> {
    incompletes
        .into_iter()
        .map(|(hash, line)| (hash, line, file.to_owned()))
        .collect()
}

fn make_fingerprints(
    incompletes: Vec<(u64, usize, String)>,
    commit_hash: &str,
) -> Vec<Fingerprint> {
    incompletes
        .into_iter()
        .map(|(hash, line, file)| Fingerprint {
            hash,
            commit: commit_hash.to_owned(),
            line,
            file,
        })
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
/// # use winnow::winnowing::ngram;
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
        // assert!(b.next().is_some(), "Input size smaller than N");
        if b.next().is_none() {
            return v.into_iter();
        }
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
fn select_hash(hashes: Vec<u64>) -> u64 {
    let mut min = hashes.get(0).unwrap();
    for v in hashes.iter() {
        if v < min {
            min = v;
        }
    }
    *min
}

fn hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

#[cfg(test)]
mod tests {
    use super::*;
    use fs::read_to_string;

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
    fn test_scheme() {
        let winnow_size = 50;
        let us = read_to_string("tests/data/play.scm").unwrap();
        let them = read_to_string("tests/data/play1.scm").unwrap();
        let us = winnow_str(&us, winnow_size);
        let them = winnow_str(&them, winnow_size);
        println!("{:?}", us);
        println!("{:?}", them);
        println!("{:?}", us.len());
        println!("{:?}", them.len());
    }

    // #[test]
    // fn test_basic() {
    //     let input = "A do run run run, a do run run";
    //     // println!("{:?}", winnow(input, 5));
    //     let output = winnow_str(input, 5);
    //     let expected: Vec<Fingerprint> = vec![
    //         Fingerprint(4020085029674966483, 3, 0),
    //         Fingerprint(1468765096528618582, 5, 0),
    //         Fingerprint(1468765096528618582, 5, 0),
    //         Fingerprint(1468765096528618582, 5, 0),
    //         Fingerprint(1468765096528618582, 5, 0),
    //         Fingerprint(1468765096528618582, 5, 0),
    //         Fingerprint(2165872647979677269, 8, 0),
    //         Fingerprint(2165872647979677269, 8, 0),
    //         Fingerprint(2165872647979677269, 8, 0),
    //         Fingerprint(2880295526655702587, 9, 0),
    //         Fingerprint(7536710649711940037, 12, 0),
    //         Fingerprint(4020085029674966483, 15, 0),
    //         Fingerprint(4020085029674966483, 15, 0),
    //     ];
    //     assert_eq!(output, expected);
    // }
}
