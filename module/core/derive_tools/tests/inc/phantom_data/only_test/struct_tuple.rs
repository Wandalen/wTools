#[ test ]
fn phantom_data()
{
    let _ = StructTuple::< bool >( "boo".into(), 3, Default::default() );
}
