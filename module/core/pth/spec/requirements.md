## 4. Functional Requirements

This section defines explicit, testable requirements that the implementation must satisfy.

### 4.1 Path Normalization Requirements

**FR-N001: Remove Current Directory Components**
- **Requirement**: The `normalize()` function must remove all `.` (current directory) components from paths
- **Pass Criteria**: Given path `/a/./b/./c`, output must equal `/a/b/c`

**FR-N002: Resolve Parent Directory Components**
- **Requirement**: The `normalize()` function must resolve `..` (parent directory) by removing the preceding normal component
- **Pass Criteria**: Given path `/a/b/../c`, output must equal `/a/c`

**FR-N003: Preserve Leading Parent Components in Relative Paths**
- **Requirement**: Leading `..` components in relative paths must be preserved when no preceding component exists
- **Pass Criteria**: Given path `../../a/b`, output must equal `../../a/b`

**FR-N004: Convert Empty Path to Current Directory**
- **Requirement**: Empty input path must normalize to `.` (current directory)
- **Pass Criteria**: Given path `""`, output must equal `.`

**FR-N005: Prevent Parent Traversal Above Root**
- **Requirement**: In absolute paths, `..` components must not traverse above the root
- **Pass Criteria**: Given path `/../../a`, output must equal `/../../a` (preserves the impossible traversal)

### 4.2 Type Construction and Validation Requirements

**FR-T001: AbsolutePath Rejects Relative Paths**
- **Requirement**: `AbsolutePath::try_from()` must reject paths starting with `.` or `..` after normalization
- **Pass Criteria**: Given path `./foo` or `../bar`, construction must return error

**FR-T002: AbsolutePath Normalizes on Construction**
- **Requirement**: `AbsolutePath::try_from()` must apply syntactic normalization during construction
- **Pass Criteria**: Constructing from `/a/b/../c` must result in AbsolutePath containing `/a/c`

**FR-T003: CanonicalPath Normalizes on Construction**
- **Requirement**: `CanonicalPath::try_from()` must apply syntactic normalization during construction
- **Pass Criteria**: Constructing from `/a/./b/../c` must result in CanonicalPath containing `/a/c`

**FR-T004: CurrentPath Resolves to Working Directory**
- **Requirement**: Converting `CurrentPath` to `PathBuf` must call `std::env::current_dir()`
- **Pass Criteria**: `PathBuf::try_from(CurrentPath)` must return current working directory or error if unavailable

### 4.3 Cross-Platform Separator Handling Requirements

**FR-S001: Accept Both Separators on Input**
- **Requirement**: All path functions must accept both `/` (Unix) and `\` (Windows) as separators on input
- **Pass Criteria**: Given path `a\b\c` on any platform, functions must process it identically to `a/b/c`

**FR-S002: Output Forward Slash Separator**
- **Requirement**: All path normalization functions must output paths using `/` separator regardless of platform
- **Pass Criteria**: On Windows, given path `C:\Users\foo`, output must equal `C:/Users/foo`

**FR-S003: Preserve Windows Drive Letters**
- **Requirement**: Windows drive letters (e.g., `C:`) must be preserved during normalization
- **Pass Criteria**: Given path `C:\foo\..\bar`, output must equal `C:/bar` (not `/bar`)

**FR-S004: Strip Windows Verbatim Prefix**
- **Requirement**: The `canonicalize()` function must strip `\\?\` verbatim prefix from Windows paths
- **Pass Criteria**: Given path `\\?\C:\foo`, output must equal `C:/foo`

### 4.4 Extension Manipulation Requirements

**FR-E001: Extract Last Extension**
- **Requirement**: The `ext()` function must return the last extension of a path
- **Pass Criteria**: Given path `file.tar.gz`, output must equal `"gz"`

**FR-E002: Extract All Extensions**
- **Requirement**: The `exts()` function must return all extensions in order
- **Pass Criteria**: Given path `file.tar.gz`, output must equal `vec!["tar", "gz"]`

**FR-E003: Handle No Extension**
- **Requirement**: Extension functions must handle paths without extensions gracefully
- **Pass Criteria**: Given path `file`, `ext()` returns `""` and `exts()` returns empty vector

**FR-E004: Remove Last Extension**
- **Requirement**: The `without_ext()` function must remove only the last extension
- **Pass Criteria**: Given path `file.tar.gz`, output must equal `Some("file.tar")`

**FR-E005: Replace Last Extension**
- **Requirement**: The `change_ext()` function must replace only the last extension
- **Pass Criteria**: Given path `file.tar.gz` and new extension `"zip"`, output must equal `Some("file.tar.zip")`

### 4.5 Path Joining Requirements

**FR-J001: Join Path Components**
- **Requirement**: The `iter_join()` function must join multiple path components with `/` separator
- **Pass Criteria**: Given paths `["a", "b", "c"]`, output must equal `"a/b/c"`

**FR-J002: Absolute Path Resets Accumulation**
- **Requirement**: When `iter_join()` encounters an absolute path (starts with `/`), it must discard all previously accumulated components
- **Pass Criteria**: Given paths `["base", "/absolute", "file"]`, output must equal `"/absolute/file"`

**FR-J003: Normalize During Joining**
- **Requirement**: The `iter_join()` function must resolve `.` and `..` components during joining
- **Pass Criteria**: Given paths `["a", "b", "..", "c"]`, output must equal `"a/c"`

### 4.6 Relative Path Computation Requirements

**FR-R001: Compute Relative Path Between Paths**
- **Requirement**: The `path_relative()` function must compute the relative path from one path to another
- **Pass Criteria**: Given from=`/a/b/c` and to=`/a/d/e`, output must equal `../../d/e`

**FR-R002: Handle Same Directory**
- **Requirement**: When from and to paths are identical, `path_relative()` must return `.`
- **Pass Criteria**: Given from=`/a/b` and to=`/a/b`, output must equal `.`

**FR-R003: Find Common Path Prefix**
- **Requirement**: The `path_common()` function must find the longest common prefix of multiple paths
- **Pass Criteria**: Given paths `["/a/b/c", "/a/b/d"]`, output must equal `Some("/a/b")`

**FR-R004: Rebase Path to New Base**
- **Requirement**: The `rebase()` function must move a file path from one base directory to another
- **Pass Criteria**: Given file_path=`/old/base/sub/file`, new_path=`/new/base`, old_path=`Some(/old/base)`, output must equal `/new/base/sub/file`

### 4.7 Conversion Trait Requirements

**FR-C001: AsPath Borrows Without Allocation**
- **Requirement**: The `AsPath` trait must provide borrowed `&Path` without allocation
- **Pass Criteria**: Given `&str` or `&Path`, `as_path()` must return borrowed reference with no heap allocation

**FR-C002: TryIntoPath Converts to Owned PathBuf**
- **Requirement**: The `TryIntoPath` trait must convert various types to owned `PathBuf`
- **Pass Criteria**: All standard types (`&str`, `String`, `&Path`, `PathBuf`, path types) must successfully convert to `PathBuf`

**FR-C003: TryIntoCowPath Avoids Unnecessary Cloning**
- **Requirement**: The `TryIntoCowPath` trait must return borrowed `Cow::Borrowed` for borrowed inputs
- **Pass Criteria**: Given `&Path`, `try_into_cow_path()` must return `Cow::Borrowed` with no cloning

**FR-C004: Support Transitive Conversion**
- **Requirement**: The `TransitiveTryFrom` trait must enable two-step conversion through intermediate types
- **Pass Criteria**: Converting type A to type C through intermediate type B must succeed when A→B and B→C conversions exist

### 4.8 Glob Pattern Detection Requirements

**FR-G001: Detect Unescaped Glob Characters**
- **Requirement**: The `is_glob()` function must detect unescaped glob metacharacters (`*`, `?`, `[...]`, `{...}`)
- **Pass Criteria**: Given path `*.txt`, output must be `true`; given `file.txt`, output must be `false`

**FR-G002: Ignore Escaped Glob Characters**
- **Requirement**: The `is_glob()` function must treat backslash-escaped metacharacters as literal
- **Pass Criteria**: Given path `\*.txt`, output must be `false` (escaped asterisk)

### 4.9 Error Handling Requirements

**FR-ERR001: Return Error for Invalid AbsolutePath**
- **Requirement**: Constructing `AbsolutePath` from relative path must return `io::Error`
- **Pass Criteria**: `AbsolutePath::try_from("./relative")` must return `Err(io::Error)`

**FR-ERR002: Return Error for Current Dir Unavailable**
- **Requirement**: Converting `CurrentPath` when current directory is unavailable must return error
- **Pass Criteria**: `PathBuf::try_from(CurrentPath)` must return `Err` if `std::env::current_dir()` fails

**FR-ERR003: No Panics on Valid UTF-8 Paths**
- **Requirement**: All public API functions must not panic on valid UTF-8 path inputs
- **Pass Criteria**: All documented functions with valid UTF-8 paths must either succeed or return error, never panic

### 4.10 No-Filesystem Requirements

**FR-NFS001: No Filesystem Access for Normalization**
- **Requirement**: The `normalize()` function must not access the filesystem
- **Pass Criteria**: Function must succeed on non-existent paths; strace/process monitor must show no filesystem syscalls

**FR-NFS002: No Symlink Resolution**
- **Requirement**: The `canonicalize()` function must not resolve symbolic links
- **Pass Criteria**: Given path containing symlink, output must process symlink name as string, not resolve to target

**FR-NFS003: No Path Existence Verification**
- **Requirement**: Path type construction must not verify filesystem existence
- **Pass Criteria**: Constructing `AbsolutePath` from `/nonexistent/path` must succeed if path is syntactically absolute

### 4.11 Non-Functional Requirements

This section defines quantifiable performance, reliability, and quality attributes the implementation must satisfy.

**NFR-PERF001: Path Normalization Performance**
- **Requirement**: The `normalize()` function must complete in O(n) time complexity where n = path length
- **Metric**: Average execution time ≤ 1μs per path component on standard hardware (3GHz CPU)
- **Measurement**: Benchmark on paths 10-1000 characters with 1-50 components
- **Target**: 50,000 normalizations/second for typical paths (5-10 components, 50-100 characters)

**NFR-PERF002: Memory Efficiency**
- **Requirement**: Newtype wrappers must have zero runtime overhead
- **Metric**: `sizeof(AbsolutePath) == sizeof(PathBuf)` (24 bytes on 64-bit systems)
- **Measurement**: Static assertion at compile time via `mem::size_of`

**NFR-PERF003: Allocation Minimization**
- **Requirement**: Most operations should allocate at most one `PathBuf`
- **Metric**: ≤ 1 heap allocation per path operation (excluding input conversion)
- **Measurement**: Allocation profiling with jemalloc or similar
- **Exception**: `iter_join()` may allocate up to n PathBufs where n = number of components

**NFR-COMPAT001: Platform Support**
- **Requirement**: The library must support Linux, Windows, and macOS without platform-specific compilation
- **Metric**: All tests pass on all three platforms with identical behavior
- **Measurement**: CI pipeline running on ubuntu-latest, windows-latest, macos-latest
- **Target**: 100% test pass rate on all platforms

**NFR-COMPAT002: Rust Version Compatibility**
- **Requirement**: The library must support Rust stable channel from at least 6 months prior
- **Metric**: MSRV (Minimum Supported Rust Version) ≤ current stable minus 6 months
- **Measurement**: CI testing on specified MSRV

**NFR-COMPAT003: no_std Compatibility**
- **Requirement**: Core path operations should work in `no_std` environments with `alloc`
- **Metric**: Crate compiles and core functions work with `#![no_std]` + `extern crate alloc`
- **Limitation**: `CurrentPath` requires std (filesystem access)

**NFR-SAFE001: Thread Safety**
- **Requirement**: All types must be `Send + Sync`
- **Metric**: Compiler verification via trait bounds
- **Measurement**: Static assertion at compile time
- **Exception**: `unique_folder_name()` uses thread-local counter (still thread-safe, not Send)

**NFR-SAFE002: Panic Safety**
- **Requirement**: Public API functions should not panic on valid inputs (UTF-8 paths, valid types)
- **Metric**: Zero panic-causing code paths for documented valid inputs
- **Measurement**: Fuzzing with cargo-fuzz for 1M iterations
- **Known Exceptions**:
  - `.join()` methods may panic (planned fix in 0.29.0)
  - `to_str().unwrap()` in `rebase()` panics on non-UTF-8 (affects ~2 functions)

**NFR-TEST001: Test Coverage**
- **Requirement**: Maintain ≥95% line coverage for core path operations
- **Metric**: Code coverage percentage measured by tarpaulin or llvm-cov
- **Target**: ≥95% line coverage, ≥90% branch coverage
- **Current**: 228 integration tests + 18 doc tests (estimated ~92% coverage)

**NFR-TEST002: Cross-Platform Test Coverage**
- **Requirement**: Tests must verify behavior on all supported platforms
- **Metric**: Test suite includes platform-specific cases (Windows drives, Unix paths)
- **Measurement**: Manual review of test cases + CI execution on all platforms

**NFR-DOC001: API Documentation**
- **Requirement**: All public functions must have rustdoc documentation with examples
- **Metric**: cargo doc with `--document-private-items` warns on missing docs
- **Target**: Zero rustdoc warnings with `-D missing-docs`

**NFR-DEP001: Dependency Minimization**
- **Requirement**: Minimize required dependencies to reduce supply chain risk
- **Metric**: ≤ 5 required dependencies in dependency tree
- **Current**: 2 required deps (mod_interface, regex) + 2 optional (serde, camino)
- **Measurement**: `cargo tree --depth 1`

### 4.12 NFR Verification Methods

This section describes comprehensive testing and measurement strategies for verifying Non-Functional Requirements.

**Performance Verification**:

1. **Benchmarking Setup**:
   - Tool: `cargo bench` with criterion.rs
   - Hardware: CI runners (GitHub Actions standard runners: 2 CPU, 7GB RAM)
   - Baseline: Establish baseline on first PR, track regressions

2. **Performance Tests**:
   ```bash
   # Benchmark path normalization
   cargo bench --bench path_normalize -- --warm-up-time 2 --measurement-time 10

   # Memory profiling
   cargo build --release
   valgrind --tool=massif --massif-out-file=massif.out ./target/release/examples/profile
   ms_print massif.out | grep "peak memory"

   # Allocation counting
   RUSTFLAGS="-C link-arg=-fuse-ld=lld" cargo test --release -- --nocapture 2>&1 | grep alloc
   ```

3. **Acceptance Criteria**:
   - Performance must not regress by >10% between releases
   - New features must justify any performance impact with rationale

**Compatibility Verification**:

1. **Platform Testing Matrix**:
   ```yaml
   platforms:
     - ubuntu-latest  # Linux x86_64
     - windows-latest # Windows x86_64
     - macos-latest   # macOS x86_64/ARM64
   rust_versions:
     - stable
     - beta
     - MSRV (specified in Cargo.toml)
   ```

2. **Cross-Platform Test Execution**:
   ```bash
   # Run on all platforms via CI
   cargo test --all-features --all-targets

   # Platform-specific test filtering
   cargo test --test platform_tests -- --test-threads=1
   ```

3. **no_std Verification**:
   ```bash
   # Verify no_std compatibility
   cargo build --no-default-features --features alloc
   cargo test --no-default-features --features alloc
   ```

**Safety Verification**:

1. **Thread Safety Checks**:
   ```rust
   // Compile-time verification via trait bounds
   fn assert_send_sync<T: Send + Sync>() {}
   assert_send_sync::<AbsolutePath>();
   assert_send_sync::<CanonicalPath>();
   ```

2. **Panic Detection**:
   ```bash
   # Fuzzing setup
   cargo install cargo-fuzz
   cargo fuzz init

   # Run fuzzer for NFR-SAFE002 verification
   cargo fuzz run normalize_fuzz -- -runs=1000000 -max_len=4096
   cargo fuzz run absolute_path_fuzz -- -runs=1000000

   # Panic detection in tests
   cargo test --release 2>&1 | grep -i "panic\|unwrap" && exit 1 || exit 0
   ```

3. **Miri Undefined Behavior Check**:
   ```bash
   cargo +nightly miri test
   ```

**Test Coverage Verification**:

1. **Coverage Measurement**:
   ```bash
   # Using llvm-cov (recommended)
   cargo install cargo-llvm-cov
   cargo llvm-cov --all-features --html
   open target/llvm-cov/html/index.html

   # Using tarpaulin (alternative)
   cargo install cargo-tarpaulin
   cargo tarpaulin --all-features --out Html --output-dir coverage
   ```

2. **Coverage Gates**:
   - Minimum line coverage: 95%
   - Minimum branch coverage: 90%
   - CI fails if coverage drops below threshold

3. **Uncovered Code Review**:
   - Manual review of uncovered lines quarterly
   - Document rationale for any intentionally uncovered code

**Documentation Verification**:

1. **Rustdoc Completeness**:
   ```bash
   # Check for missing docs
   RUSTDOCFLAGS="-D missing-docs -D rustdoc::broken-intra-doc-links" cargo doc --all-features

   # Verify doc tests
   cargo test --doc --all-features
   ```

2. **Example Verification**:
   ```bash
   # Run all examples
   for example in examples/*.rs; do
     cargo run --example $(basename $example .rs) || exit 1
   done
   ```

**Dependency Audit**:

1. **Supply Chain Security**:
   ```bash
   # Check for known vulnerabilities
   cargo install cargo-audit
   cargo audit

   # Verify dependency count
   cargo tree --depth 1 | wc -l
   # Must be ≤ 5 required dependencies

   # Check for unused dependencies
   cargo +nightly udeps --all-targets
   ```

2. **License Compliance**:
   ```bash
   cargo install cargo-license
   cargo license --json | jq '.[] | select(.license != "MIT" and .license != "Apache-2.0")'
   ```

**Continuous Integration Gates**:

All NFR verifications must pass in CI before merge:
1. ✅ All tests pass on all platforms
2. ✅ Performance benchmarks show no regression >10%
3. ✅ Coverage ≥ 95% line, ≥ 90% branch
4. ✅ Rustdoc builds with no warnings
5. ✅ Clippy passes with no warnings
6. ✅ cargo audit reports no vulnerabilities
7. ✅ Fuzzing runs for 1M iterations with no panics
8. ✅ MSRV build succeeds

---

