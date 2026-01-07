# Soroban Render SDK

[![CI](https://github.com/wyhaines/soroban-render-sdk/actions/workflows/ci.yml/badge.svg)](https://github.com/wyhaines/soroban-render-sdk/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/soroban-render-sdk.svg)](https://crates.io/crates/soroban-render-sdk)
[![License](https://img.shields.io/crates/l/soroban-render-sdk.svg)](LICENSE)

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
- **styles** (default) - `StyleBuilder` for CSS stylesheet generation
- **registry** - `BaseRegistry` for multi-contract applications

Disable defaults to reduce size:

```toml
[dependencies]
soroban-render-sdk = { version = "0.1.0", default-features = false, features = ["markdown"] }
```

Enable registry for multi-contract apps:

```toml
[dependencies]
soroban-render-sdk = { version = "0.1.0", features = ["registry"] }
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

### StyleBuilder

Build CSS stylesheets with a fluent API:

```rust
use soroban_render_sdk::styles::StyleBuilder;

let css = StyleBuilder::new(&env)
    // CSS variables
    .root_vars_start()
    .var("primary", "#0066cc")
    .var("bg", "#ffffff")
    .var("text", "#333333")
    .root_vars_end()

    // Standard rules
    .rule("body", "background: var(--bg); color: var(--text);")

    // Multi-line rules
    .rule_start("h1")
    .prop("color", "var(--primary)")
    .prop("font-size", "2rem")
    .prop("margin-bottom", "1rem")
    .rule_end()

    // Dark mode override
    .dark_mode_start()
    .rule_start(":root")
    .prop("--bg", "#1a1a1a")
    .prop("--text", "#e0e0e0")
    .rule_end()
    .media_end()

    // Responsive breakpoints
    .breakpoint_max(768)
    .rule("h1", "font-size: 1.5rem;")
    .media_end()

    .build();
```

#### StyleBuilder Methods

| Method | Description |
|--------|-------------|
| `root_var(name, value)` | Single CSS variable: `:root { --name: value; }` |
| `root_vars_start()` / `root_vars_end()` | Start/end a :root block |
| `var(name, value)` | Add variable inside :root block |
| `rule(selector, properties)` | Inline rule: `selector { properties }` |
| `rule_start(selector)` / `rule_end()` | Start/end a multi-line rule |
| `prop(property, value)` | Add property inside rule block |
| `media_start(condition)` / `media_end()` | Generic media query |
| `dark_mode_start()` | `@media (prefers-color-scheme: dark)` |
| `light_mode_start()` | `@media (prefers-color-scheme: light)` |
| `breakpoint_min(px)` | Mobile-first: `@media (min-width: Npx)` |
| `breakpoint_max(px)` | Desktop-first: `@media (max-width: Npx)` |
| `comment(text)` | CSS comment: `/* text */` |
| `raw(css)` | Insert raw CSS string |

### Registry (Multi-Contract Apps)

For applications with multiple contracts, the SDK provides registry support that enables the viewer's `form:@alias:method` and `tx:@alias:method` protocols.

#### Basic Usage

```rust
use soroban_render_sdk::registry::BaseRegistry;
use soroban_sdk::{symbol_short, Address, Env, Map};

// Initialize registry with admin and contract aliases
let mut contracts = Map::new(&env);
contracts.set(symbol_short!("theme"), theme_address);
contracts.set(symbol_short!("content"), content_address);
contracts.set(symbol_short!("perms"), permissions_address);
BaseRegistry::init(&env, &admin, contracts);

// Look up contracts by alias
let content = BaseRegistry::get_by_alias(&env, symbol_short!("content"));

// Get all registered contracts
let all = BaseRegistry::get_all(&env);

// Admin can register new contracts later
BaseRegistry::register(&env, symbol_short!("cache"), cache_address);

// Admin can remove contracts
BaseRegistry::unregister(&env, symbol_short!("cache"));
```

#### BaseRegistry API

| Method | Description |
|--------|-------------|
| `init(env, admin, contracts)` | Initialize with admin and initial contract map. Panics if already initialized. |
| `register(env, alias, address)` | Register or update a contract alias. Requires admin auth. |
| `get_by_alias(env, alias)` | Look up contract by alias. Returns `Option<Address>`. |
| `get_all(env)` | Get all registered contracts as `Map<Symbol, Address>`. |
| `get_admin(env)` | Get the registry admin address. |
| `unregister(env, alias)` | Remove a contract alias. Requires admin auth. |

#### Implementing a Registry Contract

To use the registry with the viewer, your registry contract must expose a `get_contract_by_alias` function:

```rust
#[contract]
pub struct MyRegistry;

#[contractimpl]
impl MyRegistry {
    pub fn initialize(env: Env, admin: Address, theme: Address, content: Address) {
        let mut contracts = Map::new(&env);
        contracts.set(symbol_short!("theme"), theme);
        contracts.set(symbol_short!("content"), content);
        BaseRegistry::init(&env, &admin, contracts);
    }

    // Required: The viewer calls this to resolve @alias references
    pub fn get_contract_by_alias(env: Env, alias: Symbol) -> Option<Address> {
        // Handle self-reference
        if alias == symbol_short!("registry") {
            return Some(env.current_contract_address());
        }
        BaseRegistry::get_by_alias(&env, alias)
    }
}
```

#### Emitting Aliases

The SDK provides `emit_aliases()` to generate `{{aliases ...}}` tags that enable include resolution with friendly names:

```rust
use soroban_render_sdk::registry::BaseRegistry;

// In your render function:
let aliases = BaseRegistry::emit_aliases(&env);
MarkdownBuilder::new(&env)
    .raw(aliases)  // {{aliases theme=CXYZ... content=CABC... }}
    .h1("My Page")
    // ...
```

For cross-contract calls, expose a public function:

```rust
pub fn render_aliases(env: Env) -> Bytes {
    BaseRegistry::emit_aliases(&env)
}
```

Other contracts can then fetch and emit aliases in their render output:

```rust
fn fetch_aliases(env: &Env) -> Bytes {
    let registry: Address = /* get registry address */;
    let args: Vec<Val> = Vec::new(env);
    env.try_invoke_contract::<Bytes, soroban_sdk::Error>(
        &registry,
        &Symbol::new(env, "render_aliases"),
        args,
    ).ok().and_then(|r| r.ok()).unwrap_or(Bytes::new(env))
}
```

#### Using Registry Aliases in Links

Once you have a registry, use `form_link_to` and `tx_link_to` to target specific contracts:

```rust
// Form targeting the content contract
builder.form_link_to("Post Reply", "content", "create_reply")
// Generates: [Post Reply](form:@content:create_reply)

// Transaction targeting the permissions contract
builder.tx_link_to("Flag", "perms", "flag_content", r#"{"id":1}"#)
// Generates: [Flag](tx:@perms:flag_content {"id":1})
```

The viewer resolves `@content` and `@perms` by calling your registry's `get_contract_by_alias` function.

### HTML Containers (Layout Elements)

For complex layouts, use div and span containers with CSS classes:

```rust
// Nested divs with classes
builder
    .div_start("thread")
    .h2("Thread Title")
    .div_start("replies")
    .paragraph("Reply content here...")
    .div_end()  // close replies
    .div_end()  // close thread

// Styled div with inline CSS
builder.div_start_styled("indented", "margin-left: 24px;")
    .paragraph("Indented content")
    .div_end()

// Inline spans
builder
    .text("Status: ")
    .span_start("status-badge success")
    .text("Active")
    .span_end()
```

### Progressive Loading

For large content that exceeds execution limits, use progressive loading patterns:

#### Continuation Markers

Use `continuation` when content is split into indexed chunks:

```rust
// Render first 10 items, signal there are more
builder
    .h2("Comments")
    // ... render comments 0-9 ...
    .continuation("comments", 10, Some(50))  // 40 more to load
// Generates: {{continue collection="comments" from=10 total=50}}
```

#### Render Continue (Waterfall Loading)

Use `render_continue` to trigger additional render() calls with a path:

```rust
// Load first batch of replies, then continue loading
builder
    .h2("Replies")
    // ... render first 10 replies ...
    .render_continue("/b/1/t/0/replies/10")  // fetch more from offset 10
// Generates: {{render path="/b/1/t/0/replies/10"}}
```

The viewer automatically fetches the path and inserts the result inline.

#### Chunk References

Reference specific chunks for lazy loading:

```rust
// Reference chunk by index
builder.chunk_ref("content", 3)
// Generates: {{chunk collection="content" index=3}}

// With loading placeholder
builder.chunk_ref_placeholder("body", 0, "Loading...")
// Generates: {{chunk collection="body" index=0 placeholder="Loading..."}}
```

#### Page-Based Continuation

For paginated content:

```rust
builder.continue_page("posts", 2, 10, 47)  // page 2, 10 per page, 47 total
// Generates: {{continue collection="posts" page=2 per_page=10 total=47}}
```

### Byte Utilities

The bytes module provides utilities for working with `Bytes` in Soroban contracts. This includes string conversion, JSON escaping, and comprehensive number-to-string conversion functions for all Rust integer types.

```rust
use soroban_render_sdk::bytes::*;

// Concatenate multiple Bytes
let result = concat_bytes(&env, &parts);

// Convert String to Bytes
let bytes = string_to_bytes(&env, &my_string);

// Escape for JSON output
let escaped = escape_json_string(&env, &my_string);
```

### Number Conversion

The SDK provides bidirectional conversion between numeric types and their `Bytes` string representations. These functions support all standard Rust integer types plus Soroban's `U256` and `I256`.

#### Converting Numbers to Decimal Strings

Convert any numeric type to its decimal string representation as `Bytes`. Signed types automatically handle negative values.

```rust
let bytes = u64_to_bytes(&env, 12345);
// bytes contains "12345"

let bytes = i64_to_bytes(&env, -42);
// bytes contains "-42"
```

The full set of decimal conversion functions covers `u32`, `i32`, `u64`, `i64`, `u128`, `i128`, `U256`, and `I256`.

#### Converting Numbers to Hexadecimal

Convert numeric types to lowercase hexadecimal with a `0x` prefix. Negative values use a `-0x` prefix.

```rust
let bytes = u64_to_hex(&env, 255);
// bytes contains "0xff"

let bytes = i32_to_hex(&env, -16);
// bytes contains "-0x10"
```

#### Parsing Decimal Strings

Parse a `Bytes` string back to a numeric type. These functions return `Option<T>` to handle invalid input safely.

```rust
let bytes = Bytes::from_slice(&env, b"12345");
let value = bytes_to_u64(&bytes);
// value is Some(12345)

let invalid = Bytes::from_slice(&env, b"abc");
let value = bytes_to_u64(&invalid);
// value is None
```

Parsing uses checked arithmetic to detect overflow. Values that exceed the target type's range return `None`.

#### Parsing Hexadecimal Strings

Parse hexadecimal strings to numeric types. The `0x` prefix is optional and parsing is case-insensitive.

```rust
let bytes = Bytes::from_slice(&env, b"0xFF");
let value = hex_to_u32(&bytes);
// value is Some(255)

let bytes = Bytes::from_slice(&env, b"ff");
let value = hex_to_u32(&bytes);
// value is Some(255)
```

#### String Convenience Functions

When parsing form input, use the `string_to_*` functions that work directly with `soroban_sdk::String`.

```rust
let input = String::from_str(&env, "42");
let value = string_to_u32(&env, &input);
// value is Some(42)
```

#### &str Convenience Functions

For string literals or `&str` values, use the `str_to_*` functions for a more ergonomic API.

```rust
let value = str_to_u256(&env, "12345");
// value is Some(U256)

let n = str_to_i64(&env, "-42");
// n is Some(-42)
```

These avoid the need to create an intermediate `soroban_sdk::String` or `Bytes`.

#### Supported Types

All conversion functions are available for: `u32`, `i32`, `u64`, `i64`, `u128`, `i128`, `U256`, `I256`.

For the complete function reference, see [llms-full.md](llms-full.md#byte-utilities).

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
