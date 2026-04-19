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

### Problem

Formatter configs have many optional parameters (TableConfig alone supports 20+). Positional argument lists become unwieldy and error-prone — callers cannot set only the fields relevant to their use case without writing boilerplate for every other field.

### Solution

All formatter config structs expose a fluent builder API. Each setter method returns `Self`, enabling method call chaining. Callers chain only the fields they care about; all others retain their defaults. The finished config is passed by value to the formatter constructor at call time.

#### Builder Pattern Shape

Each setter returns `Self` for chaining. Config structs are passed by value to formatter constructors.

#### Config Structs

| Config Type | Formatter |
|-------------|-----------|
| `TreeConfig` | `TreeFormatter` |
| `TableConfig` | `TableFormatter` |
| `ExpandedConfig` | `ExpandedFormatter` |

Each formatter accepts its corresponding config at construction time. Configs with zero-argument `new()` use `Default` internally to supply defaults for unchained fields.

### Applicability

Apply this pattern for any formatter config struct with more than 2–3 fields. Use when callers need to specify only a subset of options with sensible defaults for the rest. The pattern is already in use across all three config types — new config structs should follow the same convention.

### Consequences

The fluent builder pattern avoids large positional argument lists and allows callers to set only the fields relevant to their use case. Because configs are value types passed at construction, formatters are immutable after construction — the same formatter instance can be reused across multiple `format()` calls without shared mutable state. The trade-off is that all builder methods must be `#[ must_use ]` to prevent silent no-ops from call sites that forget to chain the result.

### Sources

| File | Notes |
|------|-------|
| [../architecture.md](../architecture.md) | Original source; section "Configuration Builder Pattern" extracted into this instance |
