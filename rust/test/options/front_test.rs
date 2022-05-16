
// use std::env;

// #[test]
// fn trybuild_tests()
// {
//   println!( "current_dir : {:?}", env::current_dir().unwrap() );
//   // let t = trybuild::TestCases::new();
//   // t.pass( "rust/test/former/test/basic_manual.rs" );
// }

// /* xxx : implement module::mod_at */
//
// mods_at!{ "./all"
// {
//   mod basic_manual;
//   mod basic;
//   mod without_perform;
// }}

mod basic_manual { include!( "./all/basic_manual.rs" ); }
mod basic { include!( "./all/basic.rs" ); }
mod without_perform { include!( "./all/without_perform.rs" ); }
/* zzz : use macro mod_at */
// mod custom_getter_manual { include!( "./all/custom_getter_manual.rs" ); }
// mod custom_getter { include!( "./all/custom_getter.rs" ); }
