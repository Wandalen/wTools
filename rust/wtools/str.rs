#![ allow( dead_code ) ]
#![ allow( unused_variables ) ]
#![ allow( non_camel_case_types ) ]

use regex::Regex;

pub fn regexp_is( src : &str ) -> bool
{
  let regexp = Regex::new(r"^\/.*\/[dgimsuUx]").unwrap();
  regexp.is_match( src )
}

//

// struct split;

//

#[derive(Debug, Clone)]
pub struct split_fast<'a>
{
  pub src : String,
  pub delimeter : Vec<&'a str>,
  pub preserving_empty : bool,
  pub preserving_delimeters : bool,
  formed : u8,
}

impl <'a>Default for split_fast<'a>
{
  fn default() -> Self
  {
    let opts = Self
    {
      src : String::from( "" ),
      delimeter : vec![ " " ],
      preserving_empty : true,
      preserving_delimeters : false,
      formed : 0,
    };
    opts
  }
}
impl<'a> split_fast<'a>
{
  pub fn src( &mut self, src : String ) -> &mut Self
  {
    assert!( self.formed == 0, "Context is already formed" );
    self.src = src;
    self
  }

  //

  pub fn delimeter( &mut self, delimeter : Vec<&'a str> ) -> &mut Self
  {
    assert!( self.formed == 0, "Context is already formed" );
    self.delimeter = delimeter;
    self
  }

  //

  pub fn preserving_empty( &mut self, preserving_empty : bool ) -> &mut Self
  {
    assert!( self.formed == 0, "Context is already formed" );
    self.preserving_empty = preserving_empty;
    self
  }

  //

  pub fn preserving_delimeters( &mut self, preserving_delimeters : bool ) -> &mut Self
  {
    assert!( self.formed == 0, "Context is already formed" );
    self.preserving_delimeters = preserving_delimeters;
    self
  }

  //

  pub fn form( self ) -> Self
  {
    assert!( self.formed == 0, "Context is already formed" );
    Self { formed : 1, .. self }
  }

  //

  pub fn split( o : &'a split_fast ) -> Vec<&'a str>
  {
    let mut result: Vec<&str> = vec![];

    let delimeters = &o.delimeter;

    let preserving_empty = o.preserving_empty;
    let preserving_delimeters = o.preserving_delimeters;

    let mut found_delimeters = delimeters.clone();

    if !preserving_delimeters && delimeters.len() == 1
    {
      result = o.src.split( delimeters[ 0 ] ).collect();
      if !preserving_empty
      {
        result = result.iter().filter( | x | x.len() == 0 ).map( | x | *x ).collect::<Vec<_>>();
      }
    }
    else
    {
      let src_len = o.src.len();
      let delimeter_len = delimeters.len();
      if delimeter_len == 0
      {
        result.push( &*o.src );
        return result;
      }

      let mut closests : Vec<usize> = vec![];
      let mut position : usize = 0;
      let mut closest_position : usize = 0;
      let mut closest_index : i32 = -1;
      let mut has_empty_delimeter : bool = false;

      for d in 0..delimeter_len
      {
        let delimeter = delimeters[ d ];
        if regexp_is( delimeter )
        {
          unimplemented!( "regexp splitting is not implemented" );
        }
        else
        {
          if delimeter.len() == 0
          {
            has_empty_delimeter = true;
          }
        }
        closests[ d ] = delimeter_next( &o.src, delimeter, &mut found_delimeters, position );
      }

      if position >= o.src.len()
      {
        position -= 1;
      }

      let mut delimeter = "";
      while position < src_len
      {
        delimeter = closest_which( &o.src, &found_delimeters, &closests, &mut closest_position, &mut closest_index );

        if closest_position == src_len
        {
          break;
        }

        let del_len = delimeter.len();
        if del_len == 0
        {
          position += 1;
        }

        let substring = o.src.get( position..closest_position ).unwrap();
        if preserving_empty || !substring.is_empty()
        {
          result.push( &substring );
        }

        if delimeter_len > 0 || position < src_len
        {
          if preserving_delimeters
          {
            if preserving_empty || del_len > 0
            {
              result.push( delimeter );
            }
          }
        }

        position = closests[ closest_index as usize ] + ( if del_len > 0 { del_len } else { 1 } );

        for d in 0..del_len
        {
          if closests[ d ] < position
          {
            closests[ d ] = delimeter_next( &o.src, delimeters[ d ], &mut found_delimeters, position );
          }
        }
      }

      if delimeter.len() > 0 || !has_empty_delimeter
      {
        let substring = o.src.get( position..src_len ).unwrap();
        if preserving_empty || !substring.is_empty()
        {
          result.push( &substring );
        }
      }
    }

    return result;

    /* */

    fn delimeter_next( src : &String, delimeter : &str, found_delimeters : &mut Vec<&str>, position : usize ) -> usize
    {
      assert!( position <= src.len() );

      if regexp_is( delimeter )
      {
        unimplemented!( "regexp splitting is not implemented" );
      }
      else
      {
        let indexes: Vec<( usize, &str )> = src.match_indices( delimeter ).collect();
        for ( index, _ ) in indexes
        {
          if index >= position
          {
            return index;
          }
        }
      }

      src.len()
    }

    //

    fn closest_which<'a>( src : &String, found_delimeters : &Vec<&'a str>, closests : &Vec<usize>, closest_position : &mut usize, closest_index : &mut i32  ) -> &'a str
    {
      let src_len = src.len();
      *closest_position = src_len;
      *closest_index = -1;
      for d in 0..found_delimeters.len()
      {
        if ( closests[ d ] < src_len ) && ( closests[ d ] < *closest_position )
        {
          *closest_position = closests[ d ];
          *closest_index = d as i32;
        }
      }

      found_delimeters[ *closest_index as usize ]
    }
  }
}
