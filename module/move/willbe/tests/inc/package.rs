// use super::*;
// use the_module::
// {
//   Workspace,
//   path::AbsolutePath,
//   package::PublishPlan,
// };
// use willbe::package::perform_packages_publish;
//
// #[ test ]
// fn plan_publish_many_packages()
// {
//   let workspace = Workspace::from_current_path().unwrap();
//   let package = workspace.package_find_by_manifest( /* AbsolutePath::try_from( "../wca/Cargo.toml" ).unwrap() */ ).unwrap().to_owned();
//   let mega_plan = PublishPlan::former()
//   .workspace( workspace )
//   .base_temp_dir( "temp" )
//   .packages([ package ])
//   .form();
//   dbg!( &mega_plan.plans );
// //   [module\move\willbe\tests\inc\package.rs:21:3] &mega_plan.plans = [
// //   PackagePublishInstruction {
// //     pack: PackOptions {
// //       path: ".../wTools/module/move/wca",
// //       temp_path: Some(
// //         "temp",
// //       ),
// //       dry: true,
// //     },
// //     bump: BumpOptions {
// //       crate_dir: CrateDir(
// //         AbsolutePath(
// //           ".../wTools/module/move/wca",
// //         ),
// //       ),
// //       old_version: Version(
// //         Version {
// //           major: 0,
// //           minor: 14,
// //           patch: 0,
// //         },
// //       ),
// //       new_version: Version(
// //         Version {
// //           major: 0,
// //           minor: 15,
// //           patch: 0,
// //         },
// //       ),
// //       dependencies: [
// //         CrateDir(
// //           AbsolutePath(
// //             ".../wTools",
// //           ),
// //         ),
// //       ],
// //       dry: true,
// //     },
// //     git_things: GitThingsOptions {
// //       git_root: AbsolutePath(
// //         ".../wTools",
// //       ),
// //       items: [
// //         AbsolutePath(
// //           ".../wTools/Cargo.toml",
// //         ),
// //         AbsolutePath(
// //           ".../wTools/module/move/wca/Cargo.toml",
// //         ),
// //       ],
// //       message: "wca-v0.15.0",
// //       dry: true,
// //     },
// //     publish: PublishOptions {
// //       path: ".../wTools/module/move/wca",
// //       temp_path: Some(
// //         "temp",
// //       ),
// //       dry: true,
// //     },
// //     dry: true,
// //   },
// // ]
//   let mega_plan = perform_packages_publish( mega_plan );
//   dbg!( mega_plan );
// //   [module\move\willbe\tests\inc\package.rs:89:3] mega_plan = Ok(
// //   [
// //     PublishReport {
// //       get_info: Some(
// //         Report {
// //           command: "cargo package --target-dir temp",
// //           current_path: ".../wTools/module/move/wca",
// //           out: "",
// //           err: "",
// //           error: Ok(
// //             (),
// //           ),
// //         },
// //       ),
// //       publish_required: true,
// //       bump: Some(
// //         ExtendedBumpReport {
// //           base: BumpReport {
// //             name: Some(
// //               "wca",
// //             ),
// //             old_version: Some(
// //               "0.14.0",
// //             ),
// //             new_version: Some(
// //               "0.15.0",
// //             ),
// //           },
// //           changed_files: [
// //             AbsolutePath(
// //               ".../wTools/module/move/wca/Cargo.toml",
// //             ),
// //             AbsolutePath(
// //               ".../wTools/Cargo.toml",
// //             ),
// //           ],
// //         },
// //       ),
// //       add: Some(
// //         Report {
// //           command: "git add Cargo.toml module/move/wca/Cargo.toml",
// //           current_path: ".../wTools",
// //           out: "",
// //           err: "",
// //           error: Ok(
// //             (),
// //           ),
// //         },
// //       ),
// //       commit: Some(
// //         Report {
// //           command: "git commit -m wca-v0.15.0",
// //           current_path: ".../wTools",
// //           out: "",
// //           err: "",
// //           error: Ok(
// //             (),
// //           ),
// //         },
// //       ),
// //       push: Some(
// //         Report {
// //           command: "git push",
// //           current_path: ".../wTools",
// //           out: "",
// //           err: "",
// //           error: Ok(
// //             (),
// //           ),
// //         },
// //       ),
// //       publish: Some(
// //         Report {
// //           command: "cargo publish --target-dir temp",
// //           current_path: ".../wTools/module/move/wca",
// //           out: "",
// //           err: "",
// //           error: Ok(
// //             (),
// //           ),
// //         },
// //       ),
// //     },
// //   ],
// // )
//   panic!()
// }

// qqq : for Bohdan : fix the test
