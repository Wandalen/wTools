
# strs_tools

[![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/module_strs_tools_push.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/module_strs_tools_push.yml) [![docs.rs](https://img.shields.io/docsrs/strs_tools?color=e3e8f0&logo=docs.rs)](https://docs.rs/strs_tools) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)

Advanced string manipulation tools with SIMD acceleration and intelligent parsing.

## Why strs_tools?

While Rust's standard library provides basic string operations, `strs_tools` offers sophisticated string manipulation capabilities that handle real-world complexity:

- **Smart Splitting**: Split strings with quote awareness, escape handling, and delimiter preservation
- **Intelligent Parsing**: Parse command-like strings and extract key-value parameters
- **Fast Performance**: Optional SIMD acceleration for high-throughput text processing
- **Memory Efficient**: Zero-allocation operations where possible using `Cow<str>`

## Quick Start

```sh
cargo add strs_tools
```

## Examples

### Advanced String Splitting

Unlike standard `str.split()`, handles quotes and preserves context:

```rust
# #[cfg(all(feature = "string_split", not(feature = "no_std")))]
# {
use strs_tools::string;

// Basic splitting with delimiter preservation
let text = "hello world test";
let result : Vec< String > = string::split()
.src( text )
.delimeter( " " )
.stripping( false )  // Keep delimiters
.perform()
.map( String::from )
.collect();

assert_eq!( result, vec![ "hello", " ", "world", " ", "test" ] );

// Quote-aware splitting (perfect for parsing commands)
let command = r#"run --file "my file.txt" --verbose"#;
let parts : Vec< String > = string::split()
.src( command )
.delimeter( " " )
.quoting( true )     // Handle quotes intelligently
.perform()
.map( String::from )
.collect();
// Results: ["run", "--file", "my file.txt", "--verbose"]
# }
```

### Text Indentation

Add consistent indentation to multi-line text:

```rust
# #[cfg(all(feature = "string_indentation", not(feature = "no_std")))]
# {
use strs_tools::string;

let code = "fn main() {\n    println!(\"Hello\");\n}";
let indented = string::indentation::indentation( "  ", code, "" );
// Result: "  fn main() {\n      println!(\"Hello\");\n  }"
# }
```

### Command Parsing

Parse command-line style strings into structured data:

```rust
use strs_tools::string;

let input = "deploy --env production --force --config ./deploy.toml";
// Command parsing functionality under development
println!( "Command: {}", input );
// Note: Full parse_request API is still being finalized
```

### Number Parsing

Robust number parsing with multiple format support:

```rust
let values = [ "42", "3.14", "1e6" ];
for val in values
{
  if let Ok( num ) = val.parse::< f64 >()
  {
    println!( "{} = {}", val, num );
  }
}
```

## Performance Features

Enable SIMD acceleration for demanding applications:

```toml
[dependencies]
strs_tools = { version = "0.24", features = ["simd"] }
```

SIMD features provide significant speedups for:
- Large text processing
- Pattern matching across multiple delimiters  
- Bulk string operations

## Feature Selection

Choose only the functionality you need:

```toml
[dependencies]
strs_tools = { 
    version = "0.24", 
    features = ["string_split", "string_parse_request"], 
    default-features = false 
}
```

**Available features:**
- `string_split` - Advanced splitting with quotes and escaping
- `string_indentation` - Text indentation tools
- `string_isolate` - String isolation by delimiters
- `string_parse_request` - Command parsing utilities
- `string_parse_number` - Number parsing from strings
- `simd` - SIMD acceleration (recommended for performance)

## When to Use strs_tools

**Perfect for:**
- CLI applications parsing complex commands
- Configuration file processors
- Text processing tools and parsers
- Data extraction from formatted text
- Applications requiring high-performance string operations

**Alternatives:**
- Use standard `str` methods for simple splitting and basic operations
- Consider `regex` crate for complex pattern matching
- Use `clap` or `structopt` for full CLI argument parsing frameworks

## Examples

Explore comprehensive examples showing real-world usage:

```sh
git clone https://github.com/Wandalen/wTools
cd wTools/module/core/strs_tools

# Run examples by number
cargo run --example 001_basic_usage
cargo run --example 002_advanced_splitting
cargo run --example 003_text_indentation
cargo run --example 004_command_parsing
cargo run --example 005_string_isolation
cargo run --example 006_number_parsing
cargo run --example 007_performance_and_simd --features simd
```

## Documentation

- [API Documentation](https://docs.rs/strs_tools)
- [Architecture Details](./architecture.md)
- [Performance Benchmarks](./benchmarks/readme.md)
- [Migration Guide](./changelog.md)
