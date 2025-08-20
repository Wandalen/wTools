//! Tests for benchmark validation framework

#![ allow( clippy::std_instead_of_core ) ]

#[ cfg( feature = "integration" ) ]
mod tests
{
  use benchkit::prelude::*;
  use std::collections::HashMap;
  use std::time::Duration;

  fn create_reliable_result() -> BenchmarkResult
  {
    // 12 samples with low variability - should be reliable
    let times = vec![
      Duration::from_micros( 100 ), Duration::from_micros( 102 ), Duration::from_micros( 98 ),
      Duration::from_micros( 101 ), Duration::from_micros( 99 ), Duration::from_micros( 100 ),
      Duration::from_micros( 103 ), Duration::from_micros( 97 ), Duration::from_micros( 101 ),
      Duration::from_micros( 100 ), Duration::from_micros( 102 ), Duration::from_micros( 99 )
    ];
    BenchmarkResult::new( "reliable_test", times )
  }

  fn create_unreliable_result() -> BenchmarkResult
  {
    // Few samples with high variability - should be unreliable
    let times = vec![
      Duration::from_micros( 100 ), Duration::from_micros( 200 ), Duration::from_micros( 50 ),
      Duration::from_micros( 150 ), Duration::from_micros( 80 )
    ];
    BenchmarkResult::new( "reliable_test", times )
  }

  fn create_short_duration_result() -> BenchmarkResult
  {
    // Very short durations - should trigger short measurement warning
    let times = vec![
      Duration::from_nanos( 10 ), Duration::from_nanos( 12 ), Duration::from_nanos( 8 ),
      Duration::from_nanos( 11 ), Duration::from_nanos( 9 ), Duration::from_nanos( 10 ),
      Duration::from_nanos( 13 ), Duration::from_nanos( 7 ), Duration::from_nanos( 11 ),
      Duration::from_nanos( 10 ), Duration::from_nanos( 12 ), Duration::from_nanos( 9 )
    ];
    BenchmarkResult::new( "reliable_test", times )
  }

  fn create_no_warmup_result() -> BenchmarkResult
  {
    // All measurements similar - no warmup detected
    let times = vec![
      Duration::from_micros( 100 ), Duration::from_micros( 101 ), Duration::from_micros( 99 ),
      Duration::from_micros( 100 ), Duration::from_micros( 102 ), Duration::from_micros( 98 ),
      Duration::from_micros( 101 ), Duration::from_micros( 99 ), Duration::from_micros( 100 ),
      Duration::from_micros( 102 ), Duration::from_micros( 98 ), Duration::from_micros( 101 )
    ];
    BenchmarkResult::new( "reliable_test", times )
  }

  #[ test ]
  fn test_validator_default_settings()
  {
    let validator = BenchmarkValidator::new();
    
    // Test reliable result
    let reliable = create_reliable_result();
    let warnings = validator.validate_result( &reliable );
    assert!( warnings.is_empty() || warnings.len() == 1 ); // May have warmup warning
    
    // Test unreliable result
    let unreliable = create_unreliable_result();
    let warnings = validator.validate_result( &unreliable );
    assert!( !warnings.is_empty() );
  }

  #[ test ]
  fn test_insufficient_samples_warning()
  {
    let validator = BenchmarkValidator::new().min_samples( 20 );
    let result = create_reliable_result(); // Only 12 samples
    
    let warnings = validator.validate_result( &result );
    
    let has_sample_warning = warnings.iter().any( | w | matches!( w, ValidationWarning::InsufficientSamples { .. } ) );
    assert!( has_sample_warning );
  }

  #[ test ]
  fn test_high_variability_warning()
  {
    let validator = BenchmarkValidator::new().max_coefficient_variation( 0.05 ); // Very strict
    let result = create_unreliable_result();
    
    let warnings = validator.validate_result( &result );
    
    let has_variability_warning = warnings.iter().any( | w | matches!( w, ValidationWarning::HighVariability { .. } ) );
    assert!( has_variability_warning );
  }

  #[ test ]
  fn test_short_measurement_time_warning()
  {
    let validator = BenchmarkValidator::new().min_measurement_time( Duration::from_micros( 50 ) );
    let result = create_short_duration_result();
    
    let warnings = validator.validate_result( &result );
    
    let has_duration_warning = warnings.iter().any( | w | matches!( w, ValidationWarning::ShortMeasurementTime { .. } ) );
    assert!( has_duration_warning );
  }

  #[ test ]
  fn test_no_warmup_warning()
  {
    let validator = BenchmarkValidator::new().require_warmup( true );
    let result = create_no_warmup_result();
    
    let warnings = validator.validate_result( &result );
    
    let has_warmup_warning = warnings.iter().any( | w | matches!( w, ValidationWarning::NoWarmup ) );
    assert!( has_warmup_warning );
  }

  #[ test ]
  fn test_wide_performance_range_warning()
  {
    let validator = BenchmarkValidator::new().max_time_ratio( 1.5 ); // Very strict
    let result = create_unreliable_result(); // Has wide range
    
    let warnings = validator.validate_result( &result );
    
    let has_range_warning = warnings.iter().any( | w | matches!( w, ValidationWarning::WidePerformanceRange { .. } ) );
    assert!( has_range_warning );
  }

  #[ test ]
  fn test_validator_builder_pattern()
  {
    let validator = BenchmarkValidator::new()
      .min_samples( 5 )
      .max_coefficient_variation( 0.2 )
      .require_warmup( false )
      .max_time_ratio( 5.0 )
      .min_measurement_time( Duration::from_nanos( 1 ) );
    
    let result = create_unreliable_result();
    let warnings = validator.validate_result( &result );
    
    // With relaxed criteria, should have fewer warnings
    assert!( warnings.len() <= 2 ); // Might still have some warnings
  }

  #[ test ]
  fn test_validate_multiple_results()
  {
    let validator = BenchmarkValidator::new();
    
    let mut results = HashMap::new();
    results.insert( "reliable".to_string(), create_reliable_result() );
    results.insert( "unreliable".to_string(), create_unreliable_result() );
    results.insert( "short_duration".to_string(), create_short_duration_result() );
    
    let validation_results = validator.validate_results( &results );
    
    assert_eq!( validation_results.len(), 3 );
    
    // Reliable should have few or no warnings
    let reliable_warnings = &validation_results[ "reliable" ];
    assert!( reliable_warnings.len() <= 1 ); // May have warmup warning
    
    // Unreliable should have warnings
    let unreliable_warnings = &validation_results[ "unreliable" ];
    assert!( !unreliable_warnings.is_empty() );
    
    // Short duration should have warnings
    let short_warnings = &validation_results[ "short_duration" ];
    assert!( !short_warnings.is_empty() );
  }

  #[ test ]
  fn test_is_reliable()
  {
    let validator = BenchmarkValidator::new();
    
    let reliable = create_reliable_result();
    let unreliable = create_unreliable_result();
    
    // Note: reliable may still fail due to warmup detection
    // So we test with warmup disabled
    let validator_no_warmup = validator.require_warmup( false );
    
    assert!( validator_no_warmup.is_reliable( &reliable ) );
    assert!( !validator_no_warmup.is_reliable( &unreliable ) );
  }

  #[ test ]
  fn test_validation_report_generation()
  {
    let validator = BenchmarkValidator::new();
    
    let mut results = HashMap::new();
    results.insert( "good".to_string(), create_reliable_result() );
    results.insert( "bad".to_string(), create_unreliable_result() );
    
    let report = validator.generate_validation_report( &results );
    
    // Check report structure
    assert!( report.contains( "# Benchmark Validation Report" ) );
    assert!( report.contains( "## Summary" ) );
    assert!( report.contains( "**Total benchmarks**: 2" ) );
    assert!( report.contains( "## Recommendations" ) );
    assert!( report.contains( "## Validation Criteria" ) );
    
    // Should contain benchmark names
    assert!( report.contains( "good" ) );
    assert!( report.contains( "bad" ) );
  }

  #[ test ]
  fn test_validated_results_creation()
  {
    let validator = BenchmarkValidator::new();
    
    let mut results = HashMap::new();
    results.insert( "test1".to_string(), create_reliable_result() );
    results.insert( "test2".to_string(), create_unreliable_result() );
    
    let validated = ValidatedResults::new( results, validator );
    
    assert_eq!( validated.results.len(), 2 );
    assert_eq!( validated.warnings.len(), 2 );
    assert!( !validated.all_reliable() );
    assert!( validated.reliable_count() <= 1 ); // At most 1 reliable (warmup may cause issues)
    assert!( validated.reliability_rate() <= 50.0 );
  }

  #[ test ]
  fn test_validated_results_warnings()
  {
    let validator = BenchmarkValidator::new();
    
    let mut results = HashMap::new();
    results.insert( "unreliable".to_string(), create_unreliable_result() );
    
    let validated = ValidatedResults::new( results, validator );
    
    let warnings = validated.reliability_warnings();
    assert!( warnings.is_some() );
    
    let warning_list = warnings.unwrap();
    assert!( !warning_list.is_empty() );
    assert!( warning_list[ 0 ].contains( "unreliable:" ) );
  }

  #[ test ]
  fn test_validated_results_reliable_subset()
  {
    let validator = BenchmarkValidator::new().require_warmup( false );
    
    let mut results = HashMap::new();
    results.insert( "good".to_string(), create_reliable_result() );
    results.insert( "bad".to_string(), create_unreliable_result() );
    
    let validated = ValidatedResults::new( results, validator );
    let reliable_only = validated.reliable_results();
    
    // Should only contain the reliable result
    assert!( reliable_only.len() <= 1 );
    if reliable_only.len() == 1
    {
      assert!( reliable_only.contains_key( "good" ) );
      assert!( !reliable_only.contains_key( "bad" ) );
    }
  }

  #[ test ]
  fn test_validation_warning_display()
  {
    let warning1 = ValidationWarning::InsufficientSamples { actual : 5, minimum : 10 };
    let warning2 = ValidationWarning::HighVariability { actual : 0.15, maximum : 0.1 };
    let warning3 = ValidationWarning::NoWarmup;
    let warning4 = ValidationWarning::WidePerformanceRange { ratio : 4.5 };
    let warning5 = ValidationWarning::ShortMeasurementTime { duration : Duration::from_nanos( 50 ) };
    
    assert!( warning1.to_string().contains( "Insufficient samples" ) );
    assert!( warning2.to_string().contains( "High variability" ) );
    assert!( warning3.to_string().contains( "No warmup" ) );
    assert!( warning4.to_string().contains( "Wide performance range" ) );
    assert!( warning5.to_string().contains( "Short measurement time" ) );
  }

  #[ test ]
  fn test_validated_results_report()
  {
    let validator = BenchmarkValidator::new();
    
    let mut results = HashMap::new();
    results.insert( "test".to_string(), create_unreliable_result() );
    
    let validated = ValidatedResults::new( results, validator );
    let report = validated.validation_report();
    
    assert!( report.contains( "# Benchmark Validation Report" ) );
    assert!( report.contains( "test" ) );
  }
}