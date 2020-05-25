use blake2::{Blake2b, Digest};
use log::{info, trace, warn};
use std::path::Path;
use std::{fs, io};

pub(crate) fn hash_digest(filepath: &Path) -> String {
    trace!("Hashing file {:?}", filepath);

    let mut file = fs::File::open(filepath).unwrap();
    let mut hasher = Blake2b::new();
    let n = io::copy(&mut file, &mut hasher).unwrap();
    trace!("Bytes processed: {}", n);

    format!("{:x}", hasher.result())
}