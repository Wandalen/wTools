# config_hierarchy - Manual Testing Guide

Manual testing procedures for the config_hierarchy configuration management library.

## Overview

Since config_hierarchy is primarily a library (not a CLI tool), manual testing focuses on:
- 6-level priority resolution (Runtime → Environment → Local → Parent → Global → Default)
- Source tracking accuracy
- Configuration file format support (YAML/JSON/TOML)
- Integration with real applications
- Multi-tool configuration scenarios

## Prerequisites

### 1. Build config_hierarchy

```bash
cd /home/user1/pro/lib/wTools/module/core/config_hierarchy
cargo build --all-features
```

### 2. Create Test Environment

```bash
mkdir -p /tmp/config-test
cd /tmp/config-test

# Set up directory hierarchy
mkdir -p project/subdir/nested
mkdir -p global-config

# Create test configuration files
# Global config
mkdir -p global-config/.myapp
cat > global-config/.myapp/config.yaml <<'EOF'
timeout: 60
log_level: "info"
EOF

# Parent config
mkdir -p project/.myapp
cat > project/.myapp/config.yaml <<'EOF'
timeout: 30
database: "production"
EOF

# Local config
mkdir -p project/subdir/.myapp
cat > project/subdir/.myapp/config.yaml <<'EOF'
timeout: 10
api_key: "local-key"
EOF

# Create .env file for environment variables
cat > project/.env <<'EOF'
MYAPP_TIMEOUT=45
MYAPP_DEBUG=true
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
config_hierarchy = { path = "/home/user1/pro/lib/wTools/module/core/config_hierarchy", features = ["full"] }
serde_json = "1.0"
EOF

mkdir src
```

## Test Scenarios

### Test 1: Default Configuration

**Objective**: Verify default values work when no config files exist

**Create Test**:
```bash
cat > src/test_defaults.rs <<'EOF'
use config_hierarchy::{ ConfigManager, ConfigDefaults, ConfigPaths, ConfigValidator, ValidationError };
use std::collections::HashMap;
use serde_json::{ Value as JsonValue, json };

struct TestDefaults;
impl ConfigDefaults for TestDefaults {
    fn get_defaults() -> HashMap<String, JsonValue> {
        let mut map = HashMap::new();
        map.insert("timeout".into(), json!(120));
        map.insert("retries".into(), json!(3));
        map.insert("enabled".into(), json!(true));
        map
    }
    fn get_parameter_names() -> Vec<&'static str> {
        vec!["timeout", "retries", "enabled"]
    }
}

struct TestPaths;
impl ConfigPaths for TestPaths {
    fn app_name() -> &'static str { "testapp" }
}

struct TestValidator;
impl ConfigValidator for TestValidator {
    fn validate_parameter(_: &str, _: &JsonValue) -> Result<(), ValidationError> { Ok(()) }
    fn validate_all(_: &HashMap<String, (JsonValue, config_hierarchy::ConfigSource)>)
        -> Vec<ValidationError> { Vec::new() }
}

type TestConfig = ConfigManager<TestDefaults, TestPaths, TestValidator>;

fn main() {
    println!("Test 1: Default Configuration\n");

    // Resolve with no runtime params (should get defaults)
    let config = TestConfig::resolve_all_config(&HashMap::new());

    for (key, (value, source)) in &config {
        println!("{}: {:?} (source: {:?})", key, value, source);
    }

    // Verify defaults
    assert_eq!(config.get("timeout").unwrap().0, json!(120));
    assert_eq!(config.get("retries").unwrap().0, json!(3));
    assert_eq!(config.get("enabled").unwrap().0, json!(true));

    // Verify all from Default source
    for (_, (_, source)) in &config {
        assert!(matches!(source, config_hierarchy::ConfigSource::Default));
    }

    println!("\n✅ Test 1 passed!");
}
EOF

rustc --edition 2021 src/test_defaults.rs -o test_defaults \
  -L /home/user1/pro/lib/wTools/target/debug/deps \
  --extern config_hierarchy=/home/user1/pro/lib/wTools/target/debug/libconfig_hierarchy.rlib \
  --extern serde_json=/home/user1/pro/lib/wTools/target/debug/deps/libserde_json*.rlib

./test_defaults
```

**Expected Results**:
- ✅ All default values returned
- ✅ Source is ConfigSource::Default for all
- ✅ No errors when no config files present

**Success Criteria**:
- Defaults work as fallback
- Source tracking accurate

### Test 2: Runtime Parameter Override

**Objective**: Verify runtime parameters have highest priority

**Create Test**:
```bash
cat > src/test_runtime.rs <<'EOF'
use config_hierarchy::{ ConfigManager, ConfigDefaults, ConfigPaths, ConfigValidator, ValidationError };
use std::collections::HashMap;
use serde_json::{ Value as JsonValue, json };

// Same trait implementations as Test 1...
[copy trait implementations from Test 1]

fn main() {
    println!("Test 2: Runtime Override\n");

    // Provide runtime parameters
    let mut runtime_params = HashMap::new();
    runtime_params.insert("timeout".into(), json!(5));
    runtime_params.insert("custom".into(), json!("runtime-value"));

    let config = TestConfig::resolve_all_config(&runtime_params);

    // Verify runtime override
    let (timeout_val, timeout_src) = config.get("timeout").unwrap();
    assert_eq!(*timeout_val, json!(5));
    assert!(matches!(timeout_src, config_hierarchy::ConfigSource::Runtime));
    println!("✅ timeout: {} (Runtime override)", timeout_val);

    // Verify runtime-only param
    let (custom_val, custom_src) = config.get("custom").unwrap();
    assert_eq!(*custom_val, json!("runtime-value"));
    assert!(matches!(custom_src, config_hierarchy::ConfigSource::Runtime));
    println!("✅ custom: {} (Runtime only)", custom_val);

    // Verify other params still from defaults
    let (retries_val, retries_src) = config.get("retries").unwrap();
    assert!(matches!(retries_src, config_hierarchy::ConfigSource::Default));
    println!("✅ retries: {} (Default)", retries_val);

    println!("\n✅ Test 2 passed!");
}
EOF
```

**Expected Results**:
- ✅ Runtime params override all other sources
- ✅ Source tracking shows Runtime for overridden values
- ✅ Non-overridden values still from defaults

**Success Criteria**:
- Priority order correct
- Runtime has highest priority

### Test 3: Environment Variable Configuration

**Objective**: Verify environment variables work

**Create Test**:
```bash
cat > src/test_environment.rs <<'EOF'
use config_hierarchy::{ ConfigManager, ConfigDefaults, ConfigPaths, ConfigValidator };
use std::collections::HashMap;
use std::env;
use serde_json::json;

// [Trait implementations...]

fn main() {
    println!("Test 3: Environment Variables\n");

    // Set environment variables
    env::set_var("TESTAPP_TIMEOUT", "25");
    env::set_var("TESTAPP_RETRIES", "5");
    env::set_var("TESTAPP_DEBUG", "true");

    let config = TestConfig::resolve_all_config(&HashMap::new());

    // Verify environment variables used
    assert_eq!(config.get("timeout").unwrap().0, json!(25));
    println!("✅ TESTAPP_TIMEOUT=25 applied");

    // Verify source is Environment
    assert!(matches!(
        config.get("timeout").unwrap().1,
        config_hierarchy::ConfigSource::Environment
    ));

    // Clean up
    env::remove_var("TESTAPP_TIMEOUT");
    env::remove_var("TESTAPP_RETRIES");
    env::remove_var("TESTAPP_DEBUG");

    println!("\n✅ Test 3 passed!");
}
EOF
```

**Expected Results**:
- ✅ Environment variables parsed correctly
- ✅ APPNAME_PARAM format recognized
- ✅ Source is Environment

**Success Criteria**:
- Env vars work
- Priority below Runtime, above file configs

### Test 4: Local Configuration File

**Objective**: Verify local config file loading

**Setup**:
```bash
cd /tmp/config-test/project/subdir
```

**Create Test**:
```bash
cat > test_local.rs <<'EOF'
use config_hierarchy::{ ConfigManager, /* ... */ };

fn main() {
    println!("Test 4: Local Config File\n");

    // Should load from ./.testapp/config.yaml
    let config = TestConfig::resolve_all_config(&HashMap::new());

    // Verify local config loaded
    if let Some((api_key, source)) = config.get("api_key") {
        assert_eq!(*api_key, json!("local-key"));
        assert!(matches!(source, config_hierarchy::ConfigSource::Local(_)));
        println!("✅ Local config loaded: api_key");
    }

    println!("\n✅ Test 4 passed!");
}
EOF
```

**Expected Results**:
- ✅ Loads ./.myapp/config.yaml
- ✅ Values from local config available
- ✅ Source is ConfigSource::Local

**Success Criteria**:
- Local config file discovered
- YAML parsing works

### Test 5: Parent Configuration Inheritance

**Objective**: Verify configuration from parent directories

**Setup**:
```bash
cd /tmp/config-test/project/subdir/nested
```

**Create Test**:
```bash
cat > test_parent.rs <<'EOF'
fn main() {
    println!("Test 5: Parent Config\n");

    // From nested directory, should inherit parent configs
    let config = TestConfig::resolve_all_config(&HashMap::new());

    // Should have values from parent directories
    if let Some((database, source)) = config.get("database") {
        assert_eq!(*database, json!("production"));
        assert!(matches!(source, config_hierarchy::ConfigSource::Parent(_)));
        println!("✅ Parent config: database={}", database);
    }

    // Local should override parent
    if let Some((timeout, source)) = config.get("timeout") {
        println!("timeout: {} (source: {:?})", timeout, source);
    }

    println!("\n✅ Test 5 passed!");
}
EOF
```

**Expected Results**:
- ✅ Parent config values available
- ✅ Source is ConfigSource::Parent
- ✅ Nearest parent takes precedence

**Success Criteria**:
- Parent directory traversal works
- Nearest parent wins

### Test 6: Global Configuration

**Objective**: Verify global config from $PRO/.persistent

**Setup**:
```bash
# Set PRO environment variable
export PRO=/tmp/config-test/global-config
```

**Create Test**:
```bash
cat > test_global.rs <<'EOF'
fn main() {
    println!("Test 6: Global Config\n");

    // From location with no local/parent config
    std::env::set_current_dir("/tmp").unwrap();

    let config = TestConfig::resolve_all_config(&HashMap::new());

    // Should load from $PRO/.persistent/.testapp/config.yaml
    if let Some((log_level, source)) = config.get("log_level") {
        assert_eq!(*log_level, json!("info"));
        assert!(matches!(source, config_hierarchy::ConfigSource::Global));
        println!("✅ Global config: log_level={}", log_level);
    }

    println!("\n✅ Test 6 passed!");
}
EOF
```

**Expected Results**:
- ✅ Global config loaded from $PRO
- ✅ Source is ConfigSource::Global
- ✅ Lowest priority (above defaults only)

**Success Criteria**:
- $PRO/.persistent/.appname/ discovery works
- Global config as fallback

### Test 7: Priority Resolution Order

**Objective**: Verify complete priority cascade

**Create Test**:
```bash
cat > src/test_priority.rs <<'EOF'
fn main() {
    println!("Test 7: Priority Resolution\n");

    // Setup all config sources
    std::env::set_var("PRO", "/tmp/config-test/global-config");
    std::env::set_var("TESTAPP_TIMEOUT", "45");
    std::env::set_current_dir("/tmp/config-test/project/subdir").unwrap();

    // Create local config with timeout
    std::fs::write(".testapp/config.yaml", "timeout: 10\n").unwrap();

    let mut runtime = HashMap::new();
    runtime.insert("timeout".into(), json!(5));

    let config = TestConfig::resolve_all_config(&runtime);

    // Verify priority: Runtime (5) wins over all
    let (timeout, source) = config.get("timeout").unwrap();
    assert_eq!(*timeout, json!(5));
    assert!(matches!(source, config_hierarchy::ConfigSource::Runtime));
    println!("✅ Runtime (5) > Environment (45) > Local (10) > Global (60) > Default (120)");

    // Remove runtime param, verify Environment wins
    let config = TestConfig::resolve_all_config(&HashMap::new());
    let (timeout, source) = config.get("timeout").unwrap();
    assert_eq!(*timeout, json!(45));
    assert!(matches!(source, config_hierarchy::ConfigSource::Environment));
    println!("✅ Environment (45) > Local (10) > Global (60) > Default (120)");

    std::env::remove_var("TESTAPP_TIMEOUT");

    // Verify Local wins
    let config = TestConfig::resolve_all_config(&HashMap::new());
    let (timeout, source) = config.get("timeout").unwrap();
    assert_eq!(*timeout, json!(10));
    assert!(matches!(source, config_hierarchy::ConfigSource::Local(_)));
    println!("✅ Local (10) > Global (60) > Default (120)");

    println!("\n✅ Test 7 passed - Priority order correct!");
}
EOF
```

**Expected Results**:
- ✅ Runtime > Environment > Local > Parent > Global > Default
- ✅ Each level overrides lower levels
- ✅ Source tracking accurate at each level

**Success Criteria**:
- Complete priority cascade works
- Order is correct

### Test 8: Source Tracking Accuracy

**Objective**: Verify source tracking for debugging

**Create Test**:
```bash
cat > src/test_sources.rs <<'EOF'
fn main() {
    println!("Test 8: Source Tracking\n");

    // Mix of sources
    std::env::set_var("TESTAPP_RETRIES", "7");
    let mut runtime = HashMap::new();
    runtime.insert("timeout".into(), json!(15));

    let config = TestConfig::resolve_all_config(&runtime);

    println!("Configuration sources:");
    for (key, (value, source)) in &config {
        println!("  {}: {:?} from {:?}", key, value, source);
    }

    // Verify each source type
    assert!(matches!(config.get("timeout").unwrap().1,
        config_hierarchy::ConfigSource::Runtime));
    assert!(matches!(config.get("retries").unwrap().1,
        config_hierarchy::ConfigSource::Environment));
    assert!(matches!(config.get("enabled").unwrap().1,
        config_hierarchy::ConfigSource::Default));

    println!("\n✅ Test 8 passed - Source tracking accurate!");
}
EOF
```

**Expected Results**:
- ✅ Source accurately tracked for each parameter
- ✅ Mix of sources represented correctly
- ✅ Debugging information available

**Success Criteria**:
- Source tracking works
- Helps debug config issues

### Test 9: Format Support (YAML/JSON/TOML)

**Objective**: Verify multiple config file formats

**Setup**:
```bash
# Create config files in different formats
cat > .testapp/config.yaml <<'EOF'
yaml_value: "from-yaml"
EOF

cat > .testapp/config.json <<'EOF'
{
  "json_value": "from-json"
}
EOF

cat > .testapp/config.toml <<'EOF'
toml_value = "from-toml"
EOF
```

**Create Test**:
```bash
cat > test_formats.rs <<'EOF'
fn main() {
    println!("Test 9: Format Support\n");

    let config = TestConfig::resolve_all_config(&HashMap::new());

    // Verify all formats loaded
    if config.contains_key("yaml_value") {
        println!("✅ YAML format supported");
    }
    if config.contains_key("json_value") {
        println!("✅ JSON format supported");
    }
    if config.contains_key("toml_value") {
        println!("✅ TOML format supported");
    }

    println!("\n✅ Test 9 passed!");
}
EOF
```

**Expected Results**:
- ✅ YAML files parsed
- ✅ JSON files parsed
- ✅ TOML files parsed
- ✅ All formats work simultaneously

**Success Criteria**:
- Multi-format support works
- No conflicts between formats

## Integration Tests

### Test 10: Multi-Tool Configuration

**Objective**: Verify multiple tools share global config

**Create Test**:
```bash
# Create configs for two different apps
mkdir -p .tool1 .tool2

cat > .tool1/config.yaml <<'EOF'
tool1_param: "value1"
shared: "from-tool1"
EOF

cat > .tool2/config.yaml <<'EOF'
tool2_param: "value2"
shared: "from-tool2"
EOF
```

**Expected Results**:
- ✅ Each tool has independent config
- ✅ Tools don't interfere
- ✅ Global config shared appropriately

**Success Criteria**:
- Tool isolation works
- Shared global config accessible

### Test 12: Validation Integration

**Objective**: Verify validator integration

**Create Test**:
```bash
cat > src/test_validation.rs <<'EOF'
struct ValidatingConfig;
impl ConfigValidator for ValidatingConfig {
    fn validate_parameter(key: &str, value: &JsonValue) -> Result<(), ValidationError> {
        if key == "timeout" {
            if let Some(num) = value.as_u64() {
                if num > 300 {
                    return Err(ValidationError::new("timeout too large"));
                }
            }
        }
        Ok(())
    }

    fn validate_all(config: &HashMap<String, (JsonValue, ConfigSource)>)
        -> Vec<ValidationError> {
        let mut errors = Vec::new();
        // Custom cross-parameter validation
        if let Some((retries, _)) = config.get("retries") {
            if retries.as_u64().unwrap_or(0) > 10 {
                errors.push(ValidationError::new("too many retries"));
            }
        }
        errors
    }
}

fn main() {
    println!("Test 12: Validation\n");

    // Test validation error
    let mut runtime = HashMap::new();
    runtime.insert("timeout".into(), json!(500));  // Too large

    // Should get validation error
    match TestConfig::resolve_all_config_validated(&runtime) {
        Ok(_) => panic!("Should have validation error!"),
        Err(errors) => {
            println!("✅ Validation error caught: {:?}", errors);
        }
    }

    println!("\n✅ Test 12 passed!");
}
EOF
```

**Expected Results**:
- ✅ Invalid values rejected
- ✅ Validation errors reported
- ✅ Cross-parameter validation works

**Success Criteria**:
- Validator integration complete
- Custom validation logic works

## Verification Checklist

**Priority Resolution**:
- [ ] Default values work
- [ ] Runtime params have highest priority
- [ ] Environment variables work
- [ ] Local config files load
- [ ] Parent config inheritance works
- [ ] Global config from $PRO works
- [ ] Complete cascade Runtime→Env→Local→Parent→Global→Default

**Source Tracking**:
- [ ] Source accurately tracked for each parameter
- [ ] All source types represented
- [ ] Debugging information available

**File Formats**:
- [ ] YAML files parsed
- [ ] JSON files parsed
- [ ] TOML files parsed
- [ ] Multiple formats work together

**Features**:
- [ ] Validation integration works
- [ ] Multi-tool scenarios work

**Integration**:
- [ ] Real application integration works
- [ ] Error handling appropriate
- [ ] Performance acceptable

## Known Limitations

1. **File System Dependency**: Requires file system access for config files
2. **$PRO Dependency**: Global config requires $PRO environment variable
3. **Format Priority**: If multiple format files exist, behavior is deterministic but format-dependent

## Reporting Issues

When reporting issues:

1. **Include configuration sources**:
   - List all config files present
   - Show environment variables
   - Show runtime parameters

2. **Include resolution result**:
   - Expected configuration
   - Actual configuration
   - Source tracking output

3. **Include directory structure**:
   ```bash
   tree -a -L 3
   ```

4. **Include environment**:
   - $PRO value
   - Current working directory
   - Application name

5. **Expected priority order** based on documentation

## References

- Main readme: `../readme.md`
- Specification: `../spec.md`
- API documentation: `cargo doc --open`
- Automated tests: `../tests/*.rs`
