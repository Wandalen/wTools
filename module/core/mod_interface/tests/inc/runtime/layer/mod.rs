
use super::*;

/// Private namespace of the module.
mod private
{
}

/// layer_a
pub mod layer_a;
/// layer_b
pub mod layer_b;

/// Protected namespace of the module.
pub mod protected
{
  #[ doc( inline ) ]
  pub use super::orphan::*;
  #[ doc( inline ) ]
  pub use super::layer_a::orphan::*;
  #[ doc( inline ) ]
  pub use super::layer_b::orphan::*;
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
  pub use super::layer_a::exposed::*;
  #[ doc( inline ) ]
  pub use super::layer_b::exposed::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  #[ doc( inline ) ]
  pub use super::layer_a::prelude::*;
  #[ doc( inline ) ]
  pub use super::layer_b::prelude::*;
}

//

include!( "../../only_test/layer_simple_only_test.rs" );
