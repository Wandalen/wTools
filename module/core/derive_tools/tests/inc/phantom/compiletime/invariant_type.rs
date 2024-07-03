use derive_tools::phantom;

#[ phantom ]
struct InvariantType< T >
{
  a: T,
}

fn assert_invariant< 'a >( x: InvariantType< *mut &'static str > ) -> InvariantType< *mut &'a str >
{
  x
}

fn main()
{
  let x: InvariantType< *mut &'static str > = InvariantType { a: &mut "boo", _phantom: Default::default() };
  let _: InvariantType< *mut &str > = assert_invariant( x );
}