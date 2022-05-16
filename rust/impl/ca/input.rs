
mod internal
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
  use super::internal as i;

  pub use i::ask;
}

pub use protected::*;

/// Exposed namespace of the module.
pub mod exposed
{
  use super::internal as i;

  pub use i::ask;
}

/// Namespace of the module to include with `use module::*`.
pub mod prelude
{
  use super::internal as i;

  pub use i::ask;
}

