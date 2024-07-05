use derive_tools::phantom;

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