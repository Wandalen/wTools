use super::*;

#[ test ]
fn from_inner_named()
{

  let got : GetData = From::from( "abc".to_string() );
  let exp = GetData::FromString( "abc".to_string() );
  a_id!( got, exp );

  let got : GetData = From::from( &b"abc"[ .. ] );
  let exp = GetData::FromBin( b"abc" );
  a_id!( got, exp );

}
