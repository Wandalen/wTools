
// use std::env;

// #[test]
// fn trybuild_tests()
// {
//   println!( "current_dir : {:?}", env::current_dir().unwrap() );
//   // let t = trybuild::TestCases::new();
//   // t.pass( "rust/test/former/test/basic_runtime.rs" );
// }

// /* xxx : implement module::mod_at */
//
// mods_at!{ "./all"
// {
//   mod basic_runtime;
//   mod basic;
//   mod without_perform;
// }}

mod basic_runtime { include!( "./all/basic_runtime.rs" ); }
mod basic { include!( "./all/basic.rs" ); }
mod without_perform { include!( "./all/without_perform.rs" ); }
/* xxx */
// mod custom_getter { include!( "./all/custom_getter.rs" ); }
