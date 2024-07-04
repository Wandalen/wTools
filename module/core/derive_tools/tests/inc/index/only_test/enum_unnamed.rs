#[ test ]
fn index()
{
    let x = Enum::IndexVector(vec![12, 55]);
    let exp =  (12, 55);
    let got =  (x[0], x[1]);
    
    assert_eq!(got, exp);
}

