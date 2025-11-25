//! Shared state for all command handlers
//!
//! Provides thread-local storage for the current archive that is shared
//! across all handler modules (archive, file, etc.)
//!
//! # ⚠️ TEMPORARY WORKAROUND IMPLEMENTATION
//!
//! **Status:** This is the ACTUAL state management implementation currently in use.
//!
//! **Why This Exists:** `unilang::ExecutionContext` does not yet support passing custom
//! state to command handlers (see TODOs in main.rs:42, repl.rs:79). This thread-local
//! storage provides a workaround to share archive state across handler function calls.
//!
//! **Architectural Intent:** The proper design uses `state::ArchiveState` (`Arc<RwLock<>>`)
//! passed through `ExecutionContext`. See state.rs module docs for full explanation.
//!
//! **Trade-offs of Thread-Local Approach:**
//! - ✅ Works with current unilang API constraints
//! - ✅ Simple, no lifetime/borrow complexity
//! - ⚠️ Thread-isolated (each thread has separate state)
//! - ⚠️ Global mutable state (less explicit than passing through context)
//! - ⚠️ Architectural debt vs. specification design
//!
//! **Migration Path:** When `ExecutionContext` gains state support, migrate handlers to
//! use `state::ArchiveState` and delete this module entirely.
//!
//! ## Architectural Validation (2025-01-25)
//!
//! Comprehensive exploration of unilang codebase confirms this approach is correct:
//!
//! - **`ExecutionContext` Status**: Currently empty placeholder struct with no fields
//! - **Unilang Philosophy**: Explicitly stateless by design per pipeline.rs docs
//! - **Thread-Local Blessing**: genfile's main.rs:42 and repl.rs:79 TODOs acknowledge
//!   this is the intended workaround until `ExecutionContext` API evolves
//! - **Use Case Fit**: Single-threaded REPL doesn't require cross-thread state sharing
//!
//! **Conclusion**: This thread-local pattern is the recommended approach given current
//! unilang API constraints. The dual state system (state.rs unused + `shared_state.rs`
//! active) represents well-managed architectural debt with clear migration path.

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
