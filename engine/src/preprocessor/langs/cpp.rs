use crate::preprocessor::Preprocessor;
use crate::preprocessor::steps;

// Reference: https://en.cppreference.com/w/cpp/keyword
pub const PREPROCESSOR: Preprocessor = vec![
    Box::new(steps::Identifiers::new(
        vec![
            // TODO: add in proper set of keywords
        ],
        'I'
    )),
    Box::new(steps::Whitespace::new())
];
