use super::*;

use the_module::agents::
{
  scenario_raw::ScenarioRaw,
  scenario_raw_processors::yaml_formatter,
};

#[ test ]
fn yaml_formatter_test()
{
  let expected_scenario_text = r#"
  nodes:
    - id: node_1
      type: agents::completion
      model: gpt-4o-mini
      next: node_2

    - id: node_2
      type: agents::classify
      model: gpt-4o
      next: ::scenario::termination
  "#;

  let scenario_raw = 
  ScenarioRaw::former()
  .nodes( vec!
  [
    NodeRaw::former()
    .id( "node_1".to_string() )
    .r#type( "agents::completion".to_string() )
    .model( "gpt-4o-mini".to_string() )
    .next( "node_2".to_string() )
    .form(),

    NodeRaw::former()
    .id( "node_2".to_string() )
    .r#type( "agents::classify".to_string() )
    .model( "gpt-4o".to_string() )
    .next( "::scenario::termination".to_string() )
    .form(),
  ] )
  .form();
  
  let mut buffer = String::new();

  let result = yaml_formatter( &scenario_raw, &mut buffer );

  assert!( result.is_ok() );
  assert_eq!( buffer, expected_scenario_text );
}
