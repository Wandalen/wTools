// #![ feature( pattern ) ]

use wtest_basic::*;
use wtools::*;
use wcensor as TheModule;

// use wtools::former::Former;

//

fn vec_as_ref< T >( src : &Vec< T > ) -> Vec< &str >
where
  T : AsRef< str >,
{
  src.iter().map( | e | e.as_ref() ).collect::< Vec< &str > >()
}

//

fn instruction_parse_from_splits_basic_test()
{

  // test.case( "command and several subjects" );
  let args = vec![ ".struct1", "subject1", "subject2" ];
  let instruction = TheModule::instruction::parse_from_splits( args.iter() );
  assert_eq!( instruction.command_name.as_ref(), ".struct1" );
  assert_eq!( vec_as_ref( &instruction.subject ), vec![ "subject1", "subject2" ] );
  assert_eq!( instruction.properties_map, hmap!{} );

  // // test.case( "basic comand, subject map" );
  // let args = vec![ ".struct1", "subject1", "k1:v1" ];
  // let instruction = TheModule::instruction::parse_from_splits( args.iter() );
  // assert_eq!( instruction.command_name.as_ref(), ".struct1" );
  // assert_eq!( vec_as_ref( &instruction.subject ), vec![ "subject1" ] );
  // assert_eq!( instruction.properties_map, hmap!{} );

}

//

// fn _string_split()
// {
//
//   // test.case( "basic" );
//   // let src = "ab ef";
//   // let iter = TheModule::string::split_default( src );
//   // assert_eq!( iter.map( | e | String::from( e ) ).collect::< Vec< _ > >(), vec![ "ab", " ", "ef" ] );
//
//   // test.case( "delimeter : "x" );
//   let src = "ab ef";
//   // let iter = TheModule::string::split().delimeter( "b" ).src( src ).form();
//   let iter = TheModule::string::split().delimeter( "b" ).src( src ).form();
//   assert_eq!( iter.map( | e | String::from( e ) ).collect::< Vec< _ > >(), vec![ "a", "b", " ef" ] );
//
// }

//

test_suite!
{
  instruction_parse_from_splits_basic,
  // string_split,
}
