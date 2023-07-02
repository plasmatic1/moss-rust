//! Defines a database system for storing and 

// TODO: use https://github.com/rust-lang/odht
// TODO: implement

extern crate galvanize;

use super::fs;

const INDEX_PATH: &'static str = "index.fdb";
const BOILERPLATE_PATH: &'static str = "boilerplate.fdb";
const CHECKED_PATH: &'static str = "file_checked_index.fdb";

pub struct FingerprintDatabase {
    src_fs: Box<dyn fs::Filesystem>,
}

impl FingerprintDatabase {
    pub fn new(src_fs: Box<dyn fs::Filesystem>) -> FingerprintDatabase {
        FingerprintDatabase {
            src_fs
        }
    }
}