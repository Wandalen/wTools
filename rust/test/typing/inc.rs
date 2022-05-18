
use super::TheModule;

#[ path = "./impls/implements_test.rs" ]
mod implements_test;
#[ path = "./impls/inspect_type_test.rs" ]
mod inspect_type_test;
// mod instance_of_test;

#[ path = "./impls/is_slice_test.rs" ]
mod is_slice_test;

//   mod implements_test
//   {
//     #[ allow( unused_imports ) ]
//     use wtools::typing as TheModule;
//     include!( "./typing/common/implements_test.rs" );
//   }
//   mod is_slice_test
//   {
//     #[ allow( unused_imports ) ]
//     use wtools::typing as TheModule;
//     include!( "./typing/common/is_slice_test.rs" );
//   }
//   mod inspect_type_test
//   {
//     #[ allow( unused_imports ) ]
//     use wtools::typing as TheModule;
//     include!( "./typing/common/inspect_type_test.rs" );
//   }
