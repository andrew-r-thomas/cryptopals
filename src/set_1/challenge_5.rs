pub fn repeating_key_xor(message: &[u8], key: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(message.len());

    for (m, k) in message.iter().zip(key.iter().cycle()) {
        out.push(m ^ k);
    }

    return out;
}

#[cfg(test)]
mod tests {
    use crate::set_1::challenge_1::bytes_to_hex;

    use super::*;

    #[test]
    fn challenge_5() {
        let message = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
        let expected = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";
        let key = "ICE";
        let out = bytes_to_hex(&repeating_key_xor(message.as_bytes(), key.as_bytes()));
        assert_eq!(expected, out);
    }

    #[test]
    fn reversable() {
        let message = "this is a test, idk man";
        let key = "test";
        let encrypted = repeating_key_xor(message.as_bytes(), key.as_bytes());
        let decrypted = repeating_key_xor(&encrypted, key.as_bytes());
        assert_eq!(message, String::from_utf8(decrypted).unwrap());
    }
}
