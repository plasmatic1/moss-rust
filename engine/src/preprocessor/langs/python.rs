use crate::preprocessor::Preprocessor;
use crate::preprocessor::steps;

// Reference: https://docs.python.org/3/library/keyword.html
// On CPython 3.9.13 (Windows 11), the following keywords are reserved:
// ['False', 'None', 'True', '__peg_parser__', 'and', 'as', 'assert', 'async', 'await', 'break', 'class', 'continue', 'def', 'del', 'elif', 'else', 'except', 'finally', 'for', 'from', 'global', 'if', 'import', 'in', 'is', 'lambda', 'nonlocal', 'not', 'or', 'pass', 'raise', 'return', 'try', 'while', 'with', 'yield']
pub fn get_preprocessor() -> Preprocessor {
    vec![
        // Comments (2 types)
        Box::new(steps::Comment::new(
            "#",
            ("\"\"\"", "\"\"\""),
        )),
        Box::new(steps::Comment::new(
            "#", // can be anything, really
            ("'''", "'''"),
        )),
        Box::new(steps::Identifiers::new(keywords(), 'I')),
        Box::new(steps::Whitespace::new())
    ]
}

#[inline(always)]
pub(crate) fn keywords() -> Vec<&'static str> {
    vec![
        "False",
        "None",
        "True",
        "__peg_parser__",
        "and",
        "as",
        "assert",
        "async",
        "await",
        "break",
        "class",
        "continue",
        "def",
        "del",
        "elif",
        "else",
        "except",
        "finally",
        "for",
        "from",
        "global",
        "if",
        "import",
        "in",
        "is",
        "lambda",
        "nonlocal",
        "not",
        "or",
        "pass",
        "raise",
        "return",
        "try",
        "while",
        "with",
        "yield"
    ]
}