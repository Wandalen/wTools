use super::*;
use the_module::
{
  Workspace,
  _path::AbsolutePath,
  package::{Plan, PublishPlan},
};

#[ test ]
fn plan_publish_many_packages()
{
  let workspace = Workspace::from_current_path().unwrap();
  let package = workspace.package_find_by_manifest( /* AbsolutePath::try_from( "../wca/Cargo.toml" ).unwrap() */ ).unwrap().to_owned();
  let mega_plan = PublishPlan::former()
  .workspace( workspace )
  .base_temp_dir( "temp" )
  .packages([ package ])
  .form();
  dbg!( &mega_plan.plans );
  // [module/move/willbe/tests/inc/package.rs:19:3] &mega_plan.plans = [
  //     PublishSinglePackagePlan {
  //         pack: CargoPackagePlan {
  //             crate_dir: CrateDir(
  //                 AbsolutePath(
  //                     ".../wTools/module/move/wca",
  //                 ),
  //             ),
  //             base_temp_dir: Some(
  //                 "temp",
  //             ),
  //         },
  //         version_bump: VersionBumpPlan {
  //             crate_dir: CrateDir(
  //                 AbsolutePath(
  //                     ".../wTools/module/move/wca",
  //                 ),
  //             ),
  //             old_version: Version(
  //                 Version {
  //                     major: 0,
  //                     minor: 12,
  //                     patch: 0,
  //                 },
  //             ),
  //             new_version: Version(
  //                 Version {
  //                     major: 0,
  //                     minor: 13,
  //                     patch: 0,
  //                 },
  //             ),
  //             dependencies: [
  //                 CrateDir(
  //                     AbsolutePath(
  //                         ".../wTools",
  //                     ),
  //                 ),
  //             ],
  //         },
  //         git_things: GitThingsPlan {
  //             git_root: AbsolutePath(
  //                 ".../wTools",
  //             ),
  //             items: [
  //                 AbsolutePath(
  //                     ".../wTools/Cargo.toml",
  //                 ),
  //                 AbsolutePath(
  //                     ".../wTools/module/move/wca/Cargo.toml",
  //                 ),
  //             ],
  //             message: "wca-v0.13.0",
  //         },
  //         publish: CargoPublishPlan {
  //             crate_dir: CrateDir(
  //                 AbsolutePath(
  //                     ".../wTools/module/move/wca",
  //                 ),
  //             ),
  //             base_temp_dir: Some(
  //                 "temp",
  //             ),
  //         },
  //     },
  // ]
  let mega_plan = mega_plan.perform( true );
  dbg!( mega_plan );
  // [module/move/willbe/tests/inc/package.rs:21:3] mega_plan = Ok(
  //     [
  //         PublishReport {
  //             get_info: Some(
  //                 CmdReport {
  //                     command: "cargo package --target-dir temp",
  //                     path: ".../wTools/module/move/wca",
  //                     out: "",
  //                     err: "",
  //                 },
  //             ),
  //             publish_required: true,
  //             bump: Some(
  //                 ExtendedBumpReport {
  //                     base: BumpReport {
  //                         name: Some(
  //                             "wca",
  //                         ),
  //                         old_version: Some(
  //                             "0.12.0",
  //                         ),
  //                         new_version: Some(
  //                             "0.13.0",
  //                         ),
  //                     },
  //                     changed_files: [
  //                         AbsolutePath(
  //                             ".../wTools/module/move/wca/Cargo.toml",
  //                         ),
  //                         AbsolutePath(
  //                             ".../wTools/Cargo.toml",
  //                         ),
  //                     ],
  //                 },
  //             ),
  //             add: Some(
  //                 CmdReport {
  //                     command: "git add Cargo.toml module/move/wca/Cargo.toml",
  //                     path: ".../wTools",
  //                     out: "",
  //                     err: "",
  //                 },
  //             ),
  //             commit: Some(
  //                 CmdReport {
  //                     command: "git commit -m wca-v0.13.0",
  //                     path: ".../wTools",
  //                     out: "",
  //                     err: "",
  //                 },
  //             ),
  //             push: Some(
  //                 CmdReport {
  //                     command: "git push",
  //                     path: ".../wTools",
  //                     out: "",
  //                     err: "",
  //                 },
  //             ),
  //             publish: Some(
  //                 CmdReport {
  //                     command: "cargo publish --target-dir temp",
  //                     path: ".../wTools/module/move/wca",
  //                     out: "",
  //                     err: "",
  //                 },
  //             ),
  //         },
  //     ],
  // )
  panic!()
}
