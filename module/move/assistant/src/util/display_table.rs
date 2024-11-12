//!
//! Function for displaying tabular data according to `TableConfig`.
//!

use std::fmt;

mod private
{
  
  use std::
  {
    fmt,
    borrow::Cow
  };

  use format_tools::
  {
    AsTable,
    TableFormatter,
    output_format,
    print,
    TableOutputFormat,
  };

  use crate::*;
  use commands::{ TableConfig, TableStyle };

  /// Function for displaying tabular data according to `TableConfig`.
  pub fn display_tabular_data<'a>
  (
    data: &impl Fields< &'a str, Option< Cow< 'a, str > > >,
    f : &mut fmt::Formatter< '_ >,
    table_config : &TableConfig,
  ) -> fmt::Result
  {
      match table_config.style
      {
        TableStyle::Table =>
        {
          display_table( data, f, &table_config.filter_columns )
        }

        TableStyle::AsRecords =>
        {
          display_records( data, f, &table_config.filter_columns )
        }

        TableStyle::Columns =>
        {
          display_columns( data, f, &table_config.filter_columns )
        }
      }
  }

  fn display_table<'a>
  (
    data : &impl Fields< &'a str, Option< Cow< 'a, str > > >,
    f : &mut fmt::Formatter< '_ >,
    filter_columns : &Vec< String >,
  ) -> fmt::Result
  {
    display_data( data, f, filter_columns, output_format::Table::default() )
  }

  fn display_records<'a>
  (
    data : &impl Fields< &'a str, Option< Cow< 'a, str > > >,
    f : &mut fmt::Formatter< '_ >,
    filter_columns : &Vec< String >,
  ) -> fmt::Result
  {
    display_data( data, f, filter_columns, output_format::Records::default() )
  }

  fn display_columns<'a>
  (
    data : &impl Fields< &'a str, Option< Cow< 'a, str > > >,
    f : &mut fmt::Formatter< '_ >,
    filter_columns : &Vec< String >,
  ) -> fmt::Result
  {
    display_data( data, f, filter_columns, output_format::Keys::default() )
  }

  fn display_data<'a>
  (
    data : &impl Fields< &'a str, Option< Cow< 'a, str > > >,
    f : &mut fmt::Formatter< '_ >,
    filter_columns : &Vec< String >,
    format : impl TableOutputFormat,
  ) -> fmt::Result
  {
    let mut printer = print::Printer::with_format( &format );
    printer.filter_col = &| title : &str |
    {
      filter_columns.iter().any( |c| c.as_str() == title )
    };

    let as_table = AsTable::new( &data );
    let mut context = print::Context::new( f, printer );
    TableFormatter::fmt( &as_table, &mut context )
  }

}

crate::mod_interface!
{
  own use display_tabular_data;
}