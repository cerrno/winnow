use crate::winnowing::{Fingerprint, Location};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
struct DetectorPair<'a> {
    a: &'a Document,
    b: &'a Document,
    fingerprints: Vec<Fingerprint>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Document {
    repo: String,
    commit: [u8; 20],
    file: String,
}

pub fn run(repo_map: HashMap<String, Vec<Fingerprint>>) {
    // step 0: construct document map
    let mut doc_map: HashMap<Document, Vec<Fingerprint>> = HashMap::new();
    for (repo, fingerprints) in repo_map {
        for f in fingerprints {
            doc_map
                .entry(Document {
                    repo: repo.clone(),
                    commit: f.location.commit.clone(),
                    file: f.location.file.clone(),
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
        "Done generating inverted_index; len: {}",
        inverted_index.keys().len()
    );

    // step 2: construct map from my_locations -> matched_locations
    let mut location_map: HashMap<Location, Vec<Location>> = HashMap::new();
    for (doc, fingerprints) in &doc_map {
        for f in fingerprints {
            let matched_fingerprints = inverted_index.get(&f.hash).unwrap();
            let popularity = matched_fingerprints
                .iter()
                .filter(|&match_fp| {
                    match_fp.location.repo == *doc.repo && match_fp.location.file == *doc.file
                })
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

    // step 3: O(N^2) comparison
    let mut documents = doc_map.keys();
    // pairs are (doc_a, doc_b, [match1, match2, ...])
    let mut detected_pairs = vec![];
    // fixme get rid of clone
    for (i, doc1) in documents.clone().enumerate() {
        for doc2 in documents.nth(i + 1).iter() {
            // consider pair (doc1, doc2)
            println!("({:x?}, {:x?})\n", doc1, doc2);
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
    println!("\nDETECTED PAIRS: \n\n");
    for p in detected_pairs.iter().take(10) {
        println!("{:x?}\n", p);
    }
}

fn from_same_doc(d: &Document, f: &Fingerprint) -> bool {
    d.repo == f.location.repo && d.commit == f.location.commit && d.file == f.location.file
}
