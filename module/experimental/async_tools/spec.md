# Specification: async_tools

## Overview

**async_tools** is a facade crate aggregating asynchronous programming utilities, providing async versions of standard conversion traits (From/Into/TryFrom/TryInto) and re-exporting the async_trait macro. It enables type conversions that involve asynchronous operations such as network I/O, database queries, or other async contexts where standard synchronous traits cannot be used.

**Version:** 0.1.0
**Status:** Experimental
**Category:** Core Utilities (Async Programming)
**Dependents:** Unknown (likely workspace crates requiring async conversions)

### Scope

#### Responsibility

Provide a unified facade for asynchronous programming utilities, aggregating async conversion traits (AsyncFrom, AsyncInto, AsyncTryFrom, AsyncTryInto) from async_from crate and the async_trait attribute macro, enabling ergonomic async type conversions with familiar syntax mirroring standard library traits.

#### In-Scope

1. **Async Conversion Traits (AsyncFrom/AsyncInto)**
   - `AsyncFrom<T>` trait - Async version of From<T>
   - `async_from(value: T) -> Self` method
   - `AsyncInto<T>` trait - Async version of Into<T>
   - `async_into(self) -> T` method
   - Blanket impl: AsyncFrom implies AsyncInto
   - Feature-gated via `async_from`

2. **Async Fallible Conversion Traits (AsyncTryFrom/AsyncTryInto)**
   - `AsyncTryFrom<T>` trait - Async version of TryFrom<T>
   - `async_try_from(value: T) -> Result<Self, Error>` method
   - `AsyncTryInto<T>` trait - Async version of TryInto<T>
   - `async_try_into(self) -> Result<T, Error>` method
   - Associated `Error` type for conversion failures
   - Blanket impl: AsyncTryFrom implies AsyncTryInto
   - Feature-gated via `async_try_from`

3. **async_trait Macro Re-export**
   - `#[async_trait]` attribute macro
   - Enables async methods in traits
   - Required for trait implementations
   - Re-exported from external crate

4. **Facade Aggregation**
   - Re-exports from async_from crate
   - Unified namespace
   - Traditional namespace organization (own/orphan/exposed/prelude)
   - Dependency namespace

5. **Feature Architecture**
   - `enabled` - Master switch
   - `async_from` - AsyncFrom/AsyncInto traits (default)
   - `async_try_from` - AsyncTryFrom/AsyncTryInto traits (default)
   - Granular feature control

6. **Send Bounds**
   - Send requirements for async traits
   - Thread-safe conversions
   - Compatible with multi-threaded runtimes

#### Out-of-Scope

1. **NOT Runtime Implementation**
   - No async runtime (tokio/async-std)
   - Runtime-agnostic traits
   - **Rationale:** User chooses runtime

2. **NOT Async Streams**
   - No Stream/Sink traits
   - No async iterators
   - **Rationale:** Different use case, use futures crate

3. **NOT Async Utilities**
   - No join/select combinators
   - No timeout utilities
   - **Rationale:** Use futures or runtime-specific utilities

4. **NOT Derive Macros**
   - No #[derive(AsyncFrom)]
   - Manual implementation required
   - **Rationale:** Complex, out of scope

5. **NOT Error Type Definitions**
   - No standard error types
   - User-defined errors
   - **Rationale:** Conversion-specific errors

6. **NOT Sync Conversion Integration**
   - No auto-impl from From to AsyncFrom
   - Separate traits
   - **Rationale:** Explicit async semantics

7. **NOT Cancellation Support**
   - No built-in cancellation
   - Use runtime features
   - **Rationale:** Runtime-specific

8. **NOT Backpressure**
   - No flow control
   - Simple conversion semantics
   - **Rationale:** Not applicable to conversions

#### Boundaries

- **async_tools vs async_from**: async_tools is facade; async_from is implementation
- **async_tools vs futures**: async_tools for conversions; futures for combinators
- **async_tools vs tokio**: async_tools provides traits; tokio provides runtime

## Architecture

### Dependency Structure

```
async_tools (facade)
├── External Dependencies
│   └── async-trait (workspace) - Async trait methods
├── Internal Dependencies (workspace)
│   └── async_from - AsyncFrom/AsyncInto/AsyncTryFrom/AsyncTryInto
└── Dev Dependencies
    └── tokio (workspace, rt-multi-thread, time, macros) - Testing

async_from (implementation)
├── External Dependencies
│   └── async-trait (workspace)
└── Dev Dependencies
    └── tokio (workspace) - Testing
```

**Pattern:** Thin facade over async_from

### Module Organization

```
async_tools
├── lib.rs (top-level aggregation)
├── dependency/ - Dependency namespace
│   ├── async_trait (external)
│   └── async_from (workspace)
└── Standard namespaces: own, orphan, exposed, prelude
    └── Re-exports from async_from

async_from
├── lib.rs (implementation)
├── dependency/ - async_trait
└── private/ - Trait definitions
    ├── AsyncFrom trait
    ├── AsyncInto trait (blanket impl)
    ├── AsyncTryFrom trait
    └── AsyncTryInto trait (blanket impl)
```

### Feature Architecture

```
async_tools features:
  enabled (master switch, default)
  ├── async_from (default) → async_from/async_from
  └── async_try_from (default) → async_from/async_try_from

async_from features:
  enabled (master switch, default)
  ├── async_from (default) - Infallible async conversion
  └── async_try_from (default) - Fallible async conversion

full = all features
```

**Default Features:** `enabled`, `async_from`, `async_try_from`

### Trait Hierarchy

```
AsyncFrom<T> (infallible async conversion)
  ├── async fn async_from(T) -> Self
  └── Blanket impl → AsyncInto<T>
        └── async fn async_into(self) -> T

AsyncTryFrom<T> (fallible async conversion)
  ├── type Error: Debug
  ├── async fn async_try_from(T) -> Result<Self, Error>
  └── Blanket impl → AsyncTryInto<T>
        ├── type Error
        └── async fn async_try_into(self) -> Result<T, Error>
```

## Public API

### AsyncFrom Trait

```rust
use async_trait::async_trait;

/// Async version of std::convert::From
#[async_trait]
pub trait AsyncFrom<T>: Sized {
  /// Asynchronously converts a value of type T into Self
  async fn async_from(value: T) -> Self;
}
```

### AsyncInto Trait

```rust
/// Async version of std::convert::Into
#[async_trait]
pub trait AsyncInto<T>: Sized {
  /// Asynchronously converts Self into a value of type T
  async fn async_into(self) -> T;
}

// Blanket implementation
#[async_trait]
impl<T, U> AsyncInto<U> for T
where
  U: AsyncFrom<T> + Send,
  T: Send,
{
  async fn async_into(self) -> U {
    U::async_from(self).await
  }
}
```

### AsyncTryFrom Trait

```rust
use core::fmt::Debug;

/// Async version of std::convert::TryFrom
#[async_trait]
pub trait AsyncTryFrom<T>: Sized {
  /// Error type for failed conversions
  type Error: Debug;

  /// Asynchronously attempts to convert a value of type T into Self
  async fn async_try_from(value: T) -> Result<Self, Self::Error>;
}
```

### AsyncTryInto Trait

```rust
/// Async version of std::convert::TryInto
#[async_trait]
pub trait AsyncTryInto<T>: Sized {
  /// Error type for failed conversions
  type Error: Debug;

  /// Asynchronously attempts to convert Self into a value of type T
  async fn async_try_into(self) -> Result<T, Self::Error>;
}

// Blanket implementation
#[async_trait]
impl<T, U> AsyncTryInto<U> for T
where
  U: AsyncTryFrom<T> + Send,
  T: Send,
{
  type Error = U::Error;

  async fn async_try_into(self) -> Result<U, Self::Error> {
    U::async_try_from(self).await
  }
}
```

### async_trait Macro

```rust
// Re-exported from async_trait crate
pub use async_trait::async_trait;
```

## Usage Patterns

### Pattern 1: Basic AsyncFrom Implementation

```rust
use async_tools::{async_trait, AsyncFrom};

struct User {
  id: u64,
  name: String,
}

#[async_trait]
impl AsyncFrom<u64> for User {
  async fn async_from(id: u64) -> Self {
    // Simulate async database lookup
    tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    User {
      id,
      name: format!("User_{}", id),
    }
  }
}

#[tokio::main]
async fn main() {
  let user = User::async_from(42).await;
  assert_eq!(user.id, 42);
}
```

### Pattern 2: Using AsyncInto

```rust
use async_tools::{async_trait, AsyncFrom, AsyncInto};

struct MyNumber(u32);

#[async_trait]
impl AsyncFrom<String> for MyNumber {
  async fn async_from(value: String) -> Self {
    let num = value.parse::<u32>().unwrap_or(0);
    MyNumber(num)
  }
}

#[tokio::main]
async fn main() {
  // AsyncInto is automatically available
  let num: MyNumber = "42".to_string().async_into().await;
  assert_eq!(num.0, 42);
}
```

### Pattern 3: Fallible Async Conversion

```rust
use async_tools::{async_trait, AsyncTryFrom};
use std::num::ParseIntError;

struct ValidatedNumber(u32);

#[async_trait]
impl AsyncTryFrom<String> for ValidatedNumber {
  type Error = ParseIntError;

  async fn async_try_from(value: String) -> Result<Self, Self::Error> {
    // Async validation (e.g., check against external service)
    tokio::time::sleep(std::time::Duration::from_millis(5)).await;
    let num = value.parse::<u32>()?;
    Ok(ValidatedNumber(num))
  }
}

#[tokio::main]
async fn main() {
  let result = ValidatedNumber::async_try_from("42".to_string()).await;
  assert!(result.is_ok());

  let result = ValidatedNumber::async_try_from("invalid".to_string()).await;
  assert!(result.is_err());
}
```

### Pattern 4: Using AsyncTryInto

```rust
use async_tools::{async_trait, AsyncTryFrom, AsyncTryInto};
use std::num::ParseIntError;

struct MyNumber(u32);

#[async_trait]
impl AsyncTryFrom<String> for MyNumber {
  type Error = ParseIntError;

  async fn async_try_from(value: String) -> Result<Self, Self::Error> {
    let num = value.parse::<u32>()?;
    Ok(MyNumber(num))
  }
}

#[tokio::main]
async fn main() {
  let result: Result<MyNumber, _> = "42".to_string().async_try_into().await;
  assert!(result.is_ok());
  assert_eq!(result.unwrap().0, 42);
}
```

### Pattern 5: Network-Based Conversion

```rust
use async_tools::{async_trait, AsyncTryFrom};

struct RemoteConfig {
  settings: String,
}

#[derive(Debug)]
struct FetchError(String);

#[async_trait]
impl AsyncTryFrom<String> for RemoteConfig {
  type Error = FetchError;

  async fn async_try_from(url: String) -> Result<Self, Self::Error> {
    // Simulated network fetch
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    // In real code: fetch from URL
    Ok(RemoteConfig {
      settings: format!("Config from {}", url),
    })
  }
}
```

### Pattern 6: Database Record Conversion

```rust
use async_tools::{async_trait, AsyncFrom};

struct UserId(u64);
struct UserRecord {
  id: u64,
  name: String,
  email: String,
}

#[async_trait]
impl AsyncFrom<UserId> for UserRecord {
  async fn async_from(user_id: UserId) -> Self {
    // Simulated database query
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;

    UserRecord {
      id: user_id.0,
      name: "John Doe".to_string(),
      email: "john@example.com".to_string(),
    }
  }
}
```

### Pattern 7: Chained Async Conversions

```rust
use async_tools::{async_trait, AsyncFrom, AsyncInto};

struct RawData(Vec<u8>);
struct ParsedData(String);
struct ValidatedData(String);

#[async_trait]
impl AsyncFrom<RawData> for ParsedData {
  async fn async_from(raw: RawData) -> Self {
    // Async parsing
    ParsedData(String::from_utf8_lossy(&raw.0).to_string())
  }
}

#[async_trait]
impl AsyncFrom<ParsedData> for ValidatedData {
  async fn async_from(parsed: ParsedData) -> Self {
    // Async validation
    ValidatedData(parsed.0.trim().to_string())
  }
}

#[tokio::main]
async fn main() {
  let raw = RawData(b"  hello  ".to_vec());
  let parsed: ParsedData = raw.async_into().await;
  let validated: ValidatedData = parsed.async_into().await;
  assert_eq!(validated.0, "hello");
}
```

### Pattern 8: Feature-Gated Usage

```rust
// In Cargo.toml:
// [dependencies]
// async_tools = { version = "0.1", default-features = false, features = ["async_from"] }

use async_tools::{async_trait, AsyncFrom};

// Only infallible conversions available with async_from feature
struct MyType(i32);

#[async_trait]
impl AsyncFrom<i32> for MyType {
  async fn async_from(value: i32) -> Self {
    MyType(value)
  }
}
```

## Dependencies and Consumers

### Direct Dependencies

**External:**
- `async-trait` (workspace) - Enables async methods in traits

**Workspace:**
- `async_from` (v0.2.0) - AsyncFrom/AsyncInto/AsyncTryFrom/AsyncTryInto traits

**Dev:**
- `tokio` (workspace, rt-multi-thread, time, macros) - Testing runtime

### async_from Dependencies

**External:**
- `async-trait` (workspace)

**Dev:**
- `tokio` (workspace) - Testing runtime

### Consumers (Unknown)

**Likely used by:**
- Database abstraction layers
- HTTP client libraries
- Configuration loaders
- Data transformation pipelines
- Plugin systems with async initialization
- Serialization/deserialization frameworks

**Usage Pattern:** Workspace crates implement AsyncFrom/AsyncTryFrom for types that require I/O or other async operations during conversion.

## Design Rationale

### Why Separate Async Traits?

Standard From/Into are synchronous:

**Rationale:**
1. **Async Context**: Conversions involving I/O need async
2. **Clear Semantics**: Explicit async vs sync distinction
3. **Await Required**: Forces .await at call site
4. **Compatible**: Follows standard trait naming patterns

**Example:** Database lookups during conversion

### Why Use async_trait Crate?

Trait async methods require boxing:

**Rationale:**
1. **Stable Rust**: Async trait methods not yet stable
2. **Ergonomics**: Clean trait definition syntax
3. **Compatibility**: Works with current ecosystem
4. **Future**: Can migrate when RPITIT stabilizes

**Note:** Future Rust may make async_trait unnecessary

### Why Blanket Implementations?

AsyncFrom implies AsyncInto automatically:

**Rationale:**
1. **Consistency**: Mirrors std From/Into pattern
2. **Ergonomics**: Only implement AsyncFrom
3. **Symmetry**: Both directions available
4. **Less Boilerplate**: One impl, two traits

**Pattern:** Same as standard library

### Why Send Bounds?

Blanket impls require T: Send, U: Send:

**Rationale:**
1. **Thread Safety**: Async usually multi-threaded
2. **Runtime Compatibility**: Works with tokio/async-std
3. **Explicit**: Clear about thread requirements
4. **Common Case**: Most async code needs Send

**Limitation:** Non-Send types need manual AsyncInto impl

### Why Feature Gates?

async_from and async_try_from separate:

**Rationale:**
1. **Minimal Compilation**: Only include what's needed
2. **API Surface**: Control exposed traits
3. **Flexibility**: Can use just infallible conversions
4. **Dependencies**: Minimize code generation

**Default:** Both enabled for convenience

### Why Facade Pattern?

async_tools wraps async_from:

**Rationale:**
1. **Unified Import**: Single dependency for users
2. **Future Growth**: Can add more async utilities
3. **Naming**: More discoverable name
4. **Workspace Pattern**: Consistent with other *_tools crates

**Benefit:** Clean, unified API

### Why Error: Debug Bound?

AsyncTryFrom requires Error: Debug:

**Rationale:**
1. **Debuggability**: Errors can be printed
2. **Minimal**: Debug is widely implemented
3. **Useful**: Error messages in logs
4. **Standard Pattern**: Consistent with expectations

**Alternative:** Could require std::error::Error

## Testing Strategy

### Test Coverage

**tokio Available:**
- Use tokio for async test runtime
- Test async conversions
- Integration tests

### Test Focus

1. **AsyncFrom**: Basic async conversion
2. **AsyncInto**: Blanket impl works
3. **AsyncTryFrom**: Error handling
4. **AsyncTryInto**: Error propagation
5. **Send Bounds**: Thread-safe conversions
6. **Feature Gates**: Features work independently
7. **Chaining**: Multiple async conversions
8. **Edge Cases**: Empty values, large data

### Test Example

```rust
#[tokio::test]
async fn async_from_basic() {
  struct MyNum(i32);

  #[async_trait]
  impl AsyncFrom<i32> for MyNum {
    async fn async_from(value: i32) -> Self {
      MyNum(value)
    }
  }

  let num = MyNum::async_from(42).await;
  assert_eq!(num.0, 42);
}
```

### Known Test Limitations

1. **Runtime Required**: Need tokio/async-std for tests
2. **Send Testing**: Hard to test Send bound failures
3. **Timing**: Async tests can be flaky
4. **Coverage**: Blanket impls hard to test directly

## Future Considerations

### Potential Enhancements

1. **No async_trait**: Use RPITIT when stable
2. **Derive Macros**: #[derive(AsyncFrom)]
3. **More Utilities**: async spawn, timeout helpers
4. **Stream Conversions**: AsyncFrom for streams
5. **no_std Support**: Async without std
6. **Cancellation**: Built-in cancellation support
7. **Retry Logic**: Automatic retry on failure
8. **Tracing**: Integration with tracing crate

### Breaking Changes to Consider

1. **Remove async_trait**: When native async traits stable
2. **Change Send Bounds**: More flexible bounds
3. **Error Constraints**: Stronger error type requirements
4. **Naming**: AsyncFrom vs AsyncConvert

### Known Limitations

1. **async_trait Overhead**: Boxing and allocation
2. **Send Required**: Non-Send types need manual impl
3. **No Cancellation**: Can't cancel mid-conversion
4. **Runtime Agnostic**: No runtime-specific optimizations
5. **No Derive**: Manual implementation required

## Adoption Guidelines

### When to Use async_tools

**Good Candidates:**
- Conversions involving network I/O
- Database record loading
- File-based initialization
- External service calls during conversion
- Async validation pipelines
- Plugin initialization

**Poor Candidates:**
- Simple synchronous conversions (use From)
- CPU-bound conversions (use From)
- No I/O involved (use From)
- Performance-critical tight loops

### Implementation Pattern

```rust
use async_tools::{async_trait, AsyncFrom};

struct SourceType { /* ... */ }
struct TargetType { /* ... */ }

#[async_trait]
impl AsyncFrom<SourceType> for TargetType {
  async fn async_from(source: SourceType) -> Self {
    // Perform async operations
    // Return converted value
  }
}

// Usage
let target: TargetType = source.async_into().await;
```

### Best Practices

1. **Prefer AsyncFrom**: Implement AsyncFrom, get AsyncInto free
2. **Use AsyncTry**: When conversion can fail
3. **Document Async**: Explain what's async about the conversion
4. **Handle Errors**: Don't unwrap in AsyncTryFrom
5. **Consider Timeouts**: Add timeout at call site if needed
6. **Test Thoroughly**: Async code needs careful testing
7. **Keep Simple**: Conversions should be focused

### Migration from Sync

```rust
// Before (sync)
impl From<String> for MyType {
  fn from(s: String) -> Self {
    MyType(s)
  }
}

// After (async with I/O)
#[async_trait]
impl AsyncFrom<String> for MyType {
  async fn async_from(s: String) -> Self {
    // Now can do async operations
    let validated = validate_async(&s).await;
    MyType(validated)
  }
}
```

## Related Crates

**Dependencies:**
- **async_from**: Trait implementations (workspace)
- **async-trait**: Async trait methods (external)

**Related:**
- **futures**: Async primitives and combinators
- **tokio**: Async runtime
- **async-std**: Alternative async runtime

**Alternatives:**
- **Manual impl Future**: Lower-level, more complex
- **Pin-based traits**: More control, harder to use
- **None in std**: No standard async conversion traits yet

## References

- [API Documentation](https://docs.rs/async_tools)
- [async_from Documentation](https://docs.rs/async_from)
- [Repository](https://github.com/Wandalen/wTools/tree/master/module/core/async_tools)
- [async-trait Crate](https://crates.io/crates/async-trait)
- [Rust Async Book](https://rust-lang.github.io/async-book/)
- [readme.md](./readme.md)
