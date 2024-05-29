use super::*;

tests_impls!
{
  fn yuv444_to_rgb_conversion()
  {
    let yuv =
    [
      255, 128, 128,
      0, 128, 128,
      76, 84, 255,
      149, 43, 21,
      29, 255, 107,
    ];
    let rgb =
    [
      255, 255, 255, // white
      0, 0, 0, // black
      255, 0, 0, // red
      0, 255, 0, // green
      0, 0, 255, // blue
    ];

    let converted_rgb = yuv444_to_rgb( &yuv );

    assert_eq!( converted_rgb, rgb );
  }

  fn yuv422_to_rgb_conversion()
  {
    let yuv =
    [
      255, 128, 255, 128,
      0, 128, 0, 128,
      76, 84, 76, 255,
      149, 43, 149, 21,
      29, 255, 29, 107,
    ];
    let rgb =
    [
      255, 255, 255, 255, 255, 255, // white
      0, 0, 0, 0, 0, 0, // black
      255, 0, 0, 255, 0, 0, // red
      0, 255, 0, 0, 255, 0, // green
      0, 0, 255, 0, 0, 255, // blue
    ];

    let converted_rgb = yuv422_to_rgb( &yuv );

    assert_eq!( converted_rgb, rgb );
  }

  fn yvu420p_to_rgb_conversion()
  {
    let yvu =
    [
      255, 255, 0, 0, 76, 76, 149, 149, 29, 29,
      255, 255, 0, 0, 76, 76, 149, 149, 29, 29,
      128, 128, 255, 21, 107, 128, 128, 84, 43, 255,
    ];
    let rgb =
    [
      255, 255, 255, 255, 255, 255, // white
      0, 0, 0, 0, 0, 0, // black
      255, 0, 0, 255, 0, 0, // red
      0, 255, 0, 0, 255, 0, // green
      0, 0, 255, 0, 0, 255, // blue
      255, 255, 255, 255, 255, 255,
      0, 0, 0, 0, 0, 0,
      255, 0, 0, 255, 0, 0,
      0, 255, 0, 0, 255, 0,
      0, 0, 255, 0, 0, 255,
    ];

    let converted_rgb = yvu420p_to_rgb( &yvu, 10, 2 );

    assert_eq!( converted_rgb, rgb );
  }

  fn yuv420p_to_rgb_conversion()
  {
    let yuv =
    [
      255, 255, 0, 0, 76, 76, 149, 149, 29, 29,
      255, 255, 0, 0, 76, 76, 149, 149, 29, 29,
      128, 128, 84, 43, 255, 128, 128, 255, 21, 107,
    ];
    let rgb =
    [
      255, 255, 255, 255, 255, 255, // white
      0, 0, 0, 0, 0, 0, // black
      255, 0, 0, 255, 0, 0, // red
      0, 255, 0, 0, 255, 0, // green
      0, 0, 255, 0, 0, 255, // blue
      255, 255, 255, 255, 255, 255,
      0, 0, 0, 0, 0, 0,
      255, 0, 0, 255, 0, 0,
      0, 255, 0, 0, 255, 0,
      0, 0, 255, 0, 0, 255,
    ];

    let converted_rgb = yuv420p_to_rgb( &yuv, 10, 2 );

    assert_eq!( converted_rgb, rgb );
  }

  fn yuv422p_to_rgb_conversion()
  {
    let yuv =
    [
      255, 255, 0, 0, 76, 76, 149, 149, 29, 29,
      255, 255, 0, 0, 76, 76, 149, 149, 29, 29,
      128, 128, 84, 43, 255, 128, 128, 84, 43, 255,
      128, 128, 255, 21, 107, 128, 128, 255, 21, 107,
    ];
    let rgb =
    [
      255, 255, 255, 255, 255, 255, // white
      0, 0, 0, 0, 0, 0, // black
      255, 0, 0, 255, 0, 0, // red
      0, 255, 0, 0, 255, 0, // green
      0, 0, 255, 0, 0, 255, // blue
      255, 255, 255, 255, 255, 255,
      0, 0, 0, 0, 0, 0,
      255, 0, 0, 255, 0, 0,
      0, 255, 0, 0, 255, 0,
      0, 0, 255, 0, 0, 255,
    ];

    let converted_rgb = yuv422p_to_rgb( &yuv, 10, 2 );

    assert_eq!( converted_rgb, rgb );
  }

  fn grayscale_to_rgb_conversion()
  {
    let yuv =
    [
      255, 0, 76, 149, 29,
    ];
    let rgb =
    [
      255, 255, 255,
      0, 0, 0,
      76, 76, 76,
      149, 149, 149,
      29, 29, 29,
    ];

    let converted_rgb = grayscale_to_rgb( &yuv );

    assert_eq!( converted_rgb, rgb );
  }
}
//

tests_index!
{
  yuv444_to_rgb_conversion,
  yuv422_to_rgb_conversion,
  yvu420p_to_rgb_conversion,
  yuv420p_to_rgb_conversion,
  yuv422p_to_rgb_conversion,
  grayscale_to_rgb_conversion,
}
