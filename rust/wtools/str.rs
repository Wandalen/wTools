#![allow(dead_code)]

use regex::Regex;

pub fn regexpIs( src : &str ) -> bool
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
  pub delimeter : Option<Vec<&'a str>>,
  pub preservingEmpty : Option<bool>,
  pub preservingDelimeters : Option<bool>,
}

impl split_fast<'_>
{
  pub fn split<'a>( o : &'a mut split_fast ) -> Vec<&'a str>
  {
    let mut result: Vec<&str> = vec![];

    let default = &vec![ " " ];
    let delimeters = match o.delimeter
    {
      Some( ref del ) => del,
      None => default,
    };


    let preservingEmpty = o.preservingEmpty.is_some();
    let preservingDelimeters = o.preservingDelimeters.is_some();

    let mut foundDelimeters = delimeters.clone();

    if !preservingDelimeters && delimeters.len() == 1
    {
      result = o.src.split( delimeters[ 0 ] ).collect();
      if !preservingEmpty
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
        if regexpIs( delimeter )
        {
          unimplemented!( );
        }
        else
        {
          if delimeter.len() == 0
          {
            has_empty_delimeter = true;
          }
        }
        closests[ d ] = delimeter_next( &o.src, delimeter, &mut foundDelimeters, position );
      }

      if position >= o.src.len()
      {
        position -= 1;
      }

      while position < src_len
      {
        let delimeter = closestWhich( &o.src, &foundDelimeters, &closests, &mut closest_position, &mut closest_index );

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
        if preservingEmpty || !substring.is_empty()
        {
          result.push( &substring );
        }

        if delimeter_len > 0 || position < src_len
        {
          if preservingDelimeters
          {
            if preservingEmpty || del_len > 0
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
            closests[ d ] = delimeter_next( &o.src, delimeters[ d ], &mut foundDelimeters, position );
          }
        }
      }
    }

    return result;

    /* */

    fn delimeter_next( src : &String, delimeter : &str, foundDelimeters : &mut Vec<&str>, position : usize ) -> usize
    {
      assert!( position <= src.len() );

      if regexpIs( delimeter )
      {
        unimplemented!("regexp splitting is not implemented");
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

    fn closestWhich<'a>( src : &String, foundDelimeters : &Vec<&'a str>, closests : &Vec<usize>, closest_position : &mut usize, closest_index : &mut i32  ) -> &'a str
    {
      let src_len = src.len();
      *closest_position = src_len;
      *closest_index = -1;
      for d in 0..foundDelimeters.len()
      {
        if ( closests[ d ] < src_len ) && ( closests[ d ] < *closest_position )
        {
          *closest_position = closests[ d ];
          *closest_index = d as i32;
        }
      }

      foundDelimeters[ *closest_index as usize ]
    }
  }
}
