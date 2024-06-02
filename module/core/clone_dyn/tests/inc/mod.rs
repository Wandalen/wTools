
#[ allow( unused_imports ) ]
use super::*;

//

tests_impls!
{

  //

//   // qqq2 : organize tests in the same way tests organized for derive_tools
//   fn manual()
//   {
//
//     trait Trait1
//     {
//     }
//
//     //
//
//     #[ inline ]
//     pub fn clone_into_box< T >( t : &T ) -> Box< T >
//     where
//       T : ?Sized,
//     {
//       unsafe
//       {
//         let mut ptr = t as *const T;
//         let data_ptr = &mut ptr as *mut *const T as *mut *mut ();
//         *data_ptr = Box::into_raw( Box::new( t.clone() ) ) as *mut ();
//         Box::from_raw( ptr as *mut T )
//       }
//     }
//
//     //
//
//     #[ allow( non_local_definitions ) ]
//     impl < 'c > Clone
//     for Box< dyn Trait1 + 'c >
//     {
//       #[ inline ]
//       fn clone( &self ) -> Self { clone_into_box( &**self ) }
//     }
//
//     #[ allow( non_local_definitions ) ]
//     impl < 'c > Clone
//     for Box< dyn Trait1 + Send + 'c >
//     {
//       #[ inline ]
//       fn clone( &self ) -> Self { clone_into_box( &**self ) }
//     }
//
//     #[ allow( non_local_definitions ) ]
//     impl < 'c > Clone
//     for Box< dyn Trait1 + Sync + 'c >
//     {
//       #[ inline ]
//       fn clone( &self ) -> Self { clone_into_box( &**self ) }
//     }
//
//     #[ allow( non_local_definitions ) ]
//     impl < 'c > Clone
//     for Box< dyn Trait1 + Send + Sync + 'c >
//     {
//       #[ inline ]
//       fn clone( &self ) -> Self { clone_into_box( &**self ) }
//     }
//
//     //
//
//     let vec = Vec::< Box< dyn Trait1 > >::new();
//     let vec2 = vec.clone();
//
//   }
//
//   //
//
//   fn basic()
//   {
//     use the_module::clone_dyn;
//
//     #[ clone_dyn ]
//     trait Trait1
//     {
//       fn val( &self ) -> i32;
//     }
//
//     impl Trait1 for i32
//     {
//       fn val( &self ) -> i32
//       {
//         self.clone()
//       }
//     };
//     impl Trait1 for i64
//     {
//       fn val( &self ) -> i32
//       {
//         self.clone().try_into().unwrap()
//       }
//     };
//
//     // impl PartialEq< Box< dyn Trait1 > > for Box< dyn Trait1 >
//     // {
//     //   fn eq( &self, other : &Box< dyn Trait1 > ) -> bool
//     //   {
//     //     self == other
//     //   }
//     // }
//
//     //
//
//     let vec : Vec< Box< dyn Trait1 > > = vec![ Box::new( 13 ), Box::new( 14 ) ];
//     let vec2 = vec.clone();
//
//     a_id!( vec.iter().map( | e | e.val() ).collect::< Vec< _ > >(), vec2.iter().map( | e | e.val() ).collect::< Vec< _ > >() )
//
//     // xxx2 : continue
//
//   }

  //

//   fn prelude()
//   {
//     use the_module::prelude::*;
//
//     #[ clone_dyn ]
//     trait Trait1
//     {
//     }
//
//     //
//
//     let vec = Vec::< Box< dyn Trait1 > >::new();
//     // let vec2 = vec.clone();
//     // xxx
//
//   }
//
//   //
//
//   fn parametrized()
//   {
//     use the_module::clone_dyn;
//
//     #[ clone_dyn ]
//     trait Trait2< T1 : Copy, T2 : Copy >
//     where
//       T2 : Clone + core::fmt::Debug,
//     {
//     }
//
//     //
//
//     let vec = Vec::< Box< dyn Trait2< i32, f32 > > >::new();
//     let vec2 = vec.clone();
//
//   }
//
//   //
//
//   fn sample()
//   {
//     use the_module::clone_dyn;
//
//     #[ clone_dyn ]
//     trait Trait1
//     {
//     }
//
//     let vec = Vec::< Box< dyn Trait1 > >::new();
//     let vec2 = vec.clone(); /* <- it does not work without `clone_dyn` */
//
//   }

}

//

tests_index!
{
  // manual,
  // basic,
  // prelude,
  // parametrized,
  // sample,
}
// xxx

mod basic_manual;
mod basic;
