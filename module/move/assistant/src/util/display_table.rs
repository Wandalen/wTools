//!
//! Function for displaying tabular data according to `TableConfig`.
//!

mod private
{
  
  use std::fmt;

  use format_tools::
  {
    TableFormatter,
    output_format,
    print,
    TableOutputFormat,
  };

  use crate::*;
  use commands::{ TableConfig };

  /// Function for displaying tabular data according to `TableConfig`.
  pub fn display_tabular_data<'a>
  (
    data : &'a impl TableFormatter< 'a >,
    f : &mut fmt::Formatter< '_ >,
    table_config : &'a TableConfig,
  ) -> fmt::Result
  {
    if table_config.as_table 
    {
      display_table( data, f, table_config )
    }
    else if table_config.as_records
    {
      display_records( data, f, table_config )
    }
    else if table_config.columns
    {
      display_columns( data, f, table_config )
    }
    else
    {
      display_table( data, f, table_config )
    }
  }

  fn display_table<'a>
  (
    data : &'a impl TableFormatter< 'a >,
    f : &mut fmt::Formatter< '_ >,
    table_config : &'a TableConfig,
  ) -> fmt::Result
  {
    let mut format = output_format::Table::default();
    format.max_width = table_config.max_table_width;

    display_data
    ( 
      data, 
      f, 
      format,
      &table_config.filter_columns,
    )
  }

  fn display_records<'a>
  (
    data : &'a impl TableFormatter< 'a >,
    f : &mut fmt::Formatter< '_ >,
    table_config : &'a TableConfig,
  ) -> fmt::Result
  {
    let mut format = output_format::Records::default();
    format.max_width = table_config.max_table_width;

    display_data
    ( 
      data, 
      f, 
      format,
      &table_config.filter_columns,
    )
  }

  fn display_columns<'a>
  (
    data : &'a impl TableFormatter< 'a >,
    f : &mut fmt::Formatter< '_ >,
    table_config : &'a TableConfig,
  ) -> fmt::Result
  {
    let mut format = output_format::Records::default();
    format.max_width = table_config.max_table_width;

    display_data
    ( 
      data, 
      f, 
      format,
      &table_config.filter_columns,
    )
  }

  fn display_data<'a>
  (
    data : &'a impl TableFormatter< 'a >,
    f : &mut fmt::Formatter< '_ >,
    format : impl TableOutputFormat,
    filter_columns : &'a Vec< String >,
  ) -> fmt::Result
  {
    let mut printer = print::Printer::with_format( &format );
    let binding = | title : &str |
    {
      filter_columns.is_empty() || filter_columns.iter().any( |c| c.as_str() == title )
    };
    printer.filter_col = &binding;

    let mut context = print::Context::new( f, printer );
    TableFormatter::fmt( data, &mut context )
  }

}

crate::mod_interface!
{
  own use display_tabular_data;
}