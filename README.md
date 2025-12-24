# Soroban Render SDK

A Rust SDK for building [Soroban Render](https://github.com/wyhaines/soroban-render) contracts with reduced boilerplate.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
soroban-render-sdk = "0.1.0"
```

## Quick Start

```rust
#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, String};
use soroban_render_sdk::prelude::*;

soroban_render!(markdown);

#[contract]
pub struct HelloContract;

#[contractimpl]
impl HelloContract {
    pub fn render(env: Env, _path: Option<String>, viewer: Option<Address>) -> Bytes {
        MarkdownBuilder::new(&env)
            .h1("Hello, World!")
            .paragraph("Welcome to Soroban Render.")
            .build()
    }
}
```

## Features

The SDK provides several modules that can be selectively included:

- **markdown** (default) - `MarkdownBuilder` for fluent markdown construction
- **json** (default) - `JsonDocument` for JSON UI format
- **router** (default) - Path matching and declarative routing

Disable defaults to reduce size:

```toml
[dependencies]
soroban-render-sdk = { version = "0.1.0", default-features = false, features = ["markdown"] }
```

## API Overview

### Metadata Macros

Declare render support:

```rust
// Shorthand for both contractmeta declarations
soroban_render!(markdown);
soroban_render!(json);
soroban_render!(markdown, json);
```

### MarkdownBuilder

Build markdown content with a fluent API:

```rust
let output = MarkdownBuilder::new(&env)
    .h1("Title")
    .paragraph("Content here.")
    .tip("This is a tip callout.")
    .render_link("Home", "/")
    .tx_link_id("Delete", "delete_item", 42)
    .form_link("Submit", "add_item")
    // Target specific contracts via registry alias
    .form_link_to("Update", "admin", "set_value")
    .tx_link_to("Flag", "content", "flag_post", r#"{"id":1}"#)
    .include("CONTRACT_ID", "header")
    .build();
```

### JsonDocument

Build JSON UI documents:

```rust
let output = JsonDocument::new(&env, "My App")
    .heading(1, "Welcome")
    .text("Hello, World!")
    .form("add_item")
        .text_field("name", "Enter name", true)
        .submit("Add")
    .divider()
    .build();
```

### Router

Declarative path-based routing:

```rust
pub fn render(env: Env, path: Option<String>, viewer: Option<Address>) -> Bytes {
    Router::new(&env, path)
        .handle(b"/", |_| render_home(&env))
        .or_handle(b"/tasks", |_| render_tasks(&env))
        .or_handle(b"/task/{id}", |req| {
            let id = req.get_var_u32(b"id").unwrap_or(0);
            render_task(&env, id)
        })
        .or_handle(b"/files/*", |req| {
            let path = req.get_wildcard().unwrap();
            render_file(&env, path)
        })
        .or_default(|_| render_home(&env))
}
```

### Registry (Multi-Contract Apps)

For applications with multiple contracts:

```rust
use soroban_render_sdk::registry::BaseRegistry;
use soroban_sdk::{symbol_short, Address, Env, Map};

// Initialize registry with contracts
let mut contracts = Map::new(&env);
contracts.set(symbol_short!("admin"), admin_address);
contracts.set(symbol_short!("content"), content_address);
BaseRegistry::init(&env, &admin, contracts);

// Look up by alias
let content = BaseRegistry::get_by_alias(&env, symbol_short!("content"));
```

### Byte Utilities

Low-level utilities for working with Bytes:

```rust
use soroban_render_sdk::bytes::*;

// Concatenate multiple Bytes
let result = concat_bytes(&env, &parts);

// Convert types to Bytes
let bytes = string_to_bytes(&env, &my_string);
let bytes = u32_to_bytes(&env, 42);
let bytes = i64_to_bytes(&env, -100);

// Escape for JSON
let escaped = escape_json_string(&env, &my_string);
```

## Comparison

### Before (Manual)

```rust
pub fn render(env: Env, _path: Option<String>, viewer: Option<Address>) -> Bytes {
    let mut parts: Vec<Bytes> = Vec::new(&env);

    match viewer {
        Some(_) => {
            parts.push_back(Bytes::from_slice(&env, b"# Hello, User!\n\n"));
            parts.push_back(Bytes::from_slice(&env, b"Your wallet is connected."));
        }
        None => {
            parts.push_back(Bytes::from_slice(&env, b"# Hello, World!\n\n"));
            parts.push_back(Bytes::from_slice(&env, b"Connect your wallet."));
        }
    };

    Self::concat_bytes(&env, &parts)
}

fn concat_bytes(env: &Env, parts: &Vec<Bytes>) -> Bytes {
    let mut result = Bytes::new(env);
    for part in parts.iter() { result.append(&part); }
    result
}
```

### After (With SDK)

```rust
pub fn render(env: Env, _path: Option<String>, viewer: Option<Address>) -> Bytes {
    let md = MarkdownBuilder::new(&env);

    match viewer {
        Some(_) => md.h1("Hello, User!").paragraph("Your wallet is connected."),
        None => md.h1("Hello, World!").paragraph("Connect your wallet."),
    }.build()
}
```

## Documentation

Complete documentation is available in the main Soroban Render repository:

- **[Rust SDK Reference](https://github.com/wyhaines/soroban-render/blob/main/docs/rust-sdk.md)** - Complete API documentation
- **[Router Guide](https://github.com/wyhaines/soroban-render/blob/main/docs/router-guide.md)** - Path matching patterns
- **[Examples](https://github.com/wyhaines/soroban-render/blob/main/docs/examples.md)** - Contract walkthroughs
- **[Getting Started](https://github.com/wyhaines/soroban-render/blob/main/docs/getting-started.md)** - First contract tutorial
- **[Testing](https://github.com/wyhaines/soroban-render/blob/main/docs/testing.md)** - Testing strategies
- **[Best Practices](https://github.com/wyhaines/soroban-render/blob/main/docs/best-practices.md)** - Design patterns
- **[Troubleshooting](https://github.com/wyhaines/soroban-render/blob/main/docs/troubleshooting.md)** - Common issues

## License

Apache 2.0

## Related

- [Soroban Render](https://github.com/wyhaines/soroban-render) - The main project with viewer and documentation
- [Soroban SDK](https://github.com/stellar/rs-soroban-sdk) - Stellar's Soroban SDK
