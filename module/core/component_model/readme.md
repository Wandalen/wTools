<!-- {{# generate.module_header{} #}} -->

# Module :: component_model

[![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental)
[![rust-status](https://github.com/Wandalen/wTools/actions/workflows/module_component_model_push.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/module_component_model_push.yml)
[![docs.rs](https://img.shields.io/docsrs/component_model?color=e3e8f0&logo=docs.rs)](https://docs.rs/component_model)
[![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=module%2Fcore%2Fcomponent_model%2Fexamples%2Fcomponent_model_trivial.rs,RUN_POSTFIX=--example%20module%2Fcore%2Fcomponent_model%2Fexamples%2Fcomponent_model_trivial.rs/https://github.com/Wandalen/wTools)
[![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)

Revolutionary type-safe component assignment for Rust. Build complex objects with zero boilerplate using derive macros and type-driven field setting. Perfect for configuration builders, fluent APIs, and object composition patterns.

## 🚀 Why Component Model?

Traditional struct initialization is verbose and error-prone:

```rust
# struct Config { host : String, port : i32 }
# struct ConfigBuilder;
# impl ConfigBuilder 
{
#   fn new() -> Self { ConfigBuilder }
#   fn host( self, _ : &str ) -> Self { self }
#   fn port( self, _ : i32 ) -> Self { self }
#   fn build( self ) -> Config { Config { host : "".to_string(), port : 0 } }
# }
// Traditional approach - repetitive and fragile
let config = Config
{
  host : "localhost".to_string(),
  port : 8080,
};

// Builder pattern - lots of boilerplate
let config = ConfigBuilder::new()
.host( "localhost" )
.port( 8080 )
.build();
```

**Component Model approach** - Clean, type-safe, zero boilerplate:

```rust
use component_model::Assign;

#[ derive( Default, Assign ) ]
struct Config
{
  host : String,
  port : i32,
}

// Set components by type - no field names needed!
let mut config = Config::default();
config.assign( "localhost" );  // Automatically sets String field
config.assign( 8080 );         // Automatically sets i32 field  

// Or use fluent style
let config = Config::default()
.impute( "localhost" )
.impute( 8080 );
```

## ✨ Key Features

- **🎯 Type-driven assignment** - Set fields by component type, not field name
- **🔧 Zero boilerplate** - Derive macros generate all implementations automatically  
- **🌊 Fluent APIs** - Chainable `impute()` method for builder patterns
- **🛡️ Type safety** - All assignments checked at compile time
- **🔄 Flexible conversion** - Accepts any type convertible to target field type
- **📦 Multiple assignment** - Set multiple components with `ComponentsAssign`
- **⚡ Popular types support** - Built-in support for Duration, PathBuf, SocketAddr, and more
- **🏗️ ComponentModel derive** - Unified derive macro combining all functionality

## 🚀 Quick Start

Add to your `Cargo.toml`:

```toml
[ dependencies ]
component_model = "0.4"
```

### Feature Flags

Component Model follows granular feature gating for minimal builds:

```toml
[ dependencies ]
# Minimal version - no features enabled by default  
component_model = { version = "0.4", default-features = false }

# Enable specific features as needed
component_model = { version = "0.4", features = [ "derive_component_model" ] }

# Or enable all features (default)
component_model = { version = "0.4", features = [ "full" ] }
```

Available features:
- **`enabled`** - Master switch for core functionality
- **`full`** - All features (enabled by default)
- **`derive_component_model`** - Unified ComponentModel derive macro
- **`derive_component_assign`** - Basic Assign derive macro
- **`derive_components_assign`** - Multiple component assignment
- **`derive_component_from`** - Component creation from single values
- **`derive_from_components`** - Component creation from multiple values

## 📖 Core Concepts

### 1. Basic Assignment with ComponentModel

```rust
use component_model::{ ComponentModel, Assign };

#[ derive( Default, Debug, ComponentModel ) ]
struct Person
{
  age : i32,
  name : String,
}

fn main()
{
  let mut person = Person::default();
  
  // Type-driven assignment - no field names!
  person.assign( 25 );           // Sets age : i32  
  person.assign( "Alice" );      // Sets name : String
  
  println!( "{:?}", person );    // Person { age: 25, name: "Alice" }
}
```

### 2. Popular Types Support

ComponentModel provides built-in support for popular Rust types with intelligent conversion:

```rust
use component_model::{ ComponentModel, Assign };
use std::time::Duration;
use std::path::PathBuf;

#[ derive( Default, Debug, ComponentModel ) ]
struct Config
{
  timeout : Duration,
  config_path : PathBuf,
  port : i32,
}

fn main()
{
  let mut config = Config::default();
  
  // Duration from seconds (u64)
  config.assign( 30u64 );  // Duration::from_secs( 30 )
  
  // Duration from fractional seconds (f64)  
  config.assign( 2.5f64 ); // Duration::from_secs_f64( 2.5 )
  
  // PathBuf from string slice
  config.assign( "/etc/app.conf" ); // PathBuf::from( "/etc/app.conf" )
  
  // i32 assignment
  config.assign( 8080i32 );
}
```

### 3. Enum Fields in Structs

ComponentModel works with structs that contain enum fields, enabling type-safe enum assignment:

```rust
use component_model::{ ComponentModel, Assign };

#[ derive( Debug, PartialEq ) ]
enum Status
{
  Pending,
  Processing { progress : f64 },
  Completed { result : String },
  Failed { error : String },
}

impl Default for Status
{
  fn default() -> Self { Status::Pending }
}

#[ derive( Default, Debug, ComponentModel ) ]
struct Task
{
  id : u32,
  status : Status,
  priority : u8,
}

fn main()
{
  let mut task = Task::default();
  
  // Use field-specific methods with enums
  task.id_set( 42u32 );
  task.priority_set( 5u8 );
  task.status_set( Status::Processing { progress: 0.75 } );
  
  println!( "{:?}", task );
  
  // Fluent style with enums
  let completed_task = Task::default()
    .id_with( 100u32 )
    .status_with( Status::Completed { result: "Success".to_string() } )
    .priority_with( 1u8 );
    
  match completed_task.status {
    Status::Completed { result } => println!( "Task completed: {}", result ),
    _ => println!( "Unexpected status" ),
  }
}
```

#### Complex Enum Fields

```rust
use component_model::{ ComponentModel, Assign };
use std::time::Duration;

#[ derive( Debug ) ]
enum ConnectionState
{
  Disconnected,
  Connecting { timeout : Duration },
  Connected { session_id : String },
}

impl Default for ConnectionState
{
  fn default() -> Self { ConnectionState::Disconnected }
}

#[ derive( Default, Debug, ComponentModel ) ]
struct NetworkService
{
  name : String,
  state : ConnectionState,
  retry_count : u32,
}

fn main()
{
  let mut service = NetworkService::default();
  
  // Field-specific methods work seamlessly with enum fields
  service.name_set( "WebSocket".to_string() );
  service.retry_count_set( 3u32 );
  service.state_set( ConnectionState::Connected { 
    session_id: "sess_12345".to_string() 
  } );
  
  // Fluent pattern with complex enums
  let connecting_service = NetworkService::default()
    .name_with( "HTTP Client".to_string() )
    .state_with( ConnectionState::Connecting { 
      timeout: Duration::from_secs( 30 )
    } )
    .retry_count_with( 0u32 );
    
  println!( "{:?}", connecting_service );
}
```

> **Note**: Direct ComponentModel derive on enums is planned for future releases. Currently, enums work as field types in structs with ComponentModel.

### 4. Fluent Builder Pattern

```rust
# use component_model::{ ComponentModel, Assign };
# #[ derive( Default, ComponentModel ) ]
# struct Person { name : String, age : i32 }
let person = Person::default()
.impute( "Bob" )           // Chainable assignment
.impute( 30 );             // Returns Self for chaining
```

### 5. Multiple Component Assignment

```rust
use component_model::{ ComponentModel, Assign };

#[ derive( Default, ComponentModel ) ]
struct ServerConfig
{
  host : String,
  port : i32, 
}

let mut config = ServerConfig::default();
config.assign( "localhost" );    // String component
config.assign( 8080 );           // i32 component
```

### 6. Manual Implementation (Advanced)

For custom behavior, implement traits manually:

```rust
use component_model::prelude::*;

struct Database
{
  url : String,
  pool_size : usize,
}

impl< T : Into< String > > Assign< String, T > for Database
{
  fn assign( &mut self, component : T )
  {
    self.url = component.into();
  }
}

impl< T : Into< usize > > Assign< usize, T > for Database
{  
  fn assign( &mut self, component : T )
  {
    self.pool_size = component.into();
  }
}

let config = DatabaseConfig::default()
.impute( "postgres.example.com" )    // String
.impute( 5432 )                      // i32  
.impute( 30u64 );                    // Duration from seconds
```

### HTTP Client Builders
```rust
use component_model::{ ComponentModel, Assign };
use std::time::Duration;

#[ derive( Default, ComponentModel ) ]
struct HttpClient
{
  base_url : String,
  timeout : Duration,
}

let client = HttpClient::default()
.impute( "https://api.example.com" )
.impute( 30.0f64 );  // Duration from fractional seconds
```

### Game Entity Systems
```rust
use component_model::{ ComponentModel, Assign };

#[ derive( Default, ComponentModel ) ]
struct Player
{
  name : String,
  level : i32,
}

// Initialize components
let mut player = Player::default();
player.assign( "Hero" );
player.assign( 1 );
```

## 🧪 Examples

Explore the [examples directory](examples/) for comprehensive usage patterns:

- **[`000_basic_assignment.rs`](examples/000_basic_assignment.rs)** - Basic component assignment
- **[`001_fluent_builder.rs`](examples/001_fluent_builder.rs)** - Fluent builder pattern
- **[`002_multiple_components.rs`](examples/002_multiple_components.rs)** - Multiple component handling
- **[`003_component_from.rs`](examples/003_component_from.rs)** - Component creation patterns
- **[`004_working_example.rs`](examples/004_working_example.rs)** - Real-world usage scenarios
- **[`component_model_trivial.rs`](examples/component_model_trivial.rs)** - Minimal example

## 📋 Supported Popular Types

ComponentModel includes built-in intelligent conversion for:

| Type | Input Types | Example |
|------|-------------|---------|
| `Duration` | `u64`, `f64`, `(u64, u32)` | `config.assign( 30u64 )` |
| `PathBuf` | `&str`, `String` | `config.assign( "/path/file" )` |
| `SocketAddr` | *Coming soon* | String parsing planned |
| `HashMap` | *Framework ready* | Vec conversion planned |
| `HashSet` | *Framework ready* | Vec conversion planned |

## ⚠️ Important Limitations

**Type Ambiguity**: When a struct has multiple fields of the same type, `assign()` becomes ambiguous and won't compile. This is by design for type safety.

```rust
# use component_model::{ ComponentModel, Assign };
# #[ derive( Default, ComponentModel ) ]
struct Config
{
  host : String,
  database : String,  // Multiple String fields cause ambiguity
}

// This won't compile due to ambiguity:
// let mut config = Config::default();
// config.assign( "localhost" );  // Error: which String field?
```

## 📚 Available Derive Macros

- **`ComponentModel`** - ⭐ **Recommended** - Unified derive combining all functionality
- **`Assign`** - Basic component assignment by type
- **`ComponentsAssign`** - Multiple component assignment from tuples  
- **`ComponentFrom`** - Create objects from single components
- **`FromComponents`** - Create objects from multiple components

## 🎯 Real-World Use Cases

### Configuration Management with Popular Types
```rust
use component_model::{ ComponentModel, Assign };
use std::time::Duration;
use std::path::PathBuf;

#[ derive( Default, ComponentModel ) ]
struct DatabaseConfig
{
  host : String,
  port : i32,
  timeout : Duration,
}

let config = DatabaseConfig::default()
.impute( "postgres.example.com" )    // String
.impute( 5432 )                      // i32  
.impute( 30u64 );                    // Duration from seconds
```

### HTTP Client Builders
```rust
use component_model::{ ComponentModel, Assign };
use std::time::Duration;

#[ derive( Default, ComponentModel ) ]
struct HttpClient
{
  base_url : String,
  timeout : Duration,
}

let client = HttpClient::default()
.impute( "https://api.example.com" )
.impute( 30.0f64 );  // Duration from fractional seconds
```

### Game Entity Systems
```rust
use component_model::{ ComponentModel, Assign };

#[ derive( Default, ComponentModel ) ]
struct Player
{
  name : String,
  level : i32,
}

// Initialize components
let mut player = Player::default();
player.assign( "Hero" );
player.assign( 1 );
```

## 🧪 Examples

Explore the [examples directory](examples/) for comprehensive usage patterns:

- **[`000_basic_assignment.rs`](examples/000_basic_assignment.rs)** - Basic component assignment
- **[`001_fluent_builder.rs`](examples/001_fluent_builder.rs)** - Fluent builder pattern
- **[`002_multiple_components.rs`](examples/002_multiple_components.rs)** - Multiple component handling
- **[`003_component_from.rs`](examples/003_component_from.rs)** - Component creation patterns
- **[`004_working_example.rs`](examples/004_working_example.rs)** - Real-world usage scenarios
- **[`component_model_trivial.rs`](examples/component_model_trivial.rs)** - Minimal example

## 📋 Supported Popular Types

ComponentModel includes built-in intelligent conversion for:

| Type | Input Types | Example |
|------|-------------|---------|
| `Duration` | `u64`, `f64`, `(u64, u32)` | `config.assign( 30u64 )` |
| `PathBuf` | `&str`, `String` | `config.assign( "/path/file" )` |
| `SocketAddr` | *Coming soon* | String parsing planned |
| `HashMap` | *Framework ready* | Vec conversion planned |
| `HashSet` | *Framework ready* | Vec conversion planned |

## ⚠️ Important Limitations

**Type Ambiguity**: When a struct has multiple fields of the same type, `assign()` becomes ambiguous and won't compile. This is by design for type safety.

```rust
# use component_model::{ ComponentModel, Assign };
# #[ derive( Default, ComponentModel ) ]
struct Config
{
  host : String,
  database : String,  // Multiple String fields cause ambiguity
}

// This won't compile due to ambiguity:
// let mut config = Config::default();
// config.assign( "localhost" );  // Error: which String field?
```

**Workarounds**:
1. Use different types when possible (e.g., `String` vs `PathBuf`)
2. Use direct field assignment: `config.host = "localhost".to_string();`
3. Implement manual `Assign` traits for specific use cases

## 🔗 Learn More

- **[📁 Examples](examples/)** - Step-by-step examples showing all features
- **[📖 API Docs](https://docs.rs/component_model)** - Complete API reference  
- **[🐙 Source Code](https://github.com/Wandalen/wTools/tree/master/module/core/component_model)** - Contribute or report issues
- **[💬 Discord](https://discord.gg/m3YfbXpUUY)** - Get help and discuss

---

*Made with ❤️ as part of the [wTools](https://github.com/Wandalen/wTools) ecosystem*