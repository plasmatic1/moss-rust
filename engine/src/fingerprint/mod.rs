//! The fingerprint module is responsible for the computation and storage of document fingerprints.  Full document fingerprints are computed by first preprocessing a file, and then
//! computing the fingerprints of the preprocessed file.

use std::collections::VecDeque;

use crate::hash::{RollingHash, self};
use crate::preprocessor::Seq;

// TODO: maybe implement a more robust process rather than just as libraries
// TODO: tests!!!

/// Defined in the same way as crate::preprocessor::Seq, but stores the fingerprint of a document instead of its preprocessed form.
/// The first element of each tuple is the position that the fingerprint starts at in the original document, and the second element is 
/// the fingerprint.
pub type FingerprintSeq = Vec<(usize, u64)>;

/// Fingerprinting: converts a preprocessed sequence of characters into a sequence of fingerprints.  Each fingerprint is annotated with the position that it
/// begins at in the original document.  The algorithm is known as "Robust Winnowing", and is described in the paper "Winnowing: Local Algorithms for Document Fingerprinting" 
/// 
/// The fingerprint is guaranteed to find matches of at least `guarantee_threshold` characters and drop all matches of at most `noise_threshold` characters.
/// 
/// # Arguments
/// * `in_seq` - The preprocessed sequence of characters to fingerprint
/// * `k` - The 'noise threshold': all matches under this size will be ignored
/// * `t` - The 'guarantee threshold': all matches at least this size will be guaranteed to be found when comparing fingerprints
pub fn get_fingerprint(in_seq: &Seq, k: usize, t: usize) -> FingerprintSeq {
    // First compute all k-grams
    let hash_seq = {
        let mut initial_state = RollingHash::from_iter(in_seq[0..k].iter().map(|(_, c)| *c));
        let mut ret = vec![(0 as usize, initial_state.get_u64())];
        for ((i, win_start), (_, win_end)) in in_seq.iter().zip(in_seq[k..].iter()) {
            initial_state.push_char(*win_end);
            initial_state.pop_char(*win_start);
            ret.push((*i, initial_state.get_u64()));
        }
        
        ret
    };

    // We will now compute the fingerprint
    let w = t - k + 1; // window size
    let mut smallest_hashes: VecDeque<(usize, u64)> = VecDeque::new(); // Increasing mono-deq
    let mut fingerprint = vec![];

    for (i, h_pair) in hash_seq.iter().enumerate() {
        // Add new hash to smallest_hashes
        while smallest_hashes.back().is_some_and(|(_, h)| *h >= h_pair.1) {
            smallest_hashes.pop_back();
        }
        smallest_hashes.push_back(*h_pair);

        // Pop old hashes from smallest_hashes
        while smallest_hashes.front().is_some_and(|(j, _)| *j <= i - w) {
            smallest_hashes.pop_front();
        }

        // Update fingerprint
        if i >= w - 1 {
            let window_hash = {
                let best_dq_hash = smallest_hashes.front().unwrap(); 
                match fingerprint.last() {
                    Some(back@(back_i, back_h))
                        if *back_i > i - w && *back_h <= best_dq_hash.1 => Some(back), 
                    _ => None
                }
            };

            if let Some(h) = window_hash {
                fingerprint.push(*h);
            }
        }
    }

    fingerprint
}