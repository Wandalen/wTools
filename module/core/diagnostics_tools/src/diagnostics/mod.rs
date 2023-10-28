
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
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::orphan::*;
  #[ cfg( feature = "runtime_assertions" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::rta::orphan::*;
  #[ cfg( feature = "compiletime_assertions" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::cta::orphan::*;
}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use protected::*;

/// Orphan namespace of the module.
pub mod orphan
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
pub mod exposed
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::prelude::*;
  #[ cfg( feature = "runtime_assertions" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::rta::exposed::*;
  #[ cfg( feature = "compiletime_assertions" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::cta::exposed::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  #[ cfg( feature = "runtime_assertions" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::rta::prelude::*;
  #[ cfg( feature = "compiletime_assertions" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::cta::prelude::*;
}
