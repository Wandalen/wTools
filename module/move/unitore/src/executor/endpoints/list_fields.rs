use crate::*;
use cli_table::{ format::{ Border, Separator }, Cell, Style, Table };
use executor::FeedManager;
use super::Report;
use storage::FeedStorage;
use error_tools::Result;

/// List all fields.
pub async fn list_fields(
  storage : FeedStorage< gluesql::sled_storage::SledStorage >,
  _args : &wca::Args,
) -> Result< impl Report >
{
  let mut manager = FeedManager::new( storage );
  manager.get_columns()
}

const EMPTY_CELL : &'static str = "";

#[ derive( Debug ) ]
pub struct FieldsReport
{
  pub fields_list : Vec< [ &'static str; 3 ] >,
}

impl std::fmt::Display for FieldsReport
{

  fn fmt( &self, f : &mut std::fmt::Formatter<'_> ) -> std::fmt::Result
  {
    let mut rows = Vec::new();
    for field in &self.fields_list
    {
      rows.push( vec![ EMPTY_CELL.cell(), field[ 0 ].cell(), field[ 1 ].cell(), field[ 2 ].cell() ] );
    }
    let table_struct = rows.table()
    .title( vec!
    [
      EMPTY_CELL.cell(),
      "name".cell().bold( true ),
      "type".cell().bold( true ),
      "explanation".cell().bold( true ),
    ] )
    .border( Border::builder().build() )
    .separator( Separator::builder().build() );

    let table = table_struct.display().unwrap();

    writeln!( f, "Frames fields:" )?;
    writeln!( f, "{}", table )?;

    Ok( () )
  }
}

impl Report for FieldsReport {}