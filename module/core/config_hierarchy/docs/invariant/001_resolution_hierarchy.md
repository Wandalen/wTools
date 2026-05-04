# Invariant: Resolution Hierarchy

### Scope

- **Purpose**: Define the strict priority ordering of configuration sources for all resolution operations.
- **Responsibility**: Documents the six-level hierarchy, the dual-pattern rule within directories, and the consequences of ordering violations.
- **In Scope**: Priority levels, environment variable naming format, global path construction, and ordering enforcement.
- **Out of Scope**: File format (→ format/001), trait signatures (→ api/), type conversion (→ algorithm/001).

### Invariant Statement

Configuration sources are resolved in strict priority order from highest to lowest:

| Priority | Level | Source | Requires |
|----------|-------|--------|---------|
| 1 (highest) | Runtime | Parameters passed directly at call time | always available |
| 2 | Environment | Environment variables matching `{PREFIX}{SEP}{PARAM}` | always available |
| 3 | LocalCurrent | Config file in the current working directory | I/O feature |
| 4 | LocalParent | Config files in ancestor directories, nearest first | I/O feature |
| 5 | Global | `$PRO/.persistent/.{app_name}/config.yaml` or OS fallback | I/O feature |
| 6 (lowest) | Default | Application-defined default values | always available |

The first source that contains a value for the requested parameter wins. No merging occurs — a value from priority 1 completely replaces any value from priority 2–6.

#### Dual-Pattern Rule Within a Directory

Within each directory, the temporary pattern takes priority over the permanent pattern:

- `-{app_name}/{config_filename}` — temporary (gitignored, higher within same directory)
- `.{app_name}/{config_filename}` — permanent (version-controlled, lower within same directory)

**Directory depth beats pattern type.** A permanent config in the current directory (LocalCurrent) overrides a temporary config in a parent directory (LocalParent). Priority 3 always wins over priority 4, regardless of which file pattern is used.

#### Environment Variable Format

Environment variable names follow the pattern `{env_var_prefix}{env_var_separator}{param_name_cased}`. With defaults: `{APP_NAME}_{PARAM_NAME_UPPERCASE}` — e.g., `MYAPP_TIMEOUT`.

All three components are customizable via the path configuration trait.

#### Global Path Construction

The global config path follows the pattern `${pro_env_var}/{global_persistent_dir}/{local_permanent_prefix}{app_name}/{global_config_filename}`. With defaults: `$PRO/.persistent/.{app_name}/config.yaml`

Note: `local_permanent_prefix` (default `"."`) is prepended to `app_name` in the global path — producing `.myapp`, not `myapp`.

OS-specific fallback path (when `$PRO` is not set):
- Linux: `${XDG_CONFIG_HOME}/{app_name}/` or `$HOME/.config/{app_name}/`
- macOS: `$HOME/Library/Application Support/{app_name}/`
- Windows: `%APPDATA%/{app_name}/`

### Enforcement Mechanism

The ordering is enforced structurally in the resolution implementation:

- The single-value resolver checks each level in order via early return — the first level containing a value short-circuits all lower-priority checks
- The local config discoverer returns paths tagged with directory depth, preserving depth-beats-pattern ordering
- Within each depth, the temporary pattern is placed before the permanent pattern in the returned list

### Violation Consequences

If sources are assembled in incorrect order, lower-priority values silently override higher-priority values. There is no runtime check on ordering — the invariant is maintained exclusively by the implementation.

### Algorithms

| File | Relationship |
|------|--------------|
| [algorithm/001_type_detection.md](../algorithm/001_type_detection.md) | Applied to values found at each level |

### APIs

| File | Relationship |
|------|--------------|
| [api/001_config_paths_trait.md](../api/001_config_paths_trait.md) | Methods controlling env var format and path construction |
| [api/002_config_defaults_trait.md](../api/002_config_defaults_trait.md) | Defaults are the lowest-priority level (priority 6) |

### Features

| File | Relationship |
|------|--------------|
| [feature/001_config_hierarchy.md](../feature/001_config_hierarchy.md) | Feature this invariant governs |

### Formats

| File | Relationship |
|------|--------------|
| [format/001_config_file_format.md](../format/001_config_file_format.md) | Format of files loaded at each level |
