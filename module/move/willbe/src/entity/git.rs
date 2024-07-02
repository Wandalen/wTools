mod private
{
  use crate::*;
  
  use std::fmt;
  use process_tools::process;
  use error::
  {
    Result,
    untyped::{ format_err, Context },
  };

  #[ derive( Debug, Default, Clone ) ]
  /// Represents an extended Git report with optional process reports.
  pub struct ExtendedGitReport
  {
    /// Optional report for the `git add` process.
    pub add : Option< process::Report >,
    /// Optional report for the `git commit` process.
    pub commit : Option< process::Report >,
    /// Optional report for the `git push` process.
    pub push : Option< process::Report >,
  }

  impl fmt::Display for ExtendedGitReport
  {
    fn fmt( &self, f : &mut fmt::Formatter<'_> ) -> fmt::Result
    {
      let Self { add, commit, push } = &self;

      if let Some( add ) = add { writeln!( f, "{add}" )? }
      if let Some( commit ) = commit { writeln!( f, "{commit}" )? }
      if let Some( push ) = push { writeln!( f, "{push}" )? }

      Ok( () )
    }
  }

  // aaa : for Bohdan : should not be here // aaa : done
  // aaa : for Bohdan : documentation // aaa : done
  /// The `GitOptions` struct represents a set of options used to perform a Git commit operation.
  #[ derive( Debug, Clone ) ]
  pub struct GitOptions
  {
    /// An absolute path to the root directory of the Git repository.
    pub git_root : AbsolutePath,
    /// A vector of absolute paths to the files or directories that should be committed.
    pub items : Vec< AbsolutePath >,
    /// A string containing the commit message.
    pub message : String,
    /// A boolean flag indicating whether the commit should be performed in dry run mode
    /// (i.e., no changes are actually made to the repository)
    pub dry : bool,
  }

  // aaa : for Bohdan : should not be here // aaa : done
  // aaa : for Bohdan : documentation // aaa : done
  /// Performs a Git commit operation using the provided options
  pub fn perform_git_commit( o : GitOptions ) -> Result< ExtendedGitReport >
  {
    let mut report = ExtendedGitReport::default();
    if o.items.is_empty() { return Ok( report ); }
    let items = o
    .items
    .iter()
    .map
    (
      | item | item.as_ref().strip_prefix( o.git_root.as_ref() ).map( std::path::Path::to_string_lossy )
      .with_context( || format!("git_root: {}, item: {}", o.git_root.as_ref().display(), item.as_ref().display() ) )
    )
    .collect::< Result< Vec< _ > > >()?;
    let res = tool::git::add( &o.git_root, &items, o.dry ).map_err( | e | format_err!( "{report}\n{e}" ) )?;
    report.add = Some( res );
    let res = tool::git::commit( &o.git_root, &o.message, o.dry ).map_err( | e | format_err!( "{report}\n{e}" ) )?;
    report.commit = Some( res );

    Ok( report )
  }
}

//

crate::mod_interface!
{
  protected use ExtendedGitReport;
  protected use GitOptions;
  protected use perform_git_commit;
}
