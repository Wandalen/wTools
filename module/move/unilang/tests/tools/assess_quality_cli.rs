//! CLI tool for test quality assessment
//!
//! This executable provides command-line access to the quality assessment
//! functionality with various output formats and filtering options.

use std::env;
use std::path::Path;
use std::process;
use std::fs;

mod quality_assessor;
use quality_assessor::{ QualityAssessor, AssessmentConfig, QualityReport };

#[ derive( Debug ) ]
struct CliConfig
{
  tests_dir : String,
  output_format : OutputFormat,
  output_file : Option< String >,
  verbose : bool,
  fail_threshold : f64,
  config_file : Option< String >,
  filter_category : Option< String >,
  generate_recommendations : bool,
}

#[ derive( Debug ) ]
enum OutputFormat
{
  Json,
  Html,
  Markdown,
  Text,
}

impl Default for CliConfig
{
  fn default() -> Self
  {
    Self {
      tests_dir : "tests".to_string(),
      output_format : OutputFormat::Text,
      output_file : None,
      verbose : false,
      fail_threshold : 90.0,
      config_file : None,
      filter_category : None,
      generate_recommendations : true,
    }
  }
}

fn main()
{
  let config = parse_args();

  if config.verbose
  {
    println!( "🔍 Starting test quality assessment..." );
    println!( "📁 Tests directory: {}", config.tests_dir );
    println!( "📊 Output format: {:?}", config.output_format );
  }

  let tests_path = Path::new( &config.tests_dir );
  if !tests_path.exists()
  {
    eprintln!( "❌ Error: Tests directory '{}' does not exist", config.tests_dir );
    process::exit( 1 );
  }

  // Load custom config if specified
  let assessment_config = if let Some( config_file ) = &config.config_file
  {
    load_assessment_config( config_file ).unwrap_or_else( |e| {
      eprintln!( "⚠️ Warning: Could not load config file '{}': {}", config_file, e );
      eprintln!( "Using default configuration." );
      AssessmentConfig::default()
    })
  }
  else
  {
    AssessmentConfig::default()
  };

  // Create assessor and run assessment
  let assessor = QualityAssessor::with_config( tests_path, assessment_config );

  match assessor.assess_quality()
  {
    Ok( report ) => {
      // Output report
      output_report( &report, &config );

      // Check quality gate
      let passed = check_quality_gate( &report, config.fail_threshold );

      if config.verbose
      {
        println!( "\n📋 Assessment Summary:" );
        println!( "   Overall Score: {:.1}%", report.overall_score );
        println!( "   Quality Grade: {:?}", report.summary.grade );
        println!( "   Recommendations: {}", report.recommendations.len() );
      }

      if passed
      {
        if config.verbose
        {
          println!( "\n✅ Quality gate passed!" );
        }
        process::exit( 0 );
      }
      else
      {
        if config.verbose
        {
          println!( "\n❌ Quality gate failed!" );
          println!( "   Minimum score: {:.1}%", config.fail_threshold );
          println!( "   Actual score: {:.1}%", report.overall_score );
        }
        process::exit( 1 );
      }
    }
    Err( error ) => {
      eprintln!( "❌ Error during quality assessment: {}", error );
      process::exit( 1 );
    }
  }
}

fn parse_args() -> CliConfig
{
  let args : Vec< String > = env::args().collect();
  let mut config = CliConfig::default();

  let mut i = 1;
  while i < args.len()
  {
    match args[ i ].as_str()
    {
      "--tests-dir" | "-d" => {
        if i + 1 < args.len()
        {
          config.tests_dir = args[ i + 1 ].clone();
          i += 1;
        }
        else
        {
          eprintln!( "Error: --tests-dir requires a value" );
          process::exit( 1 );
        }
      }
      "--format" | "-f" => {
        if i + 1 < args.len()
        {
          config.output_format = match args[ i + 1 ].as_str()
          {
            "json" => OutputFormat::Json,
            "html" => OutputFormat::Html,
            "markdown" | "md" => OutputFormat::Markdown,
            "text" => OutputFormat::Text,
            _ => {
              eprintln!( "Error: Invalid format '{}'. Supported: json, html, markdown, text", args[ i + 1 ] );
              process::exit( 1 );
            }
          };
          i += 1;
        }
        else
        {
          eprintln!( "Error: --format requires a value" );
          process::exit( 1 );
        }
      }
      "--output" | "-o" => {
        if i + 1 < args.len()
        {
          config.output_file = Some( args[ i + 1 ].clone() );
          i += 1;
        }
        else
        {
          eprintln!( "Error: --output requires a value" );
          process::exit( 1 );
        }
      }
      "--verbose" | "-v" => {
        config.verbose = true;
      }
      "--fail-threshold" | "-t" => {
        if i + 1 < args.len()
        {
          config.fail_threshold = args[ i + 1 ].parse().unwrap_or_else( |_| {
            eprintln!( "Error: Invalid threshold value '{}'", args[ i + 1 ] );
            process::exit( 1 );
          });
          i += 1;
        }
        else
        {
          eprintln!( "Error: --fail-threshold requires a value" );
          process::exit( 1 );
        }
      }
      "--config" | "-c" => {
        if i + 1 < args.len()
        {
          config.config_file = Some( args[ i + 1 ].clone() );
          i += 1;
        }
        else
        {
          eprintln!( "Error: --config requires a value" );
          process::exit( 1 );
        }
      }
      "--filter" => {
        if i + 1 < args.len()
        {
          config.filter_category = Some( args[ i + 1 ].clone() );
          i += 1;
        }
        else
        {
          eprintln!( "Error: --filter requires a value" );
          process::exit( 1 );
        }
      }
      "--no-recommendations" => {
        config.generate_recommendations = false;
      }
      "--help" | "-h" => {
        print_help();
        process::exit( 0 );
      }
      _ => {
        eprintln!( "Error: Unknown argument '{}'", args[ i ] );
        eprintln!( "Use --help for usage information" );
        process::exit( 1 );
      }
    }
    i += 1;
  }

  config
}

fn print_help()
{
  println!( r#"
Test Quality Assessment Tool

USAGE:
    assess_quality_cli [OPTIONS]

OPTIONS:
    -d, --tests-dir <DIR>         Tests directory to analyze [default: tests]
    -f, --format <FORMAT>         Output format: json, html, markdown, text [default: text]
    -o, --output <FILE>           Output file (stdout if not specified)
    -v, --verbose                 Verbose output
    -t, --fail-threshold <SCORE>  Fail if score below threshold [default: 90.0]
    -c, --config <FILE>           Load custom assessment configuration
        --filter <CATEGORY>       Filter by category: coverage, organization, performance, maintainability
        --no-recommendations      Skip generating recommendations
    -h, --help                    Print this help message

EXAMPLES:
    assess_quality_cli                              # Basic assessment with text output
    assess_quality_cli -f json -o report.json      # JSON report to file
    assess_quality_cli -v -t 95.0                  # Verbose with 95% threshold
    assess_quality_cli --filter coverage           # Focus on coverage metrics
    assess_quality_cli -f html -o report.html      # HTML dashboard report

QUALITY THRESHOLDS:
    95-100%  Excellent - Exceptional test quality
    85-94%   Good      - High quality with minor improvements needed
    70-84%   Fair      - Adequate quality with room for improvement
    50-69%   Poor      - Significant quality issues need attention
    <50%     Critical  - Major quality problems require immediate action
"# );
}

fn load_assessment_config( config_file : &str ) -> Result< AssessmentConfig, Box< dyn std::error::Error > >
{
  let content = fs::read_to_string( config_file )?;
  let config : serde_json::Value = serde_json::from_str( &content )?;

  Ok( AssessmentConfig {
    target_line_coverage : config.get( "target_line_coverage" )
      .and_then( |v| v.as_f64() )
      .unwrap_or( 95.0 ),
    target_function_coverage : config.get( "target_function_coverage" )
      .and_then( |v| v.as_f64() )
      .unwrap_or( 98.0 ),
    target_structure_compliance : config.get( "target_structure_compliance" )
      .and_then( |v| v.as_f64() )
      .unwrap_or( 98.0 ),
    target_naming_compliance : config.get( "target_naming_compliance" )
      .and_then( |v| v.as_f64() )
      .unwrap_or( 98.0 ),
    max_test_duration_ms : config.get( "max_test_duration_ms" )
      .and_then( |v| v.as_f64() )
      .unwrap_or( 100.0 ),
    max_function_length : config.get( "max_function_length" )
      .and_then( |v| v.as_u64() )
      .map( |v| v as usize )
      .unwrap_or( 50 ),
    min_documentation_coverage : config.get( "min_documentation_coverage" )
      .and_then( |v| v.as_f64() )
      .unwrap_or( 85.0 ),
  })
}

fn output_report( report : &QualityReport, config : &CliConfig )
{
  let content = match config.output_format
  {
    OutputFormat::Json => format_json_report( report ),
    OutputFormat::Html => format_html_report( report ),
    OutputFormat::Markdown => format_markdown_report( report ),
    OutputFormat::Text => format_text_report( report, config ),
  };

  if let Some( output_file ) = &config.output_file
  {
    if let Err( e ) = fs::write( output_file, &content )
    {
      eprintln!( "❌ Error writing to file '{}': {}", output_file, e );
      process::exit( 1 );
    }

    if config.verbose
    {
      println!( "📝 Report written to: {}", output_file );
    }
  }
  else
  {
    print!( "{}", content );
  }
}

fn format_json_report( report : &QualityReport ) -> String
{
  serde_json::to_string_pretty( report ).unwrap_or_else( |_| "Error formatting JSON".to_string() )
}

fn format_html_report( report : &QualityReport ) -> String
{
  format!(
    r#"<!DOCTYPE html>
<html>
<head>
    <title>Test Quality Report</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 40px; }}
        .score {{ font-size: 2em; font-weight: bold; color: {}; }}
        .metric {{ margin: 10px 0; padding: 10px; border-left: 4px solid #ddd; }}
        .critical {{ border-left-color: #ff4444; }}
        .warning {{ border-left-color: #ffaa00; }}
        .good {{ border-left-color: #44ff44; }}
        .recommendations {{ margin-top: 30px; }}
        .recommendation {{ margin: 10px 0; padding: 15px; background: #f5f5f5; border-radius: 5px; }}
    </style>
</head>
<body>
    <h1>🧪 Test Quality Assessment Report</h1>

    <div class="score">Overall Score: {:.1}%</div>
    <div class="grade">Grade: {:?}</div>

    <h2>📊 Metrics Summary</h2>

    <div class="metric {}">
        <h3>Coverage Metrics</h3>
        <p>Line Coverage: {:.1}%</p>
        <p>Function Coverage: {:.1}%</p>
        <p>Test Count: {}</p>
    </div>

    <div class="metric {}">
        <h3>Organization Metrics</h3>
        <p>Structure Compliance: {:.1}%</p>
        <p>Naming Compliance: {:.1}%</p>
        <p>Total Files: {}</p>
    </div>

    <div class="metric {}">
        <h3>Performance Metrics</h3>
        <p>Average Test Duration: {:.1}ms</p>
        <p>Total Test Time: {:.1}s</p>
        <p>Slow Tests: {}</p>
    </div>

    <div class="recommendations">
        <h2>💡 Recommendations ({})</h2>
        {}
    </div>

    <footer>
        <p><small>Generated: {}</small></p>
    </footer>
</body>
</html>"#,
    if report.overall_score >= 90.0 { "#44ff44" } else if report.overall_score >= 70.0 { "#ffaa00" } else { "#ff4444" },
    report.overall_score,
    report.summary.grade,
    if report.coverage_metrics.score >= 90.0 { "good" } else if report.coverage_metrics.score >= 70.0 { "warning" } else { "critical" },
    report.coverage_metrics.line_coverage,
    report.coverage_metrics.function_coverage,
    report.coverage_metrics.test_count,
    if report.organization_metrics.score >= 90.0 { "good" } else if report.organization_metrics.score >= 70.0 { "warning" } else { "critical" },
    report.organization_metrics.structure_compliance,
    report.organization_metrics.naming_compliance,
    report.organization_metrics.total_files,
    if report.performance_metrics.score >= 90.0 { "good" } else if report.performance_metrics.score >= 70.0 { "warning" } else { "critical" },
    report.performance_metrics.avg_test_duration,
    report.performance_metrics.total_test_time,
    report.performance_metrics.slow_tests.len(),
    report.recommendations.len(),
    report.recommendations.iter()
      .map( |r| format!(
        r#"<div class="recommendation">
            <strong>{:?} - {}</strong><br>
            <em>Impact:</em> {}<br>
            <em>Solution:</em> {}
        </div>"#,
        r.priority, r.issue, r.impact, r.solution
      ))
      .collect::< Vec< _ > >()
      .join( "\n" ),
    report.timestamp
  )
}

fn format_markdown_report( report : &QualityReport ) -> String
{
  let grade_emoji = match report.summary.grade
  {
    quality_assessor::QualityGrade::Excellent => "🌟",
    quality_assessor::QualityGrade::Good => "✅",
    quality_assessor::QualityGrade::Fair => "⚠️",
    quality_assessor::QualityGrade::Poor => "❌",
    quality_assessor::QualityGrade::Critical => "🚨",
  };

  format!(
    r#"# 🧪 Test Quality Assessment Report

## Overall Results

**Score**: {:.1}% {grade_emoji}
**Grade**: {:?}
**Generated**: {}

## 📊 Detailed Metrics

### Coverage Metrics
- **Line Coverage**: {:.1}%
- **Function Coverage**: {:.1}%
- **Test Count**: {}
- **Score**: {:.1}%

### Organization Metrics
- **Structure Compliance**: {:.1}%
- **Naming Compliance**: {:.1}%
- **Distribution Balance**: {:.1}%
- **Total Files**: {}
- **Score**: {:.1}%

### Performance Metrics
- **Average Test Duration**: {:.1}ms
- **Total Test Time**: {:.1}s
- **Slow Tests**: {}
- **Flaky Tests**: {}
- **Score**: {:.1}%

### Maintainability Metrics
- **Average Function Length**: {:.1} lines
- **Documentation Coverage**: {:.1}%
- **Code Duplication**: {:.1}%
- **Complexity Score**: {:.1}%
- **Score**: {:.1}%

## 💡 Recommendations ({})

{}

## 📈 Summary

### Strengths
{}

### Areas for Improvement
{}

---
*Report generated by Test Quality Assessment Tool*
"#,
    report.overall_score,
    report.summary.grade,
    report.timestamp,
    report.coverage_metrics.line_coverage,
    report.coverage_metrics.function_coverage,
    report.coverage_metrics.test_count,
    report.coverage_metrics.score,
    report.organization_metrics.structure_compliance,
    report.organization_metrics.naming_compliance,
    report.organization_metrics.distribution_balance,
    report.organization_metrics.total_files,
    report.organization_metrics.score,
    report.performance_metrics.avg_test_duration,
    report.performance_metrics.total_test_time,
    report.performance_metrics.slow_tests.len(),
    report.performance_metrics.flaky_tests.len(),
    report.performance_metrics.score,
    report.maintainability_metrics.avg_function_length,
    report.maintainability_metrics.documentation_coverage,
    report.maintainability_metrics.duplication_ratio,
    report.maintainability_metrics.complexity_score,
    report.maintainability_metrics.score,
    report.recommendations.len(),
    report.recommendations.iter()
      .map( |r| format!(
        "### {:?} Priority: {}\n**Impact**: {}\n**Solution**: {}\n",
        r.priority, r.issue, r.impact, r.solution
      ))
      .collect::< Vec< _ > >()
      .join( "\n" ),
    report.summary.strengths.iter()
      .map( |s| format!( "- {}", s ) )
      .collect::< Vec< _ > >()
      .join( "\n" ),
    report.summary.areas_for_improvement.iter()
      .map( |s| format!( "- {}", s ) )
      .collect::< Vec< _ > >()
      .join( "\n" ),
    grade_emoji = grade_emoji
  )
}

fn format_text_report( report : &QualityReport, config : &CliConfig ) -> String
{
  let mut output = String::new();

  output.push_str( "🧪 Test Quality Assessment Report\n" );
  output.push_str( "================================\n\n" );

  output.push_str( &format!( "Overall Score: {:.1}%\n", report.overall_score ) );
  output.push_str( &format!( "Quality Grade: {:?}\n", report.summary.grade ) );
  output.push_str( &format!( "Generated: {}\n\n", report.timestamp ) );

  if config.filter_category.is_none() || config.filter_category.as_ref().unwrap() == "coverage"
  {
    output.push_str( "📊 Coverage Metrics\n" );
    output.push_str( &format!( "  Line Coverage:     {:.1}%\n", report.coverage_metrics.line_coverage ) );
    output.push_str( &format!( "  Function Coverage: {:.1}%\n", report.coverage_metrics.function_coverage ) );
    output.push_str( &format!( "  Test Count:        {}\n", report.coverage_metrics.test_count ) );
    output.push_str( &format!( "  Score:             {:.1}%\n\n", report.coverage_metrics.score ) );
  }

  if config.filter_category.is_none() || config.filter_category.as_ref().unwrap() == "organization"
  {
    output.push_str( "🏗️ Organization Metrics\n" );
    output.push_str( &format!( "  Structure Compliance: {:.1}%\n", report.organization_metrics.structure_compliance ) );
    output.push_str( &format!( "  Naming Compliance:    {:.1}%\n", report.organization_metrics.naming_compliance ) );
    output.push_str( &format!( "  Distribution Balance: {:.1}%\n", report.organization_metrics.distribution_balance ) );
    output.push_str( &format!( "  Total Files:          {}\n", report.organization_metrics.total_files ) );
    output.push_str( &format!( "  Score:                {:.1}%\n\n", report.organization_metrics.score ) );
  }

  if config.filter_category.is_none() || config.filter_category.as_ref().unwrap() == "performance"
  {
    output.push_str( "⚡ Performance Metrics\n" );
    output.push_str( &format!( "  Avg Test Duration: {:.1}ms\n", report.performance_metrics.avg_test_duration ) );
    output.push_str( &format!( "  Total Test Time:   {:.1}s\n", report.performance_metrics.total_test_time ) );
    output.push_str( &format!( "  Slow Tests:        {}\n", report.performance_metrics.slow_tests.len() ) );
    output.push_str( &format!( "  Flaky Tests:       {}\n", report.performance_metrics.flaky_tests.len() ) );
    output.push_str( &format!( "  Score:             {:.1}%\n\n", report.performance_metrics.score ) );
  }

  if config.filter_category.is_none() || config.filter_category.as_ref().unwrap() == "maintainability"
  {
    output.push_str( "🔧 Maintainability Metrics\n" );
    output.push_str( &format!( "  Avg Function Length:    {:.1} lines\n", report.maintainability_metrics.avg_function_length ) );
    output.push_str( &format!( "  Documentation Coverage: {:.1}%\n", report.maintainability_metrics.documentation_coverage ) );
    output.push_str( &format!( "  Code Duplication:       {:.1}%\n", report.maintainability_metrics.duplication_ratio ) );
    output.push_str( &format!( "  Complexity Score:       {:.1}%\n", report.maintainability_metrics.complexity_score ) );
    output.push_str( &format!( "  Score:                  {:.1}%\n\n", report.maintainability_metrics.score ) );
  }

  if config.generate_recommendations && !report.recommendations.is_empty()
  {
    output.push_str( &format!( "💡 Recommendations ({})\n", report.recommendations.len() ) );
    for rec in &report.recommendations
    {
      output.push_str( &format!( "\n{:?} Priority: {}\n", rec.priority, rec.issue ) );
      output.push_str( &format!( "  Impact:   {}\n", rec.impact ) );
      output.push_str( &format!( "  Solution: {}\n", rec.solution ) );
      if !rec.files_affected.is_empty()
      {
        output.push_str( &format!( "  Files:    {}\n", rec.files_affected.join( ", " ) ) );
      }
    }
    output.push_str( "\n" );
  }

  output.push_str( "📈 Summary\n" );
  output.push_str( &format!( "Grade: {:?}\n", report.summary.grade ) );

  if !report.summary.strengths.is_empty()
  {
    output.push_str( "\nStrengths:\n" );
    for strength in &report.summary.strengths
    {
      output.push_str( &format!( "  + {}\n", strength ) );
    }
  }

  if !report.summary.areas_for_improvement.is_empty()
  {
    output.push_str( "\nAreas for Improvement:\n" );
    for area in &report.summary.areas_for_improvement
    {
      output.push_str( &format!( "  - {}\n", area ) );
    }
  }

  output
}

fn check_quality_gate( report : &QualityReport, threshold : f64 ) -> bool
{
  report.overall_score >= threshold
}