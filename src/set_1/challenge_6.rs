use crate::set_1::challenge_1::BASE64_CHARS;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    ops::Range,
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

fn decode_base64(input: &str) -> Result<Vec<u8>, &'static str> {
    // Base64 alphabet lookup table
    const DECODE_TABLE: [i8; 256] = {
        let mut table = [-1i8; 256];
        let alphabet = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

        let mut i = 0;
        while i < 64 {
            table[alphabet[i] as usize] = i as i8;
            i += 1;
        }
        table['=' as usize] = -2; // Padding character
        table
    };

    // Calculate the length of the output buffer
    // Every 4 base64 characters become 3 bytes (except for padding)
    let input_length = input.len();
    if input_length % 4 != 0 {
        return Err("Invalid base64 length");
    }

    let padding_count = input.chars().rev().take(2).filter(|&c| c == '=').count();
    let output_length = input_length / 4 * 3 - padding_count;
    let mut output = Vec::with_capacity(output_length);

    // Process input in groups of 4 characters
    let input_bytes = input.as_bytes();
    let mut i = 0;
    while i < input_length {
        // Convert each character to its 6-bit value using the lookup table
        let b1 = DECODE_TABLE[input_bytes[i] as usize];
        let b2 = DECODE_TABLE[input_bytes[i + 1] as usize];
        let b3 = DECODE_TABLE[input_bytes[i + 2] as usize];
        let b4 = DECODE_TABLE[input_bytes[i + 3] as usize];

        // Check for invalid characters
        if b1 < 0 || b2 < 0 || (b3 < -2) || (b4 < -2) {
            return Err("Invalid base64 character");
        }

        // Combine the 6-bit values into bytes
        let triple = ((b1 as u32) << 18)
            | ((b2 as u32) << 12)
            | (((if b3 >= 0 { b3 } else { 0 }) as u32) << 6)
            | ((if b4 >= 0 { b4 } else { 0 }) as u32);

        // Add the decoded bytes to the output
        output.push(((triple >> 16) & 0xFF) as u8);
        if b3 >= 0 {
            output.push(((triple >> 8) & 0xFF) as u8);
        }
        if b4 >= 0 {
            output.push((triple & 0xFF) as u8);
        }

        i += 4;
    }

    Ok(output)
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
        let cipher = find_xorchar_cipher::<1>(&block);
        out.push(cipher[0].1);
    }

    out
}

#[cfg(test)]
mod tests {
    use std::{
        fs::{self, File},
        io::{BufRead, BufReader},
    };

    use crate::set_1::{challenge_1::bytes_to_base64, challenge_5::repeating_key_xor};

    use super::*;

    #[test]
    fn sanity_check() {
        let a = "this is a test";
        let b = "wokka wokka!!!";
        let expected = 37;
        let out = hamming_dist(a.as_bytes(), b.as_bytes());
        assert_eq!(expected, out);
    }

    #[test]
    fn base64_decode() {
        let input = "SGVsbG8sIFdvcmxkIQ==";
        let out = String::from_utf8(decode_base64(input).unwrap()).unwrap();
        assert_eq!(out, "Hello, World!");
    }

    #[test]
    fn challenge_6() {
        let f = File::open("data/challenge_6.txt").unwrap();
        let reader = BufReader::new(f).lines();
        let mut input = Vec::new();
        for line in reader.map_while(Result::ok) {
            input.append(&mut decode_base64(&line).unwrap());
        }
        let key = break_repeating_xor(&input, 2..40);
        println!(
            "key: {}",
            String::from_utf8(key.clone()).unwrap_or("not utf8".into())
        );

        let decrypted = repeating_key_xor(&input, &key);
        println!(
            "decrypted: {}",
            String::from_utf8(decrypted).unwrap_or("not utf8".into())
        );
    }
}
