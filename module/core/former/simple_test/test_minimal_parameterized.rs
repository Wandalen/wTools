use former::Former;

#[ derive( Debug, PartialEq, Former ) ]
pub struct Test<T> {
    pub value: T,
}

fn main() {
    println!("Testing minimal parameterized struct");
}