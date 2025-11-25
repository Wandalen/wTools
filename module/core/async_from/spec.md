# async_from

Asynchronous versions of From, Into, TryFrom, and TryInto conversion traits.

## Overview

`async_from` provides asynchronous counterparts to Rust's standard conversion traits. While `From`/`Into` and `TryFrom`/`TryInto` handle synchronous conversions, many modern applications need to perform conversions that involve async operations like network requests, database queries, or file I/O.

The crate provides four traits:
- `AsyncFrom<T>` - Infallible async conversion from T
- `AsyncInto<T>` - Infallible async conversion into T (blanket impl from AsyncFrom)
- `AsyncTryFrom<T>` - Fallible async conversion from T
- `AsyncTryInto<T>` - Fallible async conversion into T (blanket impl from AsyncTryFrom)

All traits use the `async_trait` macro to enable async methods in traits.

### Scope

#### Responsibility

async_from is responsible for defining async conversion traits that mirror the standard library's From/Into/TryFrom/TryInto pattern. It enables type conversions that require awaiting asynchronous operations.

#### In-Scope

- **AsyncFrom trait**: Infallible async conversion from a source type
- **AsyncInto trait**: Infallible async conversion into a target type (blanket impl)
- **AsyncTryFrom trait**: Fallible async conversion from a source type
- **AsyncTryInto trait**: Fallible async conversion into a target type (blanket impl)
- **async_trait re-export**: Re-exports the `async_trait` macro for user implementations
- **Blanket implementations**: Auto-implement `AsyncInto` from `AsyncFrom`, `AsyncTryInto` from `AsyncTryFrom`

#### Out-of-Scope

- **Sync conversions**: Standard `From`/`Into` are in std, not here
- **Automatic implementations**: No derive macros for auto-generating implementations
- **Runtime-specific code**: Works with any async runtime (tokio, async-std, etc.)
- **Conversion combinators**: No chaining or composition utilities

#### Boundaries

- **Upstream**: Depends on `async_trait` for async trait method support
- **Downstream**: Used by `async_tools` (facade crate) and any code needing async conversions
- **Runtime boundary**: Runtime-agnostic; users bring their own executor

## Architecture

### Module Structure

```
async_from/
├── src/
│   └── lib.rs            # Complete implementation (traits + blanket impls)
├── tests/
├── Cargo.toml
├── readme.md
└── spec.md
```

### Trait Relationships

```
AsyncFrom<T>
    │
    └── blanket impl ──► AsyncInto<T>
        (impl<T, U> AsyncInto<U> for T where U: AsyncFrom<T>)

AsyncTryFrom<T>
    │
    └── blanket impl ──► AsyncTryInto<T>
        (impl<T, U> AsyncTryInto<U> for T where U: AsyncTryFrom<T>)
```

### Design Pattern

```
Standard Library               async_from
───────────────               ──────────────
From<T>           ──────►     AsyncFrom<T>
Into<T>           ──────►     AsyncInto<T>
TryFrom<T>        ──────►     AsyncTryFrom<T>
TryInto<T>        ──────►     AsyncTryInto<T>

Usage pattern is identical:
From::from(x)     ──────►     AsyncFrom::async_from(x).await
x.into()          ──────►     x.async_into().await
TryFrom::try_from(x)  ──►     AsyncTryFrom::async_try_from(x).await
x.try_into()      ──────►     x.async_try_into().await
```

## Public API

### Traits

#### `AsyncFrom<T>`

Trait for infallible async conversion from type T.

```rust
#[ async_trait ]
pub trait AsyncFrom< T >: Sized
{
  /// Asynchronously converts a value of type `T` into `Self`.
  async fn async_from( value: T ) -> Self;
}
```

#### `AsyncInto<T>`

Trait for infallible async conversion into type T.

```rust
#[ async_trait ]
pub trait AsyncInto< T >: Sized
{
  /// Asynchronously converts `Self` into a value of type `T`.
  async fn async_into( self ) -> T;
}

// Blanket implementation
#[ async_trait ]
impl< T, U > AsyncInto< U > for T
where
  U: AsyncFrom< T > + Send,
  T: Send,
{
  async fn async_into( self ) -> U
  {
    U::async_from( self ).await
  }
}
```

#### `AsyncTryFrom<T>`

Trait for fallible async conversion from type T.

```rust
#[ async_trait ]
pub trait AsyncTryFrom< T >: Sized
{
  /// The error type returned if the conversion fails.
  type Error: Debug;

  /// Asynchronously attempts to convert a value of type `T` into `Self`.
  async fn async_try_from( value: T ) -> Result< Self, Self::Error >;
}
```

#### `AsyncTryInto<T>`

Trait for fallible async conversion into type T.

```rust
#[ async_trait ]
pub trait AsyncTryInto< T >: Sized
{
  /// The error type returned if the conversion fails.
  type Error: Debug;

  /// Asynchronously attempts to convert `Self` into a value of type `T`.
  async fn async_try_into( self ) -> Result< T, Self::Error >;
}

// Blanket implementation
#[ async_trait ]
impl< T, U > AsyncTryInto< U > for T
where
  U: AsyncTryFrom< T > + Send,
  T: Send,
{
  type Error = U::Error;

  async fn async_try_into( self ) -> Result< U, Self::Error >
  {
    U::async_try_from( self ).await
  }
}
```

### Re-exports

```rust
// From async_trait crate
pub use async_trait::async_trait;
```

## Usage Patterns

### Infallible Async Conversion

```rust
use async_from::{ async_trait, AsyncFrom, AsyncInto };

struct User { id: u64, name: String }

struct UserId( u64 );

#[ async_trait ]
impl AsyncFrom< UserId > for User
{
  async fn async_from( id: UserId ) -> Self
  {
    // Simulate database lookup
    tokio::time::sleep( Duration::from_millis( 10 ) ).await;
    User { id: id.0, name: format!( "User {}", id.0 ) }
  }
}

#[ tokio::main ]
async fn main()
{
  // Using AsyncFrom directly
  let user = User::async_from( UserId( 42 ) ).await;

  // Using AsyncInto (blanket impl)
  let user: User = UserId( 42 ).async_into().await;
}
```

### Fallible Async Conversion

```rust
use async_from::{ async_trait, AsyncTryFrom, AsyncTryInto };
use std::io::Error as IoError;

struct Config { port: u16, host: String }

#[ async_trait ]
impl AsyncTryFrom< &str > for Config
{
  type Error = IoError;

  async fn async_try_from( path: &str ) -> Result< Self, Self::Error >
  {
    // Read config file asynchronously
    let content = tokio::fs::read_to_string( path ).await?;
    // Parse config...
    Ok( Config { port: 8080, host: "localhost".to_string() } )
  }
}

#[ tokio::main ]
async fn main()
{
  // Using AsyncTryFrom directly
  let config = Config::async_try_from( "/etc/app/config.toml" ).await;

  // Using AsyncTryInto (blanket impl)
  let config: Result< Config, _ > = "/etc/app/config.toml".async_try_into().await;
}
```

### Network-Based Conversion

```rust
use async_from::{ async_trait, AsyncTryFrom };

struct RemoteResource { data: Vec< u8 > }

#[ async_trait ]
impl AsyncTryFrom< url::Url > for RemoteResource
{
  type Error = reqwest::Error;

  async fn async_try_from( url: url::Url ) -> Result< Self, Self::Error >
  {
    let response = reqwest::get( url ).await?;
    let data = response.bytes().await?.to_vec();
    Ok( RemoteResource { data } )
  }
}
```

### Database Lookup Conversion

```rust
use async_from::{ async_trait, AsyncTryFrom };

struct UserRecord { id: i64, email: String }

#[ async_trait ]
impl AsyncTryFrom< i64 > for UserRecord
{
  type Error = sqlx::Error;

  async fn async_try_from( id: i64 ) -> Result< Self, Self::Error >
  {
    let pool = get_db_pool();  // Get connection pool
    sqlx::query_as!( UserRecord, "SELECT id, email FROM users WHERE id = $1", id )
      .fetch_one( &pool )
      .await
  }
}
```

## Feature Flags

| Feature | Default | Description |
|---------|---------|-------------|
| `enabled` | ✓ | Enable the crate |
| `async_from` | ✓ | Enable AsyncFrom/AsyncInto traits |
| `async_try_from` | ✓ | Enable AsyncTryFrom/AsyncTryInto traits |
| `full` | - | All features |

## Dependencies and Consumers

### Dependencies

| Dependency | Purpose |
|------------|---------|
| `async_trait` | Enable async methods in traits |

### Dev Dependencies

| Dependency | Purpose |
|------------|---------|
| `tokio` | Testing async code |

### Consumers

| Consumer | Relationship |
|----------|--------------|
| `async_tools` | Facade crate that re-exports async_from |

## Design Rationale

### Why async_trait?

Rust doesn't natively support async methods in traits yet (pending async trait stabilization). The `async_trait` macro provides this capability by desugaring async methods to return `Pin<Box<dyn Future>>`.

**Tradeoff**: Box allocation overhead, but necessary for trait-based async.

### Why Blanket Implementations?

Following std's pattern:
- Implement `AsyncFrom`, get `AsyncInto` automatically
- Implement `AsyncTryFrom`, get `AsyncTryInto` automatically

This reduces boilerplate and ensures consistency.

### Why Require Send?

Blanket implementations require `T: Send` and `U: Send` because:
- Most async runtimes are multi-threaded
- Futures may be moved between threads
- Without Send, the blanket impl wouldn't work with tokio/async-std

### Why Error: Debug?

`AsyncTryFrom/AsyncTryInto` require `Error: Debug` because:
- Errors should be inspectable for debugging
- Less restrictive than requiring `std::error::Error`
- Works with no_std compatible error types

## Testing Strategy

### Test Categories

1. **Infallible conversion**: Verify AsyncFrom/AsyncInto work correctly
2. **Fallible conversion**: Verify AsyncTryFrom/AsyncTryInto with success and failure
3. **Blanket implementations**: Verify Into traits work via From implementations

### Example Tests

```rust
#[ tokio::test ]
async fn test_async_from()
{
  let num = MyNumber::async_from( "42".to_string() ).await;
  assert_eq!( num.0, 42 );
}

#[ tokio::test ]
async fn test_async_into()
{
  let num: MyNumber = "42".to_string().async_into().await;
  assert_eq!( num.0, 42 );
}

#[ tokio::test ]
async fn test_async_try_from_success()
{
  let result = MyNumber::async_try_from( "42".to_string() ).await;
  assert!( result.is_ok() );
}

#[ tokio::test ]
async fn test_async_try_from_failure()
{
  let result = MyNumber::async_try_from( "not a number".to_string() ).await;
  assert!( result.is_err() );
}
```

## Future Considerations

### Potential Enhancements

1. **Native async traits**: When Rust stabilizes async traits, may remove async_trait dependency
2. **Derive macros**: Auto-generate implementations from attributes
3. **Conversion chains**: Combine multiple async conversions

### Known Limitations

1. **Box allocation**: async_trait adds heap allocation per call
2. **No no_std**: Requires std due to async_trait
3. **Send requirement**: Blanket impls require Send bounds

## Adoption Guidelines

### When to Use

- Conversions requiring I/O (network, database, filesystem)
- Conversions that take significant time
- Async contexts where blocking is not acceptable

### When Not to Use

- Simple in-memory conversions (use std From/Into)
- Synchronous code paths
- Performance-critical tight loops

### Migration Pattern

```rust
// Before: blocking conversion
impl From< UserId > for User
{
  fn from( id: UserId ) -> Self
  {
    let user = db::fetch_user_blocking( id.0 );  // Blocks!
    User { id: id.0, name: user.name }
  }
}

// After: async conversion
#[ async_trait ]
impl AsyncFrom< UserId > for User
{
  async fn async_from( id: UserId ) -> Self
  {
    let user = db::fetch_user( id.0 ).await;  // Non-blocking
    User { id: id.0, name: user.name }
  }
}
```

## Related Crates

| Crate | Relationship |
|-------|--------------|
| `async_tools` | Facade crate that re-exports this |
| `async_trait` | Enables async methods in traits |
| `futures` | Core async utilities |

## References

- [std::convert::From](https://doc.rust-lang.org/std/convert/trait.From.html)
- [std::convert::TryFrom](https://doc.rust-lang.org/std/convert/trait.TryFrom.html)
- [async_trait crate](https://docs.rs/async-trait)
