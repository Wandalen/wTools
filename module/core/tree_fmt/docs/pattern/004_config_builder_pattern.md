# Pattern: Config Builder Pattern

### Scope

- **Purpose**: Document the fluent builder API used by all formatter config structs.
- **Responsibility**: Canonical description of config construction conventions across all formatters.
- **In Scope**: Builder API shape, config struct names, construction at formatter time.
- **Out of Scope**: Per-config field details (see `api/003_config_types.md`), builder helper types (see `builder/`).

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `../architecture.md` | Original combined architecture document (retained per migration rules) |
| doc | `../api/003_config_types.md` | Config type API signatures |
| doc | `../builder/001_row_builder.md` | RowBuilder construction helper |
| doc | `../builder/002_tree_builder.md` | TreeBuilder construction helper |

### Description

All formatter config structs expose a fluent builder API for constructing formatter options. This provides a uniform construction experience across all formatters regardless of how many parameters a particular config supports. Callers chain method calls on the config object, then pass the finished config to the formatter constructor.

### Structure

#### Builder Pattern Shape

```rust
let config = TreeConfig::new()
  .show_branches( false )
  .max_depth( Some( 3 ) );
```

Each method returns `Self`, enabling chaining. Config structs are passed by value to formatter constructors.

#### Config Structs

| Config Type | Formatter |
|-------------|-----------|
| `TreeConfig` | `TreeFormatter` |
| `TableConfig` | `TableFormatter` |
| `ExpandedConfig` | `ExpandedFormatter` |

Each formatter accepts its corresponding config at construction time. Configs with zero-argument `new()` use `Default` internally to supply defaults for unchained fields.

### Rationale

The fluent builder pattern avoids large positional argument lists and allows callers to set only the fields relevant to their use case. Because configs are value types passed at construction, formatters are immutable after construction — the same formatter instance can be reused across multiple `format()` calls without shared mutable state.

### Sources

| File | Notes |
|------|-------|
| [../architecture.md](../architecture.md) | Original source; section "Configuration Builder Pattern" extracted into this instance |
