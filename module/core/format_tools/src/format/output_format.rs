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

  /*
  pub fn table_data_write< 'buf, 'data, 'context >
  (
    table : &Vec< Vec< Cow< 'data, str > > >,
    has_header : bool,
    filter_col : &'context ( dyn FilterCol + 'context ),
    filter_row : &'context ( dyn FilterRow + 'context ),
    output_format : impl TableOutputFormat,
    c : &mut Context< 'buf >,
  ) -> fmt::Result
  {
    struct TableWrapper< 'data >
    {
      has_header : bool,
      data : &'data Vec< Vec< Cow< 'data, str > > >,
    }

    struct CellsWrapper< 'data >
    {
      header : Option< &'data Vec< Cow< 'data, str
      data : &'data Vec< Cow< 'data, str > >,
    }

    impl Cells< str > for CellsWrapper
    {
      fn cells< 'a, 'b >( &'a self ) -> impl IteratorTrait< Item = ( &'b CellKey, Option< Cow< 'b, str > > ) >
      where
        'a : 'b,
        CellKey : 'b,
      {
        self.data.iter().map( | c |  )
      }
    }

    impl TableRows for TableWrapper
    {
      fn rows( &self ) -> impl IteratorTrait< Item = &Self::Row >
      {
        self.data.iter().skip( if self.has_header { 1 } else { 0 } )
      }
    }

    let wrapped_table = TableWrapper { has_header, data : table };

    InputExtract::extract( &wrapped_table, filter_col, filter_row, | x |
    {
      output_format.extract_write( x, c )
    })
  }
  */

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
