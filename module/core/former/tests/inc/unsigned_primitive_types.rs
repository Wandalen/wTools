#[ allow( unused_imports ) ]
use super::*;

// #[ allow( unused_imports ) ]
// use test_tools::exposed::*;
//
// only_for_aggregating_module!
// {
//   #[ allow( unused_imports ) ]
//   use wtools::meta::*;
//   #[ allow( unused_imports ) ]
//   use wtools::former::Former;
// }
//
// only_for_terminal_module!
// {
//   #[ allow( unused_imports ) ]
//   use meta_tools::*;
//   #[ allow( unused_imports ) ]
//   use former::Former;
// }

//

tests_impls!
{

  fn with_u8()
  {
    #[ derive( Debug, PartialEq, TheModule::Former ) ]
    pub struct Counter
    {
      count : u8,
    }

    let counter = Counter::former()
    .count( 0 )
    .form();

    let expected = Counter
    {
      count : 0,
    };

    a_id!( counter, expected );
  }

  //

// qqq : make it working
  fn with_u16()
  {
//     #[ derive( Debug, PartialEq, TheModule::Former ) ]
//     pub struct Counter
//     {
//       count : u16,
//     }
//
//     let counter = Counter::former()
//     .count( 0 )
//     .form();
//
//     let expected = Counter
//     {
//       count : 0,
//     };
//
//     a_id!( counter, expected );
  }

  //

  fn with_u32()
  {
    // #[ derive( Debug, PartialEq, Former ) ]
    // pub struct Counter
    // {
    //   count : u32,
    // }
    //
    // let counter = Counter::former()
    // .count( 0 )
    // .form();
    //
    // let expected = Counter
    // {
    //   count : 0,
    // };
    //
    // a_id!( counter, expected );
  }

  //

  fn with_u64()
  {
    // #[ derive( Debug, PartialEq, Former ) ]
    // pub struct Counter
    // {
    //   count : u64,
    // }
    //
    // let counter = Counter::former()
    // .count( 0 )
    // .form();
    //
    // let expected = Counter
    // {
    //   count : 0,
    // };
    //
    // a_id!( counter, expected );
  }

  //

  fn with_usize()
  {
    // #[ derive( Debug, PartialEq, Former ) ]
    // pub struct Counter
    // {
    //   count : usize,
    // }
    //
    // let counter = Counter::former()
    // .count( 0 )
    // .form();
    //
    // let expected = Counter
    // {
    //   count : 0,
    // };
    //
    // a_id!( counter, expected );
  }
}

//

tests_index!
{
  with_u8,
  with_u16,
  with_u32,
  with_u64,
  with_usize,
}
