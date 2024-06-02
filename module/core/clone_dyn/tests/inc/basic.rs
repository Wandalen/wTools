//
// #[ allow( unused_imports ) ]
// use super::*;
//
// #[ test ]
// fn basic()
// {
//   use the_module::clone_dyn;
//
//   #[ clone_dyn ]
//   trait Trait1
//   {
//     fn val( &self ) -> i32;
//   }
//
//   impl Trait1 for i32
//   {
//     fn val( &self ) -> i32
//     {
//       self.clone()
//     }
//   };
//   impl Trait1 for i64
//   {
//     fn val( &self ) -> i32
//     {
//       self.clone().try_into().unwrap()
//     }
//   };
//
//   // impl PartialEq< Box< dyn Trait1 > > for Box< dyn Trait1 >
//   // {
//   //   fn eq( &self, other : &Box< dyn Trait1 > ) -> bool
//   //   {
//   //     self == other
//   //   }
//   // }
//
//   //
//
//   let vec : Vec< Box< dyn Trait1 > > = vec![ Box::new( 13 ), Box::new( 14 ) ];
//   let vec2 = vec.clone();
//
//   a_id!( vec.iter().map( | e | e.val() ).collect::< Vec< _ > >(), vec2.iter().map( | e | e.val() ).collect::< Vec< _ > >() )
//
//   // xxx2 : continue
//
// }
