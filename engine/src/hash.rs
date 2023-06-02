//! Rabin-karp rolling hash implementation for ease of use
//! 
//! Uses 64-bit hashes (128-bit internally) with fixed base and modulus for more consistent results

use static_init::dynamic;

const LOW_64: i128 = (1 << 64) - 1;

const MODULUS: i128 = 9223372036854775783; // python3 -c "import sympy; f = lambda n: n if sympy.isprime(n) else f(n-1); print(f(2**63))"
const BASE: i128 = 11842660086381224053; // python3 -c "import random; random.seed(22443256); print(random.randint(1, 2**64-1))"

#[inline(always)]
fn mod_add(a: i128, b: i128) -> i128 { let c = a + b; if c >= MODULUS { c - MODULUS } else { c } }
#[inline(always)]
fn mod_sub(a: i128, b: i128) -> i128 { let c: i128 = a - b; if c < 0 { c + MODULUS } else { c } }
#[inline(always)]
fn mod_mul(a: i128, b: i128) -> i128 { (a * b) % MODULUS }

const STORE_BASE_POWERS_TO: usize = 1 << 20;
#[dynamic(lazy)]
static BASE_POWERS: Vec<i128> = {
    (0..STORE_BASE_POWERS_TO).fold(vec![1], |mut acc, _| { acc.push(mod_mul(*acc.last().unwrap(), BASE)); acc })
};

/// Quickly return BASE^p % MODULUS using cached result
/// 
/// For p > STORE_BASE_POWERS_TO, this function will try and calculate it as quickly as possible using the saved results.
/// For p <= STORE_BASE_POWERS_TO, results are automatically pulled from cache in O(1)
fn get_pow(mut p: usize) -> i128 {
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

struct Hash {
    val: i128,
    len: usize
}

impl Hash {
    pub fn new() -> Self {
        Self {
            val: 0,
            len: 0
        }
    }

    pub fn push(&mut self, c: char) {
        self.val = mod_add(mod_mul(self.val, BASE), c as i128);
        self.len += 1;
    }

    // TODO: workon
}