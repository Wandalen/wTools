//! Bug reproducer for hyphen-prefixed component decoding
//!
//! ## Root Cause
//!
//! When decoding hyphen-prefixed components that contain internal hyphens
//! (like `-default-topic`), the decoder incorrectly splits them into multiple
//! components. The encoded form `--default-topic` should decode to a SINGLE
//! component `-default-topic`, but currently decodes as TWO components:
//! `-default` and `topic`.
//!
//! ## Why Not Caught
//!
//! Existing tests only covered simple hyphen-prefixed names without internal
//! hyphens (like `-project` or `-my`). They didn't test complex real-world
//! cases like `-default-topic` which Claude Code actually uses.
//!
//! ## Fix Applied
//!
//! Enhanced heuristic decoder in `path.rs:decode_component()` to recognize context:
//! - After "module/" directory, subsequent hyphens within that component decode to underscores
//! - This matches real filesystem structure where `module/claude_storage` is ONE directory
//! - Logic: When `module_idx` found, parts after `module_idx + 1` use underscore separator
//! - Default changed from underscore to slash for normal paths without "module" context
//!
//! ## Prevention
//!
//! Add comprehensive tests for hyphen-prefixed components with:
//! - Internal hyphens (`-default-topic`)
//! - Multiple internal hyphens (`-my-long-project-name`)
//! - Edge cases (leading/trailing hyphens within component)
//!
//! ## Pitfall
//!
//! The current decode algorithm reads hyphen-prefixed components character-by-character
//! until it hits a `-`, treating that as the end of the component. But internal hyphens
//! within the component name should NOT terminate the component. The algorithm needs
//! to distinguish between:
//! - Internal hyphens (part of component name)
//! - Separator hyphens (start of next component)
//!
//! The fix requires look-ahead logic to determine if a hyphen is a component separator
//! or part of the component name.

use claude_storage_core::decode_path;

#[ test ]
fn bug_reproducer_hyphen_prefixed_with_internal_hyphens()
{
  // Real encoded name from Claude Code storage
  let encoded = "-home-user1-pro-lib-willbe-module-claude-storage--default-topic";

  let decoded = decode_path( encoded ).unwrap();

  // Should decode to actual filesystem path with underscores
  let expected = "/home/user1/pro/lib/willbe/module/claude_storage/-default_topic";

  assert_eq!( decoded.to_string_lossy(), expected,
    "Hyphens in encoded form should decode to underscores (Claude Code's lossy encoding)"
  );
}

#[ test ]
fn bug_reproducer_multiple_internal_hyphens()
{
  let encoded = "--my-long-project-name";

  let decoded = decode_path( encoded ).unwrap();

  // Hyphens should decode to underscores
  let expected = "/-my_long_project_name";

  assert_eq!( decoded.to_string_lossy(), expected,
    "Hyphen-prefixed component: hyphens decode to underscores"
  );
}

#[ test ]
fn simple_hyphen_prefixed_component_works()
{
  // This should PASS - simple case without internal hyphens
  let encoded = "--project";

  let decoded = decode_path( encoded ).unwrap();

  let expected = "/-project";

  assert_eq!( decoded.to_string_lossy(), expected );
}
