#![ cfg( feature = "enabled" ) ]
//! Multiline terminal input with rich editing capabilities
//!
//! This crate provides a simple way to collect multiline text input from the terminal
//! with intuitive key bindings:
//! - **ENTER** to submit
//! - **CTRL+ENTER** or **SHIFT+ENTER** to insert newline
//! - **ESC** or **CTRL+C** to cancel
//!
//! # Quick Start
//!
//! ```no_run
//! use multiline_input::collect;
//!
//! match collect("Enter your message:") {
//!   Ok(Some(text)) => println!("You entered: {}", text),
//!   Ok(None) => println!("Cancelled"),
//!   Err(e) => eprintln!("Error: {}", e),
//! }
//! ```
//!
//! # Advanced Usage with Builder
//!
//! ```no_run
//! use multiline_input::Builder;
//!
//! let editor = Builder::new()
//!   .prompt("Enter commit message:")
//!   .min_length(10)
//!   .show_line_numbers(true)
//!   .show_status(true)
//!   .build();
//!
//! match editor.collect() {
//!   Ok(Some(msg)) => println!("Commit: {}", msg),
//!   Ok(None) => println!("Cancelled"),
//!   Err(e) => eprintln!("Error: {}", e),
//! }
//! ```
//!
//! # Key Bindings
//!
//! | Key | Action |
//! |-----|--------|
//! | ENTER | Submit input |
//! | CTRL+ENTER | Insert newline |
//! | SHIFT+ENTER | Insert newline |
//! | ESC | Cancel |
//! | CTRL+C | Cancel |
//! | Backspace | Delete before cursor |
//! | Delete | Delete at cursor |
//! | Arrow keys | Move cursor |
//! | Home/End | Move to line start/end |
//! | CTRL+Home/End | Move to text start/end |
//!
//! # Testing
//!
//! This crate uses trait-based dependency injection for testability. The `TerminalOps` trait
//! abstracts all terminal operations, allowing tests to use `MockTerminal` instead of real
//! terminal I/O.
//!
//! ## Test Quality Guarantees
//!
//! - **Non-Fragile**: All tests use `MockTerminal` with explicit configuration
//! - **Deterministic**: Pre-programmed key sequences ensure consistent results
//! - **No Environment Dependencies**: Tests never interact with actual terminal
//! - **Fast**: Integration tests run instantly (no waiting for user input)
//!
//! ## Test Organization
//!
//! Tests are organized by functional domain in the `tests/` directory:
//! - `buffer_operations_test.rs` - Text buffer operations
//! - `key_handling_test.rs` - Key event parsing and action mapping
//! - `integration_workflows_test.rs` - End-to-end workflows
//! - `error_paths_test.rs` - Error handling (NoTty, TerminalTooSmall)
//! - `validation_test.rs` - Input validation logic
//! - `builder_config_test.rs` - Builder pattern and configuration
//!
//! Total: **44 tests** with 100% pass rate
//!
//! ## Example Test with MockTerminal
//!
//! ```text
//! use multiline_input::Builder;
//! // Note: MockTerminal is in tests/common/mock_terminal.rs
//!
//! #[test]
//! fn test_basic_input() {
//!   let mut terminal = MockTerminal::new(true, (80, 24));
//!   terminal.push_key(key(KeyCode::Char('h'), KeyModifiers::NONE));
//!   terminal.push_key(key(KeyCode::Char('i'), KeyModifiers::NONE));
//!   terminal.push_key(key(KeyCode::Enter, KeyModifiers::NONE));
//!
//!   let editor = Builder::new()
//!     .prompt("Input:")
//!     .build_with(terminal);
//!
//!   let result = editor.collect();
//!   assert_eq!(result.unwrap(), Some("hi".to_string()));
//! }
//! ```
//!
//! See `tests/readme.md` for complete test documentation and examples.

// Re-export main types
pub use builder::Builder;
pub use editor::Editor;
pub use error::Error;

// Public API modules
pub mod builder;
pub mod editor;
pub mod error;

// Internal modules (public for testing, not part of stable API)
#[ doc( hidden ) ]
pub mod buffer;
#[ doc( hidden ) ]
pub mod keys;
#[ doc( hidden ) ]
pub mod render;
#[ doc( hidden ) ]
pub mod terminal;

/// Collect multiline input from terminal
///
/// Simple convenience function for basic usage. For advanced configuration,
/// use [`Builder`].
///
/// # Example
///
/// ```no_run
/// use multiline_input::collect;
///
/// match collect("Enter message:") {
///   Ok(Some(text)) => println!("Got: {}", text),
///   Ok(None) => println!("Cancelled"),
///   Err(e) => eprintln!("Error: {}", e),
/// }
/// ```
///
/// # Returns
///
/// - `Ok(Some(String))` - User submitted text
/// - `Ok(None)` - User cancelled (ESC or CTRL+C)
/// - `Err(Error)` - Terminal error or not running in TTY
pub fn collect( prompt: &str ) -> Result< Option< String >, Error >
{
  Builder::new()
    .prompt( prompt )
    .build()
    .collect()
}

/// Collect input with validation
///
/// # Example
///
/// ```no_run
/// use multiline_input::collect_validated;
///
/// let result = collect_validated("Enter message:", |text| {
///   if text.len() < 5 {
///     Err("Too short".to_string())
///   } else {
///     Ok(())
///   }
/// });
/// ```
pub fn collect_validated< F >(
  prompt: &str,
  validator: F,
) -> Result< Option< String >, Error >
where
  F: Fn( &str ) -> Result< (), String > + 'static,
{
  Builder::new()
    .prompt( prompt )
    .validator( validator )
    .build()
    .collect()
}
