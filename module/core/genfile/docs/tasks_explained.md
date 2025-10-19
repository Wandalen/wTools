# Not Implemented Tasks - Detailed Explanations

**Last Updated:** 2025-10-19

This document explains each not-implemented or partial task from the feature table, with examples and implementation guidance.

---

## 1. Path Traversal Validation (Security) ❌

### What It Is

A security check that prevents malicious templates from writing files outside the intended target directory by rejecting file paths that contain `..` (parent directory) segments.

### Why It Matters

**Security Vulnerability:** Without this check, a malicious template could write files anywhere on the system:

```yaml
# Malicious genfile could contain:
files:
  - path: "../../../etc/passwd"  # ← DANGEROUS! Escapes target directory
    content: "malicious content"
```

If someone runs: `archive.materialize("/tmp/output")`, this would try to write to `/etc/passwd` instead of `/tmp/output/...`

### Current Status

❌ **NOT IMPLEMENTED** - The code currently accepts any path without validation.

### What Needs to Be Done

**Add validation function:**

```rust
// src/security.rs (new file)
use std::path::{Path, Component};
use crate::Error;

/// Validates that path doesn't contain directory traversal sequences.
///
/// # Security
///
/// Prevents malicious templates from writing files outside the target
/// directory by rejecting paths containing `..` segments.
///
/// # Errors
///
/// Returns `Error::InvalidTemplate` if path contains `..` segments.
///
/// # Examples
///
/// ```
/// # use genfile_core::security::validate_path;
/// # use std::path::Path;
/// // Valid paths
/// assert!(validate_path(Path::new("foo/bar.txt")).is_ok());
/// assert!(validate_path(Path::new("src/main.rs")).is_ok());
///
/// // Invalid paths - contain ".."
/// assert!(validate_path(Path::new("../etc/passwd")).is_err());
/// assert!(validate_path(Path::new("foo/../../etc/passwd")).is_err());
/// ```
pub fn validate_path(path: &Path) -> Result<(), Error>
{
  for component in path.components()
  {
    if component == Component::ParentDir
    {
      return Err(Error::InvalidTemplate(
        format!("Path contains directory traversal: {}", path.display())
      ));
    }
  }
  Ok(())
}

#[cfg(test)]
mod tests
{
  use super::*;
  use std::path::Path;

  #[test]
  fn test_rejects_parent_dir()
  {
    assert!(validate_path(Path::new("../etc/passwd")).is_err());
    assert!(validate_path(Path::new("foo/../../bar")).is_err());
  }

  #[test]
  fn test_allows_normal_paths()
  {
    assert!(validate_path(Path::new("foo/bar.txt")).is_ok());
    assert!(validate_path(Path::new("src/main.rs")).is_ok());
    assert!(validate_path(Path::new("./foo/bar")).is_ok());
  }

  #[test]
  fn test_allows_absolute_paths()
  {
    // Absolute paths are OK - they're resolved from target directory
    assert!(validate_path(Path::new("/foo/bar")).is_ok());
  }
}
```

**Integrate into materialization:**

```rust
// In src/archive.rs, materialize() method:
for file in &self.files
{
  // Validate path before writing
  validate_path(&file.path)?;

  // ... rest of materialization logic
}
```

### Effort Estimate

**1-2 hours:**
- 30 minutes to write validation function + tests
- 30 minutes to integrate into materialization
- 30 minutes to test with malicious paths

### Priority

🔴 **CRITICAL - HIGH PRIORITY**

This is a security vulnerability. Must be fixed before 1.0 release.

---

## 2. README.md Improvements (Docs) ⚠️

### What It Is

Enhancing the main README.md file with quick start guides, common use case examples, and better onboarding content.

### Why It Matters

**User Onboarding:** The README is the first thing users see. A good README with examples helps users:
- Understand what the library does in 30 seconds
- Get started quickly with copy-paste examples
- Find common use cases without reading full docs

### Current Status

⚠️ **PARTIAL** - Basic README exists but lacks comprehensive examples and quick start guide.

### What's Missing

**1. Quick Start Section:**

```markdown
## Quick Start

...rust
use genfile_core::{TemplateArchive, Value};
use std::path::{Path, PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>>
{
  // Create a new template archive
  let mut archive = TemplateArchive::new("my-template");

  // Add a template file
  archive.add_text_file(
    PathBuf::from("greeting.txt"),
    "Hello, {{name}}! Welcome to {{project}}.",
    WriteMode::Rewrite
  );

  // Set parameter values
  archive.set_value("name", Value::String("Alice".into()));
  archive.set_value("project", Value::String("genfile_core".into()));

  // Save the genfile for reuse
  archive.save_to_file("my-template.yaml")?;

  // Later: load and materialize
  let loaded = TemplateArchive::load_from_file("my-template.yaml")?;
  loaded.materialize(Path::new("./output"))?;

  println!("Files generated in ./output/");
  Ok(())
}
...
```

**2. Common Use Cases:**

```markdown
## Common Use Cases

### Use Case 1: Project Scaffolding

Generate a new Rust project structure:

...rust
let mut scaffold = TemplateArchive::new("rust-project");

scaffold.add_text_file(
  PathBuf::from("Cargo.toml"),
  r#"[package]
name = "{{project_name}}"
version = "{{version}}"
edition = "2021"
"#,
  WriteMode::Rewrite
);

scaffold.add_text_file(
  PathBuf::from("src/main.rs"),
  r#"fn main() {
    println!("Hello from {{project_name}}!");
}
"#,
  WriteMode::Rewrite
);

scaffold.set_value("project_name", Value::String("my-app".into()));
scaffold.set_value("version", Value::String("0.1.0".into()));

scaffold.materialize(Path::new("./my-app"))?;
...

### Use Case 2: Configuration File Generation

Generate config files with default values:

...rust
let mut config = TemplateArchive::new("app-config");

config.add_text_file(
  PathBuf::from("config.yaml"),
  r#"database:
  host: {{db_host}}
  port: {{db_port}}

server:
  port: {{server_port}}
  workers: {{workers}}
"#,
  WriteMode::Rewrite
);

config.set_value("db_host", Value::String("localhost".into()));
config.set_value("db_port", Value::Number(5432));
config.set_value("server_port", Value::Number(8080));
config.set_value("workers", Value::Number(4));

config.materialize(Path::new("."))?;
...

### Use Case 3: Binary Files

Include binary files (images, archives, etc.) in templates:

...rust
let mut archive = TemplateArchive::new("web-assets");

// Text template
archive.add_text_file(
  PathBuf::from("index.html"),
  "<h1>{{title}}</h1>",
  WriteMode::Rewrite
);

// Binary file (e.g., logo image)
let logo_bytes = std::fs::read("logo.png")?;
archive.add_binary_file(
  PathBuf::from("logo.png"),
  logo_bytes,
  WriteMode::Rewrite
);

archive.materialize(Path::new("./website"))?;
...
```

**3. Feature Highlights:**

```markdown
## Features

- ✅ **Self-Contained Templates**: Parameter values stored inside genfile (YAML/JSON)
- ✅ **Binary File Support**: Full support for all byte values (0x00-0xFF)
- ✅ **Trait-Based Architecture**: Pluggable renderers, custom value types, testable filesystem
- ✅ **Multiple Serialization Formats**: JSON and YAML support
- ✅ **External References**: Reference content via FileRef/UrlRef to avoid duplication
- ✅ **In-Memory Testing**: MemoryFileSystem for fast, clean tests
- ✅ **Handlebars Rendering**: Powerful template syntax with conditionals and loops
```

### Effort Estimate

**2-4 hours:**
- 1 hour to write quick start section
- 1-2 hours to create common use case examples
- 30 minutes to add feature highlights
- 30 minutes to polish and test examples

### Priority

🟡 **MEDIUM** - Helps user onboarding significantly

---

## 3. Test Coverage Measurement (NFR3) ⚠️

### What It Is

Running a code coverage tool (like `cargo tarpaulin`) to measure what percentage of code lines are executed by tests.

### Why It Matters

**Quality Metric:** Helps identify:
- Untested code paths
- Dead code
- Missing edge case tests
- Overall test quality

The specification requires ≥80% line coverage.

### Current Status

⚠️ **NOT MEASURED** - 188 tests exist (142 unit + 46 doc) but coverage percentage is unknown.

### What Needs to Be Done

**1. Install tarpaulin:**

```bash
cargo install cargo-tarpaulin
```

**2. Run coverage:**

```bash
cargo tarpaulin --all-features --out Html --output-dir coverage
```

**3. Check results:**

```bash
# Open coverage report
xdg-open coverage/index.html  # Linux
open coverage/index.html      # macOS

# Or just see percentage in terminal
cargo tarpaulin --all-features
```

**Expected output:**

```
|| Tested/Total Lines:
|| src/archive.rs: 245/256 (95.7%)
|| src/renderer.rs: 42/45 (93.3%)
|| src/filesystem.rs: 78/82 (95.1%)
|| ...
||
|| Total: 1234/1350 (91.4%)  ← Overall coverage %
```

**4. If coverage < 80%, add tests for uncovered lines:**

The HTML report shows exactly which lines aren't covered. For example:

```rust
// If report shows line 42 is not covered:
pub fn some_function(x: i32) -> Result<i32, Error>
{
  if x < 0
  {
    return Err(Error::InvalidTemplate("negative".into()));  // ← Line 42 not covered
  }
  Ok(x * 2)
}

// Add test:
#[test]
fn test_negative_input()
{
  assert!(some_function(-5).is_err());  // ← Now line 42 is covered
}
```

### Effort Estimate

**30 minutes:**
- 5 minutes to install tarpaulin
- 5 minutes to run coverage
- 10 minutes to analyze results
- 10 minutes to document coverage % (likely already ≥80%)

### Priority

🟡 **MEDIUM** - Easy win, good to know actual coverage

---

## 4. API Documentation (Docs/NFR5) ⚠️

### What It Is

Completing doc comments for all public API items (functions, structs, enums, traits, methods, fields).

### Why It Matters

**Library Usability:** Good documentation helps users:
- Understand what each type/function does
- Know what parameters mean
- See example usage
- Understand error conditions

The specification requires: "Every public API item must have doc comments explaining purpose, parameters, return values, and errors."

### Current Status

⚠️ **PARTIAL** - Most public items are documented, but some gaps remain:
- Some struct fields lack doc comments
- Some methods have minimal documentation
- Some modules lack module-level docs
- Some examples could be better

### What Needs to Be Done

**1. Check for missing docs:**

```bash
RUSTDOCFLAGS="-D missing_docs" cargo doc --no-deps --all-features 2>&1 | grep warning
```

**Example output:**

```
warning: missing documentation for a struct field
  --> src/archive.rs:73:3
   |
73 |   pub files: Vec<TemplateFile>,
   |   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

**2. Fix missing docs:**

```rust
// Before (missing doc):
pub struct TemplateArchive
{
  pub name: String,
  pub files: Vec<TemplateFile>,  // ← No doc comment
}

// After (documented):
pub struct TemplateArchive
{
  /// The name of this template archive.
  ///
  /// Used for identification and display purposes.
  pub name: String,

  /// The collection of files in this archive.
  ///
  /// Each file contains its path, content, and metadata.
  pub files: Vec<TemplateFile>,
}
```

**3. Add module-level docs:**

```rust
// At the top of src/archive.rs:

//! Template archive types and operations.
//!
//! This module provides the main `TemplateArchive` type which represents
//! a self-contained collection of template files with parameters and values.
//!
//! # Examples
//!
//! ```
//! use genfile_core::TemplateArchive;
//!
//! let mut archive = TemplateArchive::new("my-template");
//! // ... use archive
//! ```
```

**4. Improve method docs:**

```rust
// Before (minimal):
/// Materializes the archive.
pub fn materialize(&self, path: &Path) -> Result<(), Error>

// After (detailed):
/// Generates all files from this archive to the filesystem.
///
/// # Arguments
///
/// * `path` - The target directory where files will be generated. Will be
///   created if it doesn't exist.
///
/// # Returns
///
/// Returns `Ok(())` on success, or an error if:
/// - The target path cannot be created
/// - A file cannot be written
/// - A template fails to render
/// - A mandatory parameter is undefined
///
/// # Examples
///
/// ```
/// use genfile_core::TemplateArchive;
/// use std::path::Path;
///
/// let archive = TemplateArchive::new("example");
/// archive.materialize(Path::new("./output"))?;
/// ```
///
/// # Errors
///
/// - `Error::MissingParameters` if mandatory parameters are undefined
/// - `Error::Render` if template rendering fails
/// - `Error::Fs` if filesystem operations fail
pub fn materialize(&self, path: &Path) -> Result<(), Error>
```

**5. Add examples to complex APIs:**

```rust
/// Adds a file from an external content source.
///
/// # Examples
///
/// ```
/// use genfile_core::{TemplateArchive, FileRef};
/// use std::path::PathBuf;
///
/// let mut archive = TemplateArchive::new("example");
///
/// // Reference external file
/// archive.add_file_from(
///   PathBuf::from("readme.md"),
///   FileRef::new("./templates/readme.hbs"),
///   WriteMode::Rewrite
/// );
/// ```
pub fn add_file_from<S>(&mut self, path: PathBuf, source: S, mode: WriteMode)
where S: IntoContentSource
```

### Effort Estimate

**4-8 hours:**
- 1 hour to identify all missing docs
- 2-4 hours to write comprehensive doc comments
- 1-2 hours to add examples
- 1 hour to review and polish

### Priority

🟡 **MEDIUM** - Important for library quality and usability

---

## 5. Standalone Examples (Docs) ⚠️

### What It Is

Creating an `examples/` directory with runnable example programs that demonstrate common use cases.

### Why It Matters

**Learning by Example:** Users can:
- Run `cargo run --example basic_template` to see it work
- Copy example code as starting point
- Understand common patterns quickly

### Current Status

⚠️ **PARTIAL** - Examples exist in tests, but no standalone `examples/` directory.

### What Needs to Be Done

**1. Create `examples/` directory:**

```bash
mkdir examples
```

**2. Create basic example:**

```rust
// examples/basic_template.rs

use genfile_core::*;
use std::path::{Path, PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>>
{
  println!("Creating a basic template...");

  // Create template archive
  let mut archive = TemplateArchive::new("hello-world");

  // Add a simple text file
  archive.add_text_file(
    PathBuf::from("greeting.txt"),
    "Hello, {{name}}! Welcome to {{project}}.",
    WriteMode::Rewrite
  );

  // Set parameter values
  archive.set_value("name", Value::String("Alice".into()));
  archive.set_value("project", Value::String("genfile_core".into()));

  // Materialize to filesystem
  let output_dir = Path::new("./example_output");
  archive.materialize(output_dir)?;

  println!("✓ Generated files in {}", output_dir.display());
  println!("✓ Check {}/greeting.txt", output_dir.display());

  Ok(())
}
```

**Run with:**

```bash
cargo run --example basic_template
```

**3. Create binary file example:**

```rust
// examples/binary_files.rs

use genfile_core::*;
use std::path::{Path, PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>>
{
  println!("Creating template with binary files...");

  let mut archive = TemplateArchive::new("mixed-content");

  // Add text file
  archive.add_text_file(
    PathBuf::from("readme.md"),
    "# {{project_name}}\n\nVersion: {{version}}",
    WriteMode::Rewrite
  );

  // Add binary file (simulated PNG header)
  let png_header = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
  archive.add_binary_file(
    PathBuf::from("logo.png"),
    png_header,
    WriteMode::Rewrite
  );

  // Set values
  archive.set_value("project_name", Value::String("MyProject".into()));
  archive.set_value("version", Value::String("1.0.0".into()));

  // Save genfile for reuse
  archive.save_to_file("mixed-template.yaml")?;
  println!("✓ Saved genfile to mixed-template.yaml");

  // Materialize
  archive.materialize(Path::new("./binary_output"))?;
  println!("✓ Generated files in ./binary_output/");

  Ok(())
}
```

**4. Create external references example:**

```rust
// examples/external_references.rs

use genfile_core::*;
use std::path::{Path, PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>>
{
  println!("Creating template with external file references...");

  let mut archive = TemplateArchive::new("external-refs");

  // Reference external template file instead of embedding
  archive.add_file_from(
    PathBuf::from("readme.md"),
    FileRef::new("./templates/readme.hbs"),
    WriteMode::Rewrite
  );

  // Set parameters
  archive.set_value("project", Value::String("MyApp".into()));

  // Save genfile (content stays external, only reference stored)
  archive.save_to_file("external-template.yaml")?;
  println!("✓ Saved genfile with external references");

  Ok(())
}
```

**5. Create parameter discovery example:**

```rust
// examples/parameter_discovery.rs

use genfile_core::*;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>>
{
  println!("Discovering parameters in templates...");

  let mut archive = TemplateArchive::new("discovery-demo");

  // Add file with multiple parameters
  archive.add_text_file(
    PathBuf::from("config.yaml"),
    r#"
app_name: {{app_name}}
version: {{version}}
author: {{author}}
database:
  host: {{db_host}}
  port: {{db_port}}
"#,
    WriteMode::Rewrite
  );

  // Discover all parameters used in templates
  let discovered = archive.discover_parameters();
  println!("\nDiscovered parameters:");
  for param in discovered
  {
    println!("  - {}", param);
  }

  // Check which parameters are undefined
  let undefined = archive.get_undefined_parameters();
  println!("\nUndefined parameters:");
  for param in undefined
  {
    println!("  - {} (needs a value)", param);
  }

  Ok(())
}
```

**6. Update Cargo.toml:**

```toml
# Add to Cargo.toml:
[[example]]
name = "basic_template"
path = "examples/basic_template.rs"

[[example]]
name = "binary_files"
path = "examples/binary_files.rs"

[[example]]
name = "external_references"
path = "examples/external_references.rs"

[[example]]
name = "parameter_discovery"
path = "examples/parameter_discovery.rs"
```

**7. Add examples/README.md:**

```markdown
# genfile_core Examples

This directory contains example programs demonstrating common use cases.

## Running Examples

```bash
# Basic template creation and materialization
cargo run --example basic_template

# Working with binary files
cargo run --example binary_files

# Using external file references
cargo run --example external_references

# Parameter discovery
cargo run --example parameter_discovery
```

## Example Descriptions

- **basic_template.rs** - Simple template creation, parameter setting, and materialization
- **binary_files.rs** - Mixing text and binary content in templates
- **external_references.rs** - Using FileRef to avoid duplicating large content
- **parameter_discovery.rs** - Discovering and validating template parameters
```

### Effort Estimate

**2-4 hours:**
- 1 hour to create 4-5 examples
- 30 minutes to test all examples work
- 30 minutes to write examples/README.md
- 30 minutes to polish and document

### Priority

🟡 **MEDIUM** - Helps users learn quickly

---

## 6. Performance Benchmarks (NFR1) ⚠️

### What It Is

Creating benchmark tests to measure template rendering performance and verify it meets the requirement: "Template rendering must complete within 100ms for templates up to 10KB with up to 50 parameters."

### Why It Matters

**Performance Assurance:** Ensures the library:
- Performs fast enough for production use
- Doesn't have performance regressions
- Meets specification requirements

### Current Status

⚠️ **NOT MEASURED** - Implementation likely already fast, but never benchmarked.

### What Needs to Be Done

**1. Add criterion to dev-dependencies:**

```toml
# Cargo.toml
[dev-dependencies]
criterion = "0.5"

[[bench]]
name = "template_rendering"
harness = false
```

**2. Create benchmark:**

```rust
// benches/template_rendering.rs

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use genfile_core::*;
use std::path::{Path, PathBuf};

fn create_large_template(size_kb: usize, param_count: usize) -> TemplateArchive
{
  let mut archive = TemplateArchive::new("bench");

  // Create template with specified size
  let mut template_content = String::new();
  for i in 0..param_count
  {
    template_content.push_str(&format!("param_{}: {{{{param_{}}}}}\n", i, i));
  }

  // Pad to desired size
  while template_content.len() < size_kb * 1024
  {
    template_content.push_str("padding padding padding padding\n");
  }

  archive.add_text_file(
    PathBuf::from("large.txt"),
    &template_content,
    WriteMode::Rewrite
  );

  // Set all parameter values
  for i in 0..param_count
  {
    archive.set_value(
      &format!("param_{}", i),
      Value::String(format!("value_{}", i))
    );
  }

  archive
}

fn bench_rendering(c: &mut Criterion)
{
  let mut group = c.benchmark_group("template_rendering");

  // Benchmark: 10KB template with 50 parameters (specification requirement)
  group.bench_function("10kb_50params", |b| {
    let archive = create_large_template(10, 50);
    let output = Path::new("/tmp/bench_output");

    b.iter(|| {
      archive.materialize(black_box(output)).unwrap();
    });
  });

  // Benchmark: Smaller templates for comparison
  group.bench_function("1kb_10params", |b| {
    let archive = create_large_template(1, 10);
    let output = Path::new("/tmp/bench_output");

    b.iter(|| {
      archive.materialize(black_box(output)).unwrap();
    });
  });

  group.finish();
}

criterion_group!(benches, bench_rendering);
criterion_main!(benches);
```

**3. Run benchmarks:**

```bash
cargo bench
```

**Expected output:**

```
template_rendering/10kb_50params
                        time:   [42.3 ms 43.1 ms 43.9 ms]  ← Should be <100ms
template_rendering/1kb_10params
                        time:   [8.2 ms 8.5 ms 8.8 ms]
```

**4. If performance doesn't meet requirements, profile and optimize:**

```bash
# Profile with flamegraph
cargo install flamegraph
cargo flamegraph --bench template_rendering

# Or use perf
perf record cargo bench
perf report
```

### Effort Estimate

**2-3 hours:**
- 1 hour to create benchmarks
- 30 minutes to run and analyze results
- 1 hour for optimization if needed (unlikely)

### Priority

🟢 **LOW** - Nice to have, likely already fast enough

---

## 7. Memory Profiling (NFR2) ⚠️

### What It Is

Measuring heap memory usage during template operations to verify the requirement: "In-memory operations must not allocate more than 10MB for up to 100 files with 1MB total content."

### Why It Matters

**Resource Efficiency:** Ensures the library:
- Doesn't leak memory
- Uses memory efficiently
- Suitable for resource-constrained environments

### Current Status

⚠️ **NOT MEASURED** - Implementation likely efficient (simple architecture), but never profiled.

### What Needs to Be Done

**1. Create memory profiling test:**

```rust
// tests/memory_profiling.rs

#[test]
#[ignore] // Run manually: cargo test --test memory_profiling -- --ignored
fn test_memory_usage_100_files()
{
  use genfile_core::*;
  use std::path::Path;

  // Create archive with 100 files, ~10KB each (1MB total)
  let mut archive = TemplateArchive::new("memory-test");

  for i in 0..100
  {
    let content = "x".repeat(10 * 1024); // 10KB per file
    archive.add_text_file(
      PathBuf::from(format!("file_{}.txt", i)),
      &content,
      WriteMode::Rewrite
    );
  }

  // Materialize (should use <10MB heap)
  archive.materialize(Path::new("/tmp/memory_test")).unwrap();

  // Note: Actual memory measurement requires external tools
  println!("Memory test completed. Use valgrind/heaptrack to measure.");
}
```

**2. Run with valgrind massif:**

```bash
# Build test
cargo test --test memory_profiling --no-run

# Run with valgrind
valgrind --tool=massif --massif-out-file=massif.out \
  ./target/debug/deps/memory_profiling-* --ignored

# Analyze results
ms_print massif.out | head -50
```

**Expected output:**

```
--------------------------------------------------------------------------------
  KB
9.766^                                                                       #
     |                                                                       #
     |                                                                       #
     |                                                                       #
     |                                                                       #
     |                                                                       #
     |                                                                       #
     |                                                                       #
     |                                                                       #
     |                                                                       #
   0 +----------------------------------------------------------------------->KB
     0                                                                   100

Peak: 9.766 MB  ← Should be <10MB
```

**3. Alternative: Use heaptrack:**

```bash
# Install heaptrack
sudo apt install heaptrack  # Ubuntu/Debian

# Run profiling
heaptrack cargo test --test memory_profiling -- --ignored

# Analyze
heaptrack_print heaptrack.*.gz | head -100
```

### Effort Estimate

**2-3 hours:**
- 1 hour to create profiling test
- 1 hour to run valgrind/heaptrack
- 30 minutes to analyze results
- 30 minutes for optimization if needed (unlikely)

### Priority

🟢 **LOW** - Nice to have, likely already efficient

---

## 8. Compilation Time (NFR4) ⚠️

### What It Is

Measuring the impact of adding genfile_core as a dependency on clean build times of dependent projects. Requirement: "Must not increase clean build time by more than 5 seconds."

### Why It Matters

**Developer Experience:** Fast builds are important for:
- Developer productivity
- CI/CD pipeline speed
- Quick iteration cycles

### Current Status

⚠️ **NOT MEASURED** - genfile_core has minimal dependencies, so impact likely small.

### What Needs to Be Done

**1. Measure baseline willbe build time (before genfile_core):**

```bash
cd ../willbe
cargo clean
time cargo build --release
```

**Output example:**

```
...
Finished release [optimized] target(s) in 3m 42s

real    3m42.156s  ← Baseline
user    18m32.441s
sys     0m54.322s
```

**2. Add genfile_core dependency and measure again:**

```bash
# After adding genfile_core to Cargo.toml
cargo clean
time cargo build --release
```

**Output example:**

```
Finished release [optimized] target(s) in 3m 46s

real    3m46.891s  ← With genfile_core
user    19m01.234s
sys     0m56.123s

Difference: +4.7s ← Should be <5s
```

**3. If difference >5s, investigate:**

```bash
# Check dependency tree
cargo tree -p genfile_core

# Find slow dependencies
cargo build --release --timings
# Opens HTML report showing compile times
```

### Effort Estimate

**30 minutes:**
- 10 minutes to measure baseline
- 10 minutes to measure with genfile_core
- 10 minutes to analyze results

**Note:** Will be measured during willbe integration anyway.

### Priority

🟢 **LOW** - Will measure during integration, likely already fast

---

## 9 & 14. willbe Integration (SM1/US1) ❌

### What It Is

Integrating genfile_core into the willbe project by replacing the existing `template.rs` module (472 lines) with genfile_core as a dependency.

### Why It Matters

**Primary Goal:** This is the main reason genfile_core was created - to extract template logic from willbe into a reusable library.

**Benefits:**
- Removes 472 lines of duplicated code from willbe
- Makes template functionality reusable by other projects
- Improves maintainability (one place to fix bugs)
- Better testing (genfile_core has 188 tests)

### Current Status

❌ **NOT DONE** - Blocked on willbe team decision.

genfile_core is complete and ready for integration, but:
- Requires willbe team approval
- Needs coordination with willbe developers
- Must ensure all willbe tests still pass

### What Needs to Be Done

**1. Add genfile_core to willbe's Cargo.toml:**

```toml
[dependencies]
genfile_core = { path = "../genfile" }
```

**2. Find all uses of template.rs in willbe:**

```bash
cd ../willbe
grep -r "template::" src/
grep -r "use.*template" src/
```

**3. Replace with genfile_core:**

```rust
// Before (willbe's template.rs):
use crate::template::{Template, render};

let template = Template::new(files);
template.render(values)?;

// After (genfile_core):
use genfile_core::{TemplateArchive, Value};

let mut archive = TemplateArchive::new("willbe-template");
archive.add_text_file(path, content, WriteMode::Rewrite);
archive.set_value("key", Value::String(value));
archive.materialize(output_dir)?;
```

**4. Create adapter for wca::Value:**

```rust
// In willbe, create src/genfile_adapter.rs:

impl From<wca::Value> for genfile_core::Value
{
  fn from(v: wca::Value) -> Self
  {
    match v
    {
      wca::Value::String(s) => genfile_core::Value::String(s),
      wca::Value::Number(n) => genfile_core::Value::Number(n),
      wca::Value::Bool(b) => genfile_core::Value::Bool(b),
      wca::Value::List(l) => genfile_core::Value::List(
        l.into_iter().map(|x| x.to_string()).collect()
      ),
    }
  }
}
```

**5. Update all template usage sites in willbe:**

Likely sites:
- `workspace_renew` command
- `deploy_renew` command
- `cicd_renew` command
- Any other code generation commands

**6. Run all willbe tests:**

```bash
cd ../willbe
cargo test --all-features
```

**7. Remove old template.rs:**

```bash
git rm src/template.rs  # 472 lines removed
```

**8. Verify zero regressions:**

All willbe tests must pass. This verifies SM3 (Zero Regressions).

### Effort Estimate

**Unknown (requires willbe team involvement):**
- 2-4 hours to update code
- 1-2 hours to test and fix issues
- 1 hour to coordinate with team

**Total: ~4-7 hours** (but requires willbe team availability)

### Priority

🔴 **HIGH** - But blocked on external team decision

This is a success metric and user story, but genfile_core is complete. The library is ready - just waiting for willbe team to integrate it.

---

## 10. Test Coverage ≥80% (SM2) ⚠️

### What It Is

Success metric that test coverage should be at least 80%.

**Same as Task #3 (NFR3: Test Coverage Measurement)**

See Task #3 for full details.

### Current Status

⚠️ **NOT MEASURED** - Likely already meets ≥80%, just needs measurement.

### Priority

🟡 **MEDIUM** - Same as Task #3

---

## 11. Zero Regressions (SM3) N/A

### What It Is

Success metric: "All existing willbe tests continue passing after migration to genfile_core."

### Why It Matters

**Quality Assurance:** Ensures that replacing willbe's template.rs with genfile_core doesn't break anything.

### Current Status

**N/A - Cannot test until integration happens**

This will be verified during Task #9 (willbe Integration).

### What Needs to Be Done

**During willbe integration:**

```bash
# Before integration
cd ../willbe
cargo test --all-features > before_tests.txt

# After integration (with genfile_core)
cargo test --all-features > after_tests.txt

# Compare
diff before_tests.txt after_tests.txt
```

**Expected:** All tests pass, no new failures.

If tests fail:
1. Investigate failure
2. Fix adapter code or genfile_core API usage
3. Re-run tests
4. Repeat until all tests pass

### Effort Estimate

**Included in Task #9 (willbe Integration)**

### Priority

N/A - Will test during integration

---

## 12. Performance Matches willbe (SM4) ⚠️

### What It Is

Success metric: "Template rendering and file generation performance matches or exceeds current willbe implementation (within 5% variance)."

### Why It Matters

**Performance Regression Prevention:** Ensures genfile_core isn't slower than willbe's original implementation.

### Current Status

⚠️ **NOT MEASURED** - Cannot measure until willbe integration.

### What Needs to Be Done

**1. Benchmark willbe's current implementation:**

```bash
cd ../willbe

# Create benchmark test
cat > benches/template_bench.rs << 'EOF'
use criterion::*;

fn bench_workspace_renew(c: &mut Criterion)
{
  c.bench_function("workspace_renew", |b| {
    b.iter(|| {
      // Run willbe's current template code
      willbe::template::workspace_renew(...);
    });
  });
}

criterion_group!(benches, bench_workspace_renew);
criterion_main!(benches);
EOF

cargo bench
```

**Output:**

```
workspace_renew        time:   [45.2 ms 46.1 ms 47.0 ms]  ← Baseline
```

**2. After integration, benchmark genfile_core version:**

```bash
cargo bench
```

**Output:**

```
workspace_renew        time:   [43.8 ms 44.5 ms 45.2 ms]  ← With genfile_core
                        change: [-5.1% -3.5% -1.9%]         ← Within 5%!
```

**3. If performance regression >5%, profile and optimize:**

```bash
cargo flamegraph --bench template_bench
# Analyze flamegraph to find bottlenecks
```

### Effort Estimate

**2-3 hours (during willbe integration):**
- 1 hour to create benchmarks
- 1 hour to compare results
- 1 hour for optimization if needed

### Priority

🟢 **LOW** - Will measure during integration, likely already fast

---

## 13. Reusability (2+ Projects) (SM5) ❌

### What It Is

Success metric: "At least one additional wTools project beyond willbe adopts genfile for file generation needs."

### Why It Matters

**Validation of Reusability:** Proves that genfile_core is truly reusable and valuable to multiple projects, not just willbe-specific.

### Current Status

❌ **NOT ACHIEVED** - No other wTools projects are using genfile_core yet.

### What Needs to Be Done

**Identify candidate projects:**

```bash
# Find wTools projects that might need templates
cd $PRO/lib/wTools
find . -name "*.rs" -exec grep -l "template\|scaffold\|generate" {} \; | head -20

# Or look for code generation needs
grep -r "write.*file" --include="*.rs" | grep -i "config\|template"
```

**Possible candidates:**
- Project scaffolding tools
- Configuration generators
- Code generators
- Documentation generators

**Approach a project team:**

1. Identify a project that generates files
2. Propose using genfile_core
3. Help integrate it
4. Get at least one project using it

### Effort Estimate

**Unknown - depends on finding a willing project**

This happens organically as other projects discover they need template functionality.

### Priority

🟢 **LOW** - Nice to have, but not critical

Success metric will be achieved naturally over time as genfile_core becomes known.

---

## Summary by Priority

### 🔴 Critical (Do Immediately)

1. **Path traversal validation** (1-2h) - Security vulnerability

### 🟡 Medium Priority (Do for 1.0 Release)

2. **README.md improvements** (2-4h) - User onboarding
3. **Test coverage measurement** (30min) - Easy win
4. **API documentation** (4-8h) - Library quality
5. **Standalone examples** (2-4h) - User learning

### 🟢 Low Priority (Nice to Have)

6. **Performance benchmarks** (2-3h) - Likely already fast
7. **Memory profiling** (2-3h) - Likely already efficient
8. **Compilation time** (30min) - Measure during integration

### ⏸️ Blocked/External

9. **willbe Integration** (4-7h) - Waiting on team decision
10. **Zero regressions** (N/A) - Part of integration
11. **Performance vs willbe** (2-3h) - Part of integration
12. **Reusability** (Unknown) - Organic growth

---

## Estimated Total Effort

**To reach 95% maturity (priorities 1-5):**
- Critical: 1-2 hours
- Medium: 10-20 hours
- **Total: 11-22 hours**

After completing priorities 1-5, genfile_core will be ready for 1.0 release.
