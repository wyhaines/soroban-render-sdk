# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.1](https://github.com/wyhaines/soroban-render-sdk/compare/v0.1.0...v0.1.1) - 2025-12-25

### Other

- Only run release workflow if testing workflow succeeds.

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
