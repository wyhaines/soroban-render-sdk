//! Prelude module for convenient imports.
//!
//! Import everything commonly needed with:
//!
//! ```rust,ignore
//! use soroban_render_sdk::prelude::*;
//! ```

// Re-export bytes utilities
pub use crate::bytes::{
    // Decimal Bytes to number
    bytes_to_i32,
    bytes_to_i64,
    bytes_to_i128,
    bytes_to_i256,
    bytes_to_u32,
    bytes_to_u64,
    bytes_to_u128,
    bytes_to_u256,
    // Core utilities
    concat_bytes,
    escape_json_bytes,
    escape_json_string,
    // Hex Bytes to number
    hex_to_i32,
    hex_to_i64,
    hex_to_i128,
    hex_to_i256,
    hex_to_u32,
    hex_to_u64,
    hex_to_u128,
    hex_to_u256,
    // Number to decimal Bytes
    i32_to_bytes,
    // Number to hex Bytes
    i32_to_hex,
    i64_to_bytes,
    i64_to_hex,
    i128_to_bytes,
    i128_to_hex,
    i256_to_bytes,
    i256_to_hex,
    // &str convenience wrappers
    str_to_i32,
    str_to_i64,
    str_to_i128,
    str_to_i256,
    str_to_u32,
    str_to_u64,
    str_to_u128,
    str_to_u256,
    string_to_bytes,
    // String convenience wrappers (soroban_sdk::String)
    string_to_i32,
    string_to_i64,
    string_to_i128,
    string_to_i256,
    string_to_u32,
    string_to_u64,
    string_to_u128,
    string_to_u256,
    u32_to_bytes,
    u32_to_hex,
    u64_to_bytes,
    u64_to_hex,
    u128_to_bytes,
    u128_to_hex,
    u256_to_bytes,
    u256_to_hex,
};

// Re-export metadata macros
pub use crate::{render_formats, render_has_styles, render_theme, render_v1, soroban_render};

// Re-export markdown builder (when feature enabled)
#[cfg(feature = "markdown")]
pub use crate::markdown::MarkdownBuilder;

// Re-export JSON builder (when feature enabled)
#[cfg(feature = "json")]
pub use crate::json::{FormBuilder, JsonDocument, TaskBuilder};

// Re-export router (when feature enabled)
#[cfg(feature = "router")]
pub use crate::router::{
    Request, Router, RouterResult, parse_id, path_eq, path_starts_with, path_suffix, path_to_bytes,
};

// Re-export style builder (when feature enabled)
#[cfg(feature = "styles")]
pub use crate::styles::StyleBuilder;

// Re-export registry (when feature enabled)
#[cfg(feature = "registry")]
pub use crate::registry::{BaseRegistry, ContractRegistry, RegistryKey};

// Re-export Bytes from soroban_sdk for convenience
pub use soroban_sdk::Bytes;
