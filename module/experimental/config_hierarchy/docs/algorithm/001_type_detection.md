# Algorithm: Type Detection

### Scope

- **What**: String-to-typed-value conversion applied to all env var and file config values
- **Who**: `src/type_detection.rs` and all callers that read string-form config values
- **When**: Converting any string value from env vars or YAML scalar strings
- **Out of scope**: YAML structured types (parsed natively by serde_yaml), runtime params (passed as `String` and converted the same way)

### Abstract

Converts string representations of configuration values into their most specific JSON type. Applied uniformly to environment variable values and to string scalars loaded from YAML config files. The conversion is deterministic and order-dependent: boolean patterns are checked first, then integer, then float, then string fallback.

### Input

A `&str` value from one of:
- Environment variable read via `std::env::var()`
- YAML scalar value loaded by `serde_yaml` and coerced to string

### Output

A `serde_json::Value` (`JsonValue`) in one of four variants: `Bool`, `Number` (integer), `Number` (float), or `String`.

### Steps

1. **Boolean check** — match case-insensitively against known boolean literals:
   - `"true"`, `"yes"`, `"1"`, `"on"` → `JsonValue::Bool(true)`
   - `"false"`, `"no"`, `"0"`, `"off"` → `JsonValue::Bool(false)`
2. **Integer check** — attempt `str::parse::< i64 >()`:
   - Success → `JsonValue::Number(n)`
3. **Float check** — attempt `str::parse::< f64 >()`:
   - Success and value is finite → `JsonValue::Number(f)`
   - Non-finite (NaN, ±Inf) → fall through to string
4. **String fallback** — all other inputs → `JsonValue::String(s.to_string())`

### Detection Table

| Input pattern | Detected type | Examples |
|--------------|---------------|---------|
| `true`, `yes`, `1`, `on` (case-insensitive) | `Bool(true)` | `"True"`, `"YES"`, `"On"` |
| `false`, `no`, `0`, `off` (case-insensitive) | `Bool(false)` | `"False"`, `"NO"`, `"Off"` |
| Signed integer string | `Number` (i64) | `"42"`, `"-100"`, `"999999999"` |
| Decimal or scientific float string | `Number` (f64) | `"3.14"`, `"-2.5"`, `"1.23e-4"` |
| Everything else | `String` | `"hello"`, `"2025-01-19"`, `"🔥"` |

### Complexity

O(1) per value — all checks are constant-time string comparisons or scalar parses. No allocation except for the `String` fallback case.

### Cross-References

| Type | Target | Relationship |
|------|--------|-------------|
| doc | invariant/001_resolution_hierarchy.md | applied at levels 2 (env) and 3–5 (file values) |
| doc | feature/001_config_hierarchy.md | type detection is part of this feature |
