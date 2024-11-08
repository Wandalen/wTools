<!-- {{# generate.module_header{} #}} -->

# Module :: mod_interface
<!--{ generate.module_header.start() }-->
 [![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/module_mod_interface_push.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/module_mod_interface_push.yml) [![docs.rs](https://img.shields.io/docsrs/mod_interface?color=e3e8f0&logo=docs.rs)](https://docs.rs/mod_interface) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)
<!--{ generate.module_header.end }-->

Protocol of modularity unifying interface.

### Problem Solved

The `mod_interface` crate provides a structured approach to modularity, addressing two key challenges in software development:

1. **Meaningful Namespace Structuring**: The crate enables developers to organize program entities into meaningful namespaces ( read modules ) without additional development overhead. This is achieved through a set of auto-importing rules and a flexible inversion of control mechanism, allowing parent layers ( namespaces or modules ) to delegate control over its items to child layers. This approach ensures that each namespace is self-contained and meaningful, promoting better organization and modularity.

2. **Enhanced Readability and Tooling Independence**: By requiring a `mod private` section that lists all items ( read functions, structures, traits, types ) the `mod_interface` macro encourages developers to create a concise list of items at the beginning or end of a file. This improves readability, encourages refactoring, and reduces cognitive load by providing a clear, high-level grouping of items. Code tooling is not always reliable and can sometimes be counterproductive by automating tasks that should be done manually to achieve more concise code. While code tooling like `rust_analyzer` are useful, this approach minimizes reliance on them, making the program's structure easier to understand and manage.

While some may argue that inversion of control over namespaces may not always achieve the desired outcome, and code tooling can be sufficient, the `mod_interface` crate offers a cathartic solution for designing complex systems where tooling and triditional structuring often fall short. By promoting a clear and organized structure, it helps developers grasp the semantics of their programs more holistically.

### Basic Concepts

In the `mod_interface` crate, the concepts of layers and namespaces are central to its modularity approach. Here's a refined explanation:

- **Namespaces**: These are standard Rust modules that help organize code into logical groups.
- **Layers**: A layer is a specialized module that contains a set of predefined submodules, referred to as chapters. These chapters dictate how the contents of the module are propagated to parent layers.

The chapters within a layer are:

- **Private Chapter (`private`)**: This is where all the code and entities are initially defined. It is not accessible outside the module.
- **Public Chapter (`own`)**: Contains items that are not propagated to any parent layers. They remain within the module.
- **Public Chapter (`orphan`)**: Shares its contents with the immediate parent layer only.
- **Public Chapter (`exposed`)**: Propagates its contents to all parent layers, making them accessible throughout the hierarchy.
- **Public Chapter (`prelude`)**: Similar to `exposed`, but also serves as a recommendation for other crates to implicitly import its contents, akin to the prelude in the [Rust standard library](https://doc.rust-lang.org/std/prelude/index.html).

Developers should define all entities within the `private` chapter and then re-export them through the other four chapters based on the desired propagation strategy.

### Syntax of `mod_interface` Macro

The `mod_interface` macro provides several directives to manage the relationships between layers and entities:

- **`layer <layer1>`**: Establishes a relationship where the current layer uses a child layer.
- **`use <layer1>`**: Allows the current layer to use another layer defined elsewhere.
- **`reuse <layer1>`**: Enables the current layer to reuse a layer defined anywhere, promoting code reuse.
- **`<stategy> use <entity1>`**: Allows the current layer to use an entity defined anywhere, with the specified promotion stategy (`<stategy>`).

These directives provide flexibility in organizing and managing the modular structure of a Rust program, enhancing both readability and maintainability.

### Example: Using Layers and Entities

In this example, we demonstrate the basic use case of one layer utilizing another layer. For a module to be used as a layer, it must contain all the necessary chapters: `orphan`, `exposed`, and `prelude`. Generally, a layer should also have the `own` and `private` chapters, but these are typically not modified directly by the user unless explicitly defined, with the `private` chapter remaining inaccessible from outside the module.

Below is a simple example where a parent layer imports a `child` layer. The `child` layer defines several functions, each with a different propagation strategy, resulting in each function being placed in a different chapter of the parent layer, while some functions do not reach the parent layer at all.

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

// Priave namespaces is necessary.
mod private {}

pub use own::*;

/// Own namespace of the module.
#[ allow( unused_imports ) ]
pub mod own
{
  use super::*;
  pub use orphan::*;
  pub use child::orphan::*;
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
  pub use child::exposed::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;
  pub use child::prelude::*;
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

As you can see:

- The content of the `prelude` chapter is automatically propagated into the `exposed` chapter of the same layer.
- The content of the `exposed` chapter is automatically propagated into the `orphan` chapter of the same layer.
- The content of the `orphan` chapter is automatically propagated into the `own` chapter of the same layer.
- The content of the `own` chapter is not automatically propagated anywhere.

### `layer <layer1>` vs `use <layer1>`

The `use <layer1>;` syntax is used to import a layer from anywhere within the project or even from external crates. This provides flexibility in how layers are organized, as the layer being used does not need to be a direct submodule of the current module. It allows you to bring any accessible layer into the current scope without enforcing a specific file structure. The visibility of the imported layer remains as it is defined in its original location, and this syntax does not inherently change that visibility.

In contrast, the `layer <layer1>` syntax is used to establish a hierarchical relationship where the current module uses a child layer. This requires the child layer to be a direct submodule, meaning it must be physically present in the file structure as a submodule. The `layer <layer1>` syntax implies `pub mod layer1`, making the child layer publicly accessible as a submodule. This enforces a specific organizational structure, where the child layer is part of the current module's hierarchy, and its contents are directly accessible according to the defined propagation strategies.

Thus, `layer <layer1>` acts as a shortcut, combining the definition of a reference to a module file and using it, while `use <layer1>` uses a module that is already defined somewhere, not necessarily in the same crate.

### `reuse <layer1>` vs `use <layer1>`

The `reuse <layer1>` syntax treats the child layer as an integral part of the parent layer, so the normal rules of propagation do not apply to the content of the child layer. Specifically, the `own` chapter of the child layer is imported into the `own` chapter of the parent layer, and the `orphan` chapter of the child layer is imported into the `orphan` chapter of the parent layer.

In contrast, `use <layer1>` follows the standard propagation rules:

- `child::own` is not propagated.
- `child::orphan` is imported into `parent::own`.
- `child::exposed` is imported into `parent::exposed`.
- `child::prelude` is imported into `parent::prelude`.

For `reuse <layer1>`, the propagation is as follows:

- `child::own` is imported into `parent::own`.
- `child::orphan` is imported into `parent::orphan`.
- `child::exposed` is imported into `parent::exposed`.
- `child::prelude` is imported into `parent::prelude`.

`reusing` does not impact parent of parent or child of child.

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
