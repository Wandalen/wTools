#[ test ]
fn index()
{
    let x = EnumNamed::A{ a : vec![ 44, 12, 9 ]};
    let exp = ( 44, 9 );
    let got = ( x[0], x[2] );

    assert_eq!(got, exp);
}
