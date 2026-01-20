//! CSS style builder for constructing stylesheet output.
//!
//! Provides a fluent API for building CSS stylesheets with support for
//! common patterns like variables, rules, and media queries.
//!
//! # Example
//!
//! ```rust,ignore
//! use soroban_render_sdk::styles::StyleBuilder;
//!
//! let output = StyleBuilder::new(&env)
//!     .root_var("primary", "#0066cc")
//!     .root_var("bg", "#ffffff")
//!     .rule("h1", "color: var(--primary); font-size: 2rem;")
//!     .rule("a", "color: var(--primary);")
//!     .build();
//! ```

use crate::bytes::concat_bytes;
use soroban_sdk::{Bytes, Env, Vec};

/// A builder for constructing CSS stylesheets.
///
/// Uses the `Vec<Bytes>` accumulator pattern internally for efficient
/// string building in Soroban's no_std environment.
pub struct StyleBuilder<'a> {
    env: &'a Env,
    parts: Vec<Bytes>,
}

impl<'a> StyleBuilder<'a> {
    /// Create a new StyleBuilder.
    pub fn new(env: &'a Env) -> Self {
        Self {
            env,
            parts: Vec::new(env),
        }
    }

    // ========================================================================
    // Private Helpers
    // ========================================================================

    /// Push a byte slice to parts.
    fn push(&mut self, bytes: &[u8]) {
        self.parts.push_back(Bytes::from_slice(self.env, bytes));
    }

    /// Push a string to parts.
    fn push_str(&mut self, s: &str) {
        self.parts
            .push_back(Bytes::from_slice(self.env, s.as_bytes()));
    }

    /// Add an indented property line: `  prefix{name}: value;\n`
    fn indented_property(&mut self, prefix: &[u8], name: &str, value: &str) {
        self.push(b"  ");
        self.push(prefix);
        self.push_str(name);
        self.push(b": ");
        self.push_str(value);
        self.push(b";\n");
    }

    /// Close a block with `}\n`.
    fn close_block(&mut self) {
        self.push(b"}\n");
    }

    // ========================================================================
    // CSS Variables (Custom Properties)
    // ========================================================================

    /// Add a CSS custom property to :root.
    ///
    /// Creates: `:root { --name: value; }`
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// .root_var("primary", "#0066cc")
    /// // Output: :root { --primary: #0066cc; }
    /// ```
    pub fn root_var(mut self, name: &str, value: &str) -> Self {
        self.push(b":root { --");
        self.push_str(name);
        self.push(b": ");
        self.push_str(value);
        self.push(b"; }\n");
        self
    }

    /// Start a :root block for multiple CSS variables.
    ///
    /// Creates: `:root {`
    ///
    /// Use with `.var()` and `.root_vars_end()`.
    pub fn root_vars_start(mut self) -> Self {
        self.push(b":root {\n");
        self
    }

    /// Add a CSS variable within a :root block.
    ///
    /// Creates: `  --name: value;`
    ///
    /// Must be used between `.root_vars_start()` and `.root_vars_end()`.
    pub fn var(mut self, name: &str, value: &str) -> Self {
        self.indented_property(b"--", name, value);
        self
    }

    /// End a :root block.
    ///
    /// Creates: `}`
    pub fn root_vars_end(mut self) -> Self {
        self.close_block();
        self
    }

    // ========================================================================
    // CSS Rules
    // ========================================================================

    /// Add a CSS rule with inline properties.
    ///
    /// Creates: `selector { properties }`
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// .rule("h1", "color: blue; font-size: 2rem;")
    /// // Output: h1 { color: blue; font-size: 2rem; }
    /// ```
    pub fn rule(mut self, selector: &str, properties: &str) -> Self {
        self.push_str(selector);
        self.push(b" { ");
        self.push_str(properties);
        self.push(b" }\n");
        self
    }

    /// Start a rule block for multi-line properties.
    ///
    /// Creates: `selector {`
    ///
    /// Use with `.prop()` and `.rule_end()`.
    pub fn rule_start(mut self, selector: &str) -> Self {
        self.push_str(selector);
        self.push(b" {\n");
        self
    }

    /// Add a property within a rule block.
    ///
    /// Creates: `  property: value;`
    ///
    /// Must be used between `.rule_start()` and `.rule_end()`.
    pub fn prop(mut self, property: &str, value: &str) -> Self {
        self.indented_property(b"", property, value);
        self
    }

    /// End a rule block.
    ///
    /// Creates: `}`
    pub fn rule_end(mut self) -> Self {
        self.close_block();
        self
    }

    // ========================================================================
    // Media Queries
    // ========================================================================

    /// Start a media query block.
    ///
    /// Creates: `@media condition {`
    ///
    /// Use with rules and `.media_end()`.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// .media_start("(max-width: 768px)")
    ///     .rule("h1", "font-size: 1.5rem;")
    /// .media_end()
    /// ```
    pub fn media_start(mut self, condition: &str) -> Self {
        self.push(b"@media ");
        self.push_str(condition);
        self.push(b" {\n");
        self
    }

    /// End a media query block.
    ///
    /// Creates: `}`
    pub fn media_end(mut self) -> Self {
        self.close_block();
        self
    }

    /// Start a dark mode media query block.
    ///
    /// Creates: `@media (prefers-color-scheme: dark) {`
    ///
    /// Convenience method for the common dark mode pattern.
    pub fn dark_mode_start(self) -> Self {
        self.media_start("(prefers-color-scheme: dark)")
    }

    /// Start a light mode media query block.
    ///
    /// Creates: `@media (prefers-color-scheme: light) {`
    pub fn light_mode_start(self) -> Self {
        self.media_start("(prefers-color-scheme: light)")
    }

    /// Start a mobile-first responsive breakpoint.
    ///
    /// Creates: `@media (min-width: Npx) {`
    pub fn breakpoint_min(mut self, min_width: u32) -> Self {
        self.push(b"@media (min-width: ");
        self.parts
            .push_back(crate::bytes::u32_to_bytes(self.env, min_width));
        self.push(b"px) {\n");
        self
    }

    /// Start a desktop-first responsive breakpoint.
    ///
    /// Creates: `@media (max-width: Npx) {`
    pub fn breakpoint_max(mut self, max_width: u32) -> Self {
        self.push(b"@media (max-width: ");
        self.parts
            .push_back(crate::bytes::u32_to_bytes(self.env, max_width));
        self.push(b"px) {\n");
        self
    }

    // ========================================================================
    // Utilities
    // ========================================================================

    /// Add raw CSS string.
    ///
    /// Useful for complex selectors or CSS that doesn't fit the builder pattern.
    pub fn raw(mut self, css: &str) -> Self {
        self.push_str(css);
        self
    }

    /// Add a CSS comment.
    ///
    /// Creates: `/* text */`
    pub fn comment(mut self, text: &str) -> Self {
        self.push(b"/* ");
        self.push_str(text);
        self.push(b" */\n");
        self
    }

    /// Add a newline for formatting.
    pub fn newline(mut self) -> Self {
        self.push(b"\n");
        self
    }

    // ========================================================================
    // Build
    // ========================================================================

    /// Build the final CSS Bytes output.
    pub fn build(self) -> Bytes {
        concat_bytes(self.env, &self.parts)
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use super::*;

    fn bytes_to_string(bytes: &Bytes) -> alloc::string::String {
        let mut s = alloc::string::String::new();
        for i in 0..bytes.len() {
            s.push(bytes.get(i).unwrap() as char);
        }
        s
    }

    #[test]
    fn test_root_var() {
        let env = Env::default();
        let output = StyleBuilder::new(&env)
            .root_var("primary", "#0066cc")
            .build();
        let css = bytes_to_string(&output);
        assert_eq!(css, ":root { --primary: #0066cc; }\n");
    }

    #[test]
    fn test_root_vars_block() {
        let env = Env::default();
        let output = StyleBuilder::new(&env)
            .root_vars_start()
            .var("primary", "#0066cc")
            .var("bg", "#ffffff")
            .root_vars_end()
            .build();
        let css = bytes_to_string(&output);
        assert!(css.contains(":root {\n"));
        assert!(css.contains("  --primary: #0066cc;\n"));
        assert!(css.contains("  --bg: #ffffff;\n"));
        assert!(css.ends_with("}\n"));
    }

    #[test]
    fn test_rule() {
        let env = Env::default();
        let output = StyleBuilder::new(&env).rule("h1", "color: blue;").build();
        let css = bytes_to_string(&output);
        assert_eq!(css, "h1 { color: blue; }\n");
    }

    #[test]
    fn test_rule_block() {
        let env = Env::default();
        let output = StyleBuilder::new(&env)
            .rule_start("h1")
            .prop("color", "blue")
            .prop("font-size", "2rem")
            .rule_end()
            .build();
        let css = bytes_to_string(&output);
        assert!(css.contains("h1 {\n"));
        assert!(css.contains("  color: blue;\n"));
        assert!(css.contains("  font-size: 2rem;\n"));
    }

    #[test]
    fn test_dark_mode() {
        let env = Env::default();
        let output = StyleBuilder::new(&env)
            .dark_mode_start()
            .rule_start(":root")
            .prop("--bg", "#1a1a1a")
            .rule_end()
            .media_end()
            .build();
        let css = bytes_to_string(&output);
        assert!(css.contains("@media (prefers-color-scheme: dark)"));
        assert!(css.contains("--bg: #1a1a1a;"));
    }

    #[test]
    fn test_light_mode() {
        let env = Env::default();
        let output = StyleBuilder::new(&env)
            .light_mode_start()
            .rule(":root", "--bg: #ffffff;")
            .media_end()
            .build();
        let css = bytes_to_string(&output);
        assert!(css.contains("@media (prefers-color-scheme: light)"));
        assert!(css.contains("--bg: #ffffff;"));
    }

    #[test]
    fn test_breakpoint_min() {
        let env = Env::default();
        let output = StyleBuilder::new(&env)
            .breakpoint_min(768)
            .rule("h1", "font-size: 2rem;")
            .media_end()
            .build();
        let css = bytes_to_string(&output);
        assert!(css.contains("@media (min-width: 768px)"));
        assert!(css.contains("font-size: 2rem;"));
    }

    #[test]
    fn test_breakpoint_max() {
        let env = Env::default();
        let output = StyleBuilder::new(&env)
            .breakpoint_max(767)
            .rule("h1", "font-size: 1.5rem;")
            .media_end()
            .build();
        let css = bytes_to_string(&output);
        assert!(css.contains("@media (max-width: 767px)"));
        assert!(css.contains("font-size: 1.5rem;"));
    }

    #[test]
    fn test_comment() {
        let env = Env::default();
        let output = StyleBuilder::new(&env).comment("Theme styles").build();
        let css = bytes_to_string(&output);
        assert_eq!(css, "/* Theme styles */\n");
    }

    #[test]
    fn test_newline() {
        let env = Env::default();
        let output = StyleBuilder::new(&env)
            .comment("Section 1")
            .newline()
            .comment("Section 2")
            .build();
        let css = bytes_to_string(&output);
        assert!(css.contains("*/\n\n/*"));
    }

    #[test]
    fn test_raw() {
        let env = Env::default();
        let raw_css = ".complex > .selector:hover { opacity: 0.8; }";
        let output = StyleBuilder::new(&env).raw(raw_css).build();
        let css = bytes_to_string(&output);
        assert_eq!(css, raw_css);
    }

    #[test]
    fn test_chaining() {
        let env = Env::default();
        let output = StyleBuilder::new(&env)
            .comment("Base theme")
            .root_vars_start()
            .var("primary", "#0066cc")
            .root_vars_end()
            .rule("h1", "color: var(--primary);")
            .build();
        let css = bytes_to_string(&output);
        assert!(css.starts_with("/* Base theme */\n"));
        assert!(css.contains("--primary: #0066cc;"));
        assert!(css.contains("h1 { color: var(--primary); }"));
    }

    #[test]
    fn test_complete_theme() {
        let env = Env::default();
        let output = StyleBuilder::new(&env)
            .root_vars_start()
            .var("primary", "#0066cc")
            .var("bg", "#ffffff")
            .root_vars_end()
            .rule("body", "background: var(--bg);")
            .dark_mode_start()
            .rule_start(":root")
            .prop("--bg", "#1a1a1a")
            .rule_end()
            .media_end()
            .build();
        let css = bytes_to_string(&output);

        // Verify structure
        assert!(css.contains(":root {\n  --primary: #0066cc;"));
        assert!(css.contains("body { background: var(--bg); }"));
        assert!(css.contains("@media (prefers-color-scheme: dark)"));
        assert!(css.contains("--bg: #1a1a1a;"));
    }
}
