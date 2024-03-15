//! Tables metadata commands actions and reports.

use crate::*;
use executor::FeedManager;
use gluesql::prelude::Payload;
use std::collections::HashMap;
use executor::Report;
use storage::{ FeedStorage, tables::TableStore };
use error_tools::{ err, BasicError, Result };

/// Get labels of column for specified table.
pub async fn list_columns
(
  storage : FeedStorage< gluesql::sled_storage::SledStorage >,
  args : &wca::Args,
) -> Result< impl Report >
{
  let table_name : String = args
  .get_owned::< String >( 0 )
  .ok_or_else::< BasicError, _ >( || err!( "Cannot get 'Name' argument for command .table.list" ) )?
  .into()
  ;

  let mut manager = FeedManager::new( storage );
  let result = manager.storage.list_columns( table_name.clone() ).await?;

  let mut table_description = String::new();
  let mut columns = std::collections::HashMap::new();
  match &result[ 0 ]
  {
    Payload::Select { labels: _label_vec, rows: rows_vec } =>
    {
      for row in rows_vec
      {
        let table = String::from( row[ 0 ].clone() );
        columns.entry( table )
        .and_modify( | vec : &mut Vec< String > | vec.push( String::from( row[ 1 ].clone() ) ) )
        .or_insert( vec![ String::from( row[ 1 ].clone() ) ] )
        ;
      }
    },
    _ => {},
  }
  let mut columns_desc = HashMap::new();
  match table_name.as_str()
  {
    "feed" =>
    {
      table_description = String::from( "Table contains information about feed." );
      
      for label in columns.get( "feed" ).unwrap()
      {
        match label.as_str()
        {
          "id" => { columns_desc.insert( label.clone(), String::from( "A unique identifier for this feed" ) ); }
          "title" => { columns_desc.insert( label.clone(), String::from( "The title of the feed" ) ); }
          "updated" => 
          {
            columns_desc.insert( label.clone(), String::from
            (
              "The time at which the feed was last modified. If not provided in the source, or invalid, it is None."
            ) );
          },
          "type" => { columns_desc.insert( label.clone(), String::from( "Type of this feed (e.g. RSS2, Atom etc)" ) ); }
          "authors" => { columns_desc.insert( label.clone(), String::from( "Collection of authors defined at the feed level" ) ); }
          "description" => { columns_desc.insert( label.clone(), String::from( "Description of the feed" ) ); }
          "published" => { columns_desc.insert( label.clone(), String::from( "The publication date for the content in the channel" ) ); }
          "update_period" => { columns_desc.insert( label.clone(), String::from( "How often this feed must be updated" ) ); }
          _ => { columns_desc.insert( label.clone(), String::from( "Desciption for this column hasn't been added yet!" ) ); }
        }
      }
    },
    "frame" =>
    {
      for label in columns.get( "frame" ).unwrap()
      {
        match label.as_str()
        {
          "id" => { columns_desc.insert( label.clone(), String::from( "A unique identifier for this frame in the feed. " ) ); },
          "title" => { columns_desc.insert( label.clone(), String::from("Title of the frame" ) ); },
          "updated" => { columns_desc.insert( label.clone(), String::from("Time at which this item was fetched from source." ) ); },
          "authors" => { columns_desc.insert( label.clone(), String::from("List of authors of the frame, optional." ) ); },
          "content" => { columns_desc.insert( label.clone(), String::from("The content of the frame in html or plain text, optional." ) ); },
          "links" => { columns_desc.insert( label.clone(), String::from("List of links associated with this item of related Web page and attachments." ) ); },
          "summary" => { columns_desc.insert( label.clone(), String::from("Short summary, abstract, or excerpt of the frame item, optional." ) ); },
          "categories" => { columns_desc.insert( label.clone(), String::from("Specifies a list of categories that the item belongs to." ) ); },
          "published" => { columns_desc.insert( label.clone(), String::from("Time at which this item was first published or updated." ) ); },
          "source" => { columns_desc.insert( label.clone(), String::from("Specifies the source feed if the frame was copied from one feed into another feed, optional." ) ); },
          "rights" => { columns_desc.insert( label.clone(), String::from( "Conveys information about copyrights over the feed, optional." ) ); },
          "media" => { columns_desc.insert( label.clone(), String::from("List of media oblects, encountered in the frame, optional." ) ); },
          "language" => { columns_desc.insert( label.clone(), String::from("The language specified on the item, optional." ) ); },
          "feed_link" => { columns_desc.insert( label.clone(), String::from("Link of feed that contains this frame." ) ); },
          _ => { columns_desc.insert( label.clone(), String::from( "Desciption for this column hasn't been added yet!" ) ); }
        }
      }
    }
    "config" =>
    {
      for label in columns.get( "config" ).unwrap()
      {
        match label.as_str()
        {
          "path" => { columns_desc.insert( label.clone(), String::from( "Path to configuration file" ) ); }
          _ => { columns_desc.insert( label.clone(), String::from( "Desciption for this column hasn't been added yet!" ) ); }
        }
      }
    },
    _ => {},
  }

  Ok( ColumnsReport::new( table_name, table_description, columns_desc ) )
}

/// Get names of tables in storage.
pub async fn list_tables
(
  storage : FeedStorage< gluesql::sled_storage::SledStorage >,
  _args : &wca::Args,
) -> Result< impl Report >
{
  let mut manager = FeedManager::new( storage );
  manager.storage.list_tables().await
}

const EMPTY_CELL : &'static str = "";

/// Information about execution of tables commands.
#[ derive( Debug ) ]
pub struct ColumnsReport
{
  table_name : String,
  table_description : String,
  columns : std::collections::HashMap< String, String >
}

impl ColumnsReport
{
  pub fn new( table_name : String, table_description : String, columns : HashMap< String, String > ) -> Self
  {
    Self
    {
      table_name,
      table_description,
      columns,
    }
  }
}

impl std::fmt::Display for ColumnsReport
{
  fn fmt( &self, f : &mut std::fmt::Formatter<'_> ) -> std::fmt::Result
  {
    write!( f, "Table name: {}", self.table_name )?;
    writeln!( f, "{}", self.table_description )?;
    
      if !self.columns.is_empty()
      {
        let mut rows = Vec::new();
        for ( label, desc ) in &self.columns
        {
          rows.push
          (
            vec!
            [
              EMPTY_CELL.to_owned(),
              label.clone(),
              desc.clone(),
            ]
          );
        }
        let table = table_display::table_with_headers
        (
          vec!
          [
            EMPTY_CELL.to_owned(),
            "label".to_owned(),
            "description".to_owned(),
          ],
          rows,
        );

        if let Some( table ) = table
        {
          writeln!( f, "{}", table )?;
        }
      }
      else
      {
        writeln!( f, "No columns" );
      }
    

    Ok( () )
  }
}

impl Report for ColumnsReport {}

/// Information about execution of tables commands.
#[ derive( Debug ) ]
pub struct TablesReport
{
  tables : std::collections::HashMap< String, Vec< String > >
}

impl TablesReport
{
  /// Create new report from payload.
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
          EMPTY_CELL.to_owned(),
          table_name.to_owned(),
          columns_str,
        ]
      );
    }

    let table = table_display::table_with_headers
    (
      vec!
      [
        EMPTY_CELL.to_owned(),
        "name".to_owned(),
        "columns".to_owned(),
      ],
      rows,
    );
    if let Some( table ) = table
    {
      writeln!( f, "{}", table )?;
    }
    
    Ok( () )
  }
}

impl Report for TablesReport {}

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