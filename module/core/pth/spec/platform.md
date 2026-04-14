## 7. Platform Considerations

### 7.1 Path Separators

**Internal Representation**: Always `/` (forward slash)

**Input Handling**:
- Accepts both `\` (Windows) and `/` (Unix)
- Converts `\` to `/` during processing
- Output uses `/` separator

**Example**:
```rust
// Windows input
let norm = pth::path::normalize(r"C:\Users\foo\..\bar");
// norm = "C:/Users/bar"

// Unix input
let norm = pth::path::normalize("/home/foo/../bar");
// norm = "/home/bar"
```

### 7.2 Windows Drive Letters

**Special Handling**:
- Recognizes Windows drives: `C:`, `D:`, etc.
- Normalizes `C:\` to `C:/`
- Strips `\\?\` verbatim prefixes in `canonicalize()`

**Known Issue** (path.rs:925-926):
```rust
// WARNING: This strips ": " from anywhere in path, not just drive letters
from = from.replace(": ", "");
to = to.replace(": ", "");
```

This can corrupt paths containing legitimate `: ` sequences.

### 7.3 UTF-8 Assumptions

**Important**: Some functions use `to_str().unwrap()` which panics on non-UTF-8 paths.

**Affected Functions**:
- `rebase()` at line 876 (public API)
- Internal string operations

**Implication**: Paths with invalid UTF-8 sequences (possible on Unix) will cause panics.

**Recommendation**: Use `to_string_lossy()` instead (currently marked with `qqq` comments).

### 7.4 Cross-Platform Compatibility

**Tested Platforms**:
- Linux (primary development)
- Windows (special cases handled)
- macOS (standard Unix behavior)

**Test Coverage**: 228 integration tests + 18 doc tests covering cross-platform scenarios.

---

