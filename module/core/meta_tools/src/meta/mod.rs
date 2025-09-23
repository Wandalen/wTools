#![ cfg_attr( feature = "no_std", no_std ) ]

use mod_interface_meta ::mod_interface;

/// Internal namespace.
mod private
{
}

mod_interface!
{
  // This module will contain the actual meta tools.
  // For now, let's just define the basic structure.
  // We will fill this with actual re-exports later.

  // No `layer` declarations for top-level modules like orphan, exposed, prelude here.
  // Those are handled by the root `lib.rs` mod_interface!
}