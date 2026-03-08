/// Configuration validation error
#[ derive( Debug, Clone ) ]
pub struct ValidationError
{
  /// Parameter name that failed validation
  pub parameter : String,
  /// Validation error message
  pub message : String,
}

impl ValidationError
{
  /// Create new validation error
  #[ inline ]
  pub fn new( parameter : impl Into< String >, message : impl Into< String > ) -> Self
  {
    Self
    {
      parameter : parameter.into(),
      message : message.into(),
    }
  }
}

impl core::fmt::Display for ValidationError
{
  #[ inline ]
  fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result
  {
    write!( f, "Validation error for '{}': {}", self.parameter, self.message )
  }
}

impl core::error::Error for ValidationError {}
