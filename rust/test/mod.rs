
#[ allow( unused_imports ) ]
use super::TheModule as TheModule;

include!( "./_conditional/wtools.rs" );

mod dt;
mod error;

// zzz : remove include
// mod former
// {
//   include!( "./former/wtools_front_test.rs" );
// }

mod former;

mod derive;
mod meta;
mod options;

mod iter;

// mod iter
// {
//   use wtools::iter as TheModule;
//   include!( "./iter/basic_test.rs" );
// }
mod string;
mod time;
mod typing;

// mod typing
// {
//   // #![ cfg_attr( feature = "nightly", feature( type_name_of_val ) ) ]
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
// }

// #[ cfg( feature = "proc_macro" ) ]
// mod proc_macro;
// mod vector;
