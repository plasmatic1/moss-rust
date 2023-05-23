use crate::preprocessor::Seq;

pub fn whitespace(input: &Seq) -> Seq {
    input.iter().filter(|c| c.1.is_whitespace())
        .map(|c| *c)
        .collect::<Seq>()
}