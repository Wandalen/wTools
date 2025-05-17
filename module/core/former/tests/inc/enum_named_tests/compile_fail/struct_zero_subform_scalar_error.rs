#[ derive( Debug, PartialEq, former::Former ) ]
pub enum EnumWithNamedFields
{
  // S0.5: Zero-field struct variant with #[subform_scalar] (expected compile error)
  #[ subform_scalar ]
  VariantZeroSubformScalar {},
}

fn main() {} // Required for trybuild