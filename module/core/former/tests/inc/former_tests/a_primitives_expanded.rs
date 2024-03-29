#[ allow( unused_imports ) ]
use super::*;

#[ derive( Debug, PartialEq ) ]
// #[ derive( Debug, PartialEq, the_module::Former ) ] #[ debug ]
pub struct Struct1
{
  pub int_1 : i32,
  string_1 : String,
  int_optional_1 : core::option::Option< i32 >,
  string_optional_1 : Option< String >,
}

// = generated

// #[ automatically_derived ]
// impl Struct1
// {
//   #[ doc = r"" ]
//   #[ doc = r" Make former, variation of builder pattern to form structure defining values of fields step by step." ]
//   #[ doc = r"" ]
//   #[ inline( always ) ]
//   // pub fn former() -> Struct1Former< (), former::ReturnPreformed >
//   pub fn former() -> Struct1Former<>
//   {
//     Struct1Former::new()
//   }
// }
//
// #[ derive( Debug, Default ) ]
// pub struct Struct1FormerDefinition;
//
// impl former::FormerDefinitionTypes for Struct1FormerDefinition
// {
//   type Storage = Struct1FormerStorage;
//   type Formed = Struct1;
//   type Context = ();
// }
//
// impl former::FormerDefinition for Struct1FormerDefinition
// {
//   type Types = Struct1FormerDefinition;
//   type End = former::ReturnPreformed;
// }
//
// #[ doc = "Container of a corresponding former." ]
// pub struct Struct1FormerStorage
// {
//   #[ doc = r" A field" ]
//   pub int_1 : ::core::option::Option< i32 >,
//   #[ doc = r" A field" ]
//   pub string_1 : ::core::option::Option< String >,
//   #[ doc = r" A field" ]
//   pub int_optional_1 : core::option::Option< i32 >,
//   #[ doc = r" A field" ]
//   pub string_optional_1 : Option< String >,
// }
//
// impl ::core::default::Default for Struct1FormerStorage
// {
//   #[ inline( always ) ]
//   fn default() -> Self
//   {
//     Self
//     {
//       int_1 : ::core::option::Option::None,
//       string_1 : ::core::option::Option::None,
//       int_optional_1 : ::core::option::Option::None,
//       string_optional_1 : ::core::option::Option::None,
//     }
//   }
// }
//
// impl former::Storage for Struct1FormerStorage
// {
//   // type Definition = Struct1FormerDefinition;
//   type Formed = Struct1;
// }
//
// impl former::StoragePerform for Struct1FormerStorage
// {
//   fn preform( mut self ) -> < Struct1FormerDefinition as former::FormerDefinitionTypes >::Formed
//   {
//     let int_1 = if self.int_1.is_some()
//     {
//       self.int_1.take().unwrap()
//     }
//     else
//     {
//       {
//         trait MaybeDefault< T >
//         {
//           fn maybe_default( self : &Self ) -> T
//           {
//             panic!( "Field 'int_1' isn't initialized" )
//           }
//         }
//         impl< T > MaybeDefault< T > for &::core::marker::PhantomData< T > {}
//         impl< T > MaybeDefault< T > for ::core::marker::PhantomData< T >
//         where T : ::core::default::Default,
//         {
//           fn maybe_default( self : &Self ) -> T
//           {
//             T::default()
//           }
//         }
//         ( &::core::marker::PhantomData::< i32 > ).maybe_default()
//       }
//     };
//     let string_1 = if self.string_1.is_some()
//     {
//       self.string_1.take().unwrap()
//     }
//     else
//     {
//       {
//         trait MaybeDefault< T >
//         {
//           fn maybe_default( self : &Self ) -> T
//           {
//             panic!( "Field 'string_1' isn't initialized" )
//           }
//         }
//         impl< T > MaybeDefault< T > for &::core::marker::PhantomData< T > {}
//         impl< T > MaybeDefault< T > for ::core::marker::PhantomData< T >
//         where T : ::core::default::Default,
//         {
//           fn maybe_default( self : &Self ) -> T
//           {
//             T::default()
//           }
//         }
//         ( &::core::marker::PhantomData::< String > ).maybe_default()
//       }
//     };
//     let int_optional_1 = if self.int_optional_1.is_some()
//     {
//       ::core::option::Option::Some( self.int_optional_1.take().unwrap() )
//     }
//     else
//     {
//       ::core::option::Option::None
//     };
//     let string_optional_1 = if self.string_optional_1.is_some()
//     {
//       ::core::option::Option::Some( self.string_optional_1.take().unwrap() )
//     }
//     else
//     {
//       ::core::option::Option::None
//     };
//     let result = Struct1
//     {
//       int_1,
//       string_1,
//       int_optional_1,
//       string_optional_1,
//     };
//     return result;
//   }
// }
//
// pub struct Struct1Former
// <
//   // FormerContext = Struct1,
//   // FormerEnd = former::ReturnPreformed,
// >
// // where
//   // FormerEnd : former::FormingEnd< Struct1FormerDefinition >,
// {
//   storage : Struct1FormerStorage,
//   context : core::option::Option< FormerContext >,
//   on_end : core::option::Option< FormerEnd >,
// }
//
// #[ automatically_derived ]
// impl< FormerContext, FormerEnd > Struct1Former< FormerContext, FormerEnd >
// // where
//   // FormerEnd : former::FormingEnd< Struct1FormerDefinition >,
// {
//   #[ doc = r"" ]
//   #[ doc = r" Finish setting options and return formed entity." ]
//   #[ doc = r"" ]
//   #[ inline( always ) ]
//   pub fn preform( self ) -> < Struct1FormerDefinition as former::FormerDefinitionTypes >::Formed
//   {
//     < Struct1FormerStorage as former::StoragePerform >::preform( self.storage )
//   }
//
//   #[ inline( always ) ]
//   pub fn perform( self ) -> < Struct1FormerDefinition as former::FormerDefinitionTypes >::Formed
//   {
//     let result = self.form();
//     return result;
//   }
//   #[ doc = r"" ]
//   #[ doc = r" Begin the process of forming. Expects context of forming to return it after forming." ]
//   #[ doc = r"" ]
//   #[ inline( always ) ]
//   pub fn begin( mut storage : core::option::Option< Struct1FormerStorage >, context : core::option::Option< FormerContext >, on_end : FormerEnd, ) -> Self
//   {
//     if storage.is_none()
//     {
//       storage = Some( ::core::default::Default::default() );
//     }
//     Self
//     {
//       storage : storage.unwrap(),
//       context : context,
//       on_end : ::core::option::Option::Some( on_end ),
//     }
//   }
//   #[ doc = r"" ]
//   #[ doc = r" End the process of forming returning original context of forming." ]
//   #[ doc = r"" ]
//   #[ inline( always ) ]
//   pub fn form( self ) -> < Struct1FormerDefinition as former::FormerDefinitionTypes >::Formed
//   {
//     self.end()
//   }
//   #[ doc = r"" ]
//   #[ doc = r" End the process of forming returning original context of forming." ]
//   #[ doc = r"" ]
//   #[ inline( always ) ]
//   pub fn end( mut self ) -> < Struct1FormerDefinition as former::FormerDefinitionTypes >::Formed
//   {
//     let on_end = self.on_end.take().unwrap();
//     let context = self.context.take();
//     // on_end.call( self.storage, context )
//     former::FormingEnd::< Struct1FormerDefinition >::call( &on_end, self.storage, context )
//   }
//   #[ doc = "Setter for the 'int_1' field." ]
//   #[ inline ]
//   pub fn int_1< Src >( mut self, src : Src ) -> Self
//   where
//     Src : ::core::convert::Into< i32 >,
//   {
//     debug_assert!( self.storage.int_1.is_none() );
//     self.storage.int_1 = ::core::option::Option::Some( src.into() );
//     self
//   }
//   #[ doc = "Setter for the 'string_1' field." ]
//   #[ inline ]
//   pub fn string_1< Src >( mut self, src : Src ) -> Self
//   where
//     Src : ::core::convert::Into< String >,
//   {
//     debug_assert!( self.storage.string_1.is_none() );
//     self.storage.string_1 = ::core::option::Option::Some( src.into() );
//     self
//   }
//   #[ doc = "Setter for the 'int_optional_1' field." ]
//   #[ inline ]
//   pub fn int_optional_1< Src >( mut self, src : Src ) -> Self
//   where
//     Src : ::core::convert::Into< i32 >,
//   {
//     debug_assert!( self.storage.int_optional_1.is_none() );
//     self.storage.int_optional_1 = ::core::option::Option::Some( src.into() );
//     self
//   }
//   #[ doc = "Setter for the 'string_optional_1' field." ]
//   #[ inline ]
//   pub fn string_optional_1< Src >( mut self, src : Src ) -> Self
//   where
//     Src : ::core::convert::Into< String >,
//   {
//     debug_assert!( self.storage.string_optional_1.is_none() );
//     self.storage.string_optional_1 = ::core::option::Option::Some( src.into() );
//     self
//   }
// }
//
// #[ automatically_derived ]
// impl Struct1Former< (), former::ReturnPreformed >
// {
//   #[ doc = r"" ]
//   #[ doc = r" Construct new instance of former with default parameters." ]
//   #[ doc = r"" ]
//   #[ inline( always ) ]
//   pub fn new() -> Self
//   {
//     Self::begin( None, None, former::ReturnPreformed, )
//   }
// }

//

//  include!( "./only_test/primitives.rs" );
