//! Customizable format of printing table.
//!
//! # Example of table format
//!
//! ```text
//!  sid | sname | gap
//! -----+-------+-----
//!    3 | Alice |   5
//!    6 | Joe   |   1
//!   10 | Boris |   5
//! ```
//!
//! # Example of list of rows format.
//!
//! ```text
//! -[ RECORD 1 ]
//! sid   | 3
//! sname | Alice
//! gap   | 5
//! -[ RECORD 2 ]
//! sid   | 6
//! sname | Joe
//! gap   | 1
//! -[ RECORD 3 ]
//! sid   | 10
//! sname | Boris
//! gap   | 5
//! ```
//!

/// Define a private namespace for all its items.
mod private
{

  use std::borrow::Cow;

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

  //=

  /// Trait for converting table extracts into string representations.
  ///
  /// `TableOutputFormat` defines the method for formatting table data
  /// and writing it into a specified buffer, providing flexibility in
  /// output style and format.
  ///
  pub trait TableOutputFormat
  {
    /// Formats the table extract and writes it into the destination buffer.
    ///
    /// # Parameters
    /// - `x`: The `InputExtract` containing table data to be formatted.
    /// - `c`: The `Context` holding the buffer and styles for formatting.
    ///
    /// # Returns
    /// A `fmt::Result` indicating success or failure of the write operation.
    fn extract_write< 'buf, 'data >
    (
      &self,
      x : &InputExtract< 'data >,
      c : &mut Context< 'buf >,
    ) -> fmt::Result;
  }

  impl Default for &'static dyn TableOutputFormat
  {
    #[ inline( always ) ]
    fn default() -> Self
    {
      super::table::Table::instance()
    }
  }

  /// Print table, which is constructed with vectors and `Cow`s, with the
  /// specified output formatter.
  ///
  /// This function is useful when you do not want to use `AsTable`, or implement `Fields`, and
  /// other traits, but you just have string slices in vectors.
  ///
  /// `rows` should not contain header of the table, it will be automatically added if `has_header`
  /// is true.
  pub fn vector_table_write< 'data, 'context >
  (
    column_names : Vec< Cow< 'data, str > >,
    has_header : bool,
    rows : Vec< Vec< Cow< 'data, str > > >,
    c : &mut Context< 'context >,
  ) -> fmt::Result
  {
    InputExtract::extract_from_raw_table
    ( 
      column_names,
      has_header,
      rows,
      c.printer.filter_col,
      c.printer.filter_row,
      | x |
      {
        c.printer.output_format.extract_write( x, c )
      }
    )
  }

}

mod table;
mod records;
mod keys;

#[ allow( unused_imports ) ]
pub use own::*;

/// Own namespace of the module.
#[ allow( unused_imports ) ]
pub mod own
{
  use super::*;
  #[ doc( inline ) ]
  pub use orphan::*;

  #[ doc( inline ) ]
  pub use
  {
    table::Table,
    records::Records,
    keys::Keys,
  };

  #[ doc( inline ) ]
  pub use private::
  {
    vector_table_write,
  };

}

/// Orphan namespace of the module.
#[ allow( unused_imports ) ]
pub mod orphan
{
  use super::*;
  #[ doc( inline ) ]
  pub use exposed::*;
}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;
  pub use super::super::output_format;

  #[ doc( inline ) ]
  pub use private::
  {
    TableOutputFormat,
  };

}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;
}
