use std::sync::Arc;
use tokio::sync::Mutex;
use feed_rs::model::Entry;
use gluesql::
{
  core::
  {
    ast_builder::{ col, null, table, text, timestamp, Build, Execute, ExprNode },
    chrono::SecondsFormat,
    data::Value,
    executor::Payload,
    store::{ GStore, GStoreMut },
    prelude::Payload::ShowColumns,
  },
  prelude::Glue,
  sled_storage::{ sled::Config, SledStorage },
};
use wca::wtools::Itertools;

/// Storage for feed frames.
pub struct FeedStorage< S : GStore + GStoreMut + Send >
{
  /// GlueSQL storage.
  pub storage : Arc< Mutex< Glue< S > > >
}

impl FeedStorage< SledStorage >
{
  /// Initialize new storage from configuration, create feed table.
  pub async fn init_storage( config : Config ) -> Result< Self, Box< dyn std::error::Error + Send + Sync > >
  {
    let storage = SledStorage::try_from( config )?;
    let mut glue = Glue::new( storage );
  
    let table = table( "Feed" )
    .create_table_if_not_exists()
    .add_column( "id TEXT PRIMARY KEY" )
    .add_column( "title TEXT" )
    .add_column( "updated TIMESTAMP" )
    .add_column( "authors TEXT" )
    .add_column( "content TEXT" )
    .add_column( "links TEXT" )
    .add_column( "summary TEXT" )
    .add_column( "categories TEXT" )
    .add_column( "contributors TEXT" )
    .add_column( "published TIMESTAMP" )
    .add_column( "source TEXT" )
    .add_column( "rights TEXT" )
    .add_column( "media TEXT" )
    .add_column( "language TEXT" )
    .build()?
    ;
  
    table.execute( &mut glue ).await?;

    Ok( Self{ storage : Arc::new( Mutex::new( glue ) ) } )
  }
}

/// Functionality of feed storage.
#[ mockall::automock ]
#[ async_trait::async_trait(?Send ) ]
pub trait FeedStore
{
  /// Insert items from list into feed table.
  async fn save_feed( &mut self, feed : Vec< feed_rs::model::Entry > ) -> Result< (), Box< dyn std::error::Error + Send + Sync > >;

  /// Update items from list in feed table.
  async fn update_feed( &mut self, feed : Vec< feed_rs::model::Entry > ) -> Result< (), Box< dyn std::error::Error + Send + Sync > >;

  /// Process fetched feed, new items will be saved, modified items will be updated.
  async fn process_feed( &mut self, feed : Vec< feed_rs::model::Entry > ) -> Result< (), Box< dyn std::error::Error + Send + Sync > >;

  /// Get all feed frames from storage.
  async fn get_all_feed( &mut self ) -> Result< Payload, Box< dyn std::error::Error + Send + Sync > >;

  /// Execute custom query passed as String.
  async fn execute_query( &mut self, query : String ) -> Result< (), Box< dyn std::error::Error + Send + Sync > >;

  /// Get list of column titles of feed table.
  async fn columns_titles( &mut self ) -> Vec< String >;
}

#[ async_trait::async_trait(?Send) ]
impl FeedStore for FeedStorage< SledStorage >
{
  async fn columns_titles( &mut self ) -> Vec< String >
  {
    let columns = table( "Feed" ).show_columns().execute( &mut *self.storage.lock().await ).await;
    match columns
    {
      Ok( ShowColumns( col_vec ) ) => col_vec.into_iter().map( | c | c.0 ).collect_vec(),
      _ => Vec::new(),
    }
  }

  async fn execute_query( &mut self, query : String ) -> Result< (), Box< dyn std::error::Error + Send + Sync > >
  {
    let glue = &mut *self.storage.lock().await;
    let payloads = glue.execute( &query ).await.unwrap();

    for payload in payloads
    {
      match payload
      {
        Payload::ShowColumns( columns ) =>
        {
          for column in columns
          {
            println!( "{} : {}", column.0, column.1 )
          }
        },
        Payload::Create => println!( "Table created" ),
        Payload::Insert( number ) => println!( "Inserted {} rows", number ),
        Payload::Delete( number ) => println!( "Deleted {} rows", number ),
        Payload::Update( number ) => println!( "Updated {} rows", number ),
        Payload::DropTable => println!( "Table dropped" ),
        Payload::Select { labels: label_vec, rows: rows_vec } =>
        {
          println!( "labels : {}", label_vec.iter().fold( String::new(), | acc, val | format!( "{}, {}", acc, val ) ) );
          for row in rows_vec
          {
            println!( "{}", row.iter().fold( String::new(), | acc, val | format!( "{}, {:?}", acc, val ) ) );
          }
        },
        Payload::AlterTable => println!( "Table altered" ),
        Payload::StartTransaction => println!( "Transaction started" ),
        Payload::Commit => println!( "Transaction commited" ),
        Payload::Rollback => println!( "Transaction rolled back" ),
        _ => {},
      };
    }

    Ok( () )
  }

  async fn get_all_feed( &mut self ) -> Result< Payload, Box< dyn std::error::Error + Send + Sync > >
  {
    //let result = Vec::new();
    let res = table( "Feed" ).select().execute( &mut *self.storage.lock().await ).await?;
    Ok( res )
  }

  async fn save_feed( &mut self, feed : Vec< feed_rs::model::Entry > ) -> Result< (), Box< dyn std::error::Error + Send + Sync > >
  {
    let entries_rows = feed.into_iter().map( | entry | entry_row( &entry ) ).collect_vec();

    let insert = table( "Feed" )
    .insert()
    .columns
    (
      "id,
      title,
      updated,
      authors,
      content,
      links,
      summary,
      categories,
      contributors,
      published,
      source,
      rights,
      media,
      language",
    )
    .values( entries_rows )
    .execute( &mut *self.storage.lock().await )
    .await?
    ;

    Ok( () )
  }

  async fn update_feed( &mut self, feed : Vec< feed_rs::model::Entry > ) -> Result< (), Box< dyn std::error::Error + Send + Sync > >
  {
    let entries_rows = feed.into_iter().map( | entry | entry_row( &entry ) ).collect_vec();

    for entry in entries_rows
    {
      let update = table( "Feed" )
      .update()
      .set( "title", entry[ 1 ].to_owned() )
      .set( "content", entry[ 4 ].to_owned() )
      .set( "links", entry[ 5 ].to_owned() )
      .set( "summary", entry[ 6 ].to_owned() )
      .set( "published", entry[ 9 ].to_owned() )
      .set( "media", entry[ 10 ].to_owned() )
      .filter( col( "id" ).eq( entry[ 0 ].to_owned() ) )
      .execute( &mut *self.storage.lock().await )
      .await?
      ;
      
    }

    Ok( () )
  }

  async fn process_feed
  (
    &mut self,
    feed : Vec< feed_rs::model::Entry >,
  ) -> Result< (), Box< dyn std::error::Error + Send + Sync > >
  {
    let existing = table( "Feed" )
    .select()
    .project( "id, published" )
    .execute( &mut *self.storage.lock().await )
    .await?
    ;

    let mut new_entries = Vec::new();
    let mut modified_entries = Vec::new();
    if let Some( rows ) = existing.select()
    {
      let existing_entries = rows
      .map( | r | ( r.get( "id" ).map( | &val | val.clone() ), r.get( "published" ).map( | &val | val.clone() ) ) )
      .flat_map( | ( id, published ) |
        id.map( | id |
          (
            id,
            published.map( | date |
              {
                match date 
                {
                  Value::Timestamp( date_time ) => Some( date_time ),
                  _ => None,
                } 
              } )
            .flatten()
          )
        )
      )
      .flat_map( | ( id, published ) | match id { Value::Str( id ) => Some( ( id, published ) ), _ => None } )
      .collect_vec()
      ;

      let existing_ids = existing_entries.iter().map( | ( id, _ ) | id ).collect_vec();

      for entry in feed
      {
        if let Some( position ) = existing_ids.iter().position( | &id | id == &entry.id )
        {
          if let Some( date ) = existing_entries[ position ].1
          {
            if date.and_utc() != entry.published.unwrap()
            {
              modified_entries.push( entry );
            }
          }
        }
        else
        {
          new_entries.push( entry );
        }
      }
    }
    
    if new_entries.len() > 0
    {
      self.save_feed( new_entries ).await?;
    }
    if modified_entries.len() > 0
    {
      self.update_feed( modified_entries ).await?;
    }
    
    Ok( () )
  }
}

/// Create row for QlueSQL storage from Feed Entry type.
pub fn entry_row( entry : &Entry ) -> Vec< ExprNode< 'static > >
{
  let mut res = Vec::new();
  res.push( text( entry.id.clone() ) );
  res.push( entry.title.clone().map( | title | text( title.content ) ).unwrap_or( null() ) );
  res.push( entry.updated.map( | d | timestamp( d.to_rfc3339_opts( SecondsFormat::Millis, true ) ) ).unwrap_or( null() ) );
  res.push( text( entry.authors.iter().map( | p | p.name.clone() ).fold( String::new(), | acc, val | format!( "{}, {}", acc, val ) ) ).to_owned() );
  res.push
  (
    entry.content
    .clone()
    .map( | c | text( c.body.unwrap_or( c.src.map( | link | link.href ).unwrap_or_default() ) ) ).unwrap_or( null() ) 
  );
  if entry.links.len() != 0
  {
    res.push( text
      (
        entry.links
        .clone()
        .iter()
        .map( | link | link.href.clone() )
        .fold( String::new(), | acc, val | format!( "{} {}", acc, val ) )
      )
    );
  }
  else 
  {
    res.push( null() );
  }
  res.push( entry.summary.clone().map( | c | text( c.content ) ).unwrap_or( null() ) );
  if entry.categories.len() != 0
  {
    res.push( text
      (
        entry.categories
        .clone()
        .iter()
        .map( | cat | cat.term.clone() )
        .fold( String::new(), | acc, val | format!( "{} {}", acc, val ) )
      )
    );
  }
  else
  {
    res.push( null() );
  }
  if entry.contributors.len() != 0
  {
    res.push( text
      (
        entry.contributors
        .clone()
        .iter()
        .map( | c | c.name.clone() ).fold( String::new(), | acc, val | format!( "{} {}", acc, val ) )
      )
    );
  }
  else 
  {
    res.push( null() );
  }
  res.push( entry.published.map( | d | timestamp( d.to_rfc3339_opts( SecondsFormat::Millis, true ) ) ).unwrap_or( null() ) );
  res.push( entry.source.clone().map( | s | text( s ) ).unwrap_or( null() ) );
  res.push( entry.rights.clone().map( | r | text( r.content ) ).unwrap_or( null() ) );
  if entry.media.len() != 0
  {
    res.push( text
      (
        entry.media
        .clone()
        .iter()
        .map( | m | m.title.clone().map( | t | t.content ).unwrap_or_default() )
        .fold( String::new(), | acc, val | format!( "{} {}", acc, val ) )
      )
    );
  }
  else 
  {
    res.push( null() );
  }
  res.push( entry.language.clone().map( | l | text( l ) ).unwrap_or( null() ) );
  res
}
