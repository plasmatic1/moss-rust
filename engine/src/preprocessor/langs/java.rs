use crate::preprocessor::steps;
use crate::preprocessor::Preprocessor;

// Keywords from: https://docs.oracle.com/javase/tutorial/java/nutsandbolts/_keywords.html
pub fn get_preprocessor() -> Preprocessor {
    vec![
        // Comments
        Box::new(steps::Comment::new(
            "//",
            ("/*", "*/"),
        )),
        // IDs
        // Remove keywords that are typenames
        Box::new(steps::Identifiers::new(keywords(), 'I')),
        Box::new(steps::Whitespace::new()),
    ]
}

#[inline(always)]
pub(crate) fn keywords() -> Vec<&'static str> {
    vec![
        "abstract",
        "continue",
        "for",
        "new",
        "switch",
        "assert",
        "default",
        "goto",
        "package",
        "synchronized",
        // "boolean",
        "do",
        "if",
        "private",
        "this",
        "break",
        // "double",
        "implements",
        "protected",
        "throw",
        // "byte",
        "else",
        "import",
        "public",
        "throws",
        "case",
        "enum",
        "instanceof",
        "return",
        "transient",
        "catch",
        "extends",
        // "int",
        // "short",
        "try",
        // "char",
        "final",
        "interface",
        "static",
        "void",
        "class",
        "finally",
        // "long",
        "strictfp",
        "volatile",
        "const",
        // "float",
        "native",
        "super",
        "while",
    ]
}