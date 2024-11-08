
use super::*;

/// Private namespace of the module.
mod private
{
}

/// layer_a
pub mod layer_a;
/// layer_b
pub mod layer_b;

/// Own namespace of the module.
#[ allow( unused_imports ) ]
pub mod own
{
  use super::*;
  #[ doc( inline ) ]
  pub use orphan::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::layer_a::orphan::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::layer_b::orphan::*;
  #[ doc( inline ) ]
  pub use super::layer_a;
  #[ doc( inline ) ]
  pub use super::layer_b;
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
  pub use exposed::*;
}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;
  #[ doc( inline ) ]
  pub use prelude::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::layer_a::exposed::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::layer_b::exposed::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::layer_a::prelude::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::layer_b::prelude::*;
}

//

include!( "../../only_test/layer_simple_only_test.rs" );
