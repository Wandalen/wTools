
pub( crate ) mod private
{
}

#[ cfg( feature = "runtime_assertions" ) ]
/// Run-time assertions.
pub mod rta;
#[ cfg( feature = "compiletime_assertions" ) ]
/// Compile-time assertions.
pub mod cta;

/// Protected namespace of the module.
pub mod protected
{
  pub use super::orphan::*;
  #[ cfg( feature = "runtime_assertions" ) ]
  pub use super::rta::orphan::*;
  #[ cfg( feature = "compiletime_assertions" ) ]
  pub use super::cta::orphan::*;
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
  pub use super::prelude::*;
  #[ cfg( feature = "runtime_assertions" ) ]
  pub use super::rta::exposed::*;
  #[ cfg( feature = "compiletime_assertions" ) ]
  pub use super::cta::exposed::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  #[ cfg( feature = "runtime_assertions" ) ]
  pub use super::rta::prelude::*;
  #[ cfg( feature = "compiletime_assertions" ) ]
  pub use super::cta::prelude::*;
}
