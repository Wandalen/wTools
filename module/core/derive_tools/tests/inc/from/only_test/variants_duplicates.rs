#[ allow( unused_imports ) ]
use super::*;

#[ test ]
fn variant_from_duplicates()
{

  let got : GetData = From::from( &b"abc"[ .. ] );
  let exp = GetData::FromBin( b"abc" );
  a_id!( got, exp );

  let got : GetData = From::from( "abc".to_string() );
  let exp = GetData::FromString2( "abc".to_string() );
  a_id!( got, exp );

  let got : GetData = From::from( ( "a".to_string(), "b".to_string() ) );
  let exp = GetData::FromPair2( "a".to_string(), "b".to_string() );
  a_id!( got, exp );

}
