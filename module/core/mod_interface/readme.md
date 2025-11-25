<!-- {{# generate.module_header{} #}} -->

# Module :: `mod_interface`
<!--{ generate.module_header.start() }-->
 [![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/module_mod_interface_push.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/module_mod_interface_push.yml) [![docs.rs](https://img.shields.io/docsrs/mod_interface?color=e3e8f0&logo=docs.rs)](https://docs.rs/mod_interface) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)
<!--{ generate.module_header.end }-->

Provides the `mod_interface!` macro to define structured module interfaces with controlled visibility and propagation, simplifying the creation of layered architectures in Rust.

### Overview

The `mod_interface` crate introduces a procedural macro (`mod_interface!`) designed to streamline module organization in Rust projects. It helps address common challenges in maintaining complex codebases:

1.  **Structured Interfaces**: Define clear boundaries and relationships between modules (layers) using predefined exposure levels. This promotes a layered architecture where visibility and propagation of items are explicitly controlled.
2.  **Reduced Boilerplate**: The macro automatically generates the necessary `use` statements and module structures based on simple directives, reducing manual effort and potential errors.
3.  **Improved Readability**: By encouraging the explicit definition of a module's interface and how its items are exposed, the crate helps make the codebase easier to understand, navigate, and refactor, reducing cognitive load.

It offers a convention-based approach to modularity, particularly useful for designing complex systems where clear structure and controlled visibility are paramount.

### Basic Concepts

In the `mod_interface` crate, the concepts of layers and namespaces are central to its modularity approach. Here's a refined explanation:

- **Namespaces**: These are standard Rust modules that help organize code into logical groups.
- **Layers**: A layer is a specialized module structured using `mod_interface!`. It contains a set of predefined submodules, referred to as **Exposure Levels**, which dictate how the contents of the module are propagated to parent layers.

The Exposure Levels within a layer determine the visibility and propagation scope:

| Level     | Propagation Scope             | Purpose                              |
| :-------- | :---------------------------- | :----------------------------------- |
| `private` | Internal only                 | Original definitions                 |
| `own`     | Layer only (no propagation)   | Layer-specific public items          |
| `orphan`  | Immediate parent              | Items for direct parent              |
| `exposed` | All ancestors                 | Items for hierarchy use              |
| `prelude` | All ancestors + intended glob | Core interface essentials (glob use) |

Developers should define all entities within the `private` submodule and then re-export them through the other four exposure levels (`own`, `orphan`, `exposed`, `prelude`) based on the desired propagation strategy.

### Syntax of `mod_interface` Macro

The `mod_interface` macro provides several directives to manage the relationships between layers and entities:

- **`layer <name>`**: Define and include `<name>.rs` (or `<name>/mod.rs`) as a child layer within the current module.
- **`use <path>`**: Integrate an existing module at `<path>` as a layer into the current module's interface.
- **`reuse <path>`**: Similar to `use`, integrates an existing module layer, potentially with slightly different propagation rules intended for reusing common interfaces.
- **`<level> use <item>`**: Re-export `<item>` (from `private` or elsewhere) into the specified exposure level (`own`, `orphan`, `exposed`, or `prelude`).
- **`<level> mod <name>`**: Define `<name>.rs` (or `<name>/mod.rs`) as a "micro module" and include its contents directly into the specified exposure level.

These directives provide flexibility in organizing and managing the modular structure of a Rust program, enhancing both readability and maintainability.

### Example: Using Layers and Entities

This example shows a parent module using a `child` layer, demonstrating how items propagate based on their assigned exposure level.

For a module to be used as a layer, it must contain the necessary exposure levels (`private`, `own`, `orphan`, `exposed`, `prelude`). The `mod_interface!` macro helps generate these.

```rust,ignore
use mod_interface::mod_interface;

// Define a module named `child`.
pub mod child
{

  // Define a private namespace for all its items.
  mod private
  {
    /// Only my thing. (Will be in `own`)
    pub fn my_thing() -> bool
    {
       true
    }
    /// Parent module should also has this thing. (Will be in `orphan`)
    pub fn orphan_thing() -> bool
    {
       true
    }
    /// This thing should be exposed. (Will be in `exposed`)
    pub fn exposed_thing() -> bool
    {
       true
    }
    /// This thing should be in prelude. (Will be in `prelude`)
    pub fn prelude_thing() -> bool
    {
       true
    }
  }

  // Use mod_interface to define the exposure levels for child's items
  crate::mod_interface!
  {
    own use my_thing;
    orphan use orphan_thing;
    exposed use exposed_thing;
    prelude use prelude_thing;
  }

}

// Parent module also needs a private namespace.
mod private {}

// Parent module uses the `child` layer.
crate::mod_interface!
{
  /// Use the child layer.
  use super::child;
}


// fn main() // Example usage demonstrating visibility:
{

  // `prelude_thing` is in `prelude`, so it propagates everywhere.
  assert!( child::prelude_thing(), "prelude thing of child is there" );
  assert!( prelude_thing(), "Accessible in parent's root via prelude propagation" );
  assert!( own::prelude_thing(), "Accessible in parent's own via prelude propagation" );
  assert!( orphan::prelude_thing(), "Accessible in parent's orphan via prelude propagation" );
  assert!( exposed::prelude_thing(), "Accessible in parent's exposed via prelude propagation" );
  assert!( prelude::prelude_thing(), "Accessible in parent's prelude via prelude propagation" );

  // `exposed_thing` is in `exposed`, propagates to all ancestors except their prelude.
  assert!( child::exposed_thing(), "exposed thing of child is there" );
  assert!( exposed_thing(), "Accessible in parent's root via exposed propagation" );
  assert!( own::exposed_thing(), "Accessible in parent's own via exposed propagation" );
  assert!( orphan::exposed_thing(), "Accessible in parent's orphan via exposed propagation" );
  assert!( exposed::exposed_thing(), "Accessible in parent's exposed via exposed propagation" );
  // assert!( prelude::exposed_thing(), "but not in parent's prelude" ); // Fails

  // `orphan_thing` is in `orphan`, propagates only to the immediate parent's root and `own`.
  assert!( child::orphan_thing(), "orphan thing of child is there" );
  assert!( orphan_thing(), "Accessible in parent's root via orphan propagation" );
  assert!( own::orphan_thing(), "Accessible in parent's own via orphan propagation" );
  // assert!( orphan::orphan_thing(), "but not in parent's orphan" ); // Fails
  // assert!( exposed::orphan_thing(), "and not in parent's exposed" ); // Fails
  // assert!( prelude::orphan_thing(), "and not in parent's prelude" ); // Fails

  // `my_thing` is in `own`, does not propagate.
  assert!( child::my_thing(), "own thing of child is only there" );
  // assert!( my_thing(), "and not here" ); // Fails
  // assert!( own::my_thing(), "and not here" ); // Fails
  // assert!( orphan::my_thing(), "and not here" ); // Fails
  // assert!( exposed::my_thing(), "and not here" ); // Fails
  // assert!( prelude::my_thing(), "and not here" ); // Fails

}

```

<details>
<summary>Click to see the code expanded by the macro</summary>

```rust,ignore
use mod_interface::mod_interface;

// Define a module named `child`
pub mod child
{
  // Define a private namespace for all its items.
  mod private
  {
    /// Only my thing. (Will be in `own`)
    pub fn my_thing() -> bool
    {
       true
    }
    /// Parent module should also has this thing. (Will be in `orphan`)
    pub fn orphan_thing() -> bool
    {
       true
    }
    /// This thing should be exposed. (Will be in `exposed`)
    pub fn exposed_thing() -> bool
    {
       true
    }
    /// This thing should be in prelude. (Will be in `prelude`)
    pub fn prelude_thing() -> bool
    {
       true
    }
  }

  // Use mod_interface to define the exposure levels for child's items
  /* crate::mod_interface! { own use my_thing; orphan use orphan_thing; exposed use exposed_thing; prelude use prelude_thing; } */
  // Expanded code generated by the macro:
  pub use own::*;
  /// Own namespace of the module.
  pub mod own
  {
      use super::*;
      pub use orphan::*;
      pub use private::my_thing;
  }
  /// Orphan namespace of the module.
  pub mod orphan
  {
      use super::*;
      pub use exposed::*;
      pub use private::orphan_thing;
  }
  /// Exposed namespace of the module.
  pub mod exposed
  {
      use super::*;
      pub use prelude::*;
      pub use private::exposed_thing;
  }
  /// Prelude to use essentials: `use my_module::prelude::*`.
  pub mod prelude
  {
      use super::*;
      pub use private::prelude_thing;
  }

}

// Parent module also needs a private namespace.
mod private {}

// Parent module uses the `child` layer.
/* crate::mod_interface! { use super::child; } */
// Expanded code generated by the macro:
pub use own::*;
/// Own namespace of the module.
#[ allow( unused_imports ) ]
pub mod own
{
    use super::*;
    pub use orphan::*;
    #[ doc( inline ) ]
    #[ allow( unused_imports ) ]
    #[ doc = " Use the child layer."]
    pub use super::child::orphan::*; // Items from child's orphan are pulled into parent's own
    #[ doc( inline ) ]
    #[ allow( unused_imports ) ]
    #[ doc = " Use the child layer."]
    pub use super::child; // The child module itself is available in parent's own
}
/// Orphan namespace of the module.
#[ allow( unused_imports ) ]
pub mod orphan
{
    use super::*;
    pub use exposed::*;
    // Child's orphan items do not propagate to parent's orphan
}
/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
    use super::*;
    pub use prelude::*;
    #[ doc( inline ) ]
    #[ allow( unused_imports ) ]
    #[ doc = " Use the child layer."]
    pub use super::child::exposed::*; // Items from child's exposed are pulled into parent's exposed
}
/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
    use super::*;
    #[ doc( inline ) ]
    #[ allow( unused_imports ) ]
    #[ doc = " Use the child layer."]
    pub use super::child::prelude::*; // Items from child's prelude are pulled into parent's prelude
}


// fn main() // Example usage demonstrating visibility:
{

  // `prelude_thing` is in `prelude`, so it propagates everywhere.
  assert!( child::prelude_thing(), "prelude thing of child is there" );
  assert!( prelude_thing(), "Accessible in parent's root via prelude propagation" );
  assert!( own::prelude_thing(), "Accessible in parent's own via prelude propagation" );
  assert!( orphan::prelude_thing(), "Accessible in parent's orphan via prelude propagation" );
  assert!( exposed::prelude_thing(), "Accessible in parent's exposed via prelude propagation" );
  assert!( prelude::prelude_thing(), "Accessible in parent's prelude via prelude propagation" );

  // `exposed_thing` is in `exposed`, propagates to all ancestors except their prelude.
  assert!( child::exposed_thing(), "exposed thing of child is there" );
  assert!( exposed_thing(), "Accessible in parent's root via exposed propagation" );
  assert!( own::exposed_thing(), "Accessible in parent's own via exposed propagation" );
  assert!( orphan::exposed_thing(), "Accessible in parent's orphan via exposed propagation" );
  assert!( exposed::exposed_thing(), "Accessible in parent's exposed via exposed propagation" );
  // assert!( prelude::exposed_thing(), "but not in parent's prelude" ); // Fails

  // `orphan_thing` is in `orphan`, propagates only to the immediate parent's root and `own`.
  assert!( child::orphan_thing(), "orphan thing of child is there" );
  assert!( orphan_thing(), "Accessible in parent's root via orphan propagation" );
  assert!( own::orphan_thing(), "Accessible in parent's own via orphan propagation" );
  // assert!( orphan::orphan_thing(), "but not in parent's orphan" ); // Fails
  // assert!( exposed::orphan_thing(), "and not in parent's exposed" ); // Fails
  // assert!( prelude::orphan_thing(), "and not in parent's prelude" ); // Fails

  // `my_thing` is in `own`, does not propagate.
  assert!( child::my_thing(), "own thing of child is only there" );
  // assert!( my_thing(), "and not here" ); // Fails
  // assert!( own::my_thing(), "and not here" ); // Fails
  // assert!( orphan::my_thing(), "and not here" ); // Fails
  // assert!( exposed::my_thing(), "and not here" ); // Fails
  // assert!( prelude::my_thing(), "and not here" ); // Fails

}

```

</details>

### Debugging

To debug module interface use directive `#![ debug ]` in macro `mod_interface`. Let's update the main file of the example :

```rust,ignore
mod_interface::mod_interface!
{
  #![ debug ]
  /// Inner.
  layer child; // Or `use super::child;` if defined separately
}
```

Full sample see at [sample directory](https://github.com/Wandalen/wTools/tree/master/examples/mod_interface_trivial).

### To add to your project

```sh
cargo add mod_interface
```

### Try out from the repository

```sh
git clone https://github.com/Wandalen/wTools
cd wTools
cd examples/mod_interface_trivial
cargo run
```
### Try out from the repository

```sh
git clone https://github.com/Wandalen/wTools
cd wTools
cd examples/mod_interface_trivial
cargo run
```