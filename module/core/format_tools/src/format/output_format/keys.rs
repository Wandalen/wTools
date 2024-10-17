//! Implement keys list output format.
//!
//! # Example
//!
//! ```text
//! ```
//!

use crate::*;
use print::
{
  InputExtract,
  Context,
};
use core::
{
  fmt,
};
use std::sync::OnceLock;

/// A struct representing the list of keys output format.
#[derive( Debug )]
pub struct Keys
{
  // /// Prefix added to each row.
  // pub table_prefix : String,
  // /// Postfix added to each row.
  // pub table_postfix : String,
  // /// Separator used between rows.
  // pub table_separator : String,
  // /// Prefix added to each row.
  // pub row_prefix : String,
  // /// Postfix added to each row.
  // pub row_postfix : String,
  // /// Separator used between rows.
  // pub row_separator : String,
  // /// Prefix added to each cell.
  // pub cell_prefix : String,
  // /// Postfix added to each cell.
  // pub cell_postfix : String,
  // /// Separator used between table columns.
  // pub cell_separator : String,
}

impl Keys
{
  /// Returns a reference to a static instance of `Keys`.
  pub fn instance() -> &'static dyn TableOutputFormat
  {
    static INSTANCE : OnceLock< Keys > = OnceLock::new();
    INSTANCE.get_or_init( || Keys::default() )
  }
}

impl Default for Keys
{
  fn default() -> Self
  {

    // let cell_prefix = "".to_string();
    // let cell_postfix = "".to_string();
    // let cell_separator = " │ ".to_string();
    // let row_prefix = "│ ".to_string();
    // let row_postfix = " │".to_string();
    // let row_separator = "\n".to_string();
    // let table_prefix = "".to_string();
    // let table_postfix = "".to_string();
    // let table_separator = "\n".to_string();

    Self
    {
      // table_prefix,
      // table_postfix,
      // table_separator,
      // row_prefix,
      // row_postfix,
      // row_separator,
      // cell_prefix,
      // cell_postfix,
      // cell_separator,
    }
  }
}

impl TableOutputFormat for Keys
{

  fn extract_write< 'buf, 'data >(
    &self,
    x : &InputExtract< 'data >,
    c : &mut Context< 'buf >,
  ) -> fmt::Result
  {

    // dbg!( &x );

    for col in &x.col_descriptors
    {
      write!( c.buf, " - {}\n", col.label )?;
    }

    write!( c.buf, "  {} fields\n", x.col_descriptors.len() )?;

    Ok(())
  }

}
