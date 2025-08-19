//! Safe Update Chain Pattern for coordinated markdown section updates
//!
//! This module provides atomic updates for multiple markdown sections,
//! ensuring either all sections update successfully or none do.

use crate::reporting::{ MarkdownUpdater, MarkdownError };
use std::path::Path;

type Result< T > = std::result::Result< T, Box< dyn std::error::Error > >;

/// Errors that can occur during update chain operations
#[ derive( Debug ) ]
pub enum UpdateChainError
{
  /// Error during markdown processing
  Markdown( MarkdownError ),
  /// Error during file I/O operations
  Io( std::io::Error ),
  /// Validation failed - conflicts detected
  ValidationFailed
  {
    /// List of all detected conflicts
    conflicts : Vec< String >
  },
  /// Empty chain - no sections to update
  EmptyChain,
}

impl std::fmt::Display for UpdateChainError
{
  fn fmt( &self, f : &mut std::fmt::Formatter< '_ > ) -> std::fmt::Result
  {
    match self
    {
      UpdateChainError::Markdown( err ) => write!( f, "Markdown error: {}", err ),
      UpdateChainError::Io( err ) => write!( f, "IO error: {}", err ),
      UpdateChainError::ValidationFailed { conflicts } =>
      {
        write!( f, "Validation failed with conflicts: {:?}", conflicts )
      },
      UpdateChainError::EmptyChain => write!( f, "Update chain is empty" ),
    }
  }
}

impl std::error::Error for UpdateChainError
{
  fn source( &self ) -> Option< &( dyn std::error::Error + 'static ) >
  {
    match self
    {
      UpdateChainError::Markdown( err ) => Some( err ),
      UpdateChainError::Io( err ) => Some( err ),
      _ => None,
    }
  }
}

impl From< MarkdownError > for UpdateChainError
{
  fn from( err : MarkdownError ) -> Self
  {
    UpdateChainError::Markdown( err )
  }
}

impl From< std::io::Error > for UpdateChainError
{
  fn from( err : std::io::Error ) -> Self
  {
    UpdateChainError::Io( err )
  }
}

/// Section update information
#[ derive( Debug, Clone ) ]
pub struct SectionUpdate
{
  /// Section name
  pub section_name : String,
  /// New content for the section
  pub content : String,
}

impl SectionUpdate
{
  /// Create new section update
  pub fn new( section_name : impl Into< String >, content : impl Into< String > ) -> Self
  {
    Self
    {
      section_name : section_name.into(),
      content : content.into(),
    }
  }
}

/// Atomic markdown update chain for coordinated section updates
#[ derive( Debug ) ]
pub struct MarkdownUpdateChain
{
  /// Path to the markdown file
  file_path : std::path::PathBuf,
  /// List of section updates to apply
  updates : Vec< SectionUpdate >,
}

impl MarkdownUpdateChain
{
  /// Create new update chain for the specified file
  ///
  /// # Errors
  ///
  /// Returns an error if the file path is invalid.
  pub fn new( file_path : impl AsRef< Path > ) -> Result< Self >
  {
    Ok( Self
    {
      file_path : file_path.as_ref().to_path_buf(),
      updates : Vec::new(),
    })
  }

  /// Add a section update to the chain
  ///
  /// # Example
  ///
  /// ```rust,no_run
  /// use benchkit::update_chain::MarkdownUpdateChain;
  ///
  /// let chain = MarkdownUpdateChain::new( "readme.md" )?
  ///   .add_section( "Performance Benchmarks", "## Results\n\nFast!" )
  ///   .add_section( "Memory Usage", "## Memory\n\nLow usage" );
  /// # Ok::<(), error_tools::Error>(())
  /// ```
  pub fn add_section( mut self, section_name : impl Into< String >, content : impl Into< String > ) -> Self
  {
    self.updates.push( SectionUpdate::new( section_name, content ) );
    self
  }

  /// Check for conflicts across all sections in the chain
  ///
  /// # Errors
  ///
  /// Returns an error if the file cannot be read or conflicts are detected.
  pub fn check_all_conflicts( &self ) -> Result< Vec< String > >
  {
    if self.updates.is_empty()
    {
      return Ok( vec![] );
    }

    let mut all_conflicts = Vec::new();

    for update in &self.updates
    {
      let updater = MarkdownUpdater::new( &self.file_path, &update.section_name )
        .map_err( UpdateChainError::from )?;
      
      let conflicts = updater.check_conflicts()
        .map_err( UpdateChainError::from )?;
      
      all_conflicts.extend( conflicts );
    }

    // Remove duplicates
    all_conflicts.sort();
    all_conflicts.dedup();

    Ok( all_conflicts )
  }

  /// Execute all updates atomically
  ///
  /// Either all sections are updated successfully, or none are modified.
  /// This method uses a backup-and-restore strategy to ensure atomicity.
  ///
  /// # Errors
  ///
  /// Returns an error if:
  /// - The chain is empty
  /// - File operations fail
  /// - Section conflicts are detected
  /// - Any individual update fails
  pub fn execute( &self ) -> Result< () >
  {
    if self.updates.is_empty()
    {
      return Err( Box::new( UpdateChainError::EmptyChain ) );
    }

    // Check for conflicts first
    let conflicts = self.check_all_conflicts()?;
    if !conflicts.is_empty()
    {
      return Err( Box::new( UpdateChainError::ValidationFailed { conflicts } ) );
    }

    // Create backup of original file if it exists
    let backup_path = self.create_backup()?;
    
    // Attempt to apply all updates
    match self.apply_all_updates()
    {
      Ok( () ) =>
      {
        // Success - remove backup
        if let Some( backup ) = backup_path
        {
          let _ = std::fs::remove_file( backup );
        }
        Ok( () )
      },
      Err( e ) =>
      {
        // Failure - restore from backup
        if let Some( backup ) = backup_path
        {
          if let Err( restore_err ) = std::fs::copy( &backup, &self.file_path )
          {
            eprintln!( "⚠️ Failed to restore backup: {}", restore_err );
          }
          let _ = std::fs::remove_file( backup );
        }
        Err( e )
      }
    }
  }

  /// Create backup file and return its path
  fn create_backup( &self ) -> Result< Option< std::path::PathBuf > >
  {
    if !self.file_path.exists()
    {
      return Ok( None );
    }

    let backup_path = self.file_path.with_extension( "bak" );
    std::fs::copy( &self.file_path, &backup_path )
      .map_err( UpdateChainError::from )?;
    
    Ok( Some( backup_path ) )
  }

  /// Apply all updates in sequence
  fn apply_all_updates( &self ) -> Result< () >
  {
    // Read the original content once
    let mut current_content = if self.file_path.exists()
    {
      std::fs::read_to_string( &self.file_path )
        .map_err( UpdateChainError::from )?
    }
    else
    {
      String::new()
    };

    // Apply each update to the accumulating content
    for update in &self.updates
    {
      let updater = MarkdownUpdater::new( &self.file_path, &update.section_name )
        .map_err( UpdateChainError::from )?;
      
      current_content = updater.replace_section_content( &current_content, &update.content );
    }

    // Write the final result in one operation
    std::fs::write( &self.file_path, current_content )
      .map_err( UpdateChainError::from )?;

    Ok( () )
  }

  /// Get the number of pending updates
  #[ must_use ]
  pub fn len( &self ) -> usize
  {
    self.updates.len()
  }

  /// Check if the chain is empty
  #[ must_use ]
  pub fn is_empty( &self ) -> bool
  {
    self.updates.is_empty()
  }

  /// Get the file path for this chain
  #[ must_use ]
  pub fn file_path( &self ) -> &Path
  {
    &self.file_path
  }

  /// Get a reference to the pending updates
  #[ must_use ]
  pub fn updates( &self ) -> &[ SectionUpdate ]
  {
    &self.updates
  }
}