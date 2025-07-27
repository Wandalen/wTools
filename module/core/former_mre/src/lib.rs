use former::Former;

pub trait Bound {}

#[derive(Debug, PartialEq, Clone)]
pub struct MyType(String);
impl Bound for MyType {}

#[derive(Debug, PartialEq, Clone)]
pub struct InnerScalar<T: Bound> {
    pub data: T,
}

#[derive(Debug, PartialEq, Clone, Former)]
pub enum EnumScalarGeneric<T: Bound> {
    #[scalar]
    Variant1(InnerScalar<T>),
}
