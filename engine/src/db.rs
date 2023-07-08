//! Defines a database system for storing and matching fingerprint hashes.
//! 
//! The database system can efficiently query fingerprint hashes, along with quickly compute 
//! 
//! TODO: finish

extern crate sqlite;
extern crate sha2;
extern crate hex;

use std::{path::{Path, PathBuf}, sync::Mutex};

use super::fs;
use sha2::{Sha256, Digest};

/// Enum containing the different types of messages that can be passed to workers
enum IPCPacket {
    // TODO: impl
}

pub struct Options {
    k: usize,
    t: usize,
    n_workers: i32
}

const DB_DIR_PATH: &'static str = ".db";

/// A database handle for storing and matching hashes.
/// 
/// It connects to a SQLite database file based on the filesystem handle provided.
/// The filesystem handle is used to generate a unique identifier for the database file,
/// which is placed relative to DB_DIR_PATH
pub struct FingerprintDB {
    fs: Box<dyn fs::FS>,
    conn: sqlite::Connection,

    /// Options for the database
    opts: Options,
    /// Boolean value that contains values
    force_quit: Mutex<bool>, 
}

fn get_db_path(fs: &dyn fs::FS) -> PathBuf {
    let mut path_hasher = Sha256::new();
    path_hasher.update(fs.get_identifier());
    let result_str = hex::encode(path_hasher.finalize());
    PathBuf::from(DB_DIR_PATH).join(result_str)
}

impl FingerprintDB {
    /// Construct a new db handle based on a filesystem handle.
    /// On creation, the databse file will be initialized if it does not exist.
    /// 
    /// There are 2 database tables:
    /// - `fingerprints`: stores the fingerprint hashes and their locations
    /// - `file_hashes`: stores the sha256 hashes of files to track when they need updating
    pub fn new(fs: Box<dyn fs::FS>, opts: Options) -> FingerprintDB {
        let db_path = get_db_path(fs.as_ref());
        std::fs::create_dir_all(db_path.parent().unwrap()).unwrap(); // db_path will be a file under some directory, so parent() is safe
        let conn = sqlite::open(get_db_path(fs.as_ref())).unwrap();
        
        conn.execute("CREATE TABLE IF NOT EXISTS fingerprints (
            id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            hash BIGINT UNSIGNED NOT NULL,
            loc INTEGER NOT NULL,
            path VARCHAR(64),
            lang VARCHAR(16) NOT NULL
        );").unwrap();

        conn.execute("CREATE TABLE IF NOT EXISTS file_hashes (
            id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            path VARCHAR(64) NOT NULL,
            sha256 CHAR(64) NOT NULL
        );").unwrap();

        FingerprintDB {
            fs, conn, opts,
            force_quit: Mutex::new(false),
        }
    }

    pub fn refresh(&self) -> Result<(), Box<dyn std::error::Error>> {
        let found_files = self.fs.list_dir(PathBuf::new().as_path(), true)?;
        let db_files = {
            // TODO: move out of here
            let mut stmt = self.conn.prepare("SELECT * FROM file_hashes WHERE
                path IN :paths
            ")?;
            stmt.bind((":paths", format!("({})", found_files.iter().map(|p| p.to_str().unwrap()).collect::<Vec<String>>().join(",").as_str())))?;
        };
        self.conn.execute("SELECT * FROM file_hashes WHERE")?;

        Ok(())
    }

    pub fn find_best_files(&self, hash: u64) -> Vec<PathBuf> {
        todo!();
    }
}

impl Drop for FingerprintDB {
    fn drop(&mut self) {
        todo!();
    }
}
