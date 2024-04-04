//! Wrapper for command report representation.
//! Separates usage of cli-table library behind facade for convenient changes in future.

use cli_table::
{
  format::{ Border, HorizontalLine, Separator }, Cell, Style, Table, TableDisplay
};

// qqq : purpose well defined should be always be in documentation
// aaa : added explanation

/// Wrapper struct for cli-table table with implementation of Display.
/// Separates usage of cli-table library behind facade for convenient changes in future.
pub struct ReportTable( TableDisplay );

impl std::fmt::Display for ReportTable
{
  fn fmt( &self, f : &mut std::fmt::Formatter<'_> ) -> std::fmt::Result
  {
    write!( f, "{}", self.0 )
  }
}

impl std::fmt::Debug for ReportTable
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

/// Transform 2-dimensional vec of String data into displayable table with plain rows and bottom border.
pub fn plain_with_border( rows : Vec< Vec< String > > ) -> Option< ReportTable >
{
  let rows = rows
  .into_iter()
  .map( | row | row.into_iter().map( | cell_val | cell_val.cell() ).collect::< Vec< _ > >() )
  .collect::< Vec< _ > >()
  ;

  let table_struct = rows.table()
  .border( Border::builder().bottom(HorizontalLine::default()).build() )
  .separator( Separator::builder().build() )
  ;

  table_struct.display().map( | table | ReportTable( table ) ).ok()
}