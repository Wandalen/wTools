use cli_table::
{
  format::{ Border, Separator }, Cell, Style, Table, TableDisplay
};

pub struct ReportTable( TableDisplay );

impl std::fmt::Display for ReportTable
{
  fn fmt( &self, f : &mut std::fmt::Formatter<'_> ) -> std::fmt::Result
  {
    write!( f, "{}", self.0 )
  }
}

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