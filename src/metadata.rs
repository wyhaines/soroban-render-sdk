//! Metadata macros for declaring Soroban Render contract capabilities.
//!
//! These macros simplify the declaration of contract metadata that signals
//! render support to viewers.

/// Declare render v1 support.
///
/// This macro expands to `contractmeta!(key = "render", val = "v1")`.
///
/// # Example
///
/// ```rust,ignore
/// use soroban_render_sdk::render_v1;
///
/// render_v1!();
/// ```
#[macro_export]
macro_rules! render_v1 {
    () => {
        soroban_sdk::contractmeta!(key = "render", val = "v1");
    };
}

/// Declare supported render formats.
///
/// # Examples
///
/// ```rust,ignore
/// use soroban_render_sdk::render_formats;
///
/// // Markdown only
/// render_formats!(markdown);
///
/// // JSON only
/// render_formats!(json);
///
/// // Both formats
/// render_formats!(markdown, json);
/// ```
#[macro_export]
macro_rules! render_formats {
    (markdown) => {
        soroban_sdk::contractmeta!(key = "render_formats", val = "markdown");
    };
    (json) => {
        soroban_sdk::contractmeta!(key = "render_formats", val = "json");
    };
    (markdown, json) => {
        soroban_sdk::contractmeta!(key = "render_formats", val = "markdown,json");
    };
    (json, markdown) => {
        soroban_sdk::contractmeta!(key = "render_formats", val = "markdown,json");
    };
}

/// Declare full Soroban Render support with format specification.
///
/// This is a convenience macro that combines `render_v1!()` and `render_formats!()`.
///
/// # Examples
///
/// ```rust,ignore
/// use soroban_render_sdk::soroban_render;
///
/// // Markdown support
/// soroban_render!(markdown);
///
/// // JSON support
/// soroban_render!(json);
///
/// // Both formats
/// soroban_render!(markdown, json);
/// ```
#[macro_export]
macro_rules! soroban_render {
    (markdown) => {
        $crate::render_v1!();
        $crate::render_formats!(markdown);
    };
    (json) => {
        $crate::render_v1!();
        $crate::render_formats!(json);
    };
    (markdown, json) => {
        $crate::render_v1!();
        $crate::render_formats!(markdown, json);
    };
    (json, markdown) => {
        $crate::render_v1!();
        $crate::render_formats!(markdown, json);
    };
}
