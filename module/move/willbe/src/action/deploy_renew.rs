//! Deploy template generation using `genfile_core`.
//!
//! # Architecture Decision: `genfile_core` Integration
//!
//! **Migration Date:** 2025-10-19
//!
//! Migrated from custom template.rs to `genfile_core` library, inheriting advanced features
//! like `WriteMode::TomlExtend` for smart TOML merging.
//!
//! # Key Patterns Specific to Deploy Templates
//!
//! ## TOML Parameter Persistence
//!
//! Deploy templates use `.deploy_template.toml` for parameter persistence across runs.
//! The `load_existing_params()` helper loads previously-set values:
//!
//! ```rust,ignore
//! fn load_existing_params(archive: &mut TemplateArchive, path: &Path) -> Option<()> {
//!   let data = fs::read_to_string(path.join(".deploy_template.toml")).ok()?;
//!   // Parse TOML and populate archive values...
//! }
//! ```
//!
//! ## Smart TOML Merging
//!
//! Uses `WriteMode::TomlExtend` to merge new template content with existing files:
//!
//! ```rust,ignore
//! archive.add_text_file(
//!   PathBuf::from("./.deploy_template.toml"),
//!   include_str!("../../template/deploy/.deploy_template.toml.hbs"),
//!   WriteMode::TomlExtend  // Merges instead of replacing
//! );
//! ```
//!
//! ## Borrow Checker Pattern
//!
//! To avoid simultaneous immutable/mutable borrows when iterating parameters:
//!
//! ```rust,ignore
//! // Clone parameter names first
//! let param_names: Vec<String> = archive.parameters.descriptors
//!   .iter()
//!   .map(|d| d.parameter.clone())
//!   .collect();
//!
//! // Now safe to mutate archive in loop
//! for param in param_names {
//!   archive.set_value(&param, value);
//! }
//! ```
//!
//! This pattern is essential when both reading from and writing to the archive.

mod private
{

  use crate :: *;
  use std ::
  {
  fs,
  path :: { Path, PathBuf },
 };
  use error ::untyped ::Context;

  use genfile_core ::
  {
  TemplateArchive,
  WriteMode,
  Value,
  ParameterDescriptor,
 };

  /// Template for creating deploy files.
  ///
  /// Includes terraform deploy options to GCP, and Hetzner,
  /// a Makefile for useful commands, and a key directory.
  ///
  /// Uses `genfile_core` with `WriteMode::TomlExtend` for parameter persistence.
  /// See module documentation for migration details and patterns.
  #[ derive( Debug ) ]
  pub struct DeployTemplate;

  impl DeployTemplate
  {
  /// Creates an instance of `TemplateArchive` for deployment template.
  ///
  /// Used for properly initializing a template
  #[ must_use ]
  #[ allow( clippy ::should_implement_trait, clippy ::too_many_lines ) ]
  pub fn default() -> TemplateArchive
  {
   let mut archive = TemplateArchive ::new( "deploy" );

   // Define parameters
   archive.add_parameter
   (
  ParameterDescriptor
  {
   parameter: "gcp_project_id".into(),
   is_mandatory: true,
   default_value: None,
   description: None,
 }
 );
   archive.add_parameter
   (
  ParameterDescriptor
  {
   parameter: "gcp_region".into(),
   is_mandatory: false,
   default_value: None,
   description: None,
 }
 );
   archive.add_parameter
   (
  ParameterDescriptor
  {
   parameter: "gcp_artifact_repo_name".into(),
   is_mandatory: false,
   default_value: None,
   description: None,
 }
 );
   archive.add_parameter
   (
  ParameterDescriptor
  {
   parameter: "docker_image_name".into(),
   is_mandatory: false,
   default_value: None,
   description: None,
 }
 );

   add_deploy_template_files( &mut archive );

   archive
 }
 }

  #[ allow( clippy ::too_many_lines ) ]
  fn add_deploy_template_files( archive: &mut TemplateArchive )
  {
  // root
  archive.add_text_file
  (
   PathBuf ::from( "./.deploy_template.toml" ),
   include_str!( "../../template/deploy/.deploy_template.toml.hbs" ),
   WriteMode ::TomlExtend
 );
  archive.add_text_file
  (
   PathBuf ::from( "./Makefile" ),
   include_str!( "../../template/deploy/Makefile.hbs" ),
   WriteMode ::Rewrite
 );
  // /key
  archive.add_text_file
  (
   PathBuf ::from( "./key/pack.sh" ),
   include_str!( "../../template/deploy/key/pack.sh" ),
   WriteMode ::Rewrite
 );
  archive.add_text_file
  (
   PathBuf ::from( "./key/readme.md" ),
   include_str!( "../../template/deploy/key/readme.md" ),
   WriteMode ::Rewrite
 );
  // /deploy/
  archive.add_text_file
  (
   PathBuf ::from( "./deploy/redeploy.sh" ),
   include_str!( "../../template/deploy/deploy/redeploy.sh" ),
   WriteMode ::Rewrite
 );
  archive.add_text_file
  (
   PathBuf ::from( "./deploy/cloud-init.tpl" ),
   include_str!( "../../template/deploy/deploy/cloud-init.tpl.hbs" ),
   WriteMode ::Rewrite
 );
  archive.add_text_file
  (
   PathBuf ::from( "./deploy/Dockerfile" ),
   include_str!( "../../template/deploy/deploy/Dockerfile" ),
   WriteMode ::Rewrite
 );
  archive.add_text_file
  (
   PathBuf ::from( "./deploy/readme.md" ),
   include_str!( "../../template/deploy/deploy/readme.md" ),
   WriteMode ::Rewrite
 );
  // /deploy/gar
  archive.add_text_file
  (
   PathBuf ::from( "./deploy/gar/readme.md" ),
   include_str!( "../../template/deploy/deploy/gar/readme.md" ),
   WriteMode ::Rewrite
 );
  archive.add_text_file
  (
   PathBuf ::from( "./deploy/gar/main.tf" ),
   include_str!( "../../template/deploy/deploy/gar/main.tf.hbs" ),
   WriteMode ::Rewrite
 );
  archive.add_text_file
  (
   PathBuf ::from( "./deploy/gar/outputs.tf" ),
   include_str!( "../../template/deploy/deploy/gar/outputs.tf" ),
   WriteMode ::Rewrite
 );
  archive.add_text_file
  (
   PathBuf ::from( "./deploy/gar/variables.tf" ),
   include_str!( "../../template/deploy/deploy/gar/variables.tf" ),
   WriteMode ::Rewrite
 );
  // /deploy/gce
  archive.add_text_file
  (
   PathBuf ::from( "./deploy/gce/readme.md" ),
   include_str!( "../../template/deploy/deploy/gce/readme.md" ),
   WriteMode ::Rewrite
 );
  archive.add_text_file
  (
   PathBuf ::from( "./deploy/gce/main.tf" ),
   include_str!( "../../template/deploy/deploy/gce/main.tf.hbs" ),
   WriteMode ::Rewrite
 );
  archive.add_text_file
  (
   PathBuf ::from( "./deploy/gce/outputs.tf" ),
   include_str!( "../../template/deploy/deploy/gce/outputs.tf.hbs" ),
   WriteMode ::Rewrite
 );
  archive.add_text_file
  (
   PathBuf ::from( "./deploy/gce/variables.tf" ),
   include_str!( "../../template/deploy/deploy/gce/variables.tf" ),
   WriteMode ::Rewrite
 );
  // /deploy/gcs
  archive.add_text_file
  (
   PathBuf ::from( "./deploy/gcs/main.tf" ),
   include_str!( "../../template/deploy/deploy/gcs/main.tf" ),
   WriteMode ::Rewrite
 );
  // /deploy/hetzner
  archive.add_text_file
  (
   PathBuf ::from( "./deploy/hetzner/main.tf" ),
   include_str!( "../../template/deploy/deploy/hetzner/main.tf.hbs" ),
   WriteMode ::Rewrite
 );
  archive.add_text_file
  (
   PathBuf ::from( "./deploy/hetzner/outputs.tf" ),
   include_str!( "../../template/deploy/deploy/hetzner/outputs.tf.hbs" ),
   WriteMode ::Rewrite
 );
  archive.add_text_file
  (
   PathBuf ::from( "./deploy/hetzner/variables.tf" ),
   include_str!( "../../template/deploy/deploy/hetzner/variables.tf" ),
   WriteMode ::Rewrite
 );
  // /deploy/aws
  archive.add_text_file
  (
   PathBuf ::from( "./deploy/aws/main.tf" ),
   include_str!( "../../template/deploy/deploy/aws/main.tf" ),
   WriteMode ::Rewrite
 );
  archive.add_text_file
  (
   PathBuf ::from( "./deploy/aws/outputs.tf" ),
   include_str!( "../../template/deploy/deploy/aws/outputs.tf" ),
   WriteMode ::Rewrite
 );
  archive.add_text_file
  (
   PathBuf ::from( "./deploy/aws/variables.tf" ),
   include_str!( "../../template/deploy/deploy/aws/variables.tf" ),
   WriteMode ::Rewrite
 );
 }

  fn dir_name_to_formatted( dir_name: &str, separator: &str ) -> String
  {
  dir_name
  .replace( [ ' ', '_' ], separator )
  .to_lowercase()
 }

  /// Load existing parameters from TOML file into template archive.
  ///
  /// Looks for a `.deploy_template.toml` file in the specified path
  /// and loads parameter values from the `[deploy]` section.
  fn load_existing_params( archive: &mut TemplateArchive, path: &Path ) -> Option< () >
  {
  let param_file = path.join( ".deploy_template.toml" );
  let data = fs ::read_to_string( param_file ).ok()?;
  let document = data.parse :: < toml_edit ::Document >().ok()?;
  let template_table = document.get( "deploy" )?;

  // Clone parameter names to avoid borrow checker issues
  let param_names: Vec< String > = archive.parameters.descriptors
  .iter()
  .map( | d | d.parameter.clone() )
  .collect();

  for param in param_names
  {
   if let Some( value ) = template_table.get( &param )
   {
  if let Some( str_value ) = value.as_str()
  {
   // Only set if not already set
   if archive.get_value( &param ).is_none()
   {
  archive.set_value( &param, Value ::String( str_value.to_string() ) );
 }
 }
 }
 }

  Some( () )
 }

  /// Creates deploy template
  /// # Errors
  /// qqq: doc
  pub fn deploy_renew
  (
  path: &Path,
  mut template: TemplateArchive
 )
  -> error ::untyped ::Result< () >
  // qqq: typed error
  {
  if load_existing_params( &mut template, path ).is_none()
  {
   let current_dir = std ::env ::current_dir()?;
   // qqq: for Petro: use file_name
   // qqq: for Kos: bad description
   let current_dir = current_dir
   .components()
   .next_back()
   .context( "Invalid current directory" )?;

   let current_dir = current_dir.as_os_str().to_string_lossy();
   let artifact_repo_name = dir_name_to_formatted( &current_dir, "-" );
   let docker_image_name = dir_name_to_formatted( &current_dir, "_" );

   // Set defaults only if not already set
   if template.get_value( "gcp_artifact_repo_name" ).is_none()
   {
  template.set_value( "gcp_artifact_repo_name", Value ::String( artifact_repo_name ) );
 }
   if template.get_value( "docker_image_name" ).is_none()
   {
  template.set_value( "docker_image_name", Value ::String( docker_image_name ) );
 }
   if template.get_value( "gcp_region" ).is_none()
   {
  template.set_value( "gcp_region", Value ::String( "europe-central2".into() ) );
 }
 }

  // Materialize the template
  template.materialize( path ).map_err( | e | error ::untyped ::format_err!( "{}", e ) )?;

  Ok( () )
 }

}

crate ::mod_interface!
{
  orphan use deploy_renew;
  orphan use DeployTemplate;
}
