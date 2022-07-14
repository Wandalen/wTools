
/// Perform smoke testing.
#[ cfg( feature = "use_std" ) ]
mod smoke;

/// Init aggregator commands.
#[ cfg( feature = "use_std" ) ]
mod init;

/// Protected namespace of the module.
pub mod protected
{
  pub use super::orphan::*;
}

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
}

/// Prelude namespace of the module.
pub mod prelude
{
  #[ cfg( feature = "use_std" ) ]
  pub use super::smoke::*;
  #[ cfg( feature = "use_std" ) ]
  pub use super::init::*;
}
