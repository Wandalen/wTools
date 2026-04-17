# src

## Purpose

Library source files. Each file owns one cohesive responsibility; `lib.rs` re-exports the public API.

## Responsibility Table

| File | Responsibility |
|------|----------------|
| `lib.rs` | Crate root: module declarations, feature gates, public re-exports |
| `colorful_text.rs` | `ColorfulText` struct: typed text with optional ANSI color |
