//! # Soroban Render SDK
//!
//! A library for building Soroban Render contracts with reduced boilerplate.
//!
//! ## Quick Start
//!
//! ```rust,ignore
//! #![no_std]
//! use soroban_sdk::{contract, contractimpl, Address, Env, String};
//! use soroban_render_sdk::prelude::*;
//!
//! soroban_render!(markdown);
//!
//! #[contract]
//! pub struct HelloContract;
//!
//! #[contractimpl]
//! impl HelloContract {
//!     pub fn render(env: Env, _path: Option<String>, viewer: Option<Address>) -> Bytes {
//!         MarkdownBuilder::new(&env)
//!             .h1("Hello, World!")
//!             .paragraph("Welcome to Soroban Render.")
//!             .build()
//!     }
//! }
//! ```
//!
//! ## Features
//!
//! - `markdown` - MarkdownBuilder for markdown output (default)
//! - `json` - JsonDocument builder for JSON UI format (default)
//! - `router` - Router and path utilities (default)
//! - `styles` - StyleBuilder for CSS stylesheet output (default)

#![no_std]

// Core bytes module - always available
pub mod bytes;

// Metadata macros - always available
mod metadata;

// Feature-gated modules
#[cfg(feature = "markdown")]
pub mod markdown;

#[cfg(feature = "json")]
pub mod json;

#[cfg(feature = "router")]
pub mod router;

#[cfg(feature = "styles")]
pub mod styles;

// Prelude for convenient imports
pub mod prelude;

// Note: Metadata macros (soroban_render!, render_v1!, render_formats!)
// are automatically exported at crate root via #[macro_export]
