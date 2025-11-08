/// Define a private namespace for all its items.
#[ allow( clippy ::std_instead_of_alloc, clippy ::std_instead_of_core ) ]
mod private
{

  use crate :: *;
  use colored ::Colorize;
  use wca ::VerifiedCommand;
  use error ::untyped ::Context; // xxx
  use former ::Former;
  use std ::fmt ::Write;
  use crate ::entity ::channel ::Channel;
  // Explicit import for Result and its variants for pattern matching
  use std ::result ::Result :: { Ok, Err };

  #[ derive( Former ) ]
  #[ allow( clippy ::struct_excessive_bools ) ]
  struct PublishProperties
  {
  #[ former( default = Channel ::Stable ) ]
  channel: Channel,
  #[ former( default = true ) ]
  dry: bool,
  #[ former( default = true ) ]
  temp: bool,
 }

  ///
  /// Publish package.
  ///
  /// # Errors
  /// qqq: doc
  pub fn publish( o: VerifiedCommand ) -> error ::untyped ::Result< () > // qqq: use typed error
  {
  // Fix(issue-publish-pathbuf-cast): Previously attempted to cast args[0] to PathBuf,
  // but command definition specifies subject as List<String>. This caused panic when
  // malformed properties like "dry:0" were treated as subjects by the parser.
  //
  // Root cause: Type mismatch between command grammar definition (List) and implementation
  // access pattern (PathBuf). The PathBuf cast was unnecessary - args_line only needed
  // for display formatting in the dry-run message.
  //
  // Pitfall: wca's generic get_owned<T>() performs runtime type casting, so mismatches
  // between command grammar types and access types are not caught at compile time. Always
  // ensure argument access types match the command definition types exactly. When the same
  // argument is accessed multiple times, retrieve it once and reuse the variable to prevent
  // inconsistent type expectations.

  let patterns: Vec< String > = o
  .args
  .get_owned( 0 )
  .unwrap_or_else( || vec![ "./".into() ] );

  // Validate patterns - detect common syntax mistakes
  for pattern in &patterns
  {
   // Check if pattern looks like a malformed property (contains ":" without spaces)
   if pattern.contains( ':' ) && !pattern.contains( " : " )
   {
    // Could be a valid path like "C:\\" or "http://", so only warn for common property names
    let looks_like_property = [ "dry", "temp", "channel", "verbosity" ]
     .iter()
     .any( | prop | pattern.starts_with( prop ) && pattern.contains( ':' ) );

    if looks_like_property
    {
     return Err
     (
      error ::untyped ::format_err!
      (
       "Invalid property syntax: '{}'\n\
        Properties require spaces around the colon.\n\
        Did you mean: '{} : {}'?",
       pattern,
       pattern.split( ':' ).next().unwrap(),
       pattern.split( ':' ).nth( 1 ).unwrap_or( "" )
      )
     );
   }
  }
 }

  let args_line = patterns.join( "," );

  let prop_line = o
  .props
  .iter()
  .map( | p | format!( "{} : {}", p.0, p.1 ) )
  .collect :: < Vec< _ > >().join(" ");

  let PublishProperties
  {
   channel,
   dry,
   temp
 } = o.props.try_into()?;
  let plan = action ::publish_plan( &patterns, channel, dry, temp )
  .context( "Failed to plan the publication process" )?;

  let mut formatted_plan = String ::new();
  writeln!( &mut formatted_plan, "Tree: " )?;
  plan.write_as_tree( &mut formatted_plan )?;

  if !plan.plans.is_empty()
  {
   writeln!( &mut formatted_plan, "The following packages are pending for publication: " )?;
   plan.write_as_list( &mut formatted_plan )?;
 }
  println!( "{formatted_plan}" );

  match action ::publish( plan )
  {
   Ok( report ) =>
   {
  println!( "{report}" );

  if dry && !report.packages.is_empty()
  {
   let args = if args_line.is_empty() { String ::new() } else { format!(" {args_line}" ) };
   let prop = if prop_line.is_empty() { String ::new() } else { format!(" {prop_line}" ) };
   let line = format!("will .publish{args}{prop} dry: 0" );
   println!("To apply plan, call the command `{}`", line.blue() );
   // aaa: for Petro: for Bohdan: bad. should be exact command with exact parameters
   // aaa: it`s already works
 }

  Ok( () )
 }
   Err( ( report, e ) ) =>
   {
  eprintln!( "{report}" );
  Err( e.context( "publish command" ) )
 }
 }
 }

  impl TryFrom< wca ::executor ::Props > for PublishProperties
  {
  type Error = error ::untyped ::Error;
  fn try_from( value: wca ::executor ::Props ) -> Result< Self, Self ::Error >
  {
   let mut this = Self ::former();

   this = if let Some( v ) = value
   .get_owned( "channel" )
   {
  this.channel :: < Channel >( { let v: String = v; Channel ::try_from( v )? } )
 }
   else
   { this };

   this = if let Some( v ) = value
   .get_owned( "dry" ) { this.dry :: < bool >( v ) } else { this };
   this = if let Some( v ) = value
   .get_owned( "temp" ) { this.temp :: < bool >( v ) } else { this };

   Ok( this.form() )
 }
 }
}

//

crate ::mod_interface!
{
  /// List packages.
  orphan use publish;
}
