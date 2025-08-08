use former::Former;

#[ derive( Debug, PartialEq, Former ) ]
// #[ debug ] // Commented out - debug attribute only for temporary debugging
pub struct Test<'a> {
    pub value: &'a str,
}

fn main() {}