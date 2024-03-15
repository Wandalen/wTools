use crate::*;
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
      rows.push( vec![ EMPTY_CELL.to_owned(), field[ 0 ].to_owned(), field[ 1 ].to_owned(), field[ 2 ].to_owned() ] );
    }

    let table = table_display::table_with_headers
    (
      vec!
      [
        EMPTY_CELL.to_owned(),
        "name".to_owned(),
        "type".to_owned(),
        "explanation".to_owned(),
      ],
      rows
    );

    if let Some( table ) = table
    {
      writeln!( f, "Frames fields:" )?;
      writeln!( f, "{}", table )?;
    }

    Ok( () )
  }
}

impl Report for FieldsReport {}