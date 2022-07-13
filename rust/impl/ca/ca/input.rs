
pub( crate ) mod private
{
  use std::io;
  use std::io::Write;

  /// Ask use input from standard input.
  pub fn ask( request : &str ) -> String
  {
    let mut response = String::new();
    print!( "{} : ", request );
    io::stdout().flush().ok();
    io::stdin().read_line( &mut response ).ok();
    response.trim().to_string()
  }
}

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

/// Namespace of the module to include with `use module::*`.
pub mod prelude
{
  pub use super::private::ask;
}

