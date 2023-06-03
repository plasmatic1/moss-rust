use std::collections::HashMap;

use super::super::{Step, Seq};
use crate::hash::HashState;

/// A step that replaces substrings with characters, using a rolling hash algorithm to minimize runtime.
/// The mappings are specified as a vector of tuples (s1 => c1), ..., (sk => ck).  For optimization reasons,
/// the source strings s1, ..., sk must all be the same length.
pub struct SameSizeReplace {
    mappings: HashMap<u64, char>,
    len: usize,
}

impl SameSizeReplace {
    pub fn new(mappings: Vec<(&str, char)>) -> Self {
        debug_assert!(!mappings.is_empty());
        debug_assert!(mappings.iter().map(|(s, _)| s.len()).all(|l| l == mappings[0].0.len()), "All mappings must be the same length");

        Self {
            mappings: mappings.iter().map(|(s, c)| (HashState::from_str(s).get_u64(), *c)).collect(),
            len: mappings[0].0.len(),
        }
    }
}

impl Step for SameSizeReplace {
    fn apply(&self, input: &Seq) -> Seq {
        if input.len() < self.len { // edge case
            input.clone()
        }
        else {
            let mut hash = HashState::new();
            let mut ret: Seq = vec![];

            for i in 0..input.len() {
                // update rolling hash
                if hash.len() < self.len {
                    hash.push_char(input[i].1);
                }
                else {
                    hash.pop_char(input[i - self.len].1);
                    hash.push_char(input[i].1);
                }

                // Check if hash matches
                if let Some(to) = self.mappings.get(&hash.get_u64()) {
                    ret.push((i - self.len + 1, *to));
                    hash.clear();
                }
                else {
                    ret.push(input[i]);
                }
            }

            ret
        }
    }
}