use crate::preprocessor::{Step, Seq};

pub struct Whitespace {}

impl Step for Whitespace {
    fn apply(&self, input: &Seq) -> Seq {
        input.iter().filter(|c| !c.1.is_whitespace())
            .map(|c| *c)
            .collect::<Seq>()
    }
}

impl Whitespace {
    pub fn new() -> Self {
        Self {}
    }
}
