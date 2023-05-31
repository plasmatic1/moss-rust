use crate::preprocessor::{Seq, Step};
use std::collections::HashSet;

// DFA for matching identifiers
#[derive(PartialEq, Eq, Clone, Copy)]
enum DFA {
    // DFA states
    First, // Initial state
    SecondPlus, // Intermediate state
    Invalid, // Match failed, definitely not identifier
    Done, // Match is complete, full word matched
}

impl DFA {
    pub fn new() -> Self {
        Self::First
    }

    /**
     * None will behave as if the character is invalid (i.e. punctuation)
     */
    pub fn advance(&self, c_opt: Option<char>) -> DFA {
        match c_opt {
            None => match *self {
                Self::First | Self::Invalid => Self::Invalid,
                _ => Self::Done
            }
            Some(c) => match self {
                Self::First if c.is_alphabetic() || c == '_' => Self::SecondPlus,
                Self::First => Self::Invalid,
                Self::SecondPlus if c.is_alphanumeric() || c == '_' => Self::SecondPlus,
                Self::SecondPlus => Self::Done,
                o => *o
            }
        }
    }

    pub fn is_done(&self) -> bool {
        return *self == Self::Done || *self == Self::Invalid
    }
}

pub struct Identifiers<'a> {
    keywords: HashSet<&'a str>,
    normalize_into: char,
}

impl Step for Identifiers<'_> {
    /**
     * A step that normalizes identifiers into a single token.
     * - An identifier is a string consisting of alphanumeric characters and underscores, where the first character cannot be a digit
     * - The matcher is greedy: it will expand the current match until it can do so no longer
     * - A list of keywords can be supplied, which are matches that the matcher will ignore.  They should also be valid identifiers
     */
    fn apply(&self, input: &Seq) -> Seq {
        let mut i = 0;
        let mut ret = vec![];

        while i < input.len() {
            let mut dfa = DFA::new();
            let mut j = i;
            while !dfa.is_done() {
                dfa = dfa.advance(match input.get(j) {
                    Some(c) => Some(c.1),
                    None => None
                });
                j += 1
            }

            if dfa == DFA::Invalid {
                ret.push(input[j]);
            }
            else {
                let matched_str = &input[i..j].iter().map(|c| c.1).collect::<String>();
                if self.keywords.contains(matched_str.as_str()) { // oops, keyword
                    ret.extend_from_slice(&input[i..j]);
                }
                else {
                    ret.push((i, self.normalize_into));
                }
            }

            i = j;
        }

        ret
    }
}

impl<'a> Identifiers<'a> {
    pub fn new(keywords_vec: Vec<&'a str>, normalize_into: char) -> Self {
        Self {
            keywords: HashSet::from_iter(keywords_vec),
            normalize_into
        }
    }
}