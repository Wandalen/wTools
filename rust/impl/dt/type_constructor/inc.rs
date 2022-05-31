
/// Type constructor of many.
#[ cfg
(
  all
  (
    feature = "many",
    any( feature = "use_std", feature = "use_alloc" ),
  )
)]
pub mod many;
/// Type constructor of many.
#[ cfg
(
  any
  (
    not( feature = "many" ),
    all( not( feature = "use_std" ), not( feature = "use_alloc" ) ),
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

  pub use super::many::orphan::*;
  #[ doc( inline ) ]

  pub use super::pair::orphan::*;
  #[ doc( inline ) ]

  pub use super::single::orphan::*;
  #[ doc( inline ) ]

  pub use super::types::orphan::*;
  #[ doc( inline ) ]
  pub use super::helper::orphan::*;
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

  pub use super::many::exposed::*;
  #[ doc( inline ) ]

  pub use super::pair::exposed::*;
  #[ doc( inline ) ]

  pub use super::single::exposed::*;
  #[ doc( inline ) ]

  pub use super::types::exposed::*;
  #[ doc( inline ) ]
  pub use super::helper::exposed::*;
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

  pub use super::many::prelude::*;
  #[ doc( inline ) ]

  pub use super::pair::prelude::*;
  #[ doc( inline ) ]

  pub use super::single::prelude::*;
  #[ doc( inline ) ]

  pub use super::types::prelude::*;
  #[ doc( inline ) ]
  pub use super::helper::prelude::*;
  #[ doc( inline ) ]
  pub use super::traits::prelude::*;
  #[ doc( inline ) ]
  #[ cfg( feature = "make" ) ]
  pub use super::make::prelude::*;
}
