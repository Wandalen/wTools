#![allow(dead_code)]

use former::Former;

#[derive(Debug, PartialEq, Former)]
// #[debug] // Commented out - debug attribute only for temporary debugging
pub struct Minimal<'a> {
    value: &'a str,
}

fn main() {
    let data = "test";
    let instance = Minimal::former().value(data).form();
    assert_eq!(instance.value, "test");
}