use crate::winnowing::Fingerprint;
use std::collections::HashMap;

// struct Repo(String);
#[derive(Debug)]
struct RepoLocation {
    commit: [u8; 20],
    file: String,
    line: usize,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Location {
    repo: String,
    commit: [u8; 20],
    file: String,
}

pub fn run(repo_map: HashMap<String, Vec<Fingerprint>>) {
    // step 1: construct inverted index
    let mut inverted_index: HashMap<u64, HashMap<String, Vec<RepoLocation>>> = HashMap::new();
    for (repo, fingerprints) in &repo_map {
        for f in fingerprints {
            let fingerprint_hash_map = inverted_index.entry(f.hash).or_insert(HashMap::new());
            let location_vec = fingerprint_hash_map
                .entry(repo.clone())
                .or_insert(Vec::new());
            location_vec.push(RepoLocation {
                commit: f.commit,
                file: f.file.clone(),
                line: f.line,
            });
        }
    }
    for i in inverted_index.iter().take(10) {
        println!("{:?}", i);
    }
    println!("{}", inverted_index.keys().len());

    // step 2: construct map from locations -> fingerprints
    let mut location_map: HashMap<Location, Vec<Fingerprint>> = HashMap::new();
    for (repo, fingerprints) in repo_map {
        for f in fingerprints {
            let repo_map = inverted_index.get(&f.hash);
            let popularity = repo_map.unwrap().len() - 1;
            // decide if this is an interesting case or not
            if popularity > 0 && popularity < 1000 {
                let my_location = Location {
                    repo: repo.clone(),
                    commit: f.commit,
                    file: f.file.clone(),
                };
                let v = location_map.entry(my_location).or_insert(vec![]);
                v.push(f);
            }
        }
    }

    // step 3: O(N^2) comparison
    for (_, v) in location_map {
        println!("{:?}", v.len());
    }
}
