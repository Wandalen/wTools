<!-- {{# generate.module_header{} #}} -->

# Module :: `error_tools`
<!--{ generate.module_header.start() }-->
 [![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/module_error_tools_push.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/module_error_tools_push.yml) [![docs.rs](https://img.shields.io/docsrs/error_tools?color=e3e8f0&logo=docs.rs)](https://docs.rs/error_tools) [![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=module%2Fcore%2Ferror_tools%2Fexamples%2Ferror_tools_trivial.rs,RUN_POSTFIX=--example%20module%2Fcore%2Ferror_tools%2Fexamples%2Ferror_tools_trivial.rs/https://github.com/Wandalen/wTools) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)
<!--{ generate.module_header.end }-->

`error_tools` is a foundational library for error handling in Rust, providing a unified interface over the popular `anyhow` and `thiserror` crates. It simplifies error management by offering clear, consistent patterns for both untyped and typed errors, without requiring you to choose between them at the crate level.

### Key Features

-   **Unified Error Handling:** Use `anyhow`'s flexibility and `thiserror`'s structure through a single, consistent API.
-   **Simple Prelude:** A comprehensive `prelude` makes it easy to import everything you need.
-   **Contextual Errors:** Easily add context to your errors with the `ErrWith` trait.

### How It Works

`error_tools` acts as a facade, re-exporting the core functionalities of `anyhow` and `thiserror` under its `untyped` and `typed` modules, respectively. This allows you to leverage the power of these crates with simplified imports and a consistent feel across your project.

---

### Untyped Errors (like `anyhow`)

For functions where you need flexible, dynamic error handling without defining custom error types for every possible failure, use the `untyped` module. It's a direct pass-through to `anyhow`.

#### Example

This example shows a function that reads a file and can fail in multiple ways, all handled by `error_tools::untyped::Result`.

```rust
// In your code:
use error_tools::untyped::{ Result, Context, format_err };

fn read_and_process_file( path : &str ) -> Result< String >
{
  let content = std::fs::read_to_string( path )
    .context( format_err!( "Failed to read file at '{}'", path ) )?;

  if content.is_empty()
  {
    return Err( format_err!( "File is empty!" ) );
  }

  Ok( content.to_uppercase() )
}
```
> See the full runnable example in [`examples/replace_anyhow.rs`](./examples/replace_anyhow.rs).

---

### Typed Errors (like `thiserror`)

For library code or situations where you want to define a clear, structured contract for possible errors, use the `typed` module. It re-exports `thiserror`'s `Error` derive macro.

#### Example

Here, we define a custom `DataError` enum. The `#[derive(Error)]` macro comes directly from `error_tools`.
**Note:** When using `#[derive(Error)]` or other `thiserror` macros, `thiserror` must be explicitly present in the namespace. This can be achieved by adding `use error_tools::dependency::thiserror;` or `use thiserror;` in your module, depending on your project's setup.

```rust
// In your code:
use error_tools::typed::Error;
use std::path::PathBuf;

// The derive macro is re-exported for convenience.
#[ derive( Debug, Error ) ]
pub enum DataError
{
  #[ error( "I/O error for file: {0}" ) ]
  Io( std::io::Error, PathBuf ),
  #[ error( "Parsing error: {0}" ) ]
  Parse( String ),
}

// Manual implementation of From trait for DataError
impl From< std::io::Error > for DataError
{
  fn from( err : std::io::Error ) -> Self
  {
    DataError::Io( err, PathBuf::new() )
  }
}

fn process_data( path : &PathBuf ) -> Result< i32, DataError >
{
  let content = std::fs::read_to_string( path )
    .map_err( | e | DataError::Io( e, path.clone() ) )?;

  content.trim().parse::< i32 >()
    .map_err( | _ | DataError::Parse( "Could not parse content as integer".into() ) )
}
```
> See the full runnable example in [`examples/replace_thiserror.rs`](./examples/replace_thiserror.rs).

---

### To add to your project

```sh
cargo add error_tools
```

### Try out from the repository

```sh
git clone https://github.com/Wandalen/wTools
cd wTools
cargo run --example error_tools_trivial
# Or try the specific examples
cargo run --example replace_anyhow
cargo run --example replace_thiserror
```
