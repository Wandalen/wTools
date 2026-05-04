# Algorithm: Type Detection

### Scope

- **Purpose**: Document the string-to-typed-value conversion algorithm applied to all incoming configuration values.
- **Responsibility**: Documents the algorithm's inputs, ordered detection steps, and output type mapping.
- **In Scope**: Conversion of string values from environment variables and YAML scalar fields to typed configuration values.
- **Out of Scope**: YAML structured type parsing (handled natively by the YAML parser), resolution hierarchy (→ invariant/001).

### Abstract

Converts string representations of configuration values into their most specific type. Applied uniformly to environment variable values and to string scalars loaded from YAML config files. The conversion is deterministic and order-dependent: boolean patterns are checked first, then integer, then floating-point, then string fallback.

### Input

A string value sourced from an environment variable or a YAML scalar field.

### Output

A typed configuration value in one of four categories: boolean, integer, floating-point, or string.

### Algorithm

1. **Boolean check** — match case-insensitively against known boolean literals:
   - `"true"`, `"yes"`, `"1"`, `"on"` → boolean true
   - `"false"`, `"no"`, `"0"`, `"off"` → boolean false
2. **Integer check** — attempt integer parsing:
   - Success → integer value
3. **Float check** — attempt floating-point parsing:
   - Success and value is finite → floating-point value
   - Non-finite (NaN, ±Inf) → fall through to string
4. **String fallback** — all other inputs → string value

### Detection Table

| Input pattern | Detected type | Examples |
|--------------|---------------|---------|
| `true`, `yes`, `1`, `on` (case-insensitive) | boolean: true | `"True"`, `"YES"`, `"On"` |
| `false`, `no`, `0`, `off` (case-insensitive) | boolean: false | `"False"`, `"NO"`, `"Off"` |
| Signed integer string | integer | `"42"`, `"-100"`, `"999999999"` |
| Decimal or scientific float string | floating-point | `"3.14"`, `"-2.5"`, `"1.23e-4"` |
| Everything else | string | `"hello"`, `"2025-01-19"`, `"🔥"` |

### Complexity

O(1) per value — all checks are constant-time string comparisons or scalar parses. No allocation except for the string fallback case.

### Features

| File | Relationship |
|------|--------------|
| [feature/001_config_hierarchy.md](../feature/001_config_hierarchy.md) | Feature that includes type detection as a resolution step |

### Invariants

| File | Relationship |
|------|--------------|
| [invariant/001_resolution_hierarchy.md](../invariant/001_resolution_hierarchy.md) | Invariant whose levels this algorithm is applied at |
