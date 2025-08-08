# workspace_tools

[![Crates.io](https://img.shields.io/crates/v/workspace_tools)](https://crates.io/crates/workspace_tools)
[![Documentation](https://docs.rs/workspace_tools/badge.svg)](https://docs.rs/workspace_tools)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Universal workspace-relative path resolution for Rust projects. Provides consistent, reliable path management regardless of execution context or working directory.

## problem solved

Software projects frequently struggle with path resolution issues:
- **execution context dependency**: paths break when code runs from different directories
- **environment inconsistency**: different developers have different working directory habits
- **testing fragility**: tests fail when run from different locations
- **ci/cd brittleness**: automated systems may execute from unexpected directories

## solution

`workspace_tools` provides a standardized workspace-relative path resolution mechanism using cargo's built-in environment variable injection.

## quick start

### 1. configure cargo

Add to your workspace root `.cargo/config.toml`:

```toml
[env]
WORKSPACE_PATH = { value = ".", relative = true }
```

### 2. add dependency

```toml
[dependencies]
workspace_tools = "0.1"
```

### 3. use in code

```rust
use workspace_tools::{ Workspace, workspace };

// get workspace instance
let ws = workspace()?;

// resolve workspace-relative paths
let config_path = ws.config_dir().join( "app.toml" );
let data_path = ws.data_dir().join( "cache.db" );

// load configuration from standard location
let config_file = ws.find_config( "database" )?;
```

## features

### core functionality
- **workspace resolution**: automatic workspace root detection
- **path joining**: safe workspace-relative path construction
- **standard directories**: conventional subdirectory layout
- **cross-platform**: works on windows, macos, linux

### optional features
- **`glob`**: pattern-based resource discovery
- **`secret_management`**: secure configuration file handling

## standard directory layout

`workspace_tools` follows these conventions:

```
workspace-root/
├── .workspace/      # workspace metadata
├── secret/          # secret configuration files
├── config/          # configuration files
├── data/            # application data
├── logs/            # log files
├── docs/            # documentation
└── tests/           # test resources
```

## api overview

### basic usage

```rust
use workspace_tools::{ Workspace, WorkspaceError };

// resolve workspace from environment
let workspace = Workspace::resolve()?;

// access workspace root
let root = workspace.root();

// get standard directories
let config_dir = workspace.config_dir();
let data_dir = workspace.data_dir();
let logs_dir = workspace.logs_dir();

// join paths safely
let app_config = workspace.join( "config/app.toml" );
```

### resource discovery (with `glob` feature)

```rust
use workspace_tools::workspace;

let ws = workspace()?;

// find all png files in assets
let images = ws.find_resources( "assets/**/*.png" )?;

// find configuration files
let config = ws.find_config( "database" )?;
```

### error handling

```rust
use workspace_tools::{ workspace, WorkspaceError };

match workspace()
{
  Ok( ws ) =>
  {
    // use workspace
  }
  Err( WorkspaceError::EnvironmentVariableMissing( _ ) ) =>
  {
    // handle missing WORKSPACE_PATH
  }
  Err( WorkspaceError::PathNotFound( path ) ) =>
  {
    // handle invalid workspace
  }
  Err( e ) =>
  {
    // handle other errors
  }
}
```

## testing

The crate includes comprehensive test utilities:

```rust
#[ cfg( test ) ]
mod tests
{
  use workspace_tools::testing::create_test_workspace;

  #[ test ]
  fn test_my_feature()
  {
    let ( _temp_dir, workspace ) = create_test_workspace();

    // test with isolated workspace
    let config = workspace.config_dir().join( "test.toml" );
    assert!( config.starts_with( workspace.root() ) );
  }
}
```

## integration with build tools

### cargo
```toml
# .cargo/config.toml
[env]
WORKSPACE_PATH = { value = ".", relative = true }
```

### justfile
```make
# set workspace for just commands
export WORKSPACE_PATH := justfile_directory()
```

### docker
```dockerfile
ENV WORKSPACE_PATH=/app
WORKDIR /app
```

## license

licensed under the MIT license. see [license](license) for details.

## contributing

contributions are welcome! please see [contributing guidelines](contributing.md) for details.