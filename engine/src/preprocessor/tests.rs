use super::*;

#[test]
fn fib_python() {
    let code = "def fib(n):
    if n <= 1:
        return n
    return fib(n-1) + fib(n-2)

print(fib(10))";

    assert_eq!(apply(code, &langs::get_preprocessor(Lang::Python)), vec![
    ]);
}

#[test]
fn fib_struct_cpp() {
    let code = "#include <iostream>
    #include <vector>
    using namespace std;
    
    struct Fib {
        int n;
        std::vector<int> memo;
    
        Fib(int n) : n(n), memo(n+1, -1) {}
    
        int fib(int n) {
            if (memo[n] != -1) return memo[n];
            if (n <= 1) return memo[n] = n;
            return memo[n] = fib(n-1) + fib(n-2);
        }
    };

    int main() {
        Fib f(10);
        cout << f.fib(10) << endl;
    };";

    assert_eq!(apply(code, &langs::get_preprocessor(Lang::Cpp)), vec![
    ]);
}

#[test]
fn fib_class_java() {
    let code = "
    import java.util.*;
    import java.complex.*; // Kinda a silly import

    class Main {
        public static void main(String[] args) {
            System.out.println(fib(10));
        }
    
        public static int fib(int n) {
            if (n <= 1) return n;
            return fib(n-1) + fib(n-2);
        }
    }";
}