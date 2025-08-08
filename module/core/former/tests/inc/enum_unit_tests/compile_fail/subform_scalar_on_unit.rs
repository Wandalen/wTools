use former::Former;

#[ derive( Former ) ]
enum TestEnum {
  #[ subform_scalar ] // This should cause a compile error
  MyUnit,
}
fn main() {}