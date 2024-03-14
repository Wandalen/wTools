use crate::*;
use cli_table::{ format::{ Border, Separator }, Cell, Style, Table };
use executor::FeedManager;
use gluesql::core::executor::Payload;
use super::Report;
use storage::{ FeedStorage, FeedStore };
use error_tools::{ err, BasicError, Result };

/// Get labels of column for specified table.
pub async fn list_columns(
  storage : FeedStorage< gluesql::sled_storage::SledStorage >,
  args : &wca::Args,
) -> Result< impl Report >
{
  let table_name = args
  .get_owned::< String >( 0 )
  .ok_or_else::< BasicError, _ >( || err!( "Cannot get Name argument for command .table.list" ) )?
  .into()
  ;

  let mut manager = FeedManager::new( storage );
  manager.storage.list_columns( table_name ).await
}

/// Get names of tables in storage.
pub async fn list_tables(
  storage : FeedStorage< gluesql::sled_storage::SledStorage >,
  _args : &wca::Args,
) -> Result< impl Report >
{
  let mut manager = FeedManager::new( storage );
  manager.storage.list_tables().await
}

const EMPTY_CELL : &'static str = "";

#[ derive( Debug ) ]
pub struct TablesReport
{
  tables : std::collections::HashMap< String, Vec< String > >
}

impl TablesReport
{
  pub fn new( payload : Vec< Payload > ) -> Self
  {
    let mut result = std::collections::HashMap::new();
    match &payload[ 0 ]
    {
      Payload::Select { labels: _label_vec, rows: rows_vec } =>
      {
        for row in rows_vec
        {
          let table = String::from( row[ 0 ].clone() );
          result.entry( table )
          .and_modify( | vec : &mut Vec< String > | vec.push( String::from( row[ 1 ].clone() ) ) )
          .or_insert( vec![ String::from( row[ 1 ].clone() ) ] )
          ;
        }
      },
      _ => {},
    }
    TablesReport{ tables : result }
  }
}

impl std::fmt::Display for TablesReport
{
  fn fmt( &self, f : &mut std::fmt::Formatter<'_> ) -> std::fmt::Result
  {
    writeln!( f, "Storage tables:" )?;
    let mut rows = Vec::new();
    for ( table_name, columns ) in &self.tables
    {
      let columns_str = if !columns.is_empty()
      {
        let first = columns[ 0 ].clone();
        columns.iter().skip( 1 ).fold( first, | acc, val | format!( "{}, {}", acc, val ) )
      }
      else
      {
        String::from( "No columns" )
      };

      rows.push
      (
        vec!
        [
          EMPTY_CELL.cell(),
          table_name.cell(),
          columns_str.cell(),
        ]
      );
    }

    let table_struct = rows.table()
    .border( Border::builder().build() )
    .separator( Separator::builder().build() )
    .title( vec!
    [
      EMPTY_CELL.cell(),
      "name".cell().bold( true ),
      "columns".cell().bold( true ),
    ] );

    let table = table_struct.display().unwrap(); 

    writeln!( f, "{}", table )?;

    Ok( () )
  }
}

impl Report for TablesReport {}