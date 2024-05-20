#[ allow( unused_imports ) ]
use super::*;

#[ test ]
fn variant_from()
{

  let got : GetData = From::from( "abc".to_string() );
  let exp = GetData::FromString( "abc".to_string() );
  a_id!( got, exp );

  let got : GetData = From::from( ( "a".to_string(), "b".to_string() ) );
  let exp = GetData::FromPair( "a".to_string(), "b".to_string() );
  a_id!( got, exp );

  let got : GetData = From::from( &b"abc"[ .. ] );
  let exp = GetData::FromBin( b"abc" );
  a_id!( got, exp );

}
