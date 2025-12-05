# config_hierarchy

Generic hierarchical configuration management with 6-level priority system and automatic source tracking.

## Scope

**Responsibility:**
- Generic hierarchical configuration resolution framework
- 6-level priority system (Runtime → Environment → Local → Parent → Global → Default)
- Source tracking for debugging configuration origins
- Reusable configuration management for multiple CLI tools

**In Scope:**
- Runtime/Environment/Local/Parent/Global/Default config layers
- YAML/JSON/TOML parsing
- Secret management and config merging
- Source tracking for every value
- ConfigManager trait-based architecture
- Auto-migration from old config paths
- Multiple output formatters (table, JSON, YAML)

**Out of Scope:**
- ❌ Git operations → delegated to `git_tools` crate
- ❌ Workspace detection → delegated to `cargo_tools` crate
- ❌ Application-specific configuration logic → implemented by consuming crates
- ❌ CLI argument parsing → handled by calling binary

## Features

```toml
[dependencies]
config_hierarchy = { version = "0.1", features = ["full"] }
```

| Feature | Description |
|---------|-------------|
| `default` | Core resolution only |
| `file_ops` | File I/O, YAML support |
| `migration` | Auto-migration from old paths |
| `display_table` | Table formatter |
| `display_json` | JSON formatter |
| `display_yaml` | YAML formatter |
| `full` | All features |

## Quick Start

```rust
use config_hierarchy::{ ConfigManager, ConfigDefaults, ConfigPaths, ConfigValidator };
use std::collections::HashMap;
use serde_json::Value as JsonValue;

struct AppDefaults;
impl ConfigDefaults for AppDefaults {
  fn get_defaults() -> HashMap<String, JsonValue> {
    let mut map = HashMap::new();
    map.insert("timeout".into(), JsonValue::Number(30.into()));
    map
  }
  fn get_parameter_names() -> Vec<&'static str> { vec!["timeout"] }
}

struct AppPaths;
impl ConfigPaths for AppPaths {
  fn app_name() -> &'static str { "myapp" }
}

struct AppValidator;
impl ConfigValidator for AppValidator {
  fn validate_parameter(_: &str, _: &JsonValue)
    -> Result<(), config_hierarchy::ValidationError> { Ok(()) }
  fn validate_all(_: &HashMap<String, (JsonValue, config_hierarchy::ConfigSource)>)
    -> Vec<config_hierarchy::ValidationError> { Vec::new() }
}

type AppConfig = ConfigManager<AppDefaults, AppPaths, AppValidator>;

fn main() {
  let config = AppConfig::resolve_all_config(&HashMap::new());
  for (key, (value, source)) in &config {
    println!("{key}: {value:?} (from {source:?})");
  }
}
```

## Resolution Hierarchy

Configuration resolves in priority order:

1. **Runtime** - Explicit parameters at execution
2. **Environment** - `MYAPP_TIMEOUT` style vars
3. **Local (Current)** - `-./.myapp/` (temp) or `./.myapp/` (perm)
4. **Local (Parents)** - Ancestor directories (nearest first)
5. **Global** - `$PRO/.persistent/.myapp/`
6. **Defaults** - Application-defined

## Path Standards

| Source | Pattern | Example |
|--------|---------|---------|
| Local (Temp) | `-./.{name}/config.yaml` | `-./.myapp/config.yaml` |
| Local (Perm) | `./.{name}/config.yaml` | `./.myapp/config.yaml` |
| Global | `$PRO/.persistent/.{name}/config.yaml` | - |

## Testing

```bash
cargo test -p config_hierarchy
```

## License

MIT
