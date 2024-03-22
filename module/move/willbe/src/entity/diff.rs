mod private
{
  use crate::*;

  use std::
  {
    collections::HashSet,
    fmt::Formatter,
    path::PathBuf,
  };
  use std::collections::HashMap;
  use colored::Colorize;
  use crates_tools::CrateArchive;
  use similar::*;

  use wtools::iter::Itertools;


  /// The `Diff` enum is designed to represent differences between two versions
  /// of some kind of item identified.
  #[ derive( Debug ) ]
  pub enum Diff< T >
  {
    /// This variant represents items that are identical or same in both versions.
    Same( T ),
    /// This variant represents items that were added.
    Add( T ),
    /// This variant represents items that were removed.
    Rem( T ),
  }

  /// The `DiffItem` enum is designed to represent differences between two versions
  /// of an item. It contains two variants `File` and `Content`.
  #[ derive( Debug ) ]
  pub enum DiffItem
  {
    /// - `File(Diff<()>)`: Represents differences in the file itself. The `Diff` enum
    ///   contains three possible variants `Same`, `Add`, and `Rem`. Each variant of `Diff`
    ///   represents the status of the file.
    ///   - `Same(())`: Represents that the file is identical or the same in both versions.
    ///   - `Add(())`: Represents that the file was added in the new version.
    ///   - `Rem(())`: Represents that the file was removed in the new version.
    File( Diff< () > ),
    /// - `Content(Vec<Diff<String>>): Represents differences in the content of the item.
    ///   The `Diff` enum inside `Vec` represents differences in strings present in the file.
    ///   The `Diff` enum contains three possible variants `Same`, `Add`, and `Rem`. Each variant
    ///   of `Diff` represents the status of the string.
    ///   - `Same(String)`: Represents that the string is identical or the same in both versions.
    ///   - `Add(String)`: Represents that the string was added in the new version.
    ///   - `Rem(String)`: Represents that the string was removed in the new version.
    Content( Vec< Diff< String > > ),
  }

  /// The `DiffReport` struct represents a diff report containing a list of `Diff` objects.
  #[ derive( Debug, Default ) ]
  pub struct DiffReport( pub( crate ) HashMap< PathBuf, DiffItem > );

  impl std::fmt::Display for DiffReport
  {
    fn fmt( &self, f : &mut Formatter< '_ > ) -> std::fmt::Result
    {
      for ( path , diff ) in self.0.iter().sorted_by_key( |( k, _ )| k.as_path() )
      {
        match diff
        {
          DiffItem::File( item ) =>
          {
            match item
            {
              Diff::Same( _ ) => writeln!( f, " {}", path.display() )?,
              Diff::Add( _ ) => writeln!( f, "+ {} NEW", path.to_string_lossy().green() )?,
              Diff::Rem( _ ) => writeln!( f, "- {} REMOVED", path.to_string_lossy().red() )?,
            };
          }
          DiffItem::Content( items ) =>
          {
            let path = path.to_string_lossy();
            let len = path.len() + "~  MODIFIED".len();
            writeln!( f, "~ {} MODIFIED", path.yellow() )?;
            writeln!( f, "{}", "=".repeat( len + 2 ) )?;
            for item in items
            {
              match item
              {
                Diff::Same( t ) => write!( f, "|   {}", t )?,
                Diff::Add( t ) => write!( f, "| + {}", t.green() )?,
                Diff::Rem( t ) => write!( f, "| - {}", t.red() )?,
              };
            }
            writeln!( f, "{}", "=".repeat( len + 2 ) )?;
          }
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
      report.0.insert( path.to_path_buf(), DiffItem::File( Diff::Add( () ) ) );
    }

    for &path in remote_only
    {
      report.0.insert( path.to_path_buf(), DiffItem::File( Diff::Rem( () ) ) );
    }

    for &path in both
    {
      // unwraps are safe because the paths to the files was compared previously
      let local = left.content_bytes( path ).unwrap();
      let remote = right.content_bytes( path ).unwrap();

      if local == remote
      {
        report.0.insert( path.to_path_buf(), DiffItem::File( Diff::Same( () ) ) );
      }
      else
      {
        let mut items = vec![];
        let local_str = String::from_utf8_lossy( local );
        let remote_str = String::from_utf8_lossy( remote );
        let diff = TextDiff::from_lines( &remote_str, &local_str );
        for hunk in diff.unified_diff().context_radius( 5 ).iter_hunks()
        {
          for change in hunk.iter_changes()
          {
            let item = match change.tag()
            {
              ChangeTag::Delete => Diff::Rem( change.to_string() ),
              ChangeTag::Insert => Diff::Add( change.to_string() ),
              ChangeTag::Equal => Diff::Same( change.to_string() ),
            };
            items.push( item );
          }
        }
        report.0.insert( path.to_path_buf(), DiffItem::Content( items ) );
      }
    }

    report
  }
}

//

crate::mod_interface!
{
  protected use Diff;
  protected use DiffItem;
  protected use DiffReport;
  protected use crate_diff;
}