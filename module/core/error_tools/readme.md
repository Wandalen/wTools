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
use error_tools::untyped::{ Result, Context, format_err };

fn read_config_file( path : &str ) -> Result< String >
{
  let content = std::fs::read_to_string( path )
    .context( format!( "Failed to read config file at '{}'", path ) )?;

  if content.trim().is_empty()
  {
    return Err( format_err!( "Configuration file is empty" ) );
  }

  Ok( content )
}

fn main() -> Result<()>
{
  let config = read_config_file( "app.toml" )
    .context( "Failed to load application configuration" )?;
  
  println!( "Loaded config: {} bytes", config.len() );
  Ok(())
}
```

### 2. Typed Errors (Library-Focused)

Ideal for libraries where you want to provide a clear, structured contract for possible errors. This is a facade over `thiserror`.

**Key Features:**
- Structured error types with derive macros
- Clear error hierarchies
- Compile-time error checking
- Better API boundaries for library consumers

```rust
use error_tools::typed::Error;
use error_tools::dependency::thiserror;
use std::path::PathBuf;

#[ derive( Debug, Error ) ]
pub enum ConfigError
{
  #[ error( "Configuration file not found: {path}" ) ]
  NotFound { path : PathBuf },
  
  #[ error( "Invalid configuration format in {path}: {reason}" ) ]
  InvalidFormat { path : PathBuf, reason : String },
  
  #[ error( "Permission denied accessing {path}" ) ]
  PermissionDenied { path : PathBuf },
  
  #[ error( "I/O error: {0}" ) ]
  Io( #[from] std::io::Error ),
}

fn load_config( path : &PathBuf ) -> Result< String, ConfigError >
{
  match std::fs::read_to_string( path )
  {
    Ok( content ) => 
    {
      if content.trim().is_empty()
      {
        Err( ConfigError::InvalidFormat
        {
          path : path.clone(),
          reason : "File is empty".to_string(),
        })
      }
      else
      {
        Ok( content )
      }
    }
    Err( err ) => match err.kind()
    {
      std::io::ErrorKind::NotFound => Err( ConfigError::NotFound { path : path.clone() } ),
      std::io::ErrorKind::PermissionDenied => Err( ConfigError::PermissionDenied { path : path.clone() } ),
      _ => Err( ConfigError::Io( err ) ),
    }
  }
}
```

### 3. Enhanced Error Context with ErrWith

The `ErrWith` trait provides additional utilities for adding context to errors:

```rust
use error_tools::{ ErrWith, Result };

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
```

### 4. Debug Assertions

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
use anyhow::{ Result, Context, bail, format_err };

// After  
use error_tools::untyped::{ Result, Context, bail, format_err };
```

Everything else stays the same!

### From thiserror

Add the explicit `thiserror` import and use `error_tools::typed`:

```rust
// Before
use thiserror::Error;

// After
use error_tools::typed::Error;
use error_tools::dependency::thiserror;  // Required for derive macros
```

The derive macros work identically.

## Examples

### Real-World Application Error Handling

```rust
use error_tools::untyped::{ Result, Context };

struct DatabaseConfig
{
  url : String,
  timeout : u64,
}

fn load_database_config( path : &str ) -> Result< DatabaseConfig >
{
  let content = std::fs::read_to_string( path )
    .with_context( || format!( "Failed to read database config from {}", path ) )?;

  let parsed : toml::Value = toml::from_str( &content )
    .context( "Failed to parse TOML configuration" )?;

  let url = parsed.get( "database_url" )
    .and_then( |v| v.as_str() )
    .ok_or_else( || format_err!( "Missing 'database_url' in configuration" ) )?
    .to_string();

  let timeout = parsed.get( "timeout" )
    .and_then( |v| v.as_integer() )
    .unwrap_or( 30 ) as u64;

  Ok( DatabaseConfig { url, timeout } )
}
```

### Library Error Design

```rust
use error_tools::typed::Error;
use error_tools::dependency::thiserror;

#[ derive( Debug, Error ) ]
pub enum HttpClientError
{
  #[ error( "Network request failed: {url}" ) ]
  NetworkError
  {
    url : String,
    #[ source ]
    source : reqwest::Error,
  },

  #[ error( "Server returned error {status}: {message}" ) ]
  ServerError
  {
    status : u16,
    message : String,
  },

  #[ error( "Request timeout after {timeout}s" ) ]
  Timeout { timeout : u64 },

  #[ error( "Invalid URL: {0}" ) ]
  InvalidUrl( String ),
}

pub struct HttpClient
{
  client : reqwest::Client,
  base_url : String,
}

impl HttpClient
{
  pub fn get( &self, path : &str ) -> Result< String, HttpClientError >
  {
    let url = format!( "{}/{}", self.base_url.trim_end_matches('/'), path.trim_start_matches('/') );
    
    // URL validation
    if !url.starts_with( "http" )
    {
      return Err( HttpClientError::InvalidUrl( url ) );
    }

    // This is a simplified example - in real code you'd use async
    todo!( "Implement actual HTTP request" )
  }
}
```

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
// Good - specific context
.context( format!( "Failed to process user {} data", user_id ) )?

// Less helpful - generic context
.context( "An error occurred" )?
```

### 3. Error Hierarchies

For libraries, design clear error hierarchies:

```rust
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
```

### 4. Dependency Access

When you need direct access to the underlying crates:

```rust
// Access the underlying crates
use error_tools::dependency::{ anyhow, thiserror };

// Or via the specific modules
use error_tools::untyped;  // Re-exports anyhow
use error_tools::typed;    // Re-exports thiserror
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