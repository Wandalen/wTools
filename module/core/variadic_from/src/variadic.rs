/// Trait for converting from one argument.
pub trait From1<T1>
where
  Self: Sized,
{
  /// Converts from one argument.
  fn from1(a1: T1) -> Self;
}

/// Trait for converting from two arguments.
pub trait From2<T1, T2>
where
  Self: Sized,
{
  /// Converts from two arguments.
  fn from2(a1: T1, a2: T2) -> Self;
}

/// Trait for converting from three arguments.
pub trait From3<T1, T2, T3>
where
  Self: Sized,
{
  /// Converts from three arguments.
  fn from3(a1: T1, a2: T2, a3: T3) -> Self;
}

/// Macro to construct a struct from variadic arguments.
#[macro_export]
macro_rules! from {
  () => {
    core::default::Default::default()
  };
  ( $a1 : expr ) => {
    ::variadic_from::variadic::From1::from1($a1)
  };
  ( $a1 : expr, $a2 : expr ) => {
    ::variadic_from::variadic::From2::from2($a1, $a2)
  };
  ( $a1 : expr, $a2 : expr, $a3 : expr ) => {
    ::variadic_from::variadic::From3::from3($a1, $a2, $a3)
  };
  ( $( $rest : expr ),* ) => {
    compile_error!("Too many arguments");
  };
}
