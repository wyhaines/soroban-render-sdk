# soroban-render-sdk

Rust SDK for building Soroban Render contracts. Provides builders and utilities for self-rendering smart contracts.

## CONSTRAINTS

- `#![no_std]` environment
- 64KB WASM size limit
- Use soroban-sdk types only: `Bytes`, `Vec`, `String`, `Env`, `Address`, `Symbol`, `Map`
- String conversion max: 16KB

---

## FEATURE FLAGS

| Feature | Default | Exports |
|---------|---------|---------|
| `markdown` | yes | `MarkdownBuilder` |
| `json` | yes | `JsonDocument`, `FormBuilder`, `TaskBuilder` |
| `router` | yes | `Router`, `RouterResult`, `Request`, path utilities |
| `styles` | yes | `StyleBuilder` |
| `registry` | yes | `BaseRegistry`, `RegistryKey`, `ContractRegistry` |

---

## METADATA MACROS

### soroban_render!

Declares render support.

```rust
soroban_render!(markdown);
soroban_render!(json);
soroban_render!(markdown, json);
soroban_render!(markdown, styles);
soroban_render!(markdown, styles, theme = "CONTRACT_ID");
```

EXPANDS TO:
- `contractmeta!(key = "render", val = "v1")`
- `contractmeta!(key = "render_formats", val = "markdown")` or `"json"` or `"markdown,json"`
- `contractmeta!(key = "render_styles", val = "true")` (if styles)
- `contractmeta!(key = "render_theme", val = "CONTRACT_ID")` (if theme)

### Individual macros

| Macro | Output |
|-------|--------|
| `render_v1!()` | `contractmeta!(key = "render", val = "v1")` |
| `render_formats!(markdown)` | `contractmeta!(key = "render_formats", val = "markdown")` |
| `render_formats!(json)` | `contractmeta!(key = "render_formats", val = "json")` |
| `render_formats!(markdown, json)` | `contractmeta!(key = "render_formats", val = "markdown,json")` |
| `render_has_styles!()` | `contractmeta!(key = "render_styles", val = "true")` |
| `render_theme!("ID")` | `contractmeta!(key = "render_theme", val = "ID")` |

---

## MARKDOWNBUILDER

CONSTRUCTOR: `MarkdownBuilder::new(env: &Env) -> Self`

### Methods

| Method | Signature | Output |
|--------|-----------|--------|
| `h1` | `(text: &str) -> Self` | `# text\n` |
| `h2` | `(text: &str) -> Self` | `## text\n` |
| `h3` | `(text: &str) -> Self` | `### text\n` |
| `heading` | `(level: u8, text: &str) -> Self` | `#...# text\n` |
| `text` | `(text: &str) -> Self` | `text` |
| `paragraph` | `(text: &str) -> Self` | `text\n\n` |
| `bold` | `(text: &str) -> Self` | `**text**` |
| `italic` | `(text: &str) -> Self` | `*text*` |
| `code` | `(text: &str) -> Self` | `` `text` `` |
| `strikethrough` | `(text: &str) -> Self` | `~~text~~` |
| `text_string` | `(s: &String) -> Self` | dynamic string content |
| `number` | `(n: u32) -> Self` | decimal representation |
| `raw` | `(bytes: Bytes) -> Self` | raw bytes |
| `raw_str` | `(s: &str) -> Self` | raw string |
| `newline` | `() -> Self` | `\n` |
| `hr` | `() -> Self` | `---\n` |
| `list_item` | `(text: &str) -> Self` | `- text\n` |
| `checkbox` | `(checked: bool, text: &str) -> Self` | `- [x] text\n` or `- [ ] text\n` |
| `blockquote` | `(text: &str) -> Self` | `> text\n` |
| `build` | `() -> Bytes` | concatenated output |

### Links

| Method | Signature | Output |
|--------|-----------|--------|
| `link` | `(label: &str, url: &str) -> Self` | `[label](url)` |
| `render_link` | `(label: &str, path: &str) -> Self` | `[label](render:path)` |
| `tx_link` | `(label: &str, method: &str, args: &str) -> Self` | `[label](tx:method args)` |
| `tx_link_id` | `(label: &str, method: &str, id: u32) -> Self` | `[label](tx:method {"id":N})` |
| `form_link` | `(label: &str, method: &str) -> Self` | `[label](form:method)` |
| `form_link_to` | `(label: &str, alias: &str, method: &str) -> Self` | `[label](form:@alias:method)` |
| `tx_link_to` | `(label: &str, alias: &str, method: &str, args: &str) -> Self` | `[label](tx:@alias:method args)` |

### Alerts

| Method | Signature | Output |
|--------|-----------|--------|
| `tip` | `(text: &str) -> Self` | `> [!TIP]\n> text\n` |
| `note` | `(text: &str) -> Self` | `> [!NOTE]\n> text\n` |
| `warning` | `(text: &str) -> Self` | `> [!WARNING]\n> text\n` |
| `info` | `(text: &str) -> Self` | `> [!INFO]\n> text\n` |
| `caution` | `(text: &str) -> Self` | `> [!CAUTION]\n> text\n` |
| `alert` | `(alert_type: &str, text: &str) -> Self` | `> [!TYPE]\n> text\n` |

### Columns

| Method | Output |
|--------|--------|
| `columns_start()` | `:::columns\n` |
| `column_separator()` | `|||\n` |
| `columns_end()` | `:::\n` |

### Includes

| Method | Signature | Output |
|--------|-----------|--------|
| `include` | `(contract_id: &str, func: &str) -> Self` | `{{include contract=ID func="func"}}` |
| `include_with_path` | `(contract_id: &str, func: &str, path: &str) -> Self` | `{{include contract=ID func="func" path="path"}}` |

### Form Elements

| Method | Signature | Output |
|--------|-----------|--------|
| `input` | `(name: &str, placeholder: &str) -> Self` | `<input name="name" placeholder="placeholder" />` |
| `hidden_input` | `(name: &str, value: &str) -> Self` | `<input type="hidden" name="name" value="value" />` |
| `redirect` | `(path: &str) -> Self` | `<input type="hidden" name="_redirect" value="path" />` |
| `textarea` | `(name: &str, rows: u8, placeholder: &str) -> Self` | `<textarea name="name" rows="N" placeholder="placeholder"></textarea>` |

### HTML Containers

| Method | Signature | Output |
|--------|-----------|--------|
| `div_start` | `(classes: &str) -> Self` | `<div class="classes">` |
| `div_start_styled` | `(classes: &str, style: &str) -> Self` | `<div class="classes" style="style">` |
| `div_end` | `() -> Self` | `</div>` |
| `span_start` | `(classes: &str) -> Self` | `<span class="classes">` |
| `span_end` | `() -> Self` | `</span>` |

### Progressive Loading

| Method | Signature | Output |
|--------|-----------|--------|
| `continuation` | `(collection: &str, from: u32, total: Option<u32>) -> Self` | `{{continue collection="name" from=N total=M}}` |
| `chunk_ref` | `(collection: &str, index: u32) -> Self` | `{{chunk collection="name" index=N}}` |
| `chunk_ref_placeholder` | `(collection: &str, index: u32, placeholder: &str) -> Self` | `{{chunk collection="name" index=N placeholder="text"}}` |
| `continue_page` | `(collection: &str, page: u32, per_page: u32, total: u32) -> Self` | `{{continue collection="name" page=N per_page=M total=T}}` |
| `render_continue` | `(path: &str) -> Self` | `{{render path="path"}}` |

WATERFALL LOADING: Use `render_continue` to trigger additional render() calls. The viewer fetches the path and inserts the result inline.

---

## JSONDOCUMENT

CONSTRUCTOR: `JsonDocument::new(env: &Env, title: &str) -> Self`

OUTPUT FORMAT: `{"format":"soroban-render-json-v1","title":"...","components":[...]}`

### Methods

| Method | Signature | Description |
|--------|-----------|-------------|
| `heading` | `(level: u8, text: &str) -> Self` | `{"type":"heading","level":N,"text":"..."}` |
| `heading_string` | `(level: u8, text: &String) -> Self` | dynamic text |
| `text` | `(content: &str) -> Self` | `{"type":"text","content":"..."}` |
| `text_string` | `(content: &String) -> Self` | dynamic content |
| `divider` | `() -> Self` | `{"type":"divider"}` |
| `build` | `() -> Bytes` | JSON output |

### Forms

```rust
.form("action")
    .text_field("name", "placeholder", required: bool)
    .textarea_field("name", "placeholder")
    .submit("label")
```

OUTPUT: `{"type":"form","action":"...","fields":[...],"submitLabel":"..."}`

### Navigation

```rust
.nav_start()
.nav_item("label", "/path", active: bool, first: bool)
.nav_end()
```

OUTPUT: `{"type":"navigation","items":[{"label":"...","path":"...","active":true}]}`

### Charts

| Method | Signature | Description |
|--------|-----------|-------------|
| `pie_chart_start` | `(title: &str) -> Self` | start pie chart |
| `pie_slice` | `(label: &str, value: u32, color: &str, first: bool) -> Self` | add slice |
| `pie_chart_end` | `() -> Self` | end pie chart |
| `gauge` | `(value: u32, max: u32, label: &str) -> Self` | gauge chart |

### Containers

| Method | Description |
|--------|-------------|
| `container_start("class")` | start container |
| `container_end()` | end container |

### Tasks

```rust
.task(id: u32, "text", completed: bool)
    .tx_action("method", id, "label")
    .end()
```

---

## ROUTER

Declarative path-based routing.

### Pattern Types

| Pattern | Example | Description |
|---------|---------|-------------|
| Static | `/tasks` | Exact match |
| Named param | `/task/{id}` | Captures segment |
| Wildcard | `/files/*` | Captures remaining path |

### Usage

```rust
Router::new(&env, path)
    .handle(b"/", |req| render_home())
    .or_handle(b"/task/{id}", |req| {
        let id = req.get_var_u32(b"id").unwrap_or(0);
        render_task(id)
    })
    .or_handle(b"/files/*", |req| {
        let remaining = req.get_wildcard().unwrap();
        render_file(remaining)
    })
    .or_default(|req| render_home())
```

### Router Methods

| Method | Signature |
|--------|-----------|
| `new` | `(env: &Env, path: Option<String>) -> Self` |
| `from_bytes` | `(env: &Env, path: Bytes) -> Self` |
| `handle` | `(pattern: &[u8], handler: F) -> RouterResult<T>` |

### RouterResult Methods

| Method | Signature |
|--------|-----------|
| `or_handle` | `(pattern: &[u8], handler: F) -> Self` |
| `or_default` | `(handler: F) -> T` |

### Request Methods

| Method | Return |
|--------|--------|
| `path()` | `&Bytes` |
| `get_var(key: &[u8])` | `Option<Bytes>` |
| `get_var_u32(key: &[u8])` | `Option<u32>` |
| `get_wildcard()` | `Option<Bytes>` |

---

## PATH UTILITIES

| Function | Signature | Description |
|----------|-----------|-------------|
| `path_to_bytes` | `(env: &Env, path: &Option<String>) -> Bytes` | Convert, default "/" |
| `path_eq` | `(path: &Bytes, route: &[u8]) -> bool` | Exact match |
| `path_starts_with` | `(path: &Bytes, prefix: &[u8]) -> bool` | Prefix check |
| `path_suffix` | `(env: &Env, path: &Bytes, prefix: &[u8]) -> Bytes` | Extract suffix |
| `parse_id` | `(path: &Bytes, prefix: &[u8]) -> Option<u32>` | Parse numeric ID |

---

## STYLEBUILDER

CONSTRUCTOR: `StyleBuilder::new(env: &Env) -> Self`

### CSS Variables

| Method | Output |
|--------|--------|
| `root_var("name", "value")` | `:root { --name: value; }` |
| `root_vars_start()` | `:root {` |
| `var("name", "value")` | `  --name: value;` |
| `root_vars_end()` | `}` |

### CSS Rules

| Method | Output |
|--------|--------|
| `rule("selector", "properties")` | `selector { properties }` |
| `rule_start("selector")` | `selector {` |
| `prop("property", "value")` | `  property: value;` |
| `rule_end()` | `}` |

### Media Queries

| Method | Output |
|--------|--------|
| `media_start("condition")` | `@media condition {` |
| `media_end()` | `}` |
| `dark_mode_start()` | `@media (prefers-color-scheme: dark) {` |
| `light_mode_start()` | `@media (prefers-color-scheme: light) {` |
| `breakpoint_min(width)` | `@media (min-width: Npx) {` |
| `breakpoint_max(width)` | `@media (max-width: Npx) {` |

### Utilities

| Method | Output |
|--------|--------|
| `raw("css")` | raw CSS |
| `comment("text")` | `/* text */` |
| `newline()` | blank line |
| `build()` | `Bytes` |

---

## BASEREGISTRY

Contract address alias management for multi-contract applications.

### Storage Keys

| Key | Type |
|-----|------|
| `RegistryKey::Contracts` | `Map<Symbol, Address>` |
| `RegistryKey::Admin` | `Address` |

### Functions

| Function | Signature | Auth |
|----------|-----------|------|
| `init` | `(env: &Env, admin: &Address, contracts: Map<Symbol, Address>)` | admin |
| `register` | `(env: &Env, alias: Symbol, address: Address)` | admin |
| `unregister` | `(env: &Env, alias: Symbol)` | admin |
| `get_by_alias` | `(env: &Env, alias: Symbol) -> Option<Address>` | none |
| `get_all` | `(env: &Env) -> Map<Symbol, Address>` | none |
| `get_admin` | `(env: &Env) -> Option<Address>` | none |
| `emit_aliases` | `(env: &Env) -> Bytes` | none |

### emit_aliases

Generate `{{aliases ...}}` tag from all registered contracts for include resolution.

```rust
// In render function:
let aliases = BaseRegistry::emit_aliases(&env);
MarkdownBuilder::new(&env)
    .raw(aliases)  // {{aliases theme=CXYZ... content=CABC...}}
    // ...
```

OUTPUT: `{{aliases alias1=CONTRACT_ID alias2=CONTRACT_ID ...}}`

For cross-contract use, expose as public function:

```rust
pub fn render_aliases(env: Env) -> Bytes {
    BaseRegistry::emit_aliases(&env)
}
```

### Usage Pattern

```rust
use soroban_render_sdk::registry::BaseRegistry;
use soroban_sdk::{symbol_short, Map};

// Initialize
let mut contracts = Map::new(&env);
contracts.set(symbol_short!("theme"), theme_addr);
contracts.set(symbol_short!("content"), content_addr);
BaseRegistry::init(&env, &admin, contracts);

// Look up
let addr = BaseRegistry::get_by_alias(&env, symbol_short!("theme"));
```

### ContractRegistry Trait

```rust
pub trait ContractRegistry {
    fn register_contract(env: &Env, alias: Symbol, address: Address);
    fn get_contract_by_alias(env: &Env, alias: Symbol) -> Option<Address>;
    fn get_all_contracts(env: &Env) -> Map<Symbol, Address>;
}
```

---

## BYTE UTILITIES

| Constant | Value |
|----------|-------|
| `MAX_STRING_SIZE` | 16384 (16KB) |

### Core Utilities

| Function | Signature | Description |
|----------|-----------|-------------|
| `concat_bytes` | `(env: &Env, parts: &Vec<Bytes>) -> Bytes` | Join Bytes |
| `string_to_bytes` | `(env: &Env, s: &String) -> Bytes` | Convert String |
| `escape_json_string` | `(env: &Env, s: &String) -> Bytes` | JSON escape String |
| `escape_json_bytes` | `(env: &Env, input: &[u8]) -> Bytes` | JSON escape bytes |
| `address_to_bytes` | `(env: &Env, addr: &Address) -> Bytes` | Convert Address to contract ID string |
| `symbol_to_bytes` | `(env: &Env, sym: &Symbol) -> Bytes` | Convert Symbol to string |

ESCAPE RULES: `"` → `\"`, `\` → `\\`, `\n` → `\n`, `\r` → `\r`, `\t` → `\t`

### Type Conversion

```rust
// Address to 56-character contract ID string
let addr = env.current_contract_address();
let id_bytes = address_to_bytes(&env, &addr);
// id_bytes contains "CABC...XYZ" as Bytes

// Symbol to string (short symbols only, ≤9 chars)
let sym = symbol_short!("theme");
let sym_bytes = symbol_to_bytes(&env, &sym);
// sym_bytes contains "theme" as Bytes
```

### Number → Decimal Bytes

| Function | Signature |
|----------|-----------|
| `u32_to_bytes` | `(env: &Env, n: u32) -> Bytes` |
| `i32_to_bytes` | `(env: &Env, n: i32) -> Bytes` |
| `u64_to_bytes` | `(env: &Env, n: u64) -> Bytes` |
| `i64_to_bytes` | `(env: &Env, n: i64) -> Bytes` |
| `u128_to_bytes` | `(env: &Env, n: u128) -> Bytes` |
| `i128_to_bytes` | `(env: &Env, n: i128) -> Bytes` |
| `u256_to_bytes` | `(env: &Env, n: &U256) -> Bytes` |
| `i256_to_bytes` | `(env: &Env, n: &I256) -> Bytes` |

### Number → Hex Bytes

Output format: `0x...` (lowercase). Negative: `-0x...`

| Function | Signature |
|----------|-----------|
| `u32_to_hex` | `(env: &Env, n: u32) -> Bytes` |
| `i32_to_hex` | `(env: &Env, n: i32) -> Bytes` |
| `u64_to_hex` | `(env: &Env, n: u64) -> Bytes` |
| `i64_to_hex` | `(env: &Env, n: i64) -> Bytes` |
| `u128_to_hex` | `(env: &Env, n: u128) -> Bytes` |
| `i128_to_hex` | `(env: &Env, n: i128) -> Bytes` |
| `u256_to_hex` | `(env: &Env, n: &U256) -> Bytes` |
| `i256_to_hex` | `(env: &Env, n: &I256) -> Bytes` |

### Decimal Bytes → Number

Returns `None` on invalid input or overflow.

| Function | Signature | Return |
|----------|-----------|--------|
| `bytes_to_u32` | `(bytes: &Bytes)` | `Option<u32>` |
| `bytes_to_i32` | `(bytes: &Bytes)` | `Option<i32>` |
| `bytes_to_u64` | `(bytes: &Bytes)` | `Option<u64>` |
| `bytes_to_i64` | `(bytes: &Bytes)` | `Option<i64>` |
| `bytes_to_u128` | `(bytes: &Bytes)` | `Option<u128>` |
| `bytes_to_i128` | `(bytes: &Bytes)` | `Option<i128>` |
| `bytes_to_u256` | `(env: &Env, bytes: &Bytes)` | `Option<U256>` |
| `bytes_to_i256` | `(env: &Env, bytes: &Bytes)` | `Option<I256>` |

### Hex Bytes → Number

Accepts optional `0x`/`0X` prefix. Case-insensitive. Returns `None` on invalid input.

| Function | Signature | Return |
|----------|-----------|--------|
| `hex_to_u32` | `(bytes: &Bytes)` | `Option<u32>` |
| `hex_to_i32` | `(bytes: &Bytes)` | `Option<i32>` |
| `hex_to_u64` | `(bytes: &Bytes)` | `Option<u64>` |
| `hex_to_i64` | `(bytes: &Bytes)` | `Option<i64>` |
| `hex_to_u128` | `(bytes: &Bytes)` | `Option<u128>` |
| `hex_to_i128` | `(bytes: &Bytes)` | `Option<i128>` |
| `hex_to_u256` | `(env: &Env, bytes: &Bytes)` | `Option<U256>` |
| `hex_to_i256` | `(env: &Env, bytes: &Bytes)` | `Option<I256>` |

### String → Number

Convenience wrappers for parsing `soroban_sdk::String`. Useful for form input.

| Function | Signature | Return |
|----------|-----------|--------|
| `string_to_u32` | `(env: &Env, s: &String)` | `Option<u32>` |
| `string_to_i32` | `(env: &Env, s: &String)` | `Option<i32>` |
| `string_to_u64` | `(env: &Env, s: &String)` | `Option<u64>` |
| `string_to_i64` | `(env: &Env, s: &String)` | `Option<i64>` |
| `string_to_u128` | `(env: &Env, s: &String)` | `Option<u128>` |
| `string_to_i128` | `(env: &Env, s: &String)` | `Option<i128>` |
| `string_to_u256` | `(env: &Env, s: &String)` | `Option<U256>` |
| `string_to_i256` | `(env: &Env, s: &String)` | `Option<I256>` |

### &str → Number

Convenience wrappers for parsing `&str` directly. Useful for string literals.

| Function | Signature | Return |
|----------|-----------|--------|
| `str_to_u32` | `(env: &Env, s: &str)` | `Option<u32>` |
| `str_to_i32` | `(env: &Env, s: &str)` | `Option<i32>` |
| `str_to_u64` | `(env: &Env, s: &str)` | `Option<u64>` |
| `str_to_i64` | `(env: &Env, s: &str)` | `Option<i64>` |
| `str_to_u128` | `(env: &Env, s: &str)` | `Option<u128>` |
| `str_to_i128` | `(env: &Env, s: &str)` | `Option<i128>` |
| `str_to_u256` | `(env: &Env, s: &str)` | `Option<U256>` |
| `str_to_i256` | `(env: &Env, s: &str)` | `Option<I256>` |

### Behavior Summary

- Signed output: `-` prefix for negative values
- Hex output: `0x` prefix, lowercase digits
- Parsing: `Option<T>` return, `None` on invalid input or overflow
- U256/I256: require `env` parameter for SDK type construction

---

## PRELUDE

```rust
use soroban_render_sdk::prelude::*;
```

EXPORTS:
- `Bytes`
- `soroban_render!`, `render_v1!`, `render_formats!`
- `MarkdownBuilder` (if `markdown` feature)
- `JsonDocument`, `FormBuilder`, `TaskBuilder` (if `json` feature)
- `Router`, `RouterResult`, `Request` (if `router` feature)
- `StyleBuilder` (if `styles` feature)
- `BaseRegistry`, `RegistryKey`, `ContractRegistry` (if `registry` feature)
- All path utilities
- All byte utilities

---

## CONTRACT SIGNATURE

Standard render contract:

```rust
#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Bytes, Env, String};
use soroban_render_sdk::prelude::*;

soroban_render!(markdown);

#[contract]
pub struct MyContract;

#[contractimpl]
impl MyContract {
    pub fn render(env: Env, path: Option<String>, viewer: Option<Address>) -> Bytes {
        // Build and return content
    }

    // Optional: for chunked content
    pub fn get_chunk(env: Env, collection: Symbol, index: u32) -> Option<Bytes> {
        // Return chunk
    }

    // Optional: for styles
    pub fn styles(env: Env) -> Bytes {
        // Return CSS
    }
}
```
