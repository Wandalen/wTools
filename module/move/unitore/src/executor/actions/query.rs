//! Query command endpoint and report.

// qqq : don't use both
use crate::*;
use super::*;
use gluesql::core::executor::Payload;
use storage::Store;
use error_tools::{ err, BasicError, Result };

/// Execute query specified in query string.
pub async fn execute_query
(
  mut storage : impl Store,
  args : &wca::Args,
) -> Result< impl Report >
{
  let query = args
  .get_owned::< String >( 0 )
  .ok_or_else::< BasicError, _ >( || err!( "Cannot get Query argument for command .query.execute" ) )?
  ;

  storage.execute_query( query ).await
}

const EMPTY_CELL : &'static str = "";

/// Information about result of execution of custom query.
#[ derive( Debug ) ]
pub struct QueryReport( pub Vec< gluesql::prelude::Payload > );

impl std::fmt::Display for QueryReport
{
  fn fmt( &self, f : &mut std::fmt::Formatter<'_> ) -> std::fmt::Result
  {
    for payload in &self.0
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
                EMPTY_CELL.to_owned(),
                label_vec[ i ].clone(),
                textwrap::fill( &String::from( row[ i ].clone() ), 120 ),
              ];
              rows.push( new_row );
            }
            let table = table_display::plain_table( rows );
            if let Some( table ) = table
            {
              writeln!( f, "{}", table )?;
            }
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

// qqq : good tests for query action
// all tables should be touched by these tests
