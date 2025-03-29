// use super::*;
//
// //
//
// tests_impls!
// {
//
//   fn hash_map()
//   {
//
//     // test.case( "empty" );
//     let got : std::collections::HashMap< i32, i32 > = the_module::hmap!{};
//     let exp = std::collections::HashMap::new();
//     a_id!( got, exp );
//
//     // test.case( "single entry" );
//     let got = the_module::hmap!{ 3 => 13 };
//     let mut exp = std::collections::HashMap::new();
//     exp.insert( 3, 13 );
//     a_id!( got, exp );
//
//   }
//
//   //
//
//
//   fn hash_set()
//   {
//
//     // test.case( "empty" );
//     let got : std::collections::HashSet< i32 > = the_module::hset!{};
//     let exp = std::collections::HashSet::new();
//     a_id!( got, exp );
//
//     // test.case( "single entry" );
//     let got = the_module::hset!{ 13 };
//     let mut exp = std::collections::HashSet::new();
//     exp.insert( 13 );
//     a_id!( got, exp );
//
//   }
// }
//
// //
//
// tests_index!
// {
//   hash_map,
//   hash_set,
// }
