// In the preprocessor, strings are treated as pairs of (index, char) where entry s[i] denotes that the substring from indices s[i].0 to s[i+1].0-1 is compressed as s[i].1
type Seq = Vec<(usize, char)>;
type Step = dyn Fn(&Seq) -> Seq;
type Steps = Vec<Box<Step>>;

pub fn apply(input: &str, steps: &Steps) -> Seq {
    let init_seq: Seq = input.chars().enumerate().collect();
    steps.iter().fold(init_seq, |cur_seq, step| { step(&cur_seq) })
}

/**
 * Alternative idea:
 * - Apply is replaced with a macro that can assist in chaining steps together and whatnot
 * - We have a "lib" .rs file that just contains these functions for use in each language
 * - Language we can map from file extension to language? idk.  Or we can just have a enum for supported languages, and support a simple conversion function
 */

// collect different steps here
mod whitespace;
mod identifiers;

pub mod steps {
    pub use super::whitespace::whitespace;
}