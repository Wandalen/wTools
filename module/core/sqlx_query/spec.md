# sqlx_query

Feature-gated macros for switching between compile-time and runtime SQLx query checking.

## Overview

`sqlx_query` provides wrapper macros (`query!` and `query_as!`) that expand to either SQLx's compile-time checked macros or runtime query functions, based on a feature flag. This enables development workflows where compile-time query checking can be disabled for faster iteration, then re-enabled for production builds to catch SQL errors.

The key insight is that SQLx's compile-time checking (via `query!` and `query_as!` macros) requires a running database during compilation, which can slow down development. By providing a toggle, developers can:
- Disable checking during rapid development (faster builds)
- Enable checking before releases (catch SQL errors)

### Scope

#### Responsibility

sqlx_query is responsible for providing feature-gated wrapper macros that switch between SQLx's compile-time query checking macros and runtime query functions, allowing flexible development workflows.

#### In-Scope

- **`query!` macro**: Wrapper for `sqlx::query!` / `sqlx::query`
- **`query_as!` macro**: Wrapper for `sqlx::query_as!` / `sqlx::query_as`
- **Feature toggle**: `sqlx_compiletime_checks` feature controls mode
- **Bind parameter support**: Pass-through of query bindings

#### Out-of-Scope

- **SQLx implementation**: Uses SQLx as-is, no modifications
- **Database drivers**: No driver-specific code
- **Connection management**: No pooling or connection logic
- **Query builders**: Raw SQL only, no builder DSL
- **Migrations**: No schema management

#### Boundaries

- **Upstream**: Requires `sqlx` crate in consumer's dependencies
- **Downstream**: Used by applications needing flexible SQLx query checking
- **SQLx boundary**: Consumer must configure SQLx separately

## Architecture

### Module Structure

```
sqlx_query/
├── src/
│   └── lib.rs            # Macro definitions
├── Cargo.toml
├── readme.md
└── spec.md
```

### Macro Expansion

```
┌─────────────────────────────────────────────────────────────────┐
│                        query! macro                              │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  #[cfg(feature = "sqlx_compiletime_checks")]                    │
│  ─────────────────────────────────────────                      │
│  sqlx_query::query!("SELECT...")                                │
│       ↓ expands to                                              │
│  ::sqlx::query("SELECT...")  ← Runtime, no DB needed at compile │
│                                                                 │
│  #[cfg(not(feature = "sqlx_compiletime_checks"))]               │
│  ───────────────────────────────────────────────                │
│  sqlx_query::query!("SELECT...")                                │
│       ↓ expands to                                              │
│  ::sqlx::query!("SELECT...")  ← Compile-time, DB required       │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

## Public API

### Macros

#### `query!`

Execute a SQL query with optional bind parameters.

```rust
// Without bindings
query!( "SELECT * FROM users" )
  .fetch_all( &pool )
  .await?;

// With bindings
query!( "DELETE FROM users WHERE id = $1", user_id )
  .execute( &pool )
  .await?;
```

**Expansion**:
- With `sqlx_compiletime_checks`: `::sqlx::query(sql)` or `::sqlx::query(sql).bind(...)`
- Without feature: `::sqlx::query!(sql)` or `::sqlx::query!(sql, ...)`

#### `query_as!`

Execute a SQL query and map results to a type.

```rust
// Without bindings
let users: Vec<User> = query_as!( User, "SELECT * FROM users" )
  .fetch_all( &pool )
  .await?;

// With bindings
let user: User = query_as!( User, "SELECT * FROM users WHERE id = $1", id )
  .fetch_one( &pool )
  .await?;
```

**Expansion**:
- With `sqlx_compiletime_checks`: `::sqlx::query_as::<_, Type>(sql)` or with `.bind(...)`
- Without feature: `::sqlx::query_as!(Type, sql)` or `::sqlx::query_as!(Type, sql, ...)`

## Usage Patterns

### Development Mode (No Compile-Time Checks)

```toml
# Cargo.toml
[dependencies]
sqlx_query = "0.2"
sqlx = { version = "0.7", features = ["runtime-tokio", "postgres"] }

# No sqlx_compiletime_checks feature - faster builds, no DB needed
```

```rust
use sqlx_query::*;

let users = query_as!( User, "SELECT * FROM users" )
  .fetch_all( &pool )
  .await?;
```

### Production Mode (Compile-Time Checks)

```toml
# Cargo.toml
[dependencies]
sqlx_query = { version = "0.2", features = ["sqlx_compiletime_checks"] }
sqlx = { version = "0.7", features = ["runtime-tokio", "postgres"] }

# DATABASE_URL environment variable required at compile time
```

```rust
// Same code, but queries are validated at compile time
use sqlx_query::*;

let users = query_as!( User, "SELECT * FROM users" )
  .fetch_all( &pool )
  .await?;
```

### CI/CD Workflow

```yaml
# Development builds (fast)
- run: cargo build

# Release builds (with query checking)
- run: cargo build --features sqlx_compiletime_checks
  env:
    DATABASE_URL: ${{ secrets.DATABASE_URL }}
```

### Conditional Feature in Application

```toml
# Application Cargo.toml
[features]
default = []
production = ["sqlx_query/sqlx_compiletime_checks"]
```

## Feature Flags

| Feature | Default | Description |
|---------|---------|-------------|
| `enabled` | ✓ | Enable the crate |
| `full` | - | All features (same as enabled) |
| `no_std` | - | no_std compatibility |
| `use_alloc` | - | alloc in no_std |

**Note**: The `sqlx_compiletime_checks` feature is expected to be defined in the *consumer's* crate, not in sqlx_query itself. The macros check for this feature at expansion time.

## Dependencies and Consumers

### Dependencies

None - the crate only provides macros that reference `::sqlx::*`.

### Consumer Requirements

The consumer must have `sqlx` in their dependencies:

```toml
[dependencies]
sqlx_query = "0.2"
sqlx = { version = "0.7", features = ["runtime-tokio", "postgres"] }
```

### Potential Consumers

- Web applications using SQLx for database access
- CLI tools with database backends
- Services needing flexible build configurations

## Design Rationale

### Why Feature Toggle?

SQLx's compile-time checking is powerful but has costs:
1. **Requires database**: Must have DATABASE_URL and running DB at compile time
2. **Slower builds**: Extra validation step
3. **CI complexity**: Need database in CI environment

The feature toggle allows:
- Fast iteration during development
- Full validation before release
- Flexible CI/CD configurations

### Why Not Just Use sqlx::query?

Runtime `sqlx::query` doesn't provide:
- SQL syntax validation at compile time
- Type checking for query results
- Parameter type validation

The toggle preserves access to these benefits when needed.

### Why Separate Crate?

- **Reusability**: Multiple projects can share the same pattern
- **Consistency**: Uniform approach across workspace
- **Documentation**: Single place for usage guidance

## Testing Strategy

### Test Categories

1. **Macro expansion**: Verify correct expansion in both modes
2. **Syntax**: Ensure SQL and bindings pass through correctly
3. **Feature gating**: Confirm feature flag controls behavior

### Note on Testing

Full integration testing requires SQLx and a database, which is typically done in consumer applications rather than this utility crate.

## Future Considerations

### Potential Enhancements

1. **More query variants**: `query_scalar!`, `query_unchecked!`
2. **Custom error types**: Better error messages in development mode
3. **Logging**: Optional query logging in development mode

### Known Limitations

1. **Consumer feature**: `sqlx_compiletime_checks` must be in consumer's Cargo.toml
2. **SQLx version coupling**: Must match SQLx API
3. **No validation in dev mode**: SQL errors only caught at runtime

## Adoption Guidelines

### When to Use

- Projects using SQLx that want flexible compile-time checking
- CI/CD pipelines needing fast development builds
- Teams iterating quickly on database schemas

### When Not to Use

- Projects that always want compile-time checking
- Projects not using SQLx
- Simple scripts where build speed doesn't matter

### Integration Pattern

```rust
// In your application
use sqlx_query::*;

// Write queries the same way regardless of mode
pub async fn get_user( pool: &PgPool, id: i64 ) -> Result< User, sqlx::Error >
{
  query_as!( User, "SELECT * FROM users WHERE id = $1", id )
    .fetch_one( pool )
    .await
}
```

## Related Crates

| Crate | Relationship |
|-------|--------------|
| `sqlx` | Upstream - provides actual query functionality |
| `sea-query` | Alternative - query builder approach |
| `diesel` | Alternative - different ORM approach |

## References

- [SQLx documentation](https://docs.rs/sqlx)
- [SQLx compile-time checking](https://github.com/launchbadge/sqlx#compile-time-verification)
