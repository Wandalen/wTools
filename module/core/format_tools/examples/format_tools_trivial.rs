//!
//! Using the `to_string_with_fallback` macro
//! to convert values to strings with a primary and fallback formatting method.
//!

fn main()
{
  // Import necessary traits and the macro from the `format_tools` crate.
  use core::fmt;
  use format_tools::
  {
    WithDebug,
    WithDisplay,
    to_string_with_fallback,
  };

  // Define a struct that implements both Debug and Display traits.
  struct Both;

  // Implement the Debug trait for the Both struct.
  impl fmt::Debug for Both
  {
    fn fmt( &self, f : &mut fmt::Formatter< '_ > ) -> fmt::Result
    {
      write!( f, "This is debug" )
    }
  }

  // Implement the Display trait for the Both struct.
  impl fmt::Display for Both
  {
    fn fmt( &self, f : &mut fmt::Formatter< '_ > ) -> fmt::Result
    {
      write!( f, "This is display" )
    }
  }

  // Define a struct that implements only the Debug trait.
  struct OnlyDebug;

  // Implement the Debug trait for the OnlyDebug struct.
  impl fmt::Debug for OnlyDebug
  {
    fn fmt( &self, f : &mut fmt::Formatter< '_ > ) -> fmt::Result
    {
      write!( f, "This is debug" )
    }
  }

  // Example usage: Using Both which implements both Debug and Display.
  let src = Both;
  // Convert the struct to a string using `to_string_with_fallback` macro.
  // The primary formatting method WithDisplay is used.
  let got = to_string_with_fallback!( WithDisplay, WithDebug, &src );
  let exp = "This is display".to_string();
  // Assert that the result matches the expected value.
  assert_eq!( got, exp );

  // Example usage: Using OnlyDebug which implements only Debug.
  let src = OnlyDebug;
  // Convert the struct to a string using `to_string_with_fallback` macro.
  // The primary formatting method WithDisplay is not available, so the fallback WithDebug is used.
  let got = to_string_with_fallback!( WithDisplay, WithDebug, &src );
  let exp = "This is debug".to_string();
  // Assert that the result matches the expected value.
  assert_eq!( got, exp );

}