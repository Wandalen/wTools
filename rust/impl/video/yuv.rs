/// Private namespace.
pub( crate ) mod private
{
  /// Convert one Y'UV444 frame to RGB888
  pub fn yuv444_to_rgb( buffer : &[ u8 ] ) -> Vec< u8 >
  {
    buffer.chunks_exact( 3 )
      .flat_map(| yuv | yuv_to_rgb( yuv[ 0 ], yuv[ 1 ], yuv[ 2 ] ) )
      .collect()
  }

  /// Convert one Y'UV422(also known as YUYV or YUY2) frame to RGB888
  pub fn yuv422_to_rgb( buffer : &[ u8 ] ) -> Vec< u8 >
  {
    buffer.chunks_exact( 4 )
      .flat_map( | yuv |
        [
          yuv_to_rgb( yuv[ 0 ], yuv[ 1 ], yuv[ 3 ] ),
          yuv_to_rgb( yuv[ 2 ], yuv[ 1 ], yuv[ 3 ] ),
        ] )
      .flatten()
      .collect()
  }

  /// Convert one Y'VU420p(also known as YV12) frame to RGB888
  pub fn yvu420p_to_rgb( frame : &[ u8 ], width : usize, height : usize ) -> Vec< u8 >
  {
    let pixels = width * height;
    let ( y_plane, remainder ) = frame.split_at( pixels );
    let ( v_plane, u_plane ) = remainder.split_at( pixels / 4 );
    convert_square_planar( y_plane, u_plane, v_plane, width, 2 )
  }

  /// Convert one Y'UV420p(also known as YU12) frame to RGB888
  pub fn yuv420p_to_rgb( frame: &[ u8 ], width : usize, height : usize ) -> Vec< u8 >
  {
    let pixels = width * height;
    let ( y_plane, remainder ) = frame.split_at( pixels );
    let ( u_plane, v_plane ) = remainder.split_at( pixels / 4 );
    convert_square_planar( y_plane, u_plane, v_plane, width, 2 )
  }

  /// Convert one Y'UV422p frame to RGB888
  pub fn yuv422p_to_rgb( frame : &[ u8 ], width : usize, height : usize ) -> Vec< u8 >
  {
    let pixels = width * height;
    let ( y_plane, remainder ) = frame.split_at( pixels );
    let ( u_plane, v_plane ) = remainder.split_at( pixels / 2 );
    convert_consecutive_planar( y_plane, u_plane, v_plane, 2 )
  }

  /// Convert one Grayscale frame to RGB888
  pub fn grayscale_to_rgb( buffer : &[ u8 ] ) -> Vec< u8 >
  {
    let mut rgb = Vec::with_capacity( buffer.len() * 3 );
    for &y in buffer
    {
      rgb.push( y );
      rgb.push( y );
      rgb.push( y );
    }
    rgb
  }

  fn yuv_to_rgb( y : u8, u : u8, v : u8 ) -> [ u8; 3 ]
  {
    let y = ( y as f32 ) - 16.0;
    let u = ( u as f32 ) - 128.0;
    let v = ( v as f32 ) - 128.0;
    let r = 1.164 * y + 1.596 * v;
    let g = 1.164 * y - 0.392 * u - 0.813 * v;
    let b = 1.164 * y + 2.017 * u;
    [
      r.clamp( 0.0, 255.0 ) as u8,
      g.clamp( 0.0, 255.0 ) as u8,
      b.clamp( 0.0, 255.0 ) as u8,
    ]
  }

  /// Convert "square" planes.
  /// Each U/V belongs to 'shared_count' number of Y's in one row.
  fn convert_square_planar( y_plane : &[ u8 ], u_plane : &[ u8 ], v_plane : &[ u8 ], width : usize, shared_count : usize ) -> Vec< u8 >
  {
    y_plane.chunks_exact( width * 2 )
      .zip( u_plane.chunks_exact( width / shared_count).zip( v_plane.chunks_exact( width / shared_count) ) )
      .flat_map( | ( rows, ( u, v ) ) |
        {
          let ( first, second ) = rows.split_at( width );
          let mut result = convert_consecutive_planar( first, u, v, shared_count );
          result.append( &mut convert_consecutive_planar( second, u, v, shared_count ) );
          result
        })
      .collect()
  }

  /// Convert planes with the horizontal sampling only.
  /// Each U/V belongs to 'shared_count' number of Y's.
  fn convert_consecutive_planar(y_plane : &[ u8 ], u_plane : &[ u8 ], v_plane : &[ u8 ], shared_count : usize ) -> Vec< u8 >
  {
    y_plane.chunks_exact( shared_count )
    .zip( u_plane.iter().zip( v_plane.iter() ) )
    .flat_map(| ( lums, ( u, v ) ) | [ yuv_to_rgb( lums[ 0 ], *u, *v ), yuv_to_rgb( lums[ 1 ], *u, *v ) ] )
    .flatten()
    .collect()
  }
}

//

wtools::meta::mod_interface!
{
  prelude use yuv444_to_rgb;
  prelude use yuv422_to_rgb;
  prelude use yvu420p_to_rgb;
  prelude use yuv420p_to_rgb;
  prelude use yuv422p_to_rgb;
  prelude use grayscale_to_rgb;
}
