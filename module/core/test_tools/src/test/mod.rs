
//!
//! Tools for testing.
//!

mod private {}

// // #[ cfg( not( feature = "no_std" ) ) ]
// crate::mod_interface!
// {
//   layer asset;
//   layer compiletime;
//   layer helper;
//   layer smoke_test;
//   layer version;
// }

pub mod asset;
pub mod compiletime;
pub mod helper;
pub mod smoke_test;
pub mod version;

#[ cfg( feature = "enabled" ) ]
#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use own::*;

/// Own namespace of the module.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod own
{
  use super::*;

  #[ doc( inline ) ]
  pub use orphan::*;

  #[ doc( inline ) ]
  pub use
  {
    asset::orphan::*,
    compiletime::orphan::*,
    helper::orphan::*,
    smoke_test::orphan::*,
    version::orphan::*,
  };

}

/// Shared with parent namespace of the module
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod orphan
{
  use super::*;

  #[ doc( inline ) ]
  pub use exposed::*;

}

/// Exposed namespace of the module.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;

  #[ doc( inline ) ]
  pub use prelude::*;

  #[ doc( inline ) ]
  pub use
  {
    asset::exposed::*,
    compiletime::exposed::*,
    helper::exposed::*,
    smoke_test::exposed::*,
    version::exposed::*,
  };

  pub use meta_tools::
  {
    impls,
    index,
    tests_impls,
    tests_impls_optional,
    tests_index,
  };

}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ cfg( feature = "enabled" ) ]
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;

  #[ doc( inline ) ]
  pub use
  {
    asset::prelude::*,
    compiletime::prelude::*,
    helper::prelude::*,
    smoke_test::prelude::*,
    version::prelude::*,
  };

}
