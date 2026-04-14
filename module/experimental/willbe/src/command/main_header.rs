mod private
{

  use crate :: *;
  // use action;
  use error ::untyped :: { Error };
  // Explicit import for Result and its variants for pattern matching
  use core ::result ::Result :: { Ok, Err };

  /// Generates header to main readme.md file.
  ///
  /// # Errors
  /// qqq: doc
  // qqq: typed error
  pub fn readme_header_renew() -> error ::untyped ::Result< () >
  {
  let abs_path = AbsolutePath ::try_from( std ::env ::current_dir()? )?;
  let crate_dir = CrateDir ::try_from( abs_path )?;
  match crate ::action ::main_header ::action( crate_dir )
  {
   Ok( report ) =>
   {
  println!( "{report}" );
  Ok( () )
 }
   Err( ( report, e ) ) =>
   {
  eprintln!( "{report}" );
  Err( Error ::from( e ).context( "Fail to generate main header." ) )
 }
 }
 }
}

crate ::mod_interface!
{
  /// Generate header.
  orphan use readme_header_renew;
}