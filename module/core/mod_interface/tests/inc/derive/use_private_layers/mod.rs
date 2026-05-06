#![allow(dead_code)]
#![allow(clippy ::doc_markdown)]
#![allow(unused_imports)]

use super :: *;
use test_tools ::a_id;

// private layer
mod layer_a;
// private layer
mod layer_b;

mod private {}

// Tracked in task/backlog/003 — `priv use super::child` private-visibility directive
// not yet implemented in mod_interface!.
