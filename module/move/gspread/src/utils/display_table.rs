//!
//! Module with functions to display HTTP requests results in a table view.
//! 

mod private
{
  use std::fmt;
  use format_tools::
  {
    TableFormatter,
    print,
    output_format,
    TableOutputFormat
  };

  /// # `display_rows`
  ///
  /// Displays rows of data in a table view.
  ///
  /// This function calls `display_data` internally to format and render the data in a tabular format.
  ///
  /// ## Parameters:
  /// - `data`:  
  ///   A reference to an object implementing the `TableFormatter` trait, which provides the data to display.
  /// - `f`:  
  ///   A mutable reference to a `fmt::Formatter` used for formatting the output.
  ///
  /// ## Returns:
  /// - `fmt::Result`:  
  pub fn display_rows< 'a >
  (
    data :  &'a impl TableFormatter< 'a >,
    f : &mut fmt::Formatter< '_ >
  ) -> fmt::Result
  {
    display_data( data, f, output_format::Table::default() )
  }

  /// # `display_header`
  ///
  /// Displays the header of a table view.
  ///
  /// This function calls `display_data` internally to format and render the header in a tabular format.
  ///
  /// ## Parameters:
  /// - `data`:  
  ///   A reference to an object implementing the `TableFormatter` trait, which provides the header data to display.
  /// - `f`:  
  ///   A mutable reference to a `fmt::Formatter` used for formatting the output.
  ///
  /// ## Returns:
  /// - `fmt::Result`:  
  pub fn display_header < 'a >
  (
    data : &'a impl TableFormatter< 'a >,
    f : &mut fmt::Formatter< '_ >
  ) -> fmt::Result
  {
    display_data( data, f, output_format::Table::default() )
  }

  /// # `display_data`
  ///
  /// Displays data in a table view with a specific output format.
  ///
  /// This function creates a printer and context objects and delegates the rendering logic to `TableFormatter::fmt`.
  ///
  /// ## Parameters:
  /// - `data`:  
  ///   A reference to an object implementing the `TableFormatter` trait, which provides the data to display.
  /// - `f`:  
  ///   A mutable reference to a `fmt::Formatter` used for formatting the output.
  /// - `format`:  
  ///   An object implementing the `TableOutputFormat` trait, defining the desired output format for the table.
  ///
  /// ## Returns:
  /// - `fmt::Result`:  
  pub fn display_data < 'a >
  (
    data : &'a impl TableFormatter< 'a >,
    f : &mut fmt::Formatter< '_ >,
    format : impl TableOutputFormat,
  ) -> fmt::Result
  {
    let printer = print::Printer::with_format( &format );
    let mut context = print::Context::new( f, printer );
    TableFormatter::fmt( data, &mut context )
  }

}

crate::mod_interface!
{
  own use
  {
    display_rows,
    display_header
  };
}