#[ derive( Debug, PartialEq, former::Former ) ]
pub struct Test<'a> {
    value: &'a str,
}