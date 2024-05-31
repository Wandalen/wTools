mod private
{

  // qqq : for Petro : use https://github.com/console-rs/indicatif

  use crate::*;
  use table::*;
  use std::
  {
    collections::{ BTreeMap, BTreeSet, HashSet },
    fmt::Formatter,
    sync::{ Arc, Mutex },
    path::Path,
  };
  use std::collections::HashMap;
  use std::ffi::OsString;
  use std::fmt::{ Debug, Display }; /* qqq : import only fmt here and everywhere */
  use std::marker::PhantomData;
  use std::path::PathBuf;
  // aaa : for Petro : don't use cargo_metadata directly, use facade
  // aaa : ✅
  use colored::Colorize;
  // qqq : for Petro : don't do micro imports
  // qqq : for Petro : don't do micro imports
  #[ cfg( feature = "progress_bar" ) ]
  use indicatif::
  {
    MultiProgress,
    ProgressBar,
    ProgressStyle
  };
  use rayon::ThreadPoolBuilder;
  use process_tools::process::*;
  use wtools::error::anyhow::{ Error, format_err };
  use wtools::iter::Itertools;
  use wtools::error::Result;
  use former::Former;
  use channel::Channel;
  use optimization::Optimization;
  use workspace::WorkspacePackage;

  /// Newtype for package name
  #[ derive( Debug, Default, Clone ) ]
  pub struct PackageName( String );

  /// Represents a variant for testing purposes.
  #[ derive( Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Former ) ]
  pub struct TestVariant
  {
    /// Represents the channel for the test variant.
    channel : Channel,
    /// Represents the optimization setting for the test variant.
    optimization : Optimization,
    /// Contains additional features or characteristics of the test variant.
    features : BTreeSet< String >,
  }

  impl Display for TestVariant
  {
    fn fmt( &self, f : &mut Formatter< '_ >) -> std::fmt::Result
    {
      let features = if self.features.is_empty() { " ".to_string() } else { self.features.iter().join( " " ) };
      writeln!( f, "{} {} {}", self.optimization, self.channel, features )?;
      Ok( () )
    }
  }

  /// Global test plan
  #[ derive( Debug ) ]
  pub struct TestPlan
  {
    packages_plan : Vec< TestPackagePlan >,
  }

  impl Display for TestPlan
  {
    fn fmt( &self, f : &mut Formatter< '_ >) -> std::fmt::Result
    {
      writeln!( f, "Plan: " )?;
      for plan in &self.packages_plan
      {
        writeln!( f, "{plan}" )?;
      }
      Ok( () )
    }
  }

  impl TestPlan
  {
    /// Create plan from params:
    /// `packages` - List of packages which will be tested
    /// `channels` - A set of Cargo channels that are to be tested.
    /// `power` - An integer value indicating the power or intensity of testing.
    /// `include_features` - A vector of strings, each representing a feature to be included during testing.
    /// `exclude_features` - A vector of strings, each representing a feature to be excluded during testing.
    /// `optimizations` - A set of optimizations (Release & Debug)
    /// `enabled_features` - A slice of features names to always include in each subset of powerset.
    /// `with_all_features` - If it's true - add to powerset one subset which contains all features.
    /// `with_none_features` - If it's true - add to powerset one empty subset.
    /// `variants_cap` - Maximum of subset in powerset
    pub fn try_from
    (
      packages : &[ WorkspacePackage ],
      channels : &HashSet< Channel >,
      power : u32,
      include_features : Vec< String >,
      exclude_features : Vec< String >,
      optimizations : &HashSet< Optimization >,
      enabled_features : Vec< String >,
      with_all_features : bool,
      with_none_features : bool,
      variants_cap : u32,
    ) -> Result< Self >
    {
      let mut packages_plan = vec![];
      for package in packages
      {
        packages_plan.push
        (
          TestPackagePlan::try_from
          (
            package,
            channels,
            power,
            include_features.as_slice(),
            exclude_features.as_slice(),
            optimizations,
            enabled_features.as_slice(), with_all_features, with_none_features, variants_cap
          )?
        );
      }
      Ok
      (
        Self
        {
          packages_plan
        }
      )
    }
  }

  #[ derive( Debug ) ]
  pub struct TestPackagePlan
  {
    enabled_features : BTreeSet< String >,
    package : PathBuf,
    test_variants : BTreeSet< TestVariant >,
  }

  impl Display for TestPackagePlan
  {
    fn fmt( &self, f : &mut Formatter< '_ >) -> std::fmt::Result
    {
      writeln!( f, "Package : {}\nThe tests will be executed using the following configurations :", self.package.file_name().unwrap().to_string_lossy() )?;
      let mut all_features = BTreeSet::new();
      for variant in &self.test_variants
      {
        let features = variant.features.iter().cloned();
        if features.len() == 0
        {
          all_features.extend( [ "[]".to_string() ] );
        }
        all_features.extend( features );
      }
      let mut ff = Vec::from_iter( self.enabled_features.iter().cloned() );
      for feature in all_features
      {
        if !ff.contains( &feature )
        {
          ff.push( feature );
        }
      }
      let mut table = Table::default();
      // let format = format();
      // table.set_format( format );

      let mut header_row = Row::new();
      header_row.add_cell( "Channel" );
      header_row.add_cell( "Opt" );

      for feature in &ff
      {
        header_row.add_cell( feature );
      }
      table.set_header( header_row );

      for variant in &self.test_variants
      {
        let mut row = Row::new();

        row.add_cell( &variant.channel.to_string() );
        row.add_cell( &variant.optimization.to_string() );
        let counter = 0;
        let flag = true;
        generate_features_cells(&mut ff, variant, &mut row, counter, flag, &self.enabled_features );

        table.add_row( row );
      }
      // aaa : for Petro : bad, DRY
      // aaa : replace with method
      writeln!( f, "{}", table )?;
      Ok( () )
    }
  }

  impl TestPackagePlan
  {
    /// Create plan from params:
    /// `packages` - Package which will be tested
    /// `channels` - A set of Cargo channels that are to be tested.
    /// `power` - An integer value indicating the power or intensity of testing.
    /// `include_features` - A vector of strings, each representing a feature to be included during testing.
    /// `exclude_features` - A vector of strings, each representing a feature to be excluded during testing.
    /// `optimizations` - A set of optimizations (Release & Debug)
    /// `enabled_features` - A slice of features names to always include in each subset of powerset.
    /// `with_all_features` - If it's true - add to powerset one subset which contains all features.
    /// `with_none_features` - If it's true - add to powerset one empty subset.
    /// `variants_cap` - Maximum of subset in powerset
    fn try_from
    (
      package : &WorkspacePackage,
      channels : &HashSet< Channel >,
      power : u32,
      include_features : &[ String ],
      exclude_features : &[ String ],
      optimizations : &HashSet< Optimization >,
      enabled_features : &[ String ],
      with_all_features : bool,
      with_none_features : bool,
      variants_cap : u32,
    ) -> Result< Self >
    {
      let dir = package.manifest_path().parent().unwrap().as_std_path().to_path_buf();
      let mut test_variants = BTreeSet::new();
      let features_powerset = features::features_powerset
      (
        package,
        power as usize,
        exclude_features,
        include_features,
        enabled_features,
        with_all_features,
        with_none_features,
        variants_cap,
      )?;
      for optimization in optimizations
      {
        for channel in channels
        {
          for feature in &features_powerset
          {
            test_variants.insert
            (
              TestVariant
              {
                channel : channel.clone(),
                optimization : optimization.clone(),
                features : feature.clone(),
              }
            );
          }
        }
      }
      Ok
      (
        Self
        {
          enabled_features: enabled_features.iter().cloned().collect(),
          package : dir,
          test_variants,
        }
      )
    }
  }

  fn generate_features_cells( ff : &mut Vec< String >, variant : &TestVariant, row : &mut Row, mut counter : usize, mut flag : bool, enabled_features : &BTreeSet< String > )
  {
    for feature in ff
    {
      let mut c = "+";
      if variant.features.is_empty() && counter == enabled_features.len() && flag
      {
        flag = false;
        row.add_cell( c );
      }
      else if variant.features.contains( feature )
      {
        row.add_cell( c );
      }
      else
      {
        c = "";
        row.add_cell( c );
      }
      counter += 1;
    }
  }

  #[ derive( Debug, Former ) ]
  pub struct PackageTestOptions< 'a >
  {
    temp_path : Option< PathBuf >,
    plan : &'a TestPackagePlan,
    dry : bool,
    progress_bar_feature : Option< PackageTestOptionsProgressBarFeature< 'a > >,
  }

  #[ derive( Debug ) ]
  struct PackageTestOptionsProgressBarFeature< 'a >
  {
    phantom : PhantomData< &'a () >,
    #[ cfg( feature = "progress_bar" ) ]
    multi_progress : &'a Option< &'a MultiProgress >,
    #[ cfg( feature = "progress_bar" ) ]
    progress_bar : &'a Option< ProgressBar >
  }


  impl PackageTestOptionsFormer< '_ >
  {
    pub fn option_temp(  mut self, value : impl Into< Option< PathBuf > > ) -> Self
    {
      self.storage.temp_path = value.into();
      self
    }
  }

  /// Represents the options for the test.
  #[ derive( Debug, Former, Clone ) ]
  pub struct SingleTestOptions
  {
    /// Specifies the release channels for rust.
    /// More details : https://rust-lang.github.io/rustup/concepts/channels.html#:~:text=Rust%20is%20released%20to%20three,releases%20are%20made%20every%20night.
    channel : Channel,
    /// Specifies the optimization for rust.
    optimization : Optimization,
    /// Determines whether to use default features in the test.
    /// Enabled by default.
    #[ former( default = true ) ]
    with_default_features : bool,
    /// Determines whether to use all available features in the test.
    /// Disabled by default.
    #[ former( default = false ) ]
    with_all_features : bool,
    /// Specifies a list of features to be enabled in the test.
    enable_features : BTreeSet< String >,
    /// Temp directory path
    temp_directory_path : Option< PathBuf >,
    /// A boolean indicating whether to perform a dry run or not.
    dry : bool,
    /// RUST_BACKTRACE
    #[ former( default = true ) ]
    backtrace : bool,
  }

  impl SingleTestOptions
  {
    fn as_rustup_args( &self ) -> Vec< String >
    {
      debug_assert!( !self.with_default_features ); // qqq : remove later
      debug_assert!( !self.with_all_features ); // qqq : remove later
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
  pub fn _run< P >( path : P, options : SingleTestOptions ) -> Result< Report, Report >
  where
    P : AsRef< Path >
  {
    let ( program, args ) = ( "rustup", options.as_rustup_args() );
    // qqq : for Petro : rustup ???
    // aaa : for Petro : RUST_BACKTRACE=1 ?? //  add to SingleTestOptions, by default true
    // aaa : add

    if options.dry
    {
      Ok
      (
        Report
        {
          command : format!( "{program} {}", args.join( " " ) ),
          out : String::new(),
          err : String::new(),
          current_path: path.as_ref().to_path_buf(),
          error: Ok( () ),
        }
      )
    }
    else
    {
      let envs = if options.backtrace { [( "RUST_BACKTRACE".to_string(), "full".to_string() )].into_iter().collect() } else { HashMap::new() };
      Run::former()
      .bin_path( program )
      .args( args.into_iter().map( OsString::from ).collect::< Vec< _ > >() )
      .current_path( path.as_ref().to_path_buf() )
      .joining_streams( true )
      .env_variable( envs )
      .run()
    }
  }

  /// `TestOptions` is a structure used to store the arguments for tests.
  #[ derive( Former ) ]
  pub struct TestOptions
  {
    /// Plan for testing
    pub plan : TestPlan,

    /// `concurrent` - A usize value indicating how much test`s can be run at the same time.
    pub concurrent : u32,

    /// `temp_path` - path to temp directory.
    pub temp_path : Option< PathBuf >,

    /// A boolean indicating whether to perform a dry run or not.
    pub dry : bool,

    /// This field contains fields for progress_bar feature
    pub feature : Option< TestOptionsProgressBarFeature >,
  }

  // qqq : for Petro : remove after Former fix
  /// Structure for progress bar feature field
  pub struct TestOptionsProgressBarFeature
  {
    #[ cfg( feature = "progress_bar" ) ]
    /// Base progress bar
    pub multiprocess : MultiProgress,

    #[ cfg( feature = "progress_bar" ) ]
    /// Style for progress bar
    pub style : ProgressStyle,
  }

  impl Debug for TestOptionsProgressBarFeature
  {
    fn fmt( &self, f : &mut Formatter< '_ >) -> std::fmt::Result
    {
      f.debug_struct( "TestOptionsProgressBarFeature" )
      .finish()
    }
  }

  impl Debug for TestOptions
  {
    fn fmt( &self, f : &mut Formatter< '_ >) -> std::fmt::Result {
      f.debug_struct( "TestOptions" )
      .field( "plan", &self.plan)
      .field( "concurrent", &self.concurrent)
      .field( "temp_path", &self.temp_path)
      .field( "plan", &self.plan)
      .finish()
    }
  }

  impl TestOptionsFormer
  {
    pub fn option_temp(  mut self, value : impl Into< Option< PathBuf > > ) -> Self
    {
      self.storage.temp_path = value.into();
      self
    }
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
    pub package_name : PackageName, /* aaa : for Petro : bad, reuse newtype / aaa : add newtype*/
    /// A `BTreeMap` where the keys are `channel::Channel` enums representing the channels
    ///   for which the tests were run, and the values are nested `BTreeMap` where the keys are
    ///   feature names and the values are `Report` structs representing the test results for
    ///   the specific feature and channel.
    pub tests : BTreeMap< TestVariant, Result< Report, Report > > ,
    /// Enabled features
    pub enabled_features : BTreeSet< String >,
    // qqq : for Petro : rid off map of map of map, keep flat map
  }

  impl Display for TestReport
  {
    fn fmt( &self, f : &mut Formatter< '_ > ) -> std::fmt::Result
    {
      if self.dry
      {
        return Ok( () )
      }
      let mut failed = 0;
      let mut success = 0;
      let mut all_features = BTreeSet::new();
      for variant in self.tests.keys()
      {
        let features = variant.features.iter().cloned();
        if features.len() == 0
        {
          all_features.extend( [ "[]".to_string() ] );
        }
        all_features.extend( features );
      }
      let mut ff = Vec::from_iter( self.enabled_features.iter().cloned() );
      for feature in all_features
      {
        if !ff.contains( &feature )
        {
          ff.push( feature );
        }
      }
      let mut table = Table::default();
      let mut header_row = Row::new();
      header_row.add_cell( "Result" );
      header_row.add_cell( "Channel" );
      header_row.add_cell( "Opt" );
      for feature in &ff
      {
        header_row.add_cell( feature );
      }
      table.set_header( header_row );

      writeln!( f, "{} {}\n", "\n=== Module".bold(), self.package_name.0.bold() )?;
      if self.tests.is_empty()
      {
        writeln!( f, "unlucky" )?;
        return Ok( () );
      }
      for ( variant, result) in &self.tests
      {
        let mut row = Row::new();
        let result_text = match result
        {
          Ok( _ ) =>
          {
            success += 1;
            "✅"
          },
          Err( report ) =>
          {
            failed += 1;
            let mut out = report.out.replace( "\n", "\n      " );
            out.push_str( "\n" );
            write!( f, " ❌  > {}\n\n{out}", report.command )?;
            "❌"
          },
        };
        row.add_cell( result_text );
        row.add_cell( &variant.channel.to_string() );
        row.add_cell( &variant.optimization.to_string() );
        let counter = 0;
        let flag = true;
        generate_features_cells( &mut ff, variant, &mut row, counter, flag, &self.enabled_features );


        table.add_row( row );
      }
      // aaa : for Petro : bad, DRY
      // aaa : replace with method
      writeln!( f, "{}", table )?;
      writeln!( f, "  {}", generate_summary_message( failed, success ) )?;

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
    pub success_reports : Vec< TestReport >,
    /// Vector of failure reports.
    pub failure_reports : Vec< TestReport >,
  }

  impl Display for TestsReport
  {
    fn fmt( &self, f : &mut Formatter< '_ > ) -> std::fmt::Result
    {
      if self.dry
      {
        writeln!( f, "\nYou can execute the plan with 'will .test dry : 0'." )?;
        // qqq : for Petro : bad. should be exact command with exact parameters / при виклику зовнішніх команд повинен бути вивід у консоль про цей виклик і його аргументи за виключенням коли ційлий блок виводу прихований (у моєму випадку при фейлі)
        return Ok( () )
      }
      if self.success_reports.is_empty() && self.failure_reports.is_empty()
      {
        writeln!( f, "The tests have not been run."  )?;
        return Ok( () );
      }
      if !self.success_reports.is_empty()
      {
        writeln!( f, "Successful :" )?;
        for report in &self.success_reports
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
      writeln!( f, "  {}", generate_summary_message( self.failure_reports.len() as i32, self.success_reports.len() as i32 ) )?;

      Ok( () )
    }
  }

  /// `tests_run` is a function that runs tests on a given package with specified arguments.
  /// It returns a `TestReport` on success, or a `TestReport` and an `Error` on failure.
  pub fn run( options : &PackageTestOptions< '_ > ) -> Result< TestReport, ( TestReport, Error ) >
  {
    let mut report = TestReport::default();
    report.dry = options.dry;
    report.enabled_features = options.plan.enabled_features.clone();
    let report = Arc::new( Mutex::new( report ) );
    let dir = options.plan.package.clone();

    rayon::scope
    (
      | s |
      {
        for variant in &options.plan.test_variants
        {
          let TestVariant{ channel, optimization, features } = variant;
          let r = report.clone();
          let dir = dir.clone();
          s.spawn
          (
            move | _ |
            {
              let mut args_t = SingleTestOptions::former()
              .channel( *channel )
              .optimization( *optimization )
              .with_default_features( false )
              .enable_features( features.clone() )
              .dry( options.dry );

              if let Some( p ) = options.temp_path.clone()
              {
                let path = p.join( path_tools::path::unique_folder_name().unwrap() );
                // qqq : for Petro : rid off unwrap
                std::fs::create_dir_all( &path ).unwrap();
                args_t = args_t.temp_directory_path( path );
              }
              #[ cfg( feature = "progress_bar" ) ]
              let _s =
              {
                let s = if let Some( multi_progress ) = options.progress_bar_feature.as_ref().and_then( | f | f.multi_progress.as_ref() )
                {
                  let s = multi_progress.add( ProgressBar::new_spinner().with_message( format!( "{}", variant ) ) );
                  s.enable_steady_tick( std::time::Duration::from_millis( 100 ) );
                  Some( s )
                }
                else
                {
                  None
                };
                // spinner.enable_steady_tick( std::time::Duration::from_millis( 100 ) );
                s
              };
              let args = args_t.form();
              let temp_dir = args.temp_directory_path.clone();
              let cmd_rep = _run( dir, args );
              r.lock().unwrap().tests.insert( variant.clone(), cmd_rep );
              #[ cfg( feature = "progress_bar" ) ]
              options.progress_bar_feature.as_ref().unwrap().progress_bar.as_ref().map( | b | b.inc( 1 ) );
              if let Some( path ) = temp_dir
              {
                std::fs::remove_dir_all( path ).unwrap();
              }
            }
          );
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
  pub fn tests_run( args : &TestOptions ) -> Result< TestsReport, ( TestsReport, Error ) >
  {
    let mut report = TestsReport::default();
    report.dry = args.dry;
    let report = Arc::new( Mutex::new( report ) );
    let pool = ThreadPoolBuilder::new().use_current_thread().num_threads( args.concurrent as usize ).build().unwrap();
    pool.scope
    (
      | s |
      {
        for plan in &args.plan.packages_plan
        {
          let report = report.clone();
          s.spawn
          (
            move | _ |
            {
              #[ cfg( feature = "progress_bar" ) ]
              let pb =
              {
                let pb = if let Some( feature ) = args.feature.as_ref()
                {
                  let pb = feature.multiprocess.add(ProgressBar::new(plan.test_variants.len() as u64));
                  pb.set_style( args.feature.as_ref().unwrap().style.clone() );
                  pb.inc( 0 );
                  Some( pb )
                }
                else
                {
                  None
                };
                pb
              };
              #[ cfg( feature = "progress_bar" ) ]
              let multi_progress = args.feature.as_ref().map( | f | &f.multiprocess );
              let test_package_options = PackageTestOptions::former().option_temp( args.temp_path.clone() ).plan( plan ).dry( args.dry );
              #[ cfg( feature = "progress_bar" ) ]
              let test_package_options = test_package_options.progress_bar_feature
              (
                PackageTestOptionsProgressBarFeature
                {
                  phantom : PhantomData,
                  multi_progress : &multi_progress,
                  progress_bar : &pb,
                }
              );
              let options = test_package_options.form();
              match run( &options )
              {
                Ok( r ) =>
                {
                  report.lock().unwrap().success_reports.push( r );
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
}

crate::mod_interface!
{

  protected use SingleTestOptions;
  protected use TestVariant;
  protected use _run;

  protected use TestPlan;

  protected use TestOptions;
  protected use TestReport;
  protected use TestsReport;
  protected use run;
  protected use tests_run;

  protected use TestOptionsProgressBarFeature;
}