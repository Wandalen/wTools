// module/move/willbe/src/command/crate_doc.rs
mod private
{

  use crate::*;

  use std::path::PathBuf;
  use wca::VerifiedCommand;
  use error::untyped::Error; // Use untyped::Error for the command return
  use entity::{ Workspace, WorkspaceInitError, PathError }; // Import Workspace, WorkspaceInitError, PathError
  use pth::{ AbsolutePath, CurrentPath }; // Import AbsolutePath and CurrentPath from pth
  // Explicit import for Result and its variants for pattern matching
  use std::result::Result::{Ok, Err};

  ///
  /// Generate documentation for a crate in a single Markdown file.
  ///
  /// # Errors
  /// Returns an error if the command arguments are invalid, the workspace cannot be loaded,
  /// or if the documentation generation action fails.
  #[allow(clippy::needless_pass_by_value)]
  pub fn crate_doc( o : VerifiedCommand ) -> error::untyped::Result< () >
  {
    let path_arg : PathBuf = o.args.get_owned( 0 ).unwrap_or_else( || "./".into() );

    // qqq : xxx : refactor this block
    //       Use the requested `pth::absolute::join` function (see qqq in pth/src/lib.rs)
    //       to simplify this path resolution. The call should look something like:
    //       `let absolute_path = pth::absolute::join( ( CurrentPath, path_arg.clone() ) )?`
    //       This assumes `join_absolute` takes a tuple and handles the logic internally.
    // Determine the absolute path explicitly
    let absolute_path = if path_arg.is_relative()
    {
      // If relative, resolve it against the current directory
      let current_dir = AbsolutePath::try_from( CurrentPath )
        .map_err( | e | Error::new( e ).context( "Failed to get current directory" ) )?;
      current_dir.join( path_arg.clone() ) // Clone path_arg as join consumes it
    }
    else
    {
      // If already absolute, try to create AbsolutePath directly
      AbsolutePath::try_from( path_arg.clone() )
        .map_err( | e | Error::new( e ).context( format!( "Invalid absolute path provided: {}", path_arg.display() ) ) )?
    };
    // Note: AbsolutePath::try_from also performs canonicalization implicitly via path::canonicalize

    // Create CrateDir from the verified AbsolutePath
    let crate_dir = CrateDir::try_from( absolute_path ) // This should now work as AbsolutePath is canonical
      .map_err( | e : PathError | Error::new( e ).context( "Failed to identify crate directory (does Cargo.toml exist?)" ) )?;

    // Load the workspace based on the crate directory
    let workspace = Workspace::try_from( crate_dir.clone() )
      .map_err( | e : WorkspaceInitError | Error::new( e ).context( "Failed to load workspace information" ) )?;

    // Parse output property
    let output_path_req : Option< PathBuf > = o.props.get_owned( "output" );

    // Call the action, passing the workspace reference
    match action::crate_doc::doc( &workspace, &crate_dir, output_path_req )
    {
      Ok( report ) =>
      {
        println!( "{report}" ); // Print the success report
        Ok( () )
      }
      Err( ( report, e ) ) =>
      {
        eprintln!( "{report}" ); // Print the report even on failure
        // Convert the specific CrateDocError into a general untyped::Error for the command return
        Err( Error::new( e ).context( "Documentation generation failed" ) )
      }
    }
  }
}

crate::mod_interface!
{
  /// Generate documentation for a crate.
  orphan use crate_doc;
}