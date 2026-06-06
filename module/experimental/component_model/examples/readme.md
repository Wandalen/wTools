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

### 🟡 **Advanced Patterns**
| Example | Focus | Description |
|---------|--------|-------------|
| **[003_component_from.rs](./003_component_from.rs)** | Advanced Assignment | Order-independent component assignment patterns |
| **[004_working_example.rs](./004_working_example.rs)** | Real-World Usage | Practical configuration management with environment variants |

### 🟠 **Edge Cases & Debugging**
| Example | Focus | Description |
|---------|--------|-------------|
| **[boolean_assignment_error.rs](./boolean_assignment_error.rs)** | Type Ambiguity | Demonstrates solutions for boolean field ambiguity |
| **[boolean_ambiguity_solution.rs](./boolean_ambiguity_solution.rs)** | Boolean Fix | Field-specific methods to resolve boolean ambiguity |
| **[debug_macro_output.rs](./debug_macro_output.rs)** | Macro Debugging | Inspect generated code with `#[debug]` attribute |

## 🚀 Running Examples

**Run any example:**
```bash
cargo run --example <example_name>
```

**Examples:**
```bash
cargo run --example 000_basic_assignment
cargo run --example 004_working_example
cargo run --example debug_macro_output
```

## 💡 Key Concepts Demonstrated

### 🎯 **Type-Driven Assignment**
```rust
#[derive(Default, Assign)]
struct Config 
{
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

## 📊 **Performance Characteristics**

Component model derives provide:

- ✅ **Zero memory overhead** - No additional fields or vtables
- ✅ **Zero runtime cost** - All resolved at compile time via generics
- ✅ **Inline expansion** - Methods marked `#[inline(always)]` for optimization
- ✅ **Type safety** - Compile-time type checking without performance penalty

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

## 📖 **Complete Example List**

All 9 examples currently available:

1. `component_model_trivial.rs` - Quick 30-second introduction
2. `000_basic_assignment.rs` - Type-driven field assignment fundamentals
3. `001_fluent_builder.rs` - Chainable fluent API pattern
4. `002_multiple_components.rs` - Bulk component assignment from tuples
5. `003_component_from.rs` - Advanced order-independent assignment
6. `004_working_example.rs` - Real-world configuration management
7. `boolean_assignment_error.rs` - Handling type ambiguity with booleans
8. `boolean_ambiguity_solution.rs` - Field-specific methods to resolve boolean ambiguity
9. `debug_macro_output.rs` - Inspecting macro-generated code

---

🎓 **Follow the Learning Path above for the best experience learning component model!**
