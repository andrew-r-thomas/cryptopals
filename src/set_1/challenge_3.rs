use std::collections::BTreeMap;

pub fn find_xorchar_cipher<const NUM_OUT: usize>(msg: &[u8]) -> [(f64, u8); NUM_OUT] {
    let mut scratch = Vec::with_capacity(msg.len());
    let mut scores = [(f64::MAX, 0); 256];
    for b in 0..=255 {
        scratch.clear();
        for m in msg {
            scratch.push(m ^ b);
        }
        let score = is_english_score(&scratch);
        scores[b as usize] = (score, b);
    }
    let mut out = [(0.0, 0); NUM_OUT];
    scores.sort_by(|a, b| a.partial_cmp(b).unwrap().reverse());
    for i in 0..NUM_OUT {
        out[i] = scores[i];
    }
    out
}

pub fn english_char_hist() -> BTreeMap<char, f64> {
    let mut map = BTreeMap::new();

    map.insert('E', 0.111607);
    map.insert('A', 0.084966);
    map.insert('R', 0.075809);
    map.insert('I', 0.075448);
    map.insert('O', 0.071635);
    map.insert('T', 0.069509);
    map.insert('N', 0.066544);
    map.insert('S', 0.057351);
    map.insert('L', 0.054893);
    map.insert('C', 0.045388);
    map.insert('U', 0.036308);
    map.insert('D', 0.033844);
    map.insert('P', 0.031671);
    map.insert('M', 0.030129);
    map.insert('H', 0.030034);
    map.insert('G', 0.024705);
    map.insert('B', 0.020720);
    map.insert('F', 0.018121);
    map.insert('Y', 0.017779);
    map.insert('W', 0.012899);
    map.insert('K', 0.011016);
    map.insert('V', 0.010074);
    map.insert('X', 0.002902);
    map.insert('Z', 0.002722);
    map.insert('J', 0.001965);
    map.insert('Q', 0.001962);

    map
}

pub fn is_english_score(input: &[u8]) -> f64 {
    // populate histogram of char frequencies in input;
    let mut hist = BTreeMap::new();
    for c in 'A'..='Z' {
        hist.insert(c, 0);
    }
    for b in input {
        let c = b.to_ascii_uppercase() as char;
        let new_freq = match hist.get(&c) {
            Some(f) => f + 1,
            None => 1,
        };
        hist.insert(c, new_freq);
    }

    let mut score = 0.0;
    let english = english_char_hist();
    for (c, freq) in hist.iter() {
        score += (english.get(c).unwrap_or(&0.0) * (*freq as f64 / input.len() as f64)).sqrt();
    }

    score
}

pub fn single_char_xor(msg: &[u8], c: u8) -> Vec<u8> {
    let mut out = Vec::with_capacity(msg.len());
    for m in msg {
        out.push(m ^ c);
    }
    out
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Write};

    use crate::set_1::challenge_1::hex_to_bytes;

    use super::*;

    // #[test]
    // fn challeng_3() {
    //     let message = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    //     let message_bytes = hex_to_bytes(&message);
    //
    //     let ciphers = find_xorchar_cipher::<10>(&message_bytes);
    //     for cipher in ciphers {
    //         println!("cipher = {}", cipher.1 as char);
    //         println!("score = {}", cipher.0);
    //         let decrypted = String::from_utf8(single_char_xor(&message_bytes, cipher.1)).unwrap();
    //         println!("message: {decrypted}");
    //         println!();
    //     }
    // }
}
