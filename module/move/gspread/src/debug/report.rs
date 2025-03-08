

mod private
{
  use std::fmt;
  use format_tools::AsTable;

  use crate::*;
  use debug::RowWrapper;
  use utils::display_table::display_rows;

  /// # Report
  ///
  /// A structure to display retrieved rows in the console using `format_tools`.
  ///
  /// ## Fields:
  /// - `rows`:  
  ///   A `Vec< RowWrapper >` containing the rows to be displayed.
  ///
  /// ## Usage:
  /// This structure is used in conjunction with the `fmt::Display` trait to render rows in a formatted table view.
  pub struct Report
  {
    pub rows : Vec< RowWrapper >
  }

  impl fmt::Display for Report
  {
    /// Formats the rows for display by calling the `display_rows` function,
    /// which uses appropriate functions from `format_tools`.
    ///
    /// ## Parameters:
    /// - `f`:  
    ///   A mutable reference to the `fmt::Formatter` used to write the formatted output.
    ///
    /// ## Returns:
    /// - `fmt::Result`:  
    fn fmt
    (
      &self,
      f : &mut fmt::Formatter
    ) -> fmt::Result
    {
      display_rows( &AsTable::new( &self.rows ), f )
    }
  }
}

crate::mod_interface!
{
  orphan use
  {
    Report
  };
}