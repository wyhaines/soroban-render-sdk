//! Metadata macros for declaring Soroban Render contract capabilities.
//!
//! These macros simplify the declaration of contract metadata that signals
//! render support to viewers.
//!
//! # Basic Usage
//!
//! ```rust,ignore
//! use soroban_render_sdk::soroban_render;
//!
//! // Markdown support
//! soroban_render!(markdown);
//!
//! // With styles
//! soroban_render!(markdown, styles);
//!
//! // With theme contract
//! soroban_render!(markdown, theme = "CABCD123...");
//!
//! // Full featured
//! soroban_render!(markdown, json, styles, theme = "CABCD123...");
//! ```

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
    // Both orderings produce the same normalized output
    (markdown, json) => {
        soroban_sdk::contractmeta!(key = "render_formats", val = "markdown,json");
    };
    (json, markdown) => {
        soroban_sdk::contractmeta!(key = "render_formats", val = "markdown,json");
    };
}

/// Internal helper macro that emits metadata after arguments are parsed.
/// Not intended for direct use.
#[doc(hidden)]
#[macro_export]
macro_rules! __soroban_render_emit {
    // Single format without options
    (@format $fmt:ident) => {
        $crate::render_v1!();
        $crate::render_formats!($fmt);
    };
    // Single format with styles only
    (@format $fmt:ident @styles) => {
        $crate::render_v1!();
        $crate::render_formats!($fmt);
        $crate::render_has_styles!();
    };
    // Single format with theme only
    (@format $fmt:ident @theme $theme:expr) => {
        $crate::render_v1!();
        $crate::render_formats!($fmt);
        $crate::render_theme!($theme);
    };
    // Single format with both styles and theme
    (@format $fmt:ident @styles @theme $theme:expr) => {
        $crate::render_v1!();
        $crate::render_formats!($fmt);
        $crate::render_has_styles!();
        $crate::render_theme!($theme);
    };
    // Dual format without options
    (@formats $fmt1:ident $fmt2:ident) => {
        $crate::render_v1!();
        $crate::render_formats!($fmt1, $fmt2);
    };
    // Dual format with styles only
    (@formats $fmt1:ident $fmt2:ident @styles) => {
        $crate::render_v1!();
        $crate::render_formats!($fmt1, $fmt2);
        $crate::render_has_styles!();
    };
    // Dual format with theme only
    (@formats $fmt1:ident $fmt2:ident @theme $theme:expr) => {
        $crate::render_v1!();
        $crate::render_formats!($fmt1, $fmt2);
        $crate::render_theme!($theme);
    };
    // Dual format with both styles and theme
    (@formats $fmt1:ident $fmt2:ident @styles @theme $theme:expr) => {
        $crate::render_v1!();
        $crate::render_formats!($fmt1, $fmt2);
        $crate::render_has_styles!();
        $crate::render_theme!($theme);
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
    // Single format patterns
    ($fmt:ident) => {
        $crate::__soroban_render_emit!(@format $fmt);
    };
    ($fmt:ident, styles) => {
        $crate::__soroban_render_emit!(@format $fmt @styles);
    };
    ($fmt:ident, theme = $theme:expr) => {
        $crate::__soroban_render_emit!(@format $fmt @theme $theme);
    };
    ($fmt:ident, styles, theme = $theme:expr) => {
        $crate::__soroban_render_emit!(@format $fmt @styles @theme $theme);
    };

    // Dual format patterns
    ($fmt1:ident, $fmt2:ident) => {
        $crate::__soroban_render_emit!(@formats $fmt1 $fmt2);
    };
    ($fmt1:ident, $fmt2:ident, styles) => {
        $crate::__soroban_render_emit!(@formats $fmt1 $fmt2 @styles);
    };
    ($fmt1:ident, $fmt2:ident, theme = $theme:expr) => {
        $crate::__soroban_render_emit!(@formats $fmt1 $fmt2 @theme $theme);
    };
    ($fmt1:ident, $fmt2:ident, styles, theme = $theme:expr) => {
        $crate::__soroban_render_emit!(@formats $fmt1 $fmt2 @styles @theme $theme);
    };
}

/// Declare a theme contract for automatic style inheritance.
///
/// The viewer will fetch styles from this contract before rendering.
/// The theme contract should implement a `styles()` function.
///
/// # Example
///
/// ```rust,ignore
/// use soroban_render_sdk::render_theme;
///
/// render_theme!("CABCD123..."); // Contract ID of theme contract
/// ```
#[macro_export]
macro_rules! render_theme {
    ($contract_id:expr) => {
        soroban_sdk::contractmeta!(key = "render_theme", val = $contract_id);
    };
}

/// Declare that this contract provides styles.
///
/// This signals to viewers that the contract has a `styles()` function
/// that returns CSS.
///
/// # Example
///
/// ```rust,ignore
/// use soroban_render_sdk::render_has_styles;
///
/// render_has_styles!();
/// ```
#[macro_export]
macro_rules! render_has_styles {
    () => {
        soroban_sdk::contractmeta!(key = "render_styles", val = "true");
    };
}
