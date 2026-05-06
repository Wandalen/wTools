//! Frame storing and retrieving functionality.

use crate :: *;
use error_tools ::untyped ::Result;
use gluesql ::core ::
{
  ast_builder :: { null, text, timestamp, ExprNode }, chrono :: { DateTime, SecondsFormat, Utc }, executor ::Payload
};
use gluesql ::prelude ::Value;

/// Frame entity.
#[ derive( Debug ) ]
pub struct Frame
{
  /// Frame id.
  pub id: String,
  /// Frame title.
  pub title: Option< String >,
  /// Time at which this item was fetched from source.
  pub stored_time: Option< DateTime< Utc > >,
  /// List of authors of the frame.
  pub authors: Option< Vec< String > >,
  /// The content of the frame in html or plain text.
  pub content: Option< String >,
  /// List of links associated with this item of related Web page and attachments.
  pub links: Option< Vec< String > >,
  /// Short summary, abstract, or excerpt of the frame item.
  pub summary: Option< String >,
  /// A list of categories that the item belongs to.
  pub categories: Option< Vec< String > >,
  /// Time at which this item was first published or updated.
  pub published: Option< DateTime< Utc > >,
  /// Specifies the source feed if the frame was copied from one feed into another feed.
  pub source: Option< String >,
  /// Information about copyrights over the feed.
  pub rights: Option< String >,
  /// List of media oblects, encountered in the frame.
  pub media: Option< Vec< String > >,
  /// The language of the frame.
  pub language: Option< String >,
  /// Link to feed that contains this frame.
  pub feed_link: String,
}

/// Convert from `feed_rs` feed entry and feed link to Frame struct for convenient use and storage.
impl From< ( feed_rs ::model ::Entry, String ) > for Frame
{
  fn from( ( entry, feed_link ) : ( feed_rs ::model ::Entry, String ) ) -> Self
  {
  let authors = entry.authors
  .iter()
  .map( | p | p.name.clone() )
  .collect :: < Vec< _ > >()
  ;

  let content = entry.content
  .map( | c | c.body.unwrap_or( c.src.map( | link | link.href ).unwrap_or_default() ) )
  .filter( | s | !s.is_empty() )
  .clone()
  ;

  let links = entry.links
  .iter()
  .map( | link | link.href.clone() )
  .collect :: < Vec< _ > >()
  .clone()
  ;

  let categories = entry.categories
  .iter()
  .map( | cat | cat.term.clone() )
  .collect :: < Vec< _ > >()
  ;

  let media = entry.media
  .iter()
  .flat_map( | m | m.content.clone() )
  .filter_map( | m | m.url.map( | url | url.to_string() ) )
  .collect :: < Vec< _ > >()
  ;

  Frame
  {
   id: entry.id,
   title: entry.title.map( | title | title.content ).clone(),
   stored_time: entry.updated,
   authors: ( !authors.is_empty() ).then_some(authors),
   content,
   links: ( !links.is_empty() ).then_some(links),
   summary: entry.summary.map( | c | c.content ).clone(),
   categories: ( !categories.is_empty() ).then_some(categories),
   published: entry.published,
   source: entry.source.clone(),
   rights: entry.rights.map( | r | r.content ).clone(),
   media: ( !media.is_empty() ).then_some(media),
   language: entry.language.clone(),
   feed_link,
 }
 }
}

/// Frames storing and retrieving.
#[ async_trait ::async_trait( ?Send ) ]
pub trait FrameStore
{
  /// Save new frames to storage.
  /// New frames will be inserted into `frame` table. 
  async fn frames_save( &mut self, feed: Vec< Frame > ) -> Result< Payload >;

  /// Update existing frames in storage with new changes.
  /// If frames in storage were modified in feed source, they will be changed to match new version. 
  async fn frames_update( &mut self, feed: Vec< Frame > ) -> Result< () >;

  /// Get all feed frames from storage.
  async fn frames_list( &mut self ) -> Result< ListReport >;
}

/// Get convenient frame format for using with `GlueSQL` expression builder.
/// Converts from Frame struct into vec of `GlueSQL` expression nodes. 
impl From< Frame > for Vec< ExprNode< 'static > >
{
  fn from( entry: Frame ) -> Self
  {
  let title = entry.title
  .map_or( null(), text )
  ;

  let stored_time = entry.stored_time
  .map_or( null(), | d | timestamp( d.to_rfc3339_opts( SecondsFormat ::Millis, true ) ) )
  ;

  let authors = entry.authors
  .map_or( null(), | authors |
   text
   (
  format!( "[{}]", authors.into_iter().map( | a | format!( "\"{a}\"" ) ).collect :: < Vec< _ > >().join( ", " ) )
 )
 )
  ;

  let content = entry.content
  .map_or( null(), text )
  ;

  let links = entry.links
  .map_or( null(), | links |
   text
   (
  format!( "[{}]", links.into_iter().map( | link | format!( "\"{link}\"" ) ).collect :: < Vec< _ > >().join( ", " ) ) 
 )
 )
  ;

  let summary = entry.summary
  .map_or( null(), text )
  ;

  let categories = entry.categories
  .map_or( null(), | categories |
   text
   (
  format!
  (
   "[{}]",
   categories.into_iter().map( | category | format!( "\"{category}\"" ) ).collect :: < Vec< _ > >().join( ", " ),
 ) 
 )
 )
  ;

  let published = entry.published
  .map_or( null(), | d | timestamp( d.to_rfc3339_opts( SecondsFormat ::Millis, true ) ) )
  ;

  let source = entry.source.map_or( null(), text );
  let rights = entry.rights.map_or( null(), text );
  let media = entry.media
  .map_or( null(), | media |
   text
   (
  format!( "[{}]", media.into_iter().map( | media | format!( "\"{media}\"" ) ).collect :: < Vec< _ > >().join( ", " ) ) 
 )
 )
  ;

  let language = entry.language.clone().map_or( null(), text );

  vec!
  [
   text( entry.id ),
   title,
   stored_time,
   authors,
   content,
   links,
   summary,
   categories,
   published,
   source,
   rights,
   media,
   language,
   text( entry.feed_link )
 ]
 }
}

/// `GlueSQL` Value wrapper for display.
#[ derive( Debug ) ]
pub struct CellValue< 'a >( pub &'a gluesql ::prelude ::Value );

impl core ::fmt ::Display for CellValue< '_ >
{
  fn fmt( &self, f: &mut core ::fmt ::Formatter< '_ > ) -> core ::fmt ::Result
  {
  use gluesql ::prelude ::Value :: *;
  match &self.0
  {
   Bool( val ) => write!( f, "{val}" )?,
   I8( val ) => write!( f, "{val}" )?,
   I16( val ) => write!( f, "{val}" )?,
   I32( val ) => write!( f, "{val}" )?,
   I64( val ) => write!( f, "{val}" )?,
   I128( val ) => write!( f, "{val}" )?,
   U8( val ) => write!( f, "{val}" )?,
   U16( val ) => write!( f, "{val}" )?,
   U32( val ) => write!( f, "{val}" )?,
   U64( val ) => write!( f, "{val}" )?,
   U128( val ) => write!( f, "{val}" )?,
   F32( val ) => write!( f, "{val}" )?,
   F64( val ) => write!( f, "{val}" )?,
   Str( val ) => write!( f, "{val}" )?,
   Null => write!( f, "Null" )?,
   Timestamp( val ) => write!( f, "{val}" )?,
   _ => write!( f, "" )?,
 }

  Ok( () )
 }
}

impl From< CellValue< '_ > > for String
{
  fn from( value: CellValue< '_ > ) -> Self
  {
  use gluesql ::core ::data ::Value::Str;
  match &value.0
  {
   Str( val ) => val.clone(),
   _ => String ::new(),
 }
 }
}

const INDENT_CELL: &str = "  ";

/// Items retrieved by select queries from storage.
#[ derive( Debug ) ]
#[ derive( Default ) ]
pub struct SelectedEntries
{
  /// Labels of selected columns.
  pub selected_columns: Vec< String >,
  /// Selected rows with data.
  pub selected_rows: Vec< Vec< Value > >,
}

impl SelectedEntries
{
  /// Create new empty selected entries struct.
  #[ must_use ]
  pub fn new() -> Self
  {
  Self ::default()
 }
}

impl core ::fmt ::Display for SelectedEntries
{
  fn fmt( &self, f: &mut core ::fmt ::Formatter< '_ > ) -> core ::fmt ::Result
  {
  if !self.selected_columns.is_empty()
  {
   for row in &self.selected_rows
   {
  for ( column_name, cell_value ) in self.selected_columns.iter().zip( row.iter() )
  {
   write!( f, "{} : {}, ", column_name, CellValue( cell_value ) )?;
 }
  writeln!( f )?;
 }
 }

  Ok( () )
 }
}

/// Information about result of execution of command for frames.
#[ derive( Debug ) ]
pub struct FramesReport
{
  /// Link of the feed which contains the frames.
  pub feed_link: String,
  /// Number of frames from the feed that were updated.
  pub updated_frames: usize,
  /// Number of new frames from the feed that were downloaded.
  pub new_frames: usize,
  /// Selected frames for commands that list frames.
  pub selected_frames: SelectedEntries,
  /// Number of frames that were in storage before update.
  pub existing_frames: usize,
  /// True if feed is downloaded for the first time.
  pub is_new_feed: bool,
}

impl FramesReport
{
  /// Create new report.
  #[ must_use ]
  pub fn new( feed_link: String ) -> Self
  {
  Self
  {
   feed_link,
   updated_frames: 0,
   new_frames: 0,
   selected_frames: SelectedEntries ::new(),
   existing_frames: 0,
   is_new_feed: false,
 }
 }
}

impl core ::fmt ::Display for FramesReport
{
  fn fmt( &self, f: &mut core ::fmt ::Formatter< '_ > ) -> core ::fmt ::Result
  {
  let initial = [ [ format!( "Feed title: {}", self.feed_link ) ] ];
  let table = tool ::table_display ::table_with_headers( initial[ 0 ].to_vec(), Vec ::new() );
  if let Some( table ) = table
  {
   write!( f, "{table}" )?;
 }

  let mut rows = vec!
  [
   vec![ EMPTY_CELL.to_owned(), format!( "Updated frames: {}", self.updated_frames ) ],
   vec![ EMPTY_CELL.to_owned(), format!( "Inserted frames: {}", self.new_frames ) ],
   vec![ EMPTY_CELL.to_owned(), format!( "Number of frames in storage: {}", self.existing_frames + self.new_frames ) ],
 ];

  if !self.selected_frames.selected_columns.is_empty()
  {
   rows.push( vec![ EMPTY_CELL.to_owned(), format!( "Selected frames: " ) ] );
 }

  let table = tool ::table_display ::plain_table( rows );
  if let Some( table ) = table
  {
   write!( f, "{table}" )?;
 }

  for frame in &self.selected_frames.selected_rows
  {
   let first_row = vec!
   [
  INDENT_CELL.to_owned(),
  self.selected_frames.selected_columns[ 0 ].clone(),
  textwrap ::fill( &String ::from( frame[ 0 ].clone() ), 120 ),
 ];
   let mut rows = Vec ::new();
   for ( column_name, frame_value ) in self.selected_frames.selected_columns.iter().skip( 1 ).zip( frame.iter().skip( 1 ) )
   {
  let inner_row = vec!
  [
   INDENT_CELL.to_owned(),
   column_name.clone(),
   textwrap ::fill( &String ::from( frame_value.clone() ), 120 ),
 ];
  rows.push( inner_row );
 }

   let table = tool ::table_display ::table_with_headers( first_row, rows );
   if let Some( table ) = table
   {
  writeln!( f, "{table}" )?;
 }
 }

  Ok( () )
 }
}

impl Report for FramesReport {}

/// Report for downloading and updating frames.
#[ derive( Debug ) ]
pub struct UpdateReport( pub Vec< FramesReport > );

impl core ::fmt ::Display for UpdateReport
{
  fn fmt( &self, f: &mut core ::fmt ::Formatter< '_ > ) -> core ::fmt ::Result
  {
  for report in &self.0
  {
   writeln!( f, "{report}" )?;
 }
  writeln!( f, "Total new feeds dowloaded: {}", self.0.iter().filter( | fr_report | fr_report.is_new_feed ).count() )?;
  writeln!
  (
   f,
   "Total feeds with updated or new frames: {}",
   self.0.iter().filter( | fr_report | fr_report.updated_frames + fr_report.new_frames > 0 ).count()
 )?;
  writeln!( f, "Total new frames: {}", self.0.iter().fold( 0, | acc, fr_report | acc + fr_report.new_frames ) )?;
  writeln!( f, "Total updated frames: {}", self.0.iter().fold( 0, | acc, fr_report | acc + fr_report.updated_frames ) )?;

  Ok( () )
 }
}

impl Report for UpdateReport {}

/// Report for listing frames.
#[ derive( Debug ) ]
pub struct ListReport( pub Vec< FramesReport > );

impl core ::fmt ::Display for ListReport
{
  fn fmt( &self, f: &mut core ::fmt ::Formatter< '_ > ) -> core ::fmt ::Result
  {
  for report in &self.0
  {
   write!( f, "{report}" )?;
 }
  writeln!
  (
   f,
   "Total feeds in storage: {}",
   self.0.len()
 )?;
  writeln!
  (
   f,
   "Total frames in storage: {}",
   self.0.iter().fold( 0, | acc, fr_report | acc + fr_report.selected_frames.selected_rows.len() )
 )?;
  writeln!( f )?;

  Ok( () )
 }
}

impl Report for ListReport {}
