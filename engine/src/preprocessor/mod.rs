//! Preprocessing is the first step done in the plagiarism detection engine.  It is responsible for source code into a language-agnostic 
//! sequence of characters that can be checked for matching substrings.  Notably, code should be processed in a way such that matching substrings
//! of sufficient length indicates copied code.
//! 
//! The preprocessor is implemented as a sequence of steps, each of which transforms the sequence of characters in some way.  Different language processors 
//! use various configurations of steps to process code.
//! 
//! For the engine, the only interface that needs to be exposed is the `langs` module, which provides preprocessor configurations for various languages
//! along with functions to resolve the correct language based on file name and extension.
//! 
//! It may seem a bit weird to put the language-resolution code in here, but remember that MOSS is designed to be language-agnostic, so any language-specific
//! processing should be done in the preprocessor.

/// In the preprocessor, strings are treated as pairs of (index, char) where the element s[i] denotes that the substring from indices
/// s[i].0 to s[i+1].0-1 (inclusive) is compressed as s[i].1.  This is both crucial in intermediate steps and in the final result
pub type Seq = Vec<(usize, char)>;

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
/// Also defines language module to match 
pub mod langs {
    pub enum Lang {
        Java,
        Cpp,
        Python,
    }

    impl Lang {
        /// Converts a full (relative or absolute) file path to a language
        fn from_path<T: AsRef<str>>(path: &T) -> Option<Lang> {
            let ext = path.as_ref().split('.').last()?;
            Self::from_ext(ext)
        }

        fn from_ext(s: &str) -> Option<Lang> {
            // C is treated as C++.  Maybe this should be changed in the future?
            match s {
                "java" => Some(Lang::Java),
                "c" => Some(Lang::Cpp),
                "cc" => Some(Lang::Cpp),
                "cpp" => Some(Lang::Cpp),
                "py" => Some(Lang::Python),
                _ => None,
            }
        }

        fn to_ext(&self) -> &'static str {
            match self {
                Lang::Java => "java",
                Lang::Cpp => "cpp",
                Lang::Python => "py",
            }
        }
    }

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