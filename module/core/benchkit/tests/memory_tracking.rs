//! Test memory tracking functionality

#[ cfg(feature = "integration") ]
use benchkit ::prelude :: *;
#[ allow(unused_imports) ]
use benchkit ::memory_tracking;

#[ test ]
fn test_allocation_tracker()
{
  let tracker = AllocationTracker ::new();
  
  // Record some allocations
  tracker.record_allocation(1024);
  tracker.record_allocation(512);
  tracker.record_deallocation(512);
  
  let stats = tracker.get_stats();
  
  assert_eq!(stats.allocation_count, 2);
  assert_eq!(stats.total_allocated, 1536);
  assert_eq!(stats.current_usage, 1024);
  assert_eq!(stats.peak_usage, 1536);
}

#[ test ]
fn test_memory_benchmark()
{
  let benchmark = MemoryBenchmark ::new("test");
  
  let (result, stats) = benchmark.run_with_tracking(5, || 
  {
  // Simulate some work
  let _vec = vec![0u8; 1024];
  benchmark.tracker.record_allocation(1024);
 });
  
  assert_eq!(result.times.len(), 5);
  assert!(stats.total_allocated > 0);
}

#[ test ]
fn test_memory_comparison()
{
  let benchmark = MemoryBenchmark ::new("comparison_test");
  
  let comparison = benchmark.compare_memory_usage(
  "allocating",
  || 
  {
   let _vec = vec![0u8; 1024];
   benchmark.tracker.record_allocation(1024);
 },
  "non_allocating",
  || 
  {
   // No allocations
   core ::hint ::black_box(42);
 },
  3,
 );
  
  assert_eq!(comparison.impl1_name, "allocating");
  assert_eq!(comparison.impl2_name, "non_allocating");
  
  let (efficient, _) = comparison.more_memory_efficient();
  assert_eq!(efficient, "non_allocating");
}

// Note: format_bytes is a private function in memory_tracking module
// so we can't test it directly. We'll test the public API instead.
#[ test ]
fn test_allocation_stats_display()
{
  let stats = AllocationStats
  {
  allocation_count: 10,
  total_allocated: 1024,
  peak_usage: 512,
  current_usage: 256,
 };
  
  // Test that description method works (which internally uses format_bytes)
  let desc = stats.description();
  assert!(desc.contains("Allocs: 10"));
  assert!(desc.contains("Total: "));
  assert!(desc.contains("Peak: "));
}

#[ test ]
fn test_allocation_stats()
{
  let stats = AllocationStats
  {
  allocation_count: 10,
  total_allocated: 1024,
  peak_usage: 512,
  current_usage: 256,
 };
  
  #[ allow(clippy ::float_cmp) ]
  {
  assert_eq!(stats.average_allocation_size(), 102.4);
  assert_eq!(stats.memory_efficiency(), 0.5);
 }
}