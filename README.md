# moss-rust

**Note: currently only `\n` is supported because I'm lazy.  Implement CRLF soon... TODO.**

Parallel implementation of the MOSS program for plagiarism detection.  Goal is to support:

* Quickly finding matching pairs of documents
* Support matching individual documents against a previously database of documents

## Compilation Options

* `cargo test` for tests
    * Run in `engine/` for engine tests

## Installation



## TODO

* Simple solution DB
    * Maintaining solutions
    * Q: How do we want to query? Maybe a few commands is the best way
        * ADD f
        * ADD_TEMPLATE f
        * CHECK f
        * CHECK_ALL
        * REMOVE f
        * LIST
        * SEARCH f
        * CLEAR
* Core
    * Thread interop
    * Etc.
    * Maybe core implements a spinning thread that listens for events lol
        * Multiprocess into many components

### Technical Debt

* Preprocessor
    * **Rearrange preprocessor organization to be more idiomatic Rust**
        * Probably incorporate traits into this, and just nesting the various operations... lol

### Documentation

* Preprocessor