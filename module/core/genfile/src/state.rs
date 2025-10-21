//! Archive state management for REPL sessions
//!
//! Implements stateless design by using thread-safe in-memory archive storage.
//! State is isolated per session and does not persist between invocations.

use genfile_core::TemplateArchive;
use std::sync::{ Arc, RwLock };

/// Thread-safe archive state for REPL mode
///
/// Provides safe concurrent access to the current template archive
/// being edited in a REPL session. Each command can read and modify
/// the archive through this shared state.
///
/// # Examples
///
/// ```ignore
/// use genfile::state::ArchiveState;
/// use genfile_core::TemplateArchive;
///
/// let state = ArchiveState::new();
///
/// // Set archive
/// let mut archive = TemplateArchive::new();
/// archive.set_name( "test" );
/// state._set( archive );
///
/// // Get archive
/// if let Some( archive ) = state._get()
/// {
///   println!( "Archive: {}", archive.name().unwrap_or( "unnamed" ) );
/// }
/// ```
#[ derive( Clone, Debug ) ]
pub struct ArchiveState
{
  #[allow(dead_code)]
  inner : Arc< RwLock< Option< TemplateArchive > > >,
}

impl ArchiveState
{
  /// Create new empty archive state
  #[must_use] 
  pub fn new() -> Self
  {
    Self
    {
      inner : Arc::new( RwLock::new( None ) ),
    }
  }

  /// Get current archive (clone)
  ///
  /// Returns None if no archive is currently loaded.
  #[must_use] 
  pub fn _get( &self ) -> Option< TemplateArchive >
  {
    let guard = self.inner.read().ok()?;
    guard.as_ref().cloned()
  }

  /// Set current archive
  ///
  /// Replaces any existing archive with the new one.
  pub fn _set( &self, archive : TemplateArchive )
  {
    if let Ok( mut guard ) = self.inner.write()
    {
      *guard = Some( archive );
    }
  }

  /// Clear current archive
  ///
  /// Removes the archive from state, returning to empty state.
  pub fn _clear( &self )
  {
    if let Ok( mut guard ) = self.inner.write()
    {
      *guard = None;
    }
  }

  /// Check if an archive is currently loaded
  #[must_use] 
  pub fn _has_archive( &self ) -> bool
  {
    self.inner.read().ok().is_some_and( | g | g.is_some() )
  }
}

impl Default for ArchiveState
{
  fn default() -> Self
  {
    Self::new()
  }
}
