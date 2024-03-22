use super::*;

//

fn vec_as_ref< T >( src : &Vec< T > ) -> Vec< &str >
where
  T : AsRef< str >,
{
  src.iter().map( | e | e.as_ref() ).collect::< Vec< &str > >()
}

tests_impls!
{
  #[ test ]
  fn instruction_parse_from_splits_basic()
  {
    // test.case( "command and several subjects" );
    let args = vec![ ".struct1", "subject1", "subject2" ];
    let instruction = the_module::instruction::parse_from_splits( args.iter() );
    a_id!( instruction.command_name.as_ref(), ".struct1" );
    a_id!( vec_as_ref( &instruction.subject ), vec![ "subject1", "subject2" ] );
    a_id!( instruction.properties_map, std::collections::HashMap::new() );

    // // test.case( "basic comand, subject map" );
    // let args = vec![ ".struct1", "subject1", "k1:v1" ];
    // let instruction = the_module::instruction::parse_from_splits( args.iter() );
    // a_id!( instruction.command_name.as_ref(), ".struct1" );
    // a_id!( vec_as_ref( &instruction.subject ), vec![ "subject1" ] );
    // a_id!( instruction.properties_map, std::collections::HashMap::new() );
  }

  //

  // fn _string_split()
  // {
  //
  //   // test.case( "basic" );
  //   // let src = "ab ef";
  //   // let iter = the_module::string::split_default( src );
  //   // a_id!( iter.map( | e | String::from( e ) ).collect::< Vec< _ > >(), vec![ "ab", " ", "ef" ] );
  //
  //   // test.case( "delimeter : "x" );
  //   let src = "ab ef";
  //   // let iter = the_module::string::split().delimeter( "b" ).src( src ).form();
  //   let iter = the_module::string::split().delimeter( "b" ).src( src ).form();
  //   a_id!( iter.map( | e | String::from( e ) ).collect::< Vec< _ > >(), vec![ "a", "b", " ef" ] );
  //
  // }
}

//

tests_index!
{
  instruction_parse_from_splits_basic,
  // string_split,
}
