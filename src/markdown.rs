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
/// Uses the `Vec<Bytes>` accumulator pattern internally for efficient
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
        self.parts.push_back(Bytes::from_slice(self.env, b"# "));
        self.parts
            .push_back(Bytes::from_slice(self.env, text.as_bytes()));
        self.parts.push_back(Bytes::from_slice(self.env, b"\n\n"));
        self
    }

    /// Add a level 2 heading.
    pub fn h2(mut self, text: &str) -> Self {
        self.parts.push_back(Bytes::from_slice(self.env, b"## "));
        self.parts
            .push_back(Bytes::from_slice(self.env, text.as_bytes()));
        self.parts.push_back(Bytes::from_slice(self.env, b"\n\n"));
        self
    }

    /// Add a level 3 heading.
    pub fn h3(mut self, text: &str) -> Self {
        self.parts.push_back(Bytes::from_slice(self.env, b"### "));
        self.parts
            .push_back(Bytes::from_slice(self.env, text.as_bytes()));
        self.parts.push_back(Bytes::from_slice(self.env, b"\n\n"));
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
        self.parts.push_back(Bytes::from_slice(self.env, b"\n\n"));
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
        self.parts.push_back(Bytes::from_slice(self.env, b"\n\n"));
        self
    }

    /// Add bold text.
    pub fn bold(mut self, text: &str) -> Self {
        self.parts.push_back(Bytes::from_slice(self.env, b"**"));
        self.parts
            .push_back(Bytes::from_slice(self.env, text.as_bytes()));
        self.parts.push_back(Bytes::from_slice(self.env, b"**"));
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
        self.parts.push_back(Bytes::from_slice(self.env, b"~~"));
        self.parts
            .push_back(Bytes::from_slice(self.env, text.as_bytes()));
        self.parts.push_back(Bytes::from_slice(self.env, b"~~"));
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

    /// Add a form: link targeting a specific contract via registry alias.
    ///
    /// Creates: `[text](form:@alias:method)`
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// .form_link_to("Update Settings", "admin", "set_chunk_size")
    /// // Generates: [Update Settings](form:@admin:set_chunk_size)
    /// ```
    pub fn form_link_to(mut self, text: &str, alias: &str, method: &str) -> Self {
        self.parts.push_back(Bytes::from_slice(self.env, b"["));
        self.parts
            .push_back(Bytes::from_slice(self.env, text.as_bytes()));
        self.parts
            .push_back(Bytes::from_slice(self.env, b"](form:@"));
        self.parts
            .push_back(Bytes::from_slice(self.env, alias.as_bytes()));
        self.parts.push_back(Bytes::from_slice(self.env, b":"));
        self.parts
            .push_back(Bytes::from_slice(self.env, method.as_bytes()));
        self.parts.push_back(Bytes::from_slice(self.env, b")"));
        self
    }

    /// Add a tx: link targeting a specific contract via registry alias.
    ///
    /// Creates: `[text](tx:@alias:method args)`
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// .tx_link_to("Flag Post", "content", "flag_reply", r#"{"id":123}"#)
    /// // Generates: [Flag Post](tx:@content:flag_reply {"id":123})
    /// ```
    pub fn tx_link_to(mut self, text: &str, alias: &str, method: &str, args: &str) -> Self {
        self.parts.push_back(Bytes::from_slice(self.env, b"["));
        self.parts
            .push_back(Bytes::from_slice(self.env, text.as_bytes()));
        self.parts.push_back(Bytes::from_slice(self.env, b"](tx:@"));
        self.parts
            .push_back(Bytes::from_slice(self.env, alias.as_bytes()));
        self.parts.push_back(Bytes::from_slice(self.env, b":"));
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
        self.parts.push_back(Bytes::from_slice(self.env, b"]\n> "));
        self.parts
            .push_back(Bytes::from_slice(self.env, content.as_bytes()));
        self.parts.push_back(Bytes::from_slice(self.env, b"\n\n"));
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
        self.parts.push_back(Bytes::from_slice(self.env, b"|||\n"));
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
        self.parts.push_back(Bytes::from_slice(self.env, b"\"}}"));
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
        self.parts.push_back(Bytes::from_slice(self.env, b"\"}}"));
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

    /// Add an input element with a pre-populated value.
    ///
    /// Creates: `<input name="name" placeholder="placeholder" value="value" />`
    ///
    /// Use this when editing existing data so users can see and modify the current value.
    pub fn input_with_value(mut self, name: &str, placeholder: &str, value: &str) -> Self {
        self.parts
            .push_back(Bytes::from_slice(self.env, b"<input name=\""));
        self.parts
            .push_back(Bytes::from_slice(self.env, name.as_bytes()));
        self.parts
            .push_back(Bytes::from_slice(self.env, b"\" placeholder=\""));
        self.parts
            .push_back(Bytes::from_slice(self.env, placeholder.as_bytes()));
        self.parts
            .push_back(Bytes::from_slice(self.env, b"\" value=\""));
        self.parts
            .push_back(Bytes::from_slice(self.env, value.as_bytes()));
        self.parts
            .push_back(Bytes::from_slice(self.env, b"\" />\n"));
        self
    }

    /// Add an input element with a pre-populated value from a soroban String.
    ///
    /// Creates: `<input name="name" placeholder="placeholder" value="value" />`
    ///
    /// Use this when editing existing data so users can see and modify the current value.
    pub fn input_with_value_string(mut self, name: &str, placeholder: &str, value: &String) -> Self {
        self.parts
            .push_back(Bytes::from_slice(self.env, b"<input name=\""));
        self.parts
            .push_back(Bytes::from_slice(self.env, name.as_bytes()));
        self.parts
            .push_back(Bytes::from_slice(self.env, b"\" placeholder=\""));
        self.parts
            .push_back(Bytes::from_slice(self.env, placeholder.as_bytes()));
        self.parts
            .push_back(Bytes::from_slice(self.env, b"\" value=\""));
        self.parts.push_back(string_to_bytes(self.env, value));
        self.parts
            .push_back(Bytes::from_slice(self.env, b"\" />\n"));
        self
    }

    /// Add a hidden input element.
    ///
    /// Creates: `<input type="hidden" name="name" value="value" />`
    ///
    /// Useful for passing data with form submissions that shouldn't be visible to users.
    pub fn hidden_input(mut self, name: &str, value: &str) -> Self {
        self.parts.push_back(Bytes::from_slice(
            self.env,
            b"<input type=\"hidden\" name=\"",
        ));
        self.parts
            .push_back(Bytes::from_slice(self.env, name.as_bytes()));
        self.parts
            .push_back(Bytes::from_slice(self.env, b"\" value=\""));
        self.parts
            .push_back(Bytes::from_slice(self.env, value.as_bytes()));
        self.parts
            .push_back(Bytes::from_slice(self.env, b"\" />\n"));
        self
    }

    /// Add a redirect instruction for form submission.
    ///
    /// After successful transaction, the viewer will navigate to this path.
    /// Must be called within a form (between form_start and form_end).
    ///
    /// Creates: `<input type="hidden" name="_redirect" value="path" />`
    ///
    /// # Arguments
    /// * `path` - The path to navigate to after successful form submission
    ///
    /// # Example
    /// ```rust,ignore
    /// builder
    ///     .form_start("tx:create_thread", "POST")
    ///     .redirect("/b/0")  // Go back to board after creating thread
    ///     .input("title", "Enter title")
    ///     .button("submit", "Create")
    ///     .form_end()
    /// ```
    pub fn redirect(self, path: &str) -> Self {
        self.hidden_input("_redirect", path)
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

    /// Add a textarea element with a pre-populated value.
    ///
    /// Creates: `<textarea name="name" rows="N" placeholder="placeholder">value</textarea>`
    ///
    /// Use this when editing existing data so users can see and modify the current value.
    pub fn textarea_with_value(mut self, name: &str, rows: u8, placeholder: &str, value: &str) -> Self {
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
            .push_back(Bytes::from_slice(self.env, b"\">"));
        self.parts
            .push_back(Bytes::from_slice(self.env, value.as_bytes()));
        self.parts
            .push_back(Bytes::from_slice(self.env, b"</textarea>\n"));
        self
    }

    /// Add a textarea element with a pre-populated value from a soroban String.
    ///
    /// Creates: `<textarea name="name" rows="N" placeholder="placeholder">value</textarea>`
    ///
    /// Use this when editing existing data so users can see and modify the current value.
    pub fn textarea_with_value_string(
        mut self,
        name: &str,
        rows: u8,
        placeholder: &str,
        value: &String,
    ) -> Self {
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
            .push_back(Bytes::from_slice(self.env, b"\">"));
        self.parts.push_back(string_to_bytes(self.env, value));
        self.parts
            .push_back(Bytes::from_slice(self.env, b"</textarea>\n"));
        self
    }

    /// Add a textarea element with markdown editor hint.
    ///
    /// Creates: `<textarea name="name" data-editor="markdown" rows="N" placeholder="placeholder"></textarea>`
    ///
    /// When rendered in a viewer that supports it, this will display a rich markdown editor
    /// instead of a plain textarea. Falls back to a regular textarea in unsupported viewers.
    pub fn textarea_markdown(mut self, name: &str, rows: u8, placeholder: &str) -> Self {
        self.parts
            .push_back(Bytes::from_slice(self.env, b"<textarea name=\""));
        self.parts
            .push_back(Bytes::from_slice(self.env, name.as_bytes()));
        self.parts.push_back(Bytes::from_slice(
            self.env,
            b"\" data-editor=\"markdown\" rows=\"",
        ));
        self.parts.push_back(u32_to_bytes(self.env, rows as u32));
        self.parts
            .push_back(Bytes::from_slice(self.env, b"\" placeholder=\""));
        self.parts
            .push_back(Bytes::from_slice(self.env, placeholder.as_bytes()));
        self.parts
            .push_back(Bytes::from_slice(self.env, b"\"></textarea>\n"));
        self
    }

    /// Add a textarea element with markdown editor hint and a pre-populated value.
    ///
    /// Creates: `<textarea name="name" data-editor="markdown" rows="N" placeholder="placeholder">value</textarea>`
    ///
    /// When rendered in a viewer that supports it, this will display a rich markdown editor
    /// instead of a plain textarea. Falls back to a regular textarea in unsupported viewers.
    /// Use this when editing existing data so users can see and modify the current value.
    pub fn textarea_markdown_with_value(
        mut self,
        name: &str,
        rows: u8,
        placeholder: &str,
        value: &str,
    ) -> Self {
        self.parts
            .push_back(Bytes::from_slice(self.env, b"<textarea name=\""));
        self.parts
            .push_back(Bytes::from_slice(self.env, name.as_bytes()));
        self.parts.push_back(Bytes::from_slice(
            self.env,
            b"\" data-editor=\"markdown\" rows=\"",
        ));
        self.parts.push_back(u32_to_bytes(self.env, rows as u32));
        self.parts
            .push_back(Bytes::from_slice(self.env, b"\" placeholder=\""));
        self.parts
            .push_back(Bytes::from_slice(self.env, placeholder.as_bytes()));
        self.parts
            .push_back(Bytes::from_slice(self.env, b"\">"));
        self.parts
            .push_back(Bytes::from_slice(self.env, value.as_bytes()));
        self.parts
            .push_back(Bytes::from_slice(self.env, b"</textarea>\n"));
        self
    }

    /// Add a textarea element with markdown editor hint and a pre-populated value from a soroban String.
    ///
    /// Creates: `<textarea name="name" data-editor="markdown" rows="N" placeholder="placeholder">value</textarea>`
    ///
    /// When rendered in a viewer that supports it, this will display a rich markdown editor
    /// instead of a plain textarea. Falls back to a regular textarea in unsupported viewers.
    /// Use this when editing existing data so users can see and modify the current value.
    pub fn textarea_markdown_with_value_string(
        mut self,
        name: &str,
        rows: u8,
        placeholder: &str,
        value: &String,
    ) -> Self {
        self.parts
            .push_back(Bytes::from_slice(self.env, b"<textarea name=\""));
        self.parts
            .push_back(Bytes::from_slice(self.env, name.as_bytes()));
        self.parts.push_back(Bytes::from_slice(
            self.env,
            b"\" data-editor=\"markdown\" rows=\"",
        ));
        self.parts.push_back(u32_to_bytes(self.env, rows as u32));
        self.parts
            .push_back(Bytes::from_slice(self.env, b"\" placeholder=\""));
        self.parts
            .push_back(Bytes::from_slice(self.env, placeholder.as_bytes()));
        self.parts
            .push_back(Bytes::from_slice(self.env, b"\">"));
        self.parts.push_back(string_to_bytes(self.env, value));
        self.parts
            .push_back(Bytes::from_slice(self.env, b"</textarea>\n"));
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
            self.parts.push_back(Bytes::from_slice(self.env, b"- [x] "));
        } else {
            self.parts.push_back(Bytes::from_slice(self.env, b"- [ ] "));
        }
        self.parts
            .push_back(Bytes::from_slice(self.env, text.as_bytes()));
        self.parts.push_back(Bytes::from_slice(self.env, b"\n"));
        self
    }

    // ========================================================================
    // Blockquotes
    // ========================================================================

    /// Add a blockquote.
    ///
    /// Creates: `> text`
    pub fn blockquote(mut self, text: &str) -> Self {
        self.parts.push_back(Bytes::from_slice(self.env, b"> "));
        self.parts
            .push_back(Bytes::from_slice(self.env, text.as_bytes()));
        self.parts.push_back(Bytes::from_slice(self.env, b"\n\n"));
        self
    }

    // ========================================================================
    // HTML Containers (div/span)
    // ========================================================================

    /// Start a div element with CSS classes.
    ///
    /// Creates: `<div class="classes">`
    ///
    /// Must be paired with `div_end()` to close the element.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// builder
    ///     .div_start("reply reply-depth-1")
    ///     .paragraph("Nested reply content")
    ///     .div_end()
    /// ```
    pub fn div_start(mut self, classes: &str) -> Self {
        self.parts
            .push_back(Bytes::from_slice(self.env, b"<div class=\""));
        self.parts
            .push_back(Bytes::from_slice(self.env, classes.as_bytes()));
        self.parts.push_back(Bytes::from_slice(self.env, b"\">\n"));
        self
    }

    /// Start a div element with CSS classes and inline style.
    ///
    /// Creates: `<div class="classes" style="style">`
    pub fn div_start_styled(mut self, classes: &str, style: &str) -> Self {
        self.parts
            .push_back(Bytes::from_slice(self.env, b"<div class=\""));
        self.parts
            .push_back(Bytes::from_slice(self.env, classes.as_bytes()));
        self.parts
            .push_back(Bytes::from_slice(self.env, b"\" style=\""));
        self.parts
            .push_back(Bytes::from_slice(self.env, style.as_bytes()));
        self.parts.push_back(Bytes::from_slice(self.env, b"\">\n"));
        self
    }

    /// End a div element.
    ///
    /// Creates: `</div>`
    pub fn div_end(mut self) -> Self {
        self.parts
            .push_back(Bytes::from_slice(self.env, b"</div>\n"));
        self
    }

    /// Start a span element with CSS classes.
    ///
    /// Creates: `<span class="classes">`
    pub fn span_start(mut self, classes: &str) -> Self {
        self.parts
            .push_back(Bytes::from_slice(self.env, b"<span class=\""));
        self.parts
            .push_back(Bytes::from_slice(self.env, classes.as_bytes()));
        self.parts.push_back(Bytes::from_slice(self.env, b"\">"));
        self
    }

    /// End a span element.
    ///
    /// Creates: `</span>`
    pub fn span_end(mut self) -> Self {
        self.parts
            .push_back(Bytes::from_slice(self.env, b"</span>"));
        self
    }

    // ========================================================================
    // Progressive Loading / Continuation
    // ========================================================================

    /// Add a continuation marker for remaining content chunks.
    ///
    /// Used for progressive loading when content is split across multiple chunks.
    /// The viewer will fetch additional content starting from `from_index`.
    ///
    /// Creates: `{{continue collection="name" from=N total=T}}`
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// // In a contract with chunked comments:
    /// builder
    ///     .h2("Comments")
    ///     // ... render first 5 comments ...
    ///     .continuation("comments", 5, Some(50))  // 45 more to load
    /// ```
    pub fn continuation(mut self, collection: &str, from_index: u32, total: Option<u32>) -> Self {
        self.parts
            .push_back(Bytes::from_slice(self.env, b"{{continue collection=\""));
        self.parts
            .push_back(Bytes::from_slice(self.env, collection.as_bytes()));
        self.parts
            .push_back(Bytes::from_slice(self.env, b"\" from="));
        self.parts.push_back(u32_to_bytes(self.env, from_index));
        if let Some(t) = total {
            self.parts
                .push_back(Bytes::from_slice(self.env, b" total="));
            self.parts.push_back(u32_to_bytes(self.env, t));
        }
        self.parts.push_back(Bytes::from_slice(self.env, b"}}"));
        self
    }

    /// Add a chunk reference for lazy loading a specific chunk.
    ///
    /// The viewer will fetch and insert this chunk when rendering.
    ///
    /// Creates: `{{chunk collection="name" index=N}}`
    pub fn chunk_ref(mut self, collection: &str, index: u32) -> Self {
        self.parts
            .push_back(Bytes::from_slice(self.env, b"{{chunk collection=\""));
        self.parts
            .push_back(Bytes::from_slice(self.env, collection.as_bytes()));
        self.parts
            .push_back(Bytes::from_slice(self.env, b"\" index="));
        self.parts.push_back(u32_to_bytes(self.env, index));
        self.parts.push_back(Bytes::from_slice(self.env, b"}}"));
        self
    }

    /// Add a chunk reference with a loading placeholder.
    ///
    /// The placeholder text is displayed while the chunk is being loaded.
    ///
    /// Creates: `{{chunk collection="name" index=N placeholder="..."}}`
    pub fn chunk_ref_placeholder(
        mut self,
        collection: &str,
        index: u32,
        placeholder: &str,
    ) -> Self {
        self.parts
            .push_back(Bytes::from_slice(self.env, b"{{chunk collection=\""));
        self.parts
            .push_back(Bytes::from_slice(self.env, collection.as_bytes()));
        self.parts
            .push_back(Bytes::from_slice(self.env, b"\" index="));
        self.parts.push_back(u32_to_bytes(self.env, index));
        self.parts
            .push_back(Bytes::from_slice(self.env, b" placeholder=\""));
        self.parts
            .push_back(Bytes::from_slice(self.env, placeholder.as_bytes()));
        self.parts.push_back(Bytes::from_slice(self.env, b"\"}}"));
        self
    }

    /// Add a paginated continuation marker.
    ///
    /// Used for page-based progressive loading (e.g., comment threads, list views).
    ///
    /// Creates: `{{continue collection="name" page=N per_page=M total=T}}`
    pub fn continue_page(mut self, collection: &str, page: u32, per_page: u32, total: u32) -> Self {
        self.parts
            .push_back(Bytes::from_slice(self.env, b"{{continue collection=\""));
        self.parts
            .push_back(Bytes::from_slice(self.env, collection.as_bytes()));
        self.parts
            .push_back(Bytes::from_slice(self.env, b"\" page="));
        self.parts.push_back(u32_to_bytes(self.env, page));
        self.parts
            .push_back(Bytes::from_slice(self.env, b" per_page="));
        self.parts.push_back(u32_to_bytes(self.env, per_page));
        self.parts
            .push_back(Bytes::from_slice(self.env, b" total="));
        self.parts.push_back(u32_to_bytes(self.env, total));
        self.parts.push_back(Bytes::from_slice(self.env, b"}}"));
        self
    }

    /// Add a render continuation marker for waterfall loading.
    ///
    /// Used for progressive loading that triggers additional render() calls.
    /// The viewer will automatically fetch the specified path and insert
    /// the result inline.
    ///
    /// Creates: `{{render path="/some/path"}}`
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// // In a thread view, load first 10 replies, then continue loading more:
    /// builder
    ///     .h2("Replies")
    ///     // ... render first 10 replies ...
    ///     .render_continue("/b/1/t/0/replies/10")  // load more from offset 10
    /// ```
    pub fn render_continue(mut self, path: &str) -> Self {
        self.parts
            .push_back(Bytes::from_slice(self.env, b"{{render path=\""));
        self.parts
            .push_back(Bytes::from_slice(self.env, path.as_bytes()));
        self.parts.push_back(Bytes::from_slice(self.env, b"\"}}"));
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
        let output = MarkdownBuilder::new(&env).render_link("Home", "/").build();
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
    fn test_textarea_markdown() {
        let env = Env::default();
        let output = MarkdownBuilder::new(&env)
            .textarea_markdown("content", 10, "Enter markdown...")
            .build();
        // <textarea name="content" data-editor="markdown" rows="10" placeholder="Enter markdown..."></textarea>\n
        // Should contain the data-editor attribute
        assert!(output.len() > 60);
    }

    #[test]
    fn test_input_with_value() {
        let env = Env::default();
        let output = MarkdownBuilder::new(&env)
            .input_with_value("name", "Enter name", "John Doe")
            .build();
        // <input name="name" placeholder="Enter name" value="John Doe" />\n
        assert!(output.len() > 40);
    }

    #[test]
    fn test_textarea_with_value() {
        let env = Env::default();
        let output = MarkdownBuilder::new(&env)
            .textarea_with_value("bio", 5, "Enter bio", "Hello world")
            .build();
        // <textarea name="bio" rows="5" placeholder="Enter bio">Hello world</textarea>\n
        assert!(output.len() > 50);
    }

    #[test]
    fn test_textarea_markdown_with_value() {
        let env = Env::default();
        let output = MarkdownBuilder::new(&env)
            .textarea_markdown_with_value("content", 10, "Enter markdown...", "# Hello")
            .build();
        // <textarea name="content" data-editor="markdown" rows="10" placeholder="Enter markdown..."># Hello</textarea>\n
        assert!(output.len() > 70);
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

    #[test]
    fn test_blockquote() {
        let env = Env::default();
        let output = MarkdownBuilder::new(&env).blockquote("Quote text").build();
        // "> Quote text\n\n" = 14 bytes
        assert_eq!(output.len(), 14);
    }

    #[test]
    fn test_continuation() {
        let env = Env::default();
        let output = MarkdownBuilder::new(&env)
            .continuation("comments", 5, Some(50))
            .build();
        // {{continue collection="comments" from=5 total=50}}
        assert!(output.len() > 40);
    }

    #[test]
    fn test_continuation_no_total() {
        let env = Env::default();
        let output = MarkdownBuilder::new(&env)
            .continuation("data", 10, None)
            .build();
        // {{continue collection="data" from=10}}
        assert!(output.len() > 30);
    }

    #[test]
    fn test_chunk_ref() {
        let env = Env::default();
        let output = MarkdownBuilder::new(&env).chunk_ref("chunks", 3).build();
        // {{chunk collection="chunks" index=3}}
        assert!(output.len() > 30);
    }

    #[test]
    fn test_chunk_ref_placeholder() {
        let env = Env::default();
        let output = MarkdownBuilder::new(&env)
            .chunk_ref_placeholder("content", 7, "Loading...")
            .build();
        // {{chunk collection="content" index=7 placeholder="Loading..."}}
        assert!(output.len() > 50);
    }

    #[test]
    fn test_continue_page() {
        let env = Env::default();
        let output = MarkdownBuilder::new(&env)
            .continue_page("items", 2, 10, 47)
            .build();
        // {{continue collection="items" page=2 per_page=10 total=47}}
        assert!(output.len() > 50);
    }

    #[test]
    fn test_hidden_input() {
        let env = Env::default();
        let output = MarkdownBuilder::new(&env)
            .hidden_input("board_id", "42")
            .build();
        // <input type="hidden" name="board_id" value="42" />\n
        assert!(output.len() > 40);
    }

    #[test]
    fn test_redirect() {
        let env = Env::default();
        let output = MarkdownBuilder::new(&env).redirect("/b/0").build();
        // <input type="hidden" name="_redirect" value="/b/0" />\n
        assert!(output.len() > 45);
    }

    #[test]
    fn test_div_start_end() {
        let env = Env::default();
        let output = MarkdownBuilder::new(&env)
            .div_start("reply reply-depth-1")
            .text("Content")
            .div_end()
            .build();
        // <div class="reply reply-depth-1">\nContent</div>\n
        assert!(output.len() > 30);
    }

    #[test]
    fn test_div_start_styled() {
        let env = Env::default();
        let output = MarkdownBuilder::new(&env)
            .div_start_styled("container", "margin-left: 24px;")
            .text("Indented")
            .div_end()
            .build();
        // <div class="container" style="margin-left: 24px;">\nIndented</div>\n
        assert!(output.len() > 50);
    }

    #[test]
    fn test_span_start_end() {
        let env = Env::default();
        let output = MarkdownBuilder::new(&env)
            .span_start("highlight")
            .text("Important")
            .span_end()
            .build();
        // <span class="highlight">Important</span>
        assert!(output.len() > 30);
    }

    #[test]
    fn test_nested_divs() {
        let env = Env::default();
        let output = MarkdownBuilder::new(&env)
            .div_start("parent")
            .text("Parent content")
            .div_start("child")
            .text("Child content")
            .div_end()
            .div_end()
            .build();
        assert!(output.len() > 50);
    }
}
