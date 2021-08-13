( function _l0_l9_LongDescriptor_test_s_()
{

'use strict';

if( typeof module !== 'undefined' )
{
  const _ = require( 'Tools' );
  _.include( 'wTesting' );
}

const _ = _global_.wTools;

//--
// tests
//--

function isArrayUnrollArgumentsArray( test )
{
  var descriptorsList = [ 'Array', 'Unroll', 'ArgumentsArray' ];

  for( let i = 0; i < descriptorsList.length; i++ )
  {
    let descriptor = _.withLong[ descriptorsList[ i ] ].long.default;
    // let descriptor = _.withLong[ descriptorsList[ i ] ].longDescriptor;
    test.open( descriptorsList[ i ] );
    testRun( descriptor );
    test.close( descriptorsList[ i ] )
  }

  /* */

  function testRun( descriptor )
  {
    test.case = 'without argument';
    var got = descriptor.is();
    var expected  = false;
    test.identical( got, expected );

    test.case = 'null';
    var got = descriptor.is( null );
    var expected  = false;
    test.identical( got, expected );

    test.case = 'undefined';
    var got = descriptor.is( undefined );
    var expected  = false;
    test.identical( got, expected );

    test.case = 'number';
    var got = descriptor.is( 6 );
    var expected  = false;
    test.identical( got, expected );

    test.case = 'string';
    var got = descriptor.is( 'abc' );
    var expected  = false;
    test.identical( got, expected );

    test.case = 'boolean';
    var got = descriptor.is( true );
    var expected  = false;
    test.identical( got, expected );

    /* */

    test.case = 'map';
    var got = descriptor.is( {} );
    var expected  = false;
    test.identical( got, expected );

    test.case = 'HashMap';
    var got = descriptor.is( new HashMap() );
    var expected  = false;
    test.identical( got, expected );

    test.case = 'Set';
    var got = descriptor.is( new Set() );
    var expected  = false;
    test.identical( got, expected );

    test.case = 'BufferTyped';
    var got = descriptor.is( new U8x( 5 ) );
    var expected  = descriptor.TypeName === 'U8x';
    test.identical( got, expected );

    test.case = 'function';
    var got = descriptor.is( function(){} );
    var expected  = false;
    test.identical( got, expected );

    /* */

    test.case = 'an empty array';
    var got = descriptor.is( [] );
    var expected = descriptor.TypeName === 'Array';
    test.identical( got, expected );

    test.case = 'an array';
    var got = descriptor.is( [ 1, 2, 3 ] );
    var expected  = descriptor.TypeName === 'Array';
    test.identical( got, expected );

    test.case = 'empty ArgumentsArray';
    var got = descriptor.is( _.argumentsArray.make( [] ) );
    var expected = descriptor.TypeName === 'ArgumentsArray';
    test.identical( got, expected );

    test.case = 'ArgumentsArray';
    var got = descriptor.is( _.argumentsArray.make( [ true ] ) );
    var expected = descriptor.TypeName === 'ArgumentsArray';
    test.identical( got, expected );

    test.case = 'empty unroll';
    var got = descriptor.is( _.unroll.make( [] ) );
    var expected = !( descriptor.TypeName === 'ArgumentsArray' );
    test.identical( got, expected );

    test.case = 'unroll';
    var got = descriptor.is( _.unroll.make( [ true ] ) );
    var expected = !( descriptor.TypeName === 'ArgumentsArray' );
    test.identical( got, expected );
  }
}

//

function isBufferTypedInstance( test )
{
  var descriptorsList = [ 'U8x', 'U16x', 'U32x', 'Ux', 'I8x', 'I16x', 'I32x', 'Ix', 'F32x', 'F64x', 'Fx' ];

  for( let i = 0; i < descriptorsList.length; i++ )
  {
    _.assert( !!_.withLong[ descriptorsList[ i ] ] );
    let descriptor = _.withLong[ descriptorsList[ i ] ].long.default;
    // let descriptor = _.withLong[ descriptorsList[ i ] ].longDescriptor;
    test.open( descriptorsList[ i ] );
    testRun( descriptor );
    test.close( descriptorsList[ i ] )
  }

  /* */

  function testRun( descriptor )
  {
    test.case = 'null';
    var got = descriptor.is( null );
    var expected  = false;
    test.identical( got, expected );

    test.case = 'undefined';
    var got = descriptor.is( undefined );
    var expected  = false;
    test.identical( got, expected );

    test.case = 'number';
    var got = descriptor.is( 6 );
    var expected  = false;
    test.identical( got, expected );

    test.case = 'string';
    var got = descriptor.is( 'abc' );
    var expected  = false;
    test.identical( got, expected );

    test.case = 'boolean';
    var got = descriptor.is( true );
    var expected  = false;
    test.identical( got, expected );

    /* */

    test.case = 'an array';
    var got = descriptor.is( [ 1, 2, 3 ] );
    var expected  = false;
    test.identical( got, expected );

    test.case = 'ArgumentsArray';
    var got = descriptor.is( _.argumentsArray.make( [ true ] ) );
    var expected =  false;
    test.identical( got, expected );

    test.case = 'unroll';
    var got = descriptor.is( _.unroll.make( [ true ] ) );
    var expected = false;
    test.identical( got, expected );

    test.case = 'map';
    var got = descriptor.is( {} );
    var expected  = false;
    test.identical( got, expected );

    test.case = 'HashMap';
    var got = descriptor.is( new HashMap() );
    var expected  = false;
    test.identical( got, expected );

    test.case = 'Set';
    var got = descriptor.is( new Set() );
    var expected  = false;
    test.identical( got, expected );

    test.case = 'function';
    var got = descriptor.is( function(){} );
    var expected  = false;
    test.identical( got, expected );

    /* */

    test.case = 'U8x';
    var got = descriptor.is( new U8x( 5 ) );
    var expected  = descriptor.TypeName === 'U8x';
    test.identical( got, expected );

    test.case = 'U16x';
    var got = descriptor.is( new U16x( 5 ) );
    var expected  = descriptor.TypeName === 'U16x';
    test.identical( got, expected );

    test.case = 'U32x';
    var got = descriptor.is( new U32x( 5 ) );
    var expected  = descriptor.TypeName === 'U32x' || descriptor.TypeName === 'Ux';
    test.identical( got, expected );

    test.case = 'Ux';
    var got = descriptor.is( new Ux( 5 ) );
    var expected  = descriptor.TypeName === 'U32x' || descriptor.TypeName === 'Ux';
    test.identical( got, expected );

    test.case = 'I8x';
    var got = descriptor.is( new I8x( 5 ) );
    var expected  = descriptor.TypeName === 'I8x';
    test.identical( got, expected );

    test.case = 'I16x';
    var got = descriptor.is( new I16x( 5 ) );
    var expected  = descriptor.TypeName === 'I16x';
    test.identical( got, expected );

    test.case = 'I32x';
    var got = descriptor.is( new I32x( 5 ) );
    var expected  = descriptor.TypeName === 'I32x' || descriptor.TypeName === 'Ix';
    test.identical( got, expected );

    test.case = 'Ix';
    var got = descriptor.is( new Ix( 5 ) );
    var expected  = descriptor.TypeName === 'I32x' || descriptor.TypeName === 'Ix';
    test.identical( got, expected );

    test.case = 'F32x';
    var got = descriptor.is( new F32x( 5 ) );
    var expected  = descriptor.TypeName === 'F32x' || descriptor.TypeName === 'Fx';
    test.identical( got, expected );

    test.case = 'F64x';
    var got = descriptor.is( new F64x( 5 ) );
    var expected  = descriptor.TypeName === 'F64x';
    test.identical( got, expected );

    test.case = 'Fx';
    var got = descriptor.is( new Fx( 5 ) );
    var expected  = descriptor.TypeName === 'F32x' || descriptor.TypeName === 'Fx';
    test.identical( got, expected );
  }
}

//

function makeArrayUnrollArgumentsArray( test )
{
  var descriptorsList = [ 'Array', 'Unroll', 'ArgumentsArray' ];
  var getExpectedLongList =
  [
    _.array.make.bind( _.array ),
    _.unroll.make.bind( _.unroll ),
    _.argumentsArray.make.bind( _.argumentsArray )
  ];

  for( let i = 0; i < descriptorsList.length; i++ )
  {
    let descriptor = _.withLong[ descriptorsList[ i ] ].long.default;
    // let descriptor = _.withLong[ descriptorsList[ i ] ].longDescriptor;
    let getExpectedLong = getExpectedLongList[ i ];
    test.open( descriptorsList[ i ] );
    testRun( descriptor, getExpectedLong );
    test.close( descriptorsList[ i ] )
  }

  /* */

  function testRun( descriptor, getExpectedLong )
  {
    test.case = 'without arguments';
    var got = descriptor.make();
    var expected = getExpectedLong( [] );
    test.identical( got, expected );
    test.true( descriptor.is( got ) );

    // test.case = 'src = undefined';
    // var src = undefined;
    // var got = descriptor.make( src );
    // var expected = getExpectedLong( [] );
    // test.equivalent( got, expected );
    // test.true( descriptor.is( got ) );
    // test.true( src !== got );

    test.case = 'src = null';
    var src = null;
    var got = descriptor.make( src );
    var expected = getExpectedLong( [] );
    test.equivalent( got, expected );
    test.true( descriptor.is( got ) );
    test.true( src !== got );

    test.case = 'src = empty array';
    var src = [];
    var got = descriptor.make( src );
    var expected = getExpectedLong( [] );
    test.equivalent( got, expected );
    test.true( descriptor.is( got ) );
    test.true( src !== got );

    test.case = 'src = array, src.length = 1';
    var src = [ 0 ];
    var got = descriptor.make( src );
    var expected = getExpectedLong( [ 0 ] );
    test.equivalent( got, expected );
    test.true( descriptor.is( got ) );
    test.true( src !== got );

    test.case = 'src = array, src.length > 1';
    var src = [ 1, 2, 3 ];
    var got = descriptor.make( src );
    var expected = getExpectedLong( [ 1, 2, 3 ] );
    test.equivalent( got, expected );
    test.true( descriptor.is( got ) );
    test.true( src !== got );

    /* */

    test.case = 'src = number, src = 0';
    var got = descriptor.make( 0 );
    var expected = getExpectedLong( new Array( 0 ) );
    test.equivalent( got, expected );
    test.true( descriptor.is( got ) );
    test.true( src !== got );

    test.case = 'src = number, src > 0';
    var got = descriptor.make( 3 );
    var expected = getExpectedLong( new Array( 3 ) );
    test.equivalent( got, expected );
    test.true( descriptor.is( got ) );
    test.true( src !== got );

    /* */

    test.case = 'src = empty U8x';
    var src = new U8x();
    var got = descriptor.make( src );
    var expected = getExpectedLong( [] );
    test.equivalent( got, expected );
    test.true( descriptor.is( got ) );
    test.true( src !== got );

    test.case = 'src = U8x, src.length = 1';
    var src = new U8x( 1 );
    var got = descriptor.make( src );
    var expected = getExpectedLong( [ 0 ] );
    test.equivalent( got, expected );
    test.true( descriptor.is( got ) );
    test.true( src !== got );

    test.case = 'src = U8x, src.length > 1';
    var src = new U8x( [ 1, 2, 3 ] );
    var got = descriptor.make( src );
    var expected = getExpectedLong( [ 1, 2, 3 ] );
    test.equivalent( got, expected );
    test.true( descriptor.is( got ) );
    test.true( src !== got );

    /* */

    test.case = 'src = empty I16x';
    var src = new I16x();
    var got = descriptor.make( src );
    var expected = getExpectedLong( [] );
    test.equivalent( got, expected );
    test.true( descriptor.is( got ) );
    test.true( src !== got );

    test.case = 'src = I16x, src.length = 1';
    var src = new I16x( 1 );
    var got = descriptor.make( src );
    var expected = getExpectedLong( [ 0 ] );
    test.equivalent( got, expected );
    test.true( descriptor.is( got ) );
    test.true( src !== got );

    test.case = 'src = I16x, src.length > 1';
    var src = new I16x( [ 1, 2, 3 ] );
    var got = descriptor.make( src );
    var expected = getExpectedLong( [ 1, 2, 3 ] );
    test.equivalent( got, expected );
    test.true( descriptor.is( got ) );
    test.true( src !== got );

    /* */

    test.case = 'src = empty F32x';
    var src = new F32x();
    var got = descriptor.make( src );
    var expected = getExpectedLong( [] );
    test.equivalent( got, expected );
    test.true( descriptor.is( got ) );
    test.true( src !== got );

    test.case = 'src = F32x, src.length = 1';
    var src = new F32x( 1 );
    var got = descriptor.make( src );
    var expected = getExpectedLong( [ 0 ] );
    test.equivalent( got, expected );
    test.true( descriptor.is( got ) );
    test.true( src !== got );

    test.case = 'src = F32x, src.length > 1';
    var src = new F32x( [ 1, 2, 3 ] );
    var got = descriptor.make( src );
    var expected = getExpectedLong( [ 1, 2, 3 ] );
    test.equivalent( got, expected );
    test.true( descriptor.is( got ) );
    test.true( src !== got );

    /* */

    test.case = 'src = empty arguments array';
    var src = _.argumentsArray.make( [] );
    var got = descriptor.make( src );
    var expected = getExpectedLong( [] );
    test.equivalent( got, expected );
    test.true( descriptor.is( got ) );
    test.true( src !== got );

    test.case = 'src = arguments array, src.length = 1';
    var src = _.argumentsArray.make( [ {} ] );
    var got = descriptor.make( src );
    var expected = getExpectedLong( [ {} ] );
    test.equivalent( got, expected );
    test.true( descriptor.is( got ) );
    test.true( src !== got );

    test.case = 'src = arguments array, src.length > 1';
    var src = _.argumentsArray.make( [ 1, 2, 3 ] );
    var got = descriptor.make( src );
    var expected = getExpectedLong( [ 1, 2, 3 ] );
    test.equivalent( got, expected );
    test.true( descriptor.is( got ) );
    test.true( src !== got );

    /* */

    test.case = 'src = empty unroll';
    var src = _.unroll.make( [] );
    var got = descriptor.make( src );
    var expected = getExpectedLong( [] );
    test.equivalent( got, expected );
    test.true( descriptor.is( got ) );
    test.true( src !== got );

    test.case = 'src = unroll, src.length = 1';
    var src = _.unroll.make( [ 'str' ] );
    var got = descriptor.make( src );
    var expected = getExpectedLong( [ 'str' ] );
    test.equivalent( got, expected );
    test.true( descriptor.is( got ) );
    test.true( src !== got );

    test.case = 'src = unroll, src.length > 1';
    var src = _.unroll.make( [ 1, 2, 3 ] );
    var got = descriptor.make( src );
    var expected = getExpectedLong( [ 1, 2, 3 ] );
    test.equivalent( got, expected );
    test.true( descriptor.is( got ) );
    test.true( src !== got );

    /* - */

    if( Config.debug )
    {
      test.case = 'extra arguments';
      test.shouldThrowErrorSync( () => descriptor.make( 1, 3 ) );

      test.case = 'wrong type of src';
      test.shouldThrowErrorSync( () => descriptor.make( {} ) );
      test.shouldThrowErrorSync( () => descriptor.make( 'wrong' ) );
    }
  }
}

//

function makeBufferTypedInstance( test )
{
  var descriptorsList = [ 'U8x', 'U16x', 'U32x', 'Ux', 'I8x', 'I16x', 'I32x', 'Ix', 'F32x', 'F64x', 'Fx' ];
  var getExpectedLongList = [ U8x, U16x, U32x, Ux, I8x, I16x, I32x, Ix, F32x, F64x, Fx ];

  for( let i = 0; i < descriptorsList.length; i++ )
  {
    let descriptor = _.withLong[ descriptorsList[ i ] ].long.default;
    // let descriptor = _.withLong[ descriptorsList[ i ] ].longDescriptor;
    let getExpectedLong = getExpectedLongList[ i ];
    test.open( descriptorsList[ i ] );
    testRun( descriptor, getExpectedLong );
    test.close( descriptorsList[ i ] )
  }

  /* */

  function testRun( descriptor, getExpectedLong )
  {
    test.case = 'without arguments';
    var got = descriptor.make();
    var expected = new getExpectedLong( [] );
    test.identical( got, expected );
    test.true( descriptor.is( got ) );

    test.case = 'src = null';
    var src = null;
    var got = descriptor.make( src );
    var expected = new getExpectedLong( [] );
    test.equivalent( got, expected );
    test.true( descriptor.is( got ) );
    test.true( src !== got );

    test.case = 'src = empty array';
    var src = [];
    var got = descriptor.make( src );
    var expected = new getExpectedLong( [] );
    test.equivalent( got, expected );
    test.true( descriptor.is( got ) );
    test.true( src !== got );

    test.case = 'src = array, src.length = 1';
    var src = [ 0 ];
    var got = descriptor.make( src );
    var expected = new getExpectedLong( [ 0 ] );
    test.equivalent( got, expected );
    test.true( descriptor.is( got ) );
    test.true( src !== got );

    test.case = 'src = array, src.length > 1';
    var src = [ 1, 2, 3 ];
    var got = descriptor.make( src );
    var expected = new getExpectedLong( [ 1, 2, 3 ] );
    test.equivalent( got, expected );
    test.true( descriptor.is( got ) );
    test.true( src !== got );

    /* */

    test.case = 'src = number, src = 0';
    var got = descriptor.make( 0 );
    var expected = new getExpectedLong( 0 );
    test.equivalent( got, expected );
    test.true( descriptor.is( got ) );
    test.true( src !== got );

    test.case = 'src = number, src > 0';
    var got = descriptor.make( 3 );
    var expected = new getExpectedLong( 3 );
    test.equivalent( got, expected );
    test.true( descriptor.is( got ) );
    test.true( src !== got );

    /* */

    test.case = 'src = empty U8x';
    var src = new U8x();
    var got = descriptor.make( src );
    var expected = new getExpectedLong( [] );
    test.equivalent( got, expected );
    test.true( descriptor.is( got ) );
    test.true( src !== got );

    test.case = 'src = U8x, src.length = 1';
    var src = new U8x( 1 );
    var got = descriptor.make( src );
    var expected = new getExpectedLong( [ 0 ] );
    test.equivalent( got, expected );
    test.true( descriptor.is( got ) );
    test.true( src !== got );

    test.case = 'src = U8x, src.length > 1';
    var src = new U8x( [ 1, 2, 3 ] );
    var got = descriptor.make( src );
    var expected = new getExpectedLong( [ 1, 2, 3 ] );
    test.equivalent( got, expected );
    test.true( descriptor.is( got ) );
    test.true( src !== got );

    /* */

    test.case = 'src = empty I16x';
    var src = new I16x();
    var got = descriptor.make( src );
    var expected = new getExpectedLong( [] );
    test.equivalent( got, expected );
    test.true( descriptor.is( got ) );
    test.true( src !== got );

    test.case = 'src = I16x, src.length = 1';
    var src = new I16x( 1 );
    var got = descriptor.make( src );
    var expected = new getExpectedLong( [ 0 ] );
    test.equivalent( got, expected );
    test.true( descriptor.is( got ) );
    test.true( src !== got );

    test.case = 'src = I16x, src.length > 1';
    var src = new I16x( [ 1, 2, 3 ] );
    var got = descriptor.make( src );
    var expected = new getExpectedLong( [ 1, 2, 3 ] );
    test.equivalent( got, expected );
    test.true( descriptor.is( got ) );
    test.true( src !== got );

    /* */

    test.case = 'src = empty F32x';
    var src = new F32x();
    var got = descriptor.make( src );
    var expected = new getExpectedLong( [] );
    test.equivalent( got, expected );
    test.true( descriptor.is( got ) );
    test.true( src !== got );

    test.case = 'src = F32x, src.length = 1';
    var src = new F32x( 1 );
    var got = descriptor.make( src );
    var expected = new getExpectedLong( [ 0 ] );
    test.equivalent( got, expected );
    test.true( descriptor.is( got ) );
    test.true( src !== got );

    test.case = 'src = F32x, src.length > 1';
    var src = new F32x( [ 1, 2, 3 ] );
    var got = descriptor.make( src );
    var expected = new getExpectedLong( [ 1, 2, 3 ] );
    test.equivalent( got, expected );
    test.true( descriptor.is( got ) );
    test.true( src !== got );

    /* */

    test.case = 'src = empty arguments array';
    var src = _.argumentsArray.make( [] );
    var got = descriptor.make( src );
    var expected = new getExpectedLong( [] );
    test.equivalent( got, expected );
    test.true( descriptor.is( got ) );
    test.true( src !== got );

    test.case = 'src = arguments array, src.length = 1';
    var src = _.argumentsArray.make( [ {} ] );
    var got = descriptor.make( src );
    var expected = new getExpectedLong( [ {} ] );
    test.equivalent( got, expected );
    test.true( descriptor.is( got ) );
    test.true( src !== got );

    test.case = 'src = arguments array, src.length > 1';
    var src = _.argumentsArray.make( [ 1, 2, 3 ] );
    var got = descriptor.make( src );
    var expected = new getExpectedLong( [ 1, 2, 3 ] );
    test.equivalent( got, expected );
    test.true( descriptor.is( got ) );
    test.true( src !== got );

    /* */

    test.case = 'src = empty unroll';
    var src = _.unroll.make( [] );
    var got = descriptor.make( src );
    var expected = new getExpectedLong( [] );
    test.equivalent( got, expected );
    test.true( descriptor.is( got ) );
    test.true( src !== got );

    test.case = 'src = unroll, src.length = 1';
    var src = _.unroll.make( [ 'str' ] );
    var got = descriptor.make( src );
    var expected = new getExpectedLong( [ 'str' ] );
    test.equivalent( got, expected );
    test.true( descriptor.is( got ) );
    test.true( src !== got );

    test.case = 'src = unroll, src.length > 1';
    var src = _.unroll.make( [ 1, 2, 3 ] );
    var got = descriptor.make( src );
    var expected = new getExpectedLong( [ 1, 2, 3 ] );
    test.equivalent( got, expected );
    test.true( descriptor.is( got ) );
    test.true( src !== got );
  }
}

//

function fromArrayUnrollArgumentsArray( test )
{
  var descriptorsList = [ 'Array', 'Unroll', 'ArgumentsArray' ];
  var getExpectedLongList =
  [
    _.array.make.bind( _.array ),
    _.unroll.make.bind( _.unroll ),
    _.argumentsArray.make.bind( _.argumentsArray )
  ];

  for( let i = 0; i < descriptorsList.length; i++ )
  {
    let descriptor = _.withLong[ descriptorsList[ i ] ].long.default;
    // let descriptor = _.withLong[ descriptorsList[ i ] ].longDescriptor;
    let getExpectedLong = getExpectedLongList[ i ];
    test.open( descriptorsList[ i ] );
    testRun( descriptor, getExpectedLong );
    test.close( descriptorsList[ i ] )
  }

  /* */

  function testRun( descriptor, getExpectedLong )
  {
    test.case = 'src = null';
    var src = null;
    var got = descriptor.from( src );
    var expected = getExpectedLong( [] );
    test.equivalent( got, expected );
    test.true( descriptor.is( got ) );
    test.true( src !== got );

    test.case = 'src = empty array';
    var src = [];
    var got = descriptor.from( src );
    var expected = getExpectedLong( [] );
    test.equivalent( got, expected );
    test.true( descriptor.is( got ) );
    test.true( descriptor.TypeName === 'Array' ? src === got : src !== got );

    test.case = 'src = array, src.length = 1';
    var src = [ 0 ];
    var got = descriptor.from( src );
    var expected = getExpectedLong( [ 0 ] );
    test.equivalent( got, expected );
    test.true( descriptor.is( got ) );
    test.true( descriptor.TypeName === 'Array' ? src === got : src !== got );

    test.case = 'src = array, src.length > 1';
    var src = [ 1, 2, 3 ];
    var got = descriptor.from( src );
    var expected = getExpectedLong( [ 1, 2, 3 ] );
    test.equivalent( got, expected );
    test.true( descriptor.is( got ) );
    test.true( descriptor.TypeName === 'Array' ? src === got : src !== got );

    /* */

    test.case = 'src = number, src = 0';
    var got = descriptor.from( 0 );
    var expected = getExpectedLong( new Array( 0 ) );
    test.equivalent( got, expected );
    test.true( descriptor.is( got ) );
    test.true( src !== got );

    test.case = 'src = number, src > 0';
    var got = descriptor.from( 3 );
    var expected = getExpectedLong( new Array( 3 ) );
    test.equivalent( got, expected );
    test.true( descriptor.is( got ) );
    test.true( src !== got );

    /* */

    test.case = 'src = empty U8x';
    var src = new U8x();
    var got = descriptor.from( src );
    var expected = getExpectedLong( [] );
    test.equivalent( got, expected );
    test.true( descriptor.is( got ) );
    test.true( src !== got );

    test.case = 'src = U8x, src.length = 1';
    var src = new U8x( 1 );
    var got = descriptor.from( src );
    var expected = getExpectedLong( [ 0 ] );
    test.equivalent( got, expected );
    test.true( descriptor.is( got ) );
    test.true( src !== got );

    test.case = 'src = U8x, src.length > 1';
    var src = new U8x( [ 1, 2, 3 ] );
    var got = descriptor.from( src );
    var expected = getExpectedLong( [ 1, 2, 3 ] );
    test.equivalent( got, expected );
    test.true( descriptor.is( got ) );
    test.true( src !== got );

    /* */

    test.case = 'src = empty I16x';
    var src = new I16x();
    var got = descriptor.from( src );
    var expected = getExpectedLong( [] );
    test.equivalent( got, expected );
    test.true( descriptor.is( got ) );
    test.true( src !== got );

    test.case = 'src = I16x, src.length = 1';
    var src = new I16x( 1 );
    var got = descriptor.from( src );
    var expected = getExpectedLong( [ 0 ] );
    test.equivalent( got, expected );
    test.true( descriptor.is( got ) );
    test.true( src !== got );

    test.case = 'src = I16x, src.length > 1';
    var src = new I16x( [ 1, 2, 3 ] );
    var got = descriptor.from( src );
    var expected = getExpectedLong( [ 1, 2, 3 ] );
    test.equivalent( got, expected );
    test.true( descriptor.is( got ) );
    test.true( src !== got );

    /* */

    test.case = 'src = empty F32x';
    var src = new F32x();
    var got = descriptor.from( src );
    var expected = getExpectedLong( [] );
    test.equivalent( got, expected );
    test.true( descriptor.is( got ) );
    test.true( src !== got );

    test.case = 'src = F32x, src.length = 1';
    var src = new F32x( 1 );
    var got = descriptor.from( src );
    var expected = getExpectedLong( [ 0 ] );
    test.equivalent( got, expected );
    test.true( descriptor.is( got ) );
    test.true( src !== got );

    test.case = 'src = F32x, src.length > 1';
    var src = new F32x( [ 1, 2, 3 ] );
    var got = descriptor.from( src );
    var expected = getExpectedLong( [ 1, 2, 3 ] );
    test.equivalent( got, expected );
    test.true( descriptor.is( got ) );
    test.true( src !== got );

    /* */

    test.case = 'src = empty arguments array';
    var src = _.argumentsArray.make( [] );
    var got = descriptor.from( src );
    var expected = getExpectedLong( [] );
    test.equivalent( got, expected );
    test.true( descriptor.is( got ) );
    test.true( descriptor.TypeName === 'ArgumentsArray' ? src === got : src !== got );

    test.case = 'src = arguments array, src.length = 1';
    var src = _.argumentsArray.make( [ {} ] );
    var got = descriptor.from( src );
    var expected = getExpectedLong( [ {} ] );
    test.equivalent( got, expected );
    test.true( descriptor.is( got ) );
    test.true( descriptor.TypeName === 'ArgumentsArray' ? src === got : src !== got );

    test.case = 'src = arguments array, src.length > 1';
    var src = _.argumentsArray.make( [ 1, 2, 3 ] );
    var got = descriptor.from( src );
    var expected = getExpectedLong( [ 1, 2, 3 ] );
    test.equivalent( got, expected );
    test.true( descriptor.is( got ) );
    test.true( descriptor.TypeName === 'ArgumentsArray' ? src === got : src !== got );

    /* */

    test.case = 'src = empty unroll';
    var src = _.unroll.make( [] );
    var got = descriptor.from( src );
    var expected = getExpectedLong( [] );
    test.equivalent( got, expected );
    test.true( descriptor.is( got ) );
    test.true( descriptor.TypeName === 'ArgumentsArray' ? src !== got : src === got );

    test.case = 'src = unroll, src.length = 1';
    var src = _.unroll.make( [ 'str' ] );
    var got = descriptor.from( src );
    var expected = getExpectedLong( [ 'str' ] );
    test.equivalent( got, expected );
    test.true( descriptor.is( got ) );
    test.true( descriptor.TypeName === 'ArgumentsArray' ? src !== got : src === got );

    test.case = 'src = unroll, src.length > 1';
    var src = _.unroll.make( [ 1, 2, 3 ] );
    var got = descriptor.from( src );
    var expected = getExpectedLong( [ 1, 2, 3 ] );
    test.equivalent( got, expected );
    test.true( descriptor.is( got ) );
    test.true( descriptor.TypeName === 'ArgumentsArray' ? src !== got : src === got );

    /* - */

    if( Config.debug )
    {
      test.case = 'without arguments';
      test.shouldThrowErrorSync( () => descriptor.from() );

      test.case = 'extra arguments';
      test.shouldThrowErrorSync( () => descriptor.from( 1, 3 ) );
      test.shouldThrowErrorSync( () => descriptor.from( [], 3 ) );

      test.case = 'wrong type of src';
      test.shouldThrowErrorSync( () => descriptor.from( {} ) );
      test.shouldThrowErrorSync( () => descriptor.from( 'wrong' ) );
    }
  }
}

//

function fromBufferTypedInstance( test )
{
  var descriptorsList = [ 'U8x', 'U16x', 'U32x', 'Ux', 'I8x', 'I16x', 'I32x', 'Ix', 'F32x', 'F64x', 'Fx' ];
  var getExpectedLongList = [ U8x, U16x, U32x, Ux, I8x, I16x, I32x, Ix, F32x, F64x, Fx ];

  for( let i = 0; i < descriptorsList.length; i++ )
  {
    let descriptor = _.withLong[ descriptorsList[ i ] ].long.default;
    // let descriptor = _.withLong[ descriptorsList[ i ] ].longDescriptor;
    let getExpectedLong = getExpectedLongList[ i ];
    test.open( descriptorsList[ i ] );
    testRun( descriptor, getExpectedLong );
    test.close( descriptorsList[ i ] )
  }

  /* */

  function testRun( descriptor, getExpectedLong )
  {
    test.case = 'src = null';
    var src = null;
    var got = descriptor.from( src );
    var expected = new getExpectedLong( [] );
    test.equivalent( got, expected );
    test.true( descriptor.is( got ) );
    test.true( src !== got );

    test.case = 'src = empty array';
    var src = [];
    var got = descriptor.from( src );
    var expected = new getExpectedLong( [] );
    test.equivalent( got, expected );
    test.true( descriptor.is( got ) );
    test.true( src !== got );

    test.case = 'src = array, src.length = 1';
    var src = [ 0 ];
    var got = descriptor.from( src );
    var expected = new getExpectedLong( [ 0 ] );
    test.equivalent( got, expected );
    test.true( descriptor.is( got ) );
    test.true( src !== got );

    test.case = 'src = array, src.length > 1';
    var src = [ 1, 2, 3 ];
    var got = descriptor.from( src );
    var expected = new getExpectedLong( [ 1, 2, 3 ] );
    test.equivalent( got, expected );
    test.true( descriptor.is( got ) );
    test.true( src !== got );

    /* */

    test.case = 'src = number, src = 0';
    var got = descriptor.from( 0 );
    var expected = new getExpectedLong( 0 );
    test.equivalent( got, expected );
    test.true( descriptor.is( got ) );
    test.true( src !== got );

    test.case = 'src = number, src > 0';
    var got = descriptor.from( 3 );
    var expected = new getExpectedLong( 3 );
    test.equivalent( got, expected );
    test.true( descriptor.is( got ) );
    test.true( src !== got );

    /* */

    test.case = 'src = empty U8x';
    var src = new U8x();
    var got = descriptor.from( src );
    var expected = new getExpectedLong( [] );
    test.equivalent( got, expected );
    test.true( descriptor.is( got ) );
    test.true( descriptor.TypeName === 'U8x' ? src === got : src !== got );

    test.case = 'src = U16x, src.length = 1';
    var src = new U16x( 1 );
    var got = descriptor.from( src );
    var expected = new getExpectedLong( [ 0 ] );
    test.equivalent( got, expected );
    test.true( descriptor.is( got ) );
    test.true( descriptor.TypeName === 'U16x' ? src === got : src !== got );

    test.case = 'src = U32x, src.length > 1';
    var src = new U32x( [ 1, 2, 3 ] );
    var got = descriptor.from( src );
    var expected = new getExpectedLong( [ 1, 2, 3 ] );
    test.equivalent( got, expected );
    test.true( descriptor.is( got ) );
    test.true( descriptor.TypeName === 'U32x' || descriptor.TypeName === 'Ux' ? src === got : src !== got );

    /* */

    test.case = 'src = empty I8x';
    var src = new I8x();
    var got = descriptor.from( src );
    var expected = new getExpectedLong( [] );
    test.equivalent( got, expected );
    test.true( descriptor.is( got ) );
    test.true( descriptor.TypeName === 'I8x' ? src === got : src !== got );

    test.case = 'src = I16x, src.length = 1';
    var src = new I16x( 1 );
    var got = descriptor.from( src );
    var expected = new getExpectedLong( [ 0 ] );
    test.equivalent( got, expected );
    test.true( descriptor.is( got ) );
    test.true( descriptor.TypeName === 'I16x' ? src === got : src !== got );

    test.case = 'src = I32x, src.length > 1';
    var src = new I32x( [ 1, 2, 3 ] );
    var got = descriptor.from( src );
    var expected = new getExpectedLong( [ 1, 2, 3 ] );
    test.equivalent( got, expected );
    test.true( descriptor.is( got ) );
    test.true( descriptor.TypeName === 'I32x' || descriptor.TypeName === 'Ix' ? src === got : src !== got );

    /* */

    test.case = 'src = empty F32x';
    var src = new F32x();
    var got = descriptor.from( src );
    var expected = new getExpectedLong( [] );
    test.equivalent( got, expected );
    test.true( descriptor.is( got ) );
    test.true( descriptor.TypeName === 'F32x' || descriptor.TypeName === 'Fx' ? src === got : src !== got );

    test.case = 'src = F64x, src.length = 1';
    var src = new F64x( 1 );
    var got = descriptor.from( src );
    var expected = new getExpectedLong( [ 0 ] );
    test.equivalent( got, expected );
    test.true( descriptor.is( got ) );
    test.true( descriptor.TypeName === 'F64x' ? src === got : src !== got );

    test.case = 'src = Fx, src.length > 1';
    var src = new Fx( [ 1, 2, 3 ] );
    var got = descriptor.from( src );
    var expected = new getExpectedLong( [ 1, 2, 3 ] );
    test.equivalent( got, expected );
    test.true( descriptor.is( got ) );
    test.true( descriptor.TypeName === 'F32x' || descriptor.TypeName === 'Fx' ? src === got : src !== got );

    /* */

    test.case = 'src = empty arguments array';
    var src = _.argumentsArray.make( [] );
    var got = descriptor.from( src );
    var expected = new getExpectedLong( [] );
    test.equivalent( got, expected );
    test.true( descriptor.is( got ) );
    test.true( src !== got );

    test.case = 'src = arguments array, src.length = 1';
    var src = _.argumentsArray.make( [ {} ] );
    var got = descriptor.from( src );
    var expected = new getExpectedLong( [ {} ] );
    test.equivalent( got, expected );
    test.true( descriptor.is( got ) );
    test.true( src !== got );

    test.case = 'src = arguments array, src.length > 1';
    var src = _.argumentsArray.make( [ 1, 2, 3 ] );
    var got = descriptor.from( src );
    var expected = new getExpectedLong( [ 1, 2, 3 ] );
    test.equivalent( got, expected );
    test.true( descriptor.is( got ) );
    test.true( src !== got );

    /* */

    test.case = 'src = empty unroll';
    var src = _.unroll.make( [] );
    var got = descriptor.from( src );
    var expected = new getExpectedLong( [] );
    test.equivalent( got, expected );
    test.true( descriptor.is( got ) );
    test.true( src !== got );

    test.case = 'src = unroll, src.length = 1';
    var src = _.unroll.make( [ 'str' ] );
    var got = descriptor.from( src );
    var expected = new getExpectedLong( [ 'str' ] );
    test.equivalent( got, expected );
    test.true( descriptor.is( got ) );
    test.true( src !== got );

    test.case = 'src = unroll, src.length > 1';
    var src = _.unroll.make( [ 1, 2, 3 ] );
    var got = descriptor.from( src );
    var expected = new getExpectedLong( [ 1, 2, 3 ] );
    test.equivalent( got, expected );
    test.true( descriptor.is( got ) );
    test.true( src !== got );

    /* - */

    if( Config.debug )
    {
      test.case = 'extra arguments';
      test.shouldThrowErrorSync( () => descriptor.from( 1, 3 ) );
      test.shouldThrowErrorSync( () => descriptor.from( [], 3 ) );
    }
  }
}

// --
//
// --

const Proto =
{

  name : 'Tools.LongDescriptor.l0.l9',
  silencing : 1,
  routineTimeOut : 10000,

  tests :
  {

    isArrayUnrollArgumentsArray,
    isBufferTypedInstance,

    makeArrayUnrollArgumentsArray,
    makeBufferTypedInstance,

    fromArrayUnrollArgumentsArray,
    fromBufferTypedInstance,

  }

}

const Self = wTestSuite( Proto );
if( typeof module !== 'undefined' && !module.parent )
wTester.test( Self.name );

})();
