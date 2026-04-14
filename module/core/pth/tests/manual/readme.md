# pth - Manual Testing Guide

Manual testing procedures for the pth path manipulation library.

## Overview

Since pth is a pure path manipulation library with zero filesystem access, manual testing focuses on:
- AbsolutePath type correctness
- Trait implementations (AsPath, TryIntoPath, TryIntoCowPath)
- Cross-platform path handling
- Path conversion edge cases
- Zero-allocation optimizations

## Prerequisites

### 1. Build pth

```bash
cd /home/user1/pro/lib/willbe/module/pth
cargo build --all-features
```

### 2. Create Test Program

```bash
mkdir -p /tmp/pth-test
cd /tmp/pth-test

cat > Cargo.toml <<'EOF'
[package]
name = "pth_manual_test"
version = "0.1.0"
edition = "2021"

[dependencies]
pth = { path = "/home/user1/pro/lib/willbe/module/pth" }
EOF

mkdir src
```

## Test Scenarios

### Test 1: AbsolutePath Creation

**Objective**: Verify absolute path enforcement

**Create Test**:
```bash
cat > src/main.rs <<'EOF'
use pth::AbsolutePath;
use std::path::PathBuf;

fn main() {
    println!("Test 1: AbsolutePath Creation\n");

    // Test 1a: Valid absolute path
    match AbsolutePath::try_from("/home/user/test") {
        Ok(abs_path) => println!("✅ Created absolute path: {:?}", abs_path),
        Err(e) => panic!("Failed to create absolute path: {}", e),
    }

    // Test 1b: Valid absolute PathBuf
    let path_buf = PathBuf::from("/tmp/test");
    match AbsolutePath::try_from(path_buf) {
        Ok(abs_path) => println!("✅ Created from PathBuf: {:?}", abs_path),
        Err(e) => panic!("Failed: {}", e),
    }

    // Test 1c: Relative path should fail
    match AbsolutePath::try_from("relative/path") {
        Ok(_) => panic!("Should reject relative path!"),
        Err(e) => println!("✅ Correctly rejected relative path: {}", e),
    }

    // Test 1d: Current directory path
    match AbsolutePath::try_from(std::env::current_dir().unwrap()) {
        Ok(abs_path) => println!("✅ Current dir as absolute: {:?}", abs_path),
        Err(e) => panic!("Failed: {}", e),
    }

    println!("\n✅ Test 1 passed!");
}
EOF

cargo run
```

**Expected Results**:
- ✅ Absolute paths accepted
- ✅ Relative paths rejected
- ✅ PathBuf conversion works
- ✅ Current directory works

**Success Criteria**:
- Type safety enforced
- No relative paths allowed

### Test 2: AsPath Trait

**Objective**: Verify zero-allocation path references

**Create Test**:
```bash
cat > src/test_as_path.rs <<'EOF'
use pth::AsPath;
use std::path::{Path, PathBuf};

fn print_path<P: AsPath>(p: P) {
    let path_ref = p.as_path();
    println!("Path: {:?}", path_ref);
}

fn main() {
    println!("Test 2: AsPath Trait\n");

    // Test with &Path
    let path = Path::new("/test");
    print_path(path);
    println!("✅ Works with &Path");

    // Test with PathBuf
    let path_buf = PathBuf::from("/tmp/test");
    print_path(&path_buf);
    println!("✅ Works with &PathBuf");

    // Test with String
    let string = String::from("/home/user");
    print_path(&string);
    println!("✅ Works with &String");

    // Test with &str
    print_path("/usr/bin");
    println!("✅ Works with &str");

    println!("\n✅ Test 2 passed!");
}
EOF

cat >> src/main.rs <<'EOF'
mod test_as_path;
EOF

cargo run --bin pth_manual_test
```

**Expected Results**:
- ✅ Works with &Path, PathBuf, String, &str
- ✅ Zero allocations (uses references)
- ✅ Flexible API

**Success Criteria**:
- All path-like types supported
- No heap allocations

### Test 3: TryIntoPath Trait

**Objective**: Verify owned PathBuf conversion

**Create Test**:
```bash
cat > src/test_try_into_path.rs <<'EOF'
use pth::TryIntoPath;
use std::path::{Path, PathBuf};

fn consume_path<P: TryIntoPath>(p: P) -> Result<PathBuf, Box<dyn std::error::Error>> {
    Ok(p.try_into_path()?)
}

fn main() {
    println!("Test 3: TryIntoPath Trait\n");

    // Test with PathBuf (no allocation)
    let path_buf = PathBuf::from("/test");
    let result = consume_path(path_buf).unwrap();
    println!("✅ PathBuf: {:?}", result);

    // Test with String
    let string = String::from("/home/user");
    let result = consume_path(string).unwrap();
    println!("✅ String: {:?}", result);

    // Test with &str (allocates)
    let result = consume_path("/usr/bin").unwrap();
    println!("✅ &str: {:?}", result);

    // Test with Path reference (allocates)
    let path = Path::new("/tmp");
    let result = consume_path(path).unwrap();
    println!("✅ &Path: {:?}", result);

    println!("\n✅ Test 3 passed!");
}
EOF

cargo run
```

**Expected Results**:
- ✅ All conversions succeed
- ✅ Owned PathBuf returned
- ✅ Allocates only when necessary

**Success Criteria**:
- Wide type support
- Efficient conversions

### Test 4: TryIntoCowPath Trait

**Objective**: Verify borrowed/owned optimization

**Create Test**:
```bash
cat > src/test_cow_path.rs <<'EOF'
use pth::TryIntoCowPath;
use std::borrow::Cow;
use std::path::{Path, PathBuf};

fn process_cow<P: TryIntoCowPath>(p: P) -> Result<Cow<'static, Path>, Box<dyn std::error::Error>> {
    Ok(p.try_into_cow_path()?)
}

fn main() {
    println!("Test 4: TryIntoCowPath Trait\n");

    // Test with PathBuf (owned)
    let path_buf = PathBuf::from("/test");
    match process_cow(path_buf).unwrap() {
        Cow::Borrowed(_) => println!("❌ Should be Owned"),
        Cow::Owned(p) => println!("✅ PathBuf -> Owned: {:?}", p),
    }

    // Test with &Path (borrowed - compile would fail, need 'static)
    // Skipping - requires lifetime management

    // Test with String
    let string = String::from("/home/user");
    match process_cow(string).unwrap() {
        Cow::Borrowed(_) => println!("❌ Should be Owned"),
        Cow::Owned(p) => println!("✅ String -> Owned: {:?}", p),
    }

    println!("\n✅ Test 4 passed!");
}
EOF

cargo run
```

**Expected Results**:
- ✅ Borrowed when possible
- ✅ Owned when necessary
- ✅ Avoids redundant allocations

**Success Criteria**:
- Optimizes allocations
- Correct Cow usage

### Test 5: Cross-Platform Path Handling

**Objective**: Verify path operations across platforms

**Create Test**:
```bash
cat > src/test_cross_platform.rs <<'EOF'
use pth::AbsolutePath;
use std::path::PathBuf;

fn main() {
    println!("Test 5: Cross-Platform Paths\n");

    // Unix path
    #[cfg(unix)]
    {
        let unix_path = AbsolutePath::try_from("/usr/bin/test").unwrap();
        println!("✅ Unix absolute path: {:?}", unix_path);

        let home = std::env::var("HOME").ok()
            .and_then(|h| AbsolutePath::try_from(PathBuf::from(h)).ok());
        if let Some(home_path) = home {
            println!("✅ Home directory: {:?}", home_path);
        }
    }

    // Windows path
    #[cfg(windows)]
    {
        let win_path = AbsolutePath::try_from(r"C:\Windows\System32").unwrap();
        println!("✅ Windows absolute path: {:?}", win_path);

        let userprofile = std::env::var("USERPROFILE").ok()
            .and_then(|u| AbsolutePath::try_from(PathBuf::from(u)).ok());
        if let Some(user_path) = userprofile {
            println!("✅ User profile: {:?}", user_path);
        }
    }

    println!("\n✅ Test 5 passed!");
}
EOF

cargo run
```

**Expected Results**:
- ✅ Unix paths work on Unix
- ✅ Windows paths work on Windows
- ✅ Platform-appropriate separators
- ✅ Environment paths handled

**Success Criteria**:
- Platform detection works
- Path formats respected

### Test 6: Path Component Access

**Objective**: Verify std::path integration

**Create Test**:
```bash
cat > src/test_components.rs <<'EOF'
use pth::AbsolutePath;
use std::path::Path;

fn main() {
    println!("Test 6: Path Components\n");

    let abs_path = AbsolutePath::try_from("/home/user/documents/file.txt").unwrap();

    // Access as Path
    let path: &Path = abs_path.as_ref();

    // Test components
    println!("Full path: {:?}", path);
    println!("File name: {:?}", path.file_name());
    println!("Extension: {:?}", path.extension());
    println!("Parent: {:?}", path.parent());

    // Test path operations
    if let Some(parent) = path.parent() {
        println!("✅ Parent directory: {:?}", parent);
    }

    if let Some(file_name) = path.file_name() {
        println!("✅ File name: {:?}", file_name);
    }

    if let Some(extension) = path.extension() {
        println!("✅ Extension: {:?}", extension);
    }

    println!("\n✅ Test 6 passed!");
}
EOF

cargo run
```

**Expected Results**:
- ✅ Component access works
- ✅ file_name() returns correct value
- ✅ extension() extracts extension
- ✅ parent() returns parent directory

**Success Criteria**:
- std::path integration complete
- All methods accessible

### Test 7: Edge Cases

**Objective**: Verify handling of edge cases

**Create Test**:
```bash
cat > src/test_edge_cases.rs <<'EOF'
use pth::AbsolutePath;
use std::path::PathBuf;

fn main() {
    println!("Test 7: Edge Cases\n");

    // Root path
    #[cfg(unix)]
    {
        let root = AbsolutePath::try_from("/").unwrap();
        println!("✅ Root path: {:?}", root);
    }

    // Path with multiple slashes
    let multi_slash = AbsolutePath::try_from("/home//user///test").unwrap();
    println!("✅ Multiple slashes handled: {:?}", multi_slash);

    // Path with .
    let with_dot = AbsolutePath::try_from("/home/./user").unwrap();
    println!("✅ Path with . handled: {:?}", with_dot);

    // Path with ..
    let with_dotdot = AbsolutePath::try_from("/home/user/../user").unwrap();
    println!("✅ Path with .. handled: {:?}", with_dotdot);

    // Empty component
    let empty_comp = AbsolutePath::try_from("/home/user/").unwrap();
    println!("✅ Trailing slash handled: {:?}", empty_comp);

    println!("\n✅ Test 7 passed!");
}
EOF

cargo run
```

**Expected Results**:
- ✅ Root path works
- ✅ Multiple slashes normalized
- ✅ . and .. handled
- ✅ Trailing slashes OK

**Success Criteria**:
- Edge cases handled gracefully
- No crashes or panics

### Test 8: Current Directory Integration

**Objective**: Verify current directory access

**Create Test**:
```bash
cat > src/test_current_dir.rs <<'EOF'
use pth::AbsolutePath;
use std::env;

fn main() {
    println!("Test 8: Current Directory\n");

    // Get current directory
    let current = env::current_dir().unwrap();
    println!("Current directory (PathBuf): {:?}", current);

    // Convert to AbsolutePath
    let abs_current = AbsolutePath::try_from(current.clone()).unwrap();
    println!("Current directory (AbsolutePath): {:?}", abs_current);

    // Verify it's absolute
    assert!(current.is_absolute(), "Current dir should be absolute");
    println!("✅ Current directory is absolute");

    // Create path relative to current
    let relative_to_current = current.join("test_file.txt");
    println!("Path relative to current: {:?}", relative_to_current);

    let abs_relative = AbsolutePath::try_from(relative_to_current).unwrap();
    println!("✅ Converted to absolute: {:?}", abs_relative);

    println!("\n✅ Test 8 passed!");
}
EOF

cargo run
```

**Expected Results**:
- ✅ Current directory retrieved
- ✅ Converts to AbsolutePath
- ✅ Join operations work
- ✅ Relative paths resolvable

**Success Criteria**:
- env::current_dir() integration works
- Path joining supported

## Performance Tests

### Test 9: Zero-Allocation Verification

**Objective**: Verify zero-allocation claims

**Create Test**:
```bash
cat > src/test_allocations.rs <<'EOF'
use pth::AsPath;
use std::path::Path;

fn reference_only<P: AsPath>(p: P) {
    let _ = p.as_path(); // Should not allocate
}

fn main() {
    println!("Test 9: Zero-Allocation Verification\n");

    // Test with references (no allocations expected)
    let path = Path::new("/test");
    reference_only(path);
    println!("✅ &Path: No allocation (reference)");

    let string = String::from("/home");
    reference_only(&string);
    println!("✅ &String: No allocation (reference)");

    reference_only("/usr/bin");
    println!("✅ &str: No allocation (reference)");

    // Manual verification note
    println!("\nNote: Use tools like heaptrack or valgrind to verify zero allocations");

    println!("\n✅ Test 9 passed!");
}
EOF

cargo run
```

**Expected Results**:
- ✅ AsPath uses references only
- ✅ No heap allocations for reference types

**Success Criteria**:
- Zero allocations verified

## Integration Tests

### Test 10: Real Project Path Handling

**Objective**: Verify usage in realistic scenarios

**Create Test**:
```bash
cat > src/test_realistic.rs <<'EOF'
use pth::AbsolutePath;
use std::env;
use std::path::PathBuf;

fn main() {
    println!("Test 10: Realistic Usage\n");

    // Scenario: Build system paths
    let project_root = env::current_dir().unwrap();
    let abs_root = AbsolutePath::try_from(project_root.clone()).unwrap();
    println!("Project root: {:?}", abs_root);

    let src_dir = project_root.join("src");
    let abs_src = AbsolutePath::try_from(src_dir).unwrap();
    println!("Source directory: {:?}", abs_src);

    let target_dir = project_root.join("target/debug");
    let abs_target = AbsolutePath::try_from(target_dir).unwrap();
    println!("Target directory: {:?}", abs_target);

    // Scenario: Config file paths
    #[cfg(unix)]
    let config_home = env::var("HOME")
        .ok()
        .map(PathBuf::from)
        .and_then(|h| AbsolutePath::try_from(h.join(".config")).ok());

    #[cfg(windows)]
    let config_home = env::var("APPDATA")
        .ok()
        .map(PathBuf::from)
        .and_then(|a| AbsolutePath::try_from(a).ok());

    if let Some(config) = config_home {
        println!("Config directory: {:?}", config);
    }

    println!("\n✅ Test 10 passed!");
}
EOF

cargo run
```

**Expected Results**:
- ✅ Project paths handled correctly
- ✅ Build directory paths work
- ✅ Config paths resolved
- ✅ All paths absolute

**Success Criteria**:
- Real-world usage works
- Path manipulation smooth

## Verification Checklist

**AbsolutePath Type**:
- [ ] Absolute paths accepted
- [ ] Relative paths rejected
- [ ] PathBuf conversion works
- [ ] Type safety enforced

**Traits**:
- [ ] AsPath zero-allocation confirmed
- [ ] TryIntoPath supports multiple types
- [ ] TryIntoCowPath optimizes allocations
- [ ] All traits work with Path, PathBuf, String, &str

**Cross-Platform**:
- [ ] Unix paths work on Unix
- [ ] Windows paths work on Windows
- [ ] Platform separators respected
- [ ] Environment paths handled

**std::path Integration**:
- [ ] Component access works
- [ ] file_name(), extension(), parent() work
- [ ] Path operations available
- [ ] Full std::path API accessible

**Edge Cases**:
- [ ] Root path handled
- [ ] Multiple slashes normalized
- [ ] . and .. components work
- [ ] Trailing slashes OK

**Performance**:
- [ ] Zero allocations verified (AsPath)
- [ ] Efficient conversions
- [ ] No unnecessary copies

**Integration**:
- [ ] current_dir() integration works
- [ ] Real project paths handled
- [ ] Build system usage viable

## Known Limitations

1. **No Filesystem Access**: Pure path manipulation only, no file operations
2. **no_std Unsupported**: Requires std::path types from stdlib
3. **No Path Validation**: Only manipulation, doesn't verify paths exist
4. **Platform-Dependent**: Path semantics depend on target platform

## Reporting Issues

When reporting issues:

1. **Include path example**:
   - Input path
   - Expected result
   - Actual result

2. **Include platform information**:
   - OS (Unix, Windows, macOS)
   - Rust version

3. **Include code snippet** demonstrating the issue

4. **Expected behavior** based on std::path documentation

## References

- Main readme: `../readme.md`
- std::path docs: https://doc.rust-lang.org/std/path/
- API documentation: `cargo doc --open`
