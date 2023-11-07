/// Internal namespace.
pub( crate ) mod private
{
  use crate::wtools::error::Result;

  /// Command output.
  #[ derive( Debug ) ]
  pub struct Output
  {
    /// What was executing.
    pub context: String,
    /// List of actions was processed.
    pub actions: Vec< Action >,
  }

  /// Action output.
  #[ derive( Debug ) ]
  pub struct Action
  {
    /// What was executing.
    pub context: String,
    /// Stdout.
    pub out: String,
    /// Stderr.
    pub err: String,
  }

  impl Action
  {
    /// Create action output with `std::process::Output`.
    pub fn with_output< S >( context : S, output : std::process::Output ) -> Result< Self >
    where
      S : Into< String >,
    {
      Ok
      (
        Self
        {
          context: context.into(),
          out: String::from_utf8( output.stdout )?,
          err: String::from_utf8( output.stderr )?,
        }
      )
    }
  }
}

//

crate::mod_interface!
{
  protected( crate ) use Output;
  protected( crate ) use Action;
}
