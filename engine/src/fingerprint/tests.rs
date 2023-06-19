use super::{*, fs::Filesystem};
use std::path::Path;

const TEST_FS_ROOT: &str = "test_fs_local";

/// Macro for clearing test fs if it exists
#[cfg(test)]
fn clean_fs_local() {
    let fs_root = Path::new("test_fs_root");
    if fs_root.exists() && fs_root.is_dir() { 
        std::fs::remove_dir_all(fs_root).unwrap();
    }
}

#[test]
fn test_fs_local_success() {
    // Create handle
    clean_fs_local();
    let handle = fs::LocalFilesystem::new(Path::new(TEST_FS_ROOT));

    // Create some files / test write
    handle.write_file(Path::new("test.txt"), "test").unwrap();
    handle.write_file(Path::new("test2.txt"), "test2").unwrap();
    handle.write_file(Path::new("test/test9.txt"), "test9abcabcabcabcabc").unwrap();
    handle.write_file(Path::new("test/test2/abc.bat"), "this is a windows bash script").unwrap();

    // Test read, remove (and some is_file)
    assert!(handle.is_file(Path::new("test.txt")));
    assert_eq!(handle.read_file(Path::new("test.txt")).unwrap(), "test");
    handle.remove_file(Path::new("test.txt")).unwrap();
    assert!(!handle.is_file(Path::new("test.txt")));

    // Test is_file, is_dir, exists
    assert!(!handle.is_file(Path::new("this/dir/does/not/exi.st")));
    assert!(!handle.exists(Path::new("this/test89.txt")));

    assert!(handle.exists(Path::new("test")));
    assert!(handle.is_dir(Path::new("test")));
    assert!(!handle.is_file(Path::new("test")));

    assert!(handle.exists(Path::new("test/test2/abc.bat")));
    assert!(!handle.is_dir(Path::new("test/test2/abc.bat")));
    assert!(handle.is_file(Path::new("test/test2/abc.bat")));

    // Test listdir
    assert_eq!(handle.list_dir(Path::new(""), false).unwrap().sort(),
        vec![Path::new("test"), Path::new("test2.txt")].sort());
    assert_eq!(handle.list_dir(Path::new("test"), false).unwrap().sort(),
        vec![Path::new("test/test2"), Path::new("test/test9.txt")].sort());
    assert_eq!(handle.list_dir(Path::new("test"), true).unwrap().sort(),
        vec![Path::new("test/test2/abc.bat"), Path::new("test2.txt"), Path::new("test/test9.txt")].sort());
    
    // Test overwrite
    assert_eq!(handle.read_file(Path::new("test/test2/abc.bat")).unwrap(), "this is a windows bash script");
    handle.write_file(Path::new("test/test2/abc.bat"), "this is really a windows bash script").unwrap();
    assert_eq!(handle.read_file(Path::new("test/test2/abc.bat")).unwrap(), "this is really a windows bash script");

    // Clear
    handle.clear().unwrap();
}

#[test]
fn test_fs_local_clean() {
    // Create handle
    clean_fs_local();
    let handle = fs::LocalFilesystem::new(Path::new(TEST_FS_ROOT));

    // Write a bunch of files
    handle.write_file(Path::new("test1/test.txt"), "test").unwrap();
    handle.write_file(Path::new("test2/test.txt"), "test").unwrap();
    handle.write_file(Path::new("test2/nested/test.txt"), "test").unwrap();
    handle.write_file(Path::new("test3/test.txt"), "test").unwrap();
    handle.write_file(Path::new("test3/nested/test.txt"), "test").unwrap();
    handle.write_file(Path::new("test4/test.txt"), "test").unwrap();
    handle.write_file(Path::new("test4/nested/test.txt"), "test").unwrap();

    // Remove files to get empty dirs
    handle.remove_file(Path::new("test1/test.txt")).unwrap(); // remove file
    handle.remove_file(Path::new("test2/test.txt")).unwrap(); // remove all nested
    handle.remove_file(Path::new("test2/nested/test.txt")).unwrap();
    handle.remove_file(Path::new("test3/nested/test.txt")).unwrap(); // remove only nested
    handle.remove_file(Path::new("test4/test.txt")).unwrap(); // remove only unnested

    // Check file structure
    assert_eq!(handle.list_dir(Path::new(""), false).unwrap().sort(),
        vec!["test3", "test4"].sort());
    assert_eq!(handle.list_dir(Path::new("test3"), false).unwrap().sort(),
        vec!["test.txt"].sort());
    assert_eq!(handle.list_dir(Path::new("test4"), false).unwrap().sort(),
        vec!["nested"].sort());
}