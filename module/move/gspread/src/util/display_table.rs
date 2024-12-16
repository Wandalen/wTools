

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

  pub fn display_rows< 'a >
  (
    data :  &'a impl TableFormatter< 'a >,
    f : &mut fmt::Formatter< '_ >
  ) -> fmt::Result
  {
    display_data( data, f, output_format::Table::default() )
  }

  pub fn display_header < 'a >
  (
    data : &'a impl TableFormatter< 'a >,
    f : &mut fmt::Formatter< '_ >
  ) -> fmt::Result
  {
    display_data( data, f, output_format::Table::default() )
  }

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