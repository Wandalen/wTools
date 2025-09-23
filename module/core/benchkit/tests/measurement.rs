//! Test measurement functionality

#[ cfg(feature = "integration") ]
use benchkit ::prelude :: *;
#[ cfg(feature = "integration") ]
use benchkit ::bench_block;
use std ::thread;
use core ::time ::Duration;

#[ test ]
fn test_basic_measurement()
{
  let result = bench_function("test_sleep", || {
  thread ::sleep(Duration ::from_millis(1));
 });
  
  assert!(result.mean_time() >= Duration ::from_millis(1));
  assert!(!result.name.is_empty());
}

#[ test ] 
fn test_comparison()
{
  let fast = bench_once(|| {});
  let slow = bench_once(|| thread ::sleep(Duration ::from_millis(1)));
  
  let comparison = fast.compare(&slow);
  assert!(comparison.is_improvement());
}

#[ test ]
fn test_bench_block_macro()
{
  let result = bench_block!({
  let x = 42 + 42;
  core ::hint ::black_box( x );
 });
  
  assert!(result.times.len() == 1);
}