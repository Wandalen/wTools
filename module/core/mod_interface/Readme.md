<!-- {{# generate.module_header{} #}} -->

# Module :: mod_interface
<!--{ generate.module_header.start() }-->
 [![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/module_mod_interface_push.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/module_mod_interface_push.yml) [![docs.rs](https://img.shields.io/docsrs/mod_interface?color=e3e8f0&logo=docs.rs)](https://docs.rs/mod_interface) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)
<!--{ generate.module_header.end }-->

Protocol of modularity unifying interface.

### Problem Solved

The `mod_interface` crate provides a structured approach to modularity, addressing two key challenges in software development:

1. **Meaningful Namespace Structuring**: The crate enables developers to organize program entities into meaningful namespaces ( read modules ) without additional development overhead. This is achieved through a set of auto-importing rules and a flexible inversion of control mechanism, allowing parent namespaces to delegate control over its items to child namespaces. This approach ensures that each namespace is self-contained and meaningful, promoting better organization and modularity.

2. **Enhanced Readability and Tooling Independence**: By requiring a `mod private` section that lists all items ( read functions, structures, traits, types ) the `mod_interface` macro encourages developers to create a concise list of items at the beginning or end of a file. This improves readability, encourages refactoring, and reduces cognitive load by providing a clear, high-level grouping of items. Code tooling is not always reliable and can sometimes be counterproductive by automating tasks that should be done manually to achieve more concise code. While code tooling like `rust_analyzer` are useful, this approach minimizes reliance on them, making the program's structure easier to understand and manage.

While some may argue that inversion of control over namespaces may not always achieve the desired outcome, and code tooling can be sufficient, the `mod_interface` crate offers a cathartic solution for designing complex systems where tooling and triditional structuring often fall short. By promoting a clear and organized structure, it helps developers grasp the semantics of their programs more holistically.

### Example : Trivial

This example demonstrates how to use the `mod_interface` crate to organize a Rust program into structured namespaces. The code is divided into a library file (`child.rs`) and a main function. The library file defines a module with private functions and uses the `mod_interface` macro to specify which functions should be exposed in different namespaces. The main function then tests the visibility and accessibility of these functions.

```rust
use mod_interface::mod_interface;

// Define a module named `child`.
pub mod child
{

  // Define a private namespace for all its items.
  mod private
  {
    /// Only my thing.
    pub fn my_thing() -> bool { true }
    /// Parent module should also has this thing.
    pub fn orphan_thing() -> bool { true }
    /// This thing should be exposed.
    pub fn exposed_thing() -> bool { true }
    /// This thing should be in prelude.
    pub fn prelude_thing() -> bool { true }
  }

  //

  crate::mod_interface!
  {
    own use my_thing;
    orphan use orphan_thing;
    exposed use exposed_thing;
    prelude use prelude_thing;
  }

}

// Priave namespaces is necessary.
mod private {}

crate::mod_interface!
{
  /// Inner.
  use super::child;
}


fn main()
{

  assert!( child::prelude_thing(), "prelude thing of child is there" );
  assert!( prelude_thing(), "and here" );
  assert!( own::prelude_thing(), "and here" );
  assert!( orphan::prelude_thing(), "and here" );
  assert!( exposed::prelude_thing(), "and here" );
  assert!( prelude::prelude_thing(), "and here" );

  assert!( child::exposed_thing(), "exposed thing of child is there" );
  assert!( exposed_thing(), "and here" );
  assert!( own::exposed_thing(), "and here" );
  assert!( orphan::exposed_thing(), "and here" );
  assert!( exposed::exposed_thing(), "and here" );
  // assert!( prelude::exposed_thing(), "but not here" );

  assert!( child::orphan_thing(), "orphan thing of child is there" );
  assert!( orphan_thing(), "orphan thing of child is here" );
  assert!( own::orphan_thing(), "and here" );
  // assert!( orphan::orphan_thing(), "but not here" );
  // assert!( exposed::orphan_thing(), "and not here" );
  // assert!( prelude::orphan_thing(), "and not here" );

  assert!( child::my_thing(), "own thing of child is only there" );
  // assert!( my_thing(), "and not here" );
  // assert!( own::my_thing(), "and not here" );
  // assert!( orphan::my_thing(), "and not here" );
  // assert!( exposed::my_thing(), "and not here" );
  // assert!( prelude::my_thing(), "and not here" );

}

```

<details>
<summary>The code above will be expanded to this</summary>

```rust
use mod_interface::mod_interface;

// Define a module named `child`
pub mod child
{
  // Define a private namespace for all its items.
  mod private
  {
    /// Only my thing.
    pub fn my_thing() -> bool { true }
    /// Parent module should also has this thing.
    pub fn orphan_thing() -> bool { true }
    /// This thing should be exposed.
    pub fn exposed_thing() -> bool { true }
    /// This thing should be in prelude.
    pub fn prelude_thing() -> bool { true }
  }

  pub use own::*;

  /// Own namespace of the module.
  pub mod own
  {
    pub use super::orphan::*;
    pub use super::private::my_thing;
  }

  /// Orphan namespace of the module.
  pub mod orphan
  {
    pub use super::exposed::*;
    pub use super::private::orphan_thing;
  }

  /// Exposed namespace of the module.
  pub mod exposed
  {
    pub use super::prelude::*;
    pub use super::private::exposed_thing;
  }

  /// Prelude to use essentials: `use my_module::prelude::*`.
  pub mod prelude
  {
    pub use super::private::prelude_thing;
  }
}

// Priave namespaces is necessary.
mod private {}

pub use own::*;

/// Own namespace of the module.
#[ allow( unused_imports ) ]
pub mod own
{
  use super::*;
  pub use orphan::*;
  pub use super::child::orphan::*;
  pub use super::child;
}

/// Orphan namespace of the module.
#[ allow( unused_imports ) ]
pub mod orphan
{
  use super::*;
  pub use exposed::*;
}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;
  pub use prelude::*;
  pub use super::child::exposed::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;
  pub use super::child::prelude::*;
}

//

fn main()
{

  assert!( child::prelude_thing(), "prelude thing of child is there" );
  assert!( prelude_thing(), "and here" );
  assert!( own::prelude_thing(), "and here" );
  assert!( orphan::prelude_thing(), "and here" );
  assert!( exposed::prelude_thing(), "and here" );
  assert!( prelude::prelude_thing(), "and here" );

  assert!( child::exposed_thing(), "exposed thing of child is there" );
  assert!( exposed_thing(), "and here" );
  assert!( own::exposed_thing(), "and here" );
  assert!( orphan::exposed_thing(), "and here" );
  assert!( exposed::exposed_thing(), "and here" );
  // assert!( prelude::exposed_thing(), "but not here" );

  assert!( child::orphan_thing(), "orphan thing of child is there" );
  assert!( orphan_thing(), "orphan thing of child is here" );
  assert!( own::orphan_thing(), "and here" );
  // assert!( orphan::orphan_thing(), "but not here" );
  // assert!( exposed::orphan_thing(), "and not here" );
  // assert!( prelude::orphan_thing(), "and not here" );

  assert!( child::my_thing(), "own thing of child is only there" );
  // assert!( my_thing(), "and not here" );
  // assert!( own::my_thing(), "and not here" );
  // assert!( orphan::my_thing(), "and not here" );
  // assert!( exposed::my_thing(), "and not here" );
  // assert!( prelude::my_thing(), "and not here" );

}

```

</details>

### Debugging

To debug module interface use directive `#![ debug ]` in macro `mod_interface`. Let's update the main file of the example :

```rust ignore
mod_interface::mod_interface!
{
  #![ debug ]
  /// Inner.
  layer child;
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
