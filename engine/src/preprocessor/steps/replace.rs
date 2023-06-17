use std::collections::HashMap;

use super::super::{Step, Seq};

/// A step that replaces substrings with characters, using a rolling hash algorithm to minimize runtime.
/// The mappings are specified as a vector of tuples (s1 => c1), ..., (sk => ck).  For optimization reasons,
/// the source strings s1, ..., sk must all be the same length.
pub struct SameSizeReplace<'a> {
    mappings: HashMap<&'a str, char>,
    len: usize,
}

impl<'a> SameSizeReplace<'a> {
    pub fn new(mappings: Vec<(&'a str, char)>) -> Self {
        debug_assert!(!mappings.is_empty());
        debug_assert!(mappings.iter().map(|(s, _)| s.len()).all(|l| l == mappings[0].0.len()), "All mappings must be the same length");

        Self {
            len: mappings[0].0.len(),
            mappings: mappings.into_iter().collect(),
        }
    }
}

impl<'a> Step for SameSizeReplace<'a> {
    fn apply(&self, input: &Seq) -> Seq {
        if input.len() < self.len { // edge case
            input.clone()
        }
        else {
            let mut ret: Seq = vec![];
            let mut i = 0;

            while i < input.len() {
                if i + self.len <= input.len() {
                    let matched_str = &input[i..i+self.len].iter().map(|c| c.1).collect::<String>();
                    if let Some(c) = self.mappings.get(matched_str.as_str()) {
                        ret.push((i, *c));
                        i += self.len;
                        continue;
                    }
                }

                ret.push(input[i]);
                i += 1;
            }

            ret
        }
    }
}