
use super::*;
use core::fmt;
use former::Former;

// ///
// /// A trait for converting any type that implements `Debug` into a `String`.
// ///
// pub trait DebugToString
// {
//   /// Converts the value into a `String` using its `Debug` representation.
//   ///
//   /// # Returns
//   /// A `String` containing the `Debug` representation of the value.
//   fn debug_to_string( &self ) -> String;
// }
//
// impl< T > DebugToString for T
// where
//   T : fmt::Debug,
// {
//   /// Converts the value into a `String` using its `Debug` representation.
//   ///
//   /// # Returns
//   ///
//   /// A `String` containing the `Debug` representation of the value.
//   ///
//   fn debug_to_string( &self ) -> String
//   {
//     format!( "{:?}", self )
//   }
// }

/// Struct to hold table options.
#[ derive( Debug, Default, Former ) ]
pub struct TableOptions
{
  /// Optional header row for the table.
  pub header : Option< Vec< String > >,
  /// Optional delimiter for separating table columns.
  pub delimiter : Option< String >,
}

impl TableOptions
{

  /// Creates a new instance of `TableOptions`.
  pub fn new( header : Option< Vec< String > >, delimiter : Option< String > ) -> Self
  {
    TableOptions { header, delimiter }
  }

  /// Function to print a table based on the iterator of items implementing `Fields` trait.
  pub fn perform< I, F, K, E >( &self, iter : I )
  where
    I : Iterator< Item = F >,
    F : Fields< K, E >,
    K : fmt::Debug,
    E : fmt::Debug,
  {
    let delimiter = self.delimiter.clone().unwrap_or_else( || ",".to_string() );

    // Print the header if provided
    if let Some( header ) = &self.header
    {
      println!( "{}", header.join( &delimiter ) );
    }

    // Print each row
    for item in iter
    {
      let fields : Vec< String > = item
      .fields()
      .map( |field| format!( "{:?}", field ) )
      .collect();

      println!( "{}", fields.join( &delimiter ) );
    }
  }

}
