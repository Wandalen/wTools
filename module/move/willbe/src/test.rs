mod private
{
  
  use crate::*;
  use std::collections::{ BTreeMap, BTreeSet, HashSet };
  use std::fmt::Formatter;
  use std::sync::{ Arc, Mutex };
  use cargo_metadata::Package;
  use rayon::ThreadPoolBuilder;
  use crate::process::CmdReport;
  use crate::wtools::error::anyhow::{ Error, format_err };
  use crate::wtools::iter::Itertools;

  /// `TestsArgs` is a structure used to store the arguments for tests.
  #[ derive( Debug ) ]
  pub struct TestArgs
  {
    /// `channels` - A set of Cargo channels that are to be tested.
    pub channels : HashSet< cargo::Channel >,

    /// `parallel` - A boolean value indicating whether the tests should be run in parallel.
    pub parallel : bool,

    /// `power` - An integer value indicating the power or intensity of testing.
    pub power : u32,

    /// `include_features` - A vector of strings, each representing a feature to be included during testing.
    pub include_features : Vec< String >,

    /// `exclude_features` - A vector of strings, each representing a feature to be excluded during testing.
    pub exclude_features : Vec< String >,
  }


  /// Represents a report of test results.
  #[ derive( Debug, Default, Clone ) ]
  pub struct TestReport
  {
    /// A boolean flag indicating whether or not the code is being run in dry mode.
    ///
    /// Dry mode is a mode in which the code performs a dry run, simulating the execution
    /// of certain tasks without actually making any changes. When the `dry` flag is set to
    /// `true`, the code will not perform any actual actions, but instead only output the
    /// results it would have produced.
    ///
    /// This flag can be useful for testing and debugging purposes, as well as for situations
    /// where it is important to verify the correctness of the actions being performed before
    /// actually executing them.
    pub dry : bool,
    /// A string containing the name of the package being tested.
    pub package_name : String,
    /// A `BTreeMap` where the keys are `cargo::Channel` enums representing the channels
    ///   for which the tests were run, and the values are nested `BTreeMap` where the keys are
    ///   feature names and the values are `CmdReport` structs representing the test results for
    ///   the specific feature and channel.
    pub tests : BTreeMap< cargo::Channel, BTreeMap< String, CmdReport > >,
  }

  impl std::fmt::Display for TestReport
  {
    fn fmt( &self, f : &mut Formatter< '_ > ) -> std::fmt::Result
    {
      writeln!( f, "The tests will be executed using the following configurations:" )?;
      for ( channel, feature ) in self.tests.iter().flat_map( | ( c, f ) | f.iter().map ( |( f, _ )| ( *c, f ) ) )
      {
        writeln!( f, "channel : {channel} | features : [ {} ]", if feature.is_empty() { "no-features" } else { feature } )?;
      }
      writeln!( f, "\nModule: {} :", self.package_name )?;
      if self.tests.is_empty()
      {
        writeln!( f, "unlucky" )?;
        return Ok( () );
      }

      for ( channel, features ) in &self.tests
      {
        for ( feature, result ) in features
        {
          if self.dry
          {
            let feature = if feature.is_empty() { "no-features" } else { feature };
            writeln!( f, "[{channel} | {feature}]: `{}`", result.command )?
          }
          else
          {
            // if tests failed or if build failed
            let failed = result.out.contains( "failures" ) || result.err.contains( "error" );
            if !failed
            {
              let feature = if feature.is_empty() { "no-features" } else { feature };
              writeln!( f, "  [ {} | {} ]: {}", channel, feature, if failed { "❌ failed" } else { "✅ successful" } )?;
            }
            else
            {
              let feature = if feature.is_empty() { "no-features" } else { feature };
              write!( f, "  Feature: [ {} | {} ]:\n  Tests status: {}\n{}\n{}", channel, feature, if failed { "❌ failed" } else { "✅ successful" }, result.out, result.err )?;
            }
          }
        }
      }

      Ok( () )
    }
  }

  /// Represents a vector of reposts
  #[ derive( Debug, Default, Clone ) ]
  pub struct TestsReport
  {
    /// A boolean flag indicating whether or not the code is being run in dry mode.
    ///
    /// Dry mode is a mode in which the code performs a dry run, simulating the execution
    /// of certain tasks without actually making any changes. When the `dry` flag is set to
    /// `true`, the code will not perform any actual actions, but instead only output the
    /// results it would have produced.
    ///
    /// This flag can be useful for testing and debugging purposes, as well as for situations
    /// where it is important to verify the correctness of the actions being performed before
    /// actually executing them.
    pub dry : bool,
    /// Vector of succses reports.
    pub succses_reports : Vec< TestReport >,
    /// Vector of failure reports.
    pub failure_reports : Vec< TestReport >,
  }

  impl std::fmt::Display for TestsReport
  {
    fn fmt( &self, f : &mut Formatter< '_ > ) -> std::fmt::Result
    {
      if self.succses_reports.is_empty() && self.failure_reports.is_empty()
      {
        writeln!( f, "The tests have not been run."  )?;
        return Ok( () );
      }
      if !self.succses_reports.is_empty()
      {
        writeln!( f, "Successful:" )?;
        for report in &self.succses_reports
        {
          writeln!( f, "{}", report )?;
        }
      }
      if !self.failure_reports.is_empty()
      {
        writeln!( f, "Failure:" )?;
        for report in &self.failure_reports
        {
          writeln!( f, "{}", report )?;
        }
      }
      Ok( () )
    }
  }
  
  /// `run_tests` is a function that runs tests on a given package with specified arguments.
  /// It returns a `TestReport` on success, or a `TestReport` and an `Error` on failure.
  pub fn run_test( args : &TestArgs, package : &Package, dry : bool ) -> Result< TestReport, ( TestReport, Error ) >
  {
    let exclude = args.exclude_features.iter().cloned().collect();
    let mut report = TestReport::default();
    report.package_name = package.name.clone();
    let report = Arc::new( Mutex::new( report ) );

    let features_powerset = package
    .features
    .keys()
    .filter( | f | !args.exclude_features.contains( f ) && !args.include_features.contains( f ) )
    .cloned()
    .powerset()
    .map( BTreeSet::from_iter )
    .filter( | subset | subset.len() <= args.power as usize )
    .map
    (
      | mut subset | 
      { 
        subset.extend( args.include_features.clone() );
        subset.difference( &exclude ).cloned().collect()
      }
    )
    .collect::< HashSet< BTreeSet< String > > >();
    print_temp_report( &package.name, &args.channels, &features_powerset );
    rayon::scope
    (
      | s | 
      { 
        let dir = package.manifest_path.parent().unwrap();
        for channel in args.channels.clone()
        { 
          for feature in &features_powerset 
          {
            let r = report.clone();
            s.spawn
            (
              move | _ | 
              { 
                let cmd_rep = cargo::test( dir, cargo::TestArgs::former().channel( channel ).with_default_features( false ).enable_features( feature.clone() ).form(), dry ).unwrap_or_else( | rep | rep.downcast().unwrap() );
                r.lock().unwrap().tests.entry( channel ).or_default().insert( feature.iter().join( "," ), cmd_rep );
              }
            );
          }
        }
      }
    );

    // unpack. all tasks must be completed until now
    let report = Mutex::into_inner( Arc::into_inner( report ).unwrap() ).unwrap();
    let at_least_one_failed = report.tests.iter().flat_map( | ( _, v ) | v.iter().map( | ( _, v ) | v ) ).any( | r | r.out.contains( "failures" ) || r.err.contains( "error" ) );
    if at_least_one_failed { Err( ( report, format_err!( "Some tests was failed" ) ) ) } else { Ok( report ) }
  }
  
  /// Run tests for given packages.
  pub fn run_tests( args : &TestArgs, packages : &[ Package ], dry : bool ) -> Result< TestsReport, ( TestsReport, Error ) >
  {
    let mut report = TestsReport::default();
    let mut pool = ThreadPoolBuilder::new().use_current_thread();
    pool = if args.parallel { pool } else { pool.num_threads( 1 ) };
    let pool = pool.build().unwrap();
    pool.scope
    (
      | _ |
      {
        for package in packages
        { 
          match run_test( &args, package, dry )
          { 
            Ok( r ) => 
            {
              report.succses_reports.push( r );
            }
            Err(( r, _ )) =>
            { 
              report.failure_reports.push( r );
            }
          }
        }
      }
    );
    if report.failure_reports.is_empty()
    {
      Ok( report )
    }
    else
    {
      Err(( report, format_err!( "Some tests was failed" ) ))
    }
  }

  fn print_temp_report( package_name : &str, channels : &HashSet< cargo::Channel >, features : &HashSet< BTreeSet< String > > )
  {
    println!( "Package : {}", package_name );
    for channel in channels
    {
      for feature in features
      {
        let feature = if feature.is_empty() { "no-features".to_string() } else { feature.iter().join( "," ) };
        println!( "[{channel} | {feature}]" );
      }
    }
  }
}

crate::mod_interface!
{
  protected use TestArgs;
  protected use TestReport;
  protected use TestsReport;
  protected use run_test;
  protected use run_tests;
}