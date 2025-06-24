use super::*;

//

#[derive( Debug, PartialEq )]
pub enum GenericOption< T >
where
  T : std::fmt::Debug + PartialEq + Clone,
{
  Value( T ),
  NoValue,
}

// Manual implementation of Former
impl< T > GenericOption< T >
where
  T : std::fmt::Debug + PartialEq + Clone,
{
  #[inline(always)]
  pub fn value( _0 : impl Into< T > ) -> Self
  {
    Self::Value( _0.into() )
  }
  #[inline(always)]
  pub fn no_value() -> Self
  {
    Self::NoValue
  }
}

// Manual implementation of standalone constructors
#[inline(always)]
pub fn value< T >( _0 : impl Into< T > ) -> GenericOption< T >
where
  T : std::fmt::Debug + PartialEq + Clone,
{
  GenericOption::Value( _0.into() )
}

#[inline(always)]
pub fn no_value< T >() -> GenericOption< T >
where
  T : std::fmt::Debug + PartialEq + Clone,
{
  GenericOption::NoValue
}


include!( "generic_unit_variant_only_test.rs" );