use super::*;

use the_module::agents::scenario_raw::
{
  ScenarioRaw,
  NodeRaw,
};

/// Generates an example `ScenarioRaw`.
pub fn gen_test_scenario_raw() -> ScenarioRaw
{
  ScenarioRaw::former()
  .nodes( vec!
  [
    NodeRaw::former()
    .id( "node_1".to_string() )
    .r#type( "agents::completion".to_string() )
    .params(
      {
        let mut map : HashMap< String, String > = HashMap::new();
        map.insert( "model".into(), "gpt-4o-mini".into() );
        map
      }
    )
    .next( "node_2".to_string() )
    .form(),

    NodeRaw::former()
    .id( "node_2".to_string() )
    .r#type( "agents::classify".to_string() )
    .params(
      {
        let mut map : HashMap< String, String > = HashMap::new();
        map.insert( "model".into(), "gpt-4o".into() );
        map
      }
    )
    .next( "::scenario::termination".to_string() )
    .form(),
  ] )
  .form()
}

/// Generates a `ScenarioRaw` with wrong syntax for `Path`.
pub fn gen_test_scenario_raw_wrong() -> ScenarioRaw
{
  ScenarioRaw::former()
  .nodes( vec!
  [
    NodeRaw::former()
    .id( "node_1".to_string() )
    .r#type( ":agents:".to_string() ) // This part is incorrect. Path written in wrong syntax.
    .params(
      {
        let mut map : HashMap< String, String > = HashMap::new();
        map.insert( "model".into(), "gpt-4o-mini".into() );
        map
      }
    )
    .next( "node_2".to_string() )
    .form(),
  ] )
  .form()
}