// use std::path::Path;
use crate::winnowing::{winnow, Fingerprint};
use std::fs;
use unidiff::PatchSet;

pub fn parse_patch(path: &str) -> Vec<Fingerprint> {
    let patch = fs::read_to_string(path).unwrap();
    let mut patchset = PatchSet::new();
    patchset.parse(&patch).ok().expect("Error parsing diff");
    winnow(patchset, &patch)
}
