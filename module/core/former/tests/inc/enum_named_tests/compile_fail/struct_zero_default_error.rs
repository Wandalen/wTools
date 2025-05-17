#[ derive( Debug, PartialEq, former::Former ) ]
pub enum EnumWithNamedFields
{
  // S0.1: Zero-field struct variant with Default behavior (expected compile error)
  VariantZeroDefault {},
}

fn main() {} // Required for trybuild