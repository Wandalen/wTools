#[ test ]
fn index()
{
    let x = EnumNamed::A{ a : false, b : true };
    let exp = ( false, true );
    let got = ( x[0], x[1] );

    assert_eq!(got, exp);
}
