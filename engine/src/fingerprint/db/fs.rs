//! Defines an abstraction and various implementations for a simple filesystem that can store and retrieve files in various (possibly nested) directories.
//! 
//! An abstraction is provided so that source files (which must be stored to perform matching) can be stored in a variety of ways, such as locally or on AWS S3.
trait Filesystem {

}

// Filesystem types
mod local;
mod aws;

pub use local::LocalFilesystem;

// TODO: implement