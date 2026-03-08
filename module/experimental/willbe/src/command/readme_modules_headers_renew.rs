mod private
{

  use crate :: *;
  // use pth ::AbsolutePath;
  // use error :: { untyped ::Error };
  // Explicit import for Result and its variants for pattern matching
  use core ::result ::Result :: { Ok, Err };

  /// Generate headers for workspace members
  ///
  /// # Errors
  /// qqq: doc
  // qqq: typed error
  pub fn readme_modules_headers_renew() -> error ::untyped ::Result< () >
  {
  let current_path = AbsolutePath ::try_from( std ::env ::current_dir()? )?;
  let crate_dir = CrateDir ::try_from( current_path )?;
  match action ::readme_modules_headers_renew( crate_dir )
  {
   Ok( report ) =>
   {
  println!( "{report}" );
  Ok( () )
 }
   Err( ( report, e ) ) =>
   {
  eprintln!( "{report}" );
  Err( error ::untyped ::Error ::from( e ).context( "Fail to generate modules headers." ) )
  // qqq: use typed error
 }
 }
 }

}

crate ::mod_interface!
{
  /// List packages.
  orphan use readme_modules_headers_renew;
}