//! Wrapper for command report representation.
//! Separates usage of cli-table library behind facade for convenient changes in future.

use cli_table ::
{
  format :: { Border, HorizontalLine, Separator }, Cell, Style, Table, TableDisplay
};

/// Wrapper struct for cli-table table with implementation of Display.
/// Separates usage of cli-table library behind facade for convenient changes in future.
pub struct ReportTable( TableDisplay );

impl core ::fmt ::Display for ReportTable
{
  fn fmt( &self, f: &mut core ::fmt ::Formatter< '_ > ) -> core ::fmt ::Result
  {
  write!( f, "{}", self.0 )
 }
}

impl core ::fmt ::Debug for ReportTable
{
  fn fmt( &self, f: &mut core ::fmt ::Formatter< '_ > ) -> core ::fmt ::Result
  {
  write!( f, "{}", self.0 )
 }
}

/// Transform 2-dimensional vec of String data into displayable table with plain rows.
#[must_use] 
pub fn plain_table( rows: Vec< Vec< String > > ) -> Option< ReportTable >
{
  let rows = rows
  .into_iter()
  .map( | row | row.into_iter().map( cli_table::Cell::cell ).collect :: < Vec< _ > >() )
  .collect :: < Vec< _ > >()
  ;

  let table_struct = rows.table()
  .border( Border ::builder().build() )
  .separator( Separator ::builder().build() )
  ;

  table_struct.display().map( ReportTable ).ok()
}

/// Create displayable table with header from headers vec and 2-dimensional vec of String data.
#[must_use] 
pub fn table_with_headers( headers: Vec< String >, rows: Vec< Vec< String > > ) -> Option< ReportTable >
{
  let rows = rows
  .into_iter()
  .map( | row | row.into_iter().map( cli_table::Cell::cell ).collect :: < Vec< _ > >() )
  .collect :: < Vec< _ > >()
  ;

  let headers = headers
  .into_iter()
  .map( | cell_val | cell_val.cell().bold( true ) )
  .collect :: < Vec< _ > >()
  ;

  let table_struct = rows.table()
  .title( headers )
  .border( Border ::builder().build() )
  .separator( Separator ::builder().build() )
  ;

  table_struct.display().map( ReportTable ).ok()
}

/// Transform 2-dimensional vec of String data into displayable table with plain rows and bottom border.
#[must_use] 
pub fn plain_with_border( rows: Vec< Vec< String > > ) -> Option< ReportTable >
{
  let rows = rows
  .into_iter()
  .map( | row | row.into_iter().map( cli_table::Cell::cell ).collect :: < Vec< _ > >() )
  .collect :: < Vec< _ > >()
  ;

  let table_struct = rows.table()
  .border( Border ::builder().bottom(HorizontalLine ::default()).build() )
  .separator( Separator ::builder().build() )
  ;

  table_struct.display().map( ReportTable ).ok()
}