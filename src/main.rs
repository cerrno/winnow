use std::env;
use std::io;
use std::path::Path;
use std::process::Command;

#[derive(PartialEq, Default, Clone, Debug)]
struct Commit {
    hash: String,
    message: String,
    diff: String,
}

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
            println!("{:?}", clone_cmd);
            println!("repo {} already exists", self.path);
        } else if !clone_cmd.status.success() {
            println!("{}", clone_cmd.status);
            panic!("cannot clone repo {}", self.path);
        }
        Ok(())
    }

    // generate all patches since start_hash commit
    fn patches_since(&mut self, start_hash: &str) -> io::Result<()> {
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
            self.patches.push(String::from(l));
        }
        Ok(())
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

fn main() -> io::Result<()> {
    // start at either empty tree (beginning) or specified hash
    let args: Vec<String> = env::args().collect();
    let start_hash = match args.len() {
        1 => empty_tree_hash()?,
        2 => args[1].clone(),
        _ => panic!("Invalid number of arguments"),
    };

    let mut repo = Repo::new("git@github.com:schuermannator/sph.git")?;
    repo.patches_since(&start_hash)?;
    println!("{:?}", repo.patches);
    Ok(())
}
