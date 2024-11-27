use super::*;

use the_module::agents::scenario_raw::ScenarioRaw;

pub fn gen_test_scenario_raw() -> ScenarioRaw
{
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
  .form()
}