use super::*;

use the_module::agents::
{
  scenario_raw::ScenarioRaw,
  scenario_raw_processors::plantuml_formatter::plantuml_formatter,
};

use test_scenarios::gen_test_scenario_raw;


#[ test ]
fn plantuml_formatter()
{
  let expected_plantuml = r#"
  @startuml
  json node_1 {
    "type": "agents::completion",
    "model": "gpt-4o-mini"
  }
  json node_2 {
    "type": "agents::classify",
    "model": "gpt-4o"
  }
  node_1 --> node_2 : next
  @enduml"#;

  let scenario_raw = gen_test_scenario_raw();

  let mut buffer = String::new();
  let result = plantuml_formatter( &scenario_raw, &mut buffer );

  assert!( result.is_ok() );
  assert_eq!( buffer, expected_plantuml );
}
