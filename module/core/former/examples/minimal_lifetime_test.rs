// This example tests Former with a minimal lifetime struct.

use former::Former;

#[derive(Debug, Former)]
pub struct Minimal<'a> {
  data: &'a str,
}

fn main() {
  let s = "hello";
  let instance = Minimal::former().data(s).form();
  println!("{:?}", instance);
}