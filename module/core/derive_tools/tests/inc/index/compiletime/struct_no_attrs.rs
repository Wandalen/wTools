use derive_tools::Index;

#[ derive( Index ) ] 
#[ index () ]
struct StructMultipleNamed< T > 
{
  a : Vec< T >,
  b : Vec< T >,
}

fn main()
{  
}

