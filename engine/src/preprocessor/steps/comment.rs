use crate::preprocessor::{Step, Seq};

pub struct Comment {
    line_token: Vec<char>,
    block_token: (Vec<char>, Vec<char>),
}

impl Comment {
    pub fn new(line_token: &str, block_token: (&str, &str)) -> Self {
        Self {
            line_token: line_token.chars().collect(),
            block_token: (block_token.0.chars().collect(), block_token.1.chars().collect()),
        }
    }
}

/// shorthand substring function
fn check_for_str(haystack: &Seq, index: usize, needle: &Vec<char>) -> bool {
    index + needle.len() <= haystack.len() && // have enough space!!
        haystack[index..index + needle.len()].iter().enumerate().all(|(j, (_, c))| *c == needle[j]) // compare chars
}

impl Step for Comment {
    fn apply(&self, input: &Seq) -> Seq {
        let mut ret = vec![];
        let mut i = 0;

        while i < input.len() {
            if check_for_str(input, i + 1, &self.line_token) { // Found a line comment
                ret.push(input[i]);

                // TODO: CRLF
                while i < input.len() && input[i].1 != '\n' {
                    i += 1;
                }
                if i < input.len() && input[i].1 == '\n' { // skip the newline
                    i += 1;
                }
            }
            else if check_for_str(input, i + 1, &self.block_token.0) { 
                ret.push(input[i]);

                i += 1 + self.block_token.0.len();
                while i < input.len() && !check_for_str(input, i, &self.block_token.1) {
                    i += 1;
                }
                if check_for_str(input, i, &self.block_token.1) { // skip the ending block
                    i += self.block_token.1.len();
                }
            }
            else { // not a comment
                ret.push(input[i]);
                i += 1
            }
        }

        ret
    }
}