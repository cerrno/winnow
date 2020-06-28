use crate::winnowing::{Fingerprint, Location};
use std::collections::HashMap;

// struct Repo(String);

pub fn run(repo_map: HashMap<String, Vec<Fingerprint>>) {
    // step 1: construct inverted index
    let mut inverted_index: HashMap<u64, Vec<Location>> = HashMap::new();
    for (_, fingerprints) in &repo_map {
        for f in fingerprints {
            let fingerprint_vec = inverted_index.entry(f.hash).or_insert_with(Vec::new);
            fingerprint_vec.push(f.location.clone());
        }
    }
    for i in inverted_index.iter().take(10) {
        println!("{:?}", i);
    }
    println!("{}", inverted_index.keys().len());

    // step 2: construct map from my_locations -> matched_locations
    let mut location_map: HashMap<Location, Vec<Location>> = HashMap::new();
    for (repo, fingerprints) in &repo_map {
        for f in fingerprints {
            let matched_locations = inverted_index.get(&f.hash).unwrap();
            let popularity = matched_locations
                .iter()
                .filter(|&location| location.repo == *repo)
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
    for (_, v) in location_map {
        println!("{:?}", v.len());
    }
}
