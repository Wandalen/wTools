use super :: *;

// qqq: for Bohdan: bad. don't import the_module :: *
use inc ::helper :: { ProjectBuilder, WorkspaceBuilder };

use std ::collections ::BTreeSet;
use assert_fs ::TempDir;

use the_module ::action ::test :: { test, TestsCommandOptions };
use the_module ::channel :: *;
// use the_module ::optimization :: *;
use the_module ::optimization :: { self, Optimization };
use the_module ::AbsolutePath;
// qqq: for Petro: no astersisks import
use willbe ::test ::TestVariant;

#[ test ]
// if the test fails => the report is returned as an error ( Err(Report) )
fn fail_test() 
{
  let temp = TempDir ::new().unwrap();
  let temp = &temp;

  let project = ProjectBuilder ::new("fail_test")
  .toml_file("[features]\nenabled = []")
  .test_file(
   r"
  #[ test ]
  fn should_fail()
  {
   panic!()
 }
  ",
 )
  .build(temp)
  .unwrap();
  let abs = AbsolutePath ::try_from(project).unwrap();

  let args = TestsCommandOptions ::former()
  .dir(abs)
  .channels([Channel ::Stable])
  .optimizations([Optimization ::Debug])
  .with_none_features(true)
  .form();

  let rep = test(args, false).unwrap_err().0;
  println!("========= OUTPUT =========\n{rep}\n==========================");

  let no_features = rep.failure_reports[0]
  .tests
  .get(
   &TestVariant ::former()
  .optimization(Optimization ::Debug)
  .channel(Channel ::Stable)
  .features(BTreeSet ::default())
  .form(),
 )
  .unwrap();

  assert!(no_features.is_err());
  assert!(no_features.clone().unwrap_err().out.contains("failures"));
}

#[ test ]
// if a compilation error occurred => the report is returned as an error ( Err(Report) )
fn fail_build() 
{
  let temp = TempDir ::new().unwrap();
  let temp = &temp;

  let project = ProjectBuilder ::new("fail_build")
  .lib_file("compile_error!( \"achtung\" );")
  .toml_file("[features]\nenabled = []")
  .test_file(
   r"
  #[ test ]
  fn should_pass() 
  {
   assert!(true);
 }
  ",
 )
  .build(temp)
  .unwrap();
  let abs = AbsolutePath ::try_from(project).unwrap();

  let args = TestsCommandOptions ::former()
  .dir(abs)
  .channels([Channel ::Stable])
  .optimizations([Optimization ::Debug])
  .with_none_features(true)
  .form();

  let rep = test(args, false).unwrap_err().0;
  println!("========= OUTPUT =========\n{rep}\n==========================");

  let no_features = rep.failure_reports[0]
  .tests
  .get(
   &TestVariant ::former()
  .optimization(Optimization ::Debug)
  .channel(Channel ::Stable)
  .features(BTreeSet ::default())
  .form(),
 )
  .unwrap();

  assert!(no_features.clone().unwrap_err().out.contains("error") && no_features.clone().unwrap_err().out.contains("achtung"));
}

#[ test ]
// if there are 3 members in the workspace (two of them pass the tests and one of them fails) => the global report will contain 2 successful reports and 1 defeats
fn call_from_workspace_root() 
{
  let temp = TempDir ::new().unwrap();
  let temp = &temp;

  let fail_project = ProjectBuilder ::new("fail_test")
  .toml_file("[features]\nenabled = []")
  .test_file(
   r"
  #[ test ]
  fn should_fail123() 
  {
  panic!()
 }
  ",
 );

  let pass_project = ProjectBuilder ::new("apass_test")
  .toml_file("[features]\nenabled = []")
  .test_file(
   r"
  #[ test ]
  fn should_pass() 
  {
  assert_eq!(1,1);
 }
  ",
 );

  let pass_project2 = ProjectBuilder ::new("pass_test2")
  .toml_file("[features]\nenabled = []")
  .test_file(
   r"
  #[ test ]
  fn should_pass() 
  {
  assert_eq!(1,1);
 }
  ",
 );

  let workspace = WorkspaceBuilder ::new()
  .member(fail_project)
  .member(pass_project)
  .member(pass_project2)
  .build(temp);

  // from workspace root
  let abs = AbsolutePath ::try_from(workspace.clone()).unwrap();

  let args = TestsCommandOptions ::former()
  .dir(abs)
  .concurrent(1u32)
  .channels([Channel ::Stable])
  .optimizations([optimization ::Optimization ::Debug])
  .with_none_features(true)
  .form();

  let rep = test(args, false);
  let rep = rep.unwrap_err().0;

  assert_eq!(rep.failure_reports.len(), 1);
  assert_eq!(rep.success_reports.len(), 2);
}

#[ test ]
fn plan()
{
  let temp = TempDir ::new().unwrap();
  let temp = &temp;

  let project = ProjectBuilder ::new("plan_test")
  .toml_file("[features]\nenabled = []")
  .test_file(
   r"
  #[ test ]
  fn should_pass()
  {
  assert!(true);
 }
  ",
 )
  .build(temp)
  .unwrap();
  let abs = AbsolutePath ::try_from(project).unwrap();

  // Check available toolchains to avoid environmental dependency
  let available = available_channels(&abs).unwrap();
  let nightly_available = available.contains(&Channel ::Nightly);
  let mut channels_to_test = available;
  channels_to_test.clear();
  channels_to_test.insert(Channel ::Stable);
  if nightly_available
  {
    channels_to_test.insert(Channel ::Nightly);
 }

  let args = TestsCommandOptions ::former()
  .dir(abs)
  .channels(channels_to_test.clone())
  .optimizations([Optimization ::Debug, Optimization ::Release])
  .with_none_features(true);

  #[ cfg( feature = "progress_bar" ) ]
  let args = args.with_progress(false);

  let args = args.form();

  let rep = test(args, true).unwrap().success_reports[0].clone().tests;

  // Always check stable variants
  assert!(rep
  .contains_key(
   &TestVariant ::former()
  .optimization(Optimization ::Debug)
  .channel(Channel ::Stable)
  .features(BTreeSet ::default())
  .form()
 ));
  assert!(rep
  .contains_key(
   &TestVariant ::former()
  .optimization(Optimization ::Release)
  .channel(Channel ::Stable)
  .features(BTreeSet ::default())
  .form()
 ));

  // Only check nightly variants if nightly is available
  if nightly_available
  {
    assert!(rep
    .contains_key(
     &TestVariant ::former()
    .optimization(Optimization ::Debug)
    .channel(Channel ::Nightly)
    .features(BTreeSet ::default())
    .form()
   ));
    assert!(rep
    .contains_key(
     &TestVariant ::former()
    .optimization(Optimization ::Release)
    .channel(Channel ::Nightly)
    .features(BTreeSet ::default())
    .form()
   ));
 }
}

#[ test ]
fn backtrace_should_be() 
{
  let temp = TempDir ::new().unwrap();
  let temp = &temp;

  let project = ProjectBuilder ::new("fail_build")
  .toml_file("[features]\nenabled = []")
  .test_file(
   r"
  #[ test ]
  fn fail() 
  {
  assert!(false);
 }
  ",
 )
  .build(temp)
  .unwrap();
  let abs = AbsolutePath ::try_from(project).unwrap();

  let args = TestsCommandOptions ::former()
  .dir(abs)
  .channels([Channel ::Stable])
  .optimizations([Optimization ::Debug])
  .with_none_features(true)
  .form();

  let rep = test(args, false).unwrap_err().0;
  println!("========= OUTPUT =========\n{rep}\n==========================");

  let no_features = rep.failure_reports[0]
  .tests
  .get(
   &TestVariant ::former()
  .optimization(Optimization ::Debug)
  .channel(Channel ::Stable)
  .features(BTreeSet ::default())
  .form(),
 )
  .unwrap();

  assert!(!no_features.clone().unwrap_err().out.contains("RUST_BACKTRACE"));
  assert!(no_features.clone().unwrap_err().out.contains("stack backtrace"));
}
