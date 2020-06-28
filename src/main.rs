use std::collections::HashMap;
use std::env;
use std::io;
use std::path::Path;
use std::process::Command;

use winnow::detector;
use winnow::winnowing::{parse_patch, Fingerprint};

struct Repo {
    name: String,
    path: String,
    patches: Vec<String>,
}

impl Repo {
    fn new(repo: &str) -> io::Result<Self> {
        // uhhh these paths are ew.. fixme
        let repo_dir = Path::new(repo).file_name().unwrap();
        let repo = Repo {
            name: repo_dir.to_str().unwrap().to_owned(),
            path: repo.to_owned(),
            patches: vec![],
        };
        repo.git_clone()?;
        Ok(repo)
    }

    fn git_clone(&self) -> io::Result<()> {
        let clone_cmd = Command::new("git")
            .arg("clone")
            .arg("--bare")
            .arg(&self.path)
            .output()?;
        if clone_cmd.status.code().unwrap() == 128 {
            println!("repo {} already exists", self.path);
        } else if !clone_cmd.status.success() {
            panic!("cannot clone repo {}", self.path);
        }
        Ok(())
    }

    /*
    fn patches(&mut self) -> io::Result<()> {
        let start = empty_tree_hash()?;
        self.patches_since(&start)
    }
    */

    // generate all patches since start_hash commit
    fn make_patches_since(&mut self, start_hash: &str) -> io::Result<()> {
        let git_cmd = Command::new("git")
            .arg("format-patch")
            .arg("-k")
            .arg(start_hash)
            .current_dir(&self.name)
            .output()?;
        if !git_cmd.status.success() {
            println!("{}", String::from_utf8(git_cmd.stderr).unwrap());
            panic!("cannot git format-patch");
        }
        for l in String::from_utf8(git_cmd.stdout).unwrap().lines() {
            let mut s = self.name.clone();
            s.push_str("/");
            s.push_str(l);
            self.patches.push(s);
        }
        Ok(())
    }

    fn parse_patches(&self) -> Vec<Fingerprint> {
        // self.patches.into_iter().map(parse_patch).collect()
        let mut out = vec![];
        for p in self.patches.to_owned() {
            out.append(&mut parse_patch(&p));
        }
        out
    }
}

fn empty_tree_hash() -> io::Result<String> {
    let hash = Command::new("git")
        .arg("hash-object")
        .arg("-t")
        .arg("tree")
        .arg("/dev/null")
        .output()?;

    if !hash.status.success() {
        panic!("cannot git hash-object");
    }
    Ok(String::from_utf8(hash.stdout).unwrap().trim().to_owned())
}

/// main will handle external Git-related stuff and make calls to winnow/detect
fn main() -> io::Result<()> {
    // start at either empty tree (beginning) or specified hash
    let args: Vec<String> = env::args().collect();
    let (repo1, repo2, start_hash) = match args.len() {
        3 => (args[1].clone(), args[2].clone(), empty_tree_hash()?),
        4 => (args[1].clone(), args[2].clone(), args[3].clone()),
        _ => panic!("Invalid number of arguments"),
    };

    let mut fingerprint_map: HashMap<String, Vec<Fingerprint>> = HashMap::new();

    // loop through repos to generate tons of fingerprints
    let mut repo = Repo::new(&repo1)?;
    repo.make_patches_since(&start_hash)?; // dumps to disk
    let fingerprints = repo.parse_patches();
    fingerprint_map.insert(repo.name, fingerprints);

    let mut repo = Repo::new(&repo2)?;
    repo.make_patches_since(&start_hash)?; // dumps to disk
    let fingerprints = repo.parse_patches();
    fingerprint_map.insert(repo.name, fingerprints);

    detector::run(fingerprint_map);

    Ok(())
}
