use super::*;

use the_module::agents::scenario_raw_processors::yaml_formatter::yaml_formatter;

use test_scenarios::gen_test_scenario_raw;

#[ test ]
fn yaml_formatter_test()
{
  let expected_yaml = r#"nodes:
- id: node_1
  type: agents::completion
  model: gpt-4o-mini
  next: node_2
- id: node_2
  type: agents::classify
  model: gpt-4o
  next: ::scenario::termination"#;

  let scenario_raw = gen_test_scenario_raw();
  
  let mut buffer = Vec::new();
  let result = yaml_formatter( &scenario_raw, &mut buffer );
  assert!( result.is_ok() );

  let result = String::from_utf8( buffer );
  assert!( result.is_ok() );

  let result = result.unwrap();
  println!( "{}", result );

  assert_eq!( result.trim(), expected_yaml.trim() );
}
