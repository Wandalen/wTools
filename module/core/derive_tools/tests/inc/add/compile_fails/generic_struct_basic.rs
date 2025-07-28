use derive_tools::{ Add, Sub };

// // T1.5: Generic struct (should not compile)
#[derive(Add, Sub)]
pub struct GenericStruct<T> { x: T }

fn main()
{
}
