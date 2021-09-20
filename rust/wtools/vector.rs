
#![ allow( non_camel_case_types ) ]

use std::cmp::PartialEq;

//

pub fn append_vectors_once<'a, T : PartialEq + Copy, F : FnMut(T)>( dst : &'a mut Vec<T>, src : &'a Vec<Vec<T>>, on_evaluate : Option<F> ) -> &'a mut Vec<T>
{
  if on_evaluate.is_none()
  {
    for vec in src
    {
      for el in vec
      {
        if !dst.contains( el )
        {
          dst.push( *el );
        }
      }
    }
  }
  else
  {
    unimplemented!( "callbacks is not implemented" );
  }

  dst
}

//

pub struct left_index<T>
{
  src : Vec<T>,
  ins : T,
  start_from : usize,
  on_evaluate1 : Option<fn(&T) -> T>,
  on_evaluate2 : Option<fn(&T) -> T>,
  on_equalize : Option<fn(&T, &T) -> bool>,
  formed : u8,
}

//

impl<T : std::default::Default> Default for left_index<T>
{
  fn default() -> Self
  {
    let opts = Self
    {
      src : vec![],
      ins : Default::default(),
      start_from : 0,
      on_evaluate1 : None,
      on_evaluate2 : None,
      on_equalize : None,
      formed : 0,
    };
    return opts;
  }
}

impl<T : PartialEq + Copy +  Clone> left_index<T>
{
  pub fn src( &mut self, src : Vec<T> ) -> &mut Self
  {
    assert!( self.formed == 0, "Context is already formed" );
    self.src = src;
    self
  }

  //

  pub fn ins( &mut self, ins : T ) -> &mut Self
  {
    assert!( self.formed == 0, "Context is already formed" );
    self.ins = ins;
    self
  }

  //

  pub fn start_from( &mut self, start_from : usize ) -> &mut Self
  {
    assert!( self.formed == 0, "Context is already formed" );
    self.start_from = start_from;
    self
  }

  //

  pub fn on_equalize( &mut self, on_equalize : fn( &T, &T ) -> bool ) -> &mut Self
  {
    assert!( self.on_evaluate1.is_none() && self.on_evaluate2.is_none(), "Expects no evaluation callbacks." );
    assert!( self.formed == 0, "Context is already formed" );
    self.on_equalize = Some( on_equalize );
    self
  }

  //

  pub fn on_evaluate1( &mut self, on_evaluate1 : fn( &T ) -> T ) -> &mut Self
  {
    assert!( self.on_equalize.is_none(), "Expects no equalizer callback." );
    assert!( self.formed == 0, "Context is already formed" );
    self.on_evaluate1 = Some( on_evaluate1 );
    self
  }

  //

  pub fn on_evaluate2( &mut self, on_evaluate2 : fn( &T ) -> T ) -> &mut Self
  {
    assert!( self.on_equalize.is_none(), "Expects no equalizer callback." );
    assert!( self.formed == 0, "Context is already formed" );
    self.on_evaluate2 = Some( on_evaluate2 );
    self
  }

  //

  pub fn form( self ) -> Self
  {
    assert!( self.formed == 0, "Context is already formed" );
    Self { formed : 1, .. self }
  }

  //

  pub fn call( &self ) -> Option<usize>
  {
    if self.on_equalize.is_some()
    {
      let mut equalizer = self.on_equalize.unwrap();
      for i in 0..self.src.len()
      {
        if equalizer( &self.src[ i ], &self.ins )
        {
          return Some( i );
        }
      }
    }
    else if self.on_evaluate1.is_some()
    {
      let mut ins;
      let evaluate1 = self.on_evaluate1.unwrap();

      if self.on_evaluate2.is_some()
      {
        let evaluate2 = self.on_evaluate2.unwrap();
        ins = evaluate2( &self.ins );
      }
      else
      {
        ins = evaluate1( &self.ins );
      }

      for i in self.start_from..self.src.len()
      {
        if evaluate1( &self.src[ i ] ) == ins
        {
          return Some( i );
        }
      }
    }
    else
    {
      assert!( self.on_evaluate2.is_none(), "Expects callback {-on_evaluate1-}." );
      return self.src.iter().position( | &x | x == self.ins );
    }

    None
  }
}

