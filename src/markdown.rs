//! Markdown builder for constructing render output.
//!
//! Provides a fluent API for building markdown content with support for
//! Soroban Render's interactive protocols (render:, tx:, form:).
//!
//! # Example
//!
//! ```rust,ignore
//! use soroban_render_sdk::markdown::MarkdownBuilder;
//!
//! let output = MarkdownBuilder::new(&env)
//!     .h1("Welcome")
//!     .paragraph("Hello, World!")
//!     .render_link("Home", "/")
//!     .build();
//! ```

use crate::bytes::{concat_bytes, string_to_bytes, u32_to_bytes};
use soroban_sdk::{Bytes, Env, String, Vec};

/// A builder for constructing markdown content.
///
/// Uses the Vec<Bytes> accumulator pattern internally for efficient
/// string building in Soroban's no_std environment.
pub struct MarkdownBuilder<'a> {
    env: &'a Env,
    parts: Vec<Bytes>,
}

impl<'a> MarkdownBuilder<'a> {
    /// Create a new MarkdownBuilder.
    pub fn new(env: &'a Env) -> Self {
        Self {
            env,
            parts: Vec::new(env),
        }
    }

    // ========================================================================
    // Headings
    // ========================================================================

    /// Add a level 1 heading.
    pub fn h1(mut self, text: &str) -> Self {
        self.parts
            .push_back(Bytes::from_slice(self.env, b"# "));
        self.parts
            .push_back(Bytes::from_slice(self.env, text.as_bytes()));
        self.parts
            .push_back(Bytes::from_slice(self.env, b"\n\n"));
        self
    }

    /// Add a level 2 heading.
    pub fn h2(mut self, text: &str) -> Self {
        self.parts
            .push_back(Bytes::from_slice(self.env, b"## "));
        self.parts
            .push_back(Bytes::from_slice(self.env, text.as_bytes()));
        self.parts
            .push_back(Bytes::from_slice(self.env, b"\n\n"));
        self
    }

    /// Add a level 3 heading.
    pub fn h3(mut self, text: &str) -> Self {
        self.parts
            .push_back(Bytes::from_slice(self.env, b"### "));
        self.parts
            .push_back(Bytes::from_slice(self.env, text.as_bytes()));
        self.parts
            .push_back(Bytes::from_slice(self.env, b"\n\n"));
        self
    }

    /// Add a heading at a specific level (1-6).
    pub fn heading(mut self, level: u8, text: &str) -> Self {
        let prefix = match level {
            1 => b"# ".as_slice(),
            2 => b"## ".as_slice(),
            3 => b"### ".as_slice(),
            4 => b"#### ".as_slice(),
            5 => b"##### ".as_slice(),
            _ => b"###### ".as_slice(),
        };
        self.parts.push_back(Bytes::from_slice(self.env, prefix));
        self.parts
            .push_back(Bytes::from_slice(self.env, text.as_bytes()));
        self.parts
            .push_back(Bytes::from_slice(self.env, b"\n\n"));
        self
    }

    // ========================================================================
    // Text Content
    // ========================================================================

    /// Add inline text (no trailing newline).
    pub fn text(mut self, text: &str) -> Self {
        self.parts
            .push_back(Bytes::from_slice(self.env, text.as_bytes()));
        self
    }

    /// Add a paragraph (text followed by double newline).
    pub fn paragraph(mut self, text: &str) -> Self {
        self.parts
            .push_back(Bytes::from_slice(self.env, text.as_bytes()));
        self.parts
            .push_back(Bytes::from_slice(self.env, b"\n\n"));
        self
    }

    /// Add bold text.
    pub fn bold(mut self, text: &str) -> Self {
        self.parts
            .push_back(Bytes::from_slice(self.env, b"**"));
        self.parts
            .push_back(Bytes::from_slice(self.env, text.as_bytes()));
        self.parts
            .push_back(Bytes::from_slice(self.env, b"**"));
        self
    }

    /// Add italic text.
    pub fn italic(mut self, text: &str) -> Self {
        self.parts.push_back(Bytes::from_slice(self.env, b"*"));
        self.parts
            .push_back(Bytes::from_slice(self.env, text.as_bytes()));
        self.parts.push_back(Bytes::from_slice(self.env, b"*"));
        self
    }

    /// Add inline code.
    pub fn code(mut self, text: &str) -> Self {
        self.parts.push_back(Bytes::from_slice(self.env, b"`"));
        self.parts
            .push_back(Bytes::from_slice(self.env, text.as_bytes()));
        self.parts.push_back(Bytes::from_slice(self.env, b"`"));
        self
    }

    /// Add strikethrough text.
    pub fn strikethrough(mut self, text: &str) -> Self {
        self.parts
            .push_back(Bytes::from_slice(self.env, b"~~"));
        self.parts
            .push_back(Bytes::from_slice(self.env, text.as_bytes()));
        self.parts
            .push_back(Bytes::from_slice(self.env, b"~~"));
        self
    }

    /// Add a single newline.
    pub fn newline(mut self) -> Self {
        self.parts.push_back(Bytes::from_slice(self.env, b"\n"));
        self
    }

    /// Add a horizontal rule.
    pub fn hr(mut self) -> Self {
        self.parts
            .push_back(Bytes::from_slice(self.env, b"\n---\n\n"));
        self
    }

    // ========================================================================
    // Dynamic Content (from soroban_sdk types)
    // ========================================================================

    /// Add text from a soroban_sdk::String.
    pub fn text_string(mut self, s: &String) -> Self {
        self.parts.push_back(string_to_bytes(self.env, s));
        self
    }

    /// Add a u32 as text.
    pub fn number(mut self, n: u32) -> Self {
        self.parts.push_back(u32_to_bytes(self.env, n));
        self
    }

    /// Add raw Bytes.
    pub fn raw(mut self, bytes: Bytes) -> Self {
        self.parts.push_back(bytes);
        self
    }

    /// Add raw string slice.
    pub fn raw_str(mut self, s: &str) -> Self {
        self.parts
            .push_back(Bytes::from_slice(self.env, s.as_bytes()));
        self
    }

    // ========================================================================
    // Links
    // ========================================================================

    /// Add a standard markdown link.
    pub fn link(mut self, text: &str, href: &str) -> Self {
        self.parts.push_back(Bytes::from_slice(self.env, b"["));
        self.parts
            .push_back(Bytes::from_slice(self.env, text.as_bytes()));
        self.parts.push_back(Bytes::from_slice(self.env, b"]("));
        self.parts
            .push_back(Bytes::from_slice(self.env, href.as_bytes()));
        self.parts.push_back(Bytes::from_slice(self.env, b")"));
        self
    }

    /// Add a render: protocol link for navigation.
    ///
    /// Creates: `[text](render:path)`
    pub fn render_link(mut self, text: &str, path: &str) -> Self {
        self.parts.push_back(Bytes::from_slice(self.env, b"["));
        self.parts
            .push_back(Bytes::from_slice(self.env, text.as_bytes()));
        self.parts
            .push_back(Bytes::from_slice(self.env, b"](render:"));
        self.parts
            .push_back(Bytes::from_slice(self.env, path.as_bytes()));
        self.parts.push_back(Bytes::from_slice(self.env, b")"));
        self
    }

    /// Add a tx: protocol link for transactions.
    ///
    /// Creates: `[text](tx:method args)`
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// .tx_link("Delete", "delete_task", "{\"id\":1}")
    /// // Creates: [Delete](tx:delete_task {"id":1})
    /// ```
    pub fn tx_link(mut self, text: &str, method: &str, args: &str) -> Self {
        self.parts.push_back(Bytes::from_slice(self.env, b"["));
        self.parts
            .push_back(Bytes::from_slice(self.env, text.as_bytes()));
        self.parts.push_back(Bytes::from_slice(self.env, b"](tx:"));
        self.parts
            .push_back(Bytes::from_slice(self.env, method.as_bytes()));
        if !args.is_empty() {
            self.parts.push_back(Bytes::from_slice(self.env, b" "));
            self.parts
                .push_back(Bytes::from_slice(self.env, args.as_bytes()));
        }
        self.parts.push_back(Bytes::from_slice(self.env, b")"));
        self
    }

    /// Add a tx: link with a dynamically built argument (id from u32).
    ///
    /// Creates: `[text](tx:method {"id":n})`
    pub fn tx_link_id(mut self, text: &str, method: &str, id: u32) -> Self {
        self.parts.push_back(Bytes::from_slice(self.env, b"["));
        self.parts
            .push_back(Bytes::from_slice(self.env, text.as_bytes()));
        self.parts.push_back(Bytes::from_slice(self.env, b"](tx:"));
        self.parts
            .push_back(Bytes::from_slice(self.env, method.as_bytes()));
        self.parts
            .push_back(Bytes::from_slice(self.env, b" {\"id\":"));
        self.parts.push_back(u32_to_bytes(self.env, id));
        self.parts.push_back(Bytes::from_slice(self.env, b"})"));
        self
    }

    /// Add a form: protocol link for form submission.
    ///
    /// Creates: `[text](form:action)`
    pub fn form_link(mut self, text: &str, action: &str) -> Self {
        self.parts.push_back(Bytes::from_slice(self.env, b"["));
        self.parts
            .push_back(Bytes::from_slice(self.env, text.as_bytes()));
        self.parts
            .push_back(Bytes::from_slice(self.env, b"](form:"));
        self.parts
            .push_back(Bytes::from_slice(self.env, action.as_bytes()));
        self.parts.push_back(Bytes::from_slice(self.env, b")"));
        self
    }

    // ========================================================================
    // Alerts / Callouts
    // ========================================================================

    /// Add a TIP alert callout.
    pub fn tip(self, content: &str) -> Self {
        self.alert("TIP", content)
    }

    /// Add a NOTE alert callout.
    pub fn note(self, content: &str) -> Self {
        self.alert("NOTE", content)
    }

    /// Add a WARNING alert callout.
    pub fn warning(self, content: &str) -> Self {
        self.alert("WARNING", content)
    }

    /// Add an INFO alert callout.
    pub fn info(self, content: &str) -> Self {
        self.alert("INFO", content)
    }

    /// Add a CAUTION alert callout.
    pub fn caution(self, content: &str) -> Self {
        self.alert("CAUTION", content)
    }

    /// Add an alert with a custom type.
    ///
    /// Creates:
    /// ```text
    /// > [!TYPE]
    /// > content
    /// ```
    pub fn alert(mut self, alert_type: &str, content: &str) -> Self {
        self.parts.push_back(Bytes::from_slice(self.env, b"> [!"));
        self.parts
            .push_back(Bytes::from_slice(self.env, alert_type.as_bytes()));
        self.parts
            .push_back(Bytes::from_slice(self.env, b"]\n> "));
        self.parts
            .push_back(Bytes::from_slice(self.env, content.as_bytes()));
        self.parts
            .push_back(Bytes::from_slice(self.env, b"\n\n"));
        self
    }

    // ========================================================================
    // Columns Layout
    // ========================================================================

    /// Start a columns layout.
    ///
    /// Creates: `:::columns`
    pub fn columns_start(mut self) -> Self {
        self.parts
            .push_back(Bytes::from_slice(self.env, b":::columns\n"));
        self
    }

    /// Add a column separator.
    ///
    /// Creates: `|||`
    pub fn column_separator(mut self) -> Self {
        self.parts
            .push_back(Bytes::from_slice(self.env, b"|||\n"));
        self
    }

    /// End a columns layout.
    ///
    /// Creates: `:::`
    pub fn columns_end(mut self) -> Self {
        self.parts
            .push_back(Bytes::from_slice(self.env, b":::\n\n"));
        self
    }

    // ========================================================================
    // Includes
    // ========================================================================

    /// Include content from another contract.
    ///
    /// Creates: `{{include contract=ID func="name"}}`
    pub fn include(mut self, contract_id: &str, func: &str) -> Self {
        self.parts
            .push_back(Bytes::from_slice(self.env, b"{{include contract="));
        self.parts
            .push_back(Bytes::from_slice(self.env, contract_id.as_bytes()));
        self.parts
            .push_back(Bytes::from_slice(self.env, b" func=\""));
        self.parts
            .push_back(Bytes::from_slice(self.env, func.as_bytes()));
        self.parts
            .push_back(Bytes::from_slice(self.env, b"\"}}"));
        self
    }

    /// Include content from another contract with a path argument.
    ///
    /// Creates: `{{include contract=ID func="name" path="path"}}`
    pub fn include_with_path(mut self, contract_id: &str, func: &str, path: &str) -> Self {
        self.parts
            .push_back(Bytes::from_slice(self.env, b"{{include contract="));
        self.parts
            .push_back(Bytes::from_slice(self.env, contract_id.as_bytes()));
        self.parts
            .push_back(Bytes::from_slice(self.env, b" func=\""));
        self.parts
            .push_back(Bytes::from_slice(self.env, func.as_bytes()));
        self.parts
            .push_back(Bytes::from_slice(self.env, b"\" path=\""));
        self.parts
            .push_back(Bytes::from_slice(self.env, path.as_bytes()));
        self.parts
            .push_back(Bytes::from_slice(self.env, b"\"}}"));
        self
    }

    // ========================================================================
    // Form Elements (HTML)
    // ========================================================================

    /// Add an input element.
    ///
    /// Creates: `<input name="name" placeholder="placeholder" />`
    pub fn input(mut self, name: &str, placeholder: &str) -> Self {
        self.parts
            .push_back(Bytes::from_slice(self.env, b"<input name=\""));
        self.parts
            .push_back(Bytes::from_slice(self.env, name.as_bytes()));
        self.parts
            .push_back(Bytes::from_slice(self.env, b"\" placeholder=\""));
        self.parts
            .push_back(Bytes::from_slice(self.env, placeholder.as_bytes()));
        self.parts
            .push_back(Bytes::from_slice(self.env, b"\" />\n"));
        self
    }

    /// Add a textarea element.
    ///
    /// Creates: `<textarea name="name" rows="N" placeholder="placeholder"></textarea>`
    pub fn textarea(mut self, name: &str, rows: u8, placeholder: &str) -> Self {
        self.parts
            .push_back(Bytes::from_slice(self.env, b"<textarea name=\""));
        self.parts
            .push_back(Bytes::from_slice(self.env, name.as_bytes()));
        self.parts
            .push_back(Bytes::from_slice(self.env, b"\" rows=\""));
        self.parts.push_back(u32_to_bytes(self.env, rows as u32));
        self.parts
            .push_back(Bytes::from_slice(self.env, b"\" placeholder=\""));
        self.parts
            .push_back(Bytes::from_slice(self.env, placeholder.as_bytes()));
        self.parts
            .push_back(Bytes::from_slice(self.env, b"\"></textarea>\n"));
        self
    }

    // ========================================================================
    // Lists
    // ========================================================================

    /// Add a list item.
    ///
    /// Creates: `- text`
    pub fn list_item(mut self, text: &str) -> Self {
        self.parts.push_back(Bytes::from_slice(self.env, b"- "));
        self.parts
            .push_back(Bytes::from_slice(self.env, text.as_bytes()));
        self.parts.push_back(Bytes::from_slice(self.env, b"\n"));
        self
    }

    /// Add a checkbox list item.
    ///
    /// Creates: `- [x] text` or `- [ ] text`
    pub fn checkbox(mut self, checked: bool, text: &str) -> Self {
        if checked {
            self.parts
                .push_back(Bytes::from_slice(self.env, b"- [x] "));
        } else {
            self.parts
                .push_back(Bytes::from_slice(self.env, b"- [ ] "));
        }
        self.parts
            .push_back(Bytes::from_slice(self.env, text.as_bytes()));
        self.parts.push_back(Bytes::from_slice(self.env, b"\n"));
        self
    }

    // ========================================================================
    // Build
    // ========================================================================

    /// Build the final Bytes output.
    pub fn build(self) -> Bytes {
        concat_bytes(self.env, &self.parts)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_h1() {
        let env = Env::default();
        let output = MarkdownBuilder::new(&env).h1("Hello").build();
        // "# Hello\n\n" = 9 bytes (2 + 5 + 2)
        assert_eq!(output.len(), 9);
    }

    #[test]
    fn test_paragraph() {
        let env = Env::default();
        let output = MarkdownBuilder::new(&env).paragraph("Test").build();
        // "Test\n\n" = 6 bytes
        assert_eq!(output.len(), 6);
    }

    #[test]
    fn test_render_link() {
        let env = Env::default();
        let output = MarkdownBuilder::new(&env)
            .render_link("Home", "/")
            .build();
        // "[Home](render:/)" = 16 bytes
        assert_eq!(output.len(), 16);
    }

    #[test]
    fn test_tx_link_id() {
        let env = Env::default();
        let output = MarkdownBuilder::new(&env)
            .tx_link_id("Delete", "delete_task", 42)
            .build();
        // "[Delete](tx:delete_task {"id":42})" = 34 bytes
        assert_eq!(output.len(), 34);
    }

    #[test]
    fn test_form_link() {
        let env = Env::default();
        let output = MarkdownBuilder::new(&env)
            .form_link("Submit", "add_task")
            .build();
        // "[Submit](form:add_task)" = 23 bytes
        assert_eq!(output.len(), 23);
    }

    #[test]
    fn test_tip_alert() {
        let env = Env::default();
        let output = MarkdownBuilder::new(&env).tip("This is a tip").build();
        // "> [!TIP]\n> This is a tip\n\n" = 26 bytes
        assert_eq!(output.len(), 26);
    }

    #[test]
    fn test_columns() {
        let env = Env::default();
        let output = MarkdownBuilder::new(&env)
            .columns_start()
            .text("Col1")
            .column_separator()
            .text("Col2")
            .columns_end()
            .build();
        // ":::columns\nCol1|||\nCol2:::\n\n"
        assert!(output.len() > 0);
    }

    #[test]
    fn test_include() {
        let env = Env::default();
        let output = MarkdownBuilder::new(&env)
            .include("CABCD123", "header")
            .build();
        // {{include contract=CABCD123 func="header"}}
        assert!(output.len() > 30);
    }

    #[test]
    fn test_input() {
        let env = Env::default();
        let output = MarkdownBuilder::new(&env)
            .input("name", "Enter name")
            .build();
        assert!(output.len() > 20);
    }

    #[test]
    fn test_checkbox_checked() {
        let env = Env::default();
        let output = MarkdownBuilder::new(&env)
            .checkbox(true, "Done task")
            .build();
        // "- [x] Done task\n" = 16 bytes
        assert_eq!(output.len(), 16);
    }

    #[test]
    fn test_checkbox_unchecked() {
        let env = Env::default();
        let output = MarkdownBuilder::new(&env)
            .checkbox(false, "Todo task")
            .build();
        // "- [ ] Todo task\n" = 16 bytes
        assert_eq!(output.len(), 16);
    }

    #[test]
    fn test_chaining() {
        let env = Env::default();
        let output = MarkdownBuilder::new(&env)
            .h1("Title")
            .paragraph("Content")
            .render_link("Home", "/")
            .build();
        assert!(output.len() > 30);
    }
}
