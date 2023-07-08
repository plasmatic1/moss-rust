//! Defines a database system for storing and matching fingerprint hashes.
//! 
//! The database system can efficiently query fingerprint hashes, along with quickly compute 
//! 
//! TODO: finish

extern crate sqlite;
extern crate sha2;
extern crate hex;

use std::path::{Path, PathBuf};

use super::{fs, FingerprintSeq};
use sha2::{Sha256, Digest};

const DB_DIR_PATH: &'static str = ".db";

/// A database handle for storing and matching hashes.
/// 
/// It connects to a SQLite database file based on the filesystem handle provided.
/// The filesystem handle is used to generate a unique identifier for the database file,
/// which is placed relative to DB_DIR_PATH
pub struct DBHandle {
    fs: Box<dyn fs::FSHandle>,
    db: sqlite::Connection,
}

fn get_db_path(fs: &dyn fs::FSHandle) -> PathBuf {
    let mut path_hasher = Sha256::new();
    path_hasher.update(fs.get_identifier());
    let result_str = hex::encode(path_hasher.finalize());
    PathBuf::from(DB_DIR_PATH).join(result_str)
}

impl DBHandle {
    /// Construct a new db handle based on a filesystem handle.
    /// On creation, the databse file will be initialized if it does not exist.
    pub fn new(fs: Box<dyn fs::FSHandle>) -> DBHandle {
        let db_path = get_db_path(fs.as_ref());
        std::fs::create_dir_all(db_path.parent().unwrap()).unwrap(); // db_path will be a file under some directory, so parent() is safe
        let db = sqlite::open(get_db_path(fs.as_ref())).unwrap();
        
        db.execute("CREATE TABLE IF NOT EXISTS hashes (
            id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            hash BIGINT UNSIGNED NOT NULL,
            loc INTEGER NOT NULL,
            path VARCHAR(64),
            lang VARCHAR(16) NOT NULL
        );").unwrap();

        DBHandle { fs, db }
    }

    pub fn refresh(&self) {
    }

    pub fn clear(&self) {
        self.db.execute("DELETE * FROM hashes;").unwrap();
    }
}