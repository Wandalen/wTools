use former::Former;

#[derive(Debug, PartialEq, Former)]
#[debug]
pub struct Test<'a> {
    pub value: &'a str,
}

fn main() {}