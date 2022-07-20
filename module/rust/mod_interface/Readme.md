<!-- {{# generate.module_header{} #}} -->

# Module :: mod_interface
[![experimental](https://raster.shields.io/static/v1?label=stability&message=experimental&color=orange&logoColor=eee)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/ModuleModInterfacePush.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/ModuleModInterfacePush.yml) [![docs.rs](https://img.shields.io/docsrs/mod_interface?color=e3e8f0&logo=docs.rs)](https://docs.rs/mod_interface) [![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=sample%2Frust%2Fmod_interface_trivial_sample%2Fsrc%2Fmain.rs,RUN_POSTFIX=--example%20mod_interface_trivial_sample/https://github.com/Wandalen/wTools) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)

Protocol of modularity unifying interface of a module and introducing layers.

### Sample

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
mod_interface::mod_interface!
{
  /// Inner.
  layer inner;
}

//

fn main()
{
  /* test public namespaces */
  assert_eq!( prelude::inner_is(), true );
  assert_eq!( exposed::inner_is(), true );
  assert_eq!( orphan::inner_is(), true );
  assert_eq!( protected::inner_is(), true );

  /* test public module `inner` */
  assert_eq!( inner::prelude::inner_is(), true );
  assert_eq!( inner::exposed::inner_is(), true );
  assert_eq!( inner::orphan::inner_is(), true );
  assert_eq!( inner::protected::inner_is(), true );
}
```

It generates code :

```rust
/// Inner.
pub mod inner
{
  pub( crate ) mod private
  {
    /// Routine of inner module.
    pub fn inner_is() -> bool { true }
  }

  /// Protected namespace of the module.
  pub mod protected
  {
    #[ doc( inline ) ]
    pub use super::orphan::*;
  }
  #[ doc( inline ) ]
  pub use protected::*;

  /// Orphan namespace of the module.
  pub mod orphan
  {
    #[ doc( inline ) ]
    pub use super::exposed::*;
  }

  /// Exposed namespace of the module.
  pub mod exposed
  {
    #[ doc( inline ) ]
    pub use super::prelude::*;
  }

  /// Prelude to use essentials: `use my_module::prelude::*`.
  pub mod prelude
  {
    #[ doc( inline ) ]
    pub use super::private::inner_is;
  }
}

/// Protected namespace of the module.
pub mod protected
{
  #[ doc( inline ) ]
  pub use super::orphan::*;
  #[ doc( inline ) ]
  pub use super::inner::orphan::*;
}
#[ doc( inline ) ]
pub use protected::*;

/// Orphan namespace of the module.
pub mod orphan
{
  #[ doc( inline ) ]
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
pub mod exposed
{
  #[ doc( inline ) ]
  pub use super::prelude::*;
  #[ doc( inline ) ]
  pub use super::inner::exposed::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  #[ doc( inline ) ]
  pub use super::inner::prelude::*;
}

fn main()
{
  /* test public namespaces */
  assert_eq!( prelude::inner_is(), true );
  assert_eq!( exposed::inner_is(), true );
  assert_eq!( orphan::inner_is(), true );
  assert_eq!( protected::inner_is(), true );

  /* test public module `inner` */
  assert_eq!( inner::prelude::inner_is(), true );
  assert_eq!( inner::exposed::inner_is(), true );
  assert_eq!( inner::orphan::inner_is(), true );
  assert_eq!( inner::protected::inner_is(), true );
}
```

To debug module interface use directive `#![ debug ]` in macro `mod_interface`. Let's update the main file of the example :

```rust ignore
mod_interface::mod_interface!
{
  #![ debug ]
  /// Inner.
  layer inner;
}
```

The directive adds stdout output with module structure. For the case above output looks :

```ignore
 = original :

#! [debug] /// Inner.
layer inner ;


 = result :

#[doc = " Inner."] pub mod inner ; #[doc(inline)] pub use protected :: * ;
#[doc = r" Protected namespace of the module."] pub mod protected
{
    #[doc(inline)] pub use super :: orphan :: * ; #[doc(inline)]
    #[doc = " Inner."] pub use super :: inner :: orphan :: * ;
} #[doc = r" Orphan namespace of the module."] pub mod orphan
{ #[doc(inline)] pub use super :: exposed :: * ; }
#[doc = r" Exposed namespace of the module."] pub mod exposed
{
    #[doc(inline)] pub use super :: prelude :: * ; #[doc(inline)]
    #[doc = " Inner."] pub use super :: inner :: exposed :: * ;
} #[doc = r" Prelude to use essentials: `use my_module::prelude::*`."] pub mod
prelude
{ #[doc(inline)] #[doc = " Inner."] pub use super :: inner :: prelude :: * ; }
```

<!-- xxx : rewrite -->
<!-- aaa : Dmytro : added new samples -->

Full sample see at [sample directory](https://github.com/Wandalen/wTools/tree/master/sample/rust/mod_interface_trivial_sample).

### To add to your project

```sh
cargo add mod_interface
```

### Try out from the repository

```sh
git clone https://github.com/Wandalen/wTools
cd wTools
cd sample/rust/mod_interface_trivial
cargo run
```
