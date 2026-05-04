# API: ConfigPaths Trait

### Scope

- **Purpose**: Define the path and naming configuration contract for the configuration manager.
- **Responsibility**: Documents operations, default derivation rules, error conditions, and compatibility guarantees of ConfigPaths.
- **In Scope**: Application name, environment variable formatting, local and global path construction, OS fallback paths.
- **Out of Scope**: Default values (→ api/002), validation hooks (→ api/003), file format (→ format/001).

### Abstract

ConfigPaths is one of the three traits applications implement to configure the manager. It controls all path derivation — where config files live, what they are named, how environment variables are formatted, and which OS-specific directories to use as fallbacks. Only `app_name` is required; all other methods have sensible defaults derived from the application name.

### Operations

#### Path Derivation Table

With default settings and `app_name = "myapp"`:

| Path type | Formula | Result |
|-----------|---------|--------|
| Local permanent | `{local_permanent_prefix}{app_name}/{local_config_filename}` | `.myapp/config.yaml` |
| Local temporary | `{local_temporary_prefix}{app_name}/{local_config_filename}` | `-myapp/config.yaml` |
| Global | `${pro_env_var}/{global_persistent_dir}/{local_permanent_prefix}{app_name}/{global_config_filename}` | `$PRO/.persistent/.myapp/config.yaml` |
| Env var | `{env_var_prefix}{env_var_separator}{param_name_cased}` | `MYAPP_TIMEOUT` |

Note: `local_permanent_prefix` (default `"."`) is applied to `app_name` in the global path too — producing `.myapp` not `myapp`.

#### Methods Reference

| Method | Required | Default | Description |
|--------|----------|---------|-------------|
| `app_name` | yes | — | Application identifier; must be non-empty, no path separators or traversal sequences |
| `env_var_prefix` | no | app name uppercased | Prefix for all environment variables |
| `env_var_separator` | no | `"_"` | Character(s) between prefix and parameter name |
| `env_var_casing` | no | uppercase | Casing strategy for env var parameter names |
| `local_permanent_prefix` | no | `"."` | Prefix for permanent local config directories |
| `local_temporary_prefix` | no | `"-"` | Prefix for temporary local config directories |
| `local_config_filename` | no | `"config.yaml"` | Filename for all local config files |
| `global_persistent_dir` | no | `".persistent"` | Subdirectory under the workspace root for global configs |
| `global_config_filename` | no | `"config.yaml"` | Filename for global config file |
| `pro_env_var` | no | `"PRO"` | Name of env var pointing to workspace root |
| `home_env_var` | no | `"HOME"` | Name of env var pointing to home directory |
| `xdg_config_home_var` | no | `"XDG_CONFIG_HOME"` | Linux XDG Base Directory variable name |
| `appdata_var` | no | `"APPDATA"` | Windows application data directory variable |
| `linux_config_base` | no | `".config"` | Linux config base relative to home directory |
| `macos_config_base` | no | `"Library/Application Support"` | macOS config base relative to home directory |

### Error Handling

Path discovery returns an error when `app_name` fails validation:
- Empty string
- Contains path separator characters
- Contains path traversal sequences

Local config discovery silently skips invalid application names to avoid breaking the discovery loop. No other methods are validated at call time.

### Compatibility Guarantees

- Adding new optional methods with default implementations is a non-breaking change
- Changing `app_name` in an existing implementation changes all derived paths — existing config files at the old paths become unreachable
- Changing any optional method changes only the paths or env var names derived from that method

### Pitfall

The default implementation of `env_var_prefix` derives the prefix dynamically by uppercasing the application name on every call. This involves a heap allocation that is never freed. Applications calling `env_var_prefix` in a tight loop will accumulate unbounded memory. Override `env_var_prefix` with a fixed string literal to avoid this.

### APIs

| File | Relationship |
|------|--------------|
| [api/002_config_defaults_trait.md](../api/002_config_defaults_trait.md) | Companion required trait |
| [api/003_config_validator_trait.md](../api/003_config_validator_trait.md) | Companion optional trait |

### Features

| File | Relationship |
|------|--------------|
| [feature/001_config_hierarchy.md](../feature/001_config_hierarchy.md) | Feature this trait is part of |

### Formats

| File | Relationship |
|------|--------------|
| [format/001_config_file_format.md](../format/001_config_file_format.md) | Files at these paths use this format |

### Invariants

| File | Relationship |
|------|--------------|
| [invariant/001_resolution_hierarchy.md](../invariant/001_resolution_hierarchy.md) | Path formulas defined by this trait govern the invariant |
