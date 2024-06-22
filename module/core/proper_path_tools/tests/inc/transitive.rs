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

  impl TransitiveTryFrom< IntermediateType, ConversionError, InitialType > for FinalType {}

  let initial = InitialType;
  let _final_result : Result< FinalType, ConversionError > = FinalType::transitive_try_from( initial );

}
