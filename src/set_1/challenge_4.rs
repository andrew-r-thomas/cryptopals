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

    // #[test]
    // fn challenge_4() {
    //     let read = File::open("data/challenge_4.txt").unwrap();
    //     let read_lines = BufReader::new(read).lines();
    //
    //     struct Score {
    //         message: String,
    //         score: f64,
    //         cipher: char,
    //         decrypted: String,
    //     }
    //
    //     let mut all_scores = Vec::new();
    //     for (message, _) in read_lines.map_while(Result::ok).zip(0..) {
    //         let message_bytes = hex_to_bytes(&message);
    //         let scores = find_xorchar_cipher::<5>(&message_bytes);
    //         for (score, cipher) in scores {
    //             all_scores.push(Score {
    //                 message: message.clone(),
    //                 score,
    //                 cipher: cipher as char,
    //                 decrypted: String::from_utf8(single_char_xor(&message_bytes, cipher))
    //                     .unwrap_or("not utf8".into()),
    //             });
    //         }
    //     }
    //
    //     all_scores.sort_by(|a, b| a.score.partial_cmp(&b.score).unwrap().reverse());
    //
    //     for s in &all_scores[0..5] {
    //         println!("message: {}", s.message);
    //         println!("score: {}", s.score);
    //         println!("cipher: {}", s.cipher);
    //         println!("decrypted: {}", s.decrypted);
    //         println!();
    //     }
    // }
}
