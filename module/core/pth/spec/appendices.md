## 10. Future Considerations

### 10.1 Potential Breaking Changes

**High Priority**:
1. **Rename `canonicalize()`** to `normalize_unchecked()` or similar
2. **Fix `.join()` to return Result** instead of panicking
3. **Consolidate CanonicalPath/NativePath** into single type
4. **Fix UTF-8 panics** - use `to_string_lossy()` everywhere

**Medium Priority**:
5. **Refactor iter_join()** - reduce complexity, use Path APIs
6. **Standardize separator handling** - document and enforce policy
7. **Add missing tests** for CanonicalPath, NativePath edge cases

### 10.2 Non-Breaking Improvements

1. **Better Documentation**
   - Clarify syntactic-only operations
   - More examples for each function
   - Platform-specific behavior docs

2. **Performance**
   - Reduce allocations in iter_join()
   - Use Path APIs instead of string manipulation
   - Benchmark and optimize hot paths

3. **API Expansion**
   - Iterator-based `exts()` instead of Vec
   - `path_common()` return PathBuf instead of String
   - Deprecate CurrentPath in favor of function

### 10.3 No Plans For

- ❌ URI parsing (outside scope of path utilities)
- ❌ Scheme-based architecture (not needed for paths)
- ❌ Filesystem operations (use std::fs)
- ❌ Async I/O (no I/O to make async)
- ❌ Network path support (UNC paths partially supported)

---

## Appendix A: Comparison with std::path

| Feature | pth crate | std::path |
|---------|-----------|-----------|
| Normalize | `normalize()` - syntactic | No direct equivalent |
| Canonicalize | `canonicalize()` - syntactic | `fs::canonicalize()` - filesystem |
| Absolute check | String-based (no `.` or `..` prefix) | `Path::is_absolute()` - platform-specific |
| Join | `iter_join()` with absolute reset | `Path::join()` - similar |
| Extensions | `ext()`, `exts()` - all extensions | `Path::extension()` - last only |
| Type safety | `AbsolutePath`, etc. newtypes | No type-level guarantees |
| Conversions | Rich trait system | Limited to `AsRef`/`Into` |
| UTF-8 | Assumes UTF-8, may panic | `OsStr`-based, no UTF-8 requirement |
| Platform | Cross-platform normalized (`/`) | Platform-native separators |

---

## Appendix B: Migration from Old Spec

The previous specification described a URI parsing framework with RFC 3986 compliance, scheme-based architecture, and extensible registry pattern. **None of that was implemented.**

This specification describes the **actual implementation**: a path utility library focused on syntactic path manipulation.

**If you need**:
- URI parsing → Use `url` crate
- HTTP URLs → Use `http` or `reqwest` crates
- Path utilities → Use this crate (`pth`)

---

## Appendix C: Versioning and Stability

**Current Version**: 0.28.0 (pre-1.0)

**Stability**:
- Public API is functional and tested (251 passing tests)
- Some known issues exist (see Section 7.1)
- Breaking changes planned for 0.29.0 (see Section 8.1)

**Semver Policy**:
- Breaking changes will bump minor version (0.x.0) until 1.0
- Deprecations will be introduced with warnings before removal
- At least one release cycle for migration

**Roadmap to 1.0**:
1. Fix public API panics (0.29.0)
2. Rename misleading functions (0.30.0)
3. Consolidate duplicate types (0.30.0)
4. Stabilize API, comprehensive docs (1.0.0)

---

## Addendum: Conformance Checklist

This checklist provides a flat list of all Functional Requirements for tracking implementation compliance and testing coverage. Each requirement should be verified by corresponding test cases.

### Path Normalization (FR-N)
- ❌ **FR-N001**: Remove Current Directory Components - `normalize()` removes all `.` components
- ❌ **FR-N002**: Resolve Parent Directory Components - `normalize()` resolves `..` by removing preceding component
- ❌ **FR-N003**: Preserve Leading Parent Components - Leading `..` in relative paths preserved when no preceding component
- ❌ **FR-N004**: Convert Empty Path to Current Directory - Empty path normalizes to `.`
- ❌ **FR-N005**: Prevent Parent Traversal Above Root - `..` components in absolute paths don't traverse above root

### Type Construction and Validation (FR-T)
- ❌ **FR-T001**: AbsolutePath Rejects Relative Paths - Construction rejects paths starting with `.` or `..` after normalization
- ❌ **FR-T002**: AbsolutePath Normalizes on Construction - Apply syntactic normalization during construction
- ❌ **FR-T003**: CanonicalPath Normalizes on Construction - Apply syntactic normalization during construction
- ❌ **FR-T004**: CurrentPath Resolves to Working Directory - Converting CurrentPath calls `std::env::current_dir()`

### Cross-Platform Separator Handling (FR-S)
- ❌ **FR-S001**: Accept Both Separators on Input - Functions accept both `/` and `\` as separators
- ❌ **FR-S002**: Output Forward Slash Separator - Normalization outputs paths using `/` separator on all platforms
- ❌ **FR-S003**: Preserve Windows Drive Letters - Windows drive letters preserved during normalization
- ❌ **FR-S004**: Strip Windows Verbatim Prefix - `canonicalize()` strips `\\?\` verbatim prefix

### Extension Manipulation (FR-E)
- ❌ **FR-E001**: Extract Last Extension - `ext()` returns last extension
- ❌ **FR-E002**: Extract All Extensions - `exts()` returns all extensions in order
- ❌ **FR-E003**: Handle No Extension - Extension functions handle paths without extensions gracefully
- ❌ **FR-E004**: Remove Last Extension - `without_ext()` removes only last extension
- ❌ **FR-E005**: Replace Last Extension - `change_ext()` replaces only last extension

### Path Joining (FR-J)
- ❌ **FR-J001**: Join Path Components - `iter_join()` joins components with `/` separator
- ❌ **FR-J002**: Absolute Path Resets Accumulation - Absolute path in `iter_join()` discards previous components
- ❌ **FR-J003**: Normalize During Joining - `iter_join()` resolves `.` and `..` during joining

### Relative Path Computation (FR-R)
- ❌ **FR-R001**: Compute Relative Path Between Paths - `path_relative()` computes relative path correctly
- ❌ **FR-R002**: Handle Same Directory - Identical paths return `.` in `path_relative()`
- ❌ **FR-R003**: Find Common Path Prefix - `path_common()` finds longest common prefix
- ❌ **FR-R004**: Rebase Path to New Base - `rebase()` moves file path from old base to new base

### Conversion Traits (FR-C)
- ❌ **FR-C001**: AsPath Borrows Without Allocation - `AsPath` provides borrowed `&Path` without allocation
- ❌ **FR-C002**: TryIntoPath Converts to Owned PathBuf - `TryIntoPath` converts various types to owned `PathBuf`
- ❌ **FR-C003**: TryIntoCowPath Avoids Unnecessary Cloning - `TryIntoCowPath` returns borrowed Cow for borrowed inputs
- ❌ **FR-C004**: Support Transitive Conversion - `TransitiveTryFrom` enables two-step conversion through intermediate types

### Glob Pattern Detection (FR-G)
- ❌ **FR-G001**: Detect Unescaped Glob Characters - `is_glob()` detects unescaped metacharacters
- ❌ **FR-G002**: Ignore Escaped Glob Characters - `is_glob()` treats backslash-escaped metacharacters as literal

### Error Handling (FR-ERR)
- ❌ **FR-ERR001**: Return Error for Invalid AbsolutePath - Constructing AbsolutePath from relative path returns error
- ❌ **FR-ERR002**: Return Error for Current Dir Unavailable - Converting CurrentPath when CWD unavailable returns error
- ❌ **FR-ERR003**: No Panics on Valid UTF-8 Paths - Public API functions don't panic on valid UTF-8 paths

### No-Filesystem Requirements (FR-NFS)
- ❌ **FR-NFS001**: No Filesystem Access for Normalization - `normalize()` doesn't access filesystem
- ❌ **FR-NFS002**: No Symlink Resolution - `canonicalize()` doesn't resolve symbolic links
- ❌ **FR-NFS003**: No Path Existence Verification - Path type construction doesn't verify filesystem existence

**Total Requirements**: 33
**Status Legend**: ❌ Not Verified | ✅ Verified | ⚠️ Partially Verified

---

**End of Specification**
