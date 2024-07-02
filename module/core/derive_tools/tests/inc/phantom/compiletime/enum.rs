use derive_tools_meta::phantom;

#[ phantom ]
enum Enum< T >
{
  A,
  B,
  C( T ),
}

fn main()
{
}