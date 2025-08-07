<!-- {{# generate.module_header{} #}} -->

# Module :: `error_tools`
<!--{ generate.module_header.start() }-->
 [![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/module_error_tools_push.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/module_error_tools_push.yml) [![docs.rs](https://img.shields.io/docsrs/error_tools?color=e3e8f0&logo=docs.rs)](https://docs.rs/error_tools) [![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=module%2Fcore%2Ferror_tools%2Fexamples%2Ferror_tools_trivial.rs,RUN_POSTFIX=--example%20module%2Fcore%2Ferror_tools%2Fexamples%2Ferror_tools_trivial.rs/https://github.com/Wandalen/wTools) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)
<!--{ generate.module_header.end }-->

A unified error handling facade that provides a consistent interface for both typed and untyped error handling in Rust. `error_tools` acts as a standardized wrapper around the popular `thiserror` and `anyhow` crates, enabling you to write error-handling code once and use it consistently across different contexts.

## Why error_tools?

When building Rust applications and libraries, you often face these error handling challenges:

- **Library vs Application Choice**: Libraries typically use `thiserror` for typed errors, while applications prefer `anyhow` for flexibility
- **Inconsistent Error Patterns**: Different crates in your dependency tree use different error handling approaches
- **Dependency Fragmentation**: Having both `anyhow` and `thiserror` as direct dependencies across multiple crates
- **Context Switching**: Different syntax and patterns for similar error handling tasks
- **Integration Friction**: Converting between different error types when bridging library and application code

**error_tools** solves these problems by providing:

- 🎯 **Unified Interface**: Single import pattern for both typed and untyped errors
- 📦 **Dependency Facade**: Centralized re-export of `anyhow` and `thiserror` functionality
- 🔧 **Enhanced Utilities**: Additional error handling utilities like `ErrWith` trait
- 🏗️ **Consistent Patterns**: Standardized error handling across the entire wTools ecosystem
- 🚀 **Easy Migration**: Drop-in replacement for existing `anyhow`/`thiserror` usage
- 🛡️ **no_std Support**: Works in `no_std` environments when needed

## Quick Start

### Installation

```sh
cargo add error_tools
```

### Basic Usage

Choose your approach based on your needs:

```rust
// For applications - flexible, untyped errors (anyhow-style)
use error_tools::untyped::*;

// For libraries - structured, typed errors (thiserror-style)  
use error_tools::typed::*;
use error_tools::dependency::thiserror;

// For convenience - includes both
use error_tools::prelude::*;
```

## Core Concepts

### 1. Untyped Errors (Application-Focused)

Perfect for applications where you need flexible error handling without defining custom error types for every possible failure. This is a direct facade over `anyhow`.

**Key Features:**
- Dynamic error handling with context
- Easy error chaining and reporting
- Rich context information
- Perfect for rapid prototyping and applications

```rust
use error_tools::untyped::{ Result, format_err };

fn get_message() -> Result< &'static str >
{
  Ok( "Hello, world!" )
  // Err( format_err!( "An unexpected error!" ) )
}

fn main()
{
  match get_message()
  {
    Ok( msg ) => println!( "Success: {}", msg ),
    Err( e ) => println!( "Error: {:?}", e ),
  }
}
```

Run this example:
```sh
cargo run --example error_tools_trivial
```

### 2. Working with Context

Adding context to errors helps with debugging and user experience:

```rust
use error_tools::untyped::{ Result, Context, format_err };

fn read_and_process_file( path : &str ) -> Result< String >
{
  // Simulate file reading for demonstration  
  let content = if path == "test.txt" { "hello world" } else { "" };
  
  if content.is_empty()
  {
    return Err( format_err!( "File is empty or not found: {}", path ) );
  }

  Ok( content.to_uppercase() )
}

fn main()
{
  match read_and_process_file( "test.txt" )
  {
    Ok( content ) => println!( "Processed: {}", content ),
    Err( e ) => println!( "Error: {}", e ),
  }
}
```

> See the full runnable example in [`examples/replace_anyhow.rs`](./examples/replace_anyhow.rs).

### 3. Typed Errors (Library-Focused)

Ideal for libraries where you want to provide a clear, structured contract for possible errors. This is a facade over `thiserror`.

**Key Features:**
- Structured error types with derive macros
- Clear error hierarchies
- Compile-time error checking
- Better API boundaries for library consumers

```rust
use error_tools::typed::Error;
use error_tools::dependency::thiserror;

#[ derive( Debug, Error ) ]
pub enum DataError
{
  #[ error( "I/O error for file: {file}" ) ]
  Io { file : String },
  #[ error( "Parsing error: {0}" ) ]
  Parse( String ),
}

fn process_data( file_name : &str, content : &str ) -> Result< i32, DataError >
{
  if content.is_empty()
  {
    return Err( DataError::Io { file : file_name.to_string() } );
  }

  content.trim().parse::< i32 >()
    .map_err( | _ | DataError::Parse( "Could not parse content as integer".into() ) )
}

fn main()
{
  match process_data( "data.txt", "123" )
  {
    Ok( num ) => println!( "Parsed number: {}", num ),
    Err( e ) => println!( "Error: {}", e ),
  }
  
  // Example with error
  match process_data( "invalid.txt", "abc" )
  {
    Ok( _ ) => (),
    Err( e ) => println!( "Expected error: {}", e ),
  }
}
```

> See the full runnable example in [`examples/replace_thiserror.rs`](./examples/replace_thiserror.rs).

### 4. Enhanced Error Context with ErrWith

The `ErrWith` trait provides additional utilities for adding context to errors:

```rust
use error_tools::{ ErrWith };

fn process_user_data( user_id : u32, data : &str ) -> Result< String, ( String, Box< dyn std::error::Error > ) >
{
  // Add context using closures for lazy evaluation
  let parsed_data = data.parse::< i32 >()
    .err_with( || format!( "Failed to parse data for user {}", user_id ) )?;

  // Add context using references for simple messages  
  let processed = perform_calculation( parsed_data )
    .err_with_report( &format!( "Calculation failed for user {}", user_id ) )?;

  Ok( format!( "Processed: {}", processed ) )
}

fn perform_calculation( input : i32 ) -> std::result::Result< i32, &'static str >
{
  if input < 0
  {
    Err( "Negative numbers not supported" )
  }
  else
  {
    Ok( input * 2 )
  }
}

fn main()
{
  match process_user_data( 123, "42" )
  {
    Ok( result ) => println!( "Success: {}", result ),
    Err( ( report, err ) ) => println!( "Error: {} - {:?}", report, err ),
  }
}
```

> See the full runnable example in [`examples/err_with_example.rs`](./examples/err_with_example.rs).

### 5. Debug Assertions

Additional debugging utilities for development:

```rust
use error_tools::{ debug_assert_id, debug_assert_ni };

fn validate_data( expected : &str, actual : &str )
{
  // Only active in debug builds
  debug_assert_id!( expected, actual, "Data validation failed" );
  
  // Negative assertion
  debug_assert_ni!( expected, "", "Expected data should not be empty" );
}

fn main()
{
  validate_data( "test", "test" );
  println!( "Debug assertions passed!" );
}
```

## Examples

### Basic Error Handling

```rust
use error_tools::untyped::Result;

fn might_fail( should_fail : bool ) -> Result< String >
{
  if should_fail
  {
    Err( error_tools::untyped::format_err!( "Something went wrong" ) )
  }
  else
  {
    Ok( "Success!".to_string() )
  }
}

fn main()
{
  match might_fail( false )
  {
    Ok( msg ) => println!( "Result: {}", msg ),
    Err( e ) => println!( "Error: {}", e ),
  }
}
```

### Using Both Typed and Untyped Errors

```rust
use error_tools::prelude::*;
use error_tools::dependency::thiserror;

// Typed error for library API
#[ derive( Debug, Error ) ]
pub enum ConfigError
{
  #[ error( "Configuration file not found" ) ]
  NotFound,
  #[ error( "Invalid format: {0}" ) ]
  InvalidFormat( String ),
}

// Function returning typed error
fn load_config_typed() -> Result< String, ConfigError >
{
  Err( ConfigError::NotFound )
}

// Function returning untyped error
fn load_config_untyped() -> error_tools::untyped::Result< String >
{
  Err( error_tools::untyped::format_err!( "Configuration loading failed" ) )
}

fn main()
{
  // Handle typed error
  if let Err( e ) = load_config_typed()
  {
    println!( "Typed error: {}", e );
  }

  // Handle untyped error  
  if let Err( e ) = load_config_untyped()
  {
    println!( "Untyped error: {}", e );
  }
}
```

## Feature Flags

`error_tools` supports granular feature control:

```toml
[dependencies]
error_tools = { version = "0.26", features = [ "error_typed" ] }  # Only typed errors
# or
error_tools = { version = "0.26", features = [ "error_untyped" ] }  # Only untyped errors  
# or
error_tools = { version = "0.26" }  # Both (default)
```

**Available Features:**
- `default` - Enables both `error_typed` and `error_untyped`
- `error_typed` - Enables `thiserror` integration for structured errors
- `error_untyped` - Enables `anyhow` integration for flexible errors
- `no_std` - Enables `no_std` support
- `use_alloc` - Enables allocation support in `no_std` environments

## Migration Guide

### From anyhow

Replace your `anyhow` imports with `error_tools::untyped`:

```rust
// Before
// use anyhow::{ Result, Context, bail, format_err };

// After  
use error_tools::untyped::{ Result, Context, bail, format_err };

fn main() {
    println!("Migration complete - same API, different import!");
}
```

Everything else stays the same!

### From thiserror

Add the explicit `thiserror` import and use `error_tools::typed`:

```rust
// Before
// use thiserror::Error;

// After
use error_tools::typed::Error;
use error_tools::dependency::thiserror;  // Required for derive macros

fn main() {
    println!("Migration complete - same derive macros, unified import!");
}
```

The derive macros work identically.

## Complete Examples

Explore these runnable examples in the repository:

```sh
# Basic usage patterns
cargo run --example error_tools_trivial

# Migration from anyhow
cargo run --example replace_anyhow

# Migration from thiserror  
cargo run --example replace_thiserror

# Using the ErrWith trait
cargo run --example err_with_example
```

## Best Practices

### 1. Choose the Right Error Style

- **Applications**: Use `untyped` errors for flexibility and rapid development
- **Libraries**: Use `typed` errors for clear API contracts and better user experience
- **Mixed Projects**: Use both as appropriate - they interoperate well

### 2. Error Context

Always provide meaningful context:

```rust
use error_tools::untyped::{ Result, Context, format_err };

fn process_user_data( user_id : u32 ) -> Result< String >
{
  // Good - specific context
  let _result = simulate_operation()
    .context( format!( "Failed to process user {} data", user_id ) )?;

  // Less helpful - generic context  
  let _other = simulate_operation()
    .context( "An error occurred" )?;

  Ok( "Success".to_string() )
}

fn simulate_operation() -> Result< String >
{
  Ok( "data".to_string() )
}

fn main()
{
  match process_user_data( 123 )
  {
    Ok( result ) => println!( "Result: {}", result ),
    Err( e ) => println!( "Error: {}", e ),
  }
}
```

### 3. Error Hierarchies

For libraries, design clear error hierarchies:

```rust
use error_tools::typed::Error;
use error_tools::dependency::thiserror;

#[ derive( Debug, Error ) ]
pub enum LibraryError
{
  #[ error( "Configuration error: {0}" ) ]
  Config( #[from] ConfigError ),
  
  #[ error( "Network error: {0}" ) ]
  Network( #[from] NetworkError ),
  
  #[ error( "Database error: {0}" ) ]
  Database( #[from] DatabaseError ),
}

// Define the individual error types
#[ derive( Debug, Error ) ]
pub enum ConfigError
{
  #[ error( "Config not found" ) ]
  NotFound,
}

#[ derive( Debug, Error ) ]  
pub enum NetworkError
{
  #[ error( "Connection failed" ) ]
  ConnectionFailed,
}

#[ derive( Debug, Error ) ]
pub enum DatabaseError
{
  #[ error( "Query failed" ) ]
  QueryFailed,
}

fn main()
{
  let config_err = LibraryError::Config( ConfigError::NotFound );
  println!( "Error hierarchy example: {}", config_err );
}
```

### 4. Dependency Access

When you need direct access to the underlying crates:

```rust
// Access the underlying crates if needed
// use error_tools::dependency::{ anyhow, thiserror };

// Or via the specific modules
use error_tools::untyped;  // Re-exports anyhow
use error_tools::typed;    // Re-exports thiserror

fn main()
{
    println!("Direct access to underlying crates available via dependency module");
}
```

## Integration with wTools Ecosystem

`error_tools` is designed to work seamlessly with other wTools crates:

- **Consistent Error Handling**: All wTools crates use `error_tools` for unified error patterns
- **Cross-Crate Compatibility**: Errors from different wTools crates integrate naturally
- **Standardized Debugging**: Common debugging utilities across the ecosystem

## To add to your project

```sh
cargo add error_tools
```

## Try out from the repository

```sh
git clone https://github.com/Wandalen/wTools
cd wTools
cargo run --example error_tools_trivial
# Or try the specific examples
cargo run --example replace_anyhow
cargo run --example replace_thiserror
cargo run --example err_with_example
```