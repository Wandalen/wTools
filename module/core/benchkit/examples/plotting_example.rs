//! Example demonstrating benchkit's visualization capabilities
//! 
//! Run with: `cargo run --example plotting_example --features visualization`

#[ cfg(feature = "visualization") ]
use benchkit ::prelude :: *;

#[ cfg(feature = "visualization") ]
type Result< T > = core ::result ::Result< T, Box<dyn core ::error ::Error >>;

#[ cfg(feature = "visualization") ]
fn main() -> Result< () >
{
  use std ::path ::Path;
  
  println!("📊 Benchkit Visualization Example");
  println!("================================");
  
  // Create sample benchmark data
  let scaling_results = vec![
  (10, create_test_result("test_10", 1000.0)),
  (100, create_test_result("test_100", 800.0)),
  (1000, create_test_result("test_1000", 600.0)),
  (10000, create_test_result("test_10000", 400.0)),
 ];
  
  let framework_results = vec![
  ("Fast Framework".to_string(), create_test_result("fast", 1000.0)),
  ("Medium Framework".to_string(), create_test_result("medium", 600.0)),
  ("Slow Framework".to_string(), create_test_result("slow", 300.0)),
 ];
  
  // Generate scaling chart
  let scaling_path = Path ::new("target/scaling_chart.svg");
  plots ::scaling_analysis_chart(
  &scaling_results,
  "Performance Scaling Analysis",
  scaling_path
 )?;
  println!("✅ Scaling chart generated: {}", scaling_path.display());
  
  // Generate comparison chart
  let comparison_path = Path ::new("target/framework_comparison.svg");
  plots ::framework_comparison_chart(
  &framework_results,
  "Framework Performance Comparison", 
  comparison_path
 )?;
  println!("✅ Comparison chart generated: {}", comparison_path.display());
  
  // Generate trend chart
  let historical_data = vec![
  ("2024-01-01".to_string(), 500.0),
  ("2024-02-01".to_string(), 600.0),
  ("2024-03-01".to_string(), 750.0),
  ("2024-04-01".to_string(), 800.0),
  ("2024-05-01".to_string(), 900.0),
 ];
  
  let trend_path = Path ::new("target/performance_trend.svg");
  plots ::performance_trend_chart(
  &historical_data,
  "Performance Trend Over Time",
  trend_path
 )?;
  println!("✅ Trend chart generated: {}", trend_path.display());
  
  println!("\n🎉 All charts generated successfully!");
  println!("   View the SVG files in your browser or image viewer");
  
  Ok(())
}

#[ cfg(feature = "visualization") ]
fn create_test_result(name: &str, ops_per_sec: f64) -> BenchmarkResult
{
  use core ::time ::Duration;
  let duration = Duration ::from_secs_f64(1.0 / ops_per_sec);
  BenchmarkResult ::new(name, vec![duration; 5])
}

#[ cfg(not(feature = "visualization")) ]
fn main() 
{
  println!("⚠️  Visualization disabled - enable 'visualization' feature for charts");
}