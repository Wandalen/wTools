/// Internal namespace.
mod private
{
  use crate::*;
  use test::*;
  // use path::AbsolutePath;

  use std::collections::HashSet;

  use std::{ env, fs };

  #[ cfg( feature = "progress_bar" ) ]
  use indicatif::{ MultiProgress, ProgressStyle };

  use former::Former;
  use wtools::
  {
    error::
    {
      for_app::
      {
        Error,
        format_err
      },
      Result
    },
    iter::Itertools,
  };

  /// Used to store arguments for running tests.
  ///
  /// - The `dir` field represents the directory of the crate under test.
  /// - The `channels` field is a set of `Channel` enums representing the channels for which the tests should be run.
  /// - The `concurrent` field determines how match tests can be run at the same time.
  /// - The `exclude_features` field is a vector of strings representing the names of features to exclude when running tests.
  /// - The `include_features` field is a vector of strings representing the names of features to include when running tests.
  #[ derive( Debug, Former ) ]
  pub struct TestsCommandOptions
  {
    dir : AbsolutePath,
    channels : HashSet< channel::Channel >,
    #[ former( default = 0u32 ) ]
    concurrent : u32,
    #[ former( default = 1u32 ) ]
    power : u32,
    include_features : Vec< String >,
    #[ former( default  = [ "full".to_string(), "default".to_string() ] ) ]
    exclude_features : Vec< String >,
    #[ former( default = true ) ]
    temp : bool,
    enabled_features : Vec< String >,
    #[ former( default = true ) ]
    with_all_features : bool,
    #[ former( default = true ) ]
    with_none_features : bool,
    optimizations : HashSet< optimization::Optimization >,
    #[ former( default = 1000u32 ) ]
    variants_cap : u32,
    #[ former( default = false ) ]
    with_progress : bool,
  }


  /// The function runs tests with a different set of features in the selected crate (the path to the crate is specified in the dir variable).
  /// Tests are run with each feature separately, with all features together, and without any features.
  /// The tests are run in nightly and stable versions of Rust.
  /// It is possible to enable and disable various features of the crate.
  /// The function also has the ability to run tests in parallel using `Rayon` crate.
  /// The result of the tests is written to the structure `TestsReport` and returned as a result of the function execution.
  // zzz : it probably should not be here
  pub fn test( o : TestsCommandOptions, dry : bool ) -> Result< TestsReport, ( TestsReport, Error ) >
  {

    // qqq : incapsulate progress bar logic into some function of struct. don't keep it here
    #[ cfg( feature = "progress_bar" ) ]
    let multiprocess = MultiProgress::new();
    #[ cfg( feature = "progress_bar" ) ]
    let style = ProgressStyle::with_template
    (
      "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
    )
    .unwrap()
    .progress_chars( "##-" );

    let mut report = TestsReport::default();
    // fail fast if some additional installations required
    let channels = channel::available_channels( o.dir.as_ref() ).map_err( | e | ( report.clone(), e ) )?;
    let channels_diff = o.channels.difference( &channels ).collect::< Vec< _ > >();
    if !channels_diff.is_empty()
    {
      // aaa : for Petro : non readable
      // aaa : readable and with actual command
      return Err
      ((
        report,
        format_err!
        (
          "Missing toolchain(-s) that was required : [{}]. Try to install it with `rustup install {}` command(-s)",
          channels_diff.iter().join( ", " ),
          channels_diff.iter().join( " " )
        )
      ))
    }
    report.dry = dry;
    let TestsCommandOptions
    {
      dir : _ ,
      channels,
      concurrent,
      power,
      include_features,
      exclude_features,
      temp,
      enabled_features,
      with_all_features,
      with_none_features,
      optimizations,
      variants_cap,
      with_progress,
    } = o;

    // qqq : for Petro : xxx : use relevant entity. use either, implement TryFrom< Either< CrateDir, ManifestFile > >
    let path = if o.dir.as_ref().file_name() == Some( "Cargo.toml".as_ref() )
    {
      o.dir.parent().unwrap()
    }
    else
    {
      o.dir
    };

    let workspace = Workspace
    ::with_crate_dir( CrateDir::try_from( path.clone() ).map_err( | e | ( report.clone(), e.into() ) )? )
    .map_err( | e | ( report.clone(), e.into() ) )?
    // xxx : clone?
    ;

    // let packages = needed_packages( &workspace );
    let packages = workspace
    .packages()
    .filter( move | p | p.manifest_file().unwrap().starts_with( path.as_ref() ) ) // qqq : rid off unwrap
    ;

    let plan = TestPlan::try_from
    (
      packages,
      &channels,
      power,
      include_features,
      exclude_features,
      &optimizations,
      enabled_features,
      with_all_features,
      with_none_features,
      variants_cap,
    ).map_err( | e | ( report.clone(), e ) )?;

    println!( "{plan}" );
      // qqq : split on two functions for create plan and for execute

    let temp_path =  if temp
    {
      let mut unique_name = format!
      (
        "temp_dir_for_test_command_{}",
        path_tools::path::unique_folder_name().map_err( | e | ( report.clone(), e.into() ) )?
      );

      let mut temp_dir = env::temp_dir().join( unique_name );

      while temp_dir.exists()
      {
        unique_name = format!
        (
          "temp_dir_for_test_command_{}",
          path_tools::path::unique_folder_name().map_err( | e | ( report.clone(), e.into() ) )?
        );
        temp_dir = env::temp_dir().join( unique_name );
      }

      fs::create_dir( &temp_dir ).map_err( | e | ( report.clone(), e.into() ) )?;
      Some( temp_dir )
    }
    else
    {
      None
    };

    let test_options_former = TestOptions::former()
    .concurrent( concurrent )
    .plan( plan )
    .option_temp( temp_path )
    .dry( dry );

    #[ cfg( feature = "progress_bar" ) ]
    let test_options_former = if with_progress
    {
      let test_options_former = test_options_former.feature( TestOptionsProgressBarFeature{ multiprocess, style } );
      test_options_former
    }
    else
    {
      test_options_former
    };

    let options = test_options_former.form();
    let result = tests_run( &options );

    if temp
    {
      fs::remove_dir_all( options.temp_path.unwrap() ).map_err( | e | ( report.clone(), e.into() ) )?;
    }

    result
  }

}

crate::mod_interface!
{
  /// run all tests in all crates
  orphan use test;
  protected use TestsCommandOptions;
}
