#[cfg(test)]
mod tests {
    use std::{
        fs::File,
        io::{BufRead, BufReader, Write},
    };

    use crate::set_1::{
        challenge_1::hex_to_bytes,
        challenge_3::{find_xorchar_cipher, single_char_xor},
    };

    #[test]
    fn challenge_4() {
        let read = File::open("data/challenge_4.txt").unwrap();
        let read_lines = BufReader::new(read).lines();

        let mut write = File::create("data/challeng_4_answers.txt").unwrap();

        for (message, line) in read_lines.map_while(Result::ok).zip(0..) {
            let message_bytes = hex_to_bytes(&message);
            let (score, cipher) = find_xorchar_cipher(&message_bytes);
            let decrypted = String::from_utf8(single_char_xor(&message_bytes, cipher));
            if let Ok(d) = decrypted {
                write!(write, "{line}: {d} ({score})\n").unwrap();
            }
        }
    }
}
