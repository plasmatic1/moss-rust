// In the preprocessor, strings are treated as pairs of (index, char) where entry s[i] denotes that the substring from indices s[i].0 to s[i+1].0-1 is compressed as s[i].1
type Seq = Vec<(usize, char)>;
type Preprocessor = Vec<Box<dyn Step>>;

pub trait Step {
    fn apply(&self, input: &Seq) -> Seq;
}

pub fn apply(input: &str, steps: &Preprocessor) -> Seq {
    let init_seq: Seq = input.chars().enumerate().collect();
    steps.iter().fold(init_seq, |cur_seq, step| { step.apply(&cur_seq) })
}

mod steps {
    mod identifiers;
    mod whitespace;

    pub use identifiers::Identifiers;
    pub use whitespace::Whitespace;
}

mod langs {
    use crate::lang::Lang;

    mod java;
    mod cpp;
    mod python;

    fn get_preprocessor(lang: Lang) -> super::Preprocessor {
        match lang {
            Lang::Java => java::get_preprocessor(),
            Lang::Cpp => cpp::get_preprocessor(),
            Lang::Python => python::get_preprocessor(),
        }
    }
}