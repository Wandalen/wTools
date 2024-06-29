
pub( crate ) mod private
{
}

#[ cfg( feature = "diagnostics_runtime_assertions" ) ]
/// Run-time assertions.
pub mod rta;
#[ cfg( feature = "diagnostics_compiletime_assertions" ) ]
/// Compile-time assertions.
pub mod cta;
/// Compile-time asserting of memory layout.
#[ cfg( feature = "diagnostics_memory_layout" ) ]
pub mod layout;

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use protected::*;

/// Protected namespace of the module.
pub mod protected
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::orphan::*;
  #[ cfg( feature = "diagnostics_runtime_assertions" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::rta::orphan::*;
  #[ cfg( feature = "diagnostics_compiletime_assertions" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::cta::orphan::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  #[ cfg( feature = "diagnostics_memory_layout" ) ]
  pub use super::layout::orphan::*;
}

/// Orphan namespace of the module.
pub mod orphan
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::exposed::*;
}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::prelude::*;
  #[ cfg( feature = "diagnostics_runtime_assertions" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::rta::exposed::*;
  #[ cfg( feature = "diagnostics_compiletime_assertions" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::cta::exposed::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  #[ cfg( feature = "diagnostics_memory_layout" ) ]
  pub use super::layout::exposed::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{
  #[ cfg( feature = "diagnostics_runtime_assertions" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::rta::prelude::*;
  #[ cfg( feature = "diagnostics_compiletime_assertions" ) ]
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::cta::prelude::*;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  #[ cfg( feature = "diagnostics_memory_layout" ) ]
  pub use super::layout::prelude::*;
}
