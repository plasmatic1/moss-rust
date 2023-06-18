//! Defines an abstraction and various implementations for a simple filesystem that can store and retrieve files in various (possibly nested) directories.
//! 
//! An abstraction is provided so that source files (which must be stored to perform matching) can be stored in a variety of ways, such as locally or on AWS S3.

use std::path::PathBuf;
use std::{path::Path, io};

/// Filesystem abstraction.  Trait that defines a handle to a filesystem with basic functionality.
/// 
/// File operations are not mutable as the fs handle itself stays constant throughout.
pub trait Filesystem {
    /// Writes the specified contents to a path.  If the parent directories do not exist, they will be created.
    fn write_file(&self, path: &Path, contents: &str) -> Result<(), io::Error>;

    /// Removes a file.  Will error if the path does not point to an existing file.
    fn remove_file(&self, path: &Path) -> Result<(), io::Error>;

    /// Reads the contents of a file.  Will error if the path does not point to an existing file.
    fn read_file(&self, path: &Path) -> Result<String, io::Error>;

    /// Lists the contents of a directory.  If recursive is true, will list all files in the directory and all subdirectories.
    /// Otherwise, it will just list the files and directories in the current directory.
    fn list_dir(&self, path: &Path, recursive: bool) -> Result<Vec<PathBuf>, io::Error>;

    /// Checks if the path exists.  It could be a directory or a file
    fn exists(&self, path: &Path) -> bool;

    /// Checks if the path exists and is a file
    fn is_file(&self, path: &Path) -> bool;

    /// Checks if the path exists and is a directory
    fn is_dir(&self, path: &Path) -> bool;

    /// Performs any necessary cleaning functionality, such as removing unnecessary files and directories.
    /// The exact behavior of this function is implementation-dependent.
    fn clean(&self) -> Result<(), io::Error>;
    
    /// Cleanup facility: removes the filesystem pointed at by the handle
    fn clear(&self) -> Result<(), io::Error>;
}

// Filesystem types
mod local;
mod aws;

pub use local::LocalFilesystem;

// TODO: implement