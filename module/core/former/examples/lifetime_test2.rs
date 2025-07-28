use former::Former;

#[derive(Debug, PartialEq, Former)]
pub struct Other<'x> {
  data: &'x str,
}

fn main() {
  let s = "hello";
  let instance = Other::former().data(s).form();
  println!("{:?}", instance);
}