<!-- {{# generate.module_header{} #}} -->

# Module :: mod_interface
<!--{ generate.module_header.start() }-->
 [![experimental](https://raster.shields.io/static/v1?label=&message=experimental&color=orange)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/module_mod_interface_push.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/module_mod_interface_push.yml) [![docs.rs](https://img.shields.io/docsrs/mod_interface?color=e3e8f0&logo=docs.rs)](https://docs.rs/mod_interface) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)
<!--{ generate.module_header.end }-->

Protocol of modularity unifying interface of a module and introducing layers.

### Basic use-case

Library file with code `inner.rs`:

```rust ignore
pub( crate ) mod private
{
  /// Routine of inner module.
  pub fn inner_is() -> bool
  {
    true
  }
}

//

mod_interface::mod_interface!
{
  prelude use inner_is;
}
```

Main file that generates modules and namespaces `main.rs` :

```rust ignore
use mod_interface::mod_interface;

//

fn main()
{
  assert_eq!( prelude::inner_is(), inner::prelude::inner_is() );
}

//

mod_interface::mod_interface!
{
  /// Inner.
  layer inner;
}
```

It generates code :

```rust
use mod_interface::mod_interface;

//

fn main()
{
  assert_eq!( prelude::inner_is(), inner::prelude::inner_is() );
}

//

/// Inner.
pub mod inner
{
  pub( crate ) mod private
  {
    /// Routine of inner module.
    pub fn inner_is() -> bool { true }
  }

  /// Own namespace of the module.
  pub mod own
  {
    pub use orphan::*;
  }
  pub use own::*;

  /// Orphan namespace of the module.
  pub mod orphan
  {
    pub use exposed::*;
  }

  /// Exposed namespace of the module.
  pub mod exposed
  {
    pub use prelude::*;
  }

  /// Prelude to use essentials: `use my_module::prelude::*`.
  pub mod prelude
  {
    pub use private::inner_is;
  }
}

/// Own namespace of the module.
#[ allow( unused_imports ) ]
pub mod own
{
  use super::*;
  pub use orphan::*;
  pub use super::inner::orphan::*;
}
pub use own::*;

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
  pub use super::inner::exposed::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;
  pub use super::inner::prelude::*;
}
```

### Debugging

To debug module interface use directive `#![ debug ]` in macro `mod_interface`. Let's update the main file of the example :

```rust ignore
mod_interface::mod_interface!
{
  #![ debug ]
  /// Inner.
  layer inner;
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
