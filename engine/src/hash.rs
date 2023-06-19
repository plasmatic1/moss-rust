//! Implementation of the Rabin-karp rolling hash
//! 
//! Hashes are represented as values in the unsigned 64-bit range, with an internal 128-bit representation.
//! The hashing algorithm uses a fixed base and modulus for deterministic behaviour, and the probability of collision
//! is ~2^(-64).

use static_init::dynamic;

const MODULUS: u128 = 18446744073709551557; // python3 -c "import sympy; f = lambda n: n if sympy.isprime(n) else f(n-1); print(f(2**64-1))"
const BASE: u128 = 11842660086381224053; // python3 -c "import random; random.seed(22443256); print(random.randint(1, 2**64-1))"

#[inline(always)]
fn mod_add(a: u128, b: u128) -> u128 { let c = a + b; if c >= MODULUS { c - MODULUS } else { c } }
#[inline(always)]
fn mod_sub(a: u128, b: u128) -> u128 { let c = a + MODULUS - b; if c >= MODULUS { c - MODULUS } else { c } }
#[inline(always)]
fn mod_mul(a: u128, b: u128) -> u128 { (a * b) % MODULUS }

const STORE_BASE_POWERS_TO: usize = 1 << 20;
#[dynamic(lazy)]
static BASE_POWERS: Vec<u128> = {
    let ret = (0..STORE_BASE_POWERS_TO).fold(vec![1], |mut acc, _| { acc.push(mod_mul(*acc.last().unwrap(), BASE)); acc });
    debug_assert!(ret.len() == STORE_BASE_POWERS_TO + 1);
    ret
};

/// Quickly return BASE^p % MODULUS using cached result
/// 
/// For p > STORE_BASE_POWERS_TO, this function will try and calculate it as quickly as possible using the saved results.
/// For p <= STORE_BASE_POWERS_TO, results are automatically pulled from cache in O(1)
fn get_pow(mut p: usize) -> u128 {
    if p > STORE_BASE_POWERS_TO {
        let mut ret = 1;
        while p > STORE_BASE_POWERS_TO {
            p -= STORE_BASE_POWERS_TO;
            ret = mod_mul(ret, BASE_POWERS[STORE_BASE_POWERS_TO]);
        }
        mod_mul(ret, BASE_POWERS[p])
    }
    else { BASE_POWERS[p] }
}

/// Maintains the current state of the Rabin-Karp algorithm on a string.  The state can be updated by adding characters to the end,
/// popping characters from the start, or clearing the state.
/// 
/// Note that we store the hash of individual characters as `c + 1` instead of `c` so that pushing the null-byte changes the hash
pub struct RollingHash {
    val: u128,
    len: usize
}

impl RollingHash {
    /// Constructs a new RollingHash state representing one with no characters
    pub fn new() -> Self {
        Self {
            val: 0,
            len: 0
        }
    }

    /// Constructs a new RollingHash state using an iterator as a string
    pub fn from_iter(iter: impl Iterator<Item = char>) -> Self {
        iter.fold(Self::new(), |mut acc, c| { acc.push_char(c); acc })
    }

    /// Constructs a new RollingHash state using a string
    pub fn from_str(s: &str) -> Self {
        RollingHash::from_iter(s.chars())
    }

    /// Updates the state as if a character was added to the end of the string
    /// We use `c + 1` instead of `c` so that pushing the null-byte changes the hash
    pub fn push_char(&mut self, c: char) {
        self.val = mod_add(mod_mul(self.val, BASE), c as u128 + 1);
        self.len += 1;
    }

    /// Updates the state as if a character was removed from the start of the string
    /// We use `c + 1` instead of `c` so that pushing the null-byte changes the hash
    pub fn pop_char(&mut self, c: char) {
        debug_assert!(self.len > 0, "Cannot pop from empty hash");
        self.val = mod_sub(self.val, mod_mul(get_pow(self.len - 1), c as u128 + 1));
        self.len -= 1;
    }

    /// Updates the state as if the string was cleared
    pub fn clear(&mut self) {
        self.val = 0;
        self.len = 0;
    }

    /// Returns the number of characters in the string that the state represents
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns the hash value
    pub fn get_u64(&self) -> u64 {
        self.val as u64
    }
}

#[cfg(test)]
mod tests {
    use std::{vec, collections::HashMap};

    use super::*;

    const TEST_STR_1: &str = "Hello, world!";
    const TEST_STR_2: &str = "lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.";

    /// Check if the creation methods agree
    #[test]
    fn test_create_fns() {
        let test_str = String::from(TEST_STR_1);

        let mut h1 = RollingHash::new();
        for c in test_str.chars() {
            h1.push_char(c);
        }

        let h2 = RollingHash::from_iter(test_str.chars());
        let h3 = RollingHash::from_str(&test_str);

        assert_eq!(h1.get_u64(), h2.get_u64());
        assert_eq!(h1.get_u64(), h3.get_u64());
        assert_eq!(h2.get_u64(), h3.get_u64());
        assert_eq!(h1.len(), h2.len());
        assert_eq!(h1.len(), h3.len());
        assert_eq!(h2.len(), h3.len());
    }

    // TODO: remove (do I really need this lol)
    // fn make_idx_hashmap<T: Eq + std::hash::Hash>(seq: Vec<T>) -> HashMap<T, Vec<usize>> {
    //     seq.into_iter().enumerate().fold(HashMap::new(),
    //         |mut acc, (i, s)| { acc.entry(s).or_insert(vec![]).push(i); acc })
    // }

    /// Checks if the hashes match direct string comparison
    #[test]
    fn test_hash_correctness_iter() {
        let test_str: String = String::from(TEST_STR_2);
        for w in 1..=test_str.len() {
            let chars = test_str.chars().collect::<Vec<_>>();

            // Make hashmap from strings directly
            let strs = chars.windows(w).map(|s| s.iter().collect::<String>()).collect::<Vec<_>>();

            // Make hashmap from iters
            let h_from_iter = chars.windows(w).map(|s| {
                RollingHash::from_iter(s.iter().map(|c| *c)).get_u64()
            }).collect::<Vec<_>>();

            // Check that they have the same equality properties
            assert_eq!(strs.len(), h_from_iter.len());
            for i in 0..strs.len() {
                for j in i+1..strs.len() {
                    assert_eq!(strs[i] == strs[j], h_from_iter[i] == h_from_iter[j]);
                }
            }

            // TODO: remove (check same: old)
            // for i in 0..chars.len() - w + 1 {
            //     let ent_exp = strs_map.get(&chars[i..i+w].iter().collect::<String>()).unwrap();
            //     let ent_real = h_from_iter_map.get(&RollingHash::from_iter(chars[i..i+w].iter().map(|c| *c)).get_u64()).unwrap();

            //     assert_eq!(ent_exp, ent_real);
            // }
        }
    }

    /// Checks if `push_char` and `pop_char` match `from_iter`/`from_str`
    #[test]
    fn test_push_pop() {
        let test_str: String = String::from(TEST_STR_2);
        for w in 1..=test_str.len() {
            let chars = test_str.chars().collect::<Vec<_>>();

            // Generate hashes from iters
            let h_from_iter = chars.windows(w).map(|s| {
                RollingHash::from_iter(s.iter().map(|c| *c)).get_u64()
            }).collect::<Vec<_>>();

            // Generate hashes from push and pop
            let h_push_pop = {
                let mut ret = vec![];
                let mut h_state = RollingHash::new();
                for c in &chars[..w-1] {
                    h_state.push_char(*c);
                }
                for i in w-1..chars.len() {
                    h_state.push_char(chars[i]);
                    ret.push(h_state.get_u64());
                    h_state.pop_char(chars[i+1-w]);
                }

                ret
            };

            // Compare
            assert_eq!(h_from_iter, h_push_pop);
        }
    }

    /// Test clear functionality
    #[test]
    fn test_clear() {
        let test_str = String::from(TEST_STR_1);

        let mut h1 = RollingHash::new();
        for c in test_str.chars() {
            h1.push_char(c);
        }

        assert_eq!(h1.len(), test_str.len());
        assert_ne!(h1.get_u64(), 0);

        h1.clear();

        assert_eq!(h1.len(), 0);
        assert_eq!(h1.get_u64(), 0);
    }
}