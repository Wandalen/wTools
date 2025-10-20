<!-- {{# generate.module_header{} #}} -->

# Module :: genfile
[![experimental](https://raster.shields.io/static/v1?label=stability&message=experimental&color=orange&logoColor=eee)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/ModulegenfilePush.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/ModulegenfilePush.yml) [![docs.rs](https://img.shields.io/docsrs/genfile?color=e3e8f0&logo=docs.rs)](https://docs.rs/genfile) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)

A trait-based template processing and file generation library for Rust. **genfile** provides self-contained template archives with parameter storage, pluggable rendering engines, and testable in-memory file systems.

## Features

- **Self-Contained Archives**: Template files with embedded parameters stored inside (JSON/YAML serialization)
- **Binary + Text Support**: Handle both text templates and binary files (images, etc.) with base64 encoding
- **Pluggable Architecture**: Trait-based design for custom value types, renderers, and file systems
- **Testable**: Built-in `MemoryFileSystem` for fast, isolated testing without disk I/O
- **Security**: Path traversal validation prevents directory escape attacks
- **External Content**: Support for `FileRef` and `UrlRef` with custom resolvers and storage backends
- **Template Engine**: Default Handlebars renderer with support for custom engines
- **215 Tests**: Comprehensive test coverage including 27 security tests

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
genfile_core = "0.1"
```

### Basic Example

```rust
use genfile_core::{ TemplateArchive, WriteMode, Value, MemoryFileSystem, HandlebarsRenderer, FileSystem };
use std::path::{ Path, PathBuf };

// Create a template archive
let mut archive = TemplateArchive::new( "my-project" );

// Add template files
archive.add_text_file(
  PathBuf::from( "README.md" ),
  "# {{project_name}}\n\n{{description}}",
  WriteMode::Rewrite
);

archive.add_text_file(
  PathBuf::from( "src/main.rs" ),
  "fn main() {\n  println!(\"Hello from {{project_name}}!\");\n}",
  WriteMode::Rewrite
);

// Set parameter values
archive.set_value( "project_name", Value::String( "MyApp".into() ) );
archive.set_value( "description", Value::String( "A cool application".into() ) );

// Materialize to memory filesystem (for testing)
let renderer = HandlebarsRenderer::new();
let mut fs = MemoryFileSystem::new();

archive.materialize_with_components(
  Path::new( "/output" ),
  &renderer,
  &mut fs
).unwrap();

// Verify generated files
assert!( fs.exists( &PathBuf::from( "/output/README.md" ) ) );
assert_eq!(
  fs.read( &PathBuf::from( "/output/README.md" ) ).unwrap(),
  "# MyApp\n\nA cool application"
);
```

### Sample

<!-- {{# generate.sample{} #}} -->

## Examples

### Archive with Binary Files

```rust
use genfile_core::{ TemplateArchive, FileContent, WriteMode, Value };
use std::path::PathBuf;

let mut archive = TemplateArchive::new( "website" );

// Text template
archive.add_text_file(
  PathBuf::from( "index.html" ),
  "<html><title>{{title}}</title></html>",
  WriteMode::Rewrite
);

// Binary file (PNG header)
archive.add_binary_file(
  PathBuf::from( "logo.png" ),
  vec![ 0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A ]
);

archive.set_value( "title", Value::String( "My Site".into() ) );
```

### Serialization and Persistence

```rust
use genfile_core::TemplateArchive;

// Create and serialize to JSON
let archive = TemplateArchive::new( "config" );
let json = archive.to_json_pretty().unwrap();

// Save to file
std::fs::write( "template.json", json ).unwrap();

// Load from file
let json = std::fs::read_to_string( "template.json" ).unwrap();
let restored = TemplateArchive::from_json( &json ).unwrap();
```

### External Content Sources

```rust,ignore
use genfile_core::{ TemplateArchive, FileRef, UrlRef, WriteMode };
use std::path::PathBuf;

let mut archive = TemplateArchive::new( "docs" );

// Reference external file
archive.add_file_from(
  PathBuf::from( "header.txt" ),
  FileRef::new( PathBuf::from( "/templates/header.hbs" ) ),
  WriteMode::Rewrite
);

// Reference URL
archive.add_file_from(
  PathBuf::from( "footer.txt" ),
  UrlRef::new( "https://example.com/templates/footer.hbs" ),
  WriteMode::Rewrite
);
```

### Custom Storage Backend

```rust,ignore
use genfile_core::{ TemplateArchive, ContentStorage, FileContent, HandlebarsRenderer };
use std::path::Path;

struct CloudStorage;

impl ContentStorage for CloudStorage
{
  fn store( &mut self, path: &Path, content: &FileContent ) -> Result< (), genfile_core::Error >
  {
    // Upload to S3, Azure, etc.
    println!( "Uploading {} to cloud", path.display() );
    Ok( () )
  }
}
```

### Parameter Discovery and Analysis

```rust,ignore
use genfile_core::{ TemplateArchive, ParameterDescriptor, WriteMode };
use std::path::PathBuf;

let mut archive = TemplateArchive::new( "app" );
archive.add_text_file(
  PathBuf::from( "config.txt" ),
  "App: {{app_name}}",
  WriteMode::Rewrite
);

// Add parameter definitions
archive.add_parameter( ParameterDescriptor
{
  parameter: "app_name".into(),
  is_mandatory: true,
  default_value: None,
  description: Some( "Application name".into() ),
});

// Discover parameters used in templates
let discovered = archive.discover_parameters();

// Analyze parameter usage
let usage = archive.analyze_parameter_usage();

// Find undefined parameters
let undefined = archive.get_undefined_parameters();
```

## API Overview

### Core Types

- **`TemplateArchive`** - Main entity for template operations, stores files, parameters, and values
- **`Template<V,R>`** - Alternative API with custom value types and renderers
- **`TemplateFile`** - Individual file with content, metadata, and optional external source
- **`FileContent`** - Enum for `Text(String)` or `Binary(Vec<u8>)`
- **`Value`** - Default parameter value type: `String`, `Number`, `Bool`, `List`

### Traits

- **`TemplateValue`** - Trait for custom parameter value types
- **`TemplateRenderer`** - Trait for pluggable rendering engines (default: Handlebars)
- **`FileSystem`** - Trait for file operations (`RealFileSystem` or `MemoryFileSystem`)
- **`ContentResolver`** - Trait for resolving external content sources
- **`ContentStorage`** - Trait for custom storage backends

### Content Sources

- **`ContentSource::Inline`** - Content embedded directly in archive
- **`ContentSource::File`** - Reference to external file path
- **`ContentSource::Url`** - Reference to remote URL

## Security

All file paths are validated to prevent directory traversal attacks:

```rust
use genfile_core::validate_path;
use std::path::Path;

// Valid paths
assert!( validate_path( Path::new( "src/lib.rs" ) ).is_ok() );
assert!( validate_path( Path::new( "./foo/bar.txt" ) ).is_ok() );

// Invalid paths (rejected)
assert!( validate_path( Path::new( "../etc/passwd" ) ).is_err() );
assert!( validate_path( Path::new( "foo/../../bar" ) ).is_err() );
```

27 dedicated security tests ensure protection against malicious paths.

## Testing

Use `MemoryFileSystem` for fast, isolated tests:

```rust,no_run
use genfile_core::{ TemplateArchive, MemoryFileSystem, HandlebarsRenderer, FileSystem };
use std::path::Path;

#[ test ]
fn test_generation()
{
  let archive = TemplateArchive::new( "test" );
  let renderer = HandlebarsRenderer::new();
  let mut fs = MemoryFileSystem::new();

  // No disk I/O - runs in memory
  archive.materialize_with_components(
    Path::new( "/output" ),
    &renderer,
    &mut fs
  ).unwrap();

  // Fast assertions
  assert!( fs.exists( Path::new( "/output/README.md" ) ) );
}
```

## Links

- [Documentation](https://docs.rs/genfile)
- [Repository](https://github.com/Wandalen/wTools)
- [Discord](https://discord.gg/m3YfbXpUUY)
