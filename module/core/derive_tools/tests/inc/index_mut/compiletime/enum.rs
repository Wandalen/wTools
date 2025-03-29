use derive_tools::IndexMut;

#[ derive( IndexMut ) ] 
enum Enum< T >
{
  Nothing,
  #[ index ]
  IndexVector( Vec< T > )  
}

fn main()
{  
}
