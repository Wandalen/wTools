#![allow(dead_code)]

// Minimal test to check generated code for lifetime-only struct
#[derive(Debug, PartialEq, former::Former)]
#[debug]
pub struct TestLifetime<'a> {
    data: &'a str,
}

// This should generate something like:
// impl< 'a, Definition > former::FormerBegin< 'a, Definition > for TestLifetimeFormer< Definition >

fn main() {
    println!("This should compile if the macro generates correct code");
}