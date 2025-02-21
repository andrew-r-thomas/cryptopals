use crate::set_1::challenge_1::BASE64_CHARS;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    fs,
    ops::Range,
    path::Path,
};

use super::challenge_3::find_xorchar_cipher;

/// assumes a and b have the same length
pub fn hamming_dist(a: &[u8], b: &[u8]) -> usize {
    let mut out = 0;

    for (aa, bb) in a.iter().zip(b.iter()) {
        let mut diff = aa ^ bb;
        let mut count = 0;
        while diff != 0 {
            count += diff & 1;
            diff >>= 1;
        }
        out += count as usize;
    }

    out
}

pub fn base64_map() -> HashMap<char, u8> {
    let mut map = HashMap::new();
    for (i, c) in BASE64_CHARS.iter().enumerate() {
        map.insert(*c as char, i as u8);
    }
    map
}

/// TODO:
pub fn base64_to_bytes(input: &str) -> Vec<u8> {
    let mut out = Vec::new();
    let map = base64_map();
    out
}

#[derive(PartialEq, Eq, Ord)]
struct KeySizeCand {
    size: usize,
    dist: usize,
}

impl PartialOrd for KeySizeCand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.dist.partial_cmp(&other.dist)
    }
}

pub fn break_repeating_xor(input: &[u8], key_range: Range<usize>) -> Vec<u8> {
    // find best n key sizes
    let mut key_size_candidates = BinaryHeap::<Reverse<KeySizeCand>>::new();
    for size in key_range {
        let a = input.get(0..size).unwrap();
        let b = input.get(size..size * 2).unwrap();
        let dist = hamming_dist(&a, &b) / size;
        key_size_candidates.push(Reverse(KeySizeCand { size, dist }));
    }

    // make transposed blocks
    let best_key_size = key_size_candidates.pop().unwrap().0.size;
    let mut blocks = vec![Vec::<u8>::new(); best_key_size];
    for chunk in input.chunks(best_key_size) {
        for (i, b) in chunk.iter().enumerate() {
            blocks[i].push(*b);
        }
    }

    // find the cipher for each block
    let mut out = Vec::with_capacity(best_key_size);
    for block in blocks {
        let (_, cipher) = find_xorchar_cipher(&block);
        out.push(cipher);
    }

    out
}

#[cfg(test)]
mod tests {
    use crate::set_1::challenge_1::bytes_to_base64;

    use super::*;

    #[test]
    fn sanity_check() {
        let a = "this is a test";
        let b = "wokka wokka!!!";
        let expected = 37;
        let out = hamming_dist(a.as_bytes(), b.as_bytes());
        assert_eq!(expected, out);
    }

    // #[test]
    // fn base64_decode() {
    //     let input = "SGVsbG8sIFdvcmxkIQ==";
    //     let out = bytes_to_base64(&base64_to_bytes(input));
    //     assert_eq!(out, "Hello, World!");
    // }

    #[test]
    fn challenge_6() {
        // break_repeating_xor(&Path::new("some/path"), 2..40);
    }
}
