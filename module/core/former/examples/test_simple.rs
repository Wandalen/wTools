use former::Former;

#[derive(Debug, PartialEq, Former)]
#[debug]
pub struct Simple {
    value: i32,
}

fn main() {
    println!("Testing simple Former");
}