# config_hierarchy — Manual Testing Guide

Manual testing procedures for the config_hierarchy configuration management library.

## Overview

Since config_hierarchy is a library (not a CLI tool), manual testing focuses on:
- 6-level priority resolution (Runtime → Environment → LocalCurrent → LocalParent → Global → Default)
- Source tracking accuracy via `ConfigSource` enum
- Configuration file format support (YAML only)
- Multi-tool configuration isolation

## Prerequisites

### 1. Build config_hierarchy

```bash
cd "$(git rev-parse --show-toplevel)/module/experimental/config_hierarchy"
cargo build --all-features
```

### 2. Create Test Environment

```bash
mkdir -p /tmp/config-test
cd /tmp/config-test

# Set up directory hierarchy
mkdir -p project/subdir/nested

# Global config (uses dot prefix: .testapp from local_permanent_prefix default ".")
mkdir -p global-config/.persistent/.testapp
cat > global-config/.persistent/.testapp/config.yaml <<'EOF'
parameters:
  timeout: 60
  log_level: "info"
EOF

# Parent config
mkdir -p project/.testapp
cat > project/.testapp/config.yaml <<'EOF'
parameters:
  timeout: 30
  database: "production"
EOF

# Local config
mkdir -p project/subdir/.testapp
cat > project/subdir/.testapp/config.yaml <<'EOF'
parameters:
  timeout: 10
  api_key: "local-key"
EOF
```

### 3. Create Test Application

```bash
cd /tmp/config-test

cat > Cargo.toml <<'EOF'
[package]
name = "config_test"
version = "0.1.0"
edition = "2021"

[dependencies]
config_hierarchy = { path = "<WORKSPACE>/module/experimental/config_hierarchy", features = ["full"] }
serde_json = "1.0"
EOF

mkdir src
```

### 4. Shared Trait Implementations

All tests below use these implementations. Save to `src/common.rs`:

```rust
use config_hierarchy::{ ConfigDefaults, ConfigPaths, ConfigValidator, ValidationError, ConfigSource };
use std::collections::HashMap;
use serde_json::Value as JsonValue;

pub struct TestDefaults;
impl ConfigDefaults for TestDefaults
{
  fn get_defaults() -> HashMap< String, JsonValue >
  {
    let mut map = HashMap::new();
    map.insert( "timeout".into(), serde_json::json!( 120 ) );
    map.insert( "retries".into(), serde_json::json!( 3 ) );
    map.insert( "enabled".into(), serde_json::json!( true ) );
    map
  }

  fn get_parameter_names() -> Vec< &'static str >
  {
    vec![ "timeout", "retries", "enabled" ]
  }
}

pub struct TestPaths;
impl ConfigPaths for TestPaths
{
  fn app_name() -> &'static str { "testapp" }
}

pub struct TestValidator;
impl ConfigValidator for TestValidator
{
  fn validate_parameter( _: &str, _: &JsonValue ) -> Result< (), ValidationError > { Ok( () ) }
  fn validate_all( _: &HashMap< String, ( JsonValue, ConfigSource ) > ) -> Vec< ValidationError >
  {
    Vec::new()
  }
}

pub type TestConfig = config_hierarchy::ConfigManager< TestDefaults, TestPaths, TestValidator >;
```

---

## Test Scenarios

### Test 1: Default Configuration

**Objective**: Verify default values work when no config files exist.

```rust
use config_hierarchy::ConfigSource;
use std::collections::HashMap;
use serde_json::json;

fn main()
{
  println!( "Test 1: Default Configuration\n" );

  let config = TestConfig::resolve_all_config( &HashMap::new() );

  for ( key, ( value, source ) ) in &config
  {
    println!( "{}: {:?} (source: {:?})", key, value, source );
  }

  assert_eq!( config.get( "timeout" ).unwrap().0, json!( 120 ) );
  assert_eq!( config.get( "retries" ).unwrap().0, json!( 3 ) );
  assert_eq!( config.get( "enabled" ).unwrap().0, json!( true ) );

  for ( _, ( _, source ) ) in &config
  {
    assert!( matches!( source, ConfigSource::Default ) );
  }

  println!( "\nTest 1 passed!" );
}
```

**Expected Results**:
- All default values returned
- Source is `ConfigSource::Default` for all parameters
- No errors when no config files are present

---

### Test 2: Runtime Parameter Override

**Objective**: Verify runtime parameters have highest priority.

```rust
fn main()
{
  println!( "Test 2: Runtime Override\n" );

  let mut runtime_params = HashMap::new();
  runtime_params.insert( "timeout".to_string(), "5".to_string() );

  let config = TestConfig::resolve_all_config( &runtime_params );

  let ( timeout_val, timeout_src ) = config.get( "timeout" ).unwrap();
  assert!( matches!( timeout_src, ConfigSource::Runtime ) );
  println!( "timeout: {} (Runtime override)", timeout_val );

  let ( retries_val, retries_src ) = config.get( "retries" ).unwrap();
  assert!( matches!( retries_src, ConfigSource::Default ) );
  println!( "retries: {} (Default)", retries_val );

  println!( "\nTest 2 passed!" );
}
```

**Expected Results**:
- Runtime params override all other sources
- Source is `ConfigSource::Runtime` for overridden values
- Non-overridden values remain `ConfigSource::Default`

---

### Test 3: Environment Variable Configuration

**Objective**: Verify environment variables are recognized and have correct priority.

```rust
use std::env;

fn main()
{
  println!( "Test 3: Environment Variables\n" );

  env::set_var( "TESTAPP_TIMEOUT", "25" );
  env::set_var( "TESTAPP_RETRIES", "5" );

  let config = TestConfig::resolve_all_config( &HashMap::new() );

  assert_eq!( config.get( "timeout" ).unwrap().0, json!( 25 ) );
  assert!( matches!( config.get( "timeout" ).unwrap().1, ConfigSource::Environment ) );
  println!( "TESTAPP_TIMEOUT=25 applied" );

  env::remove_var( "TESTAPP_TIMEOUT" );
  env::remove_var( "TESTAPP_RETRIES" );

  println!( "\nTest 3 passed!" );
}
```

**Expected Results**:
- Environment variables parsed and typed
- `TESTAPP_PARAM` format recognized
- Source is `ConfigSource::Environment`

---

### Test 4: Local Configuration File

**Objective**: Verify local config file loading from current directory.

**Setup**:
```bash
cd /tmp/config-test/project/subdir
mkdir -p .testapp
cat > .testapp/config.yaml <<'EOF'
parameters:
  api_key: "local-key"
  timeout: 10
EOF
```

```rust
fn main()
{
  println!( "Test 4: Local Config File\n" );

  let config = TestConfig::resolve_all_config( &HashMap::new() );

  if let Some( ( api_key, source ) ) = config.get( "api_key" )
  {
    assert_eq!( *api_key, json!( "local-key" ) );
    assert!( matches!( source, ConfigSource::LocalCurrent( _ ) ) );
    println!( "Local config loaded: api_key = {}", api_key );
  }

  println!( "\nTest 4 passed!" );
}
```

**Expected Results**:
- Loads `.testapp/config.yaml` from current directory
- Source is `ConfigSource::LocalCurrent(path)`

---

### Test 5: Parent Configuration Inheritance

**Objective**: Verify configuration from parent directories (nearest first).

**Setup**:
```bash
cd /tmp/config-test/project/subdir/nested
```

```rust
fn main()
{
  println!( "Test 5: Parent Config\n" );

  let config = TestConfig::resolve_all_config( &HashMap::new() );

  if let Some( ( database, source ) ) = config.get( "database" )
  {
    assert_eq!( *database, json!( "production" ) );
    assert!( matches!( source, ConfigSource::LocalParent( _ ) ) );
    println!( "Parent config: database = {}", database );
  }

  println!( "\nTest 5 passed!" );
}
```

**Expected Results**:
- Parent directory config values available
- Source is `ConfigSource::LocalParent(path)`
- Nearest parent takes precedence

---

### Test 6: Global Configuration

**Objective**: Verify global config from `$PRO/.persistent/.{app_name}/`.

**Setup**:
```bash
export PRO=/tmp/config-test/global-config
cd /tmp  # no local configs here
```

```rust
fn main()
{
  println!( "Test 6: Global Config\n" );

  std::env::set_current_dir( "/tmp" ).unwrap();

  let config = TestConfig::resolve_all_config( &HashMap::new() );

  if let Some( ( log_level, source ) ) = config.get( "log_level" )
  {
    assert_eq!( *log_level, json!( "info" ) );
    assert!( matches!( source, ConfigSource::Global( _ ) ) );
    println!( "Global config: log_level = {}", log_level );
  }

  println!( "\nTest 6 passed!" );
}
```

**Expected Results**:
- Global config loaded from `$PRO/.persistent/.testapp/config.yaml`
- Note the dot prefix on `.testapp` — `local_permanent_prefix` default is `"."`
- Source is `ConfigSource::Global(path)`

---

### Test 7: Priority Resolution Order

**Objective**: Verify complete 6-level priority cascade.

```rust
fn main()
{
  println!( "Test 7: Priority Resolution\n" );

  std::env::set_var( "PRO", "/tmp/config-test/global-config" );
  std::env::set_var( "TESTAPP_TIMEOUT", "45" );
  std::env::set_current_dir( "/tmp/config-test/project/subdir" ).unwrap();

  std::fs::create_dir_all( ".testapp" ).unwrap();
  std::fs::write( ".testapp/config.yaml", "parameters:\n  timeout: 10\n" ).unwrap();

  let mut runtime = HashMap::new();
  runtime.insert( "timeout".to_string(), "5".to_string() );

  let config = TestConfig::resolve_all_config( &runtime );
  let ( timeout, source ) = config.get( "timeout" ).unwrap();
  assert_eq!( *timeout, json!( 5 ) );
  assert!( matches!( source, ConfigSource::Runtime ) );
  println!( "Runtime (5) wins over all" );

  let config = TestConfig::resolve_all_config( &HashMap::new() );
  let ( timeout, source ) = config.get( "timeout" ).unwrap();
  assert_eq!( *timeout, json!( 45 ) );
  assert!( matches!( source, ConfigSource::Environment ) );
  println!( "Environment (45) wins over Local (10)" );

  std::env::remove_var( "TESTAPP_TIMEOUT" );

  let config = TestConfig::resolve_all_config( &HashMap::new() );
  let ( timeout, source ) = config.get( "timeout" ).unwrap();
  assert_eq!( *timeout, json!( 10 ) );
  assert!( matches!( source, ConfigSource::LocalCurrent( _ ) ) );
  println!( "LocalCurrent (10) wins over Default (120)" );

  println!( "\nTest 7 passed — priority order correct!" );
}
```

**Expected Results**:
- Runtime > Environment > LocalCurrent > LocalParent > Global > Default
- Source tracking accurate at each level

---

### Test 8: Source Tracking Accuracy

**Objective**: Verify source tracking correctly identifies mixed sources.

```rust
fn main()
{
  println!( "Test 8: Source Tracking\n" );

  std::env::set_var( "TESTAPP_RETRIES", "7" );
  let mut runtime = HashMap::new();
  runtime.insert( "timeout".to_string(), "15".to_string() );

  let config = TestConfig::resolve_all_config( &runtime );

  println!( "Configuration sources:" );
  for ( key, ( value, source ) ) in &config
  {
    println!( "  {}: {:?} from {:?}", key, value, source );
  }

  assert!( matches!( config.get( "timeout" ).unwrap().1, ConfigSource::Runtime ) );
  assert!( matches!( config.get( "retries" ).unwrap().1, ConfigSource::Environment ) );
  assert!( matches!( config.get( "enabled" ).unwrap().1, ConfigSource::Default ) );

  std::env::remove_var( "TESTAPP_RETRIES" );

  println!( "\nTest 8 passed — source tracking accurate!" );
}
```

**Expected Results**:
- Source accurately tracked for each parameter
- Mix of `Runtime`, `Environment`, and `Default` sources represented

---

### Test 9: Multi-Tool Configuration Isolation

**Objective**: Verify multiple tools with different app names do not interfere.

**Setup**:
```bash
cd /tmp/config-test
mkdir -p .tool1 .tool2
cat > .tool1/config.yaml <<'EOF'
parameters:
  tool1_param: "value1"
  shared: "from-tool1"
EOF
cat > .tool2/config.yaml <<'EOF'
parameters:
  tool2_param: "value2"
  shared: "from-tool2"
EOF
```

Implement `Tool1Paths` and `Tool2Paths` each with a different `app_name()`, then run `resolve_all_config` independently.

**Expected Results**:
- Each tool's `ConfigManager` sees only its own config files
- Tools do not interfere with each other
- `shared` parameter resolves independently per tool

---

### Test 10: Dual-Pattern Priority (Temporary vs Permanent)

**Objective**: Verify `-{app}` (temporary) overrides `.{app}` (permanent) within the same directory.

**Setup**:
```bash
cd /tmp/config-test
mkdir -p .testapp -testapp
cat > .testapp/config.yaml <<'EOF'
parameters:
  source_check: "permanent"
  timeout: 100
EOF
cat > -testapp/config.yaml <<'EOF'
parameters:
  source_check: "temporary"
  timeout: 50
EOF
```

```rust
fn main()
{
  println!( "Test 10: Dual-Pattern Priority\n" );

  let config = TestConfig::resolve_all_config( &HashMap::new() );

  let ( source_check, src ) = config.get( "source_check" ).unwrap();
  assert_eq!( *source_check, json!( "temporary" ) );
  assert!( matches!( src, ConfigSource::LocalCurrent( _ ) ) );
  println!( "Temporary (-testapp) correctly overrides permanent (.testapp)" );

  println!( "\nTest 10 passed!" );
}
```

**Expected Results**:
- `-testapp/config.yaml` takes priority over `.testapp/config.yaml` in same directory
- Both are `ConfigSource::LocalCurrent`; temporary wins within same directory level

---

### Test 11: Validation Integration

**Objective**: Verify custom validator rejects invalid values.

```rust
use config_hierarchy::{ ConfigValidator, ValidationError, ConfigSource };
use serde_json::Value as JsonValue;
use std::collections::HashMap;

struct StrictValidator;
impl ConfigValidator for StrictValidator
{
  fn validate_parameter( param_name : &str, value : &JsonValue )
    -> Result< (), ValidationError >
  {
    if param_name == "timeout"
    {
      if let Some( num ) = value.as_u64()
      {
        if num > 300
        {
          return Err( ValidationError::new( param_name, "must not exceed 300" ) );
        }
      }
    }
    Ok( () )
  }

  fn validate_all( config : &HashMap< String, ( JsonValue, ConfigSource ) > )
    -> Vec< ValidationError >
  {
    let mut errors = Vec::new();
    if let Some( ( retries, _ ) ) = config.get( "retries" )
    {
      if retries.as_u64().unwrap_or( 0 ) > 10
      {
        errors.push( ValidationError::new( "retries", "must not exceed 10" ) );
      }
    }
    errors
  }
}

type ValidatedConfig = config_hierarchy::ConfigManager< TestDefaults, TestPaths, StrictValidator >;

fn main()
{
  println!( "Test 11: Validation\n" );

  let mut runtime = HashMap::new();
  runtime.insert( "timeout".to_string(), "500".to_string() );

  let config = ValidatedConfig::resolve_all_config( &runtime );

  // validate_parameter must be called explicitly — there is no resolve_all_config_validated()
  for ( key, ( value, _ ) ) in &config
  {
    if let Err( e ) = StrictValidator::validate_parameter( key, value )
    {
      println!( "Validation error: {:?}", e );
    }
  }

  let errors = StrictValidator::validate_all( &config );
  println!( "Cross-parameter errors: {:?}", errors );

  println!( "\nTest 11 passed!" );
}
```

**Expected Results**:
- `validate_parameter` returns error for `timeout = 500`
- `validate_all` catches cross-parameter violations
- **Note**: Validators are called explicitly; `ConfigManager` has no `resolve_all_config_validated()` method

---

## Verification Checklist

**Priority Resolution**:
- [ ] Default values work as lowest-priority fallback
- [ ] Runtime params have highest priority
- [ ] Environment variables (`APPNAME_PARAM`) recognized
- [ ] Local config files load from CWD (`.testapp/config.yaml`)
- [ ] Temporary local pattern (`-testapp/config.yaml`) has higher priority than permanent in same directory
- [ ] Parent directory traversal works; nearest parent wins
- [ ] Global config from `$PRO/.persistent/.testapp/config.yaml` works (note dot prefix on `.testapp`)
- [ ] Complete cascade: Runtime → Env → LocalCurrent → LocalParent → Global → Default

**Source Tracking**:
- [ ] `ConfigSource::Runtime` for runtime params
- [ ] `ConfigSource::Environment` for env vars
- [ ] `ConfigSource::LocalCurrent(PathBuf)` for CWD files
- [ ] `ConfigSource::LocalParent(PathBuf)` for parent directory files
- [ ] `ConfigSource::Global(PathBuf)` for global config
- [ ] `ConfigSource::Default` for fallback values

**File Operations** (requires `file_ops` feature):
- [ ] YAML files parsed correctly
- [ ] `metadata` section managed automatically
- [ ] `atomic_config_modify()` works under concurrent access
- [ ] `created_at` preserved across updates; `last_modified` updated

**Validation**:
- [ ] `validate_parameter` called per resolved value
- [ ] `validate_all` called with complete config map
- [ ] Errors collected and reported

## Known Limitations

1. **YAML only** — config files must be YAML; JSON and TOML are not supported as file formats
2. **`$PRO` required** — global config requires the `PRO` environment variable set to workspace root
3. **Static paths** — `ConfigPaths` methods return `&'static str`; path customization must be known at compile time
4. **`env_var_prefix()` memory leak** — the default implementation leaks one allocation per call via `Box::leak()`; override with a static literal if called frequently
5. **No auto-validation** — `ConfigManager` has no `resolve_all_config_validated()` method; call `validate_parameter` and `validate_all` explicitly

## Reporting Issues

When reporting issues, include:

1. All config files present and their paths
2. Environment variables set (especially `PRO`, `APPNAME_*`)
3. Runtime parameters provided
4. Expected vs actual resolved configuration
5. Source tracking output (`{:?}` on `ConfigSource`)
6. Directory structure: `tree -a -L 3`

## References

- Main readme: `../readme.md`
- Feature specification: `../docs/feature/001_config_hierarchy.md`
- Resolution invariant: `../docs/invariant/001_resolution_hierarchy.md`
- API documentation: `cargo doc --open`
- Automated tests: `../tests/*.rs`
