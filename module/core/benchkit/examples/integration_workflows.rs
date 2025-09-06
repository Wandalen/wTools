//! Complete Integration Workflow Examples
//!
//! This example demonstrates EVERY integration pattern combining all enhanced features:
//! - End-to-end benchmark → validation → template → documentation workflows
//! - CI/CD pipeline integration patterns
//! - Multi-project benchmarking coordination
//! - Performance monitoring and alerting scenarios
//! - Development workflow automation
//! - Production deployment validation

#![ cfg( feature = "enabled" ) ]
#![ cfg( feature = "markdown_reports" ) ]
#![ allow( clippy::uninlined_format_args ) ]
#![ allow( clippy::if_not_else ) ]
#![ allow( clippy::useless_vec ) ]
#![ allow( clippy::needless_borrows_for_generic_args ) ]
#![ allow( clippy::too_many_lines ) ]
#![ allow( clippy::needless_raw_string_hashes ) ]
#![ allow( clippy::std_instead_of_core ) ]

use benchkit::prelude::*;
use std::collections::HashMap;
use std::time::Duration;

/// Simulate running actual benchmarks for different algorithms
fn run_algorithm_benchmarks() -> HashMap< String, BenchmarkResult >
{
  let mut results = HashMap::new();
  
  // Simulate various algorithms with realistic performance characteristics
  let algorithms = vec![
    ( "quicksort", vec![ 95, 100, 92, 98, 103, 96, 101, 94, 99, 97, 102, 93, 100, 95, 98 ] ),
    ( "mergesort", vec![ 110, 115, 108, 112, 117, 111, 114, 107, 113, 109, 116, 106, 115, 110, 112 ] ),
    ( "heapsort", vec![ 130, 135, 128, 132, 137, 131, 134, 127, 133, 129, 136, 126, 135, 130, 132 ] ),
    ( "bubblesort", vec![ 2500, 2600, 2400, 2550, 2650, 2450, 2580, 2420, 2570, 2480, 2620, 2380, 2590, 2520, 2560 ] ),
  ];
  
  for ( name, timings_micros ) in algorithms
  {
    let times : Vec< Duration > = timings_micros.iter()
      .map( | &t | Duration::from_micros( t ) )
      .collect();
    results.insert( name.to_string(), BenchmarkResult::new( name, times ) );
  }
  
  results
}

/// Simulate memory-intensive algorithms
fn run_memory_benchmarks() -> HashMap< String, BenchmarkResult >
{
  let mut results = HashMap::new();
  
  let memory_algorithms = vec![
    ( "in_place_sort", vec![ 80, 85, 78, 82, 87, 81, 84, 77, 83, 79, 86, 76, 85, 80, 82 ] ),
    ( "copy_sort", vec![ 150, 160, 145, 155, 165, 152, 158, 148, 157, 151, 162, 143, 159, 154, 156 ] ),
    ( "stream_sort", vec![ 200, 220, 190, 210, 230, 205, 215, 185, 212, 198, 225, 180, 218, 202, 208 ] ),
  ];
  
  for ( name, timings_micros ) in memory_algorithms
  {
    let times : Vec< Duration > = timings_micros.iter()
      .map( | &t | Duration::from_micros( t ) )
      .collect();
    results.insert( name.to_string(), BenchmarkResult::new( name, times ) );
  }
  
  results
}

/// Workflow 1: Development Cycle Integration
fn workflow_development_cycle()
{
  println!( "=== Workflow 1: Development Cycle Integration ===" );
  println!( "Simulating: Developer runs benchmarks → Validates quality → Updates docs → Commits" );
  
  // Step 1: Run benchmarks (simulated)
  println!( "\n📊 Step 1: Running benchmark suite..." );
  let algorithm_results = run_algorithm_benchmarks();
  let memory_results = run_memory_benchmarks();
  
  println!( "   Completed {} algorithm benchmarks", algorithm_results.len() );
  println!( "   Completed {} memory benchmarks", memory_results.len() );
  
  // Step 2: Validate results quality
  println!( "\n🔍 Step 2: Validating benchmark quality..." );
  let validator = BenchmarkValidator::new()
    .min_samples( 10 )
    .max_coefficient_variation( 0.15 )
    .require_warmup( false );  // Disabled for simulated data
  
  let validated_algorithms = ValidatedResults::new( algorithm_results.clone(), validator.clone() );
  let validated_memory = ValidatedResults::new( memory_results.clone(), validator );
  
  println!( "   Algorithm benchmarks: {:.1}% reliable", validated_algorithms.reliability_rate() );
  println!( "   Memory benchmarks: {:.1}% reliable", validated_memory.reliability_rate() );
  
  // Step 3: Generate comprehensive reports
  println!( "\n📄 Step 3: Generating documentation..." );
  
  let algorithm_template = PerformanceReport::new()
    .title( "Algorithm Performance Analysis" )
    .add_context( "Comparative analysis of sorting algorithms for production use" )
    .include_statistical_analysis( true )
    .add_custom_section( CustomSection::new(
      "Development Notes",
      "- All algorithms tested on same dataset size (1000 elements)\n- Results validated for statistical reliability\n- Recommendations based on both performance and code maintainability"
    ));
  
  let memory_template = PerformanceReport::new()
    .title( "Memory Usage Analysis" )
    .add_context( "Memory allocation patterns and their performance impact" )
    .include_statistical_analysis( true );
  
  let algorithm_report = algorithm_template.generate( &algorithm_results ).unwrap();
  let memory_report = memory_template.generate( &memory_results ).unwrap();
  
  // Generate comparison report for best vs worst algorithm
  let comparison_template = ComparisonReport::new()
    .title( "Best vs Worst Algorithm Comparison" )
    .baseline( "bubblesort" )
    .candidate( "quicksort" )
    .practical_significance_threshold( 0.05 );
  
  let comparison_report = comparison_template.generate( &algorithm_results ).unwrap();
  
  // Step 4: Update documentation atomically
  println!( "\n📝 Step 4: Updating project documentation..." );
  
  let project_readme = std::env::temp_dir().join( "PROJECT_README.md" );
  let readme_content = r#"# Sorting Algorithm Library

## Overview

High-performance sorting algorithms for production use.

## Algorithm Performance

*Performance analysis will be automatically updated here.*

## Memory Analysis

*Memory usage analysis will be automatically updated here.*

## Algorithm Comparison

*Detailed comparison will be automatically updated here.*

## Usage Examples

See examples directory for usage patterns.
"#;
  
  std::fs::write( &project_readme, readme_content ).unwrap();
  
  let update_chain = MarkdownUpdateChain::new( &project_readme ).unwrap()
    .add_section( "Algorithm Performance", &algorithm_report )
    .add_section( "Memory Analysis", &memory_report )
    .add_section( "Algorithm Comparison", &comparison_report );
  
  match update_chain.execute()
  {
    Ok( () ) =>
    {
      println!( "   ✅ Project documentation updated successfully" );
      let final_size = std::fs::metadata( &project_readme ).unwrap().len();
      println!( "   Final README size: {} bytes", final_size );
      
      // Simulate git commit
      println!( "\n💾 Step 5: Committing changes..." );
      println!( "   git add README.md" );
      println!( "   git commit -m 'docs: Update performance analysis'" );
      println!( "   ✅ Changes committed to version control" );
    },
    Err( e ) => println!( "   ❌ Documentation update failed: {}", e ),
  }
  
  println!( "   📁 Development cycle complete - documentation at: {}", project_readme.display() );
  println!();
}

/// Workflow 2: CI/CD Pipeline Integration
fn workflow_cicd_pipeline()
{
  println!( "=== Workflow 2: CI/CD Pipeline Integration ===" );
  println!( "Simulating: PR created → Benchmarks run → Performance regression check → Merge/block decision" );
  
  // Simulate baseline performance (previous commit)
  let baseline_results = {
    let mut results = HashMap::new();
    let baseline_timings = vec![ 100, 105, 98, 102, 107, 101, 104, 97, 103, 99, 106, 96, 105, 100, 102 ];
    let times : Vec< Duration > = baseline_timings.iter()
      .map( | &t | Duration::from_micros( t ) )
      .collect();
    results.insert( "quicksort".to_string(), BenchmarkResult::new( "quicksort", times ) );
    results
  };
  
  // Simulate current PR performance (potential regression)
  let pr_results = {
    let mut results = HashMap::new();
    let pr_timings = vec![ 115, 120, 113, 117, 122, 116, 119, 112, 118, 114, 121, 111, 120, 115, 117 ];
    let times : Vec< Duration > = pr_timings.iter()
      .map( | &t | Duration::from_micros( t ) )
      .collect();
    results.insert( "quicksort".to_string(), BenchmarkResult::new( "quicksort", times ) );
    results
  };
  
  println!( "\n📊 Step 1: Running PR benchmark suite..." );
  println!( "   Baseline performance captured" );
  println!( "   PR performance measured" );
  
  // Validate both sets of results
  println!( "\n🔍 Step 2: Validating benchmark quality..." );
  let validator = BenchmarkValidator::new().require_warmup( false );
  
  let baseline_validated = ValidatedResults::new( baseline_results.clone(), validator.clone() );
  let pr_validated = ValidatedResults::new( pr_results.clone(), validator );
  
  let baseline_reliable = baseline_validated.reliability_rate() >= 90.0;
  let pr_reliable = pr_validated.reliability_rate() >= 90.0;
  
  println!( "   Baseline reliability: {:.1}% ({})", 
           baseline_validated.reliability_rate(),
           if baseline_reliable { "✅ Good" } else { "⚠️ Poor" } );
  
  println!( "   PR reliability: {:.1}% ({})", 
           pr_validated.reliability_rate(),
           if pr_reliable { "✅ Good" } else { "⚠️ Poor" } );
  
  if !baseline_reliable || !pr_reliable
  {
    println!( "   ⚠️ Quality issues detected - results may not be trustworthy" );
  }
  
  // Generate regression analysis
  println!( "\n📈 Step 3: Regression analysis..." );
  
  let _regression_template = ComparisonReport::new()
    .title( "Performance Regression Analysis" )
    .baseline( "quicksort" )  // Use same key for comparison
    .candidate( "quicksort" )
    .practical_significance_threshold( 0.05 );  // 5% regression threshold
  
  // Combine results for comparison (using different names)
  let mut combined_results = HashMap::new();
  combined_results.insert( "baseline_quicksort".to_string(), baseline_results[ "quicksort" ].clone() );
  combined_results.insert( "pr_quicksort".to_string(), pr_results[ "quicksort" ].clone() );
  
  let regression_comparison = ComparisonReport::new()
    .title( "PR Performance vs Baseline" )
    .baseline( "baseline_quicksort" )
    .candidate( "pr_quicksort" )
    .practical_significance_threshold( 0.05 );
  
  match regression_comparison.generate( &combined_results )
  {
    Ok( regression_report ) =>
    {
      // Analyze regression report for decision making
      let has_regression = regression_report.contains( "slower" );
      let has_improvement = regression_report.contains( "faster" );
      
      println!( "   Regression detected: {}", has_regression );
      println!( "   Improvement detected: {}", has_improvement );
      
      // CI/CD decision logic
      println!( "\n🚦 Step 4: CI/CD decision..." );
      
      if has_regression
      {
        println!( "   ❌ BLOCK MERGE: Performance regression detected" );
        println!( "   Action required: Investigate performance degradation" );
        println!( "   Recommendation: Review algorithmic changes in PR" );
        
        // Generate detailed report for developers
        let temp_file = std::env::temp_dir().join( "regression_report.md" );
        std::fs::write( &temp_file, &regression_report ).unwrap();
        println!( "   📄 Detailed regression report: {}", temp_file.display() );
        
        // Simulate posting comment to PR
        println!( "   💬 Posted regression warning to PR comments" );
      }
      else if has_improvement
      {
        println!( "   ✅ ALLOW MERGE: Performance improvement detected" );
        println!( "   Benefit: Code changes improve performance" );
        
        let temp_file = std::env::temp_dir().join( "improvement_report.md" );
        std::fs::write( &temp_file, &regression_report ).unwrap();
        println!( "   📄 Performance improvement report: {}", temp_file.display() );
        
        println!( "   💬 Posted performance improvement note to PR" );
      }
      else
      {
        println!( "   ✅ ALLOW MERGE: No significant performance change" );
        println!( "   Status: Performance remains within acceptable bounds" );
      }
    },
    Err( e ) => 
    {
      println!( "   ❌ Regression analysis failed: {}", e );
      println!( "   🚦 BLOCK MERGE: Cannot validate performance impact" );
    }
  }
  
  println!();
}

/// Workflow 3: Multi-Project Coordination
fn workflow_multi_project()
{
  println!( "=== Workflow 3: Multi-Project Coordination ===" );
  println!( "Simulating: Shared library changes → Test across dependent projects → Coordinate updates" );
  
  // Simulate multiple projects using the same library
  let projects = vec![
    ( "web-api", vec![ 85, 90, 83, 87, 92, 86, 89, 82, 88, 84, 91, 81, 90, 85, 87 ] ),
    ( "batch-processor", vec![ 150, 160, 145, 155, 165, 152, 158, 148, 157, 151, 162, 143, 159, 154, 156 ] ),
    ( "real-time-analyzer", vec![ 45, 50, 43, 47, 52, 46, 49, 42, 48, 44, 51, 41, 50, 45, 47 ] ),
  ];
  
  println!( "\n📊 Step 1: Running benchmarks across all dependent projects..." );
  
  let mut all_project_results = HashMap::new();
  for ( project_name, timings ) in projects
  {
    let times : Vec< Duration > = timings.iter()
      .map( | &t | Duration::from_micros( t ) )
      .collect();
    all_project_results.insert( 
      format!( "{}_performance", project_name ), 
      BenchmarkResult::new( &format!( "{}_performance", project_name ), times )
    );
    println!( "   ✅ {} benchmarks completed", project_name );
  }
  
  // Cross-project validation
  println!( "\n🔍 Step 2: Cross-project validation..." );
  let validator = BenchmarkValidator::new()
    .min_samples( 10 )
    .max_coefficient_variation( 0.20 )  // More lenient for different environments
    .require_warmup( false );
  
  let cross_project_validated = ValidatedResults::new( all_project_results.clone(), validator );
  
  println!( "   Overall reliability across projects: {:.1}%", cross_project_validated.reliability_rate() );
  
  if let Some( warnings ) = cross_project_validated.reliability_warnings()
  {
    println!( "   ⚠️ Cross-project quality issues:" );
    for warning in warnings.iter().take( 5 )  // Show first 5
    {
      println!( "     - {}", warning );
    }
  }
  
  // Generate consolidated report
  println!( "\n📄 Step 3: Generating consolidated report..." );
  
  let multi_project_template = PerformanceReport::new()
    .title( "Cross-Project Performance Impact Analysis" )
    .add_context( "Impact assessment of shared library changes across all dependent projects" )
    .include_statistical_analysis( true )
    .add_custom_section( CustomSection::new(
      "Project Impact Summary",
      r#"### Performance Impact by Project

| Project | Performance Change | Risk Level | Action Required |
|---------|-------------------|------------|-----------------|
| web-api | Baseline | 🟢 Low | None - continue monitoring |
| batch-processor | -5% throughput | 🟡 Medium | Review batch size optimization |
| real-time-analyzer | +12% improvement | 🟢 Low | Excellent - no action needed |

### Deployment Recommendations

1. **web-api**: Deploy with confidence - no performance impact
2. **batch-processor**: Deploy with monitoring - minor performance trade-off acceptable
3. **real-time-analyzer**: Priority deployment - significant performance gain

### Coordination Requirements

- All projects can upgrade simultaneously
- No breaking performance regressions detected
- Real-time-analyzer should prioritize upgrade for performance benefits"#
    ));
  
  let consolidated_report = multi_project_template.generate( &all_project_results ).unwrap();
  
  // Update shared documentation
  let shared_doc = std::env::temp_dir().join( "SHARED_LIBRARY_IMPACT.md" );
  let shared_content = r#"# Shared Library Performance Impact

## Overview

This document tracks performance impact across all dependent projects.

## Current Impact Analysis

*Cross-project performance analysis will be updated here.*

## Deployment Status

*Project-specific deployment recommendations and status.*

## Historical Trends

*Performance trends across library versions.*
"#;
  
  std::fs::write( &shared_doc, shared_content ).unwrap();
  
  let shared_chain = MarkdownUpdateChain::new( &shared_doc ).unwrap()
    .add_section( "Current Impact Analysis", &consolidated_report );
  
  match shared_chain.execute()
  {
    Ok( () ) =>
    {
      println!( "   ✅ Consolidated documentation updated" );
      println!( "   📁 Shared impact analysis: {}", shared_doc.display() );
      
      // Simulate notification to project maintainers
      println!( "\n📧 Step 4: Notifying project maintainers..." );
      println!( "   • web-api team: No action required" );
      println!( "   • batch-processor team: Minor performance impact noted" );
      println!( "   • real-time-analyzer team: Performance improvement available" );
      
      // Simulate coordination meeting
      println!( "\n🤝 Step 5: Coordination meeting scheduled..." );
      println!( "   All teams aligned on deployment strategy" );
      println!( "   Upgrade timeline coordinated across projects" );
    },
    Err( e ) => println!( "   ❌ Consolidated update failed: {}", e ),
  }
  
  println!();
}

/// Workflow 4: Production Monitoring
fn workflow_production_monitoring()
{
  println!( "=== Workflow 4: Production Monitoring & Alerting ===" );
  println!( "Simulating: Scheduled production benchmarks → Quality validation → Alert on regressions" );
  
  // Simulate production performance over time
  let production_scenarios = vec![
    ( "week_1", vec![ 95, 100, 92, 98, 103, 96, 101, 94, 99, 97 ] ),
    ( "week_2", vec![ 97, 102, 94, 100, 105, 98, 103, 96, 101, 99 ] ),  // Slight degradation
    ( "week_3", vec![ 110, 115, 108, 112, 117, 111, 114, 107, 113, 109 ] ),  // Significant regression
    ( "week_4", vec![ 98, 103, 95, 101, 106, 99, 104, 97, 102, 100 ] ),  // Recovery
  ];
  
  println!( "\n📊 Step 1: Production monitoring data collection..." );
  
  let mut weekly_results = HashMap::new();
  for ( week, timings ) in production_scenarios
  {
    let times : Vec< Duration > = timings.iter()
      .map( | &t | Duration::from_micros( t ) )
      .collect();
    weekly_results.insert( 
      format!( "production_{}", week ), 
      BenchmarkResult::new( &format!( "production_{}", week ), times )
    );
    println!( "   📈 {} performance captured", week );
  }
  
  // Production-grade validation
  println!( "\n🔍 Step 2: Production quality validation..." );
  let production_validator = BenchmarkValidator::new()
    .min_samples( 8 )  // Production data may be limited
    .max_coefficient_variation( 0.25 )  // Production has more noise
    .require_warmup( false )
    .max_time_ratio( 3.0 );
  
  let production_validated = ValidatedResults::new( weekly_results.clone(), production_validator );
  
  println!( "   Production data reliability: {:.1}%", production_validated.reliability_rate() );
  
  // Regression detection across weeks
  println!( "\n🚨 Step 3: Regression detection and alerting..." );
  
  // Compare each week to the baseline (week_1)
  let weeks = vec![ "week_2", "week_3", "week_4" ];
  let mut alerts = Vec::new();
  
  for week in weeks
  {
    let comparison = ComparisonReport::new()
      .title( &format!( "Week 1 vs {} Comparison", week ) )
      .baseline( "production_week_1" )
      .candidate( &format!( "production_{}", week ) )
      .practical_significance_threshold( 0.10 );  // 10% regression threshold
    
    match comparison.generate( &weekly_results )
    {
      Ok( report ) =>
      {
        let has_regression = report.contains( "slower" );
        let regression_percentage = if has_regression
        {
          // Extract performance change (simplified)
          if week == "week_3" { 15.0 } else { 2.0 }  // Simulated extraction
        }
        else
        {
          0.0
        };
        
        if has_regression && regression_percentage > 10.0
        {
          alerts.push( format!( 
            "🚨 CRITICAL: {} shows {:.1}% performance regression", 
            week, regression_percentage 
          ));
          
          // Save detailed regression report
          let alert_file = std::env::temp_dir().join( format!( "ALERT_{}.md", week ) );
          std::fs::write( &alert_file, &report ).unwrap();
          
          println!( "   🚨 ALERT: {} performance regression detected", week );
          println!( "   📄 Alert report: {}", alert_file.display() );
        }
        else if has_regression
        {
          println!( "   ⚠️ Minor regression in {}: {:.1}%", week, regression_percentage );
        }
        else
        {
          println!( "   ✅ {} performance within normal bounds", week );
        }
      },
      Err( e ) => println!( "   ❌ {} comparison failed: {}", week, e ),
    }
  }
  
  // Generate monitoring dashboard update
  println!( "\n📊 Step 4: Updating monitoring dashboard..." );
  
  let monitoring_template = PerformanceReport::new()
    .title( "Production Performance Monitoring Dashboard" )
    .add_context( "Automated weekly performance tracking with regression detection" )
    .include_statistical_analysis( true )
    .add_custom_section( CustomSection::new(
      "Alert Summary",
      {
        if alerts.is_empty()
        {
          "✅ **No alerts**: All performance metrics within acceptable bounds.".to_string()
        }
        else
        {
          format!( 
            "🚨 **Active Alerts**:\n\n{}\n\n**Action Required**: Investigate performance regressions immediately.",
            alerts.join( "\n" )
          )
        }
      }
    ));
  
  let dashboard_report = monitoring_template.generate( &weekly_results ).unwrap();
  
  let dashboard_file = std::env::temp_dir().join( "PRODUCTION_DASHBOARD.md" );
  let dashboard_chain = MarkdownUpdateChain::new( &dashboard_file ).unwrap()
    .add_section( "Current Status", &dashboard_report );
  
  match dashboard_chain.execute()
  {
    Ok( () ) =>
    {
      println!( "   ✅ Monitoring dashboard updated" );
      println!( "   📊 Dashboard: {}", dashboard_file.display() );
      
      // Simulate alerting system
      if !alerts.is_empty()
      {
        println!( "\n🔔 Step 5: Alerting system activated..." );
        for alert in alerts
        {
          println!( "   📧 Email sent: {}", alert );
          println!( "   📱 Slack notification posted" );
          println!( "   📞 PagerDuty incident created" );
        }
      }
      else
      {
        println!( "\n✅ Step 5: No alerts triggered - system healthy" );
      }
    },
    Err( e ) => println!( "   ❌ Dashboard update failed: {}", e ),
  }
  
  println!();
}

fn main()
{
  println!( "🚀 Complete Integration Workflow Examples\n" );
  
  workflow_development_cycle();
  workflow_cicd_pipeline();
  workflow_multi_project();
  workflow_production_monitoring();
  
  println!( "📋 Integration Workflow Patterns Covered:" );
  println!( "✅ Development cycle: benchmark → validate → document → commit" );
  println!( "✅ CI/CD pipeline: regression detection → merge decision → automated reporting" );
  println!( "✅ Multi-project coordination: impact analysis → consolidated reporting → team alignment" );
  println!( "✅ Production monitoring: continuous tracking → alerting → dashboard updates" );
  println!( "\n🎯 These patterns demonstrate real-world integration scenarios" );
  println!( "   combining validation, templating, and update chains for complete automation." );
  
  println!( "\n📁 Generated workflow artifacts saved to:" );
  println!( "   {}", std::env::temp_dir().display() );
}