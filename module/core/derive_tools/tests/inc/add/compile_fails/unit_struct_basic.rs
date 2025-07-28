use derive_tools::{ Add, Sub };

// T1.3: Unit struct (should not compile)
#[derive(Add, Sub)]
pub struct UnitStruct;

fn main()
{
}
