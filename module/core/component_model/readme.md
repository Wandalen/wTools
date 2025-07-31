<!-- {{# generate.module_header{} #}} -->

# Module :: component_model

[![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental)
[![rust-status](https://github.com/Wandalen/wTools/actions/workflows/module_component_model_push.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/module_component_model_push.yml)
[![docs.rs](https://img.shields.io/docsrs/component_model?color=e3e8f0&logo=docs.rs)](https://docs.rs/component_model)
[![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=module%2Fcore%2Fcomponent_model%2Fexamples%2Fcomponent_model_trivial.rs,RUN_POSTFIX=--example%20module%2Fcore%2Fcomponent_model%2Fexamples%2Fcomponent_model_trivial.rs/https://github.com/Wandalen/wTools)
[![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)

A flexible component model for Rust supporting generic assignment and type-based field access.

## Installation

Add `component_model` to your `Cargo.toml`:

```sh
cargo add component_model
```

## Minimal Example: Using Assign

```rust
use component_model::prelude::Assign;

#[derive(Debug, PartialEq, Default)]
struct Person {
  age: i32,
  name: String,
}

impl<IntoT> Assign<i32, IntoT> for Person
where
  IntoT: Into<i32>,
{
  fn assign(&mut self, component: IntoT) {
    self.age = component.into();
  }
}

impl<IntoT> Assign<String, IntoT> for Person
where
  IntoT: Into<String>,
{
  fn assign(&mut self, component: IntoT) {
    self.name = component.into();
  }
}

fn main() {
  let mut person = Person::default();
  person.assign(42);
  person.assign("Alice");
  assert_eq!(person, Person { age: 42, name: "Alice".to_string() });
}
```

## API Overview

- **Assign**: Generic trait for assigning values to struct fields by type.
- **AssignWithType**: Trait for assigning values with explicit type annotation.
- **ComponentsAssign**: Trait for assigning multiple components at once.

See [component_model_types documentation](https://docs.rs/component_model_types) for details.

## Where to Go Next

- [Examples Directory](https://github.com/Wandalen/wTools/tree/master/module/core/component_model/examples): Explore practical, runnable examples.
- [API Documentation (docs.rs)](https://docs.rs/component_model): Get detailed information on all public types, traits, and functions.
- [Repository (GitHub)](https://github.com/Wandalen/wTools/tree/master/module/core/component_model): View the source code, contribute, or report issues.
