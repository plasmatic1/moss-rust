#[macro_use]
extern crate lazy_static;

// submodules
mod preprocessor;
mod fingerprint;
mod matching;
mod logging;
mod lang;

fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
