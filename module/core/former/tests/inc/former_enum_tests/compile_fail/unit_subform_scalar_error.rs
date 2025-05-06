use former::Former; // Add import

#[derive(Former)] // Use #[derive(Former)]
enum MyEnum {
    #[subform_scalar] // Use #[subform_scalar] directly
    MyUnitVariant, // This should cause a compile error
}

fn main() {} // Added empty main function