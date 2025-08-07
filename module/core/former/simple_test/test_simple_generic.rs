use former::Former;

#[derive(Debug, PartialEq, Former)]
pub struct Test<T> {
    pub value: T,
}

fn main() {
    let test = Test::<i32>::former()
        .value(42)
        .form();
    println!("Test value: {}", test.value);
}