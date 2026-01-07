# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.4](https://github.com/wyhaines/soroban-render-sdk/compare/v0.1.3...v0.1.4) - 2026-01-07

### Other

- Clippy so grumpy. Fixed.
- Ah, format, I always forget ye.
- Add direct registry support for creating a list of contracts and aliases. This can be used to inform components in other contracts, or external UI elements.

## [0.1.3](https://github.com/wyhaines/soroban-render-sdk/compare/v0.1.2...v0.1.3) - 2026-01-05

### Other

- Format. Erg.
- Handle query params.

## [0.1.2](https://github.com/wyhaines/soroban-render-sdk/compare/v0.1.1...v0.1.2) - 2026-01-01

### Other

- Picky formatting that I keep forgetting. Fixed.
- Let's add some additional convenience functions to reduce boilerplate when doing string to number conversions.

## [0.1.1](https://github.com/wyhaines/soroban-render-sdk/compare/v0.1.0...v0.1.1) - 2026-01-01

### Other

- Fixup the clippy complaints.
- Clean up formatting, and get the updated documentation built.
- Let's commit the string/number conversion utilities.
- Cleanup formatting.
- Add a function to facilitate simple boolean select creation.
- Add another _with_value function to the SDK.
- Added some SDK functions to better support markdown inputs, and to enable inputs with values pre-populated.
- Only run release workflow if testing workflow succeeds.

### Added

- Number to decimal Bytes: `i32_to_bytes`, `u64_to_bytes`, `u128_to_bytes`, `i128_to_bytes`, `u256_to_bytes`, `i256_to_bytes`
- Number to hex Bytes: `u32_to_hex`, `i32_to_hex`, `u64_to_hex`, `i64_to_hex`, `u128_to_hex`, `i128_to_hex`, `u256_to_hex`, `i256_to_hex`
- Decimal parsing: `bytes_to_u32`, `bytes_to_i32`, `bytes_to_u64`, `bytes_to_i64`, `bytes_to_u128`, `bytes_to_i128`, `bytes_to_u256`, `bytes_to_i256`
- Hex parsing: `hex_to_u32`, `hex_to_i32`, `hex_to_u64`, `hex_to_i64`, `hex_to_u128`, `hex_to_i128`, `hex_to_u256`, `hex_to_i256`
- String parsing: `string_to_u32`, `string_to_i32`, `string_to_u64`, `string_to_i64`, `string_to_u128`, `string_to_i128`, `string_to_u256`, `string_to_i256`
- &str parsing: `str_to_u32`, `str_to_i32`, `str_to_u64`, `str_to_i64`, `str_to_u128`, `str_to_i128`, `str_to_u256`, `str_to_i256`

## [0.1.0] - 2024-12-25

### Added

- Initial release
- `MarkdownBuilder` for fluent markdown construction
- `JsonDocument` builder for JSON UI format
- `Router` for path-based routing with pattern matching
- `StyleBuilder` for CSS stylesheet generation
- `BaseRegistry` for multi-contract applications
- Metadata macros: `soroban_render!`, `render_v1!`, `render_formats!`
- Byte utilities: `concat_bytes`, `string_to_bytes`, `escape_json_string`
- Link protocols: `render_link`, `tx_link`, `form_link`, `form_link_to`, `tx_link_to`
- HTML containers: `div_start`, `div_end`, `span_start`, `span_end`
- Progressive loading: `continuation`, `render_continue`, `chunk_ref`
- Feature flags for selective module inclusion
