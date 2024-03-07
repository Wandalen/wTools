mod private
{

  use crate::*;
  use std::
  {
    collections::{ BTreeMap, BTreeSet, HashSet },
    fmt::Formatter,
    sync::{ Arc, Mutex },
    path::Path,
  };
  use cargo_metadata::Package;
  use colored::Colorize;
  use rayon::ThreadPoolBuilder;
  use process::CmdReport;
  use wtools::error::anyhow::{ Error, format_err };
  use wtools::iter::Itertools;
  use wtools::error::Result;
  use former::Former;
  use channel::Channel;

  /// Represents the arguments for the test.
  #[ derive( Debug, Former, Clone ) ]
  pub struct SingleTestOptions
  {
    /// Specifies the release channels for rust.
    channel : Channel,
    /// Determines whether to use default features in the test.
    /// Enabled by default.
    #[ default( true ) ]
    with_default_features : bool,
    /// Determines whether to use all available features in the test.
    /// Disabled by default.
    #[ default( false ) ]
    with_all_features : bool,
    /// Specifies a list of features to be enabled in the test.
    enable_features : BTreeSet< String >,
  }

  impl SingleTestOptions
  {
    fn as_rustup_args(&self ) -> Vec< String >
    {
      [ "run".into(), self.channel.to_string(), "cargo".into(), "test".into() ]
      .into_iter()
      .chain( if self.with_default_features { None } else { Some( "--no-default-features".into() ) } )
      .chain( if self.with_all_features { Some( "--all-features".into() ) } else { None } )
      .chain( if self.enable_features.is_empty() { None } else { Some([ "--features".into(), self.enable_features.iter().join( "," ) ]) }.into_iter().flatten() )
      .collect()
    }
  }

  /// Executes a test command with the given arguments.
  ///
  /// # Arguments
  ///
  /// * `path` - The path to the test command.
  /// * `options` - The options for the test command.
  /// * `dry` - A boolean indicating whether to perform a dry run or not.
  ///
  /// # Returns
  ///
  /// Returns a `Result` containing a `CmdReport` if the command is executed successfully,
  /// or an error if the command fails to execute.
  pub fn _run< P >( path : P, options : SingleTestOptions, dry : bool ) -> Result< CmdReport >
  where
    P : AsRef< Path >
  {
    let ( program, options ) = ( "rustup", options.as_rustup_args() );

    if dry
    {
      Ok
      (
        CmdReport
        {
          command : format!( "{program} {}", options.join( " " ) ),
          path : path.as_ref().to_path_buf(),
          out : String::new(),
          err : String::new(),
        }
      )
    }
    else
    {
      process::process_run_with_param_and_joined_steams(program, options, path )
    }
  }

  /// `TestOptions` is a structure used to store the arguments for tests.
  #[ derive( Debug ) ]
  pub struct TestOptions
  {
    /// `channels` - A set of Cargo channels that are to be tested.
    pub channels : HashSet< Channel >,

    /// `concurrent` - A usize value indicating how much test`s can be run at the same time.
    pub concurrent : u32,

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
    /// A `BTreeMap` where the keys are `channel::Channel` enums representing the channels
    ///   for which the tests were run, and the values are nested `BTreeMap` where the keys are
    ///   feature names and the values are `CmdReport` structs representing the test results for
    ///   the specific feature and channel.
    pub tests : BTreeMap< channel::Channel, BTreeMap< String, CmdReport > >,
  }

  impl std::fmt::Display for TestReport
  {
    fn fmt( &self, f : &mut Formatter< '_ > ) -> std::fmt::Result
    {
      if self.dry
      {
        return Ok( () )
      }
      let mut failed = 0;
      let mut success = 0;
      writeln!(f, "{} {}\n", "\n=== Module".bold(), self.package_name.bold() )?;
      if self.tests.is_empty()
      {
        writeln!( f, "unlucky" )?;
        return Ok( () );
      }

      for ( channel, features ) in self.tests.iter().sorted_by( | a, b | a.0.cmp( b.0 ) )
      {
        for ( feature, result ) in features
        {
          // if tests failed or if build failed
          if result.out.contains( "failures" ) || result.out.contains( "error" )
          {
            let mut out = result.out.replace( "\n", "\n      " );
            out.push_str( "\n" );
            failed += 1;
            write!( f, "  [ {} | {} ]: ❌  failed\n  \n{out}", channel, feature )?;
          }
          else
          {
            let feature = if feature.is_empty() { "no-features" } else { feature };
            success += 1;
            writeln!( f, "  [ {} | {} ]: ✅  successful", channel, feature )?;
          }
        }
      }
      if success == failed + success
      {
        writeln!( f, "  ✅  All passed {success} / {}", failed + success )?;
      }
      else
      {
        writeln!( f, "  ❌  Not all passed {success} / {}", failed + success )?;
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
      if self.dry
      {
        writeln!( f, "\nYou can execute the command with the dry-run :0, for example 'will .test dry : 0'." )?;
        return Ok( () )
      }
      if self.succses_reports.is_empty() && self.failure_reports.is_empty()
      {
        writeln!( f, "The tests have not been run."  )?;
        return Ok( () );
      }
      if !self.succses_reports.is_empty()
      {
        writeln!( f, "Successful :" )?;
        for report in &self.succses_reports
        {
          writeln!( f, "{}", report )?;
        }
      }
      if !self.failure_reports.is_empty()
      {
        writeln!( f, "Failure :" )?;
        for report in &self.failure_reports
        {
          writeln!( f, "{}", report )?;
        }
      }
      writeln!( f, "Global report" )?;
      if self.succses_reports.len() == self.failure_reports.len() + self.succses_reports.len()
      {
        writeln!( f, "  ✅  All passed {} / {}", self.succses_reports.len(),  self.succses_reports.len() )?;
      }
      else
      {
        writeln!( f, "  ❌  Not all passed {} / {}", self.succses_reports.len(),  self.failure_reports.len() + self.succses_reports.len() )?;
      }

      Ok( () )
    }
  }

  /// `tests_run` is a function that runs tests on a given package with specified arguments.
  /// It returns a `TestReport` on success, or a `TestReport` and an `Error` on failure.
  pub fn run( args : &TestOptions, package : &Package, dry : bool ) -> Result< TestReport, ( TestReport, Error ) >
  {
    // let exclude = args.exclude_features.iter().cloned().collect();
    let mut report = TestReport::default();
    report.dry = dry;
    report.package_name = package.name.clone();
    let report = Arc::new( Mutex::new( report ) );

    let features_powerset = features::features_powerset
    (
      package,
      args.power as usize,
      &args.exclude_features,
      &args.include_features
    );

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
                // qqq : for Petro : bad. tooooo long line. cap on 100 ch
                let cmd_rep = _run( dir, SingleTestOptions::former().channel( channel ).with_default_features( false ).enable_features( feature.clone() ).form(), dry ).unwrap_or_else( | rep | rep.downcast().unwrap() );
                r.lock().unwrap().tests.entry( channel ).or_default().insert( feature.iter().join( "," ), cmd_rep );
              }
            );
          }
        }
      }
    );

    // unpack. all tasks must be completed until now
    let report = Mutex::into_inner( Arc::into_inner( report ).unwrap() ).unwrap();
    let at_least_one_failed = report.tests.iter().flat_map( | ( _, v ) | v.iter().map( | ( _, v ) | v ) ).any( | r | r.out.contains( "failures" ) || r.out.contains( "error" ) );
    if at_least_one_failed { Err( ( report, format_err!( "Some tests was failed" ) ) ) } else { Ok( report ) }
  }

  /// Run tests for given packages.
  pub fn tests_run( args : &TestOptions, packages : &[ Package ], dry : bool ) -> Result< TestsReport, ( TestsReport, Error ) >
  {
    let mut report = TestsReport::default();
    report.dry = dry;
    let report = Arc::new( Mutex::new( report ) );
    let pool = ThreadPoolBuilder::new().use_current_thread().num_threads( args.concurrent as usize ).build().unwrap();
    pool.scope
    (
      | s |
      {
        for package in packages
        {
          let report = report.clone();
          s.spawn
          (
            move | _ |
            {
              match run( &args, package, dry )
              {
                Ok( r ) =>
                {
                  report.lock().unwrap().succses_reports.push( r );
                }
                Err(( r, _ )) =>
                {
                  report.lock().unwrap().failure_reports.push( r );
                }
              }
            }
          );
        }
      }
    );
    let report = Arc::into_inner( report ).unwrap().into_inner().unwrap();
    if report.failure_reports.is_empty()
    {
      Ok( report )
    }
    else
    {
      Err(( report, format_err!( "Some tests was failed" ) ))
    }
  }

  fn print_temp_report( package_name : &str, channels : &HashSet< channel::Channel >, features : &HashSet< BTreeSet< String > > )
  {
    println!( "Package : {}\nThe tests will be executed using the following configurations :", package_name );
    for channel in channels.iter().sorted()
    {
      for feature in features
      {
        let feature = if feature.is_empty() { "no-features".to_string() } else { feature.iter().join( "," ) };
        println!( "  [ channel : {channel} | feature : {feature} ]" );
      }
    }
  }
}

crate::mod_interface!
{

  protected use SingleTestOptions;
  protected use _run;

  protected use TestOptions;
  protected use TestReport;
  protected use TestsReport;
  protected use run;
  protected use tests_run;
}