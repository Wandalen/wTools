#![doc(html_logo_url = "https: //raw.githubusercontent.com/Wandalen/wTools/master/asset/img/logo_v3_trans_square.png")]
#![doc(
  html_favicon_url = "https: //raw.githubusercontent.com/Wandalen/wTools/alpha/asset/img/logo_v3_trans_square_icon_small_v2.ico"
)]
#![doc(html_root_url = "https: //docs.rs/willbe/")]
#![ cfg_attr( doc, doc = include_str!( concat!( env!( "CARGO_MANIFEST_DIR" ), "/", "readme.md" ) ) ) ]
#![ cfg_attr( not( doc ), doc = "Build and project management binary" ) ]

// qqq2: xxx2: fix broken sequence of publishing because of skipping debug dependencies
//
// cd module/core/former_meta
// cargo package --allow-dirty --no-verify
//
// Caused by :
//   failed to select a version for `former_types`.
//       ... required by package `macro_tools v0.46.0`
//       ... which satisfies dependency `macro_tools = "~0.46.0"` of package `impls_index_meta v0.10.0`
//       ... which satisfies dependency `impls_index_meta = "~0.10.0"` of package `test_tools v0.12.0`
//       ... which satisfies dependency `test_tools = "~0.12.0"` of package `former_meta v2.12.0 (C: \pro\lib\wtools\module\core\former_meta)`
//   versions that meet the requirements `~2.14.0` are: 2.14.0
//
//   all possible versions conflict with previously selected packages.
//
//     previously selected package `former_types v2.15.0`
//       ... which satisfies dependency `former_types = "~2.15.0"` of package `former_meta v2.12.0 (C: \pro\lib\wtools\module\core\former_meta)`
//
//   failed to select a version for `former_types` which could resolve this conflict

// qqq2: xx2: attempt to publish graphs_tools publish all crates do not respecting check on outdate
//
// Wrong :
// [0] interval_adapter (0.28.0 -> 0.29.0)
// [1] collection_tools (0.17.0 -> 0.18.0)
// [2] former_types (2.14.0 -> 2.15.0)
// [3] clone_dyn_types (0.28.0 -> 0.29.0)
// [4] iter_tools (0.26.0 -> 0.27.0)
// [5] macro_tools (0.46.0 -> 0.47.0)
// [6] derive_tools_meta (0.32.0 -> 0.33.0)
// [7] variadic_from (0.28.0 -> 0.29.0)
// [8] former_meta (2.12.0 -> 2.13.0)
// [9] impls_index_meta (0.10.0 -> 0.11.0)
// [10] clone_dyn_meta (0.28.0 -> 0.29.0)
// [11] clone_dyn (0.30.0 -> 0.31.0)
// [12] derive_tools (0.33.0 -> 0.34.0)
// [13] mod_interface_meta (0.30.0 -> 0.31.0)
// [14] mod_interface (0.31.0 -> 0.32.0)
// [15] for_each (0.10.0 -> 0.11.0)
// [16] impls_index (0.9.0 -> 0.10.0)
// [17] meta_tools (0.12.0 -> 0.13.0)
// [18] former (2.12.0 -> 2.13.0)
// [19] graphs_tools (0.3.0 -> 0.4.0)
//
// Correct :
// [0] impls_index (0.9.0 -> 0.10.0)
// [1] for_each (0.10.0 -> 0.11.0)
// [2] meta_tools (0.12.0 -> 0.13.0)
// [3] graphs_tools (0.3.0 -> 0.4.0)

// qqq2: xxx2: another problem
// if you publish a crate and after you try to publish another which depends on the first willbe don't see any changes and don't publish second
// for example publishing impl_index -> after publising test_tools make willbe struggle to see that publishing of test_tools is required

#![allow(ambiguous_glob_imports)]

use mod_interface ::meta ::mod_interface;

/// Define a private namespace for all its items.
mod private 
{

  use crate :: { error, command };

  /// Takes the command line arguments and perform associated function(s).
  /// If no arguments are provided, the function identifies this as an ambiguous state and prompts the user with a help message, suggesting possible commands they might want to execute.
  /// It then terminates the program with an exit code of 1 to indicate an error due to the lack of input.
  ///
  /// Do not support interactive mode.
  ///
  /// # Errors
  /// qqq: doc
  pub fn run(args: Vec< String >) -> Result< (), error ::untyped ::Error >
  {
  #[ cfg(feature = "tracing") ]
  {
   tracing_subscriber ::fmt().pretty().init();
 }

  let args: Vec< String > = args.into_iter().skip(1).collect();

  let ca = command ::ca()
   .help_variants([wca ::HelpVariants ::General, wca ::HelpVariants ::SubjectCommand])
   .perform();

  let program = args.join(" ");
  if program.is_empty() 
  {
   eprintln!("Ambiguity. Did you mean?");
   ca.perform(".help")?;
   std ::process ::exit(1);
 } else {
   Ok(ca.perform(program.as_str())?)
 }
}

}

mod_interface! {

  own use private ::run;

  /// Error handling facade.
  layer error;

  /// Entities of which spaces consists of.
  layer entity;

  /// Genera-purpose tools which might be moved out one day.
  layer tool;

  /// Describes CLI commands.
  layer command;

  /// Describes functions that can be called from an interface.
  layer action;

}

// Re-export thiserror outside of mod_interface since it doesn't have the required structure
pub use ::error_tools ::dependency ::thiserror;
