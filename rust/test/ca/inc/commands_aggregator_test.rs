use super::*;

//

tests_impls!
{
  fn basic()
  {
    let ca = wca::commands_aggregator()
    .form();
    a_id!( ca.command_prefix, "".to_string() );
  }
}

//

tests_index!
{
  basic,
}

