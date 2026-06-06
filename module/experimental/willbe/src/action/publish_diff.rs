/// Define a private namespace for all its items.
#[ allow( clippy ::std_instead_of_alloc, clippy ::std_instead_of_core ) ]
mod private
{

  use crate :: *;
  use std ::path ::PathBuf;
  use collection_tools ::collection ::HashMap;
  use std ::fmt;
  use colored ::Colorize;
  use crates_tools ::CrateArchive;
  use action ::list ::ListReport;
  use error ::untyped ::Result;
  use diff :: { DiffReport, crate_diff };
  use error ::untyped ::format_err;
  use tool ::ListNodeReport;
  use tool ::TreePrinter;

  /// Options for `publish_diff` command
  #[ derive( Debug, former ::Former ) ]
  pub struct PublishDiffOptions
  {
  path: PathBuf,
  keep_archive: Option< PathBuf >,
 }

  #[ derive( Debug ) ]
  pub struct PublishDiffReport
  {
  pub diffs: HashMap< AbsolutePath, DiffReport >,
  pub root_path: AbsolutePath,
  pub tree: ListNodeReport,
 }

  impl std ::fmt ::Display for PublishDiffReport
  {
  fn fmt( &self, f: &mut fmt ::Formatter< '_ > ) -> std ::fmt ::Result
  {
   let mut tree = self.tree.clone();
   let root_path = tree.crate_dir.as_ref().unwrap().clone();
   let root_name = tree.name.clone();
   let root_version = tree.version.as_ref().unwrap().clone();

   #[ allow( clippy ::items_after_statements, clippy ::option_map_unit_fn ) ]
   fn modify( diffs: &HashMap< AbsolutePath, DiffReport >, tree: &mut ListNodeReport )
   {
  let path = tree.crate_dir.take().unwrap();
  let root = AbsolutePath ::from( path );

  let diff = diffs.get( &root ).unwrap();

  let has_changes = diff.has_changes();
  tree.name = if has_changes
  {
   format!( "{}", tree.name.yellow() )
 }
  else
  {
   tree.name.clone()
 };
  tree
  .version
  .as_mut()
  .map
  (
   | v |
   *v = format!
   (
  "{} {}",
  if has_changes 
  { v.yellow() } else { v.as_str().into() },
  if has_changes 
  { "MODIFIED" } else { "" }
 )
 );

  for dep in &mut tree.normal_dependencies
  {
   modify( diffs, dep );
 }
 }
   modify( &self.diffs, &mut tree );

   let root = AbsolutePath ::from( root_path );
   let diff = self.diffs.get( &root ).unwrap();
   let printer = TreePrinter ::new( &tree );
   writeln!( f, "Tree: \n{printer}" )?;
   if diff.has_changes()
   {
  writeln!( f, "Changes detected in `{root_name} {root_version}` : " )?;
 }
   else
   {
  writeln!( f, "No changes found in `{root_name} {root_version}`. Files: " )?;
 }
   write!( f, "{diff}" )?;

   std ::fmt ::Result ::Ok( () )
 }
 }

  /// Return the differences between a local and remote package versions.
  ///
  /// # Errors
  ///
  /// Returns an error if there's an issue with path conversion, packing the local crate,
  /// downloading the published version from crates.io (e.g., 403 Forbidden, 404 Not Found, network errors),
  /// or if the internal `list` action returns an unexpected format.
  ///
  /// # Panics
  ///
  /// This function may panic if the internal `list_all` action fails or if a dependency tree walk
  /// encounters an unexpected structure.
  #[ cfg_attr( feature = "tracing", tracing ::instrument ) ]
  pub fn publish_diff( o: PublishDiffOptions ) -> Result< PublishDiffReport >
  {
  let path = AbsolutePath ::try_from( o.path )?;
  let dir = CrateDir ::try_from( path.clone() )?;

  let workspace = Workspace ::try_from( dir.clone() )?;

  let list = action ::list_all
  (
   action ::list ::ListOptions ::former()
   .path_to_manifest( dir )
   .format( action ::list ::ListFormat ::Tree )
   .info( [ action ::list ::PackageAdditionalInfo ::Version, action ::list ::PackageAdditionalInfo ::Path ] )
   .dependency_sources( [ action ::list ::DependencySource ::Local ] )
   .dependency_categories( [ action ::list ::DependencyCategory ::Primary ] )
   .form()
 )
  .unwrap();
  let ListReport ::Tree( tree ) = list
  else
  {
   return Err( format_err!( "Logical error. Unexpected list format" ) )
 };
  let mut tasks = vec![ tree[ 0 ].clone() ];
  let mut diffs = HashMap ::new();
  let mut current_idx = 0;
  while current_idx < tasks.len()
  {
   // let path = tasks[ current_idx ].crate_dir.as_ref().unwrap().to_string_lossy();
   let path = tasks[ current_idx ]
   .info
   .crate_dir
   .as_ref()
   .unwrap()
   .clone()
   .absolute_path();
   let dir = CrateDir ::try_from( path.clone() )?;

   let package = package ::Package ::try_from( dir.clone() )?;
   let name = &package.name()?;
   let version = &package.version()?;

   _ = cargo ::pack
   (
  cargo ::PackOptions ::former()
  .path( dir.as_ref() )
  .allow_dirty( true )
  .checking_consistency( false )
  .dry( false ).form()
 )?;
   let l = CrateArchive ::read( packed_crate ::local_path( name, version, workspace.target_directory() )? )?;
   // Fix(issue-publish-diff-403): Handle download errors gracefully instead of panicking
   // Root cause: Using .unwrap() on download_crates_io Result causes panic on HTTP errors (403, 404, network failures)
   // Pitfall: Never unwrap() network operations - they can fail for many legitimate reasons (unpublished crates, network issues, rate limiting)
   let r = CrateArchive ::download_crates_io( name, version )
   .map_err( | e | format_err!( "Failed to download published version of '{name} {version}' from crates.io. This may occur if the crate hasn't been published yet, access is restricted, or due to network issues. Error: {e}" ) )?;


   if let Some( out_path ) = &o.keep_archive
   {
  _ = std ::fs ::create_dir_all( out_path );
  for path in r.list()
  {
   let local_path = out_path.join( path );
   let folder = local_path.parent().unwrap();
   _ = std ::fs ::create_dir_all( folder );

   let content = r.content_bytes( path ).unwrap();

   std ::fs ::write( local_path, content )?;
 }
 }
   diffs.insert( path, crate_diff( &l, &r ).exclude( diff ::PUBLISH_IGNORE_LIST ) );
   let report = tasks[ current_idx ].info.normal_dependencies.clone();
   let printer: Vec< TreePrinter > = report
   .iter()
   .map( TreePrinter ::new )
   .collect();
   tasks.extend( printer );

   current_idx += 1;
 }
  let printer = tree;
  let mut rep: Vec< ListNodeReport > = printer
  .iter()
  .map( | printer | printer.info.clone() )
  .collect();
  let report = PublishDiffReport
  {
   root_path: path.clone(),
   diffs,
   tree: rep.remove( 0 ),
 };

  Ok( report )
 }
}

//

crate ::mod_interface!
{
  orphan use PublishDiffOptions;
  /// Publishes the difference between the local and published versions of a package.
  orphan use publish_diff;
}
