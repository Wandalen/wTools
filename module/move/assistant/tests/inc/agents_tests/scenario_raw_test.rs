use super::*;

use the_module::agents::scenario_raw::ScenarioRaw;

#[ test ]
fn scenario_read()
{
  let scenario_text = r#"
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

  let expected_scenario_raw = 
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
  
  let scenario_raw = ScenarioRaw::read( scenario_text );

  assert!( scenario_raw.is_ok() );

  let scenario_raw = scenario_raw.unwrap();
  assert_eq!( scenario_raw, expected_scenario_raw );
}

#[ test ]
fn scenario_wrong()
{
  let scenario_text = r#"
  nodes:
    - completion:
      model:
        company: openai
        name: gpt-4o
      depends_on:
        node_2
  "#;

  let scenario_raw = ScenarioRaw::read( scenario_text );

  assert!( scenario_raw.is_err() );
}