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

//

crate::mod_interface!
{
  exposed use ask;
}
