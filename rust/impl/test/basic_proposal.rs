struct TestCase
{
  name: String,
  test_fn: fn(),
}

struct TestRoutine
{
  name: String,
  cases: Vec<TestCase>,
}

macro_rules! test_case
{
  ( $name : expr => { $( $tree : tt )* } ) =>
  {
    TestCase
    {
      name : stringify!( $name ).to_owned(),
      test_fn : ||
      {
        $( $tree )*
      },
    }
  };
}

macro_rules! test_routine
{
  ( $name : ident => { $( $cases : expr )* } ) =>
  {
    fn $name()
    {
      let mut routine = TestRoutine
      {
        name : stringify!( $name ).to_owned(),
        cases : vec![],
      };
      $( routine.cases.push( $cases ); )*
      println!( "Test routine::{}", routine.name );
      for case in routine.cases
      {
        println!( "Test case::{}", case.name );
        ( case.test_fn )();
      }
    }
  };
}

macro_rules! test_suite
{
  ( $name : ident => [ $( $routines : ident ),* $( , )? ] $( , )? ) =>
  {
    mod $name
    {
      $(
        #[test]
        fn $routines()
        {
          super::$routines();
        }
      )*
    }
  };
}

/// # Sample
///
/// ```rust
/// test_routine!{ equal_ints =>
/// {
///   test_case!{ "equal ints" =>
///   {
///     assert_eq!( 1, 1 );
///   }}
///
///   test_case!{ "equal bools" =>
///   {
///     assert_eq!( true, true );
///   }}
/// }}
///
/// test_routine!{ equal_bools =>
/// {
///   test_case!{ "equal ints" =>
///   {
///     assert_eq!( 1, 2 );
///   }}
///
///   test_case!{ "equal bools" =>
///   {
///     assert_eq!( true, true );
///   }}
/// }}
///
/// test_suite!
/// {
///   simple =>
///   [
///     equal_ints,
///     equal_bools,
///   ]
/// }
/// ```
