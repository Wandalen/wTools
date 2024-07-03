//!
//! Type constructors of fundamental data types.
//!

/// Type constructor of many.
#[ cfg
(
  all
  (
    feature = "many",
    any( not( feature = "no_std" ), feature = "use_alloc" ),
  )
)]
pub mod many;
/// Type constructor of many.
#[ cfg
(
  any
  (
    not( feature = "many" ),
    all( feature = "no_std", not( feature = "use_alloc" ) ),
  )
)]
#[ path = "./no_many.rs" ]
pub mod many;

/// Type constructor of pair.
pub mod pair;
/// Type constructor of single.
pub mod single;
/// Type constructors.
pub mod types;
/// Macro helpers.
pub mod helper;

/// From/Into traits.
#[ cfg( feature = "vectorized_from" ) ]
pub mod vectorized_from;
/// Generic traits.
pub mod traits;
/// Traits Enumerable.
pub mod enumerable;
/// Variadic constructor.
#[ cfg( feature = "make" ) ]
pub mod make;

/// Own namespace of the module.
#[ allow( unused_imports ) ]
pub mod own
{
  use super::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use orphan::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::many::orphan::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::pair::orphan::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::single::orphan::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::types::orphan::*;

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  #[ cfg( feature = "vectorized_from" ) ]
  pub use super::vectorized_from::orphan::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::helper::orphan::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::traits::orphan::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::enumerable::orphan::*;
  // #[ doc( inline ) ]
  // #[ allow( unused_imports ) ]
  // #[ cfg( feature = "make" ) ]
  // pub use super::make::orphan::*;
}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use own::*;

/// Orphan namespace of the module.
#[ allow( unused_imports ) ]
pub mod orphan
{
  use super::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use exposed::*;
}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use prelude::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::many::exposed::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::pair::exposed::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::single::exposed::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::types::exposed::*;

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  #[ cfg( feature = "vectorized_from" ) ]
  pub use super::vectorized_from::exposed::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::helper::exposed::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::traits::exposed::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::enumerable::exposed::*;
  // #[ doc( inline ) ]
  // #[ allow( unused_imports ) ]
  // #[ cfg( feature = "make" ) ]
  // pub use super::make::exposed::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::many::prelude::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::pair::prelude::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::single::prelude::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::types::prelude::*;

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  #[ cfg( feature = "vectorized_from" ) ]
  pub use super::vectorized_from::prelude::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::helper::prelude::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::traits::prelude::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::enumerable::prelude::*;
  // #[ doc( inline ) ]
  // #[ allow( unused_imports ) ]
  // #[ cfg( feature = "make" ) ]
  // pub use super::make::prelude::*;
}
