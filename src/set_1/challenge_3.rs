use std::collections::BTreeMap;

pub fn find_xorchar_cipher(msg: &[u8]) -> (f64, u8) {
    let mut scratch = Vec::with_capacity(msg.len());
    let mut max_score = (0.0, 0);
    for b in 0..=255 {
        scratch.clear();
        for m in msg {
            scratch.push(m ^ b);
        }
        let score = is_english_score(&scratch);
        if score < max_score.0 {
            max_score.0 = score;
            max_score.1 = b;
        }
    }
    max_score
}

pub fn english_char_hist() -> BTreeMap<char, f64> {
    let mut map = BTreeMap::new();

    map.insert('E', 11.1607);
    map.insert('A', 8.4966);
    map.insert('R', 7.5809);
    map.insert('I', 7.5448);
    map.insert('O', 7.1635);
    map.insert('T', 6.9509);
    map.insert('N', 6.6544);
    map.insert('S', 5.7351);
    map.insert('L', 5.4893);
    map.insert('C', 4.5388);
    map.insert('U', 3.6308);
    map.insert('D', 3.3844);
    map.insert('P', 3.1671);
    map.insert('M', 3.0129);
    map.insert('H', 3.0034);
    map.insert('G', 2.4705);
    map.insert('B', 2.0720);
    map.insert('F', 1.8121);
    map.insert('Y', 1.7779);
    map.insert('W', 1.2899);
    map.insert('K', 1.1016);
    map.insert('V', 1.0074);
    map.insert('X', 0.2902);
    map.insert('Z', 0.2722);
    map.insert('J', 0.1965);
    map.insert('Q', 0.1962);

    map
}

pub fn is_english_score(input: &[u8]) -> f64 {
    // populate histogram with empty values
    let mut hist = BTreeMap::new();
    for c in 'A'..='Z' {
        hist.insert(c, 0);
    }

    // for every byte, if its a char, increment the char freq in the hist
    for b in input {
        if b.is_ascii_alphabetic() {
            let c = b.to_ascii_uppercase() as char;
            let old = hist.get(&c).unwrap();
            hist.insert(c, old + 1);
        }
    }

    let mut score = 0.0;
    let english = english_char_hist();
    for (in_freq, english_percentage) in hist.values().zip(english.values()) {
        let in_percentage = *in_freq as f64 / input.len() as f64;
        score += (in_percentage.sqrt() - (english_percentage / 100.0).sqrt()).powi(2);
    }
    score = 1.0 - (score.sqrt() / 2.0_f64.sqrt());

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
    use crate::set_1::challenge_1::hex_to_bytes;

    use super::*;

    #[test]
    fn challeng_3() {
        let message = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
        let message_bytes = hex_to_bytes(&message);

        let cipher = find_xorchar_cipher(&message_bytes);
        println!("cipher = {}", cipher.1 as char);

        let decrypted = String::from_utf8(single_char_xor(&message_bytes, cipher.1)).unwrap();
        println!("message: {decrypted}");
    }

    #[test]
    fn easter_egg() {
        let raw_message = "ETAOIN SHRDLU";
        let raw_message_bytes = raw_message.as_bytes();
        let cipher = find_xorchar_cipher(&raw_message_bytes);
        println!("cipher = {}", cipher.1 as char);
        let decrypted = String::from_utf8(single_char_xor(&raw_message_bytes, cipher.1)).unwrap();
        println!("message: {decrypted}");
    }
}
