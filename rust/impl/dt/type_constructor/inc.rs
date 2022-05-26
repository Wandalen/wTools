
/// Type constructor of many.
#[ cfg( feature = "types" ) ]
pub mod many;
/// Type constructor of pair.
#[ cfg( feature = "types" ) ]
pub mod pair;
/// Type constructor of single.
#[ cfg( feature = "types" ) ]
pub mod single;
/// Type constructors.
#[ cfg( feature = "types" ) ]
pub mod types;

/// Generic traits.
pub mod traits;
/// Variadic constructor.
#[ cfg( feature = "make" ) ]
pub mod make;

/// Protected namespace of the module.
pub mod protected
{
  #[ doc( inline ) ]
  pub use super::orphan::*;
  #[ doc( inline ) ]
  #[ cfg( feature = "types" ) ]
  pub use super::many::orphan::*;
  #[ doc( inline ) ]
  #[ cfg( feature = "types" ) ]
  pub use super::pair::orphan::*;
  #[ doc( inline ) ]
  #[ cfg( feature = "types" ) ]
  pub use super::single::orphan::*;
  #[ doc( inline ) ]
  #[ cfg( feature = "types" ) ]
  pub use super::types::orphan::*;
  #[ doc( inline ) ]
  pub use super::traits::orphan::*;
  #[ doc( inline ) ]
  #[ cfg( feature = "make" ) ]
  pub use super::make::orphan::*;
}

#[ doc( inline ) ]
pub use protected::*;

/// Orphan namespace of the module.
pub mod orphan
{
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
pub mod exposed
{
  #[ doc( inline ) ]
  pub use super::prelude::*;
  #[ doc( inline ) ]
  #[ cfg( feature = "types" ) ]
  pub use super::many::exposed::*;
  #[ doc( inline ) ]
  #[ cfg( feature = "types" ) ]
  pub use super::pair::exposed::*;
  #[ doc( inline ) ]
  #[ cfg( feature = "types" ) ]
  pub use super::single::exposed::*;
  #[ doc( inline ) ]
  #[ cfg( feature = "types" ) ]
  pub use super::types::exposed::*;
  #[ doc( inline ) ]
  pub use super::traits::exposed::*;
  #[ doc( inline ) ]
  #[ cfg( feature = "make" ) ]
  pub use super::make::exposed::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  #[ doc( inline ) ]
  #[ cfg( feature = "types" ) ]
  pub use super::many::prelude::*;
  #[ doc( inline ) ]
  #[ cfg( feature = "types" ) ]
  pub use super::pair::prelude::*;
  #[ doc( inline ) ]
  #[ cfg( feature = "types" ) ]
  pub use super::single::prelude::*;
  #[ doc( inline ) ]
  #[ cfg( feature = "types" ) ]
  pub use super::types::prelude::*;
  #[ doc( inline ) ]
  pub use super::traits::prelude::*;
  #[ doc( inline ) ]
  #[ cfg( feature = "make" ) ]
  pub use super::make::prelude::*;
}
