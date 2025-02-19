pub fn find_xorchar_cipher(msg: &[u8]) -> u8 {
    let mut scratch = Vec::with_capacity(msg.len());
    let mut max_freq = (0, 0);
    for b in 0..=255 {
        scratch.clear();
        for m in msg {
            scratch.push(m ^ b);
        }
        let freq = char_freq(&scratch);
        if freq > max_freq.0 {
            max_freq.0 = freq;
            max_freq.1 = b;
        }
    }
    max_freq.1
}

pub fn char_freq(input: &[u8]) -> usize {
    let mut out = 0;
    for byte in input {
        if byte.is_ascii_alphabetic() {
            out += 1;
        }
    }
    out
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
    use crate::set_1::{
        challenge_1::{bytes_to_hex, hex_to_bytes},
        challenge_3::single_char_xor,
    };

    use super::*;

    #[test]
    fn challeng_3() {
        let message = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
        let message_bytes = hex_to_bytes(&message);

        let cipher = find_xorchar_cipher(&message_bytes);
        println!("cipher = {}", cipher as char);

        let decrypted = String::from_utf8(single_char_xor(&message_bytes, cipher)).unwrap();
        println!("message: {decrypted}");
    }

    #[test]
    fn easter_egg() {
        let raw_message = "ETAOIN SHRDLU";
        let raw_message_bytes = raw_message.as_bytes();
        let cipher = find_xorchar_cipher(&raw_message_bytes);
        println!("cipher = {}", cipher as char);
        let decrypted = String::from_utf8(single_char_xor(&raw_message_bytes, cipher)).unwrap();
        println!("message: {decrypted}");
    }
}
