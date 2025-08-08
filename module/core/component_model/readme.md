<!-- {{# generate.module_header{} #}} -->

# Module :: component_model

[![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental)
[![rust-status](https://github.com/Wandalen/wTools/actions/workflows/module_component_model_push.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/module_component_model_push.yml)
[![docs.rs](https://img.shields.io/docsrs/component_model?color=e3e8f0&logo=docs.rs)](https://docs.rs/component_model)
[![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=module%2Fcore%2Fcomponent_model%2Fexamples%2Fcomponent_model_trivial.rs,RUN_POSTFIX=--example%20module%2Fcore%2Fcomponent_model%2Fexamples%2Fcomponent_model_trivial.rs/https://github.com/Wandalen/wTools)
[![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)

**Revolutionary type-safe component assignment for Rust** - Build complex objects with zero boilerplate using derive macros and type-driven field setting. Perfect for configuration builders, fluent APIs, and object composition patterns.

## 🚀 Why Component Model?

Traditional struct initialization is verbose and error-prone:

```rust
# struct Config { host: String, port: i32 }
# struct ConfigBuilder;
# impl ConfigBuilder {
#   fn new() -> Self { ConfigBuilder }
#   fn host(self, _: &str) -> Self { self }
#   fn port(self, _: i32) -> Self { self }
#   fn build(self) -> Config { Config { host: "".to_string(), port: 0 } }
# }
// Traditional approach - repetitive and fragile
let config = Config {
  host: "localhost".to_string(),
  port: 8080,
};

// Builder pattern - lots of boilerplate
let config = ConfigBuilder::new()
  .host("localhost")
  .port(8080)  
  .build();
```

**Component Model approach** - Clean, type-safe, zero boilerplate:

```rust
use component_model::Assign;

#[derive(Default, Assign)]
struct Config {
  host: String,
  port: i32,
}

// Set components by type - no field names needed!
let mut config = Config::default();
config.assign("localhost");  // Automatically sets String field
config.assign(8080);         // Automatically sets i32 field  

// Or use fluent style
let config = Config::default()
  .impute("localhost")
  .impute(8080);
```

## ✨ Key Features

- **🎯 Type-driven assignment** - Set fields by component type, not field name
- **🔧 Zero boilerplate** - Derive macros generate all implementations automatically  
- **🌊 Fluent APIs** - Chainable `impute()` method for builder patterns
- **🛡️ Type safety** - All assignments checked at compile time
- **🔄 Flexible conversion** - Accepts any type convertible to target field type
- **📦 Multiple assignment** - Set multiple components with `ComponentsAssign`

## 🚀 Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
component_model = "0.4"
```

## 📖 Core Concepts

### 1. Basic Assignment with Derive

```rust
use component_model::Assign;

#[derive(Default, Debug, Assign)]
struct Person {
  age: i32,
  name: String,
}

fn main() {
  let mut person = Person::default();
  
  // Type-driven assignment - no field names!
  person.assign(25);           // Sets age: i32  
  person.assign("Alice");      // Sets name: String
  
  println!("{:?}", person);    // Person { age: 25, name: "Alice" }
}
```

### 2. Fluent Builder Pattern

```rust
# use component_model::Assign;
# #[derive(Default, Assign)]
# struct Person { name: String, age: i32 }
let person = Person::default()
  .impute("Bob")           // Chainable assignment
  .impute(30);             // Returns Self for chaining
```

### 3. Multiple Component Assignment

```rust
use component_model::Assign;

#[derive(Default, Assign)]
struct ServerConfig {
  host: String,
  port: i32, 
}

let mut config = ServerConfig::default();
config.assign("localhost");    // String component
config.assign(8080);           // i32 component
```

### 4. Manual Implementation (Advanced)

For custom behavior, implement traits manually:

```rust
use component_model::prelude::*;

struct Database {
  url: String,
  pool_size: usize,
}

impl<T: Into<String>> Assign<String, T> for Database {
  fn assign(&mut self, component: T) {
    self.url = component.into();
  }
}

impl<T: Into<usize>> Assign<usize, T> for Database {  
  fn assign(&mut self, component: T) {
    self.pool_size = component.into();
  }
}
```

## 📚 Available Derive Macros

- **`Assign`** - Basic component assignment by type
- **`ComponentsAssign`** - Multiple component assignment from tuples  
- **`ComponentFrom`** - Create objects from single components
- **`FromComponents`** - Create objects from multiple components

## 🎯 Real-World Use Cases

### Configuration Management
```rust
use component_model::Assign;

#[derive(Default, Assign)]
struct DatabaseConfig {
  host: String,
  port: i32,
}

let config = DatabaseConfig::default()
  .impute("postgres.example.com")
  .impute(5432);
```

### HTTP Client Builders
```rust
use component_model::Assign;

#[derive(Default, Assign)]
struct HttpClient {
  base_url: String,
  timeout_secs: i32,
}

let client = HttpClient::default()
  .impute("https://api.example.com")
  .impute(30);
```

### Game Entity Systems
```rust
use component_model::Assign;

#[derive(Default, Assign)]
struct Player {
  name: String,
  level: i32,
}

// Initialize components
let mut player = Player::default();
player.assign("Hero");
player.assign(1);
```

## 🔗 Learn More

- **[📁 Examples](examples/)** - Step-by-step examples showing all features
- **[📖 API Docs](https://docs.rs/component_model)** - Complete API reference  
- **[🐙 Source Code](https://github.com/Wandalen/wTools/tree/master/module/core/component_model)** - Contribute or report issues
- **[💬 Discord](https://discord.gg/m3YfbXpUUY)** - Get help and discuss

---

*Made with ❤️ as part of the [wTools](https://github.com/Wandalen/wTools) ecosystem*
