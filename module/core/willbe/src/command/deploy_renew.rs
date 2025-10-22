//! Deploy command implementation with wca::Value → genfile_core::Value conversion.
//!
//! # Architecture: Value Type Conversion
//!
//! This module bridges between willbe's command-line interface (using `wca::Value`)
//! and genfile_core's template engine (using `genfile_core::Value`).
//!
//! ## Conversion Pattern
//!
//! The `values_from_props()` function converts between type systems:
//!
//! ```rust,ignore
//! wca::Value::String(s) => Value::String(s.clone())
//! wca::Value::Number(n) => Value::Number(*n as i64)  // f64 → i64
//! wca::Value::Bool(b) => Value::Bool(*b)
//! wca::Value::Path(_) | wca::Value::List(_) => continue  // Unsupported
//! ```
//!
//! **Note:** f64 → i64 cast is intentional and marked with `#[allow(clippy::cast_possible_truncation)]`.
//! genfile_core uses i64 for numbers, while wca uses f64 for JSON compatibility.
//!
//! ## Interactive Parameter Collection
//!
//! The `interactive_if_empty()` function prompts users for missing mandatory parameters,
//! implementing a CLI workflow for template configuration.

mod private
{

  use crate :: *;
  use std ::io :: { self, Write };

  use wca ::VerifiedCommand;
  use error :: { untyped ::Context };
  use genfile_core ::Value;

  use action ::deploy_renew :: *;

  /// Helper to get missing mandatory parameters from archive.
  ///
  /// Filters archive parameters to find mandatory ones without values set.
  fn get_missing_mandatory( archive: &genfile_core ::TemplateArchive ) -> Vec< String >
  {
  archive
  .parameters
  .descriptors
  .iter()
  .filter( | d | d.is_mandatory && archive.get_value( &d.parameter ).is_none() )
  .map( | d | d.parameter.clone() )
  .collect()
 }

  /// Interactively prompts user for a parameter value if not set.
  fn interactive_if_empty( archive: &mut genfile_core ::TemplateArchive, key: &str )
  {
  if archive.get_value( key ).is_none()
  {
   println!( "Parameter `{key}` is not set" );
   print!( "Enter value: " );
   io ::stdout().flush().unwrap();
   let mut answer = String ::new();
   io ::stdin().read_line( &mut answer ).unwrap();
   let answer = answer.trim().to_string();
   archive.set_value( key, Value ::String( answer ) );
 }
 }

  /// Extract values from wca props and set them in the archive.
  fn values_from_props
  (
  archive: &mut genfile_core ::TemplateArchive,
  props: &wca ::executor ::Props
 )
  {
  // Clone parameter names to avoid borrow checker issues
  let param_names: Vec< String > = archive.parameters.descriptors
  .iter()
  .map( | d | d.parameter.clone() )
  .collect();

  for param in param_names
  {
   if let Some( wca_value ) = props.get( &param )
   {
  // Convert wca::Value to genfile_core::Value
  #[ allow( clippy ::cast_possible_truncation ) ]
  let value = match wca_value
  {
   wca ::Value ::String( s ) => Value ::String( s.clone() ),
   wca ::Value ::Number( n ) => Value ::Number( *n as i64 ),
   wca ::Value ::Bool( b ) => Value ::Bool( *b ),
   wca ::Value ::Path( _ ) | wca ::Value ::List( _ ) => continue, // Skip path and list values
 };
  archive.set_value( &param, value );
 }
 }
 }

  ///
  /// Create new deploy.
  ///
  /// # Errors
  /// qqq: doc
  // xxx: qqq: typed error
  #[ allow( clippy ::needless_pass_by_value ) ]
  pub fn deploy_renew( o: VerifiedCommand ) -> error ::untyped ::Result< () >
  {
  let current_dir = std ::env ::current_dir()?;

  let mut template = DeployTemplate ::default();

  // Extract values from command props
  values_from_props( &mut template, &o.props );

  // Prompt for missing mandatory parameters
  for mandatory in get_missing_mandatory( &template )
  {
   interactive_if_empty( &mut template, &mandatory );
 }

  action ::deploy_renew( &current_dir, template )
  .context( "Fail to create deploy template" )
 }

}

crate ::mod_interface!
{
  /// Create deploy from template.
  orphan use deploy_renew;
}

