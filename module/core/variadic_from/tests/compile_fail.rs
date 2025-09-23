// tests/compile_fail.rs

//! ## Test Matrix for Compile-Fail Tests
//!
//! This matrix outlines the test cases for `trybuild` to verify that the `VariadicFrom` macro correctly produces compile errors for invalid input.
//!
//! **Test Combinations: **
//!
//! | ID    | Struct Type | Field Count | Expected Error                               | Notes                                                              |
//! |-------|-------------|-------------|----------------------------------------------|--------------------------------------------------------------------|
//! | C5.1  | Named       | 0           | "VariadicFrom can only be derived for structs with 1, 2, or 3 fields." | Struct with no fields should fail.                                 |
//! | C5.2  | Named       | 4           | "VariadicFrom can only be derived for structs with 1, 2, or 3 fields." | Struct with more than 3 fields should fail.                        |
//! | C5.3  | N/A         | N/A         | "VariadicFrom can only be derived for structs with 1, 2, or 3 fields." | `from!` macro invoked with too many arguments (creates 4-field helper). |

#[ test ]
fn compile_fail() 
{
  let t = trybuild ::TestCases ::new();
  t.compile_fail("tests/compile_fail/*.rs");
}
