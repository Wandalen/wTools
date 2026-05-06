# config_hierarchy
<!--{ generate.module_header.start() }-->
 [![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://img.shields.io/github/actions/workflow/status/Wandalen/wTools/workspace_push.yml?label=&branch=master&job=config_hierarchy)](https://github.com/Wandalen/wTools/actions/workflows/workspace_push.yml) [![docs.rs](https://img.shields.io/docsrs/config_hierarchy?color=e3e8f0&logo=docs.rs)](https://docs.rs/config_hierarchy) [![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=module%2Fexperimental%2Fconfig_hierarchy%2Fexamples%2Fbasic_usage.rs,RUN_POSTFIX=--example%20basic_usage/https://github.com/Wandalen/wTools) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)
<!--{ generate.module_header.end }-->

A trait-based hierarchical configuration management library for Rust applications, featuring a 6-level priority resolution system, automatic source tracking, and cross-platform file operations with atomic writes.

## Overview

`config_hierarchy` eliminates configuration ambiguity by tracking the exact source of every configuration value. Users implement three traits and compose a zero-cost `ConfigManager< D, P, V >` type.

**Key Capabilities:**
- **Priority-based resolution** across 6 configuration layers (Runtime → Env → LocalCurrent → LocalParent → Global → Default)
- **Source tracking** for every configuration value via `ConfigSource` enum
- **Atomic file operations** with cross-platform file locking (`fs2`)
- **Type detection** with automatic string-to-type conversion
- **Trait-based architecture** for application-specific customization

## Installation

Add `config_hierarchy` to your project's `Cargo.toml`:

```toml
[dependencies]
config_hierarchy = { version = "0.5", features = ["full"] }
```

### Feature Flags

| Feature | Description |
|---------|-------------|
| `enabled` | Core resolution engine — hierarchy, source tracking, type detection |
| `file_ops` | File persistence — YAML I/O, atomic writes, file locking, path discovery |
| `display_table` | Table output formatter |
| `display_json` | JSON output formatter |
| `display_yaml` | YAML output formatter |
| `full` | All features enabled |

## Quick Start

Implement three traits and compose a type alias:

```rust
use config_hierarchy::{ ConfigManager, ConfigDefaults, ConfigPaths, ConfigValidator };
use std::collections::HashMap;
use serde_json::Value as JsonValue;

struct AppDefaults;
impl ConfigDefaults for AppDefaults
{
  fn get_defaults() -> HashMap< String, JsonValue >
  {
    let mut map = HashMap::new();
    map.insert( "timeout".into(), JsonValue::Number( 30.into() ) );
    map.insert( "retries".into(), JsonValue::Number( 3.into() ) );
    map
  }

  fn get_parameter_names() -> Vec< &'static str >
  {
    vec![ "timeout", "retries" ]
  }
}

struct AppPaths;
impl ConfigPaths for AppPaths
{
  fn app_name() -> &'static str { "myapp" }
}

struct AppValidator;
impl ConfigValidator for AppValidator
{
  fn validate_parameter( _name : &str, _value : &JsonValue )
    -> Result< (), config_hierarchy::ValidationError >
  {
    Ok( () )
  }

  fn validate_all( _config : &HashMap< String, ( JsonValue, config_hierarchy::ConfigSource ) > )
    -> Vec< config_hierarchy::ValidationError >
  {
    Vec::new()
  }
}

type AppConfig = ConfigManager< AppDefaults, AppPaths, AppValidator >;

fn main()
{
  let runtime_overrides = HashMap::new();
  let config = AppConfig::resolve_all_config( &runtime_overrides );

  for ( key, ( value, source ) ) in &config
  {
    println!( "{key}: {value:?} (from {source:?})" );
  }
}
```

**Output** (with no config files or env vars present):
```text
timeout: Number(30) (from Default)
retries: Number(3) (from Default)
```

If `MYAPP_TIMEOUT=60` is set:
```text
timeout: Number(60) (from Environment)
retries: Number(3) (from Default)
```

## Documentation

See [`docs/`](docs/) for complete specifications:

| Document | Description |
|----------|-------------|
| [Feature: Config Hierarchy](docs/feature/001_config_hierarchy.md) | All crate behaviors and responsibilities |
| [Invariant: Resolution Hierarchy](docs/invariant/001_resolution_hierarchy.md) | Priority order, path formulas, dual-pattern rule |
| [API: ConfigPaths Trait](docs/api/001_config_paths_trait.md) | All 15 path and env var configuration methods |
| [API: ConfigDefaults Trait](docs/api/002_config_defaults_trait.md) | Default values and parameter enumeration |
| [API: ConfigValidator Trait](docs/api/003_config_validator_trait.md) | Per-value and cross-parameter validation |
| [Algorithm: Type Detection](docs/algorithm/001_type_detection.md) | String-to-type conversion rules |
| [Format: Config File Format](docs/format/001_config_file_format.md) | YAML file structure and fields |

## Tasks

See [`task/`](task/) for active implementation work items.

## License

MIT
