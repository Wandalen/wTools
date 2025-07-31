use former::Former;

// xxx : Re-enable when trailing comma issue is fully fixed in macro_tools::generic_params::decompose

// #[derive(Former)]

#[derive()]
enum TestEnum {
  #[subform_scalar] // This should cause a compile error
  MyUnit,
}
fn main() {}