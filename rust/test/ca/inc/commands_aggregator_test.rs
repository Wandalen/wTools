use super::*;

//

tests_impls!
{
  fn basic()
  {
    let ca = wca::commands_aggregator()
    .form();
    a_id!( ca.base_path, None );
    a_id!( ca.command_prefix, "".to_string() );
    a_id!( ca.delimeter, vec![ ".".to_string(), " ".to_string() ] );
    a_id!( ca.command_explicit_delimeter, ";".to_string() );
    a_id!( ca.command_implicit_delimeter, " ".to_string() );
    a_id!( ca.commands_explicit_delimiting, true );
    a_id!( ca.commands_implicit_delimiting, false );
    a_id!( ca.properties_map_parsing, false );
    a_id!( ca.several_values, true );
    a_id!( ca.with_help, true );
    a_id!( ca.changing_exit_code, true );
    a_id!( ca.commands, std::collections::HashMap::new() );
  }
}

//

tests_index!
{
  basic,
}

