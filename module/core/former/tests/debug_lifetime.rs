#![allow(dead_code)]

use former::Former;

#[derive(Debug, PartialEq, Former)]
#[debug]
pub struct Simple<'a> {
    data: &'a str,
}

fn main() {
    let s = "test";
    let _instance = Simple::former().data(s).form();
}