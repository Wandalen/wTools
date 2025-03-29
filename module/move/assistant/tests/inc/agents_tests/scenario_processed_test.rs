use super::*;

use the_module::agents::scenario_processed::ScenarioProcessed;

use test_scenarios::
{
  gen_test_scenario_raw,
  gen_test_scenario_raw_wrong,
};

#[ test ]
fn scenario_processed_right()
{
  let scenario_processed = ScenarioProcessed::try_from( gen_test_scenario_raw() );

  assert!( scenario_processed.is_ok() );
}

#[ test ]
fn scenario_processed_wrong()
{
  let scenario_processed = ScenarioProcessed::try_from( gen_test_scenario_raw_wrong() );

  assert!( scenario_processed.is_err() );
}