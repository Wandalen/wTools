#[cfg(feature = "enabled")]
use former_meta::Former;

#[derive(Debug, PartialEq, Former)]
#[debug]
pub struct Test<'a> {
  data: &'a str,
}

fn main() {
  println!("This won't compile, but we can see the debug output");
}