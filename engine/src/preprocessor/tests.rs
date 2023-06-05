use std::os::windows::process;

use super::*;
use crate::lang::Lang;

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
    assert_eq!(processed_chars, "defI(I):ifI<=1:returnIreturnI(I-1)+I(I-2)printI(I(10))");
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
    assert_eq!(processed_chars, "#I<I>#I<I>usingnamespaceI;structI{II;I::I<I>I;I(II):I(I),I(I+1,-1){}II(II){if(I[I]!=-1)returnI[I];if(I<=1)returnI[I]=I;returnI[I]=I(I-1)+I(I-2);}};II(){II(10);cout<<I.I(10)<<I;};");
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
    // TODO: finish
    assert_eq!(processed_chars,)
}

#[test]
fn python_indices() {
    let code = "a = 5; b = 6; \"\"\"bruh\"\"\"print(f(a + b))";
    // TODO: finish
}

#[test]
fn cpp_indices() {
    let code = "int a = 5; char* b = 6; cout << f(a + b) << endl;";
    // TODO: finish
}

#[test]
fn java_indices() {
    // simple java program in one line
    let code = "class Main { public static void /*ok*/main(String[] args) { System.out.println(\"Hello World\"); } }";
    // TODO: finish
}