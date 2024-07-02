
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
pub use own::*;

/// Own namespace of the module.
#[ allow( unused_imports ) ]
pub mod own
{
  use super::*;
  #[ doc( inline ) ]
  pub use orphan::*;
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
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;
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
