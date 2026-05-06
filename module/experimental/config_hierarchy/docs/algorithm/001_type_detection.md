# Algorithm: Type Detection

### Scope

- **Purpose**: Document the string-to-typed-value conversion algorithm applied to env var and file config values.
- **Responsibility**: Define the detection steps, detection table, and complexity guarantee.
- **In Scope**: String values from environment variables and YAML scalar strings.
- **Out of Scope**: YAML structured types (parsed natively by the YAML library), runtime params (passed as String and converted the same way).

### Abstract

Converts string representations of configuration values into their most specific JSON type. Applied uniformly to environment variable values and to string scalars loaded from YAML config files. The conversion is deterministic and order-dependent: boolean patterns are checked first, then integer, then float, then string fallback.

### Input

A string value from one of:
- Environment variable read from the process environment
- YAML scalar value loaded from a config file and coerced to string

### Output

One of four JSON value types: boolean, integer number, floating-point number, or string.

### Algorithm

1. **Boolean check** — match case-insensitively against known boolean literals:
   - `"true"`, `"yes"`, `"1"`, `"on"` → boolean true
   - `"false"`, `"no"`, `"0"`, `"off"` → boolean false
2. **Integer check** — attempt signed integer parse:
   - Success → integer number value
3. **Float check** — attempt floating-point parse:
   - Success and value is finite → float number value
   - Non-finite (NaN, ±Inf) → fall through to string
4. **String fallback** — all other inputs → string value

### Detection Table

| Input pattern | Detected type | Examples |
|--------------|---------------|---------|
| `true`, `yes`, `1`, `on` (case-insensitive) | boolean true | `"True"`, `"YES"`, `"On"` |
| `false`, `no`, `0`, `off` (case-insensitive) | boolean false | `"False"`, `"NO"`, `"Off"` |
| Signed integer string | integer number | `"42"`, `"-100"`, `"999999999"` |
| Decimal or scientific float string | float number | `"3.14"`, `"-2.5"`, `"1.23e-4"` |
| Everything else | string | `"hello"`, `"2025-01-19"`, `"🔥"` |

### Complexity

O(1) per value — all checks are constant-time string comparisons or scalar parses. No allocation except for the string fallback case.

### Algorithms

| File | Relationship |
|------|--------------|
| [algorithm/002_resolution_waterfall.md](002_resolution_waterfall.md) | Resolution waterfall that invokes this algorithm as a sub-step at levels 2–5 |

### Features

| File | Relationship |
|------|--------------|
| [feature/001_config_hierarchy.md](../feature/001_config_hierarchy.md) | Feature that uses this type detection algorithm |

### Invariants

| File | Relationship |
|------|--------------|
| [invariant/001_resolution_hierarchy.md](../invariant/001_resolution_hierarchy.md) | Applied at resolution levels 2 (env) and 3–5 (file values) |

### Sources

| File | Relationship |
|------|--------------|
| [src/type_detection.rs](../../src/type_detection.rs) | Complete algorithm implementation |

### Tests

| File | Relationship |
|------|--------------|
| [tests/type_detection_tests.rs](../../tests/type_detection_tests.rs) | Full algorithm test coverage |
