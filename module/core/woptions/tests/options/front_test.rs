
// use std::env;

// #[test]
// fn trybuild_tests()
// {
//   println!( "current_dir : {:?}", env::current_dir().unwrap() );
//   // let t = trybuild::TestCases::new();
//   // t.pass( "rust/test/former/test/basic_manual.rs" );
// }

// /* zzz : implement module::mod_at */
//
// mods_at!{ "./all"
// {
//   mod basic_manual;
//   mod basic;
//   mod without_perform;
// }}

use super::*;
use super::Former;

#[ path = "./all/basic_manual.rs" ]
mod basic_manual;
#[ path = "./all/basic.rs" ]
mod basic;
#[ path = "./all/without_perform.rs" ]
mod without_perform;

/* zzz : use macro mod_at */
// mod custom_getter_manual { include!( "./all/custom_getter_manual.rs" ); }
// mod custom_getter { include!( "./all/custom_getter.rs" ); }
