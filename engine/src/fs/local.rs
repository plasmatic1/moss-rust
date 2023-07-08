use std::path::{Path, PathBuf};
use std::io;
use super::FSHandle;

pub struct LocalFSHandle {
    root: PathBuf,
}

impl LocalFSHandle {
    pub fn new(root: &Path) -> Self {
        // Create folder if it does not exist and test
        if !root.exists() {
            std::fs::create_dir(root).expect(format!("Failed to create root directory of LocalFileSystem at {}", root.display()).as_str());
        }
        else if !root.is_dir() {
            panic!("Root path is not a directory");
        }

        Self {
            root: root.to_path_buf(),
        }
    }

    /// Helper for list_dir.  list_dir needs some path translation utilities to work properly.
    fn list_dir_helper(&self, path: &Path, recursive: bool) -> Result<Vec<PathBuf>, io::Error> {
        let mut ret = vec![];

        for p_info in std::fs::read_dir(path)? {
            let p = p_info?.path();
            if p.is_dir() && recursive {
                ret.append(&mut self.list_dir_helper(&p, recursive)?);
            }
            else {
                ret.push(p);
            }
        }

        Ok(ret)
    }

    /// Helper for clean
    fn clean_helper(&self, path: &Path) -> Result<(), io::Error> {
        for p_info in std::fs::read_dir(self.root.join(path))? {
            let p = p_info?.path();
            if p.is_dir() {
                // Hit a empty, non-root directory
                if std::fs::read_dir(p.as_path()).unwrap().next().is_none() && p != self.root {
                    std::fs::remove_dir(p)?; 
                }
                else {
                    self.clean_helper(p.as_path())?;
                }
            }
        }

        Ok(())
    }
}

impl FSHandle for LocalFSHandle {
    fn write_file(&self, path: &Path, contents: &str) -> Result<(), io::Error> {
        std::fs::create_dir_all(self.root.join(path).parent().unwrap())?; // `path` should point to a file, so this should never error with correct input
        std::fs::write(self.root.join(path), contents)
    }

    fn remove_file(&self, path: &Path) -> Result<(), std::io::Error> {
        std::fs::remove_file(self.root.join(path))
    }

    fn exists(&self, path: &Path) -> bool {
        self.root.join(path).exists()
    }

    fn is_file(&self, path: &Path) -> bool {
        self.root.join(path).is_file()
    }

    fn is_dir(&self, path: &Path) -> bool {
        self.root.join(path).is_dir()
    }

    fn read_file(&self, path: &Path) -> Result<String, io::Error> {
        std::fs::read_to_string(self.root.join(path))
    }

    fn list_dir(&self, p: &Path, recursive: bool) -> Result<Vec<PathBuf>, io::Error> {
        let ret = self.list_dir_helper(self.root.join(p).as_path(), recursive)?;
        Ok(ret.into_iter().map(|path| path.strip_prefix(self.root.as_path()).unwrap().to_path_buf()).collect())
    }

    fn clear(&self) -> Result<(), io::Error> {
        std::fs::remove_dir_all(self.root.as_path())
    }

    fn clean(&self) -> Result<(), io::Error> {
        self.clean_helper(self.root.as_path())
    }

    fn get_identifier(&self) -> String {
        format!("LocalFS: {:?}", self.root)
    }
}
