
// use std::env;

// #[test]
// fn trybuild_tests()
// {
//   println!( "current_dir : {:?}", env::current_dir().unwrap() );
//   // let t = trybuild::TestCases::new();
//   // t.pass( "rust/test/former/test/basic_manual.rs" );
// }

mod basic_manual { include!( "./all/basic_manual.rs" ); }
// mod custom_getter_manual { include!( "./all/custom_getter_manual.rs" ); }

/* zzz : use macro mod_at */
