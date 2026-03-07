use super :: *;
use test_tools::a_id;

//

//

//

//

tests_impls!
{

  //

  #[allow(unused_variables)]
  #[allow(unused_assignments)]
  fn basic()
  {
  let mut a = 0;

  // Environment variables for path resolution
  // xxx: add to program_tools :: { path ::modules(), path ::workspace() }
  // Note: MODULES_PATH and WORKSPACE_PATH are optional compile-time env vars

  macro_rules! macro1
  {
   ( $Number: tt ) =>
   {
  a = 13;
  // let xy3_ = 13;
  the_module ::meta_idents_concat!
  {
   let [< x $Number _ >] = 13;
 };
  a_id!( xy3_, a );
 };
 }

  macro1!( y3 );
  a_id!( a, 13 );

 }

}

//

tests_index!
{
  basic,
}
