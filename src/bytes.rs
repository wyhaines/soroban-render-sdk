//! Byte manipulation utilities for Soroban Render contracts.
//!
//! These functions provide common operations for working with `Bytes` in a `no_std` environment.

use soroban_sdk::{Bytes, Env, I256, String, U256, Vec};

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

    let mut result = Bytes::new(env);
    if negative {
        result.push_back(b'-');
    }
    for j in (0..i).rev() {
        result.push_back(digits[j]);
    }
    result
}

/// Convert an i32 to its decimal Bytes representation.
///
/// Handles negative numbers by prepending a minus sign.
///
/// # Example
///
/// ```rust,ignore
/// let bytes = i32_to_bytes(&env, -42);
/// // bytes contains "-42"
/// ```
pub fn i32_to_bytes(env: &Env, n: i32) -> Bytes {
    if n == 0 {
        return Bytes::from_slice(env, b"0");
    }

    let negative = n < 0;
    let mut num = if negative { -(n as i64) } else { n as i64 } as u32;
    let mut digits: [u8; 11] = [0; 11]; // i32 max is 10 digits + sign
    let mut i = 0;

    while num > 0 {
        digits[i] = b'0' + (num % 10) as u8;
        num /= 10;
        i += 1;
    }

    let mut result = Bytes::new(env);
    if negative {
        result.push_back(b'-');
    }
    for j in (0..i).rev() {
        result.push_back(digits[j]);
    }
    result
}

/// Convert a u64 to its decimal Bytes representation.
///
/// # Example
///
/// ```rust,ignore
/// let bytes = u64_to_bytes(&env, 42);
/// // bytes contains "42"
/// ```
pub fn u64_to_bytes(env: &Env, n: u64) -> Bytes {
    if n == 0 {
        return Bytes::from_slice(env, b"0");
    }

    let mut num = n;
    let mut digits: [u8; 20] = [0; 20]; // u64 max is 18,446,744,073,709,551,615 (20 digits)
    let mut i = 0;

    while num > 0 {
        digits[i] = b'0' + (num % 10) as u8;
        num /= 10;
        i += 1;
    }

    let mut result = Bytes::new(env);
    for j in (0..i).rev() {
        result.push_back(digits[j]);
    }
    result
}

/// Parse decimal Bytes to a u32.
///
/// Returns `None` if the input is empty, contains non-digit characters,
/// or the value would overflow u32.
///
/// # Example
///
/// ```rust,ignore
/// let bytes = Bytes::from_slice(&env, b"42");
/// assert_eq!(bytes_to_u32(&bytes), Some(42));
/// ```
pub fn bytes_to_u32(bytes: &Bytes) -> Option<u32> {
    if bytes.is_empty() {
        return None;
    }

    let mut result: u32 = 0;
    for i in 0..bytes.len() {
        let b = bytes.get(i)?;
        if !b.is_ascii_digit() {
            return None;
        }
        result = result.checked_mul(10)?;
        result = result.checked_add((b - b'0') as u32)?;
    }
    Some(result)
}

/// Parse decimal Bytes to an i32.
///
/// Returns `None` if the input is empty, contains invalid characters,
/// or the value would overflow i32. Handles optional leading minus sign.
///
/// # Example
///
/// ```rust,ignore
/// let bytes = Bytes::from_slice(&env, b"-42");
/// assert_eq!(bytes_to_i32(&bytes), Some(-42));
/// ```
pub fn bytes_to_i32(bytes: &Bytes) -> Option<i32> {
    if bytes.is_empty() {
        return None;
    }

    let negative = bytes.get(0) == Some(b'-');
    let start = if negative { 1 } else { 0 };

    if start >= bytes.len() {
        return None;
    }

    let mut result: i32 = 0;
    for i in start..bytes.len() {
        let b = bytes.get(i)?;
        if !b.is_ascii_digit() {
            return None;
        }
        result = result.checked_mul(10)?;
        result = result.checked_add((b - b'0') as i32)?;
    }

    if negative {
        Some(-result)
    } else {
        Some(result)
    }
}

/// Parse decimal Bytes to a u64.
///
/// Returns `None` if the input is empty, contains non-digit characters,
/// or the value would overflow u64.
///
/// # Example
///
/// ```rust,ignore
/// let bytes = Bytes::from_slice(&env, b"42");
/// assert_eq!(bytes_to_u64(&bytes), Some(42));
/// ```
pub fn bytes_to_u64(bytes: &Bytes) -> Option<u64> {
    if bytes.is_empty() {
        return None;
    }

    let mut result: u64 = 0;
    for i in 0..bytes.len() {
        let b = bytes.get(i)?;
        if !b.is_ascii_digit() {
            return None;
        }
        result = result.checked_mul(10)?;
        result = result.checked_add((b - b'0') as u64)?;
    }
    Some(result)
}

/// Parse decimal Bytes to an i64.
///
/// Returns `None` if the input is empty, contains invalid characters,
/// or the value would overflow i64. Handles optional leading minus sign.
///
/// # Example
///
/// ```rust,ignore
/// let bytes = Bytes::from_slice(&env, b"-42");
/// assert_eq!(bytes_to_i64(&bytes), Some(-42));
/// ```
pub fn bytes_to_i64(bytes: &Bytes) -> Option<i64> {
    if bytes.is_empty() {
        return None;
    }

    let negative = bytes.get(0) == Some(b'-');
    let start = if negative { 1 } else { 0 };

    if start >= bytes.len() {
        return None;
    }

    let mut result: i64 = 0;
    for i in start..bytes.len() {
        let b = bytes.get(i)?;
        if !b.is_ascii_digit() {
            return None;
        }
        result = result.checked_mul(10)?;
        result = result.checked_add((b - b'0') as i64)?;
    }

    if negative {
        Some(-result)
    } else {
        Some(result)
    }
}

/// Convert a u128 to its decimal Bytes representation.
///
/// # Example
///
/// ```rust,ignore
/// let bytes = u128_to_bytes(&env, 42);
/// // bytes contains "42"
/// ```
pub fn u128_to_bytes(env: &Env, n: u128) -> Bytes {
    if n == 0 {
        return Bytes::from_slice(env, b"0");
    }

    let mut num = n;
    let mut digits: [u8; 39] = [0; 39]; // u128 max is 39 digits
    let mut i = 0;

    while num > 0 {
        digits[i] = b'0' + (num % 10) as u8;
        num /= 10;
        i += 1;
    }

    let mut result = Bytes::new(env);
    for j in (0..i).rev() {
        result.push_back(digits[j]);
    }
    result
}

/// Convert an i128 to its decimal Bytes representation.
///
/// Handles negative numbers by prepending a minus sign.
///
/// # Example
///
/// ```rust,ignore
/// let bytes = i128_to_bytes(&env, -42);
/// // bytes contains "-42"
/// ```
pub fn i128_to_bytes(env: &Env, n: i128) -> Bytes {
    if n == 0 {
        return Bytes::from_slice(env, b"0");
    }

    let negative = n < 0;
    // Handle i128::MIN specially since -i128::MIN would overflow
    let mut num = if n == i128::MIN {
        // i128::MIN = -170141183460469231731687303715884105728
        // We handle it by treating it as unsigned after negation would overflow
        (i128::MAX as u128) + 1
    } else if negative {
        (-n) as u128
    } else {
        n as u128
    };

    let mut digits: [u8; 40] = [0; 40]; // i128 min is 40 chars with sign
    let mut i = 0;

    while num > 0 {
        digits[i] = b'0' + (num % 10) as u8;
        num /= 10;
        i += 1;
    }

    let mut result = Bytes::new(env);
    if negative {
        result.push_back(b'-');
    }
    for j in (0..i).rev() {
        result.push_back(digits[j]);
    }
    result
}

/// Parse decimal Bytes to a u128.
///
/// Returns `None` if the input is empty, contains non-digit characters,
/// or the value would overflow u128.
///
/// # Example
///
/// ```rust,ignore
/// let bytes = Bytes::from_slice(&env, b"42");
/// assert_eq!(bytes_to_u128(&bytes), Some(42));
/// ```
pub fn bytes_to_u128(bytes: &Bytes) -> Option<u128> {
    if bytes.is_empty() {
        return None;
    }

    let mut result: u128 = 0;
    for i in 0..bytes.len() {
        let b = bytes.get(i)?;
        if !b.is_ascii_digit() {
            return None;
        }
        result = result.checked_mul(10)?;
        result = result.checked_add((b - b'0') as u128)?;
    }
    Some(result)
}

/// Parse decimal Bytes to an i128.
///
/// Returns `None` if the input is empty, contains invalid characters,
/// or the value would overflow i128. Handles optional leading minus sign.
///
/// # Example
///
/// ```rust,ignore
/// let bytes = Bytes::from_slice(&env, b"-42");
/// assert_eq!(bytes_to_i128(&bytes), Some(-42));
/// ```
pub fn bytes_to_i128(bytes: &Bytes) -> Option<i128> {
    if bytes.is_empty() {
        return None;
    }

    let negative = bytes.get(0) == Some(b'-');
    let start = if negative { 1 } else { 0 };

    if start >= bytes.len() {
        return None;
    }

    // Parse as u128 first to handle full range including i128::MIN
    let mut result: u128 = 0;
    for i in start..bytes.len() {
        let b = bytes.get(i)?;
        if !b.is_ascii_digit() {
            return None;
        }
        result = result.checked_mul(10)?;
        result = result.checked_add((b - b'0') as u128)?;
    }

    if negative {
        // i128::MIN magnitude is 170141183460469231731687303715884105728
        if result > (i128::MAX as u128) + 1 {
            return None;
        }
        if result == (i128::MAX as u128) + 1 {
            return Some(i128::MIN);
        }
        Some(-(result as i128))
    } else {
        if result > i128::MAX as u128 {
            return None;
        }
        Some(result as i128)
    }
}

// =============================================================================
// Hex Conversion Functions
// =============================================================================

/// Hex character lookup table
const HEX_CHARS: &[u8] = b"0123456789abcdef";

/// Convert a u32 to its hexadecimal Bytes representation with "0x" prefix.
///
/// # Example
///
/// ```rust,ignore
/// let bytes = u32_to_hex(&env, 255);
/// // bytes contains "0xff"
/// ```
pub fn u32_to_hex(env: &Env, n: u32) -> Bytes {
    if n == 0 {
        return Bytes::from_slice(env, b"0x0");
    }

    let mut num = n;
    let mut digits: [u8; 8] = [0; 8]; // u32 max is 8 hex digits
    let mut i = 0;

    while num > 0 {
        digits[i] = HEX_CHARS[(num & 0xF) as usize];
        num >>= 4;
        i += 1;
    }

    let mut result = Bytes::from_slice(env, b"0x");
    for j in (0..i).rev() {
        result.push_back(digits[j]);
    }
    result
}

/// Convert an i32 to its hexadecimal Bytes representation with "0x" prefix.
///
/// Negative numbers are prefixed with "-0x".
///
/// # Example
///
/// ```rust,ignore
/// let bytes = i32_to_hex(&env, -255);
/// // bytes contains "-0xff"
/// ```
pub fn i32_to_hex(env: &Env, n: i32) -> Bytes {
    if n == 0 {
        return Bytes::from_slice(env, b"0x0");
    }

    let negative = n < 0;
    let mut num = if negative { -(n as i64) } else { n as i64 } as u32;
    let mut digits: [u8; 8] = [0; 8];
    let mut i = 0;

    while num > 0 {
        digits[i] = HEX_CHARS[(num & 0xF) as usize];
        num >>= 4;
        i += 1;
    }

    let mut result = if negative {
        Bytes::from_slice(env, b"-0x")
    } else {
        Bytes::from_slice(env, b"0x")
    };
    for j in (0..i).rev() {
        result.push_back(digits[j]);
    }
    result
}

/// Convert a u64 to its hexadecimal Bytes representation with "0x" prefix.
///
/// # Example
///
/// ```rust,ignore
/// let bytes = u64_to_hex(&env, 255);
/// // bytes contains "0xff"
/// ```
pub fn u64_to_hex(env: &Env, n: u64) -> Bytes {
    if n == 0 {
        return Bytes::from_slice(env, b"0x0");
    }

    let mut num = n;
    let mut digits: [u8; 16] = [0; 16]; // u64 max is 16 hex digits
    let mut i = 0;

    while num > 0 {
        digits[i] = HEX_CHARS[(num & 0xF) as usize];
        num >>= 4;
        i += 1;
    }

    let mut result = Bytes::from_slice(env, b"0x");
    for j in (0..i).rev() {
        result.push_back(digits[j]);
    }
    result
}

/// Convert an i64 to its hexadecimal Bytes representation with "0x" prefix.
///
/// Negative numbers are prefixed with "-0x".
///
/// # Example
///
/// ```rust,ignore
/// let bytes = i64_to_hex(&env, -255);
/// // bytes contains "-0xff"
/// ```
pub fn i64_to_hex(env: &Env, n: i64) -> Bytes {
    if n == 0 {
        return Bytes::from_slice(env, b"0x0");
    }

    let negative = n < 0;
    let mut num = if negative { -(n as i128) } else { n as i128 } as u64;
    let mut digits: [u8; 16] = [0; 16];
    let mut i = 0;

    while num > 0 {
        digits[i] = HEX_CHARS[(num & 0xF) as usize];
        num >>= 4;
        i += 1;
    }

    let mut result = if negative {
        Bytes::from_slice(env, b"-0x")
    } else {
        Bytes::from_slice(env, b"0x")
    };
    for j in (0..i).rev() {
        result.push_back(digits[j]);
    }
    result
}

/// Convert a u128 to its hexadecimal Bytes representation with "0x" prefix.
///
/// # Example
///
/// ```rust,ignore
/// let bytes = u128_to_hex(&env, 255);
/// // bytes contains "0xff"
/// ```
pub fn u128_to_hex(env: &Env, n: u128) -> Bytes {
    if n == 0 {
        return Bytes::from_slice(env, b"0x0");
    }

    let mut num = n;
    let mut digits: [u8; 32] = [0; 32]; // u128 max is 32 hex digits
    let mut i = 0;

    while num > 0 {
        digits[i] = HEX_CHARS[(num & 0xF) as usize];
        num >>= 4;
        i += 1;
    }

    let mut result = Bytes::from_slice(env, b"0x");
    for j in (0..i).rev() {
        result.push_back(digits[j]);
    }
    result
}

/// Convert an i128 to its hexadecimal Bytes representation with "0x" prefix.
///
/// Negative numbers are prefixed with "-0x".
///
/// # Example
///
/// ```rust,ignore
/// let bytes = i128_to_hex(&env, -255);
/// // bytes contains "-0xff"
/// ```
pub fn i128_to_hex(env: &Env, n: i128) -> Bytes {
    if n == 0 {
        return Bytes::from_slice(env, b"0x0");
    }

    let negative = n < 0;
    // Handle i128::MIN specially
    let mut num = if n == i128::MIN {
        (i128::MAX as u128) + 1
    } else if negative {
        (-n) as u128
    } else {
        n as u128
    };

    let mut digits: [u8; 32] = [0; 32];
    let mut i = 0;

    while num > 0 {
        digits[i] = HEX_CHARS[(num & 0xF) as usize];
        num >>= 4;
        i += 1;
    }

    let mut result = if negative {
        Bytes::from_slice(env, b"-0x")
    } else {
        Bytes::from_slice(env, b"0x")
    };
    for j in (0..i).rev() {
        result.push_back(digits[j]);
    }
    result
}

/// Parse hex Bytes to a u32.
///
/// Accepts optional "0x" or "0X" prefix. Case-insensitive.
/// Returns `None` if the input is empty, contains invalid characters,
/// or the value would overflow u32.
///
/// # Example
///
/// ```rust,ignore
/// let bytes = Bytes::from_slice(&env, b"0xff");
/// assert_eq!(hex_to_u32(&bytes), Some(255));
/// ```
pub fn hex_to_u32(bytes: &Bytes) -> Option<u32> {
    let len = bytes.len();
    if len == 0 {
        return None;
    }

    let start = if len >= 2
        && bytes.get(0) == Some(b'0')
        && (bytes.get(1) == Some(b'x') || bytes.get(1) == Some(b'X'))
    {
        2
    } else {
        0
    };

    if start >= len {
        return None;
    }

    let mut result: u32 = 0;
    for i in start..len {
        let b = bytes.get(i)?;
        let digit = match b {
            b'0'..=b'9' => b - b'0',
            b'a'..=b'f' => b - b'a' + 10,
            b'A'..=b'F' => b - b'A' + 10,
            _ => return None,
        };
        result = result.checked_mul(16)?;
        result = result.checked_add(digit as u32)?;
    }
    Some(result)
}

/// Parse hex Bytes to an i32.
///
/// Accepts optional "0x" or "0X" prefix and optional leading minus sign.
/// Case-insensitive. Returns `None` if invalid or would overflow.
///
/// # Example
///
/// ```rust,ignore
/// let bytes = Bytes::from_slice(&env, b"-0xff");
/// assert_eq!(hex_to_i32(&bytes), Some(-255));
/// ```
pub fn hex_to_i32(bytes: &Bytes) -> Option<i32> {
    let len = bytes.len();
    if len == 0 {
        return None;
    }

    let negative = bytes.get(0) == Some(b'-');
    let after_sign = if negative { 1 } else { 0 };

    if after_sign >= len {
        return None;
    }

    let start = if len >= after_sign + 2
        && bytes.get(after_sign) == Some(b'0')
        && (bytes.get(after_sign + 1) == Some(b'x') || bytes.get(after_sign + 1) == Some(b'X'))
    {
        after_sign + 2
    } else {
        after_sign
    };

    if start >= len {
        return None;
    }

    let mut result: u32 = 0;
    for i in start..len {
        let b = bytes.get(i)?;
        let digit = match b {
            b'0'..=b'9' => b - b'0',
            b'a'..=b'f' => b - b'a' + 10,
            b'A'..=b'F' => b - b'A' + 10,
            _ => return None,
        };
        result = result.checked_mul(16)?;
        result = result.checked_add(digit as u32)?;
    }

    if negative {
        if result > (i32::MAX as u32) + 1 {
            return None;
        }
        if result == (i32::MAX as u32) + 1 {
            return Some(i32::MIN);
        }
        Some(-(result as i32))
    } else {
        if result > i32::MAX as u32 {
            return None;
        }
        Some(result as i32)
    }
}

/// Parse hex Bytes to a u64.
///
/// Accepts optional "0x" or "0X" prefix. Case-insensitive.
/// Returns `None` if the input is empty, contains invalid characters,
/// or the value would overflow u64.
///
/// # Example
///
/// ```rust,ignore
/// let bytes = Bytes::from_slice(&env, b"0xff");
/// assert_eq!(hex_to_u64(&bytes), Some(255));
/// ```
pub fn hex_to_u64(bytes: &Bytes) -> Option<u64> {
    let len = bytes.len();
    if len == 0 {
        return None;
    }

    let start = if len >= 2
        && bytes.get(0) == Some(b'0')
        && (bytes.get(1) == Some(b'x') || bytes.get(1) == Some(b'X'))
    {
        2
    } else {
        0
    };

    if start >= len {
        return None;
    }

    let mut result: u64 = 0;
    for i in start..len {
        let b = bytes.get(i)?;
        let digit = match b {
            b'0'..=b'9' => b - b'0',
            b'a'..=b'f' => b - b'a' + 10,
            b'A'..=b'F' => b - b'A' + 10,
            _ => return None,
        };
        result = result.checked_mul(16)?;
        result = result.checked_add(digit as u64)?;
    }
    Some(result)
}

/// Parse hex Bytes to an i64.
///
/// Accepts optional "0x" or "0X" prefix and optional leading minus sign.
/// Case-insensitive. Returns `None` if invalid or would overflow.
///
/// # Example
///
/// ```rust,ignore
/// let bytes = Bytes::from_slice(&env, b"-0xff");
/// assert_eq!(hex_to_i64(&bytes), Some(-255));
/// ```
pub fn hex_to_i64(bytes: &Bytes) -> Option<i64> {
    let len = bytes.len();
    if len == 0 {
        return None;
    }

    let negative = bytes.get(0) == Some(b'-');
    let after_sign = if negative { 1 } else { 0 };

    if after_sign >= len {
        return None;
    }

    let start = if len >= after_sign + 2
        && bytes.get(after_sign) == Some(b'0')
        && (bytes.get(after_sign + 1) == Some(b'x') || bytes.get(after_sign + 1) == Some(b'X'))
    {
        after_sign + 2
    } else {
        after_sign
    };

    if start >= len {
        return None;
    }

    let mut result: u64 = 0;
    for i in start..len {
        let b = bytes.get(i)?;
        let digit = match b {
            b'0'..=b'9' => b - b'0',
            b'a'..=b'f' => b - b'a' + 10,
            b'A'..=b'F' => b - b'A' + 10,
            _ => return None,
        };
        result = result.checked_mul(16)?;
        result = result.checked_add(digit as u64)?;
    }

    if negative {
        if result > (i64::MAX as u64) + 1 {
            return None;
        }
        if result == (i64::MAX as u64) + 1 {
            return Some(i64::MIN);
        }
        Some(-(result as i64))
    } else {
        if result > i64::MAX as u64 {
            return None;
        }
        Some(result as i64)
    }
}

/// Parse hex Bytes to a u128.
///
/// Accepts optional "0x" or "0X" prefix. Case-insensitive.
/// Returns `None` if the input is empty, contains invalid characters,
/// or the value would overflow u128.
///
/// # Example
///
/// ```rust,ignore
/// let bytes = Bytes::from_slice(&env, b"0xff");
/// assert_eq!(hex_to_u128(&bytes), Some(255));
/// ```
pub fn hex_to_u128(bytes: &Bytes) -> Option<u128> {
    let len = bytes.len();
    if len == 0 {
        return None;
    }

    let start = if len >= 2
        && bytes.get(0) == Some(b'0')
        && (bytes.get(1) == Some(b'x') || bytes.get(1) == Some(b'X'))
    {
        2
    } else {
        0
    };

    if start >= len {
        return None;
    }

    let mut result: u128 = 0;
    for i in start..len {
        let b = bytes.get(i)?;
        let digit = match b {
            b'0'..=b'9' => b - b'0',
            b'a'..=b'f' => b - b'a' + 10,
            b'A'..=b'F' => b - b'A' + 10,
            _ => return None,
        };
        result = result.checked_mul(16)?;
        result = result.checked_add(digit as u128)?;
    }
    Some(result)
}

/// Parse hex Bytes to an i128.
///
/// Accepts optional "0x" or "0X" prefix and optional leading minus sign.
/// Case-insensitive. Returns `None` if invalid or would overflow.
///
/// # Example
///
/// ```rust,ignore
/// let bytes = Bytes::from_slice(&env, b"-0xff");
/// assert_eq!(hex_to_i128(&bytes), Some(-255));
/// ```
pub fn hex_to_i128(bytes: &Bytes) -> Option<i128> {
    let len = bytes.len();
    if len == 0 {
        return None;
    }

    let negative = bytes.get(0) == Some(b'-');
    let after_sign = if negative { 1 } else { 0 };

    if after_sign >= len {
        return None;
    }

    let start = if len >= after_sign + 2
        && bytes.get(after_sign) == Some(b'0')
        && (bytes.get(after_sign + 1) == Some(b'x') || bytes.get(after_sign + 1) == Some(b'X'))
    {
        after_sign + 2
    } else {
        after_sign
    };

    if start >= len {
        return None;
    }

    let mut result: u128 = 0;
    for i in start..len {
        let b = bytes.get(i)?;
        let digit = match b {
            b'0'..=b'9' => b - b'0',
            b'a'..=b'f' => b - b'a' + 10,
            b'A'..=b'F' => b - b'A' + 10,
            _ => return None,
        };
        result = result.checked_mul(16)?;
        result = result.checked_add(digit as u128)?;
    }

    if negative {
        if result > (i128::MAX as u128) + 1 {
            return None;
        }
        if result == (i128::MAX as u128) + 1 {
            return Some(i128::MIN);
        }
        Some(-(result as i128))
    } else {
        if result > i128::MAX as u128 {
            return None;
        }
        Some(result as i128)
    }
}

// =============================================================================
// U256/I256 Conversion Functions
// =============================================================================

/// Convert a U256 to its decimal Bytes representation.
///
/// Note: This function performs multi-precision division which can be
/// computationally expensive for large numbers.
///
/// # Example
///
/// ```rust,ignore
/// let n = U256::from_u128(&env, 12345);
/// let bytes = u256_to_bytes(&env, &n);
/// // bytes contains "12345"
/// ```
pub fn u256_to_bytes(env: &Env, n: &U256) -> Bytes {
    let be_bytes = n.to_be_bytes();

    let mut all_zero = true;
    for i in 0..32u32 {
        if be_bytes.get(i) != Some(0) {
            all_zero = false;
            break;
        }
    }
    if all_zero {
        return Bytes::from_slice(env, b"0");
    }

    let mut num = [0u8; 32];
    for (i, item) in num.iter_mut().enumerate() {
        *item = be_bytes.get(i as u32).unwrap_or(0);
    }

    // U256 max is ~78 decimal digits
    let mut digits = [0u8; 78];
    let mut digit_count = 0;

    while !is_zero_256(&num) {
        let remainder = div_by_10_256(&mut num);
        digits[digit_count] = b'0' + remainder;
        digit_count += 1;
    }

    let mut result = Bytes::new(env);
    for i in (0..digit_count).rev() {
        result.push_back(digits[i]);
    }
    result
}

/// Helper: check if a 256-bit big-endian number is zero
fn is_zero_256(num: &[u8; 32]) -> bool {
    for &b in num.iter() {
        if b != 0 {
            return false;
        }
    }
    true
}

/// Helper: divide a 256-bit big-endian number by 10, returning remainder
fn div_by_10_256(num: &mut [u8; 32]) -> u8 {
    let mut carry: u16 = 0;
    for item in num.iter_mut() {
        let current = (carry << 8) | (*item as u16);
        *item = (current / 10) as u8;
        carry = current % 10;
    }
    carry as u8
}

/// Convert an I256 to its decimal Bytes representation.
///
/// Handles negative numbers by prepending a minus sign.
/// Note: This function performs multi-precision division which can be
/// computationally expensive for large numbers.
///
/// # Example
///
/// ```rust,ignore
/// let n = I256::from_i128(&env, -12345);
/// let bytes = i256_to_bytes(&env, &n);
/// // bytes contains "-12345"
/// ```
pub fn i256_to_bytes(env: &Env, n: &I256) -> Bytes {
    let be_bytes = n.to_be_bytes();
    let is_negative = be_bytes.get(0).unwrap_or(0) & 0x80 != 0;

    if is_negative {
        // Two's complement negation to get absolute value: invert all bits and add 1
        let mut abs_num = [0u8; 32];
        for (i, item) in abs_num.iter_mut().enumerate() {
            *item = !be_bytes.get(i as u32).unwrap_or(0);
        }
        let mut carry = 1u16;
        for i in (0..32).rev() {
            let sum = (abs_num[i] as u16) + carry;
            abs_num[i] = sum as u8;
            carry = sum >> 8;
        }

        if is_zero_256(&abs_num) {
            return Bytes::from_slice(env, b"0");
        }

        let mut digits = [0u8; 78];
        let mut digit_count = 0;

        while !is_zero_256(&abs_num) {
            let remainder = div_by_10_256(&mut abs_num);
            digits[digit_count] = b'0' + remainder;
            digit_count += 1;
        }

        let mut result = Bytes::from_slice(env, b"-");
        for i in (0..digit_count).rev() {
            result.push_back(digits[i]);
        }
        result
    } else {
        let mut num = [0u8; 32];
        for (i, item) in num.iter_mut().enumerate() {
            *item = be_bytes.get(i as u32).unwrap_or(0);
        }

        if is_zero_256(&num) {
            return Bytes::from_slice(env, b"0");
        }

        let mut digits = [0u8; 78];
        let mut digit_count = 0;

        while !is_zero_256(&num) {
            let remainder = div_by_10_256(&mut num);
            digits[digit_count] = b'0' + remainder;
            digit_count += 1;
        }

        let mut result = Bytes::new(env);
        for i in (0..digit_count).rev() {
            result.push_back(digits[i]);
        }
        result
    }
}

/// Convert a U256 to its hexadecimal Bytes representation with "0x" prefix.
///
/// # Example
///
/// ```rust,ignore
/// let n = U256::from_u128(&env, 255);
/// let bytes = u256_to_hex(&env, &n);
/// // bytes contains "0xff"
/// ```
pub fn u256_to_hex(env: &Env, n: &U256) -> Bytes {
    let be_bytes = n.to_be_bytes();

    let mut all_zero = true;
    for i in 0..32u32 {
        if be_bytes.get(i) != Some(0) {
            all_zero = false;
            break;
        }
    }
    if all_zero {
        return Bytes::from_slice(env, b"0x0");
    }

    let mut result = Bytes::from_slice(env, b"0x");
    let mut started = false;

    for i in 0..32u32 {
        let byte = be_bytes.get(i).unwrap_or(0);
        if byte != 0 || started {
            if !started {
                // First non-zero byte: skip leading zero nibble if present
                let high = byte >> 4;
                if high != 0 {
                    result.push_back(HEX_CHARS[high as usize]);
                }
                result.push_back(HEX_CHARS[(byte & 0xF) as usize]);
            } else {
                result.push_back(HEX_CHARS[(byte >> 4) as usize]);
                result.push_back(HEX_CHARS[(byte & 0xF) as usize]);
            }
            started = true;
        }
    }

    result
}

/// Convert an I256 to its hexadecimal Bytes representation with "0x" prefix.
///
/// Negative numbers are prefixed with "-0x".
///
/// # Example
///
/// ```rust,ignore
/// let n = I256::from_i128(&env, -255);
/// let bytes = i256_to_hex(&env, &n);
/// // bytes contains "-0xff"
/// ```
pub fn i256_to_hex(env: &Env, n: &I256) -> Bytes {
    let be_bytes = n.to_be_bytes();
    let is_negative = be_bytes.get(0).unwrap_or(0) & 0x80 != 0;

    if is_negative {
        // Two's complement negation to get absolute value
        let mut abs_num = [0u8; 32];
        for (i, item) in abs_num.iter_mut().enumerate() {
            *item = !be_bytes.get(i as u32).unwrap_or(0);
        }
        let mut carry = 1u16;
        for i in (0..32).rev() {
            let sum = (abs_num[i] as u16) + carry;
            abs_num[i] = sum as u8;
            carry = sum >> 8;
        }

        let mut all_zero = true;
        for &b in abs_num.iter() {
            if b != 0 {
                all_zero = false;
                break;
            }
        }
        if all_zero {
            return Bytes::from_slice(env, b"0x0");
        }

        let mut result = Bytes::from_slice(env, b"-0x");
        let mut started = false;

        for &byte in abs_num.iter() {
            if byte != 0 || started {
                if !started {
                    let high = byte >> 4;
                    if high != 0 {
                        result.push_back(HEX_CHARS[high as usize]);
                    }
                    result.push_back(HEX_CHARS[(byte & 0xF) as usize]);
                } else {
                    result.push_back(HEX_CHARS[(byte >> 4) as usize]);
                    result.push_back(HEX_CHARS[(byte & 0xF) as usize]);
                }
                started = true;
            }
        }

        result
    } else {
        let mut all_zero = true;
        for i in 0..32u32 {
            if be_bytes.get(i) != Some(0) {
                all_zero = false;
                break;
            }
        }
        if all_zero {
            return Bytes::from_slice(env, b"0x0");
        }

        let mut result = Bytes::from_slice(env, b"0x");
        let mut started = false;

        for i in 0..32u32 {
            let byte = be_bytes.get(i).unwrap_or(0);
            if byte != 0 || started {
                if !started {
                    let high = byte >> 4;
                    if high != 0 {
                        result.push_back(HEX_CHARS[high as usize]);
                    }
                    result.push_back(HEX_CHARS[(byte & 0xF) as usize]);
                } else {
                    result.push_back(HEX_CHARS[(byte >> 4) as usize]);
                    result.push_back(HEX_CHARS[(byte & 0xF) as usize]);
                }
                started = true;
            }
        }

        result
    }
}

/// Parse decimal Bytes to a U256.
///
/// Returns `None` if the input is empty, contains non-digit characters,
/// or the value would overflow U256.
///
/// # Example
///
/// ```rust,ignore
/// let bytes = Bytes::from_slice(&env, b"12345");
/// let n = bytes_to_u256(&env, &bytes);
/// ```
pub fn bytes_to_u256(env: &Env, bytes: &Bytes) -> Option<U256> {
    if bytes.is_empty() {
        return None;
    }

    let mut num = [0u8; 32];

    for i in 0..bytes.len() {
        let b = bytes.get(i)?;
        if !b.is_ascii_digit() {
            return None;
        }
        let digit = b - b'0';

        if !mul_by_10_and_add_256(&mut num, digit) {
            return None;
        }
    }

    let num_bytes = Bytes::from_slice(env, &num);
    Some(U256::from_be_bytes(env, &num_bytes))
}

/// Helper: multiply a 256-bit big-endian number by 10 and add a digit
/// Returns false on overflow
fn mul_by_10_and_add_256(num: &mut [u8; 32], digit: u8) -> bool {
    let mut carry: u16 = digit as u16;

    for i in (0..32).rev() {
        let product = (num[i] as u16) * 10 + carry;
        num[i] = product as u8;
        carry = product >> 8;
    }

    carry == 0
}

/// Parse decimal Bytes to an I256.
///
/// Returns `None` if the input is empty, contains invalid characters,
/// or the value would overflow I256. Handles optional leading minus sign.
///
/// # Example
///
/// ```rust,ignore
/// let bytes = Bytes::from_slice(&env, b"-12345");
/// let n = bytes_to_i256(&env, &bytes);
/// ```
pub fn bytes_to_i256(env: &Env, bytes: &Bytes) -> Option<I256> {
    if bytes.is_empty() {
        return None;
    }

    let negative = bytes.get(0) == Some(b'-');
    let start = if negative { 1 } else { 0 };

    if start >= bytes.len() {
        return None;
    }

    let mut num = [0u8; 32];

    for i in start..bytes.len() {
        let b = bytes.get(i)?;
        if !b.is_ascii_digit() {
            return None;
        }
        let digit = b - b'0';

        if !mul_by_10_and_add_256(&mut num, digit) {
            return None;
        }
    }

    // I256 positive max has MSB=0; negative can represent magnitude up to 2^255
    if !negative && (num[0] & 0x80) != 0 {
        return None;
    }

    if negative {
        // Check for I256::MIN: magnitude 2^255 represented as 0x80 followed by zeros
        if (num[0] & 0x80) != 0 {
            let mut is_min = num[0] == 0x80;
            if is_min {
                for item in num.iter().skip(1) {
                    if *item != 0 {
                        is_min = false;
                        break;
                    }
                }
            }
            if !is_min {
                return None;
            }
            let num_bytes = Bytes::from_slice(env, &num);
            return Some(I256::from_be_bytes(env, &num_bytes));
        }

        // Two's complement negation
        for item in num.iter_mut() {
            *item = !*item;
        }
        let mut carry = 1u16;
        for i in (0..32).rev() {
            let sum = (num[i] as u16) + carry;
            num[i] = sum as u8;
            carry = sum >> 8;
        }
    }

    let num_bytes = Bytes::from_slice(env, &num);
    Some(I256::from_be_bytes(env, &num_bytes))
}

/// Parse hex Bytes to a U256.
///
/// Accepts optional "0x" or "0X" prefix. Case-insensitive.
/// Returns `None` if the input is empty, contains invalid characters,
/// or the value would overflow U256.
///
/// # Example
///
/// ```rust,ignore
/// let bytes = Bytes::from_slice(&env, b"0xff");
/// let n = hex_to_u256(&env, &bytes);
/// ```
pub fn hex_to_u256(env: &Env, bytes: &Bytes) -> Option<U256> {
    let len = bytes.len();
    if len == 0 {
        return None;
    }

    let start = if len >= 2
        && bytes.get(0) == Some(b'0')
        && (bytes.get(1) == Some(b'x') || bytes.get(1) == Some(b'X'))
    {
        2
    } else {
        0
    };

    if start >= len {
        return None;
    }

    let hex_len = len - start;
    if hex_len > 64 {
        return None;
    }

    let mut num = [0u8; 32];

    for i in start..len {
        let b = bytes.get(i)?;
        let digit = match b {
            b'0'..=b'9' => b - b'0',
            b'a'..=b'f' => b - b'a' + 10,
            b'A'..=b'F' => b - b'A' + 10,
            _ => return None,
        };

        // Shift left 4 bits and add digit
        let mut carry = digit;
        for j in (0..32).rev() {
            let new_val = ((num[j] as u16) << 4) | (carry as u16);
            num[j] = new_val as u8;
            carry = (new_val >> 8) as u8;
        }

        if carry != 0 {
            return None;
        }
    }

    let num_bytes = Bytes::from_slice(env, &num);
    Some(U256::from_be_bytes(env, &num_bytes))
}

/// Parse hex Bytes to an I256.
///
/// Accepts optional "0x" or "0X" prefix and optional leading minus sign.
/// Case-insensitive. Returns `None` if invalid or would overflow.
///
/// # Example
///
/// ```rust,ignore
/// let bytes = Bytes::from_slice(&env, b"-0xff");
/// let n = hex_to_i256(&env, &bytes);
/// ```
pub fn hex_to_i256(env: &Env, bytes: &Bytes) -> Option<I256> {
    let len = bytes.len();
    if len == 0 {
        return None;
    }

    let negative = bytes.get(0) == Some(b'-');
    let after_sign = if negative { 1 } else { 0 };

    if after_sign >= len {
        return None;
    }

    let start = if len >= after_sign + 2
        && bytes.get(after_sign) == Some(b'0')
        && (bytes.get(after_sign + 1) == Some(b'x') || bytes.get(after_sign + 1) == Some(b'X'))
    {
        after_sign + 2
    } else {
        after_sign
    };

    if start >= len {
        return None;
    }

    let hex_len = len - start;
    if hex_len > 64 {
        return None;
    }

    let mut num = [0u8; 32];

    for i in start..len {
        let b = bytes.get(i)?;
        let digit = match b {
            b'0'..=b'9' => b - b'0',
            b'a'..=b'f' => b - b'a' + 10,
            b'A'..=b'F' => b - b'A' + 10,
            _ => return None,
        };

        // Shift left 4 bits and add digit
        let mut carry = digit;
        for j in (0..32).rev() {
            let new_val = ((num[j] as u16) << 4) | (carry as u16);
            num[j] = new_val as u8;
            carry = (new_val >> 8) as u8;
        }

        if carry != 0 {
            return None;
        }
    }

    if !negative && (num[0] & 0x80) != 0 {
        return None;
    }

    if negative {
        // Check for I256::MIN: magnitude 2^255 represented as 0x80 followed by zeros
        if (num[0] & 0x80) != 0 {
            let mut is_min = num[0] == 0x80;
            if is_min {
                for item in num.iter().skip(1) {
                    if *item != 0 {
                        is_min = false;
                        break;
                    }
                }
            }
            if !is_min {
                return None;
            }
            let num_bytes = Bytes::from_slice(env, &num);
            return Some(I256::from_be_bytes(env, &num_bytes));
        }

        // Two's complement negation
        for item in num.iter_mut() {
            *item = !*item;
        }
        let mut carry = 1u16;
        for i in (0..32).rev() {
            let sum = (num[i] as u16) + carry;
            num[i] = sum as u8;
            carry = sum >> 8;
        }
    }

    let num_bytes = Bytes::from_slice(env, &num);
    Some(I256::from_be_bytes(env, &num_bytes))
}

// =============================================================================
// String Convenience Wrappers
// =============================================================================

/// Parse a soroban_sdk::String to a u32.
///
/// This is a convenience wrapper around `string_to_bytes` and `bytes_to_u32`.
///
/// # Example
///
/// ```rust,ignore
/// let s = String::from_str(&env, "12345");
/// assert_eq!(string_to_u32(&env, &s), Some(12345));
/// ```
pub fn string_to_u32(env: &Env, s: &String) -> Option<u32> {
    let bytes = string_to_bytes(env, s);
    bytes_to_u32(&bytes)
}

/// Parse a soroban_sdk::String to an i32.
///
/// This is a convenience wrapper around `string_to_bytes` and `bytes_to_i32`.
///
/// # Example
///
/// ```rust,ignore
/// let s = String::from_str(&env, "-12345");
/// assert_eq!(string_to_i32(&env, &s), Some(-12345));
/// ```
pub fn string_to_i32(env: &Env, s: &String) -> Option<i32> {
    let bytes = string_to_bytes(env, s);
    bytes_to_i32(&bytes)
}

/// Parse a soroban_sdk::String to a u64.
///
/// This is a convenience wrapper around `string_to_bytes` and `bytes_to_u64`.
///
/// # Example
///
/// ```rust,ignore
/// let s = String::from_str(&env, "12345");
/// assert_eq!(string_to_u64(&env, &s), Some(12345));
/// ```
pub fn string_to_u64(env: &Env, s: &String) -> Option<u64> {
    let bytes = string_to_bytes(env, s);
    bytes_to_u64(&bytes)
}

/// Parse a soroban_sdk::String to an i64.
///
/// This is a convenience wrapper around `string_to_bytes` and `bytes_to_i64`.
///
/// # Example
///
/// ```rust,ignore
/// let s = String::from_str(&env, "-12345");
/// assert_eq!(string_to_i64(&env, &s), Some(-12345));
/// ```
pub fn string_to_i64(env: &Env, s: &String) -> Option<i64> {
    let bytes = string_to_bytes(env, s);
    bytes_to_i64(&bytes)
}

/// Parse a soroban_sdk::String to a u128.
///
/// This is a convenience wrapper around `string_to_bytes` and `bytes_to_u128`.
///
/// # Example
///
/// ```rust,ignore
/// let s = String::from_str(&env, "12345");
/// assert_eq!(string_to_u128(&env, &s), Some(12345));
/// ```
pub fn string_to_u128(env: &Env, s: &String) -> Option<u128> {
    let bytes = string_to_bytes(env, s);
    bytes_to_u128(&bytes)
}

/// Parse a soroban_sdk::String to an i128.
///
/// This is a convenience wrapper around `string_to_bytes` and `bytes_to_i128`.
///
/// # Example
///
/// ```rust,ignore
/// let s = String::from_str(&env, "-12345");
/// assert_eq!(string_to_i128(&env, &s), Some(-12345));
/// ```
pub fn string_to_i128(env: &Env, s: &String) -> Option<i128> {
    let bytes = string_to_bytes(env, s);
    bytes_to_i128(&bytes)
}

/// Parse a soroban_sdk::String to a U256.
///
/// This is a convenience wrapper around `string_to_bytes` and `bytes_to_u256`.
///
/// # Example
///
/// ```rust,ignore
/// let s = String::from_str(&env, "12345");
/// let n = string_to_u256(&env, &s);
/// ```
pub fn string_to_u256(env: &Env, s: &String) -> Option<U256> {
    let bytes = string_to_bytes(env, s);
    bytes_to_u256(env, &bytes)
}

/// Parse a soroban_sdk::String to an I256.
///
/// This is a convenience wrapper around `string_to_bytes` and `bytes_to_i256`.
///
/// # Example
///
/// ```rust,ignore
/// let s = String::from_str(&env, "-12345");
/// let n = string_to_i256(&env, &s);
/// ```
pub fn string_to_i256(env: &Env, s: &String) -> Option<I256> {
    let bytes = string_to_bytes(env, s);
    bytes_to_i256(env, &bytes)
}

// =============================================================================
// &str Convenience Wrappers
// =============================================================================

/// Parse a &str to a u32.
///
/// Converts the string slice directly to Bytes and parses.
/// More ergonomic than `string_to_u32` when working with string literals.
///
/// # Example
///
/// ```rust,ignore
/// let n = str_to_u32(&env, "12345");
/// // n is Some(12345)
/// ```
pub fn str_to_u32(env: &Env, s: &str) -> Option<u32> {
    let bytes = Bytes::from_slice(env, s.as_bytes());
    bytes_to_u32(&bytes)
}

/// Parse a &str to an i32.
///
/// Converts the string slice directly to Bytes and parses.
/// Handles optional leading minus sign.
///
/// # Example
///
/// ```rust,ignore
/// let n = str_to_i32(&env, "-12345");
/// // n is Some(-12345)
/// ```
pub fn str_to_i32(env: &Env, s: &str) -> Option<i32> {
    let bytes = Bytes::from_slice(env, s.as_bytes());
    bytes_to_i32(&bytes)
}

/// Parse a &str to a u64.
///
/// Converts the string slice directly to Bytes and parses.
///
/// # Example
///
/// ```rust,ignore
/// let n = str_to_u64(&env, "12345");
/// // n is Some(12345)
/// ```
pub fn str_to_u64(env: &Env, s: &str) -> Option<u64> {
    let bytes = Bytes::from_slice(env, s.as_bytes());
    bytes_to_u64(&bytes)
}

/// Parse a &str to an i64.
///
/// Converts the string slice directly to Bytes and parses.
/// Handles optional leading minus sign.
///
/// # Example
///
/// ```rust,ignore
/// let n = str_to_i64(&env, "-12345");
/// // n is Some(-12345)
/// ```
pub fn str_to_i64(env: &Env, s: &str) -> Option<i64> {
    let bytes = Bytes::from_slice(env, s.as_bytes());
    bytes_to_i64(&bytes)
}

/// Parse a &str to a u128.
///
/// Converts the string slice directly to Bytes and parses.
///
/// # Example
///
/// ```rust,ignore
/// let n = str_to_u128(&env, "12345");
/// // n is Some(12345)
/// ```
pub fn str_to_u128(env: &Env, s: &str) -> Option<u128> {
    let bytes = Bytes::from_slice(env, s.as_bytes());
    bytes_to_u128(&bytes)
}

/// Parse a &str to an i128.
///
/// Converts the string slice directly to Bytes and parses.
/// Handles optional leading minus sign.
///
/// # Example
///
/// ```rust,ignore
/// let n = str_to_i128(&env, "-12345");
/// // n is Some(-12345)
/// ```
pub fn str_to_i128(env: &Env, s: &str) -> Option<i128> {
    let bytes = Bytes::from_slice(env, s.as_bytes());
    bytes_to_i128(&bytes)
}

/// Parse a &str to a U256.
///
/// Converts the string slice directly to Bytes and parses.
/// More ergonomic than creating a soroban_sdk::String first.
///
/// # Example
///
/// ```rust,ignore
/// let n = str_to_u256(&env, "12345");
/// // n is Some(U256)
/// ```
pub fn str_to_u256(env: &Env, s: &str) -> Option<U256> {
    let bytes = Bytes::from_slice(env, s.as_bytes());
    bytes_to_u256(env, &bytes)
}

/// Parse a &str to an I256.
///
/// Converts the string slice directly to Bytes and parses.
/// Handles optional leading minus sign.
///
/// # Example
///
/// ```rust,ignore
/// let n = str_to_i256(&env, "-12345");
/// // n is Some(I256)
/// ```
pub fn str_to_i256(env: &Env, s: &str) -> Option<I256> {
    let bytes = Bytes::from_slice(env, s.as_bytes());
    bytes_to_i256(env, &bytes)
}

// =============================================================================
// JSON Escaping
// =============================================================================

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

    // i32_to_bytes tests
    #[test]
    fn test_i32_to_bytes_zero() {
        let env = Env::default();
        let bytes = i32_to_bytes(&env, 0);
        assert_eq!(bytes.len(), 1);
        assert_eq!(bytes.get(0), Some(b'0'));
    }

    #[test]
    fn test_i32_to_bytes_positive() {
        let env = Env::default();
        let bytes = i32_to_bytes(&env, 42);
        assert_eq!(bytes.len(), 2);
        assert_eq!(bytes.get(0), Some(b'4'));
        assert_eq!(bytes.get(1), Some(b'2'));
    }

    #[test]
    fn test_i32_to_bytes_negative() {
        let env = Env::default();
        let bytes = i32_to_bytes(&env, -42);
        assert_eq!(bytes.len(), 3);
        assert_eq!(bytes.get(0), Some(b'-'));
        assert_eq!(bytes.get(1), Some(b'4'));
        assert_eq!(bytes.get(2), Some(b'2'));
    }

    #[test]
    fn test_i32_to_bytes_max() {
        let env = Env::default();
        let bytes = i32_to_bytes(&env, i32::MAX);
        assert_eq!(bytes.len(), 10); // 2147483647 is 10 digits
    }

    #[test]
    fn test_i32_to_bytes_min() {
        let env = Env::default();
        let bytes = i32_to_bytes(&env, i32::MIN);
        assert_eq!(bytes.len(), 11); // -2147483648 is 11 chars
    }

    // u64_to_bytes tests
    #[test]
    fn test_u64_to_bytes_zero() {
        let env = Env::default();
        let bytes = u64_to_bytes(&env, 0);
        assert_eq!(bytes.len(), 1);
        assert_eq!(bytes.get(0), Some(b'0'));
    }

    #[test]
    fn test_u64_to_bytes_single_digit() {
        let env = Env::default();
        let bytes = u64_to_bytes(&env, 7);
        assert_eq!(bytes.len(), 1);
        assert_eq!(bytes.get(0), Some(b'7'));
    }

    #[test]
    fn test_u64_to_bytes_multi_digit() {
        let env = Env::default();
        let bytes = u64_to_bytes(&env, 12345678901234);
        assert_eq!(bytes.len(), 14);
    }

    #[test]
    fn test_u64_to_bytes_max() {
        let env = Env::default();
        let bytes = u64_to_bytes(&env, u64::MAX);
        assert_eq!(bytes.len(), 20); // 18446744073709551615 is 20 digits
    }

    // bytes_to_u32 tests
    #[test]
    fn test_bytes_to_u32_zero() {
        let env = Env::default();
        let bytes = Bytes::from_slice(&env, b"0");
        assert_eq!(bytes_to_u32(&bytes), Some(0));
    }

    #[test]
    fn test_bytes_to_u32_simple() {
        let env = Env::default();
        let bytes = Bytes::from_slice(&env, b"42");
        assert_eq!(bytes_to_u32(&bytes), Some(42));
    }

    #[test]
    fn test_bytes_to_u32_max() {
        let env = Env::default();
        let bytes = Bytes::from_slice(&env, b"4294967295");
        assert_eq!(bytes_to_u32(&bytes), Some(u32::MAX));
    }

    #[test]
    fn test_bytes_to_u32_overflow() {
        let env = Env::default();
        let bytes = Bytes::from_slice(&env, b"4294967296"); // u32::MAX + 1
        assert_eq!(bytes_to_u32(&bytes), None);
    }

    #[test]
    fn test_bytes_to_u32_invalid() {
        let env = Env::default();
        let bytes = Bytes::from_slice(&env, b"abc");
        assert_eq!(bytes_to_u32(&bytes), None);
    }

    #[test]
    fn test_bytes_to_u32_empty() {
        let env = Env::default();
        let bytes = Bytes::from_slice(&env, b"");
        assert_eq!(bytes_to_u32(&bytes), None);
    }

    // bytes_to_i32 tests
    #[test]
    fn test_bytes_to_i32_zero() {
        let env = Env::default();
        let bytes = Bytes::from_slice(&env, b"0");
        assert_eq!(bytes_to_i32(&bytes), Some(0));
    }

    #[test]
    fn test_bytes_to_i32_positive() {
        let env = Env::default();
        let bytes = Bytes::from_slice(&env, b"42");
        assert_eq!(bytes_to_i32(&bytes), Some(42));
    }

    #[test]
    fn test_bytes_to_i32_negative() {
        let env = Env::default();
        let bytes = Bytes::from_slice(&env, b"-42");
        assert_eq!(bytes_to_i32(&bytes), Some(-42));
    }

    #[test]
    fn test_bytes_to_i32_max() {
        let env = Env::default();
        let bytes = Bytes::from_slice(&env, b"2147483647");
        assert_eq!(bytes_to_i32(&bytes), Some(i32::MAX));
    }

    #[test]
    fn test_bytes_to_i32_just_minus() {
        let env = Env::default();
        let bytes = Bytes::from_slice(&env, b"-");
        assert_eq!(bytes_to_i32(&bytes), None);
    }

    // bytes_to_u64 tests
    #[test]
    fn test_bytes_to_u64_zero() {
        let env = Env::default();
        let bytes = Bytes::from_slice(&env, b"0");
        assert_eq!(bytes_to_u64(&bytes), Some(0));
    }

    #[test]
    fn test_bytes_to_u64_simple() {
        let env = Env::default();
        let bytes = Bytes::from_slice(&env, b"42");
        assert_eq!(bytes_to_u64(&bytes), Some(42));
    }

    #[test]
    fn test_bytes_to_u64_large() {
        let env = Env::default();
        let bytes = Bytes::from_slice(&env, b"18446744073709551615");
        assert_eq!(bytes_to_u64(&bytes), Some(u64::MAX));
    }

    #[test]
    fn test_bytes_to_u64_overflow() {
        let env = Env::default();
        let bytes = Bytes::from_slice(&env, b"18446744073709551616"); // u64::MAX + 1
        assert_eq!(bytes_to_u64(&bytes), None);
    }

    // bytes_to_i64 tests
    #[test]
    fn test_bytes_to_i64_zero() {
        let env = Env::default();
        let bytes = Bytes::from_slice(&env, b"0");
        assert_eq!(bytes_to_i64(&bytes), Some(0));
    }

    #[test]
    fn test_bytes_to_i64_positive() {
        let env = Env::default();
        let bytes = Bytes::from_slice(&env, b"42");
        assert_eq!(bytes_to_i64(&bytes), Some(42));
    }

    #[test]
    fn test_bytes_to_i64_negative() {
        let env = Env::default();
        let bytes = Bytes::from_slice(&env, b"-42");
        assert_eq!(bytes_to_i64(&bytes), Some(-42));
    }

    #[test]
    fn test_bytes_to_i64_max() {
        let env = Env::default();
        let bytes = Bytes::from_slice(&env, b"9223372036854775807");
        assert_eq!(bytes_to_i64(&bytes), Some(i64::MAX));
    }

    // u128_to_bytes tests
    #[test]
    fn test_u128_to_bytes_zero() {
        let env = Env::default();
        let bytes = u128_to_bytes(&env, 0);
        assert_eq!(bytes.len(), 1);
        assert_eq!(bytes.get(0), Some(b'0'));
    }

    #[test]
    fn test_u128_to_bytes_simple() {
        let env = Env::default();
        let bytes = u128_to_bytes(&env, 42);
        assert_eq!(bytes.len(), 2);
    }

    #[test]
    fn test_u128_to_bytes_max() {
        let env = Env::default();
        let bytes = u128_to_bytes(&env, u128::MAX);
        // 340282366920938463463374607431768211455 is 39 digits
        assert_eq!(bytes.len(), 39);
    }

    // i128_to_bytes tests
    #[test]
    fn test_i128_to_bytes_zero() {
        let env = Env::default();
        let bytes = i128_to_bytes(&env, 0);
        assert_eq!(bytes.len(), 1);
        assert_eq!(bytes.get(0), Some(b'0'));
    }

    #[test]
    fn test_i128_to_bytes_positive() {
        let env = Env::default();
        let bytes = i128_to_bytes(&env, 42);
        assert_eq!(bytes.len(), 2);
    }

    #[test]
    fn test_i128_to_bytes_negative() {
        let env = Env::default();
        let bytes = i128_to_bytes(&env, -42);
        assert_eq!(bytes.len(), 3);
        assert_eq!(bytes.get(0), Some(b'-'));
    }

    #[test]
    fn test_i128_to_bytes_max() {
        let env = Env::default();
        let bytes = i128_to_bytes(&env, i128::MAX);
        // 170141183460469231731687303715884105727 is 39 digits
        assert_eq!(bytes.len(), 39);
    }

    #[test]
    fn test_i128_to_bytes_min() {
        let env = Env::default();
        let bytes = i128_to_bytes(&env, i128::MIN);
        // -170141183460469231731687303715884105728 is 40 chars
        assert_eq!(bytes.len(), 40);
        assert_eq!(bytes.get(0), Some(b'-'));
    }

    // bytes_to_u128 tests
    #[test]
    fn test_bytes_to_u128_zero() {
        let env = Env::default();
        let bytes = Bytes::from_slice(&env, b"0");
        assert_eq!(bytes_to_u128(&bytes), Some(0));
    }

    #[test]
    fn test_bytes_to_u128_simple() {
        let env = Env::default();
        let bytes = Bytes::from_slice(&env, b"42");
        assert_eq!(bytes_to_u128(&bytes), Some(42));
    }

    #[test]
    fn test_bytes_to_u128_max() {
        let env = Env::default();
        let bytes = Bytes::from_slice(&env, b"340282366920938463463374607431768211455");
        assert_eq!(bytes_to_u128(&bytes), Some(u128::MAX));
    }

    #[test]
    fn test_bytes_to_u128_overflow() {
        let env = Env::default();
        let bytes = Bytes::from_slice(&env, b"340282366920938463463374607431768211456"); // u128::MAX + 1
        assert_eq!(bytes_to_u128(&bytes), None);
    }

    // bytes_to_i128 tests
    #[test]
    fn test_bytes_to_i128_zero() {
        let env = Env::default();
        let bytes = Bytes::from_slice(&env, b"0");
        assert_eq!(bytes_to_i128(&bytes), Some(0));
    }

    #[test]
    fn test_bytes_to_i128_positive() {
        let env = Env::default();
        let bytes = Bytes::from_slice(&env, b"42");
        assert_eq!(bytes_to_i128(&bytes), Some(42));
    }

    #[test]
    fn test_bytes_to_i128_negative() {
        let env = Env::default();
        let bytes = Bytes::from_slice(&env, b"-42");
        assert_eq!(bytes_to_i128(&bytes), Some(-42));
    }

    #[test]
    fn test_bytes_to_i128_max() {
        let env = Env::default();
        let bytes = Bytes::from_slice(&env, b"170141183460469231731687303715884105727");
        assert_eq!(bytes_to_i128(&bytes), Some(i128::MAX));
    }

    #[test]
    fn test_bytes_to_i128_min() {
        let env = Env::default();
        let bytes = Bytes::from_slice(&env, b"-170141183460469231731687303715884105728");
        assert_eq!(bytes_to_i128(&bytes), Some(i128::MIN));
    }

    #[test]
    fn test_bytes_to_i128_overflow_positive() {
        let env = Env::default();
        let bytes = Bytes::from_slice(&env, b"170141183460469231731687303715884105728"); // i128::MAX + 1
        assert_eq!(bytes_to_i128(&bytes), None);
    }

    #[test]
    fn test_bytes_to_i128_overflow_negative() {
        let env = Env::default();
        let bytes = Bytes::from_slice(&env, b"-170141183460469231731687303715884105729"); // i128::MIN - 1
        assert_eq!(bytes_to_i128(&bytes), None);
    }

    // ==========================================================================
    // Hex conversion tests
    // ==========================================================================

    // u32_to_hex tests
    #[test]
    fn test_u32_to_hex_zero() {
        let env = Env::default();
        let bytes = u32_to_hex(&env, 0);
        assert_eq!(bytes, Bytes::from_slice(&env, b"0x0"));
    }

    #[test]
    fn test_u32_to_hex_simple() {
        let env = Env::default();
        let bytes = u32_to_hex(&env, 255);
        assert_eq!(bytes, Bytes::from_slice(&env, b"0xff"));
    }

    #[test]
    fn test_u32_to_hex_max() {
        let env = Env::default();
        let bytes = u32_to_hex(&env, u32::MAX);
        assert_eq!(bytes, Bytes::from_slice(&env, b"0xffffffff"));
    }

    // i32_to_hex tests
    #[test]
    fn test_i32_to_hex_zero() {
        let env = Env::default();
        let bytes = i32_to_hex(&env, 0);
        assert_eq!(bytes, Bytes::from_slice(&env, b"0x0"));
    }

    #[test]
    fn test_i32_to_hex_positive() {
        let env = Env::default();
        let bytes = i32_to_hex(&env, 255);
        assert_eq!(bytes, Bytes::from_slice(&env, b"0xff"));
    }

    #[test]
    fn test_i32_to_hex_negative() {
        let env = Env::default();
        let bytes = i32_to_hex(&env, -255);
        assert_eq!(bytes, Bytes::from_slice(&env, b"-0xff"));
    }

    // u64_to_hex tests
    #[test]
    fn test_u64_to_hex_zero() {
        let env = Env::default();
        let bytes = u64_to_hex(&env, 0);
        assert_eq!(bytes, Bytes::from_slice(&env, b"0x0"));
    }

    #[test]
    fn test_u64_to_hex_simple() {
        let env = Env::default();
        let bytes = u64_to_hex(&env, 0xDEADBEEF);
        assert_eq!(bytes, Bytes::from_slice(&env, b"0xdeadbeef"));
    }

    #[test]
    fn test_u64_to_hex_max() {
        let env = Env::default();
        let bytes = u64_to_hex(&env, u64::MAX);
        assert_eq!(bytes, Bytes::from_slice(&env, b"0xffffffffffffffff"));
    }

    // i64_to_hex tests
    #[test]
    fn test_i64_to_hex_negative() {
        let env = Env::default();
        let bytes = i64_to_hex(&env, -0xFF);
        assert_eq!(bytes, Bytes::from_slice(&env, b"-0xff"));
    }

    // u128_to_hex tests
    #[test]
    fn test_u128_to_hex_zero() {
        let env = Env::default();
        let bytes = u128_to_hex(&env, 0);
        assert_eq!(bytes, Bytes::from_slice(&env, b"0x0"));
    }

    #[test]
    fn test_u128_to_hex_simple() {
        let env = Env::default();
        let bytes = u128_to_hex(&env, 255);
        assert_eq!(bytes, Bytes::from_slice(&env, b"0xff"));
    }

    // i128_to_hex tests
    #[test]
    fn test_i128_to_hex_negative() {
        let env = Env::default();
        let bytes = i128_to_hex(&env, -255);
        assert_eq!(bytes, Bytes::from_slice(&env, b"-0xff"));
    }

    #[test]
    fn test_i128_to_hex_min() {
        let env = Env::default();
        let bytes = i128_to_hex(&env, i128::MIN);
        // i128::MIN = -170141183460469231731687303715884105728 = -0x80000000000000000000000000000000
        assert_eq!(bytes.len(), 35); // "-0x" (3) + 32 hex digits = 35
    }

    // hex_to_u32 tests
    #[test]
    fn test_hex_to_u32_simple() {
        let env = Env::default();
        let bytes = Bytes::from_slice(&env, b"0xff");
        assert_eq!(hex_to_u32(&bytes), Some(255));
    }

    #[test]
    fn test_hex_to_u32_no_prefix() {
        let env = Env::default();
        let bytes = Bytes::from_slice(&env, b"ff");
        assert_eq!(hex_to_u32(&bytes), Some(255));
    }

    #[test]
    fn test_hex_to_u32_uppercase() {
        let env = Env::default();
        let bytes = Bytes::from_slice(&env, b"0XFF");
        assert_eq!(hex_to_u32(&bytes), Some(255));
    }

    #[test]
    fn test_hex_to_u32_mixed_case() {
        let env = Env::default();
        let bytes = Bytes::from_slice(&env, b"0xDeAdBeEf");
        assert_eq!(hex_to_u32(&bytes), Some(0xDEADBEEF));
    }

    #[test]
    fn test_hex_to_u32_max() {
        let env = Env::default();
        let bytes = Bytes::from_slice(&env, b"0xffffffff");
        assert_eq!(hex_to_u32(&bytes), Some(u32::MAX));
    }

    #[test]
    fn test_hex_to_u32_overflow() {
        let env = Env::default();
        let bytes = Bytes::from_slice(&env, b"0x100000000"); // u32::MAX + 1
        assert_eq!(hex_to_u32(&bytes), None);
    }

    #[test]
    fn test_hex_to_u32_invalid() {
        let env = Env::default();
        let bytes = Bytes::from_slice(&env, b"0xgg");
        assert_eq!(hex_to_u32(&bytes), None);
    }

    #[test]
    fn test_hex_to_u32_empty() {
        let env = Env::default();
        let bytes = Bytes::from_slice(&env, b"");
        assert_eq!(hex_to_u32(&bytes), None);
    }

    #[test]
    fn test_hex_to_u32_just_prefix() {
        let env = Env::default();
        let bytes = Bytes::from_slice(&env, b"0x");
        assert_eq!(hex_to_u32(&bytes), None);
    }

    // hex_to_i32 tests
    #[test]
    fn test_hex_to_i32_positive() {
        let env = Env::default();
        let bytes = Bytes::from_slice(&env, b"0xff");
        assert_eq!(hex_to_i32(&bytes), Some(255));
    }

    #[test]
    fn test_hex_to_i32_negative() {
        let env = Env::default();
        let bytes = Bytes::from_slice(&env, b"-0xff");
        assert_eq!(hex_to_i32(&bytes), Some(-255));
    }

    #[test]
    fn test_hex_to_i32_negative_no_prefix() {
        let env = Env::default();
        let bytes = Bytes::from_slice(&env, b"-ff");
        assert_eq!(hex_to_i32(&bytes), Some(-255));
    }

    // hex_to_u64 tests
    #[test]
    fn test_hex_to_u64_simple() {
        let env = Env::default();
        let bytes = Bytes::from_slice(&env, b"0xdeadbeef");
        assert_eq!(hex_to_u64(&bytes), Some(0xDEADBEEF));
    }

    #[test]
    fn test_hex_to_u64_max() {
        let env = Env::default();
        let bytes = Bytes::from_slice(&env, b"0xffffffffffffffff");
        assert_eq!(hex_to_u64(&bytes), Some(u64::MAX));
    }

    // hex_to_i64 tests
    #[test]
    fn test_hex_to_i64_negative() {
        let env = Env::default();
        let bytes = Bytes::from_slice(&env, b"-0xff");
        assert_eq!(hex_to_i64(&bytes), Some(-255));
    }

    // hex_to_u128 tests
    #[test]
    fn test_hex_to_u128_simple() {
        let env = Env::default();
        let bytes = Bytes::from_slice(&env, b"0xff");
        assert_eq!(hex_to_u128(&bytes), Some(255));
    }

    #[test]
    fn test_hex_to_u128_large() {
        let env = Env::default();
        let bytes = Bytes::from_slice(&env, b"0xffffffffffffffffffffffffffffffff");
        assert_eq!(hex_to_u128(&bytes), Some(u128::MAX));
    }

    // hex_to_i128 tests
    #[test]
    fn test_hex_to_i128_positive() {
        let env = Env::default();
        let bytes = Bytes::from_slice(&env, b"0xff");
        assert_eq!(hex_to_i128(&bytes), Some(255));
    }

    #[test]
    fn test_hex_to_i128_negative() {
        let env = Env::default();
        let bytes = Bytes::from_slice(&env, b"-0xff");
        assert_eq!(hex_to_i128(&bytes), Some(-255));
    }

    // ==========================================================================
    // U256/I256 conversion tests
    // ==========================================================================

    // u256_to_bytes tests
    #[test]
    fn test_u256_to_bytes_zero() {
        let env = Env::default();
        let n = U256::from_u32(&env, 0);
        let bytes = u256_to_bytes(&env, &n);
        assert_eq!(bytes, Bytes::from_slice(&env, b"0"));
    }

    #[test]
    fn test_u256_to_bytes_simple() {
        let env = Env::default();
        let n = U256::from_u32(&env, 12345);
        let bytes = u256_to_bytes(&env, &n);
        assert_eq!(bytes, Bytes::from_slice(&env, b"12345"));
    }

    #[test]
    fn test_u256_to_bytes_large() {
        let env = Env::default();
        // Use from_u128 for a larger number
        let n = U256::from_u128(&env, 123456789012345678901234567890);
        let bytes = u256_to_bytes(&env, &n);
        assert_eq!(
            bytes,
            Bytes::from_slice(&env, b"123456789012345678901234567890")
        );
    }

    // i256_to_bytes tests
    #[test]
    fn test_i256_to_bytes_zero() {
        let env = Env::default();
        let n = I256::from_i32(&env, 0);
        let bytes = i256_to_bytes(&env, &n);
        assert_eq!(bytes, Bytes::from_slice(&env, b"0"));
    }

    #[test]
    fn test_i256_to_bytes_positive() {
        let env = Env::default();
        let n = I256::from_i32(&env, 12345);
        let bytes = i256_to_bytes(&env, &n);
        assert_eq!(bytes, Bytes::from_slice(&env, b"12345"));
    }

    #[test]
    fn test_i256_to_bytes_negative() {
        let env = Env::default();
        let n = I256::from_i32(&env, -12345);
        let bytes = i256_to_bytes(&env, &n);
        assert_eq!(bytes, Bytes::from_slice(&env, b"-12345"));
    }

    // u256_to_hex tests
    #[test]
    fn test_u256_to_hex_zero() {
        let env = Env::default();
        let n = U256::from_u32(&env, 0);
        let bytes = u256_to_hex(&env, &n);
        assert_eq!(bytes, Bytes::from_slice(&env, b"0x0"));
    }

    #[test]
    fn test_u256_to_hex_simple() {
        let env = Env::default();
        let n = U256::from_u32(&env, 255);
        let bytes = u256_to_hex(&env, &n);
        assert_eq!(bytes, Bytes::from_slice(&env, b"0xff"));
    }

    #[test]
    fn test_u256_to_hex_large() {
        let env = Env::default();
        let n = U256::from_u32(&env, 0xDEADBEEF);
        let bytes = u256_to_hex(&env, &n);
        assert_eq!(bytes, Bytes::from_slice(&env, b"0xdeadbeef"));
    }

    // i256_to_hex tests
    #[test]
    fn test_i256_to_hex_zero() {
        let env = Env::default();
        let n = I256::from_i32(&env, 0);
        let bytes = i256_to_hex(&env, &n);
        assert_eq!(bytes, Bytes::from_slice(&env, b"0x0"));
    }

    #[test]
    fn test_i256_to_hex_positive() {
        let env = Env::default();
        let n = I256::from_i32(&env, 255);
        let bytes = i256_to_hex(&env, &n);
        assert_eq!(bytes, Bytes::from_slice(&env, b"0xff"));
    }

    #[test]
    fn test_i256_to_hex_negative() {
        let env = Env::default();
        let n = I256::from_i32(&env, -255);
        let bytes = i256_to_hex(&env, &n);
        assert_eq!(bytes, Bytes::from_slice(&env, b"-0xff"));
    }

    // bytes_to_u256 tests
    #[test]
    fn test_bytes_to_u256_zero() {
        let env = Env::default();
        let bytes = Bytes::from_slice(&env, b"0");
        let n = bytes_to_u256(&env, &bytes);
        assert!(n.is_some());
        let result = u256_to_bytes(&env, &n.unwrap());
        assert_eq!(result, Bytes::from_slice(&env, b"0"));
    }

    #[test]
    fn test_bytes_to_u256_simple() {
        let env = Env::default();
        let bytes = Bytes::from_slice(&env, b"12345");
        let n = bytes_to_u256(&env, &bytes);
        assert!(n.is_some());
        let result = u256_to_bytes(&env, &n.unwrap());
        assert_eq!(result, Bytes::from_slice(&env, b"12345"));
    }

    #[test]
    fn test_bytes_to_u256_empty() {
        let env = Env::default();
        let bytes = Bytes::from_slice(&env, b"");
        assert_eq!(bytes_to_u256(&env, &bytes), None);
    }

    #[test]
    fn test_bytes_to_u256_invalid() {
        let env = Env::default();
        let bytes = Bytes::from_slice(&env, b"abc");
        assert_eq!(bytes_to_u256(&env, &bytes), None);
    }

    // bytes_to_i256 tests
    #[test]
    fn test_bytes_to_i256_positive() {
        let env = Env::default();
        let bytes = Bytes::from_slice(&env, b"12345");
        let n = bytes_to_i256(&env, &bytes);
        assert!(n.is_some());
        let result = i256_to_bytes(&env, &n.unwrap());
        assert_eq!(result, Bytes::from_slice(&env, b"12345"));
    }

    #[test]
    fn test_bytes_to_i256_negative() {
        let env = Env::default();
        let bytes = Bytes::from_slice(&env, b"-12345");
        let n = bytes_to_i256(&env, &bytes);
        assert!(n.is_some());
        let result = i256_to_bytes(&env, &n.unwrap());
        assert_eq!(result, Bytes::from_slice(&env, b"-12345"));
    }

    // hex_to_u256 tests
    #[test]
    fn test_hex_to_u256_simple() {
        let env = Env::default();
        let bytes = Bytes::from_slice(&env, b"0xff");
        let n = hex_to_u256(&env, &bytes);
        assert!(n.is_some());
        let result = u256_to_hex(&env, &n.unwrap());
        assert_eq!(result, Bytes::from_slice(&env, b"0xff"));
    }

    #[test]
    fn test_hex_to_u256_no_prefix() {
        let env = Env::default();
        let bytes = Bytes::from_slice(&env, b"ff");
        let n = hex_to_u256(&env, &bytes);
        assert!(n.is_some());
        let result = u256_to_hex(&env, &n.unwrap());
        assert_eq!(result, Bytes::from_slice(&env, b"0xff"));
    }

    #[test]
    fn test_hex_to_u256_empty() {
        let env = Env::default();
        let bytes = Bytes::from_slice(&env, b"");
        assert_eq!(hex_to_u256(&env, &bytes), None);
    }

    // hex_to_i256 tests
    #[test]
    fn test_hex_to_i256_positive() {
        let env = Env::default();
        let bytes = Bytes::from_slice(&env, b"0xff");
        let n = hex_to_i256(&env, &bytes);
        assert!(n.is_some());
        let result = i256_to_hex(&env, &n.unwrap());
        assert_eq!(result, Bytes::from_slice(&env, b"0xff"));
    }

    #[test]
    fn test_hex_to_i256_negative() {
        let env = Env::default();
        let bytes = Bytes::from_slice(&env, b"-0xff");
        let n = hex_to_i256(&env, &bytes);
        assert!(n.is_some());
        let result = i256_to_hex(&env, &n.unwrap());
        assert_eq!(result, Bytes::from_slice(&env, b"-0xff"));
    }

    // ==========================================================================
    // String convenience wrapper tests
    // ==========================================================================

    #[test]
    fn test_string_to_u32() {
        let env = Env::default();
        let s = String::from_str(&env, "12345");
        assert_eq!(string_to_u32(&env, &s), Some(12345));
    }

    #[test]
    fn test_string_to_u32_invalid() {
        let env = Env::default();
        let s = String::from_str(&env, "abc");
        assert_eq!(string_to_u32(&env, &s), None);
    }

    #[test]
    fn test_string_to_i32() {
        let env = Env::default();
        let s = String::from_str(&env, "-12345");
        assert_eq!(string_to_i32(&env, &s), Some(-12345));
    }

    #[test]
    fn test_string_to_u64() {
        let env = Env::default();
        let s = String::from_str(&env, "12345678901234");
        assert_eq!(string_to_u64(&env, &s), Some(12345678901234));
    }

    #[test]
    fn test_string_to_i64() {
        let env = Env::default();
        let s = String::from_str(&env, "-12345678901234");
        assert_eq!(string_to_i64(&env, &s), Some(-12345678901234));
    }

    #[test]
    fn test_string_to_u128() {
        let env = Env::default();
        let s = String::from_str(&env, "12345678901234567890");
        assert_eq!(string_to_u128(&env, &s), Some(12345678901234567890));
    }

    #[test]
    fn test_string_to_i128() {
        let env = Env::default();
        let s = String::from_str(&env, "-12345678901234567890");
        assert_eq!(string_to_i128(&env, &s), Some(-12345678901234567890));
    }

    #[test]
    fn test_string_to_u256() {
        let env = Env::default();
        let s = String::from_str(&env, "12345");
        let n = string_to_u256(&env, &s);
        assert!(n.is_some());
        let result = u256_to_bytes(&env, &n.unwrap());
        assert_eq!(result, Bytes::from_slice(&env, b"12345"));
    }

    #[test]
    fn test_string_to_i256() {
        let env = Env::default();
        let s = String::from_str(&env, "-12345");
        let n = string_to_i256(&env, &s);
        assert!(n.is_some());
        let result = i256_to_bytes(&env, &n.unwrap());
        assert_eq!(result, Bytes::from_slice(&env, b"-12345"));
    }

    // ==========================================================================
    // &str convenience wrapper tests
    // ==========================================================================

    #[test]
    fn test_str_to_u32() {
        let env = Env::default();
        assert_eq!(str_to_u32(&env, "12345"), Some(12345));
    }

    #[test]
    fn test_str_to_u32_invalid() {
        let env = Env::default();
        assert_eq!(str_to_u32(&env, "abc"), None);
    }

    #[test]
    fn test_str_to_i32() {
        let env = Env::default();
        assert_eq!(str_to_i32(&env, "-12345"), Some(-12345));
    }

    #[test]
    fn test_str_to_u64() {
        let env = Env::default();
        assert_eq!(str_to_u64(&env, "12345678901234"), Some(12345678901234));
    }

    #[test]
    fn test_str_to_i64() {
        let env = Env::default();
        assert_eq!(str_to_i64(&env, "-12345678901234"), Some(-12345678901234));
    }

    #[test]
    fn test_str_to_u128() {
        let env = Env::default();
        assert_eq!(
            str_to_u128(&env, "12345678901234567890"),
            Some(12345678901234567890)
        );
    }

    #[test]
    fn test_str_to_i128() {
        let env = Env::default();
        assert_eq!(
            str_to_i128(&env, "-12345678901234567890"),
            Some(-12345678901234567890)
        );
    }

    #[test]
    fn test_str_to_u256() {
        let env = Env::default();
        let n = str_to_u256(&env, "12345");
        assert!(n.is_some());
        let result = u256_to_bytes(&env, &n.unwrap());
        assert_eq!(result, Bytes::from_slice(&env, b"12345"));
    }

    #[test]
    fn test_str_to_i256() {
        let env = Env::default();
        let n = str_to_i256(&env, "-12345");
        assert!(n.is_some());
        let result = i256_to_bytes(&env, &n.unwrap());
        assert_eq!(result, Bytes::from_slice(&env, b"-12345"));
    }

    #[test]
    fn test_str_to_u256_large() {
        let env = Env::default();
        // Test with a larger number
        let n = str_to_u256(
            &env,
            "115792089237316195423570985008687907853269984665640564039457584007913129639935",
        );
        assert!(n.is_some());
    }

    #[test]
    fn test_str_to_u32_empty() {
        let env = Env::default();
        assert_eq!(str_to_u32(&env, ""), None);
    }
}
