mod private
{
  use std::io::{ self, Write };

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
  /// This struct is designed to encapsulate a single piece of input data as a `Vec< String >`.
  /// It provides a simple wrapper that can be used to convert various types of string
  /// representations into a uniform `Input` struct.
  #[ derive( Debug ) ]
  pub struct Input( pub Vec< String > );

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
    /// use wca::IntoInput;
    ///
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
      Input( self.split( ' ' ).map( ToString::to_string ).collect() )
    }
  }

  impl IntoInput for String
  {
    fn into_input( self ) -> Input
    {
      Input( self.split( ' ' ).map( ToString::to_string ).collect() )
    }
  }

  impl IntoInput for Vec< String >
  {
    fn into_input( self ) -> Input
    {
      Input( self )
    }
  }

}

//

crate::mod_interface!
{
  exposed use ask;
  orphan use Input;
  orphan use IntoInput;
}
