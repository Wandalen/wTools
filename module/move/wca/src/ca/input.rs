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

  /// A structure representing an input with a single string value.
  ///
  /// This struct is designed to encapsulate a single piece of input data as a `String`.
  /// It provides a simple wrapper that can be used to convert various types of string
  /// representations into a uniform `Input` struct.
  #[ derive( Debug ) ]
  pub struct Input( pub String );

  /// A trait for converting various types into `Input`.
  ///
  /// The `IntoInput` trait defines a method `into_input` for converting an implementing type
  /// into the `Input` struct. This allows for a flexible way of handling different string
  /// representations and aggregating them into a single `Input` type.
  pub trait IntoInput
  {
    /// Converts the implementing type into an `Input` instance.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// let string_input: &str = "example string";
    /// let input_struct = string_input.into_input();
    ///
    /// let owned_string_input: String = "owned example".to_string();
    /// let owned_input_struct = owned_string_input.into_input();
    /// ```
    fn into_input( self ) -> Input;
  }

  impl IntoInput for &str
  {

    fn into_input( self ) -> Input
    {
      Input( self.to_string() )
    }
  }

  impl IntoInput for String
  {
    fn into_input( self ) -> Input
    {
      Input( self )
    }
  }

  impl IntoInput for Vec< String >
  {
    fn into_input( self ) -> Input
    {
      Input( self.join( " " ) )
    }
  }

}

//

crate::mod_interface!
{
  exposed use ask;
  exposed use Input;
  exposed use IntoInput;
}
