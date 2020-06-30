use crate::winnowing::{Fingerprint, Location};
use colored::*;
use std::collections::HashMap;
use std::fs;
use unidiff::PatchSet;

#[derive(Debug, PartialEq, Eq, Hash)]
struct DetectorPair<'a> {
    a: &'a Document,
    b: &'a Document,
    fingerprints: Vec<Fingerprint>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Document {
    repo: String,
    patch: String,
    commit: String,
    file: String,
    hunk_index: usize,
}

pub fn run(repo_map: HashMap<String, Vec<Fingerprint>>) {
    // step 0: construct document map
    let mut doc_map: HashMap<Document, Vec<Fingerprint>> = HashMap::new();
    for (repo, fingerprints) in repo_map {
        for f in fingerprints {
            doc_map
                .entry(Document {
                    repo: repo.clone(),
                    patch: f.location.patch.clone(),
                    commit: f.location.commit.clone(),
                    file: f.location.file.clone(),
                    hunk_index: f.location.hunk,
                })
                .or_insert_with(Vec::new)
                .push(f);
        }
    }
    println!("Done generating doc_map");

    // step 1: construct inverted index
    let mut inverted_index: HashMap<u64, Vec<Fingerprint>> = HashMap::new();
    for fingerprints in doc_map.values() {
        for f in fingerprints {
            let fingerprint_vec = inverted_index.entry(f.hash).or_insert_with(Vec::new);
            fingerprint_vec.push(f.clone());
        }
    }
    println!(
        "Done generating inverted_index; keys: {}",
        inverted_index.keys().len()
    );

    // step 2: construct map from my_locations -> matched_locations
    let mut location_map: HashMap<Location, Vec<Location>> = HashMap::new();
    for (doc, fingerprints) in &doc_map {
        for f in fingerprints {
            let matched_fingerprints = inverted_index.get(&f.hash).unwrap();
            let popularity = matched_fingerprints
                .iter()
                .filter(|&match_fp| match_fp.location.repo != *doc.repo)
                .count();
            // decide if this is an interesting case or not
            if popularity > 0 && popularity < 1000 {
                let v = location_map
                    .entry(f.location.clone())
                    .or_insert_with(Vec::new);
                v.push(f.location.clone());
            }
        }
    }
    println!(
        "Done generating location_map; keys: {}",
        location_map.keys().len()
    );

    // step 3: O(N^2) comparison
    let mut documents = doc_map.keys();
    // pairs are (doc_a, doc_b, [match1, match2, ...])
    let mut detected_pairs = vec![];
    // fixme get rid of clone
    for (i, doc1) in documents.clone().enumerate() {
        for doc2 in documents.nth(i + 1).iter() {
            // consider pair (doc1, doc2)
            dprintln!("({:x?}\n{:x?})\n", doc1, doc2);
            // matched fingerprints
            let mut fingerprints: Vec<Fingerprint> = vec![];
            // get this doc's fingerprints and look them up in the index
            for f in doc_map.get(doc1).unwrap() {
                let mut match_fingerprints = inverted_index
                    .get(&f.hash)
                    .unwrap()
                    .iter()
                    .cloned()
                    .filter(|fp| from_same_doc(doc2, fp))
                    .filter(|fp| doc1.repo != fp.location.repo)
                    .collect();
                fingerprints.append(&mut match_fingerprints);
            }
            detected_pairs.push(DetectorPair {
                a: doc1,
                b: doc2,
                fingerprints,
            });
        }
    }
    // sort document pairs by number of matched fingerprints
    detected_pairs.sort_by(|a, b| b.fingerprints.len().cmp(&a.fingerprints.len()));
    println!("{}", "\nDETECTED PAIRS: \n\n".cyan().bold());
    for p in detected_pairs.iter().take(3) {
        show_pair(p).unwrap();
    }
}

fn show_pair(pair: &'_ DetectorPair<'_>) -> std::io::Result<()> {
    let p1 = fs::read_to_string(&pair.a.patch)?;
    let p2 = fs::read_to_string(&pair.b.patch)?;
    let mut d1 = PatchSet::new();
    d1.parse(&p1).unwrap();
    let mut d2 = PatchSet::new();
    d2.parse(&p2).unwrap();

    // println!(
    //     "{}",
    //     d1.into_iter()
    //         .find(|d| d.target_file == pair.a.file)
    //         .expect("couldn't find file")
    //         .to_string()
    //         .green()
    // );
    // println!("{}", pair.a.hunk_index);
    // println!(
    //     "{}",
    //     d2.into_iter()
    //         .find(|d| d.target_file == pair.b.file)
    //         .expect("couldn't find file")
    //         .to_string()
    //         .red()
    // );
    // println!("{}", pair.b.hunk_index);
    println!(
        "{}",
        d1.into_iter()
            .find(|d| d.target_file == pair.a.file)
            .expect("couldn't find file")
            .into_iter()
            .nth(pair.a.hunk_index)
            .expect("couldn't find hunk")
            .to_string()
            .green()
    );
    println!(
        "{}",
        d2.into_iter()
            .find(|d| d.target_file == pair.b.file)
            .expect("couldn't find file")
            .into_iter()
            .nth(pair.b.hunk_index)
            .expect("couldn't find hunk")
            .to_string()
            .red()
    );
    Ok(())
}

fn from_same_doc(d: &Document, f: &Fingerprint) -> bool {
    d.repo == f.location.repo && d.commit == f.location.commit && d.file == f.location.file
}
