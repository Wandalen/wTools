mod private
{

  use std::borrow::{ Cow };

  use crate::*;

  pub fn wrap_text< 'data >
  (
    data: &'data Vec< Vec< Cow< 'data, str > > >,
    _limit: usize
  ) -> Vec< Vec< Cow< 'data, str > > >
  {
    let mut new_data = Vec::new();

    for row in data
    {
      let unwrapped_text : Vec< Vec< Cow< 'data, str > > > = row.iter().map( |c| string::lines( c.as_ref() ).map( Cow::from ).collect() ).collect();

      let max_rows = unwrapped_text.iter().map( Vec::len ).max().unwrap_or(0);
      
      let mut transposed : Vec< Vec< Cow< 'data, str > > > = Vec::new();

      for i in 0..max_rows
      {
        let mut row_vec : Vec< Cow< 'data, str > > = Vec::new();

        for col_lines in &unwrapped_text
        {
          if col_lines.len() > i
          {
            row_vec.push( col_lines[ i ].clone() );
          }
          else
          {
            row_vec.push( Cow::from( "" ) );
          }
        }

        transposed.push( row_vec );
      }

      new_data.extend( transposed );
    }

    new_data
  }
}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use own::*;

/// Own namespace of the module.
#[ allow( unused_imports ) ]
pub mod own
{
  use super::*;

  #[ doc( inline ) ]
  pub use orphan::*;
  #[ doc( inline ) ]
  pub use private::
  {
    wrap_text,
  };
}

/// Orphan namespace of the module.
#[ allow( unused_imports ) ]
pub mod orphan
{
  use super::*;
  pub use super::super::to_string_with_fallback;

  #[ doc( inline ) ]
  pub use exposed::*;

  #[ doc( inline ) ]
  pub use private::
  {
  };

}

/// Exposed namespace of the module.
#[ allow( unused_imports ) ]
pub mod exposed
{
  use super::*;
  #[ doc( inline ) ]
  pub use prelude::*;

  #[ doc( inline ) ]
  pub use private::
  {
  };

}

/// Prelude to use essentials: `use my_module::prelude::*`.
#[ allow( unused_imports ) ]
pub mod prelude
{
  use super::*;
}