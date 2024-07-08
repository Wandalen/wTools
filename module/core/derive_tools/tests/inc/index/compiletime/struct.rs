use derive_tools::Index;

#[ derive( Index ) ] 
struct StructMultipleNamed< T > 
{
  #[ index ]
  a: Vec< T >,
  #[ index ]
  b : Vec< T >,
}

fn main()
{  
}
