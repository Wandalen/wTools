// /// Internal namespace.
// pub( crate ) mod private
// {
//   use crate::error::BasicError;
//
//   /// Type alias for Result with BasicError.
//   pub type Result< T, E = BasicError > = std::result::Result< T, E >;
// }
//
// /// Protected namespace of the module.
// pub mod protected
// {
//   #[ doc( inline ) ]
//   #[ allow( unused_imports ) ]
//   pub use orphan::*;
// }
//
// #[ doc( inline ) ]
// #[ allow( unused_imports ) ]
// pub use protected::*;
//
// /// Shared with parent namespace of the module
// pub mod orphan
// {
//   #[ doc( inline ) ]
//   #[ allow( unused_imports ) ]
//   pub use exposed::*;
// }
//
// /// Exposed namespace of the module.
// pub mod exposed
// {
//   #[ doc( inline ) ]
//   #[ allow( unused_imports ) ]
//   pub use prelude::*;
// }
//
// /// Prelude to use essentials: `use my_module::prelude::*`.
// pub mod prelude
// {
//   pub use private::Result;
// }
//
