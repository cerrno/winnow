use std::path::Path;
use std::fs;
use unidiff::PatchSet;

fn parse_patch_str(patch_str: String) {
    let mut patch = PatchSet::new();
    patch.parse(patch_str).ok().expect("Error parsing diff");
}

pub fn parse_patch(path: Path) {
    let patch = fs::read_to_string(path)?;
    parse_patch_str(patch);
}
