#[ allow( unused_imports ) ]
use super::*;

#[ test ]
fn variant_from()
{

  let got : GetData< '_, str > = From::from( "abc" );
  let exp = GetData::< '_, str >::FromT( "abc" );
  a_id!( got, exp );

}
