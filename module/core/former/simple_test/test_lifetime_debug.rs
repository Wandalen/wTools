use former::Former;

#[derive(Debug, PartialEq, Former)]
#[debug]
pub struct TestLifetime<'a> {
    pub value: &'a str,
}

fn main() {
    let data = "test";
    let _instance = TestLifetime::former()
        .value(data)
        .form();
}