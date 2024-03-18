mod private
{

  // qqq : for Petro : use https://github.com/console-rs/indicatif

  use crate::*;
  use std::
  {
    collections::{ BTreeMap, BTreeSet, HashSet },
    fmt::Formatter,
    sync::{ Arc, Mutex },
    path::Path,
  };
  use std::ffi::OsString;
  use std::path::PathBuf;
  use cargo_metadata::Package;
  // qqq : for Petro : don't use cargo_metadata directly, use facade
  use colored::Colorize;
  use rayon::ThreadPoolBuilder;
  use process::Report;
  use wtools::error::anyhow::{ Error, format_err };
  use wtools::iter::Itertools;
  use wtools::error::Result;
  use former::Former;
  use channel::Channel;
  use optimization::Optimization;

  pub struct TestPackagePlan
  {
    package : PathBuf,
    test_variants : BTreeSet< TestVariant >,
    temp_directory_path : Option< PathBuf >,
  }

  /// Represents a variant for testing purposes.
  #[ derive( Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Former ) ]
  pub struct TestVariant
  {
    /// Represents the channel for the test variant.
    channel : Channel,
    /// Represents the optimization setting for the test variant.
    optimization : Optimization,
    /// Contains additional features or characteristics of the test variant.
    features : String,
  }

  /// Represents the options for the test.
  #[ derive( Debug, Former, Clone ) ]
  pub struct SingleTestOptions
  {
    // qqq : for Petro : poor description
    /// Specifies the release channels for rust.
    channel : Channel,
    /// Specifies the optimization for rust.
    optimization : Optimization,
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
    /// Temp directory path
    temp_directory_path : Option< PathBuf >,
    // qqq : for Petro : why dry not here?
  }

  impl SingleTestOptions
  {
    fn as_rustup_args( &self ) -> Vec< String >
    {
      [ "run".into(), self.channel.to_string(), "cargo".into(), "test".into() ]
      .into_iter()
      .chain( if self.optimization == Optimization::Release { Some( "--release".into() ) } else { None } )
      .chain( if self.with_default_features { None } else { Some( "--no-default-features".into() ) } )
      // qqq : for Petro : bad, --no-default-features is always enabled!
      .chain( if self.with_all_features { Some( "--all-features".into() ) } else { None } )
      // qqq : for Petro : bad, --all-features is always disabled!
      .chain( if self.enable_features.is_empty() { None } else { Some([ "--features".into(), self.enable_features.iter().join( "," ) ]) }.into_iter().flatten() )
      .chain( self.temp_directory_path.clone().map( | p | vec![ "--target-dir".to_string(), p.to_string_lossy().into() ] ).into_iter().flatten() )
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
  /// Returns a `Result` containing a `Report` if the command is executed successfully,
  /// or an error if the command fails to execute.
  pub fn _run< P >( path : P, options : SingleTestOptions, dry : bool ) -> Result< Report, ( Report, Error ) >
  where
    P : AsRef< Path >
  {
    let ( program, args ) = ( "rustup", options.as_rustup_args() );
    // qqq : for Petro : rustup ???
    // qqq : for Petro : RUST_BACKTRACE=1 ?? //  add to SingleTestOptions, by default true

    if dry
    {
      Ok
      (
        Report
        {
          command : format!( "{program} {}", args.join( " " ) ),
          path : path.as_ref().to_path_buf(),
          out : String::new(),
          err : String::new(),
        }
      )
    }
    else
    {
      let options = process::Run::former()
      .application( program )
      .args( args.into_iter().map( OsString::from ).collect::< Vec< _ > >() )
      .path( path.as_ref().to_path_buf() )
      .joining_streams( true )
      .form();
      process::run( options )
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

    /// `temp_path` - path to temp directory.
    pub temp_path : Option< PathBuf >,

    ///  optimizations
    pub optimizations : HashSet< Optimization >,

    /// todo
    pub enabled_features : Vec< String >,

    /// todo
    pub with_all_features : bool,

    /// todo
    pub with_none_features : bool,

    /// todo
    pub variants_cap : u32,
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
    pub package_name : String, /* qqq : for Petro : bad, reuse newtype */
    /// A `BTreeMap` where the keys are `channel::Channel` enums representing the channels
    ///   for which the tests were run, and the values are nested `BTreeMap` where the keys are
    ///   feature names and the values are `Report` structs representing the test results for
    ///   the specific feature and channel.
    pub tests : BTreeMap< TestVariant, Result< Report, Report > > ,
    // qqq : for Petro : rid off map of map of map, keep flat map
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
      for ( variant, result) in &self.tests
      {
        let feature = if variant.features.is_empty() { "-" } else { &variant.features };
        // if tests failed or if build failed
        match result
        {
          Ok( _ ) =>
          {
            success += 1;
            writeln!( f, "  [ {} | {} | {} ]: ✅  successful", variant.optimization, variant.channel, feature )?;
          }
          Err( result) =>
          {
            let mut out = result.out.replace("\n", "\n      ");
            out.push_str("\n");
            failed += 1;
            write!( f, "  [ {} | {} | {} ]: ❌  failed\n  \n{out}", variant.optimization, variant.channel, feature )?;
          }
        }
      }
      // aaa : for Petro : bad, DRY
      // aaa : replace with method
      writeln!(f, "  {}", generate_summary_message(failed, success ) )?;

      Ok( () )
    }
  }


  fn generate_summary_message( failed : i32, success : i32 ) -> String
  {
    if success == failed + success
    {
      format!( "✅  All passed {success} / {}", failed + success )
    }
    else
    {
      format!( "❌  Not all passed {success} / {}", failed + success )
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
        writeln!( f, "\nYou can execute the plan with 'will .test dry : 0'." )?;
        // qqq : for Petro : bad. should be exact command with exact parameters / при виклику зовнішніх команд повинен бути вивід у консоль про цей виклик і його аргументи за виключенням коли ційлий блок виводу прихований (у моєму випадку при фейлі)
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
      writeln!( f, "  {}", generate_summary_message( self.failure_reports.len() as i32, self.succses_reports.len() as i32 ) )?;

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
    let features_powerset = features::features_powerset
      (
        package,
        args.power as usize,
        &args.exclude_features,
        &args.include_features,
        &args.enabled_features,
        args.with_all_features,
        args.with_none_features,
        args.variants_cap,
      ).map_err( | e | ( report.clone(), e.into() ) )?;
    let report = Arc::new( Mutex::new( report ) );

    print_temp_report( &package.name, &args.optimizations, &args.channels, &features_powerset );
    rayon::scope
    (
      | s |
      {
        let dir = package.manifest_path.parent().unwrap();
        // qqq : for Petro : bad, DRY
        for optimization in args.optimizations.clone()
        {
          for channel in args.channels.clone()
          {
            for feature in &features_powerset
            {
              let r = report.clone();
              s.spawn
              (
                move | _ |
                {
                  let mut args_t = SingleTestOptions::former()
                  .channel( channel )
                  .optimization( optimization )
                  .with_default_features( false )
                  .enable_features( feature.clone() );

                  if let Some( p ) = args.temp_path.clone()
                  {
                    let path = p.join( format!( "{}_{}_{}_{}", package.name.clone(), optimization, channel, feature.iter().join( "," ) ) );
                    std::fs::create_dir_all( &path ).unwrap();
                    args_t = args_t.temp_directory_path( path );
                  }
                  let cmd_rep = _run(dir, args_t.form(), dry);
                  let variant = TestVariant::former().channel( channel ).optimization( optimization ).features( feature.iter().join( "," ) ).form();
                  r.lock().unwrap().tests.insert( variant, cmd_rep.map_err( | e | e.0 ) );
                }
              );
            }
          }
        }
      }
    );

    // unpack. all tasks must be completed until now
    let report = Mutex::into_inner( Arc::into_inner( report ).unwrap() ).unwrap();
    let at_least_one_failed = report
    .tests
    .iter()
    .any( | ( _, result ) | result.is_err() );
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

  // qqq : for Petro : should be entity `struct Plan {}`
  // qqq : for Petro : no! Plan should inplement Display
  fn print_temp_report( package_name : &str, optimizations : &HashSet< Optimization >, channels : &HashSet< Channel >, features : &HashSet< BTreeSet< String > > )
  {
    println!( "Package : {}\nThe tests will be executed using the following configurations :", package_name );
    for optimization in optimizations.iter().sorted()
    {
      for channel in channels.iter().sorted()
      {
        for feature in features
        {
          let feature = if feature.is_empty() { "-".to_string() } else { feature.iter().join( "," ) };
          println!( "  [ optimization : {optimization} | channel : {channel} | feature : {feature} ]" );
        }
      }
    }
  }
}

crate::mod_interface!
{

  protected use SingleTestOptions;
  protected use TestVariant;
  protected use _run;

  protected use TestOptions;
  protected use TestReport;
  protected use TestsReport;
  protected use run;
  protected use tests_run;
}