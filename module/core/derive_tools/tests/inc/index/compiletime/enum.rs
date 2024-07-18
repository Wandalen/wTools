use derive_tools::Index;

#[ derive( Index ) ] 
enum Enum< T >
{
  Nothing,
  #[ index ]
  IndexVector( Vec< T > )  
}

fn main()
{  
}
