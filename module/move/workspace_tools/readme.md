# workspace_tools

[![Crates.io](https://img.shields.io/crates/v/workspace_tools)](https://crates.io/crates/workspace_tools)
[![Documentation](https://docs.rs/workspace_tools/badge.svg)](https://docs.rs/workspace_tools)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://img.shields.io/badge/tests-94%20passing-brightgreen)](#testing)

**The missing piece of Rust workspace development** — Runtime workspace-relative path resolution that just works.

## 🎯 why workspace_tools?

Rust's cargo workspaces solve dependency management beautifully, but leave a gap for **runtime path resolution**. Applications struggle with:

```rust
// ❌ fragile - breaks when execution context changes
let config = std::fs::read_to_string("../../../config/app.toml")?;

// ❌ brittle - fails when run from different directories  
let data_path = Path::new("./data/cache.db");

// ❌ hardcoded - not portable across environments
let logs = Path::new("/tmp/myapp/logs");
```

**workspace_tools** provides the missing runtime workspace resolution:

```rust
// ✅ reliable - works from any execution context
let ws = workspace()?;
let config = std::fs::read_to_string(ws.join("config/app.toml"))?;
let data_path = ws.data_dir().join("cache.db");
let logs = ws.logs_dir();
```

## 🚀 key benefits

- **📦 cargo integration** - automatic cargo workspace detection (NEW!)
- **🔧 serde integration** - seamless configuration loading/saving (NEW!)  
- **🎯 zero configuration** - works with simple `.cargo/config.toml` setup
- **🏗️ standard layout** - promotes consistent project structure  
- **🔒 built-in secrets** - secure configuration loading with fallbacks
- **🔍 resource discovery** - find files with glob patterns
- **🧪 testing ready** - isolated workspace utilities for tests
- **🌍 cross-platform** - handles Windows/Mac/Linux path differences
- **⚡ lightweight** - single file, optional features, zero runtime deps

## ⚡ quick start

### 1. add to cargo.toml

```toml
[dependencies]
workspace_tools = "0.1"
```

### 2. configure workspace

Add to workspace root `.cargo/config.toml`:

```toml
[env]
WORKSPACE_PATH = { value = ".", relative = true }
```

### 3. use in your code

```rust
use workspace_tools::workspace;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ws = workspace()?;
    
    // access standard directories
    let config = ws.config_dir().join("app.toml");
    let data = ws.data_dir().join("cache.db");
    let logs = ws.logs_dir();
    
    // check workspace boundaries
    assert!(ws.is_workspace_file(&config));
    
    println!("workspace root: {}", ws.root().display());
    Ok(())
}
```

## 📁 standard directory layout

workspace_tools promotes a consistent, predictable project structure:

```
workspace-root/
├── .cargo/config.toml    # workspace configuration
├── .workspace/           # workspace metadata
├── config/              # ← ws.config_dir()
│   ├── app.toml
│   ├── database.yaml
│   └── services.json
├── data/                # ← ws.data_dir()  
│   ├── cache.db
│   └── state.json
├── logs/                # ← ws.logs_dir()
├── docs/                # ← ws.docs_dir()
├── tests/               # ← ws.tests_dir()
└── .secret/             # ← ws.secret_dir() [secret_management]
    └── -secrets.sh
```

## 🎭 feature showcase

### core functionality

```rust
use workspace_tools::{ workspace, WorkspaceError };

let ws = workspace()?;

// workspace introspection
println!("root: {}", ws.root().display());
ws.validate()?; // ensure workspace is accessible

// path operations  
let app_config = ws.join("config/app.toml");
let normalized = ws.normalize_path("config/../data/file.json")?;

// boundary checking
assert!(ws.is_workspace_file(&app_config));
assert!(!ws.is_workspace_file("/etc/passwd"));
```

### resource discovery (glob feature)

```toml
[dependencies]
workspace_tools = { version = "0.1", features = ["glob"] }
```

```rust
let ws = workspace()?;

// find files with patterns
let rust_files = ws.find_resources("src/**/*.rs")?;
let test_files = ws.find_resources("tests/**/*.rs")?;

// smart config discovery
let db_config = ws.find_config("database")?;  
// finds config/database.{toml,yaml,json} or .database.toml
```

### secret management (secret_management feature)

```toml
[dependencies] 
workspace_tools = { version = "0.1", features = ["secret_management"] }
```

```rust
// .secret/-secrets.sh
// API_KEY=your_secret_here
// DATABASE_URL="postgresql://localhost/db"

let ws = workspace()?;

// load all secrets
let secrets = ws.load_secrets_from_file("-secrets.sh")?;

// load specific key with environment fallback
let api_key = ws.load_secret_key("API_KEY", "-secrets.sh")?;
```

### cargo integration (cargo_integration feature)

```toml
[dependencies]
workspace_tools = { version = "0.1", features = ["cargo_integration"] }
```

```rust
use workspace_tools::Workspace;

// automatic cargo workspace detection - no .cargo/config.toml needed!
let ws = Workspace::from_cargo_workspace()?;

// or use resolve_or_fallback for cargo-first detection
let ws = Workspace::resolve_or_fallback(); // tries cargo → env → current_dir → git

// access cargo metadata
if ws.is_cargo_workspace() {
    let metadata = ws.cargo_metadata()?;
    println!("workspace root: {}", metadata.workspace_root.display());
    
    for member in metadata.members {
        println!("  • {} v{}", member.name, member.version);
    }
    
    let member_dirs = ws.workspace_members()?;
}
```

### serde integration (serde_integration feature)

```toml
[dependencies]
workspace_tools = { version = "0.1", features = ["serde_integration"] }
serde = { version = "1.0", features = ["derive"] }
```

```rust
use serde::{Serialize, Deserialize};
use workspace_tools::workspace;

#[derive(Serialize, Deserialize)]
struct AppConfig {
    name: String,
    port: u16,
    database: DatabaseConfig,
}

let ws = workspace()?;

// automatic format detection - loads config/app.{toml,yaml,json}
let config: AppConfig = ws.load_config("app")?;

// save configuration with format preservation
ws.save_config("app", &config)?;

// load from specific file with format detection
let config: AppConfig = ws.load_config_from("config/custom.json")?;

// layered configuration merging
let config: AppConfig = ws.load_config_layered(&["base", "development"])?;

// partial configuration updates
let updates = serde_json::json!({ "port": 9090 });
let updated: AppConfig = ws.update_config("app", updates)?;
```

## 🧪 testing integration

workspace_tools makes testing with isolated workspaces trivial:

```rust
#[cfg(test)]
mod tests {
    use workspace_tools::testing::create_test_workspace;
    
    #[test]
    fn test_config_loading() {
        let (_temp_dir, ws) = create_test_workspace();
        
        // test in complete isolation
        let config_path = ws.config_dir().join("test.toml");  
        // ... test logic
    }
}
```

## 🏗️ integration examples

### with serde configuration

```rust
use serde::Deserialize;

#[derive(Deserialize)]
struct AppConfig {
    name: String,
    port: u16,
}

let ws = workspace()?;
let config_path = ws.find_config("app")?;
let config: AppConfig = toml::from_str(&std::fs::read_to_string(config_path)?)?;
```

### with tracing logs

```rust
use tracing_appender::rolling::{RollingFileAppender, Rotation};

let ws = workspace()?;
let log_dir = ws.logs_dir();
std::fs::create_dir_all(&log_dir)?;

let file_appender = RollingFileAppender::new(Rotation::DAILY, log_dir, "app.log");
```

### with database migrations

```rust
let ws = workspace()?;
let migrations_dir = ws.join("migrations");
// run migrations from consistent location regardless of cwd
```

## 🌍 deployment flexibility

### docker containers

```dockerfile
FROM rust:alpine
ENV WORKSPACE_PATH=/app
WORKDIR /app
COPY . .
RUN cargo build --release
```

### systemd services

```ini
[Service]
Environment=WORKSPACE_PATH=/opt/myapp
WorkingDirectory=/opt/myapp
ExecStart=/opt/myapp/target/release/myapp
```

### just/make integration

```just
# justfile
export WORKSPACE_PATH := justfile_directory()

test:
    cargo test

run:
    cargo run
```

## 📊 use cases

✅ **cli applications** - consistent config/data/log paths  
✅ **web services** - reliable asset and config loading  
✅ **desktop apps** - standard directory structures  
✅ **build tools** - workspace-aware file processing  
✅ **testing frameworks** - isolated workspace environments  
✅ **data processing** - portable path resolution  

## ⚙️ fallback strategies

workspace_tools is resilient with multiple resolution strategies:

1. **environment variable** (`WORKSPACE_PATH`) - primary method
2. **current directory** - when no env var set  
3. **git repository root** - searches upward for `.git/`
4. **current working directory** - ultimate fallback (never fails)

```rust
// always succeeds with some valid workspace root
let ws = Workspace::resolve_or_fallback();
```

## 📚 comprehensive examples

Check out `/examples/` for detailed usage patterns:

- `workspace_basic_usage.rs` - core functionality walkthrough
- `secret_management.rs` - secure configuration patterns  
- `resource_discovery.rs` - file finding with glob patterns

## 🧪 testing

workspace_tools maintains **94 passing tests** with comprehensive coverage:

- core workspace resolution (13 tests)
- comprehensive integration suite (63 tests)  
- secret management functionality (1 test)
- documentation examples (11 tests)
- performance benchmarks (5 ignored/optional)

```bash
cargo test                    # run core tests
cargo test --all-features     # run all feature tests
cargo test --features glob    # test glob functionality
```

## 📈 roadmap & contributing

### planned features

- **config validation** - schema-based configuration checking
- **workspace templates** - scaffold standard layouts  
- **plugin system** - extensible workspace behaviors
- **async file operations** - tokio integration
- **workspace watching** - file change notifications

### contributing

contributions welcome! workspace_tools follows the **design rulebook** patterns:

- explicit lifetimes and error handling
- comprehensive testing with matrix coverage  
- feature-gated optional functionality
- consistent 2-space formatting

see [contributing guidelines](contributing.md) for details.

## ⚖️ license

licensed under the [MIT license](license).

---

> **"finally, a workspace tool that works the way rust developers think"** — eliminate path resolution pain forever