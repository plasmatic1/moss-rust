/// Wlecome to the zone!!! TODO: elaborate on processing scheme
/// In the preprocessor, strings are treated as pairs of (index, char) where the element s[i] denotes that the substring from indices
/// s[i].0 to s[i+1].0-1 is compressed as s[i].1
type Seq = Vec<(usize, char)>;

/// A single step in a preprocessor that transforms a sequence of characters
pub trait Step {
    fn apply(&self, input: &Seq) -> Seq;
}

/// A preprocessor can be viewed as a sequence of steps
type Preprocessor = Vec<Box<dyn Step>>;

/// Applies a preprocessor to a string
pub fn apply(input: &str, steps: &Preprocessor) -> Seq {
    let init_seq: Seq = input.chars().enumerate().collect();
    steps.iter().fold(init_seq, |cur_seq, step| { step.apply(&cur_seq) })
}

/// Different preprocessor steps that can be applied in sequence to form a full preprocessor
mod steps {
    #[cfg(not(test))]
    mod identifiers;
    mod whitespace;
    mod replace;
    mod comment;

    pub use identifiers::Identifiers;
    pub use whitespace::Whitespace;
    pub use replace::SameSizeReplace;
    pub use comment::Comment;

    #[cfg(test)]
    pub mod identifiers;
}

/// Preprocessor implementations for various languages 
pub mod langs {
    use crate::lang::Lang;

    #[cfg(not(test))]
    mod java;
    #[cfg(not(test))]
    mod cpp;
    #[cfg(not(test))]
    mod python;

    pub fn get_preprocessor(lang: Lang) -> super::Preprocessor {
        match lang {
            Lang::Java => java::get_preprocessor(),
            Lang::Cpp => cpp::get_preprocessor(),
            Lang::Python => python::get_preprocessor(),
        }
    }

    #[cfg(test)]
    pub mod java;
    #[cfg(test)]
    pub mod cpp;
    #[cfg(test)]
    pub mod python;
}

#[cfg(test)]
mod tests;