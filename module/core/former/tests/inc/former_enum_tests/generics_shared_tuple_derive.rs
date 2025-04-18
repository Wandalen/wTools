// File: module/core/former/tests/inc/former_enum_tests/generics_shared_tuple_derive.rs
use super::*; // Imports testing infrastructure and potentially other common items

// --- Dummy Bounds ---
// Defined in _only_test.rs, but repeated here conceptually for clarity
// pub trait BoundA : core::fmt::Debug + Default + Clone + PartialEq {}
// pub trait BoundB : core::fmt::Debug + Default + Clone + PartialEq {}

// --- Inner Struct Definition with Bounds ---
// Needs to derive Former for the enum's derive to work correctly for subforming.
#[ derive( Debug, Clone, Default, PartialEq, former::Former ) ]
pub struct InnerG3< T : BoundB > // BoundB required by the inner struct
{
  pub inner_field : T,
}

// --- Enum Definition with Bounds ---
// Apply Former derive here. This is what we are testing.
#[ derive( Debug, PartialEq, Clone, former::Former ) ]
// #[ derive( Debug, PartialEq, Clone ) ]
#[ debug ] // Uncomment to see generated code later
pub enum EnumG3< T : BoundA + BoundB > // BoundA required by enum, BoundB required by InnerG3<T>
{
  V1( InnerG3< T > ), // Inner type uses T
}

// // xxx
//   #[automatically_derived] impl < T : BoundA + BoundB, > EnumG3 < T, > where
//   {
//       #[doc = r" Starts forming the #variant_ident variant using a subformer."]
//       #[inline(always)] pub fn v_1 < T : BoundA + BoundB, > () -> InnerG3Former
//       < T, InnerG3FormerDefinition < T, (), EnumG3 < T, > , EnumG3V1End < T, > >
//       > where
//       { InnerG3Former :: begin(None, None, EnumG3V1End :: < T, > :: default()) }
//   } #[derive(Default, Debug)] pub struct EnumG3V1End < T : BoundA + BoundB, >
//   where { _phantom : :: core :: marker :: PhantomData < (* const T,) > , }
//   #[automatically_derived] impl < T : BoundA + BoundB, > former :: FormingEnd <
//   InnerG3FormerDefinitionTypes < T, (), EnumG3 < T, > > > for EnumG3V1End < T, >
//   where
//   {
//       #[inline(always)] fn
//       call(& self, sub_storage : InnerG3FormerStorage < T > , _context : Option
//       < () > ,) -> EnumG3 < T, >
//       {
//           let data = former :: StoragePreform :: preform(sub_storage); EnumG3 ::
//           V1(data)
//       }
//   }
// // xxx

// --- Include the Test Logic ---
// This file contains the actual #[test] functions.
include!( "generics_shared_tuple_only_test.rs" );

// qqq : xxx : enable