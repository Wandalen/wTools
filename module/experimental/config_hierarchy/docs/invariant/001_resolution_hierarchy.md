# Invariant: Resolution Hierarchy

### Scope

- **Purpose**: Define the priority ordering of configuration sources for all resolution operations.
- **Responsibility**: Document the ordering contract that all callers of resolution functions must follow.
- **In Scope**: Every config value lookup; six-level priority hierarchy; dual-pattern directory rule; environment variable format; global path construction.
- **Out of Scope**: File format (→ format/001), trait signatures (→ api/), type conversion (→ algorithm/001).

### Invariant Statement

Configuration sources are resolved in strict priority order from highest to lowest:

| Priority | Level | Source | Requires |
|----------|-------|--------|---------|
| 1 (highest) | Runtime | Parameters passed directly at call time | always available |
| 2 | Environment | Environment variables matching `{PREFIX}{SEP}{PARAM}` | always available |
| 3 | LocalCurrent | Config file in the current working directory | `file_ops` feature |
| 4 | LocalParent | Config files in ancestor directories, nearest first | `file_ops` feature |
| 5 | Global | `$PRO/.persistent/.{app_name}/config.yaml` or OS fallback | `file_ops` feature |
| 6 (lowest) | Default | Application-defined default values | always available |

The first source that contains a value for the requested parameter wins. No merging occurs — a value from priority 1 completely replaces any value that would have come from priority 2–6.

#### Dual-Pattern Rule Within a Directory

Within each directory, the temporary pattern takes priority over the permanent pattern:

- `-{app_name}/{config_filename}` — temporary (gitignored, higher within same directory)
- `.{app_name}/{config_filename}` — permanent (version-controlled, lower within same directory)

**Directory depth beats pattern type.** A permanent config in the current directory (`LocalCurrent`) overrides a temporary config in a parent directory (`LocalParent`). Priority 3 always wins over priority 4, regardless of which file pattern is used.

#### Environment Variable Format

Environment variable names are constructed as:

```
{env_var_prefix}{env_var_separator}{param_name_cased}
```

With defaults: `{APP_NAME}_{PARAM_NAME_UPPERCASE}` — e.g., `MYAPP_TIMEOUT`.

All three components are customizable via `ConfigPaths` methods. See [api/001_config_paths_trait.md](../api/001_config_paths_trait.md) for the method signatures controlling each component.

#### Global Path Construction

The global config path is constructed as:

```
${pro_env_var}/{global_persistent_dir}/{local_permanent_prefix}{app_name}/{global_config_filename}
```

With defaults: `$PRO/.persistent/.{app_name}/config.yaml`

Note: `local_permanent_prefix` (default `"."`) is prepended to `app_name` in the global path — producing `.myapp`, not `myapp`.

OS-specific fallback path (when `$PRO` is not set):
- Linux: `${XDG_CONFIG_HOME}/{app_name}/` or `$HOME/.config/{app_name}/`
- macOS: `$HOME/Library/Application Support/{app_name}/`
- Windows: `%APPDATA%/{app_name}/`

### Enforcement Mechanism

The ordering is enforced structurally in the resolution implementation:

- The resolution function checks each level in order via early return — the first level that yields a value short-circuits all lower-priority checks
- The path discovery function returns config paths tagged with directory depth (`0` = current, `1+` = parent ancestor), preserving depth-beats-pattern ordering
- Within each depth level, the temporary pattern is placed before the permanent pattern

### Violation Consequences

If the sources are assembled in incorrect order (e.g., for hypothetical future direct use), lower-priority values silently override higher-priority values. There is no runtime check on ordering — the invariant is maintained exclusively by the resolution implementation.

### Algorithms

| File | Relationship |
|------|--------------|
| [algorithm/001_type_detection.md](../algorithm/001_type_detection.md) | Type detection applied to values found at each level |
| [algorithm/002_resolution_waterfall.md](../algorithm/002_resolution_waterfall.md) | Algorithm that implements this priority ordering |

### APIs

| File | Relationship |
|------|--------------|
| [api/001_config_paths_trait.md](../api/001_config_paths_trait.md) | Methods controlling env var format and path construction |
| [api/002_config_defaults_trait.md](../api/002_config_defaults_trait.md) | Defaults are the lowest-priority level (priority 6) |
| [api/004_config_manager.md](../api/004_config_manager.md) | Manager type whose resolution operations must follow this ordering |

### Features

| File | Relationship |
|------|--------------|
| [feature/001_config_hierarchy.md](../feature/001_config_hierarchy.md) | Feature this invariant governs |

### Formats

| File | Relationship |
|------|--------------|
| [format/001_config_file_format.md](../format/001_config_file_format.md) | Format of files loaded at each level |

### Sources

| File | Relationship |
|------|--------------|
| [src/hierarchy.rs](../../src/hierarchy.rs) | 6-level waterfall resolution implementation |
| [src/path_discovery.rs](../../src/path_discovery.rs) | Local config discovery with depth tagging |

### Tests

| File | Relationship |
|------|--------------|
| [tests/hierarchy_tests.rs](../../tests/hierarchy_tests.rs) | Resolution priority ordering and early-return tests |
| [tests/dual_pattern_tests.rs](../../tests/dual_pattern_tests.rs) | Dual-pattern and depth-beats-pattern tests |
| [tests/path_standards_tests.rs](../../tests/path_standards_tests.rs) | Path discovery and depth tagging tests |
