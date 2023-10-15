use super::*;

tests_impls!
{

  //

  fn basic()
  {
    let mut a = 0;

    macro_rules! macro1
    {
      ( $Number:tt ) =>
      {
        a = 13;
        // let xy3_ = 13;
        TheModule::idents_concat!
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
