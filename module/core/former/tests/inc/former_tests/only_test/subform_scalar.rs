
#[ test ]
fn subforme_scalar()
{

  let got = Parent::former()
  .child().name( "a" ).data( true ).end()
  .form();

  let exp = Parent { child : Child { name : "a".to_string(), data : true } };
  a_id!( got, exp );

}
