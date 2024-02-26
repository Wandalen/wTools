use std::sync::{ Arc, Mutex };

use feed_rs::model::Entry;
use gluesql::
{
  core::
  {
    ast_builder::{ null, table, text, timestamp, Build, Execute, ExprNode },
    chrono::SecondsFormat,
    data::Value,
    executor::Payload,
  },
  prelude::Glue,
  sled_storage::SledStorage,
};
use wca::wtools::Itertools;

pub async fn init_storage() -> Result< Glue< SledStorage >, Box< dyn std::error::Error + Send + Sync > >
{
  let storage = SledStorage::new( "data/temp" ).unwrap();
  let mut glue = Glue::new( storage );

  let drop = table( "Feed1" )
  .drop_table_if_exists()
  .build()?
  ;

  drop.execute( &mut glue ).await?;

  let table = table( "Feed" )
  .create_table_if_not_exists()
  .add_column( "id TEXT PRIMARY KEY" )
  .add_column( "title TEXT" )
  .add_column( "updated TIMESTAMP" )
  //.add_column( "authors LIST" )
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

  Ok( glue )
}

pub async fn save_feed( feed : Vec< Entry >, glue : Arc< Mutex< Glue< SledStorage > > > ) -> Result< (), Box< dyn std::error::Error > >
{
  let mut rows = Vec::new();
  let mut glue = glue.lock().unwrap();

  let existing = table( "Feed" )
  .select()
  .project( "id, updated" )
  .execute( &mut glue )
  .await?
  ;

  for row in existing.select().unwrap()
  {
    println!( "{:?}", row );
  }

  let mut filtered = Vec::new();
  if let Some( rows ) = existing.select()
  {
    let existing_entries = rows.map( | r | ( r.get( "id" ).map( | &val | val.clone() ), r.get( "updated" ).map( | &val | val.clone() ) ) )
    .flat_map( | ( id, updated ) | id.map( | id | ( id, updated.map( | date | match date { Value::Timestamp( date_time ) => Some( date_time ), _ => None } ).flatten() ) ) )
    .flat_map( | ( id, updated ) | match id { Value::Str( id ) => Some( ( id, updated ) ), _ => None } )
    .collect_vec()
    ;

    let existing_ids = existing_entries.iter().map( | ( id, _ ) | id ).collect_vec();
    filtered = feed.into_iter().filter( | entry | 
    {
      if let Some( position ) = existing_ids.iter().position( | &id | id == &entry.id )
      {
        return false;
        // if let Some( date ) = existing_entries[ position ].1
        // {
          
        //   println!("{:?}  {:?}", date.and_utc( ), entry.updated.unwrap() );
        //   if date.and_utc() == entry.updated.unwrap()
        //   {
            
        //   }
        // }
      }
      true
    } ).collect_vec();
  }

  for i in 0..filtered.len()
  {
    println!("{:?}", filtered[ i ].id);
    rows.push( entry_row( &filtered[ i ] ) );
  }
  
  let insert = table( "Feed" )
  .insert()
  .columns( "id, title, updated, content, links, summary, categories, contributors, published, source, rights, media, language" )
  .values( rows )
  .execute( &mut glue )
  .await?
  ;

  if let Payload::Insert( n ) = insert
  {
    println!("inserted {} entries", n );
  }

  let check = table( "Feed" )
  .select()
  .project( "id, title, summary" )
  .execute( &mut glue )
  .await?
  ;

  // for row in check.select().unwrap()
  // {
  //   println!( "{:?}", row );
  // }

  Ok( () )
}

pub fn entry_row( entry : &Entry ) -> Vec< ExprNode< 'static > >
{
  let mut res = Vec::new();
  res.push( text( entry.id.clone() ) );
  res.push( entry.title.clone().map( | title | text( title.content ) ).unwrap_or( null() ) );
  res.push( entry.updated.map( | d | timestamp( d.to_rfc3339_opts( SecondsFormat::Millis, true ) ) ).unwrap_or( null() ) );
  //res.push( text( entry.authors.iter().map( | p | p.name.clone() ).fold( String::new(), | acc, val | format!( "{} {}", acc, val ) ) ).to_owned() );
  res.push( entry.content.clone().map( | c | text( c.body.unwrap_or( c.src.map( | link | link.href ).unwrap_or_default() ) ) ).unwrap_or( null() ) );
  if entry.links.len() != 0
  {
    res.push( text( entry.links.clone().iter().map( | link | link.href.clone() ).fold( String::new(), | acc, val | format!( "{} {}", acc, val ) ) ) );
  }
  else 
  {
    res.push( null() );
  }
  res.push( entry.summary.clone().map( | c | text( c.content ) ).unwrap_or( null() ) );
  if entry.categories.len() != 0
  {
    res.push( text( entry.categories.clone().iter().map( | cat | cat.term.clone() ).fold( String::new(), | acc, val | format!( "{} {}", acc, val ) ) ) );
  }
  else
  {
    res.push( null() );
  }
  if entry.contributors.len() != 0
  {
    res.push( text( entry.contributors.clone().iter().map( | c | c.name.clone() ).fold( String::new(), | acc, val | format!( "{} {}", acc, val ) ) ) );
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
    res.push( text( entry.media.clone().iter().map( | m | m.title.clone().map( | t | t.content ).unwrap_or_default() ).fold( String::new(), | acc, val | format!( "{} {}", acc, val ) ) ) );
  }
  else 
  {
    res.push( null() );
  }
  res.push( entry.language.clone().map( | l | text( l ) ).unwrap_or( null() ) );
  res
}
