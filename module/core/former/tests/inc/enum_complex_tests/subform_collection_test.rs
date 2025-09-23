#![allow(clippy::used_underscore_binding, clippy::all, warnings, missing_docs)]
//! Purpose: This file is a test case demonstrating the current limitation and compilation failure
//! when attempting to use the `#[ subform_entry ]` attribute on a field that is a collection of enums
//! (specifically, `Vec<SimpleEnum>`). It highlights a scenario that is not currently supported by
//! the `Former` macro.
//!
//! Coverage:
//! - This file primarily demonstrates a scenario *not* covered by the defined "Expected Enum Former Behavior Rules"
//!   because the interaction of `#[ subform_entry ]` with collections of enums is not a supported feature.
//!   It implicitly relates to the concept of subform collection handling but serves as a test for an unsupported case.
//!
//! Test Relevance/Acceptance Criteria:
//! - Defines a simple enum `SimpleEnum` deriving `Former`.
//! - Defines a struct `StructWithEnumVec` containing a `Vec<SimpleEnum>` field.
//! - Applies `#[ subform_entry ]` to the `Vec<SimpleEnum>` field.
//! - The entire file content is commented out, including a test function (`attempt_subform_enum_vec`) that demonstrates the intended (but unsupported) usage of a hypothetical subformer for the enum collection.
//! - This file is intended to be a compile-fail test or a placeholder for a future supported feature. The test is accepted if attempting to compile code that uses `#[ subform_entry ]` on a collection of enums results in a compilation error (as indicated by the comments).

// // File: module/core/former/tests/inc/former_enum_tests/subform_collection_test.rs
// //! Minimal test case demonstrating the compilation failure
// //! when using `#[ subform_entry ]` on a `Vec<Enum>`.
// //
// // use super::*;
// // use former::Former;
// // use std::vec::Vec;
// //
// // /// A simple enum deriving Former.
// // #[ derive( Debug, PartialEq, Clone, Former ) ]
// // pub enum SimpleEnum
// // {
// //   /// Unit variant.
// //   Unit,
// //   /// Tuple variant with a single value.
// //   #[ scalar ] // Use scalar for direct constructor
// //   Value( i32 ),
// // }
// //
// // /// A struct containing a vector of the enum.
// // #[ derive( Debug, PartialEq, Default, Former ) ]
// // pub struct StructWithEnumVec
// // {
// //   /// Field attempting to use subform_entry on Vec<Enum>.
// //   #[ subform_entry ]
// //   items : Vec< SimpleEnum >,
// // }
// //
// // /// Test attempting to use the subformer generated for `items`.
// // /// This test FAIL TO COMPILE because `former` does not
// // /// currently support generating the necessary subformer logic for enum entries
// // /// within a collection via `#[ subform_entry ]`.
// // #[ test ]
// // fn attempt_subform_enum_vec()
// // {
// //   // This code block demonstrates the intended usage that fails.
// //   /*
// //   let _result = StructWithEnumVec::former()
// //     // Trying to access the subformer for the Vec<SimpleEnum> field.
// //     // The derive macro does not generate the `.items()` method correctly
// //     // for Vec<Enum> with #[ subform_entry ]. It doesn't know how to
// //     // return a former that can then construct *specific enum variants*.
// //     .items()
// //       // Attempting to call a variant constructor method (e.g., .value())
// //       // on the hypothetical subformer returned by .items(). This method
// //       // would not be generated.
// //       .value( 10 )
// //     // Ending the hypothetical subformer for the first enum entry.
// //     .end()
// //     // Attempting to start another entry.
// //     .items()
// //       // Attempting to call the unit variant constructor method.
// //       .unit()
// //     // Ending the hypothetical subformer for the second enum entry.
// //     .end()
// //   // Finalizing the parent struct.
// //   .form();
// //   */
// //
// //   // Assertion to make the test function valid, though it won't be reached
// //   // if the compilation fails as expected.
// //   assert!( true, "Test executed - compilation should have failed before this point." );
// // }
// // // qqq : xxx : make it working
