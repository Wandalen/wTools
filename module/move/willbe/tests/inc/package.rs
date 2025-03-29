use std::*;
use std::io::Write;

use crate::the_module::{ action, channel, package };

enum Dependency
{
  Normal { name: String, path: Option< path::PathBuf >, is_macro: bool },
  Dev { name: String, path: Option< path::PathBuf >, is_macro: bool },
}

impl Dependency
{
  fn as_toml( &self ) -> String
  {
    match self
    {
      Dependency::Normal { name, path, is_macro } if !is_macro => 
      if let Some( path ) = path
      {
        format!( "[dependencies.{name}]\npath = \"../{}\"", path.display().to_string().replace( "\\", "/" ) )
      }
      else
      {
        format!( "[dependencies.{name}]\nversion = \"*\"" )
      }
      Dependency::Normal { name, .. } => format!( "[dependencies.{name}]\nworkspace = true" ),
      Dependency::Dev { name, path, is_macro } if !is_macro =>
      if let Some( path ) = path
      {
        format!( "[dev-dependencies.{name}]\npath = \"../{}\"", path.display().to_string().replace( "\\", "/" ) )
      }
      else
      {
        format!( "[dev-dependencies.{name}]\nversion = \"*\"" )
      }
      Dependency::Dev { name, .. } => format!( "[dev-dependencies.{name}]\nworkspace = true" ),
    }
  }
}

struct TestPackage
{
  name: String,
  dependencies: Vec< Dependency >,
  path: Option< path::PathBuf >,
}

impl TestPackage
{
  pub fn new( name: impl Into< String > ) -> Self
  {
    Self { name: name.into(), dependencies: vec![], path: None }
  }
  
  pub fn dependency( mut self, name: impl Into< String > ) -> Self
  {
    self.dependencies.push( Dependency::Normal { name: name.into(), path: None, is_macro: false } );
    self
  }
  
  pub fn macro_dependency( mut self, name: impl Into< String > ) -> Self
  {
    self.dependencies.push( Dependency::Normal { name: name.into(), path: None, is_macro: true } );
    self
  }
  
  pub fn dev_dependency( mut self, name: impl Into< String > ) -> Self
  {
    self.dependencies.push( Dependency::Dev { name: name.into(), path: None, is_macro: false } );
    self
  }

  pub fn macro_dev_dependency( mut self, name: impl Into< String > ) -> Self
  {
    self.dependencies.push( Dependency::Dev { name: name.into(), path: None, is_macro: true } );
    self
  }
  
  pub fn create( &mut self, path: impl AsRef< path::Path > ) -> io::Result< () >
  {
    let path = path.as_ref().join( &self.name );

    () = fs::create_dir_all( path.join( "src" ) )?;
    () = fs::write( path.join( "src" ).join( "lib.rs" ), &[] )?;
    
    let cargo = format!
    (
      r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"
{}"#,
      self.name,
      self.dependencies.iter().map( Dependency::as_toml ).fold( String::new(), | acc, d |
      {
        format!( "{acc}\n\n{d}" )
      })
    );
    () = fs::write( path.join( "Cargo.toml" ), cargo.as_bytes() )?;
    
    self.path = Some( path );
    
    Ok( () )
  }
}

impl Drop for TestPackage
{
  fn drop( &mut self )
  {
    if let Some( path ) = &self.path
    {
      _ = fs::remove_dir_all( path ).ok();
    }
  }
}

struct TestWorkspace
{
  packages: Vec< TestPackage >,
  path: path::PathBuf,
}

impl TestWorkspace
{
  fn new( path: impl AsRef< path::Path > ) -> io::Result< Self >
  {
    let path = path.as_ref();
    () = fs::create_dir_all( path )?;

    let cargo = r#"[workspace]
resolver = "2"
members = [
    "members/*",
]
"#;
    () = fs::write( path.join( "Cargo.toml" ), cargo.as_bytes() )?;

    Ok(Self { packages: vec![], path: path.into() })
  }
  
  fn find( &self, package_name: impl AsRef< str > ) -> Option< &TestPackage >
  {
    let name = package_name.as_ref();
    self.packages.iter().find( | p | p.name == name )
  }

  fn with_package( mut self, mut package: TestPackage ) -> io::Result< Self >
  {
    let mut macro_deps = collections::HashMap::new();
    for dep in &mut package.dependencies
    {
      match dep
      {
        Dependency::Normal { name, is_macro, .. } if *is_macro =>
        {
          if let Some( package ) = self.find( &name )
          {
            if let Some( path ) = &package.path
            {
              macro_deps.insert( name.clone(), path.clone() );
              continue;
            }
          }
          eprintln!( "macro dependency {} not found. required for {}", name, package.name );
        }
        Dependency::Normal { name, path, .. } =>
        {
          if let Some( package ) = self.find( &name )
          {
            if let Some( real_path ) = &package.path
            {
              let real_path = real_path.strip_prefix( self.path.join( "members" ) ).unwrap_or( real_path );
              *path = Some( real_path.into() );
            }
          }
        }
        Dependency::Dev { name, is_macro, .. } if *is_macro =>
        {
          if let Some( package ) = self.find( &name )
          {
            if let Some( path ) = &package.path
            {
              macro_deps.insert( name.clone(), path.clone() );
              continue;
            }
          }
          eprintln!( "macro dev-dependency {} not found. required for {}", name, package.name );
        }
        Dependency::Dev { name, path, .. } =>
        {
          if let Some( package ) = self.find( &name )
          {
            if let Some( real_path ) = &package.path
            {
              let real_path = real_path.strip_prefix( self.path.join( "members" ) ).unwrap_or( real_path );
              *path = Some( real_path.into() );
            }
          }
        }
      }
    }
    let mut cargo = fs::OpenOptions::new().append( true ).open( self.path.join( "Cargo.toml" ) )?;
    for ( name, _ ) in macro_deps
    {
      writeln!( cargo, 
        r#"[workspace.dependencies.{name}]
version = "*"
path = "members/{name}""#,
      )?;
    }
    package.create( self.path.join( "members" ) )?;
    self.packages.push( package );
    
    Ok( self )
  }
  
  fn with_packages( mut self, packages: impl IntoIterator< Item = TestPackage > ) -> io::Result< Self >
  {
    for package in packages { self = self.with_package( package )?; }
    
    Ok( self )
  }
}

impl Drop for TestWorkspace
{
  fn drop( &mut self )
  {
    _ = fs::remove_dir_all( &self.path ).ok();
  }
}

#[ test ]
fn kos_plan()
{
  let tmp_folder = env::temp_dir().join( "publish_plan_kos_plan" );
  _ = fs::remove_dir_all( &tmp_folder ).ok();
  
  let workspace = TestWorkspace::new( tmp_folder ).unwrap()
  .with_packages(
  [
    TestPackage::new( "a" ),
    TestPackage::new( "b" ).dependency( "a" ),
    TestPackage::new( "c" ).dependency( "a" ),
    TestPackage::new( "d" ).dependency( "a" ),
    TestPackage::new( "e" ).dependency( "b" ).macro_dev_dependency( "c" ),//.macro_dependency( "c" ),
  ]).unwrap();
  let the_patterns: Vec< String > = workspace
  .packages
  .iter()
  .flat_map( | p | p.path.as_ref().map( | p | p.to_string_lossy().into_owned() ) )
  .collect();
  dbg!(&the_patterns);
  
  let plan = action::publish_plan
  (
    the_patterns,
    channel::Channel::Stable,
    false,
    false,
    true,
    false,
  )
  .unwrap();
  
  let queue: Vec< &package::PackageName > = plan.plans.iter().map( | i | &i.package_name ).collect();
  dbg!(&queue);
  
  // We don’t consider dev dependencies when constructing the project graph, which results in this number of variations.
  // If you'd like to modify this behavior, please check `entity/workspace_graph.rs` in the `module_dependency_filter`.
  let expected_one_of=
  [
    [ "a", "b", "d", "c", "e" ],
    [ "a", "b", "c", "d", "e" ],
    [ "a", "d", "b", "c", "e" ],
    [ "a", "c", "b", "d", "e" ],
    [ "a", "d", "c", "b", "e" ],
    [ "a", "c", "d", "b", "e" ],
    [ "a", "b", "d", "e", "c" ],
    [ "a", "d", "b", "e", "c" ],
    [ "a", "b", "e", "d", "c" ],
    [ "a", "e", "b", "d", "c" ],
    [ "a", "d", "e", "b", "c" ],
    [ "a", "e", "d", "b", "c" ],
    [ "a", "b", "c", "e", "d" ],
    [ "a", "c", "b", "e", "d" ],
    [ "a", "b", "e", "c", "d" ],
    [ "a", "e", "b", "c", "d" ],
    [ "a", "c", "e", "b", "d" ],
    [ "a", "e", "c", "b", "d" ],
  ];
  
  let mut fail = true;
  'sequences: for sequence in expected_one_of
  {
    for index in 0 .. 5
    {
      if *queue[ index ] != sequence[ index ].to_string().into() { continue 'sequences; }
    }
    fail = false;
    break;
  }
  assert!( !fail );
}

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
