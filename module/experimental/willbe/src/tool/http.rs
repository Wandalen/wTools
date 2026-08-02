/// Define a private namespace for all its items.
#[ allow( clippy ::std_instead_of_alloc, clippy ::std_instead_of_core ) ]
mod private
{
  #[ allow( unused_imports, clippy ::wildcard_imports ) ]
  use crate ::tool :: *;

  use std ::
  {
  fmt ::Write,
  time ::Duration
 };
  use error :: { untyped ::Context };
  use ureq ::Agent;

  ///
  /// Get data of remote package.
  ///
  /// # Errors
  ///
  /// # Panics
  ///
  pub fn download< 'a >( name: &'a str, version: &'a str ) -> error ::untyped ::Result< Vec< u8 > >
  {
  let config = Agent ::config_builder()
  .timeout_global( Some( Duration ::from_secs( 30 ) ) )
  .timeout_send_request( Some( Duration ::from_secs( 5 ) ) )
  .timeout_recv_response( Some( Duration ::from_secs( 5 ) ) )
  .timeout_recv_body( Some( Duration ::from_secs( 5 ) ) )
  .build();
  let agent = Agent ::new_with_config( config );
  // Fix(issue-download-url-malformed-space): stray space after "https:" produced an
  // invalid URI ("http: invalid uri character"), so every call failed before any request
  // was sent. Root cause: unnoticed for years because `download` had zero callers and zero
  // test coverage — first exercised by tests/inc/tool/http_test.rs.
  let mut buf = String ::new();
  write!( &mut buf, "https://static.crates.io/crates/{name}/{name}-{version}.crate" )?;

  let mut resp = agent.get( &buf[ .. ] ).call().context( "Get data of remote package" )?;

  let bytes: Vec< u8 > = resp.body_mut().with_config().limit( u64 ::MAX ).read_to_vec()?;

  Ok( bytes )
 }

}

//

crate ::mod_interface!
{
  orphan use download;
}
