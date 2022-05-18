
// include!( "./all/for_each_test.rs" );

// mod for_each_test
// {
//   use for_each as TheModule;
//   include!( "./all/for_each_test.rs" );
// }

use for_each as TheModule;
#[ path = "./meta/for_each_test.rs" ]
pub mod for_each_test;
