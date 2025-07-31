//! Error handling module for willbe.

/// Namespace with dependencies.
pub mod dependency {
  pub use ::error_tools::dependency::*;
}

/// Own namespace of the module.
pub mod own {
  use super::*;
  pub use orphan::*;
}

/// Orphan namespace of the module.
pub mod orphan {
  use super::*;
  pub use exposed::*;
}

/// Exposed namespace of the module.
pub mod exposed {
  pub use ::error_tools::*;
  pub use ::error_tools::typed;
  pub use ::error_tools::untyped;
  
  // Re-export standard library Result and Option
  pub use std::result::Result;
  pub use std::option::Option;
}

/// Prelude of the module.
pub mod prelude {
  pub use ::error_tools::prelude::*;
}

// Direct re-exports for common usage
pub use ::error_tools::*;
pub use ::error_tools::typed;
pub use ::error_tools::untyped;

// Re-export standard library Result and Option for pattern matching
pub use std::result::Result;
pub use std::option::Option;