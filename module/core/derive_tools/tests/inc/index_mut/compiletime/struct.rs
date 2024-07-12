use derive_tools::IndexMut;

#[ derive( IndexMut ) ] 
struct StructMultipleNamed< T > 
{
  #[ index ]
  a : Vec< T >,
  #[ index ]
  b : Vec< T >,
}

fn main()
{  
}
