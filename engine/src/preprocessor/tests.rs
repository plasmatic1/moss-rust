use super::*;
use super::langs::Lang;

#[cfg(test)]
fn unsubst_keywords(s: &str, keywords: Vec<&'static str>) -> String {
    keywords.into_iter().enumerate().fold(s.to_string(), |acc, (i, k)| {
        acc.replace(char::from_u32((steps::identifiers::KEYWORD_OFF + i) as u32).unwrap().to_owned(), k)
    })
}

#[cfg(test)]
fn check_match_with_holes<T: Eq>(v1: Vec<T>, v2: Vec<Option<T>>) -> bool {
    v1.iter().zip(v2.iter()).all(|(x, y)| {
        match y {
            Some(y) => x == y,
            None => true,
        }
    })
}

#[test]
fn fib_python_chars() {
    let code = "def fib(n):
    if n <= 1:
        return n
        \"\"\"
        This is a multiline comment.
        In python these are actually just raw strings, but I'll treat these as comments.
        Multiline strings shouldn't really matter for plagiarism detection anyways.
        \"\"\"
    return fib(n-1) + fib(n-2)
    #what in the world

print(fib(10))";

    let processed_chars = apply(code, &langs::get_preprocessor(Lang::Python)).iter().map(|(_, c)| c).collect::<String>();
    assert_eq!(unsubst_keywords(processed_chars.as_str(), langs::python::keywords()), 
        "defI(I):ifI<=1:returnIreturnI(I-1)+I(I-2)I(I(10))");
}

#[test]
fn fib_struct_cpp_chars() {
    let code = "#include <iostream>
    ??= include <vector>
    using namespace std;
    
    struct Fib {
        int n;
        std::vector<int> memo;
        // ok
    
        Fib(int n) : n(n), memo(n+1, -1) {}
    
        int fib(int n) {
            if /*bruh */ (memo[n] != -1) return memo[n];
            if (n <= 1) return memo[n] = n;
            return memo[n] = fib(n-1) + fib(n-2);
        }
    };

    int main() {
        Fib f(10);
        cout << f.fib(10) << endl;
    };";

    let processed_chars = apply(code, &langs::get_preprocessor(Lang::Cpp)).iter().map(|(_, c)| c).collect::<String>();
    assert_eq!(unsubst_keywords(processed_chars.as_str(), langs::cpp::keywords()), 
        "#I<I>#I<I>usingnamespaceI;structI{II;I::I<I>I;I(II):I(I),I(I+1,-1){}II(II){if(I[I]!=-1)returnI[I];if(I<=1)returnI[I]=I;returnI[I]=I(I-1)+I(I-2);}};II(){II(10);I<<I.I(10)<<I;};");
}

#[test]
fn fib_class_java_chars() {
    let code = "
    import java.util.*;
    import java.complex.*; // Kinda a silly import

    class Main {
        public static void main(String[] args) {
            System.out.println(/* this is a ***** effing comment
                that goes mutlilines*/fib(10));
        }
    
        public static int fib(int n) {
            // java comment lol
            if (n <= 1) return n;
            return fib(n-1) + fib(n-2);
        }
    }";

    let processed_chars = apply(code, &langs::get_preprocessor(Lang::Java)).iter().map(|(_, c)| c).collect::<String>();
    assert_eq!(unsubst_keywords(processed_chars.as_str(), langs::java::keywords()),
        "importI.I.*;importI.I.*;classI{publicstaticvoidI(I[]I){I.I.I(I(10));}publicstaticII(II){if(I<=1)returnI;returnI(I-1)+I(I-2);}}");
}

#[test]
fn python_indices() {
    let code = "a = 5; b = 6; \"\"\"bruh\"\"\"print(f(a + # b))";

    let processed= apply(code, &langs::get_preprocessor(Lang::Python));
    // Output: I=5;I=6;I(f(I+
    assert_eq!(processed, vec![
        (0, 'I'), (2, '='), (4, '5'), (5, ';'), (7, 'I'), (9, '='), (11, '6'), (12, ';'), (24, 'I'), (29, '('), (30, 'I'), (31, '('), (32, 'I'), (34, '+')
    ]);
}

#[test]
fn cpp_indices() {
    let code = "??=pragma once /* abcabacbac */ int a = 5; char* b = 6; cout << f(a + b) << // endl;";

    let processed = apply(code, &langs::get_preprocessor(Lang::Cpp));
    // Output: #IIII=5;I*I=6;I<<I(I+I)<<
    assert_eq!(processed, vec![
        (0, '#'), (3, 'I'), (10, 'I'), (32, 'I'), (36, 'I'), (38, '='), (40, '5'), (41, ';'), (43, 'I'), (47, '*'), (49, 'I'), (51, '='), (53, '6'), (54, ';'), (56, 'I'), (61, '<'), (62, '<'), (64, 'I'), (65, '('), (66, 'I'), (68, '+'), (70, 'I'), (71, ')'), (73, '<'), (74, '<')
    ]);
}

#[test]
fn java_indices() {
    // simple java program in one line
    let code = "class Main { public static void /*ok*/main(String[] args) { System.out.println(\"Hello World\"); // } }";

    let processed = apply(code, &langs::get_preprocessor(Lang::Java));
    // Output (keywords): classI{publicstaticvoidI(I[]I){I.I.I("II");
    // Output           : ?I{???I(I[]I){I.I.I("II");
    let expect = vec![
        (0, None),
        (6, Some('I')),
        (11, Some('{')),
        (13, None),
        (20, None),
        (27, None),
        (38, Some('I')),
        (42, Some('(')),
        (43, Some('I')),
        (49, Some('[')),
        (50, Some(']')),
        (52, Some('I')),
        (56, Some(')')),
        (58, Some('{')),
        (60, Some('I')),
        (66, Some('.')),
        (67, Some('I')),
        (70, Some('.')),
        (71, Some('I')),
        (78, Some('(')),
        (79, Some('"')),
        (80, Some('I')),
        (86, Some('I')),
        (91, Some('"')),
        (92, Some(')')),
        (93, Some(';'))
    ];

    assert_eq!(processed.iter().map(|t| t.0).collect::<Vec<usize>>(), expect.iter().map(|t| t.0).collect::<Vec<usize>>());
    assert!(check_match_with_holes(processed.iter().map(|t| t.1).collect(), expect.iter().map(|t| t.1).collect()));
}