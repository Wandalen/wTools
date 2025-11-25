# Specification: crates_tools

## Overview

**crates_tools** is a crate archive analysis utility providing functionality to download, read, and decode `.crate` archive files from crates.io and local filesystem. It enables inspection of packaged Rust crates for purposes such as version comparison, pre-publish security auditing, file size analysis, and content verification without requiring extraction to disk.

**Version:** 0.20.0
**Status:** Experimental
**Category:** Development Tools (Crate Analysis)
**Dependents:** Unknown (likely workspace tools for crate inspection)

### Scope

#### Responsibility

Provide the `CrateArchive` struct and associated methods for downloading crate archives from crates.io, reading local `.crate` files, decoding gzip-compressed tar archives, listing archive contents, and accessing file contents as byte slices for analysis and inspection.

#### In-Scope

1. **Crate Archive Reading (CrateArchive)**
   - `CrateArchive` struct - In-memory crate archive representation
   - HashMap<PathBuf, Vec<u8>> internal storage
   - File path to content mapping
   - Debug implementation showing file list
   - Clone, PartialEq, Default derives

2. **Local File Reading**
   - `read(path)` - Read `.crate` file from filesystem
   - Path-based loading
   - Returns io::Result<CrateArchive>
   - Automatic decompression and decoding

3. **Network Download (network feature)**
   - `download(url)` - Download from any URL
   - `download_crates_io(name, version)` - Download from crates.io
   - HTTP GET with timeout (5s read/write)
   - Uses ureq for HTTP client
   - Feature-gated via `network`

4. **Archive Decoding**
   - `decode(bytes)` - Decode raw archive bytes
   - Gzip decompression (flate2)
   - Tar archive extraction (tar)
   - Returns io::Result<CrateArchive>
   - Handles empty archives

5. **Content Access**
   - `list()` - List all file paths in archive
   - `content_bytes(path)` - Get file content by path
   - Returns Option<&[u8]> for content
   - Non-destructive inspection

6. **Feature Architecture**
   - `enabled` - Master switch (default), requires flate2, tar, network
   - `network` - HTTP download support (default, requires ureq)
   - Granular dependency control

7. **Traditional Namespace Organization**
   - own/orphan/exposed/prelude namespaces
   - CrateArchive in prelude
   - Standard pattern

#### Out-of-Scope

1. **NOT Extraction to Disk**
   - No filesystem extraction
   - In-memory only
   - **Rationale:** Use case is inspection, not extraction

2. **NOT Crate Modification**
   - Read-only operations
   - No archive creation/editing
   - **Rationale:** Focus on analysis, not packaging

3. **NOT Metadata Parsing**
   - No Cargo.toml parsing
   - Raw bytes only
   - **Rationale:** Use cargo_metadata for metadata

4. **NOT Version Resolution**
   - No version range resolution
   - Exact version required
   - **Rationale:** Use cargo for dependency resolution

5. **NOT Crate Verification**
   - No checksum verification
   - No signature checking
   - **Rationale:** Trust crates.io verification

6. **NOT Registry API**
   - No crate search
   - No index querying
   - **Rationale:** Use crates.io API directly

7. **NOT Diff Generation**
   - No comparison utilities
   - User implements comparison
   - **Rationale:** Keep scope focused

8. **NOT Async Download**
   - Blocking HTTP only
   - No async/await support
   - **Rationale:** Simplicity, use tokio wrapper if needed

#### Boundaries

- **crates_tools vs cargo**: crates_tools inspects archives; cargo manages dependencies
- **crates_tools vs tar/flate2**: crates_tools specializes for crate format; tar/flate2 are generic
- **crates_tools vs crates.io API**: crates_tools downloads archives; API provides metadata

## Architecture

### Dependency Structure

```
crates_tools
├── External Dependencies
│   ├── flate2 (workspace, optional via enabled) - Gzip decompression
│   ├── tar (workspace, optional via enabled) - Tar archive handling
│   └── ureq (~2.9, optional via network) - HTTP client
└── Dev Dependencies
    └── test_tools (workspace, full) - Testing
```

**Note:** All core dependencies are optional, gated on `enabled` feature

### Module Organization

```
crates_tools
├── lib.rs (single-file implementation)
├── private module
│   └── CrateArchive - Main struct and methods
└── Standard namespaces: own, orphan, exposed, prelude
    └── prelude exports CrateArchive
```

### Feature Architecture

```
enabled (master switch, default)
├── dep:flate2 - Gzip compression
├── dep:tar - Tar archive
└── network (default)
    └── dep:ureq - HTTP client

full = enabled + network (all features)
```

**Default Features:** `enabled` (includes network)

### Data Model

```
CrateArchive(HashMap<PathBuf, Vec<u8>>)
│
├── Keys: File paths within archive
│   └── e.g., "crate-1.0.0/src/lib.rs"
│
└── Values: File contents as bytes
    └── Raw file data, not decoded

Reading Flow:
  File/URL → Raw bytes → Gzip decode → Tar extract → HashMap
```

## Public API

### CrateArchive Struct

```rust
/// Represents a `.crate` archive, a collection of files and contents.
#[derive(Default, Clone, PartialEq)]
pub struct CrateArchive(HashMap<PathBuf, Vec<u8>>);

impl Debug for CrateArchive {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    f.debug_struct("CrateArchive")
      .field("files", &self.0.keys())
      .finish()
  }
}
```

### Local File Reading

```rust
impl CrateArchive {
  /// Reads and decodes a `.crate` archive from filesystem.
  ///
  /// # Arguments
  /// * `path` - Path to the `.crate` file
  ///
  /// # Returns
  /// * `io::Result<Self>` - Archive or IO error
  pub fn read<P: AsRef<Path>>(path: P) -> std::io::Result<Self>;
}
```

### Network Download

```rust
impl CrateArchive {
  /// Downloads and decodes a `.crate` archive from URL.
  ///
  /// # Arguments
  /// * `url` - Full URL to download
  ///
  /// # Returns
  /// * `Result<Self, ureq::Error>` - Archive or network error
  #[cfg(feature = "network")]
  pub fn download<Url: AsRef<str>>(url: Url) -> Result<Self, ureq::Error>;

  /// Downloads from crates.io by name and version.
  ///
  /// # Arguments
  /// * `name` - Crate name (e.g., "serde")
  /// * `version` - Exact version (e.g., "1.0.0")
  ///
  /// # Returns
  /// * `Result<Self, ureq::Error>` - Archive or network error
  #[cfg(feature = "network")]
  pub fn download_crates_io<N, V>(name: N, version: V) -> Result<Self, ureq::Error>
  where
    N: core::fmt::Display,
    V: core::fmt::Display;
}
```

### Archive Decoding

```rust
impl CrateArchive {
  /// Decodes raw bytes representing a `.crate` file.
  ///
  /// Handles gzip decompression and tar extraction.
  ///
  /// # Arguments
  /// * `bytes` - Raw archive bytes
  ///
  /// # Returns
  /// * `io::Result<Self>` - Archive or decode error
  pub fn decode<B: AsRef<[u8]>>(bytes: B) -> std::io::Result<Self>;
}
```

### Content Access

```rust
impl CrateArchive {
  /// Lists all file paths in the archive.
  ///
  /// # Returns
  /// * `Vec<&Path>` - All file paths
  pub fn list(&self) -> Vec<&Path>;

  /// Gets file content by path.
  ///
  /// # Arguments
  /// * `path` - Path within archive
  ///
  /// # Returns
  /// * `Option<&[u8]>` - File bytes or None if not found
  pub fn content_bytes<P: AsRef<Path>>(&self, path: P) -> Option<&[u8]>;
}
```

## Usage Patterns

### Pattern 1: Download and List Contents

```rust
use crates_tools::*;

#[cfg(feature = "enabled")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
  let archive = CrateArchive::download_crates_io("serde", "1.0.0")?;

  for path in archive.list() {
    println!("{}", path.display());
  }

  Ok(())
}
```

### Pattern 2: Read Specific File Content

```rust
use crates_tools::*;

#[cfg(feature = "enabled")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
  let archive = CrateArchive::download_crates_io("test_experimental_c", "0.1.0")?;

  for path in archive.list() {
    let bytes = archive.content_bytes(path).unwrap();
    let content = std::str::from_utf8(bytes)?;
    println!("# {}\n```\n{}```", path.display(), content);
  }

  Ok(())
}
```

### Pattern 3: Read Local Crate File

```rust
use crates_tools::*;

#[cfg(feature = "enabled")]
fn main() -> std::io::Result<()> {
  let archive = CrateArchive::read("./my-crate-1.0.0.crate")?;

  println!("Files in archive: {:?}", archive);

  Ok(())
}
```

### Pattern 4: Compare Crate Versions

```rust
use crates_tools::*;

#[cfg(feature = "enabled")]
fn compare_versions(name: &str, v1: &str, v2: &str) -> Result<(), Box<dyn std::error::Error>> {
  let archive1 = CrateArchive::download_crates_io(name, v1)?;
  let archive2 = CrateArchive::download_crates_io(name, v2)?;

  let files1: std::collections::HashSet<_> = archive1.list().into_iter().collect();
  let files2: std::collections::HashSet<_> = archive2.list().into_iter().collect();

  // Files added in v2
  for path in files2.difference(&files1) {
    println!("+ {}", path.display());
  }

  // Files removed in v2
  for path in files1.difference(&files2) {
    println!("- {}", path.display());
  }

  Ok(())
}
```

### Pattern 5: Pre-Publish Security Audit

```rust
use crates_tools::*;

#[cfg(feature = "enabled")]
fn audit_for_secrets(archive: &CrateArchive) -> Vec<&std::path::Path> {
  let suspicious_patterns = [".env", "secret", "password", "api_key", "private_key"];

  archive.list().into_iter().filter(|path| {
    let path_str = path.to_string_lossy().to_lowercase();
    suspicious_patterns.iter().any(|pattern| path_str.contains(pattern))
  }).collect()
}
```

### Pattern 6: Size Analysis

```rust
use crates_tools::*;

#[cfg(feature = "enabled")]
fn analyze_sizes(archive: &CrateArchive) {
  let mut sizes: Vec<_> = archive.list().iter().map(|path| {
    let size = archive.content_bytes(path).map(|b| b.len()).unwrap_or(0);
    (path, size)
  }).collect();

  sizes.sort_by(|a, b| b.1.cmp(&a.1));

  println!("Largest files:");
  for (path, size) in sizes.iter().take(10) {
    println!("  {:>8} bytes: {}", size, path.display());
  }
}
```

### Pattern 7: Extract Specific File

```rust
use crates_tools::*;
use std::fs;

#[cfg(feature = "enabled")]
fn extract_cargo_toml(archive: &CrateArchive, output: &str) -> std::io::Result<()> {
  // Find Cargo.toml (usually in crate-version/ directory)
  let cargo_toml = archive.list().into_iter()
    .find(|p| p.ends_with("Cargo.toml"))
    .ok_or_else(|| std::io::Error::new(
      std::io::ErrorKind::NotFound,
      "Cargo.toml not found"
    ))?;

  let content = archive.content_bytes(cargo_toml)
    .ok_or_else(|| std::io::Error::new(
      std::io::ErrorKind::NotFound,
      "Content not available"
    ))?;

  fs::write(output, content)?;
  Ok(())
}
```

### Pattern 8: Decode From Memory

```rust
use crates_tools::*;

#[cfg(feature = "enabled")]
fn from_downloaded_bytes(bytes: Vec<u8>) -> std::io::Result<CrateArchive> {
  CrateArchive::decode(&bytes)
}
```

## Dependencies and Consumers

### Direct Dependencies

**External:**
- `flate2` (workspace, optional) - Gzip compression/decompression
- `tar` (workspace, optional) - Tar archive reading
- `ureq` (~2.9, optional) - Blocking HTTP client

**Dev:**
- `test_tools` (workspace, full) - Testing utilities

### Consumers (Unknown)

**Likely used by:**
- willbe (workspace build tool)
- Crate publishing pipelines
- Security audit tools
- Documentation generators
- Size analysis tools
- Version comparison tools

**Usage Pattern:** Workspace tools use crates_tools to download and inspect published crate archives for analysis, verification, or comparison purposes.

## Design Rationale

### Why HashMap Storage?

Files stored in HashMap<PathBuf, Vec<u8>>:

**Rationale:**
1. **Random Access**: Fast O(1) file lookup
2. **Iteration**: Can list all files
3. **Memory**: Entire archive in memory
4. **Simplicity**: Standard collection
5. **Ownership**: Clear ownership model

**Tradeoff:** Memory usage for large archives

### Why Blocking HTTP?

Uses blocking ureq, not async:

**Rationale:**
1. **Simplicity**: No runtime needed
2. **Use Case**: Download typically one-off
3. **Dependencies**: Minimal
4. **Integration**: Easy to wrap in async

**Alternative:** Add async feature with reqwest

### Why External flate2/tar?

Uses external crates for decompression:

**Rationale:**
1. **Correctness**: Proven implementations
2. **Performance**: Optimized C bindings
3. **Maintenance**: Community maintained
4. **Features**: Full format support

**Pattern:** Standard approach in Rust ecosystem

### Why Download from URL?

Both generic URL and crates.io specific:

**Rationale:**
1. **Flexibility**: Any crate host works
2. **Convenience**: crates.io shortcut
3. **Testing**: Local/custom registries
4. **Future**: Other registries possible

**Pattern:** Generic with convenient specialization

### Why Exact Version Required?

No version range support:

**Rationale:**
1. **Simplicity**: Direct URL construction
2. **Determinism**: Specific archive
3. **Use Case**: Inspection of known version
4. **Resolution**: User does version resolution

**Alternative:** Integrate with crates.io API

### Why In-Memory Only?

No disk extraction:

**Rationale:**
1. **Speed**: No filesystem overhead
2. **Cleanup**: No temp files
3. **Security**: No file creation
4. **Simplicity**: Simpler API
5. **Use Case**: Inspection, not build

**Pattern:** Fit for purpose

### Why Optional Dependencies?

All deps are optional on enabled:

**Rationale:**
1. **Compilation**: Faster when disabled
2. **Size**: Smaller binary if unused
3. **Flexibility**: Use case dependent
4. **Testing**: Can test without network

**Pattern:** Feature-gated dependencies

## Testing Strategy

### Test Coverage

**test_tools Available:**
- Can use test_tools for testing
- Network tests need connectivity
- Local file tests with fixtures

### Test Focus

1. **Decode Valid**: Proper .crate files
2. **Decode Empty**: Empty archives
3. **Decode Invalid**: Malformed data
4. **List Files**: Correct paths returned
5. **Content Access**: Bytes match expected
6. **Download**: crates.io access (network)
7. **Local Read**: Filesystem reading
8. **Path Handling**: Various path formats
9. **Large Files**: Memory handling
10. **Edge Cases**: Unicode paths, special chars

### Known Test Limitations

1. **Network Required**: download tests need internet
2. **crates.io Rate Limits**: May fail under load
3. **Version Existence**: Test versions may be yanked
4. **Large Archives**: Memory constraints
5. **Platform Paths**: Windows vs Unix paths

## Future Considerations

### Potential Enhancements

1. **Async Download**: tokio/async-std support
2. **Streaming**: Process without full memory load
3. **Disk Extraction**: Optional file extraction
4. **Metadata Parsing**: Parse Cargo.toml
5. **Checksums**: Verify archive integrity
6. **Registry API**: Search and index queries
7. **Diff Generation**: Compare archives
8. **Size Limits**: Configurable memory limits
9. **Retry Logic**: Download retry on failure
10. **Caching**: Local cache for downloads

### Breaking Changes to Consider

1. **Storage Type**: Change internal representation
2. **Error Types**: Custom error type
3. **Async API**: Breaking sync API
4. **Path Type**: Use different path type
5. **Version Format**: Semver parsing

### Known Limitations

1. **Memory Usage**: Full archive in memory
2. **Blocking I/O**: No async support
3. **No Streaming**: Must download entire file
4. **No Verification**: Trusts source
5. **No Retry**: Single attempt downloads
6. **Path Assumptions**: Expects POSIX-style paths in archive
7. **Version Format**: Requires exact version string

## Adoption Guidelines

### When to Use crates_tools

**Good Candidates:**
- Inspecting published crates
- Pre-publish auditing
- Version comparison
- Size analysis
- Content verification
- Documentation extraction
- Automated crate inspection

**Poor Candidates:**
- Building crates (use cargo)
- Dependency resolution (use cargo)
- Crate publishing (use cargo publish)
- Metadata queries (use crates.io API)
- Large-scale analysis (memory limits)

### Basic Usage

```rust
use crates_tools::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  // Download from crates.io
  let archive = CrateArchive::download_crates_io("my_crate", "1.0.0")?;

  // List files
  for path in archive.list() {
    println!("{}", path.display());
  }

  // Read specific file
  if let Some(bytes) = archive.content_bytes("my_crate-1.0.0/src/lib.rs") {
    let content = std::str::from_utf8(bytes)?;
    println!("{}", content);
  }

  Ok(())
}
```

### Best Practices

1. **Handle Errors**: Network can fail
2. **Check Existence**: Files might not exist
3. **UTF-8 Safely**: Not all files are text
4. **Memory Aware**: Large crates use memory
5. **Timeout Handling**: Downloads can hang
6. **Version Format**: Use exact versions
7. **Path Matching**: Archive paths include version prefix

### Integration Example

```rust
use crates_tools::*;

fn verify_crate_contents(name: &str, version: &str) -> Result<bool, Box<dyn std::error::Error>> {
  let archive = CrateArchive::download_crates_io(name, version)?;

  // Check required files exist
  let required = ["Cargo.toml", "src/lib.rs"];

  for req in required {
    let found = archive.list().iter().any(|p| p.ends_with(req));
    if !found {
      eprintln!("Missing required file: {}", req);
      return Ok(false);
    }
  }

  // Check for suspicious files
  let suspicious = archive.list().iter().any(|p| {
    let s = p.to_string_lossy();
    s.contains(".env") || s.contains("secret")
  });

  if suspicious {
    eprintln!("Found suspicious files!");
    return Ok(false);
  }

  Ok(true)
}
```

## Related Crates

**Dependencies:**
- **flate2**: Gzip compression (external)
- **tar**: Tar archive handling (external)
- **ureq**: HTTP client (external)

**Related:**
- **workspace_tools**: Workspace management (workspace)
- **cargo_metadata**: Cargo metadata parsing (external)
- **crates_io_api**: crates.io API client (external)

**Alternatives:**
- **Manual tar + flate2**: More control, more code
- **download + extract**: File-based approach
- **cargo download**: Cargo subcommand

## References

- [API Documentation](https://docs.rs/crates_tools)
- [Repository](https://github.com/Wandalen/wTools/tree/master/module/core/crates_tools)
- [crates.io Download API](https://crates.io/policies)
- [flate2 Documentation](https://docs.rs/flate2)
- [tar Documentation](https://docs.rs/tar)
- [ureq Documentation](https://docs.rs/ureq)
- [readme.md](./readme.md)
