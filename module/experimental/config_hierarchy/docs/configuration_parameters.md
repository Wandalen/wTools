# Configuration Parameters

Complete reference for all configurable parameters in the `ConfigPaths` trait.

## Required Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `app_name()` | `&'static str` | Application identifier. Must be non-empty, no path separators (`/`, `\`) or parent references (`..`). |

## Optional Parameters

All optional parameters have sensible defaults. Override only if needed.

### Local Path Configuration

| Parameter | Default | Description | Example |
|-----------|---------|-------------|---------|
| `local_permanent_prefix()` | `"."` | Prefix for permanent local configs | `.myapp/` |
| `local_temporary_prefix()` | `"-"` | Prefix for temporary local configs | `-myapp/` |
| `local_config_filename()` | `"config.yaml"` | Local config filename | `settings.toml` |

### Global Path Configuration

| Parameter | Default | Description | Example |
|-----------|---------|-------------|---------|
| `global_persistent_dir()` | `".persistent"` | Subdirectory in global config | `.storage` |
| `global_config_filename()` | `"config.yaml"` | Global config filename | `app.yaml` |
| `pro_env_var()` | `"PRO"` | Environment variable for workspace root | `WORKSPACE` |
| `home_env_var()` | `"HOME"` | Environment variable for home directory | `USERPROFILE` |
| `xdg_config_home_var()` | `"XDG_CONFIG_HOME"` | Linux XDG Base Directory variable | `XDG_CONFIG` |
| `appdata_var()` | `"APPDATA"` | Windows application data variable | `LOCALAPPDATA` |

### Environment Variable Configuration

| Parameter | Default | Description | Example |
|-----------|---------|-------------|---------|
| `env_var_prefix()` | `app_name().to_uppercase()` | Prefix for env vars | `MYAPP` |
| `env_var_separator()` | `"_"` | Separator in env var names | `__` |
| `env_var_casing()` | `EnvVarCasing::UpperCase` | Casing strategy | `LowerCase` |

### OS-Specific Fallback Paths

| Parameter | Default | Description |
|-----------|---------|-------------|
| `linux_config_base()` | `".config"` | Linux config directory relative to $HOME |
| `macos_config_base()` | `"Library/Application Support"` | macOS config directory relative to $HOME |

## Usage Example

```rust
use config_hierarchy::{ ConfigPaths, EnvVarCasing };

struct MyApp;

impl ConfigPaths for MyApp
{
  fn app_name() -> &'static str { "myapp" }

  // Customize only what you need
  fn local_config_filename() -> &'static str { "settings.toml" }
  fn env_var_prefix() -> &'static str { "MY_PREFIX" }
  fn env_var_separator() -> &'static str { "__" }
}
```

## Path Resolution Examples

With default settings for `app_name() = "myapp"`:

**Local permanent**: `.myapp/config.yaml`
**Local temporary**: `-myapp/config.yaml`
**Global**: `$PRO/.persistent/myapp/config.yaml` or `$HOME/.config/myapp/config.yaml`
**Environment**: `MYAPP_PARAM_NAME`

With custom settings:

```rust
fn local_permanent_prefix() -> &'static str { "_" }
fn local_config_filename() -> &'static str { "app.toml" }
```

**Local permanent**: `_myapp/app.toml`
