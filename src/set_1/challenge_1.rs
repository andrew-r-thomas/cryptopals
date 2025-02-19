const BASE64_CHARS: &'static [u8] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

pub fn hex_to_base64(input: &str) -> String {
    bytes_to_base64(&hex_to_bytes(input))
}

pub fn hex_to_bytes(input: &str) -> Vec<u8> {
    input
        .chars()
        .collect::<Vec<char>>()
        .chunks(2)
        .map(|chunk| {
            let high = chunk[0].to_digit(16).unwrap() as u8;
            let low = chunk[1].to_digit(16).unwrap() as u8;
            high << 4 | low
        })
        .collect()
}

pub fn bytes_to_hex(input: &[u8]) -> String {
    let mut out = String::new();
    for byte in input {
        out.push_str(&format!("{:02x}", byte));
    }
    out
}

pub fn bytes_to_base64(input: &[u8]) -> String {
    let mut out = String::new();

    for chunk in input.chunks(3) {
        let len = chunk.len();
        let mut combined = (chunk[0] as u32) << 16;
        if len > 1 {
            combined |= (chunk[1] as u32) << 8;
        }
        if len > 2 {
            combined |= chunk[2] as u32;
        }

        let c1 = ((combined >> 18) & 0b00111111) as u8;
        let c2 = ((combined >> 12) & 0b00111111) as u8;
        let c3 = ((combined >> 6) & 0b00111111) as u8;
        let c4 = (combined & 0b00111111) as u8;

        out.push(BASE64_CHARS[c1 as usize] as char);
        out.push(BASE64_CHARS[c2 as usize] as char);

        if len > 1 {
            out.push(BASE64_CHARS[c3 as usize] as char);
        } else {
            out.push('=');
        }
        if len > 2 {
            out.push(BASE64_CHARS[c4 as usize] as char);
        } else {
            out.push('=');
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn challenge_1() {
        let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
        let out = hex_to_base64(&input);
        assert_eq!(out, expected);
    }
}
