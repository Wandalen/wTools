use derive_tools::{ Add, Sub };

// // T1.4: Named struct with String (should not compile)
#[derive(Add, Sub)]
pub struct StringStruct { x: String }

fn main()
{
}
