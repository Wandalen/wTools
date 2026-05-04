//! Terminal abstraction tests
//!
//! ## Domain
//!
//! Tests for terminal abstraction layer:
//! - Terminal creation
//! - Size querying
//!
//! ## Organization
//!
//! Tests migrated from `src/terminal.rs` module tests.

use multiline_input::terminal::{ RealTerminal, size };

#[ test ]
fn test_terminal_creation()
{
  let _terminal = RealTerminal::new();
  // Just check that it doesnt panic
}

#[ test ]
fn test_size_query()
{
  // Should not panic even if no TTY
  let _ = size();
}
