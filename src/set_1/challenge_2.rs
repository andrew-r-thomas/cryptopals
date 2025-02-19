pub fn fixed_xor(a: &[u8], b: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    for (aa, bb) in a.iter().zip(b.iter()) {
        out.push(aa ^ bb);
    }
    out
}

#[cfg(test)]
mod tests {
    use crate::set_1::challenge_1::{bytes_to_hex, hex_to_bytes};

    use super::*;

    #[test]
    fn challenge_2() {
        let input_a = "1c0111001f010100061a024b53535009181c";
        let input_b = "686974207468652062756c6c277320657965";
        let expected = "746865206b696420646f6e277420706c6179";

        let out = fixed_xor(&hex_to_bytes(&input_a), &hex_to_bytes(&input_b));

        assert_eq!(bytes_to_hex(&out), expected);
    }
}
