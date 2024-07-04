#[ test ]
fn index()
{
    let x = EnumTuple::A( true );
    let x1 = EnumTuple::B( false );
    let exp =  (true, false);
    let got =  (x[0], x1[0]);
    
    assert_eq!(got, exp);
}

