## 8. Usage Examples

### 8.1 Basic Path Normalization

```rust
use pth::path;

// Remove . and .. components
let clean = path::normalize("/a/./b/../c");
assert_eq!(clean, PathBuf::from("/a/c"));

// Handle relative paths
let clean = path::normalize("../../foo/bar");
assert_eq!(clean, PathBuf::from("../../foo/bar"));

// Empty path becomes .
let clean = path::normalize("");
assert_eq!(clean, PathBuf::from("."));
```

### 8.2 Type-Safe Absolute Paths

```rust
use pth::AbsolutePath;

// Create absolute path
let abs = AbsolutePath::try_from("/home/user/project")?;

// Type system enforces absoluteness
fn requires_absolute(path: AbsolutePath) {
    // Guaranteed not to start with . or ..
    println!("Processing: {}", path.display());
}

requires_absolute(abs);

// Joining preserves type
let subpath = abs.join("src/main.rs");
// subpath is still AbsolutePath
```

### 8.3 Generic Path Functions

```rust
use pth::{AsPath, TryIntoPath};

// Accept various path types
fn process_path<P: TryIntoPath>(path: P) -> io::Result<()> {
    let path = path.try_into_path()?;
    println!("Processing: {}", path.display());
    Ok(())
}

// Works with all of these:
process_path("relative/path")?;
process_path(PathBuf::from("/absolute/path"))?;
process_path(&some_path_variable)?;
process_path(CurrentPath)?; // Resolves to current_dir()
```

### 8.4 Extension Manipulation

```rust
use pth::path;

// Get all extensions
let exts = path::exts("archive.tar.gz");
assert_eq!(exts, vec!["tar".to_string(), "gz".to_string()]);

// Get single extension
let ext = path::ext("file.txt");
assert_eq!(ext, "txt");

// Remove extension
let without = path::without_ext("file.txt").unwrap();
assert_eq!(without, PathBuf::from("file"));

// Change extension
let changed = path::change_ext("file.txt", "json").unwrap();
assert_eq!(changed, PathBuf::from("file.json"));
```

### 8.5 Path Joining with Absolute Reset

```rust
use pth::path;

// Regular joining
let result = path::iter_join(vec!["base", "sub", "file.txt"].into_iter());
assert_eq!(result, PathBuf::from("base/sub/file.txt"));

// Absolute path resets
let result = path::iter_join(vec!["base", "/absolute", "file.txt"].into_iter());
assert_eq!(result, PathBuf::from("/absolute/file.txt"));
// "base" was discarded when "/absolute" was encountered
```

### 8.6 Relative Path Computation

```rust
use pth::path;

// Compute relative path
let rel = path::path_relative("/home/user/project", "/home/user/docs/file.txt");
assert_eq!(rel, PathBuf::from("../../docs/file.txt"));

// Same directory
let rel = path::path_relative("/a/b", "/a/b");
assert_eq!(rel, PathBuf::from("."));

// Child directory
let rel = path::path_relative("/a/b", "/a/b/c/d");
assert_eq!(rel, PathBuf::from("c/d"));
```

### 8.7 Path Rebasing

```rust
use pth::path;

// Move file from old base to new base
let rebased = path::rebase(
    "/project/old/src/main.rs",
    "/project/new",
    Some("/project/old")
)?;
assert_eq!(rebased, PathBuf::from("/project/new/src/main.rs"));

// Without old_path, uses common prefix
let rebased = path::rebase(
    "/project/src/main.rs",
    "/new/location",
    None
)?;
```

---

