use former::Former;

#[ derive( Debug, PartialEq, Former ) ]
// #[ debug ] // Commented out - debug attribute only for temporary debugging
pub struct Child<K: core::hash::Hash + core::cmp::Eq> {
    pub name: String,
}

fn main() {
    println!("Testing Child struct compilation");
}