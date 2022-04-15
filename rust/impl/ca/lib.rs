
/// Common macroses to fill modules.
pub mod common;

/// Handle commands.
pub mod command;

/// Get input.
pub mod input;

/// Handle instruction from input.
pub mod instruction;

/// Exposed namespace of the module.
pub mod exposed
{
  pub use super::command::exposed::*;
  pub use super::input::exposed::*;
  pub use super::instruction::exposed::*;
}

pub use exposed::*;

/// Namespace of the module to include with `use module::*`.
pub mod prelude
{
  pub use super::command::prelude::*;
  pub use super::input::prelude::*;
  pub use super::instruction::prelude::*;
}
