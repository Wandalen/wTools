# Component Model Examples

🚀 **Learn component model step-by-step with comprehensive examples!**

This directory contains a complete learning path for the `component_model` crate, from basic concepts to advanced patterns. Each example is self-contained and builds upon previous concepts.

## 🎯 Quick Start

**New to component model?** Start here:

```bash
cargo run --example component_model_trivial
```

Then follow the **Learning Path** below for a structured progression.

## 📚 Learning Path 

### 🟢 **Core Concepts** (Start Here)
| Example | Focus | Description |
|---------|--------|-------------|
| **[component_model_trivial.rs](./component_model_trivial.rs)** | Quick Start | Minimal working example - see it in 30 seconds |
| **[000_basic_assignment.rs](./000_basic_assignment.rs)** | Fundamentals | Type-driven field assignment with `assign()` |
| **[001_fluent_builder.rs](./001_fluent_builder.rs)** | Builder Pattern | Chainable `impute()` method for fluent APIs |
| **[002_multiple_components.rs](./002_multiple_components.rs)** | Bulk Operations | Assigning multiple components from tuples |

### 🟡 **Creation Patterns**  
| Example | Focus | Description |
|---------|--------|-------------|
| **[003_component_from.rs](./003_component_from.rs)** | Object Creation | Creating objects FROM single components |
| **[004_from_components.rs](./004_from_components.rs)** | Bulk Creation | Creating objects FROM multiple components |

### 🟠 **Real-World Usage**
| Example | Focus | Description |
|---------|--------|-------------|
| **[006_real_world_config.rs](./006_real_world_config.rs)** | Configuration | Practical config management system |
| **[005_manual_implementation.rs](./005_manual_implementation.rs)** | Customization | Custom trait implementations with validation |

### 🔴 **Advanced Topics**
| Example | Focus | Description |
|---------|--------|-------------|
| **[007_advanced_patterns.rs](./007_advanced_patterns.rs)** | Advanced Usage | Generics, nesting, optional components |
| **[008_performance_comparison.rs](./008_performance_comparison.rs)** | Performance | Benchmarks and zero-cost abstraction proof |

## 🚀 Running Examples

**Run any example:**
```bash
cargo run --example <example_name>
```

**Examples:**
```bash
cargo run --example 000_basic_assignment
cargo run --example 006_real_world_config
cargo run --example 008_performance_comparison
```

## 💡 Key Concepts Demonstrated

### 🎯 **Type-Driven Assignment**
```rust
#[derive(Default, Assign)]
struct Config {
  host : String,
  port : u16,
  timeout : f64,
}

let config = Config::default()
  .impute("localhost")    // Automatically sets String field
  .impute(8080u16)        // Automatically sets u16 field
  .impute(30.0f64);       // Automatically sets f64 field
```

### 🔗 **Multiple Component Assignment**
```rust
config.components_assign((
  "localhost",      // String component  
  8080u16,          // u16 component
  30.0f64,          // f64 component
));
```

### 🏗️ **Object Creation from Components**
```rust
let config : Config = FromComponents::from_components((
  "localhost", 8080u16, 30.0f64
));
```

## 📊 **Performance Highlights**

From `008_performance_comparison.rs`:

- ✅ **Zero memory overhead** vs traditional structs
- ✅ **Zero runtime cost** - compiles to optimized assembly
- ✅ **Comparable performance** to hand-written builders
- ✅ **Type safety** without performance penalty

## 🎯 **Use Cases Covered**

- **Configuration Management** - Environment-specific settings
- **Builder Patterns** - Fluent object construction  
- **HTTP Clients** - API configuration builders
- **Database Connections** - Connection pool setup
- **Game Development** - Entity component systems
- **Validation** - Custom assignment logic
- **Performance-Critical** - Zero-cost abstractions

## 🛠️ **Available Derive Macros**

All examples demonstrate these derives:

```rust
#[derive(Assign)]              // Basic component assignment
#[derive(ComponentsAssign)]    // Multiple component assignment  
#[derive(ComponentFrom)]       // Create from single component
#[derive(FromComponents)]      // Create from multiple components
```

## 📖 **Legacy Examples** 

The following are legacy examples from the previous codebase (may use older patterns):

| Group | Example | Description |
|-------|---------|-------------|
| **Legacy Usage** | `component_model_many_fields.rs` | Various field types with scalar setters |
| **Legacy Collections** | `component_model_collection_*.rs` | Collection building patterns |
| **Legacy Customization** | `component_model_custom_*.rs` | Custom defaults and setters |

---

🎓 **Follow the Learning Path above for the best experience learning component model!**
