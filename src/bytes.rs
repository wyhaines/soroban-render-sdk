//! Byte manipulation utilities for Soroban Render contracts.
//!
//! These functions provide common operations for working with `Bytes` in a `no_std` environment.

use soroban_sdk::{Bytes, Env, String, Vec};

/// Maximum supported string length for conversion.
/// Strings longer than this cannot be fully converted due to Soroban SDK
/// limitations (copy_into_slice requires a buffer >= string length).
pub const MAX_STRING_SIZE: usize = 16384;

/// Concatenate a vector of Bytes into a single Bytes object.
///
/// # Example
///
/// ```rust,ignore
/// let mut parts: Vec<Bytes> = Vec::new(&env);
/// parts.push_back(Bytes::from_slice(&env, b"Hello, "));
/// parts.push_back(Bytes::from_slice(&env, b"World!"));
/// let result = concat_bytes(&env, &parts);
/// // result contains "Hello, World!"
/// ```
pub fn concat_bytes(env: &Env, parts: &Vec<Bytes>) -> Bytes {
    let mut result = Bytes::new(env);
    for part in parts.iter() {
        result.append(&part);
    }
    result
}

/// Convert a soroban_sdk::String to Bytes.
///
/// Uses tiered buffer sizes for efficiency: 256B, 1KB, 4KB, or 16KB based on
/// string length. Strings up to 16KB are fully converted. Strings exceeding
/// 16KB return a placeholder message since Soroban's `copy_into_slice` requires
/// a buffer at least as large as the string.
///
/// # Example
///
/// ```rust,ignore
/// let s = String::from_str(&env, "Hello");
/// let bytes = string_to_bytes(&env, &s);
/// ```
pub fn string_to_bytes(env: &Env, s: &String) -> Bytes {
    let len = s.len() as usize;

    if len == 0 {
        return Bytes::new(env);
    }

    // Tiered buffers to balance stack usage vs. capability.
    // Each tier only allocates its specific size on the stack.
    if len <= 256 {
        let mut buf = [0u8; 256];
        s.copy_into_slice(&mut buf[..len]);
        return Bytes::from_slice(env, &buf[..len]);
    }

    if len <= 1024 {
        let mut buf = [0u8; 1024];
        s.copy_into_slice(&mut buf[..len]);
        return Bytes::from_slice(env, &buf[..len]);
    }

    if len <= 4096 {
        let mut buf = [0u8; 4096];
        s.copy_into_slice(&mut buf[..len]);
        return Bytes::from_slice(env, &buf[..len]);
    }

    if len <= MAX_STRING_SIZE {
        let mut buf = [0u8; MAX_STRING_SIZE];
        s.copy_into_slice(&mut buf[..len]);
        return Bytes::from_slice(env, &buf[..len]);
    }

    // String exceeds maximum supported size.
    // We cannot truncate because copy_into_slice requires a buffer >= string length.
    Bytes::from_slice(env, b"[content exceeds 16KB limit]")
}

/// Convert a u32 to its decimal Bytes representation.
///
/// # Example
///
/// ```rust,ignore
/// let bytes = u32_to_bytes(&env, 42);
/// // bytes contains "42"
/// ```
pub fn u32_to_bytes(env: &Env, n: u32) -> Bytes {
    if n == 0 {
        return Bytes::from_slice(env, b"0");
    }

    let mut num = n;
    let mut digits: [u8; 10] = [0; 10]; // u32 max is 4,294,967,295 (10 digits)
    let mut i = 0;

    while num > 0 {
        digits[i] = b'0' + (num % 10) as u8;
        num /= 10;
        i += 1;
    }

    // Reverse the digits
    let mut result = Bytes::new(env);
    for j in (0..i).rev() {
        result.push_back(digits[j]);
    }
    result
}

/// Convert an i64 to its decimal Bytes representation.
///
/// Handles negative numbers by prepending a minus sign.
///
/// # Example
///
/// ```rust,ignore
/// let bytes = i64_to_bytes(&env, -42);
/// // bytes contains "-42"
/// ```
pub fn i64_to_bytes(env: &Env, n: i64) -> Bytes {
    if n == 0 {
        return Bytes::from_slice(env, b"0");
    }

    let negative = n < 0;
    let mut num = if negative { -(n as i128) } else { n as i128 } as u64;
    let mut digits: [u8; 20] = [0; 20]; // i64 max is 19 digits + sign
    let mut i = 0;

    while num > 0 {
        digits[i] = b'0' + (num % 10) as u8;
        num /= 10;
        i += 1;
    }

    // Build result with optional minus sign
    let mut result = Bytes::new(env);
    if negative {
        result.push_back(b'-');
    }
    for j in (0..i).rev() {
        result.push_back(digits[j]);
    }
    result
}

/// Escape a String for safe inclusion in JSON.
///
/// Escapes the following characters:
/// - `"` → `\"`
/// - `\` → `\\`
/// - newline → `\n`
/// - carriage return → `\r`
/// - tab → `\t`
///
/// # Example
///
/// ```rust,ignore
/// let s = String::from_str(&env, "Hello \"World\"");
/// let escaped = escape_json_string(&env, &s);
/// // escaped contains: Hello \"World\"
/// ```
pub fn escape_json_string(env: &Env, s: &String) -> Bytes {
    let input = string_to_bytes(env, s);
    let mut result = Bytes::new(env);

    for i in 0..input.len() {
        if let Some(b) = input.get(i) {
            match b {
                b'"' => {
                    result.push_back(b'\\');
                    result.push_back(b'"');
                }
                b'\\' => {
                    result.push_back(b'\\');
                    result.push_back(b'\\');
                }
                b'\n' => {
                    result.push_back(b'\\');
                    result.push_back(b'n');
                }
                b'\r' => {
                    result.push_back(b'\\');
                    result.push_back(b'r');
                }
                b'\t' => {
                    result.push_back(b'\\');
                    result.push_back(b't');
                }
                _ => {
                    result.push_back(b);
                }
            }
        }
    }

    result
}

/// Escape a byte slice for safe inclusion in JSON.
///
/// Like `escape_json_string` but works directly with byte slices.
pub fn escape_json_bytes(env: &Env, input: &[u8]) -> Bytes {
    let mut result = Bytes::new(env);

    for &b in input {
        match b {
            b'"' => {
                result.push_back(b'\\');
                result.push_back(b'"');
            }
            b'\\' => {
                result.push_back(b'\\');
                result.push_back(b'\\');
            }
            b'\n' => {
                result.push_back(b'\\');
                result.push_back(b'n');
            }
            b'\r' => {
                result.push_back(b'\\');
                result.push_back(b'r');
            }
            b'\t' => {
                result.push_back(b'\\');
                result.push_back(b't');
            }
            _ => {
                result.push_back(b);
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::Env;

    #[test]
    fn test_concat_bytes() {
        let env = Env::default();
        let mut parts: Vec<Bytes> = Vec::new(&env);
        parts.push_back(Bytes::from_slice(&env, b"Hello, "));
        parts.push_back(Bytes::from_slice(&env, b"World!"));

        let result = concat_bytes(&env, &parts);
        assert_eq!(result.len(), 13);
    }

    #[test]
    fn test_concat_bytes_empty() {
        let env = Env::default();
        let parts: Vec<Bytes> = Vec::new(&env);
        let result = concat_bytes(&env, &parts);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_string_to_bytes() {
        let env = Env::default();
        let s = String::from_str(&env, "Hello");
        let bytes = string_to_bytes(&env, &s);
        assert_eq!(bytes.len(), 5);
    }

    #[test]
    fn test_string_to_bytes_empty() {
        let env = Env::default();
        let s = String::from_str(&env, "");
        let bytes = string_to_bytes(&env, &s);
        assert_eq!(bytes.len(), 0);
    }

    #[test]
    fn test_string_to_bytes_256_boundary() {
        let env = Env::default();
        // Exactly 256 bytes - should use first tier
        let content = "a".repeat(256);
        let s = String::from_str(&env, &content);
        let bytes = string_to_bytes(&env, &s);
        assert_eq!(bytes.len(), 256);
    }

    #[test]
    fn test_string_to_bytes_257_uses_1kb_tier() {
        let env = Env::default();
        // 257 bytes - should use second tier (1KB buffer)
        let content = "a".repeat(257);
        let s = String::from_str(&env, &content);
        let bytes = string_to_bytes(&env, &s);
        assert_eq!(bytes.len(), 257);
    }

    #[test]
    fn test_string_to_bytes_1kb_boundary() {
        let env = Env::default();
        let content = "a".repeat(1024);
        let s = String::from_str(&env, &content);
        let bytes = string_to_bytes(&env, &s);
        assert_eq!(bytes.len(), 1024);
    }

    #[test]
    fn test_string_to_bytes_4kb() {
        let env = Env::default();
        let content = "a".repeat(4000);
        let s = String::from_str(&env, &content);
        let bytes = string_to_bytes(&env, &s);
        assert_eq!(bytes.len(), 4000);
    }

    #[test]
    fn test_string_to_bytes_large() {
        let env = Env::default();
        // 10KB string - should use 16KB tier
        let content = "a".repeat(10000);
        let s = String::from_str(&env, &content);
        let bytes = string_to_bytes(&env, &s);
        assert_eq!(bytes.len(), 10000);
    }

    #[test]
    fn test_string_to_bytes_max_size() {
        let env = Env::default();
        // Exactly at the 16KB limit
        let content = "a".repeat(MAX_STRING_SIZE);
        let s = String::from_str(&env, &content);
        let bytes = string_to_bytes(&env, &s);
        assert_eq!(bytes.len(), MAX_STRING_SIZE as u32);
    }

    #[test]
    fn test_u32_to_bytes_zero() {
        let env = Env::default();
        let bytes = u32_to_bytes(&env, 0);
        assert_eq!(bytes.len(), 1);
        assert_eq!(bytes.get(0), Some(b'0'));
    }

    #[test]
    fn test_u32_to_bytes_single_digit() {
        let env = Env::default();
        let bytes = u32_to_bytes(&env, 7);
        assert_eq!(bytes.len(), 1);
        assert_eq!(bytes.get(0), Some(b'7'));
    }

    #[test]
    fn test_u32_to_bytes_multi_digit() {
        let env = Env::default();
        let bytes = u32_to_bytes(&env, 12345);
        assert_eq!(bytes.len(), 5);
        assert_eq!(bytes.get(0), Some(b'1'));
        assert_eq!(bytes.get(1), Some(b'2'));
        assert_eq!(bytes.get(2), Some(b'3'));
        assert_eq!(bytes.get(3), Some(b'4'));
        assert_eq!(bytes.get(4), Some(b'5'));
    }

    #[test]
    fn test_i64_to_bytes_positive() {
        let env = Env::default();
        let bytes = i64_to_bytes(&env, 42);
        assert_eq!(bytes.len(), 2);
        assert_eq!(bytes.get(0), Some(b'4'));
        assert_eq!(bytes.get(1), Some(b'2'));
    }

    #[test]
    fn test_i64_to_bytes_negative() {
        let env = Env::default();
        let bytes = i64_to_bytes(&env, -42);
        assert_eq!(bytes.len(), 3);
        assert_eq!(bytes.get(0), Some(b'-'));
        assert_eq!(bytes.get(1), Some(b'4'));
        assert_eq!(bytes.get(2), Some(b'2'));
    }

    #[test]
    fn test_i64_to_bytes_zero() {
        let env = Env::default();
        let bytes = i64_to_bytes(&env, 0);
        assert_eq!(bytes.len(), 1);
        assert_eq!(bytes.get(0), Some(b'0'));
    }

    #[test]
    fn test_escape_json_bytes_quotes() {
        let env = Env::default();
        let bytes = escape_json_bytes(&env, b"Hello \"World\"");
        // Should be: Hello \"World\"
        assert_eq!(bytes.len(), 15); // 5 + 1 + 1 + 5 + 1 + 1 + 1 = 15
    }

    #[test]
    fn test_escape_json_bytes_backslash() {
        let env = Env::default();
        let bytes = escape_json_bytes(&env, b"path\\to\\file");
        // Should be: path\\to\\file
        assert_eq!(bytes.len(), 14); // 4 + 2 + 2 + 2 + 4 = 14
    }

    #[test]
    fn test_escape_json_bytes_newline() {
        let env = Env::default();
        let bytes = escape_json_bytes(&env, b"line1\nline2");
        // Should be: line1\nline2
        assert_eq!(bytes.len(), 12); // 5 + 2 + 5 = 12
    }
}
