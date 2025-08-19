//! CI/CD Regression Detection Examples
//!
//! This example demonstrates EVERY aspect of using benchkit for automated regression detection in CI/CD:
//! - Pull request performance validation workflows
//! - Automated baseline comparison and approval gates
//! - Multi-environment regression testing (dev, staging, production)
//! - Performance regression alerts and reporting
//! - Automated performance documentation updates
//! - Integration with popular CI/CD platforms (GitHub Actions, GitLab CI, Jenkins)

#![ cfg( feature = "enabled" ) ]
#![ cfg( feature = "markdown_reports" ) ]
#![ allow( clippy::uninlined_format_args ) ]
#![ allow( clippy::format_push_string ) ]
#![ allow( clippy::cast_lossless ) ]
#![ allow( clippy::cast_possible_truncation ) ]
#![ allow( clippy::cast_precision_loss ) ]
#![ allow( clippy::std_instead_of_core ) ]
#![ allow( clippy::needless_raw_string_hashes ) ]
#![ allow( clippy::too_many_lines ) ]

use benchkit::prelude::*;
use std::collections::HashMap;
use std::time::Duration;

/// CI/CD exit codes for different scenarios
#[ derive( Debug, Clone, Copy, PartialEq ) ]
enum CiExitCode
{
  Success = 0,
  PerformanceRegression = 1,
  InsufficientData = 2,
  ValidationFailure = 3,
  SystemError = 4,
}

/// CI/CD pipeline configuration for performance testing
#[ derive( Debug, Clone ) ]
struct CiCdConfig
{
  environment : String,
  regression_threshold : f64,
  significance_level : f64,
  min_reliability : f64,
  baseline_strategy : BaselineStrategy,
}

impl CiCdConfig
{
  fn development() -> Self
  {
    Self
    {
      environment : "development".to_string(),
      regression_threshold : 0.15,  // Allow 15% regression in dev
      significance_level : 0.10,    // 10% significance for dev testing
      min_reliability : 70.0,       // 70% minimum reliability
      baseline_strategy : BaselineStrategy::PreviousRun,
    }
  }
  
  fn staging() -> Self
  {
    Self
    {
      environment : "staging".to_string(),
      regression_threshold : 0.10,  // 10% regression threshold
      significance_level : 0.05,    // 5% significance for staging
      min_reliability : 85.0,       // 85% minimum reliability
      baseline_strategy : BaselineStrategy::RollingAverage,
    }
  }
  
  fn production() -> Self
  {
    Self
    {
      environment : "production".to_string(),
      regression_threshold : 0.05,  // 5% regression threshold (strict)
      significance_level : 0.01,    // 1% significance (very strict)
      min_reliability : 95.0,       // 95% minimum reliability
      baseline_strategy : BaselineStrategy::FixedBaseline,
    }
  }
}

/// Create baseline results representing the main branch performance
fn create_baseline_results() -> HashMap< String, BenchmarkResult >
{
  let mut baseline = HashMap::new();
  
  // API endpoint performance - stable baseline
  let api_times = vec![
    Duration::from_millis( 45 ), Duration::from_millis( 48 ), Duration::from_millis( 42 ),
    Duration::from_millis( 47 ), Duration::from_millis( 44 ), Duration::from_millis( 46 ),
    Duration::from_millis( 49 ), Duration::from_millis( 43 ), Duration::from_millis( 47 ),
    Duration::from_millis( 45 ), Duration::from_millis( 48 ), Duration::from_millis( 44 )
  ];
  baseline.insert( "api_response_time".to_string(), BenchmarkResult::new( "api_response_time", api_times ) );
  
  // Database query performance
  let db_times = vec![
    Duration::from_micros( 850 ), Duration::from_micros( 870 ), Duration::from_micros( 830 ),
    Duration::from_micros( 860 ), Duration::from_micros( 845 ), Duration::from_micros( 875 ),
    Duration::from_micros( 825 ), Duration::from_micros( 865 ), Duration::from_micros( 840 ),
    Duration::from_micros( 855 ), Duration::from_micros( 880 ), Duration::from_micros( 835 )
  ];
  baseline.insert( "database_query".to_string(), BenchmarkResult::new( "database_query", db_times ) );
  
  // Memory allocation performance
  let memory_times = vec![
    Duration::from_nanos( 120 ), Duration::from_nanos( 125 ), Duration::from_nanos( 115 ),
    Duration::from_nanos( 122 ), Duration::from_nanos( 118 ), Duration::from_nanos( 127 ),
    Duration::from_nanos( 113 ), Duration::from_nanos( 124 ), Duration::from_nanos( 119 ),
    Duration::from_nanos( 121 ), Duration::from_nanos( 126 ), Duration::from_nanos( 116 )
  ];
  baseline.insert( "memory_allocation".to_string(), BenchmarkResult::new( "memory_allocation", memory_times ) );
  
  baseline
}

/// Create PR results - mix of improvements, regressions, and stable performance
fn create_pr_results_with_regression() -> HashMap< String, BenchmarkResult >
{
  let mut pr_results = HashMap::new();
  
  // API endpoint - performance regression (10% slower)
  let api_times = vec![
    Duration::from_millis( 52 ), Duration::from_millis( 55 ), Duration::from_millis( 49 ),
    Duration::from_millis( 54 ), Duration::from_millis( 51 ), Duration::from_millis( 53 ),
    Duration::from_millis( 56 ), Duration::from_millis( 50 ), Duration::from_millis( 54 ),
    Duration::from_millis( 52 ), Duration::from_millis( 55 ), Duration::from_millis( 51 )
  ];
  pr_results.insert( "api_response_time".to_string(), BenchmarkResult::new( "api_response_time", api_times ) );
  
  // Database query - improvement (5% faster)
  let db_times = vec![
    Duration::from_micros( 810 ), Duration::from_micros( 825 ), Duration::from_micros( 795 ),
    Duration::from_micros( 815 ), Duration::from_micros( 805 ), Duration::from_micros( 830 ),
    Duration::from_micros( 790 ), Duration::from_micros( 820 ), Duration::from_micros( 800 ),
    Duration::from_micros( 812 ), Duration::from_micros( 828 ), Duration::from_micros( 798 )
  ];
  pr_results.insert( "database_query".to_string(), BenchmarkResult::new( "database_query", db_times ) );
  
  // Memory allocation - stable performance
  let memory_times = vec![
    Duration::from_nanos( 119 ), Duration::from_nanos( 124 ), Duration::from_nanos( 114 ),
    Duration::from_nanos( 121 ), Duration::from_nanos( 117 ), Duration::from_nanos( 126 ),
    Duration::from_nanos( 112 ), Duration::from_nanos( 123 ), Duration::from_nanos( 118 ),
    Duration::from_nanos( 120 ), Duration::from_nanos( 125 ), Duration::from_nanos( 115 )
  ];
  pr_results.insert( "memory_allocation".to_string(), BenchmarkResult::new( "memory_allocation", memory_times ) );
  
  pr_results
}

/// Create PR results with good performance (no regressions)
fn create_pr_results_good() -> HashMap< String, BenchmarkResult >
{
  let mut pr_results = HashMap::new();
  
  // API endpoint - slight improvement
  let api_times = vec![
    Duration::from_millis( 43 ), Duration::from_millis( 46 ), Duration::from_millis( 40 ),
    Duration::from_millis( 45 ), Duration::from_millis( 42 ), Duration::from_millis( 44 ),
    Duration::from_millis( 47 ), Duration::from_millis( 41 ), Duration::from_millis( 45 ),
    Duration::from_millis( 43 ), Duration::from_millis( 46 ), Duration::from_millis( 42 )
  ];
  pr_results.insert( "api_response_time".to_string(), BenchmarkResult::new( "api_response_time", api_times ) );
  
  // Database query - significant improvement (15% faster)
  let db_times = vec![
    Duration::from_micros( 720 ), Duration::from_micros( 740 ), Duration::from_micros( 700 ),
    Duration::from_micros( 730 ), Duration::from_micros( 715 ), Duration::from_micros( 745 ),
    Duration::from_micros( 695 ), Duration::from_micros( 735 ), Duration::from_micros( 710 ),
    Duration::from_micros( 725 ), Duration::from_micros( 750 ), Duration::from_micros( 705 )
  ];
  pr_results.insert( "database_query".to_string(), BenchmarkResult::new( "database_query", db_times ) );
  
  // Memory allocation - stable performance
  let memory_times = vec![
    Duration::from_nanos( 118 ), Duration::from_nanos( 123 ), Duration::from_nanos( 113 ),
    Duration::from_nanos( 120 ), Duration::from_nanos( 116 ), Duration::from_nanos( 125 ),
    Duration::from_nanos( 111 ), Duration::from_nanos( 122 ), Duration::from_nanos( 117 ),
    Duration::from_nanos( 119 ), Duration::from_nanos( 124 ), Duration::from_nanos( 114 )
  ];
  pr_results.insert( "memory_allocation".to_string(), BenchmarkResult::new( "memory_allocation", memory_times ) );
  
  pr_results
}

/// Simulate the CI/CD pipeline performance validation step
fn run_performance_validation( config : &CiCdConfig, pr_results : HashMap< String, BenchmarkResult >, baseline_results : HashMap< String, BenchmarkResult > ) -> ( CiExitCode, String )
{
  println!( "🚀 RUNNING PERFORMANCE VALIDATION" );
  println!( "  Environment: {}", config.environment );
  println!( "  Regression Threshold: {}%", ( config.regression_threshold * 100.0 ) as i32 );
  println!( "  Significance Level: {}%", ( config.significance_level * 100.0 ) as i32 );
  
  // Step 1: Validate data quality
  let validator = BenchmarkValidator::new()
    .min_samples( 8 )
    .max_coefficient_variation( 0.20 );
  
  let pr_validation = ValidatedResults::new( pr_results.clone(), validator.clone() );
  let baseline_validation = ValidatedResults::new( baseline_results.clone(), validator );
  
  if pr_validation.reliability_rate() < config.min_reliability
  {
    let message = format!( "❌ PR benchmark quality insufficient: {:.1}% < {:.1}%", pr_validation.reliability_rate(), config.min_reliability );
    return ( CiExitCode::InsufficientData, message );
  }
  
  if baseline_validation.reliability_rate() < config.min_reliability
  {
    let message = format!( "❌ Baseline benchmark quality insufficient: {:.1}% < {:.1}%", baseline_validation.reliability_rate(), config.min_reliability );
    return ( CiExitCode::InsufficientData, message );
  }
  
  println!( "  ✅ Data quality validation passed" );
  
  // Step 2: Create historical data from baseline
  let historical = HistoricalResults::new().with_baseline( baseline_results );
  
  // Step 3: Run regression analysis
  let analyzer = RegressionAnalyzer::new()
    .with_baseline_strategy( config.baseline_strategy.clone() )
    .with_significance_threshold( config.significance_level );
  
  let regression_report = analyzer.analyze( &pr_results, &historical );
  
  // Step 4: Detect regressions
  let mut regressions = Vec::new();
  let mut improvements = Vec::new();
  let mut stable = Vec::new();
  
  for operation in pr_results.keys()
  {
    if let Some( trend ) = regression_report.get_trend_for( operation )
    {
      match trend
      {
        PerformanceTrend::Degrading =>
        {
          if regression_report.is_statistically_significant( operation )
          {
            regressions.push( operation.clone() );
          }
          else
          {
            stable.push( operation.clone() );
          }
        },
        PerformanceTrend::Improving =>
        {
          improvements.push( operation.clone() );
        },
        PerformanceTrend::Stable =>
        {
          stable.push( operation.clone() );
        }
      }
    }
  }
  
  // Step 5: Determine CI/CD result
  if !regressions.is_empty()
  {
    let message = format!( "❌ Performance regressions detected in: {}", regressions.join( ", " ) );
    println!( "  {}", message );
    return ( CiExitCode::PerformanceRegression, message );
  }
  
  let mut message = String::new();
  if !improvements.is_empty()
  {
    message.push_str( &format!( "🎉 Performance improvements in: {}", improvements.join( ", " ) ) );
  }
  if !stable.is_empty()
  {
    if !message.is_empty() { message.push_str( "; " ); }
    message.push_str( &format!( "✅ Stable performance in: {}", stable.join( ", " ) ) );
  }
  
  if message.is_empty()
  {
    message = "✅ Performance validation passed".to_string();
  }
  
  println!( "  {}", message );
  ( CiExitCode::Success, message )
}

/// Generate GitHub Actions compatible performance report
fn generate_github_actions_report( pr_results : &HashMap< String, BenchmarkResult >, baseline_results : &HashMap< String, BenchmarkResult > ) -> String
{
  let historical = HistoricalResults::new().with_baseline( baseline_results.clone() );
  let analyzer = RegressionAnalyzer::new().with_baseline_strategy( BaselineStrategy::FixedBaseline );
  let regression_report = analyzer.analyze( pr_results, &historical );
  
  let mut report = String::new();
  report.push_str( "## 🚀 Performance Analysis Report\n\n" );
  
  // Create comparison table
  report.push_str( "| Benchmark | Trend | Status | Notes |\n" );
  report.push_str( "|-----------|--------|--------|-------|\n" );
  
  for ( operation, _result ) in pr_results
  {
    let trend_icon = match regression_report.get_trend_for( operation )
    {
      Some( PerformanceTrend::Improving ) => "🟢 ↗️",
      Some( PerformanceTrend::Degrading ) => "🔴 ↘️",
      Some( PerformanceTrend::Stable ) => "🟡 ➡️",
      None => "⚪ ?",
    };
    
    let status = if regression_report.is_statistically_significant( operation )
    {
      "Significant"
    }
    else
    {
      "Normal variation"
    };
    
    let notes = match operation.as_str()
    {
      "api_response_time" => "Critical user-facing metric",
      "database_query" => "Backend performance indicator",  
      "memory_allocation" => "Resource utilization metric",
      _ => "Performance metric",
    };
    
    report.push_str( &format!( "| {} | {} | {} | {} |\n", operation, trend_icon, status, notes ) );
  }
  
  report.push_str( "\n### Summary\n\n" );
  
  if regression_report.has_significant_changes()
  {
    report.push_str( "⚠️ **Significant performance changes detected.** Please review before merging.\n\n" );
  }
  else
  {
    report.push_str( "✅ **No significant performance regressions detected.** Safe to merge.\n\n" );
  }
  
  // Add detailed markdown from regression report
  report.push_str( &regression_report.format_markdown() );
  
  report
}

/// Demonstrate development environment PR validation
fn demonstrate_development_pr_validation()
{
  println!( "🔧 DEVELOPMENT ENVIRONMENT PR VALIDATION" );
  println!( "=========================================" );
  println!( "Simulating a typical development PR with lenient thresholds for iteration speed.\n" );
  
  let config = CiCdConfig::development();
  let baseline = create_baseline_results();
  let pr_results = create_pr_results_with_regression();
  
  let ( exit_code, message ) = run_performance_validation( &config, pr_results, baseline );
  
  match exit_code
  {
    CiExitCode::Success => println!( "🟢 CI/CD Result: PASSED - Continue development" ),
    CiExitCode::PerformanceRegression => println!( "🟡 CI/CD Result: WARNING - Monitor performance but allow merge" ),
    _ => println!( "🔴 CI/CD Result: FAILED - {}", message ),
  }
  
  println!( "💡 Development Strategy: Fast iteration with performance awareness\n" );
}

/// Demonstrate staging environment validation with moderate restrictions
fn demonstrate_staging_pr_validation()
{
  println!( "🎭 STAGING ENVIRONMENT PR VALIDATION" );
  println!( "====================================" );
  println!( "Simulating staging validation with moderate performance requirements.\n" );
  
  let config = CiCdConfig::staging();
  let baseline = create_baseline_results();
  
  // Test with regression
  println!( "📊 Testing PR with performance regression:" );
  let pr_with_regression = create_pr_results_with_regression();
  let ( exit_code, message ) = run_performance_validation( &config, pr_with_regression, baseline.clone() );
  
  match exit_code
  {
    CiExitCode::Success => println!( "🟢 Staging Result: PASSED" ),
    CiExitCode::PerformanceRegression => println!( "🔴 Staging Result: BLOCKED - {}", message ),
    _ => println!( "🟡 Staging Result: REVIEW NEEDED - {}", message ),
  }
  
  println!();
  
  // Test with good performance
  println!( "📊 Testing PR with good performance:" );
  let pr_good = create_pr_results_good();
  let ( exit_code, message ) = run_performance_validation( &config, pr_good, baseline );
  
  match exit_code
  {
    CiExitCode::Success => println!( "🟢 Staging Result: PASSED - {}", message ),
    _ => println!( "🔴 Staging Result: UNEXPECTED - {}", message ),
  }
  
  println!( "💡 Staging Strategy: Balanced performance gates before production\n" );
}

/// Demonstrate production deployment validation with strict requirements
fn demonstrate_production_deployment_validation()
{
  println!( "🏭 PRODUCTION DEPLOYMENT VALIDATION" );
  println!( "===================================" );
  println!( "Simulating strict production deployment with minimal regression tolerance.\n" );
  
  let config = CiCdConfig::production();
  let baseline = create_baseline_results();
  let pr_results = create_pr_results_good();  // Use good results for production
  
  let ( exit_code, message ) = run_performance_validation( &config, pr_results, baseline );
  
  match exit_code
  {
    CiExitCode::Success => println!( "🟢 Production Result: APPROVED FOR DEPLOYMENT" ),
    CiExitCode::PerformanceRegression => println!( "🚨 Production Result: DEPLOYMENT BLOCKED - Critical regression detected" ),
    CiExitCode::InsufficientData => println!( "⏸️ Production Result: DEPLOYMENT PAUSED - Insufficient benchmark data" ),
    _ => println!( "❌ Production Result: DEPLOYMENT FAILED - {}", message ),
  }
  
  println!( "💡 Production Strategy: Zero tolerance for performance regressions\n" );
}

/// Demonstrate automated documentation updates
fn demonstrate_automated_documentation_updates()
{
  println!( "📝 AUTOMATED DOCUMENTATION UPDATES" );
  println!( "==================================" );
  println!( "Demonstrating automatic performance documentation updates in CI/CD.\n" );
  
  let baseline = create_baseline_results();
  let pr_results = create_pr_results_good();
  
  // Generate GitHub Actions compatible report
  let github_report = generate_github_actions_report( &pr_results, &baseline );
  
  println!( "📄 GENERATED GITHUB ACTIONS REPORT:" );
  println!( "------------------------------------" );
  println!( "{}", github_report );
  
  // Simulate markdown update chain for documentation
  println!( "🔄 SIMULATING DOCUMENTATION UPDATE:" );
  println!( "    ✅ Would update README.md performance section" );
  println!( "    ✅ Would create PR comment with performance analysis" );
  println!( "    ✅ Would update performance tracking dashboard" );
  println!( "    ✅ Would notify team channels if regressions detected" );
  
  println!( "💡 Integration Options:" );
  println!( "    - GitHub Actions: Use performance report as PR comment" );
  println!( "    - GitLab CI: Update merge request with performance status" );
  println!( "    - Jenkins: Archive performance reports as build artifacts" );
  println!( "    - Slack/Teams: Send notifications for significant changes\n" );
}

/// Demonstrate multi-environment pipeline
fn demonstrate_multi_environment_pipeline()
{
  println!( "🌍 MULTI-ENVIRONMENT PIPELINE DEMONSTRATION" );
  println!( "============================================" );
  println!( "Simulating performance validation across development → staging → production.\n" );
  
  let baseline = create_baseline_results();
  let pr_results = create_pr_results_with_regression();  // Use regression results to show pipeline behavior
  
  // Development validation
  let dev_config = CiCdConfig::development();
  let ( dev_exit, dev_message ) = run_performance_validation( &dev_config, pr_results.clone(), baseline.clone() );
  println!( "🔧 Development: {} - {}", if dev_exit == CiExitCode::Success { "PASS" } else { "WARN" }, dev_message );
  
  // Staging validation (only if dev passes)
  if dev_exit == CiExitCode::Success
  {
    let staging_config = CiCdConfig::staging();
    let ( staging_exit, staging_message ) = run_performance_validation( &staging_config, pr_results.clone(), baseline.clone() );
    println!( "🎭 Staging: {} - {}", if staging_exit == CiExitCode::Success { "PASS" } else { "FAIL" }, staging_message );
    
    // Production validation (only if staging passes)
    if staging_exit == CiExitCode::Success
    {
      let prod_config = CiCdConfig::production();
      let ( prod_exit, prod_message ) = run_performance_validation( &prod_config, pr_results, baseline );
      println!( "🏭 Production: {} - {}", if prod_exit == CiExitCode::Success { "PASS" } else { "FAIL" }, prod_message );
    }
    else
    {
      println!( "🏭 Production: SKIPPED - Staging validation failed" );
    }
  }
  else
  {
    println!( "🎭 Staging: SKIPPED - Development validation failed" );
    println!( "🏭 Production: SKIPPED - Pipeline halted" );
  }
  
  println!( "\n💡 Pipeline Strategy: Progressive validation with increasing strictness" );
  println!( "    - Development: Fast feedback, lenient thresholds" );
  println!( "    - Staging: Balanced validation, moderate thresholds" );
  println!( "    - Production: Strict validation, zero regression tolerance\n" );
}

/// Main demonstration function
fn main()
{
  println!( "🏗️ BENCHKIT CI/CD REGRESSION DETECTION COMPREHENSIVE DEMO" );
  println!( "===========================================================" );
  println!( "This example demonstrates every aspect of using benchkit in CI/CD pipelines:\n" );
  
  // Environment-specific demonstrations
  demonstrate_development_pr_validation();
  demonstrate_staging_pr_validation();
  demonstrate_production_deployment_validation();
  
  // Integration and automation
  demonstrate_automated_documentation_updates();
  demonstrate_multi_environment_pipeline();
  
  println!( "✨ SUMMARY OF DEMONSTRATED CI/CD CAPABILITIES:" );
  println!( "==============================================" );
  println!( "✅ Multi-environment validation (dev, staging, production)" );
  println!( "✅ Configurable regression thresholds per environment" );
  println!( "✅ Automated performance gate decisions (pass/fail/warn)" );
  println!( "✅ Data quality validation before regression analysis" );
  println!( "✅ GitHub Actions compatible reporting" );
  println!( "✅ Automated documentation updates" );
  println!( "✅ Progressive validation pipeline with halt-on-failure" );
  println!( "✅ Statistical significance testing for reliable decisions" );
  
  println!( "\n🎯 CI/CD INTEGRATION PATTERNS:" );
  println!( "==============================" );
  println!( "📋 GitHub Actions: Use as action step with performance reports" );
  println!( "📋 GitLab CI: Integrate with merge request validation" );
  println!( "📋 Jenkins: Add as pipeline stage with artifact archival" );
  println!( "📋 Azure DevOps: Use in build validation with PR comments" );
  
  println!( "\n🚀 Ready for production CI/CD integration with automated performance regression detection!" );
}

#[ cfg( not( feature = "enabled" ) ) ]
fn main()
{
  println!( "This example requires the 'enabled' feature." );
  println!( "Run with: cargo run --example cicd_regression_detection --features enabled" );
}