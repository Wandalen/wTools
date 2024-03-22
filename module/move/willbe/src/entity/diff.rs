mod private
{
  use crate::*;

  use std::
  {
    collections::HashSet,
    fmt::Formatter,
    path::PathBuf,
  };
  use colored::Colorize;
  use crates_tools::CrateArchive;

  use wtools::iter::Itertools;

  /// The `Diff` enum is designed to represent differences between two versions
  /// of some kind of item identified.
  #[ derive( Debug ) ]
  pub enum Diff< T >
  {
    /// This variant represents items that are identical or same in both versions.
    Same( T ),
    /// This variant represents items that exists in both versions but have been modified.
    Modified( T ),
    /// This variant represents items that were added.
    Add( T ),
    /// This variant represents items that were removed.
    Rem( T ),
  }

  /// The `DiffReport` struct represents a diff report containing a list of `Diff` objects.
  #[ derive( Debug, Default ) ]
  pub struct DiffReport( Vec< Diff< PathBuf > > );

  impl std::fmt::Display for DiffReport
  {
    fn fmt( &self, f : &mut Formatter< '_ > ) -> std::fmt::Result
    {
      for diff in self.0.iter()
      .sorted_by_key( | d | match d { Diff::Modified( df ) | Diff::Same( df ) | Diff::Rem( df ) | Diff::Add( df ) => df } )
      {
        match diff
        {
          Diff::Same( t ) => writeln!( f, "{}", t.display() )?,
          Diff::Modified( t ) => writeln!( f, "~ {}", t.to_string_lossy().yellow() )?,
          Diff::Add( t ) => writeln!( f, "+ {}", t.to_string_lossy().green() )?,
          Diff::Rem( t ) => writeln!( f, "- {}", t.to_string_lossy().red() )?,
        };
      }

      Ok( () )
    }
  }

  /// Compare two crate archives and create a difference report.
  ///
  /// # Arguments
  ///
  /// * `left` - A reference to the left crate archive.
  /// * `right` - A reference to the right crate archive.
  ///
  /// # Returns
  ///
  /// A `DiffReport` struct representing the difference between the two crate archives.
  pub fn crate_diff( left : &CrateArchive, right : &CrateArchive ) -> DiffReport
  {
    let mut report = DiffReport::default();

    let local_package_files : HashSet< _ > = left.list().into_iter().collect();
    let remote_package_files : HashSet< _ > = right.list().into_iter().collect();

    let local_only = local_package_files.difference( &remote_package_files );
    let remote_only = remote_package_files.difference( &local_package_files );
    let both = local_package_files.intersection( &remote_package_files );

    for &path in local_only
    {
      report.0.push( Diff::Add( path.to_path_buf() ) );
    }

    for &path in remote_only
    {
      report.0.push( Diff::Rem( path.to_path_buf() ) );
    }

    for &path in both
    {
      // unwraps are safe because the paths to the files was compared previously
      let local = left.content_bytes( path ).unwrap();
      let remote = right.content_bytes( path ).unwrap();

      if local == remote
      {
        report.0.push( Diff::Same( path.to_path_buf() ) );
      }
      else
      {
        report.0.push( Diff::Modified( path.to_path_buf() ) );
      }
    }

    report
  }
}

//

crate::mod_interface!
{
  protected use Diff;
  protected use DiffReport;
  protected use crate_diff;
}