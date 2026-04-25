# Invariant: Resolution Hierarchy

### Scope

- **What**: Priority ordering of configuration sources for all resolution operations
- **Who**: All callers of `resolve_config_value` and `resolve_all_config`
- **When**: Every config value lookup, regardless of source mix
- **Out of scope**: File format (→ format/001), trait signatures (→ api/), type conversion (→ algorithm/001)

### Statement

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

All three components are customizable via `ConfigPaths` methods.

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

### Enforcement

The ordering is enforced structurally in `src/hierarchy.rs`:

- `resolve_config_value< D, P >()` checks each level in order via early return
- The first `return` reached short-circuits all lower-priority checks
- `discover_local_configs_internal< P >()` returns paths tagged with depth (`0` = current, `1+` = parent), preserving depth-beats-pattern ordering
- Within each depth, the temporary pattern (`-`) is placed before the permanent pattern (`.`) in the returned slice

### Violation Consequences

If the `sources` slice is assembled in incorrect order (e.g., for hypothetical future direct use), lower-priority values silently override higher-priority values. There is no runtime check on ordering — the invariant is maintained exclusively by the implementation in `src/hierarchy.rs`.

### Cross-References

| Type | Target | Relationship |
|------|--------|-------------|
| doc | feature/001_config_hierarchy.md | feature this invariant governs |
| doc | api/001_config_paths_trait.md | methods controlling env var format and path construction |
| doc | algorithm/001_type_detection.md | applied to values found at each level |
