//! Base62 encoding/decoding for short URLs
//!
//! Converts numeric IDs to short alphanumeric strings and back.
//! Example: 12345 -> "3D7", "3D7" -> 12345

const ALPHABET: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
const BASE: i64 = 62;

/// Encode a number to base62 string
pub fn encode(mut num: i64) -> String {
    if num == 0 {
        return "0".to_string();
    }

    let mut result = Vec::new();

    while num > 0 {
        let remainder = (num % BASE) as usize;
        result.push(ALPHABET[remainder]);
        num /= BASE;
    }

    result.reverse();
    String::from_utf8(result).unwrap()
}

/// Decode a base62 string to number
/// Returns None if the string contains invalid characters
#[allow(dead_code)]
pub fn decode(s: &str) -> Option<i64> {
    if s.is_empty() {
        return None;
    }

    let mut result: i64 = 0;

    for c in s.chars() {
        let digit = match c {
            '0'..='9' => c as i64 - '0' as i64,
            'A'..='Z' => c as i64 - 'A' as i64 + 10,
            'a'..='z' => c as i64 - 'a' as i64 + 36,
            _ => return None,
        };

        result = result.checked_mul(BASE)?.checked_add(digit)?;
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_zero() {
        assert_eq!(encode(0), "0");
    }

    #[test]
    fn test_encode_single_digits() {
        assert_eq!(encode(9), "9");
        assert_eq!(encode(10), "A");
        assert_eq!(encode(35), "Z");
        assert_eq!(encode(36), "a");
        assert_eq!(encode(61), "z");
    }

    #[test]
    fn test_encode_multiple_digits() {
        assert_eq!(encode(62), "10");
        assert_eq!(encode(12345), "3D7");
    }

    #[test]
    fn test_decode_valid() {
        assert_eq!(decode("0"), Some(0));
        assert_eq!(decode("z"), Some(61));
        assert_eq!(decode("10"), Some(62));
        assert_eq!(decode("3D7"), Some(12345));
    }

    #[test]
    fn test_decode_invalid() {
        assert_eq!(decode(""), None);
        assert_eq!(decode("!@#"), None);
    }

    #[test]
    fn test_roundtrip() {
        for n in [0, 1, 61, 62, 100, 12345, 999999] {
            assert_eq!(decode(&encode(n)), Some(n));
        }
    }
}
