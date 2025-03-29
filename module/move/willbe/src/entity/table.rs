#[ allow( clippy::std_instead_of_alloc, clippy::std_instead_of_core ) ]
mod private
{
  use std::fmt::{Display, Formatter};

  /// An owned printable table.
  #[ derive( Debug ) ]
  pub struct Table
  {
    inner : prettytable::Table,
  }

  impl Display for Table
  {
    fn fmt( &self, f : &mut Formatter< '_ > ) -> std::fmt::Result
    {
      writeln!( f, "{}", self.inner )
    }
  }

  impl Table
  {
    /// Create an empty table.
    #[ must_use ]
    pub fn new() -> Self
    {
      Self
      {
        inner : prettytable::Table::new(),
      }
    }
  }

  impl Table
  {
    /// Set the optional header.
    pub fn set_header(&mut self, row : Row )
    {
      self.inner.set_titles( row.inner );
    }

    /// Append a row in the table.
    pub fn add_row(&mut self, row : Row )
    {
      self.inner.add_row( row.inner );
    }
  }

  impl Default for Table
  {
    fn default() -> Self
    {
      let mut table = Self::new();
      let format = default_format();
      table.inner.set_format( format );
      table
    }
  }

  fn default_format() -> prettytable::format::TableFormat
  {
    prettytable::format::FormatBuilder::new()
    .column_separator( ' ' )
    .borders( ' ' )
    .separators
    (
      &[ prettytable::format::LinePosition::Title ],
      prettytable::format::LineSeparator::new( '-', '+', '+', '+' )
    )
    .padding( 1, 1 )
    .build()
  }

  /// Represent a table row made of cells.
  #[ derive( Debug ) ]
  pub struct Row
  {
    inner : prettytable::Row,
  }

  impl Row
  {

    /// Append a cell at the end of the row.
    pub fn add_cell( &mut self, title : &str )
    {
      let mut cell = prettytable::Cell::new( title );
      cell.align( prettytable::format::Alignment::CENTER );
      self.inner.add_cell( prettytable::Cell::new( title ) );
    }
  }

  #[ allow( clippy::new_without_default ) ]
  impl Row
  {
    /// Create an row of length size, with empty strings stored.
    #[ must_use ]
    pub fn new() -> Self
    {
      Self
      {
        inner : prettytable::Row::empty(),
      }
    }
  }
}

crate::mod_interface!
{
  own use Table;
  own use Row;
}