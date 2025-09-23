//! Test parser analysis functionality

#[ cfg(feature = "integration") ]
use benchkit ::prelude :: *;
use core ::time ::Duration;
use std ::collections ::HashMap;

fn create_test_result(time_ms: u64) -> BenchmarkResult
{
  let times = vec![Duration ::from_millis(time_ms); 5];
  BenchmarkResult ::new("test", times)
}

#[ test ]
fn test_parser_analyzer()
{
  let analyzer = ParserAnalyzer ::new("test_parser", 100, 5000);
  let result = create_test_result(100); // 100ms
  
  let metrics = analyzer.analyze(&result);
  
  assert_eq!(metrics.command_count, 100);
  assert_eq!(metrics.character_count, 5000);
  assert!(metrics.commands_per_second > 0.0);
  assert!(metrics.characters_per_second > 0.0);
}

#[ test ]
fn test_parser_comparison()
{
  let analyzer = ParserAnalyzer ::new("comparison_test", 50, 2500);
  
  let mut results = HashMap ::new();
  results.insert("fast_parser".to_string(), create_test_result(50));
  results.insert("slow_parser".to_string(), create_test_result(200));
  
  let comparison = analyzer.compare_parsers(&results);
  
  assert_eq!(comparison.metrics.len(), 2);
  
  let (fastest_name, _) = comparison.fastest_parser().unwrap();
  assert_eq!(fastest_name, "fast_parser");
}

#[ test ]
fn test_pipeline_analyzer()
{
  let mut analyzer = ParserPipelineAnalyzer ::new();
  
  analyzer
  .add_stage("tokenization", create_test_result(50))
  .add_stage("parsing", create_test_result(100))
  .add_stage("ast_build", create_test_result(25));
  
  let analysis = analyzer.analyze_bottlenecks();
  
  assert_eq!(analysis.stage_count, 3);
  assert!(analysis.bottleneck.is_some());
  
  let (bottleneck_name, _) = analysis.bottleneck.unwrap();
  assert_eq!(bottleneck_name, "parsing");
}