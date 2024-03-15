//! Helper for command report representation.

use cli_table::
{
  format::{ Border, Separator }, Cell, Style, Table, TableDisplay
};

/// Wrapper struct for cli-table table with iplementation of Display.
pub struct ReportTable( TableDisplay );

impl std::fmt::Display for ReportTable
{
  fn fmt( &self, f : &mut std::fmt::Formatter<'_> ) -> std::fmt::Result
  {
    write!( f, "{}", self.0 )
  }
}

/// Transform 2-dimensional vec of String data into displayable table with plain rows.
pub fn plain_table( rows : Vec< Vec< String > > ) -> Option< ReportTable >
{
  let rows = rows
  .into_iter()
  .map( | row | row.into_iter().map( | cell_val | cell_val.cell() ).collect::< Vec< _ > >() )
  .collect::< Vec< _ > >()
  ;

  let table_struct = rows.table()
  .border( Border::builder().build() )
  .separator( Separator::builder().build() )
  ;

  table_struct.display().map( | table | ReportTable( table ) ).ok()
}

/// Create displayable table with header from headers vec and 2-dimensional vec of String data.
pub fn table_with_headers( headers : Vec< String >, rows : Vec< Vec< String > > ) -> Option< ReportTable >
{
  let rows = rows
  .into_iter()
  .map( | row | row.into_iter().map( | cell_val | cell_val.cell() ).collect::< Vec< _ > >() )
  .collect::< Vec< _ > >()
  ;

  let headers = headers
  .into_iter()
  .map( | cell_val | cell_val.cell().bold( true ) )
  .collect::< Vec< _ > >()
  ;

  let table_struct = rows.table()
  .title( headers )
  .border( Border::builder().build() )
  .separator( Separator::builder().build() )
  ;

  table_struct.display().map( | table | ReportTable( table ) ).ok()
}