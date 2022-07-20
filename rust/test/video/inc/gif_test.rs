use super::*;

tests_impls!
{
  fn basic()
  {
    use super::gif::{ Frame, Repeat, Encoder };
    use std::fs::File;

    let color_map = &[ 0xFF, 0xFF, 0xFF, 0, 0, 0 ];
    let ( width, height ) = ( 100, 100 );
    let mut states : [ u8; 10_000 ] = [ 0; 10_000 ];

    let mut image = File::create( "../../../target/out.gif" ).unwrap();
    let mut encoder = Encoder::new( &mut image, width, height, color_map ).unwrap();
    encoder.set_repeat( Repeat::Infinite ).unwrap();

    /* first frame */
    states[ 0 ] = 1;
    let frame = Frame::from_indexed_pixels( width, height, &states, None );
    encoder.write_frame( &frame ).unwrap();

    for i in 1..100
    {
      states[ i - 1 + ( i - 1 ) * 100 ] = 0;
      states[ i + i * 100 ] = 1;
      let frame = Frame::from_indexed_pixels( width, height, &states, None );
      encoder.write_frame( &frame ).unwrap();
    }

    assert!( true );
  }
}

//

tests_index!
{
  basic,
}
