//! Tables metadata actions and reports.

use crate::*;
use gluesql::prelude::Payload;
use std::collections::HashMap;
use action::Report;
use sled_adapter::FeedStorage;
use entity::table::TableStore;
use error_tools::Result;

/// Get labels of column for specified table.
pub async fn table_list
(
  mut storage : FeedStorage< gluesql::sled_storage::SledStorage >,
  table_name : Option< String >,
) -> Result< impl Report >
{
  let mut table_names = Vec::new();
  if let Some( name ) = table_name
  {
    table_names.push( name );
  }
  else
  {
    let tables = storage.tables_list().await?;

    let names = tables.0.keys().map( | k | k.clone() ).collect::< Vec< _ > >();
    table_names.extend( names.into_iter() );
  }

  let mut reports = Vec::new();
  for table_name in table_names
  {
    let result = storage.table_list( table_name.clone() ).await?;

    let mut table_description = String::new();
    let mut columns = HashMap::new();
    if let Payload::Select { labels: _label_vec, rows: rows_vec } = &result[ 0 ]
    {
      for row in rows_vec
      {
        let table = String::from( row[ 0 ].clone() );
        columns.entry( table )
        .and_modify( | vec : &mut Vec< String > | vec.push( String::from( row[ 1 ].clone() ) ) )
        .or_insert( vec![ String::from( row[ 1 ].clone() ) ] )
        ;
      }
    }
    let mut columns_desc = HashMap::new();
    match table_name.as_str()
    {
      "feed" =>
      {
        table_description = String::from( "Contains feed items." );

        for label in columns.get( "feed" ).unwrap()
        {
          match label.as_str()
          {
            "link" => { columns_desc.insert
              (
                label.clone(),
                String::from( "Link to feed source, unique identifier for the feed" ),
              ); }
            "title" => { columns_desc.insert( label.clone(), String::from( "The title of the feed" ) ); }
            "updated" =>
            {
              columns_desc.insert( label.clone(), String::from
              (
                "The time at which the feed was last modified. If not provided in the source, or invalid, is Null."
              ) );
            },
            "type" => { columns_desc.insert( label.clone(), String::from( "Type of this feed (e.g. RSS2, Atom etc)" ) ); }
            "authors" => { columns_desc.insert
              (
                label.clone(),
                String::from( "Collection of authors defined at the feed level" )
              ); }
            "description" => { columns_desc.insert( label.clone(), String::from( "Description of the feed" ) ); }
            "published" => { columns_desc.insert
              (
                label.clone(),
                String::from( "The publication date for the content in the channel" ),
              ); }
            "update_period" => { columns_desc.insert( label.clone(), String::from( "How often this feed must be updated" ) ); }
            _ => { columns_desc.insert( label.clone(), String::from( "Desciption for this column hasn't been added yet!" ) ); }
          }
        }
      },
      "frame" =>
      {
        table_description = String::from( "Contains frame items." );
        for label in columns.get( "frame" ).unwrap()
        {
          match label.as_str()
          {
            "id" =>
            {
              columns_desc.insert
              (
                label.clone(),
                String::from( "A unique identifier for this frame in the feed. " ),
              );
            },
            "title" =>
            {
              columns_desc.insert
              (
                label.clone(),
                String::from( "Title of the frame" ),
              );
            },
            "updated" => 
            {
              columns_desc.insert
              (
                label.clone(),
                String::from( "Time at which this item was fetched from source." ),
              );
            },
            "authors" =>
            {
              columns_desc.insert
              (
                label.clone(),
                String::from( "List of authors of the frame, optional." )
              );
            },
            "content" =>
            {
              columns_desc.insert
              (
                label.clone(),
                String::from( "The content of the frame in html or plain text, optional." ),
              );
            },
            "links" =>
            {
              columns_desc.insert
              (
                label.clone(),
                String::from( "List of links associated with this item of related Web page and attachments." ),
              );
            },
            "summary" =>
            {
              columns_desc.insert
              (
                label.clone(),
                String::from( "Short summary, abstract, or excerpt of the frame item, optional." ),
              );
            },
            "categories" =>
            {
              columns_desc.insert
              (
                label.clone(),
                String::from( "Specifies a list of categories that the item belongs to." ),
              );
            },
            "published" =>
            {
              columns_desc.insert
              (
                label.clone(),
                String::from( "Time at which this item was first published or updated." ),
              );
            },
            "source" =>
            {
              columns_desc.insert
              (
                label.clone(),
                String::from( "Specifies the source feed if the frame was copied from one feed into another feed, optional." ),
              );
            },
            "rights" =>
            {
              columns_desc.insert
              (
                label.clone(),
                String::from( "Conveys information about copyrights over the feed, optional." ),
              );
            },
            "media" =>
            {
              columns_desc.insert
              (
                label.clone(),
                String::from( "List of media oblects, encountered in the frame, optional." ),
              );
            },
            "language" =>
            {
              columns_desc.insert
              (
                label.clone(),
                String::from( "The language specified on the item, optional." ),
              );
            },
            "feed_link" =>
            {
              columns_desc.insert
              (
                label.clone(),
                String::from( "Link of feed that contains this frame." ),
              );
            },
            _ => { columns_desc.insert( label.clone(), String::from( "Desciption for this column hasn't been added yet!" ) ); }
          }
        }
      }
      "config" =>
      {
        table_description = String::from( "Contains paths to feed config files." );
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

    reports.push( ColumnsReport::new( table_name, table_description, columns_desc ) );
  }

  Ok( TablesColumnsReport( reports ) )
}

/// Get information about tables in storage.
pub async fn tables_list( mut storage : FeedStorage< gluesql::sled_storage::SledStorage > ) -> Result< impl Report >
{
  storage.tables_list().await
}

const EMPTY_CELL : &'static str = "";

/// Information about execution of table columns commands.
#[ derive( Debug ) ]
pub struct TablesColumnsReport( pub Vec< ColumnsReport > );

impl std::fmt::Display for TablesColumnsReport
{
  fn fmt( &self, f : &mut std::fmt::Formatter< '_ > ) -> std::fmt::Result
  {
    for report in &self.0
    {
      writeln!( f, "{}", report )?;
    }
    
    Ok( () )
  }
}

impl Report for TablesColumnsReport {}

/// Information about execution of columns listing action.
#[ derive( Debug ) ]
pub struct ColumnsReport
{
  table_name : String,
  table_description : String,
  columns : std::collections::HashMap< String, String >
}

impl ColumnsReport
{
  /// Create new table columns report.
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
  fn fmt( &self, f : &mut std::fmt::Formatter< '_ > ) -> std::fmt::Result
  {
    writeln!( f, "Table name: {}", self.table_name )?;
    writeln!( f, "Description: {}", self.table_description )?;

    if !self.columns.is_empty()
    {
      writeln!( f, "Columns:" )?;
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
      let table = tool::table_display::table_with_headers
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
      writeln!( f, "No columns" )?;
    }

    Ok( () )
  }
}

impl Report for ColumnsReport {}

/// Information about execution of tables commands.
/// Contains tables name, description and list of columns.
#[ derive( Debug ) ]
pub struct TablesReport( pub HashMap< String, ( String, Vec< String > ) > );

impl TablesReport
{
  /// Create new report from payload.
  pub fn new( payload : Vec< Payload > ) -> Self
  {
    let mut result = std::collections::HashMap::new();
    if let Payload::Select { labels: _label_vec, rows: rows_vec } = &payload[ 0 ]
    {
      {
        for row in rows_vec
        {
          let table = String::from( row[ 0 ].clone() );
          let table_description = match table.as_str()
          {
            "feed" => String::from( "Contains feed items." ),
            "frame" => String::from( "Contains frame items." ),
            "config" => String::from( "Contains paths to feed config files." ),
            _ => String::new(),
          };
          result.entry( table )
          .and_modify( | ( _, vec ) : &mut ( String, Vec< String > ) | vec.push( String::from( row[ 1 ].clone() ) ) )
          .or_insert( ( table_description, vec![ String::from( row[ 1 ].clone() ) ] ) )
          ;
        }
      }
    }
    TablesReport( result )
  }
}

impl std::fmt::Display for TablesReport
{
  fn fmt( &self, f : &mut std::fmt::Formatter<'_> ) -> std::fmt::Result
  {
    writeln!( f, "Storage tables:" )?;
    let mut rows = Vec::new();
    for ( table_name, ( desc, columns ) ) in &self.0
    {
      let columns_str = if !columns.is_empty()
      {
        format!( "{};", columns.join( ", " ) )
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
          textwrap::fill( desc, 80 ),
          textwrap::fill( &columns_str, 80 ),
        ]
      );
    }

    let table = tool::table_display::table_with_headers
    (
      vec!
      [
        EMPTY_CELL.to_owned(),
        "name".to_owned(),
        "description".to_owned(),
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
