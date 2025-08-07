use former::Former;

#[derive(Debug, PartialEq, Former)]
pub struct Child<K: core::hash::Hash + core::cmp::Eq> {
    pub name: String,
}

fn main() {
    let _child = Child::<&str>::former()
        .name("test")
        .form();
}