#[ allow( unused_imports ) ]
use super::*;

#[ test ]
fn basic_from()
{
  use proper_path_tools::TransitiveTryFrom;
  use std::convert::TryFrom;

  struct InitialType;
  struct IntermediateType;
  struct FinalType;
  struct ConversionError;

  impl TryFrom< InitialType > for IntermediateType
  {
    type Error = ConversionError;
    fn try_from( _value : InitialType ) -> Result< Self, Self::Error >
    {
      // Conversion logic here
      Ok( IntermediateType )
    }
  }

  impl TryFrom< IntermediateType > for FinalType
  {
    type Error = ConversionError;
    fn try_from( _value : IntermediateType ) -> Result< Self, Self::Error >
    {
      // Conversion logic here
      Ok( FinalType )
    }
  }

  // impl TransitiveTryFrom< IntermediateType, ConversionError, InitialType > for FinalType {}

  let initial = InitialType;
  let _final_result : Result< FinalType, ConversionError > = FinalType::transitive_try_from::< IntermediateType >( initial );

}

#[ test ]
fn test_transitive_try_into()
{
  use proper_path_tools::TransitiveTryInto;

  // Define NewType1 wrapping a String
  #[ derive( Debug, PartialEq ) ]
  struct NewType1( String );

  // Define NewType2 wrapping NewType1
  #[ derive( Debug, PartialEq ) ]
  struct NewType2( NewType1 );

  // Define an error type for conversion
  #[ derive( Debug, PartialEq ) ]
  struct ConversionError;

  // Implement TryInto for converting String to NewType1
  impl TryInto< NewType1 > for String
  {
    type Error = ConversionError;
    fn try_into( self ) -> Result< NewType1, Self::Error >
    {
      Ok( NewType1( self ) )
    }
  }

  // Implement TryInto for converting NewType1 to NewType2
  impl TryInto< NewType2 > for NewType1
  {
    type Error = ConversionError;
    fn try_into( self ) -> Result< NewType2, Self::Error >
    {
      Ok( NewType2( self ) )
    }
  }

  let initial = String::from( "Hello, world!" );
  let final_result : Result< NewType2, ConversionError > = initial.transitive_try_into::< NewType1 >();
  assert_eq!( final_result, Ok( NewType2( NewType1( String::from( "Hello, world!" ) ) ) ) );

  let initial = String::from( "Hello, world!" );
  let _final_result : NewType2 = initial.transitive_try_into::< NewType1 >().unwrap();

}
