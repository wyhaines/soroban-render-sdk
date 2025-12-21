//! Prelude module for convenient imports.
//!
//! Import everything commonly needed with:
//!
//! ```rust,ignore
//! use soroban_render_sdk::prelude::*;
//! ```

// Re-export bytes utilities
pub use crate::bytes::{
    concat_bytes, escape_json_bytes, escape_json_string, i64_to_bytes, string_to_bytes,
    u32_to_bytes,
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
    parse_id, path_eq, path_starts_with, path_suffix, path_to_bytes, Request, Router,
    RouterResult,
};

// Re-export style builder (when feature enabled)
#[cfg(feature = "styles")]
pub use crate::styles::StyleBuilder;

// Re-export Bytes from soroban_sdk for convenience
pub use soroban_sdk::Bytes;
