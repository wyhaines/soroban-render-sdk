//! JSON document builder for constructing render output.
//!
//! Provides a fluent API for building JSON UI documents following the
//! `soroban-render-json-v1` format.
//!
//! # Example
//!
//! ```rust,ignore
//! use soroban_render_sdk::json::JsonDocument;
//!
//! let output = JsonDocument::new(&env, "My App")
//!     .heading(1, "Welcome")
//!     .text("Hello, World!")
//!     .divider()
//!     .build();
//! ```

use crate::bytes::{concat_bytes, escape_json_bytes, escape_json_string, u32_to_bytes};
use soroban_sdk::{Bytes, Env, String, Vec};

/// A builder for constructing JSON UI documents.
///
/// Outputs JSON following the `soroban-render-json-v1` format.
pub struct JsonDocument<'a> {
    env: &'a Env,
    parts: Vec<Bytes>,
    component_count: u32,
}

impl<'a> JsonDocument<'a> {
    /// Create a new JSON document with a title.
    pub fn new(env: &'a Env, title: &str) -> Self {
        let mut parts = Vec::new(env);
        parts.push_back(Bytes::from_slice(
            env,
            b"{\"format\":\"soroban-render-json-v1\",\"title\":\"",
        ));
        parts.push_back(escape_json_bytes(env, title.as_bytes()));
        parts.push_back(Bytes::from_slice(env, b"\",\"components\":["));

        Self {
            env,
            parts,
            component_count: 0,
        }
    }

    /// Add a comma separator if needed.
    fn maybe_comma(&mut self) {
        if self.component_count > 0 {
            self.parts.push_back(Bytes::from_slice(self.env, b","));
        }
        self.component_count += 1;
    }

    // ========================================================================
    // Basic Components
    // ========================================================================

    /// Add a heading component.
    pub fn heading(mut self, level: u8, text: &str) -> Self {
        self.maybe_comma();
        self.parts.push_back(Bytes::from_slice(
            self.env,
            b"{\"type\":\"heading\",\"level\":",
        ));
        self.parts.push_back(u32_to_bytes(self.env, level as u32));
        self.parts
            .push_back(Bytes::from_slice(self.env, b",\"text\":\""));
        self.parts
            .push_back(escape_json_bytes(self.env, text.as_bytes()));
        self.parts.push_back(Bytes::from_slice(self.env, b"\"}"));
        self
    }

    /// Add a heading with dynamic text from a String.
    pub fn heading_string(mut self, level: u8, text: &String) -> Self {
        self.maybe_comma();
        self.parts.push_back(Bytes::from_slice(
            self.env,
            b"{\"type\":\"heading\",\"level\":",
        ));
        self.parts.push_back(u32_to_bytes(self.env, level as u32));
        self.parts
            .push_back(Bytes::from_slice(self.env, b",\"text\":\""));
        self.parts.push_back(escape_json_string(self.env, text));
        self.parts.push_back(Bytes::from_slice(self.env, b"\"}"));
        self
    }

    /// Add a text component.
    pub fn text(mut self, content: &str) -> Self {
        self.maybe_comma();
        self.parts.push_back(Bytes::from_slice(
            self.env,
            b"{\"type\":\"text\",\"content\":\"",
        ));
        self.parts
            .push_back(escape_json_bytes(self.env, content.as_bytes()));
        self.parts.push_back(Bytes::from_slice(self.env, b"\"}"));
        self
    }

    /// Add a text component with dynamic content from a String.
    pub fn text_string(mut self, content: &String) -> Self {
        self.maybe_comma();
        self.parts.push_back(Bytes::from_slice(
            self.env,
            b"{\"type\":\"text\",\"content\":\"",
        ));
        self.parts.push_back(escape_json_string(self.env, content));
        self.parts.push_back(Bytes::from_slice(self.env, b"\"}"));
        self
    }

    /// Add a divider component.
    pub fn divider(mut self) -> Self {
        self.maybe_comma();
        self.parts
            .push_back(Bytes::from_slice(self.env, b"{\"type\":\"divider\"}"));
        self
    }

    // ========================================================================
    // Form
    // ========================================================================

    /// Start a form component. Returns a FormBuilder.
    pub fn form(mut self, action: &str) -> FormBuilder<'a> {
        self.maybe_comma();
        self.parts.push_back(Bytes::from_slice(
            self.env,
            b"{\"type\":\"form\",\"action\":\"",
        ));
        self.parts
            .push_back(escape_json_bytes(self.env, action.as_bytes()));
        self.parts
            .push_back(Bytes::from_slice(self.env, b"\",\"fields\":["));

        FormBuilder {
            doc: self,
            field_count: 0,
        }
    }

    // ========================================================================
    // Navigation
    // ========================================================================

    /// Start a navigation component.
    pub fn nav_start(mut self) -> Self {
        self.maybe_comma();
        self.parts.push_back(Bytes::from_slice(
            self.env,
            b"{\"type\":\"navigation\",\"items\":[",
        ));
        self
    }

    /// Add a navigation item. Must be called between nav_start and nav_end.
    /// Set first=true for the first item (no comma prefix).
    pub fn nav_item(mut self, label: &str, path: &str, active: bool, first: bool) -> Self {
        if !first {
            self.parts.push_back(Bytes::from_slice(self.env, b","));
        }
        self.parts
            .push_back(Bytes::from_slice(self.env, b"{\"label\":\""));
        self.parts
            .push_back(escape_json_bytes(self.env, label.as_bytes()));
        self.parts
            .push_back(Bytes::from_slice(self.env, b"\",\"path\":\""));
        self.parts
            .push_back(escape_json_bytes(self.env, path.as_bytes()));
        self.parts.push_back(Bytes::from_slice(self.env, b"\""));
        if active {
            self.parts
                .push_back(Bytes::from_slice(self.env, b",\"active\":true"));
        }
        self.parts.push_back(Bytes::from_slice(self.env, b"}"));
        self
    }

    /// End a navigation component.
    pub fn nav_end(mut self) -> Self {
        self.parts.push_back(Bytes::from_slice(self.env, b"]}"));
        self
    }

    // ========================================================================
    // Charts
    // ========================================================================

    /// Start a pie chart component.
    pub fn pie_chart_start(mut self, title: &str) -> Self {
        self.maybe_comma();
        self.parts.push_back(Bytes::from_slice(
            self.env,
            b"{\"type\":\"chart\",\"chartType\":\"pie\",\"title\":\"",
        ));
        self.parts
            .push_back(escape_json_bytes(self.env, title.as_bytes()));
        self.parts
            .push_back(Bytes::from_slice(self.env, b"\",\"data\":["));
        self
    }

    /// Add a pie chart slice. Set first=true for the first slice.
    pub fn pie_slice(mut self, label: &str, value: u32, color: &str, first: bool) -> Self {
        if !first {
            self.parts.push_back(Bytes::from_slice(self.env, b","));
        }
        self.parts
            .push_back(Bytes::from_slice(self.env, b"{\"label\":\""));
        self.parts
            .push_back(escape_json_bytes(self.env, label.as_bytes()));
        self.parts
            .push_back(Bytes::from_slice(self.env, b"\",\"value\":"));
        self.parts.push_back(u32_to_bytes(self.env, value));
        self.parts
            .push_back(Bytes::from_slice(self.env, b",\"color\":\""));
        self.parts
            .push_back(escape_json_bytes(self.env, color.as_bytes()));
        self.parts.push_back(Bytes::from_slice(self.env, b"\"}"));
        self
    }

    /// End a pie chart component.
    pub fn pie_chart_end(mut self) -> Self {
        self.parts.push_back(Bytes::from_slice(self.env, b"]}"));
        self
    }

    /// Add a gauge chart component.
    pub fn gauge(mut self, value: u32, max: u32, label: &str) -> Self {
        self.maybe_comma();
        self.parts.push_back(Bytes::from_slice(
            self.env,
            b"{\"type\":\"chart\",\"chartType\":\"gauge\",\"value\":",
        ));
        self.parts.push_back(u32_to_bytes(self.env, value));
        self.parts
            .push_back(Bytes::from_slice(self.env, b",\"max\":"));
        self.parts.push_back(u32_to_bytes(self.env, max));
        self.parts
            .push_back(Bytes::from_slice(self.env, b",\"label\":\""));
        self.parts
            .push_back(escape_json_bytes(self.env, label.as_bytes()));
        self.parts.push_back(Bytes::from_slice(self.env, b"\"}"));
        self
    }

    // ========================================================================
    // Container
    // ========================================================================

    /// Start a container component.
    pub fn container_start(mut self, class_name: &str) -> Self {
        self.maybe_comma();
        self.parts.push_back(Bytes::from_slice(
            self.env,
            b"{\"type\":\"container\",\"className\":\"",
        ));
        self.parts
            .push_back(escape_json_bytes(self.env, class_name.as_bytes()));
        self.parts
            .push_back(Bytes::from_slice(self.env, b"\",\"components\":["));
        // Reset component count for nested components
        self.component_count = 0;
        self
    }

    /// End a container component.
    pub fn container_end(mut self) -> Self {
        self.parts.push_back(Bytes::from_slice(self.env, b"]}"));
        self.component_count = 1; // Mark that we have content after container
        self
    }

    // ========================================================================
    // Task Component
    // ========================================================================

    /// Add a task component with actions.
    pub fn task(mut self, id: u32, text: &str, completed: bool) -> TaskBuilder<'a> {
        self.maybe_comma();
        self.parts
            .push_back(Bytes::from_slice(self.env, b"{\"type\":\"task\",\"id\":"));
        self.parts.push_back(u32_to_bytes(self.env, id));
        self.parts
            .push_back(Bytes::from_slice(self.env, b",\"text\":\""));
        self.parts
            .push_back(escape_json_bytes(self.env, text.as_bytes()));
        self.parts
            .push_back(Bytes::from_slice(self.env, b"\",\"completed\":"));
        if completed {
            self.parts.push_back(Bytes::from_slice(self.env, b"true"));
        } else {
            self.parts.push_back(Bytes::from_slice(self.env, b"false"));
        }
        self.parts
            .push_back(Bytes::from_slice(self.env, b",\"actions\":["));

        TaskBuilder {
            doc: self,
            action_count: 0,
        }
    }

    /// Add a task component with dynamic text.
    pub fn task_string(mut self, id: u32, text: &String, completed: bool) -> TaskBuilder<'a> {
        self.maybe_comma();
        self.parts
            .push_back(Bytes::from_slice(self.env, b"{\"type\":\"task\",\"id\":"));
        self.parts.push_back(u32_to_bytes(self.env, id));
        self.parts
            .push_back(Bytes::from_slice(self.env, b",\"text\":\""));
        self.parts.push_back(escape_json_string(self.env, text));
        self.parts
            .push_back(Bytes::from_slice(self.env, b"\",\"completed\":"));
        if completed {
            self.parts.push_back(Bytes::from_slice(self.env, b"true"));
        } else {
            self.parts.push_back(Bytes::from_slice(self.env, b"false"));
        }
        self.parts
            .push_back(Bytes::from_slice(self.env, b",\"actions\":["));

        TaskBuilder {
            doc: self,
            action_count: 0,
        }
    }

    // ========================================================================
    // Build
    // ========================================================================

    /// Build the final JSON Bytes output.
    pub fn build(mut self) -> Bytes {
        self.parts.push_back(Bytes::from_slice(self.env, b"]}"));
        concat_bytes(self.env, &self.parts)
    }
}

/// Builder for form fields.
pub struct FormBuilder<'a> {
    doc: JsonDocument<'a>,
    field_count: u32,
}

impl<'a> FormBuilder<'a> {
    /// Add a comma separator if needed.
    fn maybe_comma(&mut self) {
        if self.field_count > 0 {
            self.doc
                .parts
                .push_back(Bytes::from_slice(self.doc.env, b","));
        }
        self.field_count += 1;
    }

    /// Add a text field.
    pub fn text_field(mut self, name: &str, placeholder: &str, required: bool) -> Self {
        self.maybe_comma();
        self.doc
            .parts
            .push_back(Bytes::from_slice(self.doc.env, b"{\"name\":\""));
        self.doc
            .parts
            .push_back(escape_json_bytes(self.doc.env, name.as_bytes()));
        self.doc.parts.push_back(Bytes::from_slice(
            self.doc.env,
            b"\",\"type\":\"text\",\"placeholder\":\"",
        ));
        self.doc
            .parts
            .push_back(escape_json_bytes(self.doc.env, placeholder.as_bytes()));
        self.doc
            .parts
            .push_back(Bytes::from_slice(self.doc.env, b"\""));
        if required {
            self.doc
                .parts
                .push_back(Bytes::from_slice(self.doc.env, b",\"required\":true"));
        }
        self.doc
            .parts
            .push_back(Bytes::from_slice(self.doc.env, b"}"));
        self
    }

    /// Add a textarea field.
    pub fn textarea_field(mut self, name: &str, placeholder: &str) -> Self {
        self.maybe_comma();
        self.doc
            .parts
            .push_back(Bytes::from_slice(self.doc.env, b"{\"name\":\""));
        self.doc
            .parts
            .push_back(escape_json_bytes(self.doc.env, name.as_bytes()));
        self.doc.parts.push_back(Bytes::from_slice(
            self.doc.env,
            b"\",\"type\":\"textarea\",\"placeholder\":\"",
        ));
        self.doc
            .parts
            .push_back(escape_json_bytes(self.doc.env, placeholder.as_bytes()));
        self.doc
            .parts
            .push_back(Bytes::from_slice(self.doc.env, b"\"}"));
        self
    }

    /// Complete the form with a submit label.
    pub fn submit(mut self, label: &str) -> JsonDocument<'a> {
        self.doc
            .parts
            .push_back(Bytes::from_slice(self.doc.env, b"],\"submitLabel\":\""));
        self.doc
            .parts
            .push_back(escape_json_bytes(self.doc.env, label.as_bytes()));
        self.doc
            .parts
            .push_back(Bytes::from_slice(self.doc.env, b"\"}"));
        self.doc
    }
}

/// Builder for task actions.
pub struct TaskBuilder<'a> {
    doc: JsonDocument<'a>,
    action_count: u32,
}

impl<'a> TaskBuilder<'a> {
    /// Add a comma separator if needed.
    fn maybe_comma(&mut self) {
        if self.action_count > 0 {
            self.doc
                .parts
                .push_back(Bytes::from_slice(self.doc.env, b","));
        }
        self.action_count += 1;
    }

    /// Add a transaction action.
    pub fn tx_action(mut self, method: &str, id: u32, label: &str) -> Self {
        self.maybe_comma();
        self.doc.parts.push_back(Bytes::from_slice(
            self.doc.env,
            b"{\"type\":\"tx\",\"method\":\"",
        ));
        self.doc
            .parts
            .push_back(escape_json_bytes(self.doc.env, method.as_bytes()));
        self.doc
            .parts
            .push_back(Bytes::from_slice(self.doc.env, b"\",\"args\":{\"id\":"));
        self.doc.parts.push_back(u32_to_bytes(self.doc.env, id));
        self.doc
            .parts
            .push_back(Bytes::from_slice(self.doc.env, b"},\"label\":\""));
        self.doc
            .parts
            .push_back(escape_json_bytes(self.doc.env, label.as_bytes()));
        self.doc
            .parts
            .push_back(Bytes::from_slice(self.doc.env, b"\"}"));
        self
    }

    /// Complete the task.
    pub fn end(mut self) -> JsonDocument<'a> {
        self.doc
            .parts
            .push_back(Bytes::from_slice(self.doc.env, b"]}"));
        self.doc
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_document() {
        let env = Env::default();
        let output = JsonDocument::new(&env, "Test").build();
        // Should contain format, title, and empty components array
        assert!(output.len() > 50);
    }

    #[test]
    fn test_heading() {
        let env = Env::default();
        let output = JsonDocument::new(&env, "Test").heading(1, "Hello").build();
        assert!(output.len() > 60);
    }

    #[test]
    fn test_text() {
        let env = Env::default();
        let output = JsonDocument::new(&env, "Test")
            .text("Hello, World!")
            .build();
        assert!(output.len() > 60);
    }

    #[test]
    fn test_divider() {
        let env = Env::default();
        let output = JsonDocument::new(&env, "Test").divider().build();
        assert!(output.len() > 50);
    }

    #[test]
    fn test_multiple_components() {
        let env = Env::default();
        let output = JsonDocument::new(&env, "Test")
            .heading(1, "Title")
            .text("Content")
            .divider()
            .build();
        // Should have commas between components
        assert!(output.len() > 100);
    }

    #[test]
    fn test_form() {
        let env = Env::default();
        let output = JsonDocument::new(&env, "Test")
            .form("add_item")
            .text_field("name", "Enter name", true)
            .submit("Add")
            .build();
        assert!(output.len() > 100);
    }

    #[test]
    fn test_navigation() {
        let env = Env::default();
        let output = JsonDocument::new(&env, "Test")
            .nav_start()
            .nav_item("Home", "/", true, true)
            .nav_item("About", "/about", false, false)
            .nav_end()
            .build();
        assert!(output.len() > 100);
    }

    #[test]
    fn test_pie_chart() {
        let env = Env::default();
        let output = JsonDocument::new(&env, "Test")
            .pie_chart_start("Status")
            .pie_slice("Done", 5, "#22c55e", true)
            .pie_slice("Pending", 3, "#eab308", false)
            .pie_chart_end()
            .build();
        assert!(output.len() > 100);
    }

    #[test]
    fn test_gauge() {
        let env = Env::default();
        let output = JsonDocument::new(&env, "Test")
            .gauge(75, 100, "Progress")
            .build();
        assert!(output.len() > 80);
    }

    #[test]
    fn test_task() {
        let env = Env::default();
        let output = JsonDocument::new(&env, "Test")
            .task(1, "My Task", false)
            .tx_action("complete", 1, "Done")
            .tx_action("delete", 1, "Delete")
            .end()
            .build();
        assert!(output.len() > 150);
    }
}
