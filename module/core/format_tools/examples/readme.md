# Examples

This directory contains usage examples for the format_tools crate.

## Available Examples

| Example | Description |
|---------|-------------|
| format_tools_trivial.rs | Demonstrates `to_string_with_fallback` macro with Display/Debug formatting fallback mechanism |

## Running Examples

Run an example using:

```sh
cargo run --example format_tools_trivial
```

## Example Coverage

The examples demonstrate:
- Fallback-based string conversion (`to_string_with_fallback` macro)
- Using `WithDisplay` as primary formatter
- Using `WithDebug` as fallback formatter
- Handling types with both Display and Debug traits
- Handling types with only Debug trait
