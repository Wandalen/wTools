//! Passthrough behavior test for the `output_passthrough` feature.
//!
//! When compiled without the `string_split` feature (i.e., only with
//! `output_passthrough = ["enabled", "std"]`), `process_output` does not perform
//! line-based filtering — it returns content unchanged with zero lines reported as omitted.
//! This file exercises that code path (FT-41) which is structurally unreachable from the
//! standard `--all-features` test suite because `output = ["enabled", "std", "string_split"]`
//! always enables `string_split`, making the passthrough branch dead code under normal testing.

#![ cfg( all( feature = "output_passthrough", not( feature = "string_split" ) ) ) ]

use cli_fmt::output::*;

#[ test ]
fn feature_flag_line_filtering_passthrough()
{
  let input = "line1\nline2\nline3";
  let config = OutputConfig::default().with_head( 2 );
  let result = process_output( input, "", &config );
  assert_eq!
  (
    result.content,
    input,
    "FT-41: passthrough returns content unchanged when string_split absent"
  );
  assert_eq!
  (
    result.lines_omitted,
    0,
    "FT-41: passthrough reports zero lines omitted when string_split absent"
  );
}
