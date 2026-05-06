# Algorithm: Resolution Waterfall

### Scope

- **Purpose**: Document the 6-level priority waterfall used to resolve a single configuration parameter.
- **Responsibility**: Define the ordered check sequence, short-circuit rule, and secondary scan behavior.
- **In Scope**: Single-value resolution; all-values resolution; secondary scan for undeclared keys.
- **Out of Scope**: Type conversion of found values (→ algorithm/001); path construction (→ api/001); format of files read (→ format/001).

### Abstract

Resolves a configuration parameter by checking six prioritized sources in strict descending order. The first source that contains a value for the requested parameter short-circuits all lower-priority checks — no merging occurs. A secondary scan collects undeclared keys from config files after primary resolution completes.

### Input

- A parameter name (string key to look up)
- A runtime override map (highest-priority source, always available)
- An environment variable reader (second-priority source, always available)
- A set of discovered config file paths with priority order (requires `file_ops` feature)
- A defaults map (lowest-priority source, always available)

### Algorithm

**Single-value resolution:**

1. **Level 1 — Runtime check**: If the runtime override map contains the parameter key, return its value immediately.
2. **Level 2 — Environment check**: Construct the env var name from prefix + separator + cased parameter name. If the variable is set, convert the string value via type detection (→ algorithm/001) and return.
3. **Level 3 — LocalCurrent check** *(file_ops only)*: Read the config file in the current working directory (if it exists). If it contains the key, return the value.
4. **Level 4 — LocalParent check** *(file_ops only)*: Iterate discovered ancestor config files, nearest ancestor first. Return the value from the first file that contains the key.
5. **Level 5 — Global check** *(file_ops only)*: Read the global config file at the OS-specific path. If it contains the key, return the value.
6. **Level 6 — Default check**: Return the application-defined default for this parameter. If no default exists, return a null value with a default source label.

**All-values resolution:**

1. Iterate the declared parameter list; resolve each via the single-value algorithm above.
2. **Secondary scan**: Read all global and local config files. For each key found that was not in the declared parameter list, resolve it via the single-value algorithm and add it to the result map.

### Output

For single-value: a pair of (typed value, source label) — never absent; falls back to null with default source label.

For all-values: a map of parameter name → (typed value, source label) covering all declared parameters plus any undeclared keys found in config files.

### Complexity

O(P × L) per all-values resolution, where P = parameter count and L = number of config file sources. The short-circuit rule makes single-value resolution O(1) in the common case when a runtime or env override is present.

### Examples

**Env override wins:** Parameter `timeout`, no runtime override, env `MYAPP_TIMEOUT=30`:
- Level 1: not in runtime map — continue
- Level 2: env var set to `"30"` → type detection yields integer 30 → return `(30, Environment)`

**Global file fallback:** Parameter `retries`, no overrides, global config contains `retries: 5`:
- Levels 1–4: not found — continue
- Level 5: global file found, `retries: 5` → return `(5, Global("/path/to/config.yaml"))`

### Algorithms

| File | Relationship |
|------|--------------|
| [algorithm/001_type_detection.md](001_type_detection.md) | Type conversion applied at levels 2–5 |

### APIs

| File | Relationship |
|------|--------------|
| [api/004_config_manager.md](../api/004_config_manager.md) | Public entry point that calls this algorithm |

### Features

| File | Relationship |
|------|--------------|
| [feature/001_config_hierarchy.md](../feature/001_config_hierarchy.md) | Feature that uses this resolution algorithm |

### Invariants

| File | Relationship |
|------|--------------|
| [invariant/001_resolution_hierarchy.md](../invariant/001_resolution_hierarchy.md) | Priority ordering contract this algorithm implements |

### Sources

| File | Relationship |
|------|--------------|
| [src/hierarchy.rs](../../src/hierarchy.rs) | Complete algorithm implementation |

### Tests

| File | Relationship |
|------|--------------|
| [tests/hierarchy_tests.rs](../../tests/hierarchy_tests.rs) | Priority ordering and short-circuit tests |
| [tests/feature_tests.rs](../../tests/feature_tests.rs) | End-to-end resolution tests |
| [tests/scope_operations_tests.rs](../../tests/scope_operations_tests.rs) | Scope-specific resolution tests |
