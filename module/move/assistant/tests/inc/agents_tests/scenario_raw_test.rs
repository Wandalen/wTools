use super::*;

use the_module::agents::scenario_raw::ScenarioRaw;

use test_scenarios::gen_test_scenario_raw;

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

  let expected_scenario_raw = gen_test_scenario_raw();

  let scenario_raw = ScenarioRaw::read( scenario_text.as_bytes() );

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

  let scenario_raw = ScenarioRaw::read( scenario_text.as_bytes() );

  assert!( scenario_raw.is_err() );
}