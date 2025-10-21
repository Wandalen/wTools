//! Shared state for all command handlers
//!
//! Provides thread-local storage for the current archive that is shared
//! across all handler modules (archive, file, etc.)

use core::cell::RefCell;
use genfile_core::TemplateArchive;

thread_local! {
  static CURRENT_ARCHIVE : RefCell< Option< TemplateArchive > > = const { RefCell::new( None ) };
}

pub fn get_current_archive() -> Option< TemplateArchive >
{
  CURRENT_ARCHIVE.with( | arc | arc.borrow().clone() )
}

pub fn set_current_archive( archive : TemplateArchive )
{
  CURRENT_ARCHIVE.with( | arc | *arc.borrow_mut() = Some( archive ) );
}

#[allow(dead_code)]
pub fn clear_current_archive()
{
  CURRENT_ARCHIVE.with( | arc | *arc.borrow_mut() = None );
}
