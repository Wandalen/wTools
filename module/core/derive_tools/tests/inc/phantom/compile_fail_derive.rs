use the_module::PhantomData;

#[ derive( PhantomData ) ]
struct MyStruct;

#[ derive( PhantomData ) ]
enum MyEnum
{
  Variant1,
  Variant2,
}

#[ derive( PhantomData ) ]
union MyUnion
{
  field1 : u32,
  field2 : f32,
}