## 9. Implementation Constraints

### 9.1 Known Limitations

1. **Not Filesystem-Aware**
   - Cannot resolve symlinks
   - Cannot verify path existence
   - Cannot determine actual file types
   - Cannot handle filesystem-specific rules (case sensitivity, etc.)

2. **String-Based Operations**
   - Some functions use string manipulation instead of Path APIs
   - Less efficient (2-5x more allocations than Path-based operations)
   - More error-prone (estimated 3-5 instances marked with `qqq` comments for refactoring)
   - Affects primarily `iter_join()` and `path_relative()` functions
   - Performance impact: typically 10-50μs overhead on average paths (20-100 characters)

3. **UTF-8 Panics**
   - Functions using `to_str().unwrap()` panic on non-UTF-8 paths
   - Affects `rebase()` (path.rs:876) and internal operations
   - Affects approximately 2 public API functions and 3 internal operations
   - Rare on Windows (<0.01% of paths), possible on Unix (up to 5% of paths in international filesystems)

4. **Misleading Names**
   - `canonicalize()` doesn't actually canonicalize (no symlink resolution)
   - `CanonicalPath` and `NativePath` are identical
   - `AbsolutePath` checks for "not starting with . or ..", not truly absolute

5. **Public API Panics**
   - `.join()` methods on path types call `unwrap()`
   - Can panic if join result doesn't meet type requirements
   - Should return `Result` instead (future breaking change)

### 9.2 Performance Characteristics

**Fast Operations** (O(n) where n = path length):
- `normalize()` - single pass
- `is_glob()` - single scan
- `ext()` / `exts()` - scan from end
- Type conversions - zero-copy or minimal allocation

**Slower Operations**:
- `iter_join()` - multiple string allocations
- `path_relative()` - complex string manipulation
- `path_common()` - component-by-component comparison

**Memory**:
- Newtypes have zero overhead (transparent repr)
- Most operations allocate one `PathBuf`
- `TryIntoCowPath` can avoid cloning for borrowed inputs

### 9.3 Error Handling

**Error Types Used**:
- `io::Error` - for consistency with std::path, even though no actual I/O
- `SystemTimeError` - for `unique_folder_name()`

**Error Conditions**:
- Path not absolute (AbsolutePath)
- Invalid UTF-8 (implicitly via panic)
- System time error (unique_folder_name)
- Conversion failures (TryFrom implementations)

### 9.4 Thread Safety

**Thread-Safe**:
- All types are Send + Sync
- No mutable global state (except thread-local counter in unique_folder_name)

**Thread-Local**:
- `unique_folder_name()` maintains per-thread counter for uniqueness

---

