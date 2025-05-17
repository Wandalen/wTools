// // File: module/core/former/tests/inc/former_enum_tests/subform_collection_test.rs
// //! Minimal test case demonstrating the compilation failure
// //! when using `#[subform_entry]` on a `Vec<Enum>`.
//
// use super::*;
// use former::Former;
// use std::vec::Vec;
//
// /// A simple enum deriving Former.
// #[ derive( Debug, PartialEq, Clone, Former ) ]
// pub enum SimpleEnum
// {
//   /// Unit variant.
//   Unit,
//   /// Tuple variant with a single value.
//   #[ scalar ] // Use scalar for direct constructor
//   Value( i32 ),
// }
//
// /// A struct containing a vector of the enum.
// #[ derive( Debug, PartialEq, Default, Former ) ]
// pub struct StructWithEnumVec
// {
//   /// Field attempting to use subform_entry on Vec<Enum>.
//   #[ subform_entry ]
//   items : Vec< SimpleEnum >,
// }
//
// /// Test attempting to use the subformer generated for `items`.
// /// This test FAIL TO COMPILE because `former` does not
// /// currently support generating the necessary subformer logic for enum entries
// /// within a collection via `#[subform_entry]`.
// #[ test ]
// fn attempt_subform_enum_vec()
// {
//   // This code block demonstrates the intended usage that fails.
//   /*
//   let _result = StructWithEnumVec::former()
//     // Trying to access the subformer for the Vec<SimpleEnum> field.
//     // The derive macro does not generate the `.items()` method correctly
//     // for Vec<Enum> with #[subform_entry]. It doesn't know how to
//     // return a former that can then construct *specific enum variants*.
//     .items()
//       // Attempting to call a variant constructor method (e.g., .value())
//       // on the hypothetical subformer returned by .items(). This method
//       // would not be generated.
//       .value( 10 )
//     // Ending the hypothetical subformer for the first enum entry.
//     .end()
//     // Attempting to start another entry.
//     .items()
//       // Attempting to call the unit variant constructor method.
//       .unit()
//     // Ending the hypothetical subformer for the second enum entry.
//     .end()
//   // Finalizing the parent struct.
//   .form();
//   */
//
//   // Assertion to make the test function valid, though it won't be reached
//   // if the compilation fails as expected.
//   assert!( true, "Test executed - compilation should have failed before this point." );
// }
// // qqq : xxx : make it working
