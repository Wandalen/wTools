use crate::*;
use cli_table::{ format::{ Border, Separator }, Cell, Table };
use gluesql::core::executor::Payload;
use super::Report;
use storage::{ FeedStorage, FeedStore };
use executor::FeedManager;

pub async fn execute_query( storage : FeedStorage< gluesql::sled_storage::SledStorage >, args : &wca::Args ) -> Result< impl Report, Box< dyn std::error::Error + Send + Sync > >
{
  let query = args.get_owned::< Vec::< String > >( 0 ).unwrap().join( " " );

  let mut manager = FeedManager::new( storage );
  manager.storage.execute_query( query ).await
}

const EMPTY_CELL : &'static str = "";

/// Information about result of execution of custom query.
#[ derive( Debug ) ]
pub struct QueryReport
{
  pub result : Vec< gluesql::prelude::Payload >,
}

impl std::fmt::Display for QueryReport
{
  fn fmt( &self, f : &mut std::fmt::Formatter<'_> ) -> std::fmt::Result
  {
    for payload in &self.result
    {
      match payload
      {
        Payload::ShowColumns( columns ) =>
        {
          writeln!( f, "Show columns:" )?;
          for column in columns
          {
            writeln!( f, "{} : {}", column.0, column.1 )?;
          }
        },
        Payload::Create => writeln!( f, "Table created" )?,
        Payload::Insert( number ) => writeln!( f, "Inserted {} rows", number )?,
        Payload::Delete( number ) => writeln!( f, "Deleted {} rows", number )?,
        Payload::Update( number ) => writeln!( f, "Updated {} rows", number )?,
        Payload::DropTable => writeln!( f, "Table dropped" )?,
        Payload::Select { labels: label_vec, rows: rows_vec } =>
        {
          writeln!( f, "Selected entries:" )?;
          for row in rows_vec
          {
            let mut rows = Vec::new();
            for i in 0..label_vec.len()
            {
              let new_row = vec!
              [
                EMPTY_CELL.cell(),
                label_vec[ i ].clone().cell(),
                textwrap::fill( &String::from( row[ i ].clone() ), 120 ).cell(),
              ];
              rows.push( new_row );
            }
            let table_struct = rows.table()
            .border( Border::builder().build() )
            .separator( Separator::builder().build() );

            let table = table_struct.display().unwrap();

            writeln!( f, "{}", table )?;
          }
        },
        Payload::AlterTable => writeln!( f, "Table altered" )?,
        Payload::StartTransaction => writeln!( f, "Transaction started" )?,
        Payload::Commit => writeln!( f, "Transaction commited" )?,
        Payload::Rollback => writeln!( f, "Transaction rolled back" )?,
        _ => {},
      };
    }

    Ok( () )
  }
}

impl Report for QueryReport {}