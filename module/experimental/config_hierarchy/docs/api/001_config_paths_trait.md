# API: ConfigPaths Trait

### Scope

- **Purpose**: Define the contract for supplying path and naming configuration to ConfigManager.
- **Responsibility**: Document all path derivation methods, defaults, env var naming, and OS-specific fallbacks.
- **In Scope**: All 15 ConfigPaths methods; path derivation formulas; env var naming; only app_name() is required.
- **Out of Scope**: Default values (→ api/002), validation (→ api/003), file format (→ format/001).

### Abstract

`ConfigPaths` is one of the three traits users implement to configure `ConfigManager< D, P, V >`. It controls all path derivation — where config files live, what they are named, how environment variables are formatted, and which OS-specific directories to use as fallbacks. Only `app_name()` is required; all other 14 methods have sensible defaults derived from the app name.

### Operations

#### Path Derivation Table

With default settings and `app_name() = "myapp"`:

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
| `app_name()` | yes | — | Application identifier; must be non-empty, no `/`, `\`, or `..` |
| `env_var_prefix()` | no | `app_name().to_uppercase()` | Prefix for all environment variables |
| `env_var_separator()` | no | `"_"` | Character(s) between prefix and parameter name |
| `env_var_casing()` | no | `EnvVarCasing::UpperCase` | Casing strategy for env var names |
| `local_permanent_prefix()` | no | `"."` | Prefix for permanent local config directories |
| `local_temporary_prefix()` | no | `"-"` | Prefix for temporary local config directories |
| `local_config_filename()` | no | `"config.yaml"` | Filename for all local config files |
| `global_persistent_dir()` | no | `".persistent"` | Subdirectory under `$PRO` for global configs |
| `global_config_filename()` | no | `"config.yaml"` | Filename for global config file |
| `pro_env_var()` | no | `"PRO"` | Name of env var pointing to workspace root |
| `home_env_var()` | no | `"HOME"` | Name of env var pointing to home directory |
| `xdg_config_home_var()` | no | `"XDG_CONFIG_HOME"` | Linux XDG Base Directory variable name |
| `appdata_var()` | no | `"APPDATA"` | Windows application data directory variable |
| `linux_config_base()` | no | `".config"` | Linux config base relative to `$HOME` |
| `macos_config_base()` | no | `"Library/Application Support"` | macOS config base relative to `$HOME` |

### Error Handling

Path discovery functions return `Err(String)` when `app_name()` fails validation:
- Empty string
- Contains `/` or `\`
- Contains `..`

`discover_local_configs()` silently skips invalid app names to avoid breaking the discovery loop. No other methods on `ConfigPaths` are validated at call time.

### Compatibility Guarantees

- Adding new optional methods with default implementations is a non-breaking change
- Changing `app_name()` in an existing implementation changes all derived paths — existing config files at the old paths become unreachable
- Changing any optional method changes only the paths or env var names derived from that method

### Pitfall

The default `env_var_prefix()` allocates a new heap string on every call and never frees it, accumulating unbounded memory. Override `env_var_prefix()` with a static string literal when called in a loop or performance-sensitive path.

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
