( function _l0_l9_Interval_test_s()
{

'use strict';

if( typeof module !== 'undefined' )
{
  const _ = require( 'Tools' );
  _.include( 'wTesting' );
}

const _ = _global_.wTools;

//--
// interval l0/l3/Interval.s
//--

function intervalIs( test )
{
  test.case = 'undefined';
  var got = _.intervalIs( undefined );
  var expected = false;
  test.identical( got, expected );

  test.case = 'null';
  var got = _.intervalIs( null );
  var expected = false;
  test.identical( got, expected );

  test.case = 'false';
  var got = _.intervalIs( false );
  var expected = false;
  test.identical( got, expected );

  test.case = 'empty string';
  var got = _.intervalIs( '' );
  var expected = false;
  test.identical( got, expected );

  test.case = 'zero';
  var got = _.intervalIs( 0 );
  var expected = false;
  test.identical( got, expected );

  test.case = 'NaN';
  var got = _.intervalIs( NaN );
  var expected = false;
  test.identical( got, expected );

  test.case = 'a boolean';
  var got = _.intervalIs( true );
  var expected = false;
  test.identical( got, expected );

  test.case = 'a number';
  var got = _.intervalIs( 13 );
  var expected = false;
  test.identical( got, expected );

  test.case = 'a function';
  var got = _.intervalIs( function() {} );
  var expected = false;
  test.identical( got, expected );

  test.case = 'constructor';
  function Constr( x )
  {
    this.x = x;
    return this;
  }
  var got = _.intervalIs( new Constr( 0 ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'a string';
  var got = _.intervalIs( 'str' );
  var expected = false;
  test.identical( got, expected );

  test.case = 'BufferRaw';
  var got = _.intervalIs( new BufferRaw( 5 ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'BufferView';
  var got = _.intervalIs( new BufferView( new BufferRaw( 5 ) ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'Set';
  var got = _.intervalIs( new Set( [ 5 ] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'Map';
  var got = _.intervalIs( new HashMap( [ [ 1, 2 ] ] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'pure empty map';
  var got = _.intervalIs( Object.create( null ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'pure map';
  var src = Object.create( null );
  src.x = 1;
  var got = _.intervalIs( src );
  var expected = false;
  test.identical( got, expected );

  test.case = 'map from pure map';
  var src = Object.create( Object.create( null ) );
  var got = _.intervalIs( src );
  var expected = false;
  test.identical( got, expected );

  test.case = 'an empty object';
  var got = _.intervalIs( {} );
  var expected = false;
  test.identical( got, expected );

  test.case = 'an object';
  var got = _.intervalIs( { a : 7, b : 13 } );
  var expected = false;
  test.identical( got, expected );

  /* */

  test.case = 'array.length = 0';
  var got = _.intervalIs( [] );
  var expected = false;
  test.identical( got, expected );

  test.case = 'array.length = 1';
  var got = _.intervalIs( [ 1 ] );
  var expected = false;
  test.identical( got, expected );

  test.case = 'array.length = 2, numbers';
  var got = _.intervalIs( [ 1, 2 ] );
  var expected = true;
  test.identical( got, expected );

  test.case = 'array.length = 2, number and undefined';
  var got = _.intervalIs( [ 1, undefined ] );
  var expected = false;
  test.identical( got, expected );

  test.case = 'array.length > 2';
  var got = _.intervalIs( [ 1, 2, 3 ] );
  var expected = false;
  test.identical( got, expected );

  test.case = 'unroll.length = 0';
  var got = _.intervalIs( _.unroll.make( [] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'unroll.length = 1';
  var got = _.intervalIs( _.unroll.make( [ 1 ] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'unroll.length = 2, numbers';
  var got = _.intervalIs( _.unroll.make( [ 1, 2 ] ) );
  var expected = true;
  test.identical( got, expected );

  test.case = 'unroll.length = 2, number and undefined';
  var got = _.intervalIs( _.unroll.make( [ 1, undefined ] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'unroll.length > 2';
  var got = _.intervalIs( _.unroll.make( [ 1, 2, 3 ] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'argumentsArray.length = 0';
  var got = _.intervalIs( _.argumentsArray.make( [] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'argumentsArray.length = 1';
  var got = _.intervalIs( _.argumentsArray.make( [ 1 ] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'argumentsArray.length = 2, numbers';
  var got = _.intervalIs( _.argumentsArray.make( [ 1, 2 ] ) );
  var expected = true;
  test.identical( got, expected );

  test.case = 'argumentsArray.length = 2, number and undefined';
  var got = _.intervalIs( _.argumentsArray.make( [ 1, undefined ] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'argumentsArray.length > 2';
  var got = _.intervalIs( _.argumentsArray.make( [ 1, 2, 3 ] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'BufferTyped.length = 0';
  var got = _.intervalIs( new U8x() );
  var expected = false;
  test.identical( got, expected );

  test.case = 'BufferTyped.length = 1';
  var got = _.intervalIs( new I16x( 1 ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'BufferTyped.length = 2';
  var got = _.intervalIs( new F32x( 2 ) );
  var expected = true;
  test.identical( got, expected );

  test.case = 'BufferTyped.length > 2';
  var got = _.intervalIs( new F32x( 4 ) );
  var expected = false;
  test.identical( got, expected );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.intervalIs() );

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.intervalIs( [ 1, 2 ], 'extra' ) );
}

//

function intervalIsValid( test )
{
  test.case = 'not a range, long with 1 element';
  var got = _.intervalIsValid([ 1 ]);
  var expected = false;
  test.identical( got, expected );

  test.case = 'not a range, long with 3 elements';
  var got = _.intervalIsValid([ 1, 2, 3 ]);
  var expected = false;
  test.identical( got, expected );

  test.case = 'range, elements isValid undefined';
  var got = _.intervalIsValid([ undefined, undefined ]);
  var expected = false;
  test.identical( got, expected );

  test.case = 'range, first element isValid undefined';
  var got = _.intervalIsValid([ undefined, 2 ]);
  var expected = false;
  test.identical( got, expected );

  test.case = 'range, second element isValid undefined';
  var got = _.intervalIsValid([ 2, undefined ]);
  var expected = false;
  test.identical( got, expected );

  /* */

  test.case = 'range, two NaN';
  var got = _.intervalIsValid([ NaN, NaN ]);
  var expected = false;
  test.identical( got, expected );

  test.case = 'range, first element isValid NaN';
  var got = _.intervalIsValid([ NaN, 2 ]);
  var expected = false;
  test.identical( got, expected );

  test.case = 'range, second element isValid NaN';
  var got = _.intervalIsValid([ 2, NaN ]);
  var expected = false;
  test.identical( got, expected );

  /* */

  test.case = 'range, two not integer numbers';
  var got = _.intervalIsValid([ 2.01, 10/3 ]);
  var expected = false;
  test.identical( got, expected );

  test.case = 'range, first element isValid not an integer';
  var got = _.intervalIsValid([ 2.01, 2 ]);
  var expected = false;
  test.identical( got, expected );

  test.case = 'range, second element isValid not an integer';
  var got = _.intervalIsValid([ 2, 10/3 ]);
  var expected = false;
  test.identical( got, expected );

  /* */

  test.case = 'range, two Infinity';
  var got = _.intervalIsValid([ Infinity, Infinity ]);
  var expected = false;
  test.identical( got, expected );

  test.case = 'range, first element isValid Infinity';
  var got = _.intervalIsValid([ Infinity, 2 ]);
  var expected = false;
  test.identical( got, expected );

  test.case = 'range, second element isValid Infinity';
  var got = _.intervalIsValid([ 2, Infinity ]);
  var expected = false;
  test.identical( got, expected );

  /* */

  test.case = 'range, two positive integer';
  var got = _.intervalIsValid([ 2, 10 ]);
  var expected = true;
  test.identical( got, expected );

  test.case = 'range, two negative integer';
  var got = _.intervalIsValid([ -2, -10 ]);
  var expected = true;
  test.identical( got, expected );

  test.case = 'range, first element +0';
  var got = _.intervalIsValid([ +0, -10 ]);
  var expected = true;
  test.identical( got, expected );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.intervalIsValid() );

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.intervalIsValid( [ 1, 2 ], 'extra' ) );
}

// --
// common
// --

function is( test )
{
  let namespaces =
  [
    'cinterval',
    'linterval',
    'ointerval',
  ];

  for( let i = 0 ; i < namespaces.length ; i++ )
  {
    test.open( `namespace - ${ namespaces[ i ] }` );
    testRun( namespaces[ i ] );
    test.close( `namespace - ${ namespaces[ i ] }` );
  }

  /* - */

  function testRun( namespace )
  {
    test.case = 'undefined';
    var got = _[ namespace ].is( undefined );
    var expected = false;
    test.identical( got, expected );

    test.case = 'null';
    var got = _[ namespace ].is( null );
    var expected = false;
    test.identical( got, expected );

    test.case = 'false';
    var got = _[ namespace ].is( false );
    var expected = false;
    test.identical( got, expected );

    test.case = 'empty string';
    var got = _[ namespace ].is( '' );
    var expected = false;
    test.identical( got, expected );

    test.case = 'zero';
    var got = _[ namespace ].is( 0 );
    var expected = false;
    test.identical( got, expected );

    test.case = 'NaN';
    var got = _[ namespace ].is( NaN );
    var expected = false;
    test.identical( got, expected );

    test.case = 'a boolean';
    var got = _[ namespace ].is( true );
    var expected = false;
    test.identical( got, expected );

    test.case = 'a number';
    var got = _[ namespace ].is( 13 );
    var expected = false;
    test.identical( got, expected );

    test.case = 'a function';
    var got = _[ namespace ].is( function() {} );
    var expected = false;
    test.identical( got, expected );

    test.case = 'constructor';
    function Constr( x )
    {
      this.x = x;
      return this;
    }
    var got = _[ namespace ].is( new Constr( 0 ) );
    var expected = false;
    test.identical( got, expected );

    test.case = 'a string';
    var got = _[ namespace ].is( 'str' );
    var expected = false;
    test.identical( got, expected );

    test.case = 'BufferRaw';
    var got = _[ namespace ].is( new BufferRaw( 5 ) );
    var expected = false;
    test.identical( got, expected );

    test.case = 'BufferView';
    var got = _[ namespace ].is( new BufferView( new BufferRaw( 5 ) ) );
    var expected = false;
    test.identical( got, expected );

    test.case = 'Set';
    var got = _[ namespace ].is( new Set( [ 5 ] ) );
    var expected = false;
    test.identical( got, expected );

    test.case = 'Map';
    var got = _[ namespace ].is( new HashMap( [ [ 1, 2 ] ] ) );
    var expected = false;
    test.identical( got, expected );

    test.case = 'pure empty map';
    var got = _[ namespace ].is( Object.create( null ) );
    var expected = false;
    test.identical( got, expected );

    test.case = 'pure map';
    var src = Object.create( null );
    src.x = 1;
    var got = _[ namespace ].is( src );
    var expected = false;
    test.identical( got, expected );

    test.case = 'map from pure map';
    var src = Object.create( Object.create( null ) );
    var got = _[ namespace ].is( src );
    var expected = false;
    test.identical( got, expected );

    test.case = 'an empty object';
    var got = _[ namespace ].is( {} );
    var expected = false;
    test.identical( got, expected );

    test.case = 'an object';
    var got = _[ namespace ].is( { a : 7, b : 13 } );
    var expected = false;
    test.identical( got, expected );

    /* */

    test.case = 'array.length = 0';
    var got = _[ namespace ].is( [] );
    var expected = false;
    test.identical( got, expected );

    test.case = 'array.length = 1';
    var got = _[ namespace ].is( [ 1 ] );
    var expected = false;
    test.identical( got, expected );

    test.case = 'array.length = 2, numbers';
    var got = _[ namespace ].is( [ 1, 2 ] );
    var expected = true;
    test.identical( got, expected );

    test.case = 'array.length = 2, number and undefined';
    var got = _[ namespace ].is( [ 1, undefined ] );
    var expected = false;
    test.identical( got, expected );

    test.case = 'array.length > 2';
    var got = _[ namespace ].is( [ 1, 2, 3 ] );
    var expected = false;
    test.identical( got, expected );

    test.case = 'unroll.length = 0';
    var got = _[ namespace ].is( _.unroll.make( [] ) );
    var expected = false;
    test.identical( got, expected );

    test.case = 'unroll.length = 1';
    var got = _[ namespace ].is( _.unroll.make( [ 1 ] ) );
    var expected = false;
    test.identical( got, expected );

    test.case = 'unroll.length = 2, numbers';
    var got = _[ namespace ].is( _.unroll.make( [ 1, 2 ] ) );
    var expected = true;
    test.identical( got, expected );

    test.case = 'unroll.length = 2, number and undefined';
    var got = _[ namespace ].is( _.unroll.make( [ 1, undefined ] ) );
    var expected = false;
    test.identical( got, expected );

    test.case = 'unroll.length > 2';
    var got = _[ namespace ].is( _.unroll.make( [ 1, 2, 3 ] ) );
    var expected = false;
    test.identical( got, expected );

    test.case = 'argumentsArray.length = 0';
    var got = _[ namespace ].is( _.argumentsArray.make( [] ) );
    var expected = false;
    test.identical( got, expected );

    test.case = 'argumentsArray.length = 1';
    var got = _[ namespace ].is( _.argumentsArray.make( [ 1 ] ) );
    var expected = false;
    test.identical( got, expected );

    test.case = 'argumentsArray.length = 2, numbers';
    var got = _[ namespace ].is( _.argumentsArray.make( [ 1, 2 ] ) );
    var expected = true;
    test.identical( got, expected );

    test.case = 'argumentsArray.length = 2, number and undefined';
    var got = _[ namespace ].is( _.argumentsArray.make( [ 1, undefined ] ) );
    var expected = false;
    test.identical( got, expected );

    test.case = 'argumentsArray.length > 2';
    var got = _[ namespace ].is( _.argumentsArray.make( [ 1, 2, 3 ] ) );
    var expected = false;
    test.identical( got, expected );

    test.case = 'BufferTyped.length = 0';
    var got = _[ namespace ].is( new U8x() );
    var expected = false;
    test.identical( got, expected );

    test.case = 'BufferTyped.length = 1';
    var got = _[ namespace ].is( new I16x( 1 ) );
    var expected = false;
    test.identical( got, expected );

    test.case = 'BufferTyped.length = 2';
    var got = _[ namespace ].is( new F32x( 2 ) );
    var expected = true;
    test.identical( got, expected );

    test.case = 'BufferTyped.length > 2';
    var got = _[ namespace ].is( new F32x( 4 ) );
    var expected = false;
    test.identical( got, expected );

    /* - */

    if( !Config.debug )
    return;

    test.case = 'without arguments';
    test.shouldThrowErrorSync( () => _[ namespace ].is() );

    test.case = 'extra arguments';
    test.shouldThrowErrorSync( () => _[ namespace ].is( [ 1, 2 ], 'extra' ) );
  }
}

//

function isValid( test )
{
  let namespaces =
  [
    'cinterval',
    'linterval',
    'ointerval',
  ];

  for( let i = 0 ; i < namespaces.length ; i++ )
  {
    test.open( `namespace - ${ namespaces[ i ] }` );
    testRun( namespaces[ i ] );
    test.close( `namespace - ${ namespaces[ i ] }` );
  }

  /* - */

  function testRun( namespace )
  {
    test.case = 'not a range, long with 1 element';
    var got = _[ namespace ].isValid([ 1 ]);
    var expected = false;
    test.identical( got, expected );

    test.case = 'not a range, long with 3 elements';
    var got = _[ namespace ].isValid([ 1, 2, 3 ]);
    var expected = false;
    test.identical( got, expected );

    test.case = 'range, elements isValid undefined';
    var got = _[ namespace ].isValid([ undefined, undefined ]);
    var expected = false;
    test.identical( got, expected );

    test.case = 'range, first element isValid undefined';
    var got = _[ namespace ].isValid([ undefined, 2 ]);
    var expected = false;
    test.identical( got, expected );

    test.case = 'range, second element isValid undefined';
    var got = _[ namespace ].isValid([ 2, undefined ]);
    var expected = false;
    test.identical( got, expected );

    /* */

    test.case = 'range, two NaN';
    var got = _[ namespace ].isValid([ NaN, NaN ]);
    var expected = false;
    test.identical( got, expected );

    test.case = 'range, first element isValid NaN';
    var got = _[ namespace ].isValid([ NaN, 2 ]);
    var expected = false;
    test.identical( got, expected );

    test.case = 'range, second element isValid NaN';
    var got = _[ namespace ].isValid([ 2, NaN ]);
    var expected = false;
    test.identical( got, expected );

    /* */

    test.case = 'range, two not integer numbers';
    var got = _[ namespace ].isValid([ 2.01, 10/3 ]);
    var expected = false;
    test.identical( got, expected );

    test.case = 'range, first element isValid not an integer';
    var got = _[ namespace ].isValid([ 2.01, 2 ]);
    var expected = false;
    test.identical( got, expected );

    test.case = 'range, second element isValid not an integer';
    var got = _[ namespace ].isValid([ 2, 10/3 ]);
    var expected = false;
    test.identical( got, expected );

    /* */

    test.case = 'range, two Infinity';
    var got = _[ namespace ].isValid([ Infinity, Infinity ]);
    var expected = false;
    test.identical( got, expected );

    test.case = 'range, first element isValid Infinity';
    var got = _[ namespace ].isValid([ Infinity, 2 ]);
    var expected = false;
    test.identical( got, expected );

    test.case = 'range, second element isValid Infinity';
    var got = _[ namespace ].isValid([ 2, Infinity ]);
    var expected = false;
    test.identical( got, expected );

    /* */

    test.case = 'range, two positive integer';
    var got = _[ namespace ].isValid([ 2, 10 ]);
    var expected = true;
    test.identical( got, expected );

    test.case = 'range, two negative integer';
    var got = _[ namespace ].isValid([ -2, -10 ]);
    var expected = true;
    test.identical( got, expected );

    test.case = 'range, first element +0';
    var got = _[ namespace ].isValid([ +0, -10 ]);
    var expected = true;
    test.identical( got, expected );

    /* - */

    if( !Config.debug )
    return;

    test.case = 'without arguments';
    test.shouldThrowErrorSync( () => _[ namespace ].isValid() );

    test.case = 'extra arguments';
    test.shouldThrowErrorSync( () => _[ namespace ].isValid( [ 1, 2 ], 'extra' ) );
  }
}

//

function fromLeft( test )
{
  let namespaces =
  [
    'cinterval',
    'linterval',
    'ointerval',
  ];

  for( let i = 0 ; i < namespaces.length ; i++ )
  {
    test.open( `namespace - ${ namespaces[ i ] }` );
    testRun( namespaces[ i ] );
    test.close( `namespace - ${ namespaces[ i ] }` );
  }

  /* - */

  function testRun( namespace )
  {
    test.case = 'range - number';
    var got = _[ namespace ].fromLeft( 1 );
    test.identical( got, [ 1, Infinity ] );

    /* */

    test.case = 'range.length === 1';
    var src = [ 1 ];
    var got = _[ namespace ].fromLeft( src );
    test.identical( got, [ 1, Infinity ] );
    test.true( got !== src );

    test.case = 'range[ 0 ] - undefined';
    var src = [ undefined, 1 ];
    var got = _[ namespace ].fromLeft( src );
    test.identical( got, [ 0, 1 ] );
    test.true( got === src );

    test.case = 'range[ 1 ] - undefined';
    var src = [ 1, undefined ];
    var got = _[ namespace ].fromLeft( src );
    test.identical( got, [ 1, Infinity ] );
    test.true( got === src );

    test.case = 'range[ 0 ] - undefined, range[ 1 ] - undefined';
    var src = [ undefined, undefined ];
    var got = _[ namespace ].fromLeft( src );
    test.identical( got, [ 0, Infinity ] );
    test.true( got === src );

    test.case = 'range[ 0 ] - number, range[ 1 ] - number';
    var src = [ 1, 5 ];
    var got = _[ namespace ].fromLeft( src );
    test.identical( got, [ 1, 5 ] );
    test.true( got === src );

    /* */

    test.case = 'range.length === 1';
    var src = _.unroll.make( [ 1 ] );
    var got = _[ namespace ].fromLeft( src );
    test.identical( got, [ 1, Infinity ] );
    test.true( got !== src );

    test.case = 'range[ 0 ] - undefined';
    var src = _.unroll.make( [ undefined, 1 ] );
    var got = _[ namespace ].fromLeft( src );
    test.identical( got, [ 0, 1 ] );
    test.true( got === src );

    test.case = 'range[ 1 ] - undefined';
    var src = _.unroll.make( [ 1, undefined ] );
    var got = _[ namespace ].fromLeft( src );
    test.identical( got, [ 1, Infinity ] );
    test.true( got === src );

    test.case = 'range[ 0 ] - undefined, range[ 1 ] - undefined';
    var src = _.unroll.make( [ undefined, undefined ] );
    var got = _[ namespace ].fromLeft( src );
    test.identical( got, [ 0, Infinity ] );
    test.true( got === src );

    test.case = 'range[ 0 ] - number, range[ 1 ] - number';
    var src = _.unroll.make( [ 1, 5 ] );
    var got = _[ namespace ].fromLeft( src );
    test.identical( got, _.unroll.make( [ 1, 5 ] ) );
    test.true( got === src );

    /* */

    test.case = 'range.length === 1';
    var src = _.argumentsArray.make( [ 1 ] );
    var got = _[ namespace ].fromLeft( src );
    test.identical( got, [ 1, Infinity ] );
    test.true( got !== src );

    test.case = 'range[ 0 ] - undefined';
    var src = _.argumentsArray.make( [ undefined, 1 ] );
    var got = _[ namespace ].fromLeft( src );
    test.identical( got, _.argumentsArray.make([ 0, 1 ]) );
    test.true( got === src );

    test.case = 'range[ 1 ] - undefined';
    var src = _.argumentsArray.make( [ 1, undefined ] );
    var got = _[ namespace ].fromLeft( src );
    test.identical( got, _.argumentsArray.make([ 1, Infinity ]) );
    test.true( got === src );

    test.case = 'range[ 0 ] - undefined, range[ 1 ] - undefined';
    var src = _.argumentsArray.make( [ undefined, undefined ] );
    var got = _[ namespace ].fromLeft( src );
    test.identical( got, _.argumentsArray.make([ 0, Infinity ]) );
    test.true( got === src );

    test.case = 'range[ 0 ] - number, range[ 1 ] - number';
    var src = _.argumentsArray.make( [ 1, 5 ] );
    var got = _[ namespace ].fromLeft( src );
    test.identical( got, _.argumentsArray.make( [ 1, 5 ] ) );
    test.true( got === src );

    /* */

    test.case = 'range.length === 1';
    var src = new U8x( [ 1 ] );
    var got = _[ namespace ].fromLeft( src );
    test.identical( got, [ 1, Infinity ] );
    test.true( got !== src );

    test.case = 'range[ 0 ] - number, range[ 1 ] - number';
    var src = new I16x( [ 1, 5 ] );
    var got = _[ namespace ].fromLeft( src );
    test.identical( got, new I16x( [ 1, 5 ] ) );
    test.true( got === src );

    /* - */

    if( !Config.debug )
    return;

    test.case = 'without arguments';
    test.shouldThrowErrorSync( () => _[ namespace ].fromLeft() );

    test.case = 'extra arguments';
    test.shouldThrowErrorSync( () => _[ namespace ].fromLeft( [ 1, 2 ], 1 ) );

    test.case = 'wrong type of range';
    test.shouldThrowErrorSync( () => _[ namespace ].fromLeft( { 0 : 1, 1 : 2 } ) );

    test.case = 'wrong range length';
    test.shouldThrowErrorSync( () => _[ namespace ].fromLeft( [ 0, 1, 2 ] ) );

    test.case = 'wrong elements in range';
    test.shouldThrowErrorSync( () => _[ namespace ].fromLeft( [ null, 2 ] ) );
    test.shouldThrowErrorSync( () => _[ namespace ].fromLeft( [ 2, 'abc' ] ) );
  }
}

fromLeft.timeOut = 10000;

//

function fromRight( test )
{
  let namespaces =
  [
    'cinterval',
    'linterval',
    'ointerval',
  ];

  for( let i = 0 ; i < namespaces.length ; i++ )
  {
    test.open( `namespace - ${ namespaces[ i ] }` );
    testRun( namespaces[ i ] );
    test.close( `namespace - ${ namespaces[ i ] }` );
  }

  /* - */

  function testRun( namespace )
  {
    test.case = 'range - number';
    var got = _[ namespace ].fromRight( 1 );
    test.identical( got, [ 0, 1 ] );

    /* */

    test.case = 'range.length === 1';
    var src = [ 1 ];
    var got = _[ namespace ].fromRight( src );
    test.identical( got, [ 1, Infinity ] );
    test.true( got !== src );

    test.case = 'range[ 0 ] - undefined';
    var src = [ undefined, 1 ];
    var got = _[ namespace ].fromRight( src );
    test.identical( got, [ 0, 1 ] );
    test.true( got === src );

    test.case = 'range[ 1 ] - undefined';
    var src = [ 1, undefined ];
    var got = _[ namespace ].fromRight( src );
    test.identical( got, [ 1, Infinity ] );
    test.true( got === src );

    test.case = 'range[ 0 ] - undefined, range[ 1 ] - undefined';
    var src = [ undefined, undefined ];
    var got = _[ namespace ].fromRight( src );
    test.identical( got, [ 0, Infinity ] );
    test.true( got === src );

    test.case = 'range[ 0 ] - number, range[ 1 ] - number';
    var src = [ 1, 5 ];
    var got = _[ namespace ].fromRight( src );
    test.identical( got, [ 1, 5 ] );
    test.true( got === src );

    /* */

    test.case = 'range.length === 1';
    var src = _.unroll.make( [ 1 ] );
    var got = _[ namespace ].fromRight( src );
    test.identical( got, [ 1, Infinity ] );
    test.true( got !== src );

    test.case = 'range[ 0 ] - undefined';
    var src = _.unroll.make( [ undefined, 1 ] );
    var got = _[ namespace ].fromRight( src );
    test.identical( got, [ 0, 1 ] );
    test.true( got === src );

    test.case = 'range[ 1 ] - undefined';
    var src = _.unroll.make( [ 1, undefined ] );
    var got = _[ namespace ].fromRight( src );
    test.identical( got, [ 1, Infinity ] );
    test.true( got === src );

    test.case = 'range[ 0 ] - undefined, range[ 1 ] - undefined';
    var src = _.unroll.make( [ undefined, undefined ] );
    var got = _[ namespace ].fromRight( src );
    test.identical( got, [ 0, Infinity ] );
    test.true( got === src );

    test.case = 'range[ 0 ] - number, range[ 1 ] - number';
    var src = _.unroll.make( [ 1, 5 ] );
    var got = _[ namespace ].fromRight( src );
    test.identical( got, _.unroll.make( [ 1, 5 ] ) );
    test.true( got === src );

    /* */

    test.case = 'range.length === 1';
    var src = _.argumentsArray.make( [ 1 ] );
    var got = _[ namespace ].fromRight( src );
    test.identical( got, [ 1, Infinity ] );
    test.true( got !== src );

    test.case = 'range[ 0 ] - undefined';
    var src = _.argumentsArray.make( [ undefined, 1 ] );
    var got = _[ namespace ].fromRight( src );
    test.identical( got, _.argumentsArray.make([ 0, 1 ]) );
    test.true( got === src );

    test.case = 'range[ 1 ] - undefined';
    var src = _.argumentsArray.make( [ 1, undefined ] );
    var got = _[ namespace ].fromRight( src );
    test.identical( got, _.argumentsArray.make([ 1, Infinity ]) );
    test.true( got === src );

    test.case = 'range[ 0 ] - undefined, range[ 1 ] - undefined';
    var src = _.argumentsArray.make( [ undefined, undefined ] );
    var got = _[ namespace ].fromRight( src );
    test.identical( got, _.argumentsArray.make([ 0, Infinity ]) );
    test.true( got === src );

    test.case = 'range[ 0 ] - number, range[ 1 ] - number';
    var src = _.argumentsArray.make( [ 1, 5 ] );
    var got = _[ namespace ].fromRight( src );
    test.identical( got, _.argumentsArray.make( [ 1, 5 ] ) );
    test.true( got === src );

    /* */

    test.case = 'range.length === 1';
    var src = new U8x( [ 1 ] );
    var got = _[ namespace ].fromRight( src );
    test.identical( got, [ 1, Infinity ] );
    test.true( got !== src );

    test.case = 'range[ 0 ] - number, range[ 1 ] - number';
    var src = new I16x( [ 1, 5 ] );
    var got = _[ namespace ].fromRight( src );
    test.identical( got, new I16x( [ 1, 5 ] ) );
    test.true( got === src );

    /* - */

    if( !Config.debug )
    return;

    test.case = 'without arguments';
    test.shouldThrowErrorSync( () => _[ namespace ].fromRight() );

    test.case = 'extra arguments';
    test.shouldThrowErrorSync( () => _[ namespace ].fromRight( [ 1, 2 ], 1 ) );

    test.case = 'wrong type of range';
    test.shouldThrowErrorSync( () => _[ namespace ].fromRight( { 0 : 1, 1 : 2 } ) );

    test.case = 'wrong range length';
    test.shouldThrowErrorSync( () => _[ namespace ].fromRight( [ 0, 1, 2 ] ) );

    test.case = 'wrong elements in range';
    test.shouldThrowErrorSync( () => _[ namespace ].fromRight( [ null, 2 ] ) );
    test.shouldThrowErrorSync( () => _[ namespace ].fromRight( [ 2, 'abc' ] ) );
  }
}

//

function firstGet( test )
{
  let namespaces =
  [
    'cinterval',
    'linterval',
    'ointerval',
  ];

  for( let i = 0 ; i < namespaces.length ; i++ )
  {
    test.open( `namespace - ${ namespaces[ i ] }` );
    testRun( namespaces[ i ] );
    test.close( `namespace - ${ namespaces[ i ] }` );
  }

  /* - */

  function testRun( namespace )
  {
    test.open( 'range - array' );

    test.case = 'without options';
    var got = _[ namespace ].firstGet( [ 1, 2 ] );
    test.identical( got, 1 );

    // test.case = 'options - undefined';
    // var options = undefined;
    // var got = _[ namespace ].firstGet( [ 1, 2 ], options );
    // test.identical( got, 1 );
    // test.identical( options, undefined );
    //
    // test.case = 'options - empty map';
    // var options = {};
    // var got = _[ namespace ].firstGet( [ 1, 2 ], options );
    // test.identical( got, 1 );
    // test.identical( options, { increment : 1 } );
    //
    // test.case = 'options - map with own field increment';
    // var options = { increment : 2 };
    // var got = _[ namespace ].firstGet( [ 1, 2 ], options );
    // test.identical( got, 1 );
    // test.identical( options, { increment : 2 } );
    //
    // test.case = 'options - map with own field increment - undefined';
    // var options = { increment : undefined };
    // var got = _[ namespace ].firstGet( [ 1, 2 ], options );
    // test.identical( got, 1 );
    // test.identical( options, { increment : 1 } );

    test.close( 'range - array' );

    /* - */

    test.open( 'range - unroll' );

    test.case = 'without options';
    var got = _[ namespace ].firstGet( _.unroll.make( [ 1, 2 ] ) );
    test.identical( got, 1 );

    // test.case = 'options - undefined';
    // var options = undefined;
    // var got = _[ namespace ].firstGet( _.unroll.make( [ 1, 2 ] ), options );
    // test.identical( got, 1 );
    // test.identical( options, undefined );
    //
    // test.case = 'options - empty map';
    // var options = {};
    // var got = _[ namespace ].firstGet( _.unroll.make( [ 1, 2 ] ), options );
    // test.identical( got, 1 );
    // test.identical( options, { increment : 1 } );
    //
    // test.case = 'options - map with own field increment';
    // var options = { increment : 2 };
    // var got = _[ namespace ].firstGet( _.unroll.make( [ 1, 2 ] ), options );
    // test.identical( got, 1 );
    // test.identical( options, { increment : 2 } );
    //
    // test.case = 'options - map with own field increment - undefined';
    // var options = { increment : undefined };
    // var got = _[ namespace ].firstGet( _.unroll.make( [ 1, 2 ] ), options );
    // test.identical( got, 1 );
    // test.identical( options, { increment : 1 } );

    test.close( 'range - unroll' );

    /* - */

    test.open( 'range - argumentsArray' );

    test.case = 'without options';
    var got = _[ namespace ].firstGet( _.argumentsArray.make( [ 1, 2 ] ) );
    test.identical( got, 1 );

    // test.case = 'options - undefined';
    // var options = undefined;
    // var got = _[ namespace ].firstGet( _.argumentsArray.make( [ 1, 2 ] ), options );
    // test.identical( got, 1 );
    // test.identical( options, undefined );
    //
    // test.case = 'options - empty map';
    // var options = {};
    // var got = _[ namespace ].firstGet( _.argumentsArray.make( [ 1, 2 ] ), options );
    // test.identical( got, 1 );
    // test.identical( options, { increment : 1 } );
    //
    // test.case = 'options - map with own field increment';
    // var options = { increment : 2 };
    // var got = _[ namespace ].firstGet( _.argumentsArray.make( [ 1, 2 ] ), options );
    // test.identical( got, 1 );
    // test.identical( options, { increment : 2 } );
    //
    // test.case = 'options - map with own field increment - undefined';
    // var options = { increment : undefined };
    // var got = _[ namespace ].firstGet( _.argumentsArray.make( [ 1, 2 ] ), options );
    // test.identical( got, 1 );
    // test.identical( options, { increment : 1 } );

    test.close( 'range - argumentsArray' );

    /* - */

    test.open( 'range - BufferTyped' );

    test.case = 'without options';
    var got = _[ namespace ].firstGet( new U8x( [ 1, 2 ] ) );
    test.identical( got, 1 );

    // test.case = 'options - undefined';
    // var options = undefined;
    // var got = _[ namespace ].firstGet( new I16x( [ 1, 2 ] ), options );
    // test.identical( got, 1 );
    // test.identical( options, undefined );
    //
    // test.case = 'options - empty map';
    // var options = {};
    // var got = _[ namespace ].firstGet( new F32x( [ 1, 2 ] ), options );
    // test.identical( got, 1 );
    // test.identical( options, { increment : 1 } );
    //
    // test.case = 'options - map with own field increment';
    // var options = { increment : 2 };
    // var got = _[ namespace ].firstGet( new F64x( [ 1, 2 ] ), options );
    // test.identical( got, 1 );
    // test.identical( options, { increment : 2 } );
    //
    // test.case = 'options - map with own field increment - undefined';
    // var options = { increment : undefined };
    // var got = _[ namespace ].firstGet( new I8x( [ 1, 2 ] ), options );
    // test.identical( got, 1 );
    // test.identical( options, { increment : 1 } );

    test.close( 'range - BufferTyped' );

    /* - */

    test.open( 'range - map' );

    test.case = 'without options, first - undefined';
    var got = _[ namespace ].firstGet( { first : undefined, last : undefined } );
    test.identical( got, undefined );

    test.case = 'without options, first - not exists';
    var got = _[ namespace ].firstGet( { last : undefined } );
    test.identical( got, undefined );

    test.case = 'without options, first - number';
    var got = _[ namespace ].firstGet( { first : 1, last : undefined } );
    test.identical( got, 1 );

    /* */

    // test.case = 'options - undefined, first - undefined';
    // var options = undefined;
    // var got = _[ namespace ].firstGet( { first : undefined, last : undefined }, options );
    // test.identical( got, undefined );
    // test.identical( options, undefined );
    //
    // test.case = 'options - undefined, first - not exists';
    // var options = undefined;
    // var got = _[ namespace ].firstGet( { last : undefined }, options );
    // test.identical( got, undefined );
    // test.identical( options, undefined );
    //
    // test.case = 'options - undefined, first - number';
    // var options = undefined;
    // var got = _[ namespace ].firstGet( { first : 1, last : undefined }, options );
    // test.identical( got, 1 );
    // test.identical( options, undefined );
    //
    // /* */
    //
    // test.case = 'options - empty map, first - undefined';
    // var options = {};
    // var got = _[ namespace ].firstGet( { first : undefined, last : undefined }, options );
    // test.identical( got, undefined );
    // test.identical( options, { increment : 1 } );
    //
    // test.case = 'options - empty map, first - not exists';
    // var options = {};
    // var got = _[ namespace ].firstGet( { last : undefined }, options );
    // test.identical( got, undefined );
    // test.identical( options, { increment : 1 } );
    //
    // test.case = 'options - empty map, first - number';
    // var options = {};
    // var got = _[ namespace ].firstGet( { first : 1, last : undefined }, options );
    // test.identical( got, 1 );
    // test.identical( options, { increment : 1 } );
    //
    // /* */
    //
    // test.case = 'options - map with own field increment, first - undefined';
    // var options = { increment : 2 };
    // var got = _[ namespace ].firstGet( { first : undefined, last : undefined }, options );
    // test.identical( got, undefined );
    // test.identical( options, { increment : 2 } );
    //
    // test.case = 'options - map with own field increment, first - not exists';
    // var options = { increment : 2 };
    // var got = _[ namespace ].firstGet( { last : undefined }, options );
    // test.identical( got, undefined );
    // test.identical( options, { increment : 2 } );
    //
    // test.case = 'options - map with own field increment, first - number';
    // var options = { increment : 2 };
    // var got = _[ namespace ].firstGet( { first : 1, last : undefined }, options );
    // test.identical( got, 1 );
    // test.identical( options, { increment : 2 } );
    //
    // /* */
    //
    // test.case = 'options - map with own field increment - undefined, first - undefined';
    // var options = { increment : undefined };
    // var got = _[ namespace ].firstGet( { first : undefined, last : undefined }, options );
    // test.identical( got, undefined );
    // test.identical( options, { increment : 1 } );
    //
    // test.case = 'options - map with own field increment - undefined, first - not exists';
    // var options = { increment : undefined };
    // var got = _[ namespace ].firstGet( { last : undefined }, options );
    // test.identical( got, undefined );
    // test.identical( options, { increment : 1 } );
    //
    // test.case = 'options - map with own field increment - undefined, first - number';
    // var options = { increment : undefined };
    // var got = _[ namespace ].firstGet( { first : 1, last : undefined }, options );
    // test.identical( got, 1 );
    // test.identical( options, { increment : 1 } );

    test.close( 'range - map' );

    /* - */

    if( !Config.debug )
    return;

    test.case = 'without arguments';
    test.shouldThrowErrorSync( () => _[ namespace ].firstGet() );

    test.case = 'extra arguments';
    test.shouldThrowErrorSync( () => _[ namespace ].firstGet( [ 1, 2 ], {}, 'extra' ) );

    test.case = 'range is a long, but is not a range';
    test.shouldThrowErrorSync( () => _[ namespace ].firstGet( [ 1, 2, 3 ] ) );
    test.shouldThrowErrorSync( () => _[ namespace ].firstGet( [ 1, 'wrong' ] ) );
    test.shouldThrowErrorSync( () => _[ namespace ].firstGet( [ undefined, 1 ] ) );

    test.case = 'wrong type of range';
    test.shouldThrowErrorSync( () => _[ namespace ].firstGet( new Set( [ 1, 2 ] ) ) );
    test.shouldThrowErrorSync( () => _[ namespace ].firstGet( new HashMap( [ [ 1, 2 ] ] ) ) );
  }
}

//

function toStr( test )
{
  let namespaces =
  [
    'cinterval',
    'linterval',
    'ointerval',
  ];

  for( let i = 0 ; i < namespaces.length ; i++ )
  {
    test.open( `namespace - ${ namespaces[ i ] }` );
    testRun( namespaces[ i ] );
    test.close( `namespace - ${ namespaces[ i ] }` );
  }

  /* - */

  function testRun( namespace )
  {
    test.case = 'range';
    var got = _[ namespace ].toStr( [ 1, 2 ] );
    test.identical( got, '1..2' );

    test.case = 'range with negative numbers';
    var got = _[ namespace ].toStr( [ -1, -2 ] );
    test.identical( got, '-1..-2' );

    /* - */

    if( !Config.debug )
    return;

    test.case = 'without arguments';
    test.shouldThrowErrorSync( () => _[ namespace ].toStr() );

    test.case = 'extra arguments';
    test.shouldThrowErrorSync( () => _[ namespace ].toStr( [ 1, 2 ], [ 1, 2 ] ) );

    test.case = 'range is not a range';
    test.shouldThrowErrorSync( () => _[ namespace ].toStr( [ 1, 2, 3 ] ) );
    test.shouldThrowErrorSync( () => _[ namespace ].toStr( [ 1, 'wrong' ] ) );
    test.shouldThrowErrorSync( () => _[ namespace ].toStr( [ undefined, 1 ] ) );
  }
}

// --
// cinterval
// --

function cintervalIsEmpty( test )
{
  test.case = 'undefined';
  var got = _.cinterval.isEmpty( undefined );
  var expected = false;
  test.identical( got, expected );

  test.case = 'null';
  var got = _.cinterval.isEmpty( null );
  var expected = false;
  test.identical( got, expected );

  test.case = 'false';
  var got = _.cinterval.isEmpty( false );
  var expected = false;
  test.identical( got, expected );

  test.case = 'empty string';
  var got = _.cinterval.isEmpty( '' );
  var expected = false;
  test.identical( got, expected );

  test.case = 'zero';
  var got = _.cinterval.isEmpty( 0 );
  var expected = false;
  test.identical( got, expected );

  test.case = 'NaN';
  var got = _.cinterval.isEmpty( NaN );
  var expected = false;
  test.identical( got, expected );

  test.case = 'a boolean';
  var got = _.cinterval.isEmpty( true );
  var expected = false;
  test.identical( got, expected );

  test.case = 'a number';
  var got = _.cinterval.isEmpty( 13 );
  var expected = false;
  test.identical( got, expected );

  test.case = 'a function';
  var got = _.cinterval.isEmpty( function() {} );
  var expected = false;
  test.identical( got, expected );

  test.case = 'constructor';
  function Constr( x )
  {
    this.x = x;
    return this;
  }
  var got = _.cinterval.isEmpty( new Constr( 0 ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'a string';
  var got = _.cinterval.isEmpty( 'str' );
  var expected = false;
  test.identical( got, expected );

  test.case = 'BufferRaw';
  var got = _.cinterval.isEmpty( new BufferRaw( 5 ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'BufferView';
  var got = _.cinterval.isEmpty( new BufferView( new BufferRaw( 5 ) ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'Set';
  var got = _.cinterval.isEmpty( new Set( [ 5 ] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'Map';
  var got = _.cinterval.isEmpty( new HashMap( [ [ 1, 2 ] ] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'pure empty map';
  var got = _.cinterval.isEmpty( Object.create( null ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'pure map';
  var src = Object.create( null );
  src.x = 1;
  var got = _.cinterval.isEmpty( src );
  var expected = false;
  test.identical( got, expected );

  test.case = 'map from pure map';
  var src = Object.create( Object.create( null ) );
  var got = _.cinterval.isEmpty( src );
  var expected = false;
  test.identical( got, expected );

  test.case = 'an empty object';
  var got = _.cinterval.isEmpty( {} );
  var expected = false;
  test.identical( got, expected );

  test.case = 'an object';
  var got = _.cinterval.isEmpty( { a : 7, b : 13 } );
  var expected = false;
  test.identical( got, expected );

  /* */

  test.case = 'array.length = 0';
  var got = _.cinterval.isEmpty( [] );
  var expected = false;
  test.identical( got, expected );

  test.case = 'array.length = 1';
  var got = _.cinterval.isEmpty( [ 1 ] );
  var expected = false;
  test.identical( got, expected );

  test.case = 'array.length = 2, different numbers';
  var got = _.cinterval.isEmpty( [ 1, 2 ] );
  var expected = false;
  test.identical( got, expected );

  test.case = 'array.length = 2, equal numbers';
  var got = _.cinterval.isEmpty( [ 1, 1 ] );
  var expected = false;
  test.identical( got, expected );

  test.case = 'array.length = 2, cinterval[ 0 ] - cinterval[ 1 ] === 1';
  var got = _.cinterval.isEmpty( [ 1, 0 ] );
  var expected = true;
  test.identical( got, expected );

  test.case = 'array.length = 2, number and undefined';
  var got = _.cinterval.isEmpty( [ 1, undefined ] );
  var expected = false;
  test.identical( got, expected );

  test.case = 'array.length > 2';
  var got = _.cinterval.isEmpty( [ 1, 2, 3 ] );
  var expected = false;
  test.identical( got, expected );

  test.case = 'unroll.length = 0';
  var got = _.cinterval.isEmpty( _.unroll.make( [] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'unroll.length = 1';
  var got = _.cinterval.isEmpty( _.unroll.make( [ 1 ] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'unroll.length = 2, different numbers';
  var got = _.cinterval.isEmpty( _.unroll.make( [ 1, 2 ] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'unroll.length = 2, equal numbers';
  var got = _.cinterval.isEmpty( _.unroll.make( [ 10, 10 ] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'unroll.length = 2, cinterval[ 0 ] - cinterval[ 1 ] === 1';
  var got = _.cinterval.isEmpty( _.unroll.make( [ 10, 9 ] ) );
  var expected = true;
  test.identical( got, expected );

  test.case = 'unroll.length = 2, number and undefined';
  var got = _.cinterval.isEmpty( _.unroll.make( [ 1, undefined ] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'unroll.length > 2';
  var got = _.cinterval.isEmpty( _.unroll.make( [ 1, 2, 3 ] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'argumentsArray.length = 0';
  var got = _.cinterval.isEmpty( _.argumentsArray.make( [] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'argumentsArray.length = 1';
  var got = _.cinterval.isEmpty( _.argumentsArray.make( [ 1 ] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'argumentsArray.length = 2, different numbers';
  var got = _.cinterval.isEmpty( _.argumentsArray.make( [ 1, 2 ] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'argumentsArray.length = 2, equal numbers';
  var got = _.cinterval.isEmpty( _.argumentsArray.make( [ -2, -2 ] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'argumentsArray.length = 2, cinterval[ 0 ] - cinterval[ 1 ] === 1';
  var got = _.cinterval.isEmpty( _.argumentsArray.make( [ -1, -2 ] ) );
  var expected = true;
  test.identical( got, expected );

  test.case = 'argumentsArray.length = 2, number and undefined';
  var got = _.cinterval.isEmpty( _.argumentsArray.make( [ 1, undefined ] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'argumentsArray.length > 2';
  var got = _.cinterval.isEmpty( _.argumentsArray.make( [ 1, 2, 3 ] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'BufferTyped.length = 0';
  var got = _.cinterval.isEmpty( new U8x() );
  var expected = false;
  test.identical( got, expected );

  test.case = 'BufferTyped.length = 1';
  var got = _.cinterval.isEmpty( new I16x( 1 ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'BufferTyped.length = 2, different numbers';
  var got = _.cinterval.isEmpty( new F32x( [ 1, 3 ] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'BufferTyped.length = 2, equal numbers';
  var got = _.cinterval.isEmpty( new F32x( [ 1, 1 ] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'BufferTyped.length = 2, cinterval[ 0 ] - cinterval[ 1 ] === 1';
  var got = _.cinterval.isEmpty( new F32x( [ 0, -1 ] ) );
  var expected = true;
  test.identical( got, expected );

  test.case = 'BufferTyped.length > 2';
  var got = _.cinterval.isEmpty( new F32x( 4 ) );
  var expected = false;
  test.identical( got, expected );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.cinterval.isEmpty() );

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.cinterval.isEmpty( [ 1, 2 ], 'extra' ) );
}

//

function cintervalIsPopulated( test )
{
  test.case = 'undefined';
  var got = _.cinterval.isPopulated( undefined );
  var expected = false;
  test.identical( got, expected );

  test.case = 'null';
  var got = _.cinterval.isPopulated( null );
  var expected = false;
  test.identical( got, expected );

  test.case = 'false';
  var got = _.cinterval.isPopulated( false );
  var expected = false;
  test.identical( got, expected );

  test.case = 'empty string';
  var got = _.cinterval.isPopulated( '' );
  var expected = false;
  test.identical( got, expected );

  test.case = 'zero';
  var got = _.cinterval.isPopulated( 0 );
  var expected = false;
  test.identical( got, expected );

  test.case = 'NaN';
  var got = _.cinterval.isPopulated( NaN );
  var expected = false;
  test.identical( got, expected );

  test.case = 'a boolean';
  var got = _.cinterval.isPopulated( true );
  var expected = false;
  test.identical( got, expected );

  test.case = 'a number';
  var got = _.cinterval.isPopulated( 13 );
  var expected = false;
  test.identical( got, expected );

  test.case = 'a function';
  var got = _.cinterval.isPopulated( function() {} );
  var expected = false;
  test.identical( got, expected );

  test.case = 'constructor';
  function Constr( x )
  {
    this.x = x;
    return this;
  }
  var got = _.cinterval.isPopulated( new Constr( 0 ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'a string';
  var got = _.cinterval.isPopulated( 'str' );
  var expected = false;
  test.identical( got, expected );

  test.case = 'BufferRaw';
  var got = _.cinterval.isPopulated( new BufferRaw( 5 ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'BufferView';
  var got = _.cinterval.isPopulated( new BufferView( new BufferRaw( 5 ) ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'Set';
  var got = _.cinterval.isPopulated( new Set( [ 5 ] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'Map';
  var got = _.cinterval.isPopulated( new HashMap( [ [ 1, 2 ] ] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'pure empty map';
  var got = _.cinterval.isPopulated( Object.create( null ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'pure map';
  var src = Object.create( null );
  src.x = 1;
  var got = _.cinterval.isPopulated( src );
  var expected = false;
  test.identical( got, expected );

  test.case = 'map from pure map';
  var src = Object.create( Object.create( null ) );
  var got = _.cinterval.isPopulated( src );
  var expected = false;
  test.identical( got, expected );

  test.case = 'an empty object';
  var got = _.cinterval.isPopulated( {} );
  var expected = false;
  test.identical( got, expected );

  test.case = 'an object';
  var got = _.cinterval.isPopulated( { a : 7, b : 13 } );
  var expected = false;
  test.identical( got, expected );

  /* */

  test.case = 'array.length = 0';
  var got = _.cinterval.isPopulated( [] );
  var expected = false;
  test.identical( got, expected );

  test.case = 'array.length = 1';
  var got = _.cinterval.isPopulated( [ 1 ] );
  var expected = false;
  test.identical( got, expected );

  test.case = 'array.length = 2, different numbers';
  var got = _.cinterval.isPopulated( [ 1, 2 ] );
  var expected = true;
  test.identical( got, expected );

  test.case = 'array.length = 2, equal numbers';
  var got = _.cinterval.isPopulated( [ 1, 1 ] );
  var expected = true;
  test.identical( got, expected );

  test.case = 'array.length = 2, cinterval[ 0 ] - cinterval[ 1 ] === 1';
  var got = _.cinterval.isPopulated( [ 1, 0 ] );
  var expected = false;
  test.identical( got, expected );

  test.case = 'array.length = 2, number and undefined';
  var got = _.cinterval.isPopulated( [ 1, undefined ] );
  var expected = false;
  test.identical( got, expected );

  test.case = 'array.length > 2';
  var got = _.cinterval.isPopulated( [ 1, 2, 3 ] );
  var expected = false;
  test.identical( got, expected );

  test.case = 'unroll.length = 0';
  var got = _.cinterval.isPopulated( _.unroll.make( [] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'unroll.length = 1';
  var got = _.cinterval.isPopulated( _.unroll.make( [ 1 ] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'unroll.length = 2, different numbers';
  var got = _.cinterval.isPopulated( _.unroll.make( [ 1, 2 ] ) );
  var expected = true;
  test.identical( got, expected );

  test.case = 'unroll.length = 2, equal numbers';
  var got = _.cinterval.isPopulated( _.unroll.make( [ 10, 10 ] ) );
  var expected = true;
  test.identical( got, expected );

  test.case = 'unroll.length = 2, cinterval[ 0 ] - cinterval[ 1 ] === 1';
  var got = _.cinterval.isPopulated( _.unroll.make( [ 10, 9 ] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'unroll.length = 2, number and undefined';
  var got = _.cinterval.isPopulated( _.unroll.make( [ 1, undefined ] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'unroll.length > 2';
  var got = _.cinterval.isPopulated( _.unroll.make( [ 1, 2, 3 ] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'argumentsArray.length = 0';
  var got = _.cinterval.isPopulated( _.argumentsArray.make( [] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'argumentsArray.length = 1';
  var got = _.cinterval.isPopulated( _.argumentsArray.make( [ 1 ] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'argumentsArray.length = 2, different numbers';
  var got = _.cinterval.isPopulated( _.argumentsArray.make( [ 1, 2 ] ) );
  var expected = true;
  test.identical( got, expected );

  test.case = 'argumentsArray.length = 2, equal numbers';
  var got = _.cinterval.isPopulated( _.argumentsArray.make( [ -2, -2 ] ) );
  var expected = true;
  test.identical( got, expected );

  test.case = 'argumentsArray.length = 2, cinterval[ 0 ] - cinterval[ 1 ] === 1';
  var got = _.cinterval.isPopulated( _.argumentsArray.make( [ -1, -2 ] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'argumentsArray.length = 2, number and undefined';
  var got = _.cinterval.isPopulated( _.argumentsArray.make( [ 1, undefined ] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'argumentsArray.length > 2';
  var got = _.cinterval.isPopulated( _.argumentsArray.make( [ 1, 2, 3 ] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'BufferTyped.length = 0';
  var got = _.cinterval.isPopulated( new U8x() );
  var expected = false;
  test.identical( got, expected );

  test.case = 'BufferTyped.length = 1';
  var got = _.cinterval.isPopulated( new I16x( 1 ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'BufferTyped.length = 2, different numbers';
  var got = _.cinterval.isPopulated( new F32x( [ 1, 3 ] ) );
  var expected = true;
  test.identical( got, expected );

  test.case = 'BufferTyped.length = 2, equal numbers';
  var got = _.cinterval.isPopulated( new F32x( [ 1, 1 ] ) );
  var expected = true;
  test.identical( got, expected );

  test.case = 'BufferTyped.length = 2, cinterval[ 0 ] - cinterval[ 1 ] === 1';
  var got = _.cinterval.isPopulated( new F32x( [ 1, 0 ] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'BufferTyped.length > 2';
  var got = _.cinterval.isPopulated( new F32x( 4 ) );
  var expected = false;
  test.identical( got, expected );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.cinterval.isPopulated() );

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.cinterval.isPopulated( [ 1, 2 ], 'extra' ) );
}

// //
//
// function cintervalInInclusive( test )
// {
//   test.case = 'srcNumber - number, srcNumber < cinterval[ 0 ]';
//   var got = _.cinterval.inInclusive( [ 2, 5 ], 1 );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - number, srcNumber = cinterval[ 0 ]';
//   var got = _.cinterval.inInclusive( [ 2, 5 ], 2 );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - number, srcNumber > cinterval[ 1 ]';
//   var got = _.cinterval.inInclusive( [ 2, 5 ], 7 );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - number, srcNumber = cinterval[ 1 ]';
//   var got = _.cinterval.inInclusive( [ 2, 5 ], 5 );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - number, cinterval[ 0 ] < srcNumber < cinterval[ 1 ]';
//   var got = _.cinterval.inInclusive( [ 2, 5 ], 4 );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - array, srcNumber < cinterval[ 0 ]';
//   var got = _.cinterval.inInclusive( [ 2, 5 ], [ 0 ] );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - array, srcNumber = cinterval[ 0 ]';
//   var got = _.cinterval.inInclusive( [ 2, 5 ], [ 0, 0 ] );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - array, srcNumber > cinterval[ 1 ]';
//   var got = _.cinterval.inInclusive( [ 2, 5 ], [ 0, 0, 0, 0, 0, 0, 0 ] );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - array, srcNumber = cinterval[ 1 ]';
//   var got = _.cinterval.inInclusive( [ 2, 5 ], [ 0, 0, 0, 0, 0 ] );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - array, cinterval[ 0 ] < srcNumber < cinterval[ 1 ]';
//   var got = _.cinterval.inInclusive( [ 2, 5 ], [ 0, 0, 0, 0 ] );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - unroll, srcNumber < cinterval[ 0 ]';
//   var got = _.cinterval.inInclusive( [ 2, 5 ], _.unroll.make( [ 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - unroll, srcNumber = cinterval[ 0 ]';
//   var got = _.cinterval.inInclusive( [ 2, 5 ], _.unroll.make( [ 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - unroll, srcNumber > cinterval[ 1 ]';
//   var got = _.cinterval.inInclusive( [ 2, 5 ], _.unroll.make( [ 0, 0, 0, 0, 0, 0, 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - unroll, srcNumber = cinterval[ 1 ]';
//   var got = _.cinterval.inInclusive( [ 2, 5 ], _.unroll.make( [ 0, 0, 0, 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - unroll, cinterval[ 0 ] < srcNumber < cinterval[ 1 ]';
//   var got = _.cinterval.inInclusive( [ 2, 5 ], _.unroll.make( [ 0, 0, 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - argumentsArray, srcNumber < cinterval[ 0 ]';
//   var got = _.cinterval.inInclusive( [ 2, 5 ], _.argumentsArray.make( [ 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - argumentsArray, srcNumber = cinterval[ 0 ]';
//   var got = _.cinterval.inInclusive( [ 2, 5 ], _.argumentsArray.make( [ 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - argumentsArray, srcNumber > cinterval[ 1 ]';
//   var got = _.cinterval.inInclusive( [ 2, 5 ], _.argumentsArray.make( [ 0, 0, 0, 0, 0, 0, 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - argumentsArray, srcNumber = cinterval[ 1 ]';
//   var got = _.cinterval.inInclusive( [ 2, 5 ], _.argumentsArray.make( [ 0, 0, 0, 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - argumentsArray, cinterval[ 0 ] < srcNumber < cinterval[ 1 ]';
//   var got = _.cinterval.inInclusive( [ 2, 5 ], _.argumentsArray.make( [ 0, 0, 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - BufferTyped, srcNumber < cinterval[ 0 ]';
//   var got = _.cinterval.inInclusive( [ 2, 5 ], new I8x( [ 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - BufferTyped, srcNumber = cinterval[ 0 ]';
//   var got = _.cinterval.inInclusive( [ 2, 5 ], new I8x( [ 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - BufferTyped, srcNumber > cinterval[ 1 ]';
//   var got = _.cinterval.inInclusive( [ 2, 5 ], new U16x( [ 0, 0, 0, 0, 0, 0, 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - BufferTyped, srcNumber = cinterval[ 1 ]';
//   var got = _.cinterval.inInclusive( [ 2, 5 ], new U16x( [ 0, 0, 0, 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - BufferTyped, cinterval[ 0 ] < srcNumber < cinterval[ 1 ]';
//   var got = _.cinterval.inInclusive( [ 2, 5 ], new F32x( [ 0, 0, 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   /* - */
//
//   if( !Config.debug )
//   return;
//
//   test.case = 'without arguments';
//   test.shouldThrowErrorSync( () => _.cinterval.inInclusive() );
//
//   test.case = 'not enough arguments';
//   test.shouldThrowErrorSync( () => _.cinterval.inInclusive( [ 1, 2 ] ) );
//
//   test.case = 'extra arguments';
//   test.shouldThrowErrorSync( () => _.cinterval.inInclusive( [ 1, 2 ], 3, 'extra' ) );
//
//   test.case = 'cinterval is not cinterval';
//   test.shouldThrowErrorSync( () => _.cinterval.inInclusive( 'wrong', 3 ) );
//
//   test.case = 'srcNumber is not Long, not Number';
//   test.shouldThrowErrorSync( () => _.cinterval.inInclusive( [ 1, 2 ], 'wrong' ) );
// }
//
// //
//
// function cintervalInExclusive( test )
// {
//   test.case = 'srcNumber - number, srcNumber < cinterval[ 0 ]';
//   var got = _.cinterval.inExclusive( [ 2, 5 ], 1 );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - number, srcNumber = cinterval[ 0 ]';
//   var got = _.cinterval.inExclusive( [ 2, 5 ], 2 );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - number, srcNumber > cinterval[ 1 ] + 1';
//   var got = _.cinterval.inExclusive( [ 2, 5 ], 7 );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - number, srcNumber === cinterval[ 1 ] + 1';
//   var got = _.cinterval.inExclusive( [ 2, 5 ], 6 );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - number, srcNumber = cinterval[ 1 ]';
//   var got = _.cinterval.inExclusive( [ 2, 5 ], 5 );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - number, cinterval[ 0 ] < srcNumber < cinterval[ 1 ]';
//   var got = _.cinterval.inExclusive( [ 2, 5 ], 4 );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - array, srcNumber < cinterval[ 0 ]';
//   var got = _.cinterval.inExclusive( [ 2, 5 ], [ 0 ] );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - array, srcNumber = cinterval[ 0 ]';
//   var got = _.cinterval.inExclusive( [ 2, 5 ], [ 0, 0 ] );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - array, srcNumber > cinterval[ 1 ] + 1';
//   var got = _.cinterval.inExclusive( [ 2, 5 ], [ 0, 0, 0, 0, 0, 0, 0 ] );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - array, srcNumber === cinterval[ 1 ] + 1';
//   var got = _.cinterval.inExclusive( [ 2, 5 ], [ 0, 0, 0, 0, 0, 0 ] );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - array, srcNumber = cinterval[ 1 ]';
//   var got = _.cinterval.inExclusive( [ 2, 5 ], [ 0, 0, 0, 0, 0 ] );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - array, cinterval[ 0 ] < srcNumber < cinterval[ 1 ]';
//   var got = _.cinterval.inExclusive( [ 2, 5 ], [ 0, 0, 0, 0 ] );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - unroll, srcNumber < cinterval[ 0 ]';
//   var got = _.cinterval.inExclusive( [ 2, 5 ], _.unroll.make( [ 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - unroll, srcNumber = cinterval[ 0 ]';
//   var got = _.cinterval.inExclusive( [ 2, 5 ], _.unroll.make( [ 0, 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - unroll, srcNumber > cinterval[ 1 ], srcNumber > cinterval[ 1 ] + 1';
//   var got = _.cinterval.inExclusive( [ 2, 5 ], _.unroll.make( [ 0, 0, 0, 0, 0, 0, 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - unroll, srcNumber > cinterval[ 1 ], srcNumber === cinterval[ 1 ] + 1';
//   var got = _.cinterval.inExclusive( [ 2, 5 ], _.unroll.make( [ 0, 0, 0, 0, 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - unroll, srcNumber = cinterval[ 1 ]';
//   var got = _.cinterval.inExclusive( [ 2, 5 ], _.unroll.make( [ 0, 0, 0, 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - unroll, cinterval[ 0 ] < srcNumber < cinterval[ 1 ]';
//   var got = _.cinterval.inExclusive( [ 2, 5 ], _.unroll.make( [ 0, 0, 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - argumentsArray, srcNumber < cinterval[ 0 ]';
//   var got = _.cinterval.inExclusive( [ 2, 5 ], _.argumentsArray.make( [ 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - argumentsArray, srcNumber = cinterval[ 0 ]';
//   var got = _.cinterval.inExclusive( [ 2, 5 ], _.argumentsArray.make( [ 0, 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - argumentsArray, srcNumber > cinterval[ 1 ] + 1';
//   var got = _.cinterval.inExclusive( [ 2, 5 ], _.argumentsArray.make( [ 0, 0, 0, 0, 0, 0, 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - argumentsArray, srcNumber === cinterval[ 1 ] + 1';
//   var got = _.cinterval.inExclusive( [ 2, 5 ], _.argumentsArray.make( [ 0, 0, 0, 0, 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - argumentsArray, srcNumber = cinterval[ 1 ]';
//   var got = _.cinterval.inExclusive( [ 2, 5 ], _.argumentsArray.make( [ 0, 0, 0, 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - argumentsArray, cinterval[ 0 ] < srcNumber < cinterval[ 1 ]';
//   var got = _.cinterval.inExclusive( [ 2, 5 ], _.argumentsArray.make( [ 0, 0, 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - BufferTyped, srcNumber < cinterval[ 0 ]';
//   var got = _.cinterval.inExclusive( [ 2, 5 ], new I8x( [ 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - BufferTyped, srcNumber = cinterval[ 0 ]';
//   var got = _.cinterval.inExclusive( [ 2, 5 ], new I8x( [ 0, 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - BufferTyped, srcNumber > cinterval[ 1 ] + 1';
//   var got = _.cinterval.inExclusive( [ 2, 5 ], new U16x( [ 0, 0, 0, 0, 0, 0, 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - BufferTyped, srcNumber === cinterval[ 1 ] + 1';
//   var got = _.cinterval.inExclusive( [ 2, 5 ], new U16x( [ 0, 0, 0, 0, 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - BufferTyped, srcNumber = cinterval[ 1 ]';
//   var got = _.cinterval.inExclusive( [ 2, 5 ], new U16x( [ 0, 0, 0, 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - BufferTyped, cinterval[ 0 ] < srcNumber < cinterval[ 1 ]';
//   var got = _.cinterval.inExclusive( [ 2, 5 ], new F32x( [ 0, 0, 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   /* - */
//
//   if( !Config.debug )
//   return;
//
//   test.case = 'without arguments';
//   test.shouldThrowErrorSync( () => _.cinterval.inExclusive() );
//
//   test.case = 'not enough arguments';
//   test.shouldThrowErrorSync( () => _.cinterval.inExclusive( [ 1, 2 ] ) );
//
//   test.case = 'extra arguments';
//   test.shouldThrowErrorSync( () => _.cinterval.inExclusive( [ 1, 2 ], 3, 'extra' ) );
//
//   test.case = 'cinterval is not cinterval';
//   test.shouldThrowErrorSync( () => _.cinterval.inExclusive( 'wrong', 3 ) );
//
//   test.case = 'srcNumber is not Long, not Number';
//   test.shouldThrowErrorSync( () => _.cinterval.inExclusive( [ 1, 2 ], 'wrong' ) );
// }
//
// //
//
// function cintervalInInclusiveLeft( test )
// {
//   test.case = 'srcNumber - number, srcNumber < cinterval[ 0 ]';
//   var got = _.cinterval.inInclusiveLeft( [ 2, 5 ], 1 );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - number, srcNumber = cinterval[ 0 ]';
//   var got = _.cinterval.inInclusiveLeft( [ 2, 5 ], 2 );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - number, srcNumber > cinterval[ 1 ]';
//   var got = _.cinterval.inInclusiveLeft( [ 2, 5 ], 7 );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - number, srcNumber = cinterval[ 1 ]';
//   var got = _.cinterval.inInclusiveLeft( [ 2, 5 ], 5 );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - number, cinterval[ 0 ] < srcNumber < cinterval[ 1 ]';
//   var got = _.cinterval.inInclusiveLeft( [ 2, 5 ], 4 );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - array, srcNumber < cinterval[ 0 ]';
//   var got = _.cinterval.inInclusiveLeft( [ 2, 5 ], [ 0 ] );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - array, srcNumber = cinterval[ 0 ]';
//   var got = _.cinterval.inInclusiveLeft( [ 2, 5 ], [ 0, 0 ] );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - array, srcNumber > cinterval[ 1 ]';
//   var got = _.cinterval.inInclusiveLeft( [ 2, 5 ], [ 0, 0, 0, 0, 0, 0, 0 ] );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - array, srcNumber = cinterval[ 1 ]';
//   var got = _.cinterval.inInclusiveLeft( [ 2, 5 ], [ 0, 0, 0, 0, 0 ] );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - array, cinterval[ 0 ] < srcNumber < cinterval[ 1 ]';
//   var got = _.cinterval.inInclusiveLeft( [ 2, 5 ], [ 0, 0, 0, 0 ] );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - unroll, srcNumber < cinterval[ 0 ]';
//   var got = _.cinterval.inInclusiveLeft( [ 2, 5 ], _.unroll.make( [ 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - unroll, srcNumber = cinterval[ 0 ]';
//   var got = _.cinterval.inInclusiveLeft( [ 2, 5 ], _.unroll.make( [ 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - unroll, srcNumber > cinterval[ 1 ]';
//   var got = _.cinterval.inInclusiveLeft( [ 2, 5 ], _.unroll.make( [ 0, 0, 0, 0, 0, 0, 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - unroll, srcNumber = cinterval[ 1 ]';
//   var got = _.cinterval.inInclusiveLeft( [ 2, 5 ], _.unroll.make( [ 0, 0, 0, 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - unroll, cinterval[ 0 ] < srcNumber < cinterval[ 1 ]';
//   var got = _.cinterval.inInclusiveLeft( [ 2, 5 ], _.unroll.make( [ 0, 0, 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - argumentsArray, srcNumber < cinterval[ 0 ]';
//   var got = _.cinterval.inInclusiveLeft( [ 2, 5 ], _.argumentsArray.make( [ 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - argumentsArray, srcNumber = cinterval[ 0 ]';
//   var got = _.cinterval.inInclusiveLeft( [ 2, 5 ], _.argumentsArray.make( [ 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - argumentsArray, srcNumber > cinterval[ 1 ]';
//   var got = _.cinterval.inInclusiveLeft( [ 2, 5 ], _.argumentsArray.make( [ 0, 0, 0, 0, 0, 0, 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - argumentsArray, srcNumber = cinterval[ 1 ]';
//   var got = _.cinterval.inInclusiveLeft( [ 2, 5 ], _.argumentsArray.make( [ 0, 0, 0, 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - argumentsArray, cinterval[ 0 ] < srcNumber < cinterval[ 1 ]';
//   var got = _.cinterval.inInclusiveLeft( [ 2, 5 ], _.argumentsArray.make( [ 0, 0, 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - BufferTyped, srcNumber < cinterval[ 0 ]';
//   var got = _.cinterval.inInclusiveLeft( [ 2, 5 ], new I8x( [ 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - BufferTyped, srcNumber = cinterval[ 0 ]';
//   var got = _.cinterval.inInclusiveLeft( [ 2, 5 ], new I8x( [ 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - BufferTyped, srcNumber > cinterval[ 1 ]';
//   var got = _.cinterval.inInclusiveLeft( [ 2, 5 ], new U16x( [ 0, 0, 0, 0, 0, 0, 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - BufferTyped, srcNumber = cinterval[ 1 ]';
//   var got = _.cinterval.inInclusiveLeft( [ 2, 5 ], new U16x( [ 0, 0, 0, 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - BufferTyped, cinterval[ 0 ] < srcNumber < cinterval[ 1 ]';
//   var got = _.cinterval.inInclusiveLeft( [ 2, 5 ], new F32x( [ 0, 0, 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   /* - */
//
//   if( !Config.debug )
//   return;
//
//   test.case = 'without arguments';
//   test.shouldThrowErrorSync( () => _.cinterval.inInclusiveLeft() );
//
//   test.case = 'not enough arguments';
//   test.shouldThrowErrorSync( () => _.cinterval.inInclusiveLeft( [ 1, 2 ] ) );
//
//   test.case = 'extra arguments';
//   test.shouldThrowErrorSync( () => _.cinterval.inInclusiveLeft( [ 1, 2 ], 3, 'extra' ) );
//
//   test.case = 'cinterval is not cinterval';
//   test.shouldThrowErrorSync( () => _.cinterval.inInclusiveLeft( 'wrong', 3 ) );
//
//   test.case = 'srcNumber is not Long, not Number';
//   test.shouldThrowErrorSync( () => _.cinterval.inInclusiveLeft( [ 1, 2 ], 'wrong' ) );
// }
//
// //
//
// function cintervalInInclusiveRight( test )
// {
//   test.case = 'srcNumber - number, srcNumber < cinterval[ 0 ]';
//   var got = _.cinterval.inInclusiveRight( [ 2, 5 ], 1 );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - number, srcNumber = cinterval[ 0 ]';
//   var got = _.cinterval.inInclusiveRight( [ 2, 5 ], 2 );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - number, srcNumber > cinterval[ 1 ]';
//   var got = _.cinterval.inInclusiveRight( [ 2, 5 ], 7 );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - number, srcNumber = cinterval[ 1 ]';
//   var got = _.cinterval.inInclusiveRight( [ 2, 5 ], 5 );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - number, cinterval[ 0 ] < srcNumber < cinterval[ 1 ]';
//   var got = _.cinterval.inInclusiveRight( [ 2, 5 ], 4 );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - array, srcNumber < cinterval[ 0 ]';
//   var got = _.cinterval.inInclusiveRight( [ 2, 5 ], [ 0 ] );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - array, srcNumber = cinterval[ 0 ]';
//   var got = _.cinterval.inInclusiveRight( [ 2, 5 ], [ 0, 0 ] );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - array, srcNumber > cinterval[ 1 ]';
//   var got = _.cinterval.inInclusiveRight( [ 2, 5 ], [ 0, 0, 0, 0, 0, 0, 0 ] );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - array, srcNumber = cinterval[ 1 ]';
//   var got = _.cinterval.inInclusiveRight( [ 2, 5 ], [ 0, 0, 0, 0, 0 ] );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - array, cinterval[ 0 ] < srcNumber < cinterval[ 1 ]';
//   var got = _.cinterval.inInclusiveRight( [ 2, 5 ], [ 0, 0, 0, 0 ] );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - unroll, srcNumber < cinterval[ 0 ]';
//   var got = _.cinterval.inInclusiveRight( [ 2, 5 ], _.unroll.make( [ 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - unroll, srcNumber = cinterval[ 0 ]';
//   var got = _.cinterval.inInclusiveRight( [ 2, 5 ], _.unroll.make( [ 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - unroll, srcNumber > cinterval[ 1 ]';
//   var got = _.cinterval.inInclusiveRight( [ 2, 5 ], _.unroll.make( [ 0, 0, 0, 0, 0, 0, 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - unroll, srcNumber = cinterval[ 1 ]';
//   var got = _.cinterval.inInclusiveRight( [ 2, 5 ], _.unroll.make( [ 0, 0, 0, 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - unroll, cinterval[ 0 ] < srcNumber < cinterval[ 1 ]';
//   var got = _.cinterval.inInclusiveRight( [ 2, 5 ], _.unroll.make( [ 0, 0, 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - argumentsArray, srcNumber < cinterval[ 0 ]';
//   var got = _.cinterval.inInclusiveRight( [ 2, 5 ], _.argumentsArray.make( [ 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - argumentsArray, srcNumber = cinterval[ 0 ]';
//   var got = _.cinterval.inInclusiveRight( [ 2, 5 ], _.argumentsArray.make( [ 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - argumentsArray, srcNumber > cinterval[ 1 ]';
//   var got = _.cinterval.inInclusiveRight( [ 2, 5 ], _.argumentsArray.make( [ 0, 0, 0, 0, 0, 0, 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - argumentsArray, srcNumber = cinterval[ 1 ]';
//   var got = _.cinterval.inInclusiveRight( [ 2, 5 ], _.argumentsArray.make( [ 0, 0, 0, 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - argumentsArray, cinterval[ 0 ] < srcNumber < cinterval[ 1 ]';
//   var got = _.cinterval.inInclusiveRight( [ 2, 5 ], _.argumentsArray.make( [ 0, 0, 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - BufferTyped, srcNumber < cinterval[ 0 ]';
//   var got = _.cinterval.inInclusiveRight( [ 2, 5 ], new I8x( [ 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - BufferTyped, srcNumber = cinterval[ 0 ]';
//   var got = _.cinterval.inInclusiveRight( [ 2, 5 ], new I8x( [ 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - BufferTyped, srcNumber > cinterval[ 1 ]';
//   var got = _.cinterval.inInclusiveRight( [ 2, 5 ], new U16x( [ 0, 0, 0, 0, 0, 0, 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - BufferTyped, srcNumber = cinterval[ 1 ]';
//   var got = _.cinterval.inInclusiveRight( [ 2, 5 ], new U16x( [ 0, 0, 0, 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - BufferTyped, cinterval[ 0 ] < srcNumber < cinterval[ 1 ]';
//   var got = _.cinterval.inInclusiveRight( [ 2, 5 ], new F32x( [ 0, 0, 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   /* - */
//
//   if( !Config.debug )
//   return;
//
//   test.case = 'without arguments';
//   test.shouldThrowErrorSync( () => _.cinterval.inInclusiveRight() );
//
//   test.case = 'not enough arguments';
//   test.shouldThrowErrorSync( () => _.cinterval.inInclusiveRight( [ 1, 2 ] ) );
//
//   test.case = 'extra arguments';
//   test.shouldThrowErrorSync( () => _.cinterval.inInclusiveRight( [ 1, 2 ], 3, 'extra' ) );
//
//   test.case = 'cinterval is not cinterval';
//   test.shouldThrowErrorSync( () => _.cinterval.inInclusiveRight( 'wrong', 3 ) );
//
//   test.case = 'srcNumber is not Long, not Number';
//   test.shouldThrowErrorSync( () => _.cinterval.inInclusiveRight( [ 1, 2 ], 'wrong' ) );
// }

//

function cintervalHas( test )
{
  test.open( 'src - number' );

  test.case = 'src < cinterval[ 0 ]';
  var got = _.cinterval.has( [ 2, 5 ], 1 );
  var expected = false;
  test.identical( got, expected );

  test.case = 'src === cinterval[ 0 ]';
  var got = _.cinterval.has( [ 2, 5 ], 2 );
  var expected = true;
  test.identical( got, expected );

  test.case = 'src > cinterval[ 1 ]';
  var got = _.cinterval.has( [ 2, 5 ], 7 );
  var expected = false;
  test.identical( got, expected );

  test.case = 'src === cinterval[ 1 ]';
  var got = _.cinterval.has( [ 2, 5 ], 5 );
  var expected = true;
  test.identical( got, expected );

  test.case = 'cinterval[ 0 ] < src < cinterval[ 1 ]';
  var got = _.cinterval.has( [ 2, 5 ], 4 );
  var expected = true;
  test.identical( got, expected );

  test.close( 'src - number' );

  /* - */

  test.open( 'src - interval' );

  test.case = 'src[ 0 ] < cinterval[ 0 ]';
  var got = _.cinterval.has( [ 2, 5 ], [ 1, 4 ] );
  var expected = false;
  test.identical( got, expected );

  test.case = 'src[ 0 ] === cinterval[ 0 ]';
  var got = _.cinterval.has( [ 2, 5 ], [ 2, 4 ] );
  var expected = true;
  test.identical( got, expected );

  test.case = 'src[ 1 ] > cinterval[ 1 ]';
  var got = _.cinterval.has( [ 2, 5 ], [ 2, 6 ] );
  var expected = false;
  test.identical( got, expected );

  test.case = 'src[ 1 ] === cinterval[ 1 ]';
  var got = _.cinterval.has( [ 2, 5 ], [ 2, 5 ] );
  var expected = true;
  test.identical( got, expected );

  test.case = 'cinterval[ 0 ] < src[ 0 ] && src[ 1 ] < cinterval[ 1 ]';
  var got = _.cinterval.has( [ 2, 5 ], [ 3, 4 ] );
  var expected = true;
  test.identical( got, expected );

  test.close( 'src - interval' );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.cinterval.has() );

  test.case = 'not enough arguments';
  test.shouldThrowErrorSync( () => _.cinterval.has( [ 1, 2 ] ) );

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.cinterval.has( [ 1, 2 ], 3, 'extra' ) );

  test.case = 'cinterval is not cinterval';
  test.shouldThrowErrorSync( () => _.cinterval.has( 'wrong', 3 ) );

  test.case = 'src is not Interval, not Number';
  test.shouldThrowErrorSync( () => _.cinterval.has( [ 1, 2 ], 'wrong' ) );
}

//

function cintervalSureInRange( test )
{
  test.case = 'two arguments, src - number, in cinterval';
  var got = _.cinterval.sureIn( 3, [ 1, 5 ] );
  var expected = true;
  test.identical( got, expected );

  test.case = 'two arguments, src - array, in cinterval';
  var got = _.cinterval.sureIn( [ 0, 0, 0 ], [ 1, 5 ] );
  var expected = true;
  test.identical( got, expected );

  test.case = 'more then two arguments, src - number, in cinterval';
  var got = _.cinterval.sureIn( 3, [ 1, 5 ], 'extra', [ 'next' ] );
  var expected = true;
  test.identical( got, expected );

  test.case = 'more then two arguments, src - array, in cinterval';
  var got = _.cinterval.sureIn( [ 0, 0, 0 ], [ 1, 5 ], 'extra', [ 'next' ] );
  var expected = true;
  test.identical( got, expected );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.cinterval.sureIn() );

  test.case = 'not enough arguments';
  test.shouldThrowErrorSync( () => _.cinterval.sureIn( 2 ) );

  test.case = 'src out of cinterval';
  test.shouldThrowErrorSync( () => _.cinterval.sureIn( 1, [ 2, 5 ] ) );
  test.shouldThrowErrorSync( () => _.cinterval.sureIn( 6, [ 2, 5 ] ) );
  test.shouldThrowErrorSync( () => _.cinterval.sureIn( 1, [ 2, 5 ], [] ) );
  test.shouldThrowErrorSync( () => _.cinterval.sureIn( 6, [ 2, 5 ], {} ) );
}

//

function cintervalAssertInRange( test )
{
  test.case = 'two arguments, src - number, in cinterval';
  var got = _.cinterval.assertIn( 3, [ 1, 5 ] );
  var expected = true;
  test.identical( got, expected );

  test.case = 'two arguments, src - array, in cinterval';
  var got = _.cinterval.assertIn( [ 0, 0, 0 ], [ 1, 5 ] );
  var expected = true;
  test.identical( got, expected );

  test.case = 'more then two arguments, src - number, in cinterval';
  var got = _.cinterval.assertIn( 3, [ 1, 5 ], 'extra', [ 'next' ] );
  var expected = true;
  test.identical( got, expected );

  test.case = 'more then two arguments, src - array, in cinterval';
  var got = _.cinterval.assertIn( [ 0, 0, 0 ], [ 1, 5 ], 'extra', [ 'next' ] );
  var expected = true;
  test.identical( got, expected );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.cinterval.assertIn() );

  test.case = 'not enough arguments';
  test.shouldThrowErrorSync( () => _.cinterval.assertIn( 2 ) );

  test.case = 'src out of cinterval';
  test.shouldThrowErrorSync( () => _.cinterval.assertIn( 1, [ 2, 5 ] ) );
  test.shouldThrowErrorSync( () => _.cinterval.assertIn( 6, [ 2, 5 ] ) );
  test.shouldThrowErrorSync( () => _.cinterval.assertIn( 1, [ 2, 5 ], [] ) );
  test.shouldThrowErrorSync( () => _.cinterval.assertIn( 6, [ 2, 5 ], {} ) );
}

//

function cintervalFromSingle( test )
{
  test.case = 'cinterval - number';
  var got = _.cinterval.fromSingle( 1 );
  test.identical( got, [ 1, 1 ] );

  /* */

  test.case = 'cinterval.length === 1';
  var src = [ 1 ];
  var got = _.cinterval.fromSingle( src );
  test.identical( got, [ 1, 1 ] );
  test.true( got !== src );

  test.case = 'cinterval[ 0 ] - undefined';
  var src = [ undefined, 2 ];
  var got = _.cinterval.fromSingle( src );
  test.identical( got, [ 2, 2 ] );
  test.true( got !== src );

  test.case = 'cinterval[ 1 ] - undefined';
  var src = [ 1, undefined ];
  var got = _.cinterval.fromSingle( src );
  test.identical( got, [ 1, 1 ] );
  test.true( got !== src );

  test.case = 'cinterval[ 0 ] - undefined, cinterval[ 1 ] - undefined';
  var src = [ undefined, undefined ];
  var got = _.cinterval.fromSingle( src );
  test.identical( got, [ 0, 0 ] );
  test.true( got !== src );

  test.case = 'cinterval[ 0 ] - number, cinterval[ 1 ] - number';
  var src = [ 1, 5 ];
  var got = _.cinterval.fromSingle( src );
  test.identical( got, [ 1, 5 ] );
  test.true( got === src );

  /* */

  test.case = 'cinterval.length === 1';
  var src = _.unroll.make( [ 1 ] );
  var got = _.cinterval.fromSingle( src );
  test.identical( got, [ 1, 1 ] );
  test.true( got !== src );

  test.case = 'cinterval[ 0 ] - undefined';
  var src = _.unroll.make( [ undefined, 2 ] );
  var got = _.cinterval.fromSingle( src );
  test.identical( got, [ 2, 2 ] );
  test.true( got !== src );

  test.case = 'cinterval[ 1 ] - undefined';
  var src = _.unroll.make( [ 1, undefined ] );
  var got = _.cinterval.fromSingle( src );
  test.identical( got, [ 1, 1 ] );
  test.true( got !== src );

  test.case = 'cinterval[ 0 ] - undefined, cinterval[ 1 ] - undefined';
  var src = _.unroll.make( [ undefined, undefined ] );
  var got = _.cinterval.fromSingle( src );
  test.identical( got, [ 0, 0 ] );
  test.true( got !== src );

  test.case = 'cinterval[ 0 ] - number, cinterval[ 1 ] - number';
  var src = _.unroll.make( [ 1, 5 ] );
  var got = _.cinterval.fromSingle( src );
  test.identical( got, _.unroll.make( [ 1, 5 ] ) );
  test.true( got === src );

  /* */

  test.case = 'cinterval.length === 1';
  var src = _.argumentsArray.make( [ 1 ] );
  var got = _.cinterval.fromSingle( src );
  test.identical( got, [ 1, 1 ] );
  test.true( got !== src );

  test.case = 'cinterval[ 0 ] - undefined';
  var src = _.argumentsArray.make( [ undefined, 2 ] );
  var got = _.cinterval.fromSingle( src );
  test.identical( got, [ 2, 2 ] );
  test.true( got !== src );

  test.case = 'cinterval[ 1 ] - undefined';
  var src = _.argumentsArray.make( [ 1, undefined ] );
  var got = _.cinterval.fromSingle( src );
  test.identical( got, [ 1, 1 ] );
  test.true( got !== src );

  test.case = 'cinterval[ 0 ] - undefined, cinterval[ 1 ] - undefined';
  var src = _.argumentsArray.make( [ undefined, undefined ] );
  var got = _.cinterval.fromSingle( src );
  test.identical( got, [ 0, 0 ] );
  test.true( got !== src );

  test.case = 'cinterval[ 0 ] - number, cinterval[ 1 ] - number';
  var src = _.argumentsArray.make( [ 1, 5 ] );
  var got = _.cinterval.fromSingle( src );
  test.identical( got, _.argumentsArray.make( [ 1, 5 ] ) );
  test.true( got === src );

  /* */

  test.case = 'cinterval.length === 1';
  var src = new U8x( [ 1 ] );
  var got = _.cinterval.fromSingle( src );
  test.identical( got, [ 1, 1 ] );
  test.true( got !== src );

  test.case = 'cinterval[ 0 ] - number, cinterval[ 1 ] - number';
  var src = new I16x( [ 1, 5 ] );
  var got = _.cinterval.fromSingle( src );
  test.identical( got, new I16x( [ 1, 5 ] ) );
  test.true( got === src );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.cinterval.fromSingle() );

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.cinterval.fromSingle( [ 1, 2 ], 1 ) );

  test.case = 'wrong type of cinterval';
  test.shouldThrowErrorSync( () => _.cinterval.fromSingle( { 0 : 1, 1 : 2 } ) );

  test.case = 'wrong cinterval length';
  test.shouldThrowErrorSync( () => _.cinterval.fromSingle( [ 0, 1, 2 ] ) );

  test.case = 'wrong elements in cinterval';
  test.shouldThrowErrorSync( () => _.cinterval.fromSingle( [ null, 2 ] ) );
  test.shouldThrowErrorSync( () => _.cinterval.fromSingle( [ 2, 'abc' ] ) );
}

//

function cintervalClamp( test )
{
  test.open( 'array' );

  test.case = 'identical ranges';
  var dst = [ 2, 2 ];
  var clamp = [ 2, 2 ];
  var got = _.cinterval.clamp( dst, clamp );
  test.identical( got, [ 2, 2 ] );
  test.true( got === dst );

  test.case = 'dst[ 0 ] < clamp[ 0 ], dst[ 1 ] < clamp[ 0 ]';
  var dst = [ 2, 2 ];
  var clamp = [ 3, 6 ];
  var got = _.cinterval.clamp( dst, clamp );
  test.identical( got, [ 3, 3 ] );
  test.true( got === dst );

  test.case = 'dst[ 0 ] < clamp[ 0 ], clamp[ 1 ] > dst[ 1 ] > clamp[ 0 ]';
  var dst = [ 2, 4 ];
  var clamp = [ 3, 6 ];
  var got = _.cinterval.clamp( dst, clamp );
  test.identical( got, [ 3, 4 ] );
  test.true( got === dst );

  test.case = 'clamp[ 1 ] > dst[ 0 ] > clamp[ 0 ], dst[ 1 ] < clamp[ 0 ]';
  var dst = [ 4, 2 ];
  var clamp = [ 3, 6 ];
  var got = _.cinterval.clamp( dst, clamp );
  test.identical( got, [ 4, 3 ] );
  test.true( got === dst );

  test.case = 'clamp[ 1 ] > dst[ 0 ] > clamp[ 0 ], clamp[ 1 ] > dst[ 1 ] > clamp[ 0 ]';
  var dst = [ 4, 5 ];
  var clamp = [ 3, 6 ];
  var got = _.cinterval.clamp( dst, clamp );
  test.identical( got, [ 4, 5 ] );
  test.true( got === dst );

  test.case = 'dst[ 0 ] > clamp[ 1 ], clamp[ 1 ] > dst[ 1 ] > clamp[ 0 ]';
  var dst = [ 7, 5 ];
  var clamp = [ 3, 6 ];
  var got = _.cinterval.clamp( dst, clamp );
  test.identical( got, [ 6, 5 ] );
  test.true( got === dst );

  test.case = 'clamp[ 1 ] > dst[ 0 ] > clamp[ 0 ], dst[ 1 ] > clamp[ 1 ]';
  var dst = [ 4, 7 ];
  var clamp = [ 3, 6 ];
  var got = _.cinterval.clamp( dst, clamp );
  test.identical( got, [ 4, 6 ] );
  test.true( got === dst );

  test.case = 'dst[ 0 ] > clamp[ 1 ], dst[ 1 ] > clamp[ 1 ]';
  var dst = [ 8, 7 ];
  var clamp = [ 3, 6 ];
  var got = _.cinterval.clamp( dst, clamp );
  test.identical( got, [ 6, 6 ] );
  test.true( got === dst );

  test.case = 'dst[ 0 ] < clamp[ 0 ], dst[ 1 ] > clamp[ 1 ]';
  var dst = [ 2, 7 ];
  var clamp = [ 3, 6 ];
  var got = _.cinterval.clamp( dst, clamp );
  test.identical( got, [ 3, 6 ] );
  test.true( got === dst );

  test.close( 'array' );

  /* - */

  test.open( 'unroll' );

  test.case = 'identical ranges';
  var dst = _.unroll.make( [ 2, 2 ] );
  var clamp = [ 2, 2 ];
  var got = _.cinterval.clamp( dst, clamp );
  test.identical( got, _.unroll.make( [ 2, 2 ] ) );
  test.true( got === dst );

  test.case = 'dst[ 0 ] < clamp[ 0 ], dst[ 1 ] < clamp[ 0 ]';
  var dst = _.unroll.make( [ 2, 2 ] );
  var clamp = [ 3, 6 ];
  var got = _.cinterval.clamp( dst, clamp );
  test.identical( got, _.unroll.make( [ 3, 3 ] ) );
  test.true( got === dst );

  test.case = 'dst[ 0 ] < clamp[ 0 ], clamp[ 1 ] > dst[ 1 ] > clamp[ 0 ]';
  var dst = _.unroll.make( [ 2, 4 ] );
  var clamp = [ 3, 6 ];
  var got = _.cinterval.clamp( dst, clamp );
  test.identical( got, _.unroll.make( [ 3, 4 ] ) );
  test.true( got === dst );

  test.case = 'clamp[ 1 ] > dst[ 0 ] > clamp[ 0 ], dst[ 1 ] < clamp[ 0 ]';
  var dst = _.unroll.make( [ 4, 2 ] );
  var clamp = [ 3, 6 ];
  var got = _.cinterval.clamp( dst, clamp );
  test.identical( got, _.unroll.make( [ 4, 3 ] ) );
  test.true( got === dst );

  test.case = 'clamp[ 1 ] > dst[ 0 ] > clamp[ 0 ], clamp[ 1 ] > dst[ 1 ] > clamp[ 0 ]';
  var dst = _.unroll.make( [ 4, 5 ] );
  var clamp = [ 3, 6 ];
  var got = _.cinterval.clamp( dst, clamp );
  test.identical( got, _.unroll.make( [ 4, 5 ] ) );
  test.true( got === dst );

  test.case = 'dst[ 0 ] > clamp[ 1 ], clamp[ 1 ] > dst[ 1 ] > clamp[ 0 ]';
  var dst = _.unroll.make( [ 7, 5 ] );
  var clamp = [ 3, 6 ];
  var got = _.cinterval.clamp( dst, clamp );
  test.identical( got, _.unroll.make( [ 6, 5 ] ) );
  test.true( got === dst );

  test.case = 'clamp[ 1 ] > dst[ 0 ] > clamp[ 0 ], dst[ 1 ] > clamp[ 1 ]';
  var dst = _.unroll.make( [ 4, 7 ] );
  var clamp = [ 3, 6 ];
  var got = _.cinterval.clamp( dst, clamp );
  test.identical( got, _.unroll.make( [ 4, 6 ] ) );
  test.true( got === dst );

  test.case = 'dst[ 0 ] > clamp[ 1 ], dst[ 1 ] > clamp[ 1 ]';
  var dst = _.unroll.make( [ 8, 7 ] );
  var clamp = [ 3, 6 ];
  var got = _.cinterval.clamp( dst, clamp );
  test.identical( got, _.unroll.make( [ 6, 6 ] ) );
  test.true( got === dst );

  test.case = 'dst[ 0 ] < clamp[ 0 ], dst[ 1 ] > clamp[ 1 ]';
  var dst = _.unroll.make( [ 2, 7 ] );
  var clamp = [ 3, 6 ];
  var got = _.cinterval.clamp( dst, clamp );
  test.identical( got, _.unroll.make( [ 3, 6 ] ) );
  test.true( got === dst );

  test.close( 'unroll' );

  /* - */

  test.open( 'argumentsArray' );

  test.case = 'identical ranges';
  var dst = _.argumentsArray.make( [ 2, 2 ] );
  var clamp = [ 2, 2 ];
  var got = _.cinterval.clamp( dst, clamp );
  test.identical( got, _.argumentsArray.make( [ 2, 2 ] ) );
  test.true( got === dst );

  test.case = 'dst[ 0 ] < clamp[ 0 ], dst[ 1 ] < clamp[ 0 ]';
  var dst = _.argumentsArray.make( [ 2, 2 ] );
  var clamp = [ 3, 6 ];
  var got = _.cinterval.clamp( dst, clamp );
  test.identical( got, _.argumentsArray.make( [ 3, 3 ] ) );
  test.true( got === dst );

  test.case = 'dst[ 0 ] < clamp[ 0 ], clamp[ 1 ] > dst[ 1 ] > clamp[ 0 ]';
  var dst = _.argumentsArray.make( [ 2, 4 ] );
  var clamp = [ 3, 6 ];
  var got = _.cinterval.clamp( dst, clamp );
  test.identical( got, _.argumentsArray.make( [ 3, 4 ] ) );
  test.true( got === dst );

  test.case = 'clamp[ 1 ] > dst[ 0 ] > clamp[ 0 ], dst[ 1 ] < clamp[ 0 ]';
  var dst = _.argumentsArray.make( [ 4, 2 ] );
  var clamp = [ 3, 6 ];
  var got = _.cinterval.clamp( dst, clamp );
  test.identical( got, _.argumentsArray.make( [ 4, 3 ] ) );
  test.true( got === dst );

  test.case = 'clamp[ 1 ] > dst[ 0 ] > clamp[ 0 ], clamp[ 1 ] > dst[ 1 ] > clamp[ 0 ]';
  var dst = _.argumentsArray.make( [ 4, 5 ] );
  var clamp = [ 3, 6 ];
  var got = _.cinterval.clamp( dst, clamp );
  test.identical( got, _.argumentsArray.make( [ 4, 5 ] ) );
  test.true( got === dst );

  test.case = 'dst[ 0 ] > clamp[ 1 ], clamp[ 1 ] > dst[ 1 ] > clamp[ 0 ]';
  var dst = _.argumentsArray.make( [ 7, 5 ] );
  var clamp = [ 3, 6 ];
  var got = _.cinterval.clamp( dst, clamp );
  test.identical( got, _.argumentsArray.make( [ 6, 5 ] ) );
  test.true( got === dst );

  test.case = 'clamp[ 1 ] > dst[ 0 ] > clamp[ 0 ], dst[ 1 ] > clamp[ 1 ]';
  var dst = _.argumentsArray.make( [ 4, 7 ] );
  var clamp = [ 3, 6 ];
  var got = _.cinterval.clamp( dst, clamp );
  test.identical( got, _.argumentsArray.make( [ 4, 6 ] ) );
  test.true( got === dst );

  test.case = 'dst[ 0 ] > clamp[ 1 ], dst[ 1 ] > clamp[ 1 ]';
  var dst = _.argumentsArray.make( [ 8, 7 ] );
  var clamp = [ 3, 6 ];
  var got = _.cinterval.clamp( dst, clamp );
  test.identical( got, _.argumentsArray.make( [ 6, 6 ] ) );
  test.true( got === dst );

  test.case = 'dst[ 0 ] < clamp[ 0 ], dst[ 1 ] > clamp[ 1 ]';
  var dst = _.argumentsArray.make( [ 2, 7 ] );
  var clamp = [ 3, 6 ];
  var got = _.cinterval.clamp( dst, clamp );
  test.identical( got, _.argumentsArray.make( [ 3, 6 ] ) );
  test.true( got === dst );

  test.close( 'argumentsArray' );

  /* - */

  test.open( 'bufferTyped' );

  test.case = 'identical ranges';
  var dst = new U8x( [ 2, 2 ] );
  var clamp = [ 2, 2 ];
  var got = _.cinterval.clamp( dst, clamp );
  test.identical( got, new U8x( [ 2, 2 ] ) );
  test.true( got === dst );

  test.case = 'dst[ 0 ] < clamp[ 0 ], dst[ 1 ] < clamp[ 0 ]';
  var dst = new I8x( [ 2, 2 ] );
  var clamp = [ 3, 6 ];
  var got = _.cinterval.clamp( dst, clamp );
  test.identical( got, new I8x( [ 3, 3 ] ) );
  test.true( got === dst );

  test.case = 'dst[ 0 ] < clamp[ 0 ], clamp[ 1 ] > dst[ 1 ] > clamp[ 0 ]';
  var dst = new U16x( [ 2, 4 ] );
  var clamp = [ 3, 6 ];
  var got = _.cinterval.clamp( dst, clamp );
  test.identical( got, new U16x( [ 3, 4 ] ) );
  test.true( got === dst );

  test.case = 'clamp[ 1 ] > dst[ 0 ] > clamp[ 0 ], dst[ 1 ] < clamp[ 0 ]';
  var dst = new I16x( [ 4, 2 ] );
  var clamp = [ 3, 6 ];
  var got = _.cinterval.clamp( dst, clamp );
  test.identical( got, new I16x( [ 4, 3 ] ) );
  test.true( got === dst );

  test.case = 'clamp[ 1 ] > dst[ 0 ] > clamp[ 0 ], clamp[ 1 ] > dst[ 1 ] > clamp[ 0 ]';
  var dst = new U32x( [ 4, 5 ] );
  var clamp = [ 3, 6 ];
  var got = _.cinterval.clamp( dst, clamp );
  test.identical( got, new U32x( [ 4, 5 ] ) );
  test.true( got === dst );

  test.case = 'dst[ 0 ] > clamp[ 1 ], clamp[ 1 ] > dst[ 1 ] > clamp[ 0 ]';
  var dst = new I32x( [ 7, 5 ] );
  var clamp = [ 3, 6 ];
  var got = _.cinterval.clamp( dst, clamp );
  test.identical( got, new I32x( [ 6, 5 ] ) );
  test.true( got === dst );

  test.case = 'clamp[ 1 ] > dst[ 0 ] > clamp[ 0 ], dst[ 1 ] > clamp[ 1 ]';
  var dst = new F32x( [ 4, 7 ] );
  var clamp = [ 3, 6 ];
  var got = _.cinterval.clamp( dst, clamp );
  test.identical( got, new F32x( [ 4, 6 ] ) );
  test.true( got === dst );

  test.case = 'dst[ 0 ] > clamp[ 1 ], dst[ 1 ] > clamp[ 1 ]';
  var dst = new F64x( [ 8, 7 ] );
  var clamp = [ 3, 6 ];
  var got = _.cinterval.clamp( dst, clamp );
  test.identical( got, new F64x( [ 6, 6 ] ) );
  test.true( got === dst );

  test.case = 'dst[ 0 ] < clamp[ 0 ], dst[ 1 ] > clamp[ 1 ]';
  var dst = new U8x( [ 2, 7 ] );
  var clamp = [ 3, 6 ];
  var got = _.cinterval.clamp( dst, clamp );
  test.identical( got, new U8x( [ 3, 6 ] ) );
  test.true( got === dst );

  test.close( 'bufferTyped' );

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.cinterval.clamp() );

  test.case = 'not enough arguments';
  test.shouldThrowErrorSync( () => _.cinterval.clamp( [ 1, 2 ] ) );

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.cinterval.clamp( [ 1, 2 ], [ 3, 4 ], [ 5, 6 ] ) );

  test.case = 'dstRange is not a cinterval';
  test.shouldThrowErrorSync( () => _.cinterval.clamp( [ 1, 2, 3 ], [ 3, 4 ] ) );
  test.shouldThrowErrorSync( () => _.cinterval.clamp( [ 1, 'wrong' ], [ 3, 4 ] ) );
  test.shouldThrowErrorSync( () => _.cinterval.clamp( [ undefined, 1 ], [ 3, 4 ] ) );

  test.case = 'clampRange is not a cinterval';
  test.shouldThrowErrorSync( () => _.cinterval.clamp( [ 3, 4 ], [ 1, 2, 3 ] ) );
  test.shouldThrowErrorSync( () => _.cinterval.clamp( [ 3, 4 ], [ 1, 'wrong' ] ) );
  test.shouldThrowErrorSync( () => _.cinterval.clamp( [ 3, 4 ], [ undefined, 1 ] ) );
}

//

function cintervalCountElements( test )
{
  test.case = 'increment - zero, positive substruction result';
  var got = _.cinterval.countElements( [ 1, 6 ], 0 );
  test.identical( got, 0 );

  test.case = 'increment - zero, negative substruction result';
  var got = _.cinterval.countElements( [ 6, 1 ], 0 );
  test.identical( got, 0 );

  test.case = 'increment - zero, zero substruction result';
  var got = _.cinterval.countElements( [ 5, 5 ], 0 );
  test.identical( got, 0 );

  /* */

  test.case = 'increment - 1, positive substruction result';
  var got = _.cinterval.countElements( [ 1, 6 ], 1 );
  test.identical( got, 6 );

  test.case = 'increment - 1, negative substruction result';
  var got = _.cinterval.countElements( [ 6, 1 ], 1 );
  test.identical( got, -4 );

  test.case = 'increment - 1, zero substruction result';
  var got = _.cinterval.countElements( [ 5, 5 ], 1 );
  test.identical( got, 1 );

  /* */

  test.case = 'increment - undefined, positive substruction result';
  var got = _.cinterval.countElements( [ 1, 6 ], undefined );
  test.identical( got, 6 );

  test.case = 'increment - undefined, negative substruction result';
  var got = _.cinterval.countElements( [ 6, 1 ], undefined );
  test.identical( got, -4 );

  test.case = 'increment - undefined, zero substruction result';
  var got = _.cinterval.countElements( [ 5, 5 ], undefined );
  test.identical( got, 1 );

  /* */

  test.case = 'increment - not passed, positive substruction result';
  var got = _.cinterval.countElements( [ 1, 6 ] );
  test.identical( got, 6 );

  test.case = 'increment - not passed, negative substruction result';
  var got = _.cinterval.countElements( [ 6, 1 ] );
  test.identical( got, -4 );

  test.case = 'increment - not passed, zero substruction result';
  var got = _.cinterval.countElements( [ 5, 5 ] );
  test.identical( got, 1 );

  /* */

  test.case = 'increment - -1, positive substruction result';
  var got = _.cinterval.countElements( [ 1, 6 ], -1 );
  test.identical( got, -6 );

  test.case = 'increment - -1, negative substruction result';
  var got = _.cinterval.countElements( [ 6, 1 ], -1 );
  test.identical( got, 4 );

  test.case = 'increment - -1, zero substruction result';
  var got = _.cinterval.countElements( [ 5, 5 ], -1 );
  test.identical( got, -1 );

  /* */

  test.case = 'increment - 5, positive substruction result';
  var got = _.cinterval.countElements( [ 1, 6 ], 5 );
  test.identical( got, 1 );

  test.case = 'increment - 5, negative substruction result';
  var got = _.cinterval.countElements( [ 6, 1 ], 5 );
  test.identical( got, -1 );

  test.case = 'increment - 5, zero substruction result';
  var got = _.cinterval.countElements( [ 5, 5 ], 5 );
  test.identical( got, 1 );

  /* */

  test.case = 'increment - -5, positive substruction result';
  var got = _.cinterval.countElements( [ 1, 6 ], -5 );
  test.identical( got, -1 );

  test.case = 'increment - -5, negative substruction result';
  var got = _.cinterval.countElements( [ 6, 1 ], -5 );
  test.identical( got, 1 );

  test.case = 'increment - 5, zero substruction result';
  var got = _.cinterval.countElements( [ 5, 5 ], -5 );
  test.identical( got, -1 );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.cinterval.countElements() );

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.cinterval.countElements( [ 1, 2 ], 1, 'extra' ) );

  test.case = 'cinterval is not a cinterval';
  test.shouldThrowErrorSync( () => _.cinterval.countElements( [ 1, 2, 3 ], [ 3, 4 ] ) );
  test.shouldThrowErrorSync( () => _.cinterval.countElements( [ 1, 'wrong' ], [ 3, 4 ] ) );
  test.shouldThrowErrorSync( () => _.cinterval.countElements( [ undefined, 1 ], [ 3, 4 ] ) );

  test.case = 'increment is not a number and not undefined';
  test.shouldThrowErrorSync( () => _.cinterval.countElements( [ 1, 2 ], 'wrong' ) );
}

//

function cintervalLastGet( test )
{
  test.open( 'cinterval - array' );

  test.case = 'without options';
  var got = _.cinterval.lastGet( [ 1, 2 ] );
  test.identical( got, 2 );

  // test.case = 'options - undefined';
  // var options = undefined;
  // var got = _.cinterval.lastGet( [ 1, 2 ], options );
  // test.identical( got, 2 );
  // test.identical( options, undefined );
  //
  // test.case = 'options - empty map';
  // var options = {};
  // var got = _.cinterval.lastGet( [ 1, 2 ], options );
  // test.identical( got, 2 );
  // test.identical( options, { increment : 1 } );
  //
  // test.case = 'options - map with own field increment';
  // var options = { increment : 2 };
  // var got = _.cinterval.lastGet( [ 1, 2 ], options );
  // test.identical( got, 2 );
  // test.identical( options, { increment : 2 } );
  //
  // test.case = 'options - map with own field increment - undefined';
  // var options = { increment : undefined };
  // var got = _.cinterval.lastGet( [ 1, 2 ], options );
  // test.identical( got, 2 );
  // test.identical( options, { increment : 1 } );

  test.close( 'cinterval - array' );

  /* - */

  test.open( 'cinterval - unroll' );

  test.case = 'without options';
  var got = _.cinterval.lastGet( _.unroll.make( [ 1, 2 ] ) );
  test.identical( got, 2 );

  // test.case = 'options - undefined';
  // var options = undefined;
  // var got = _.cinterval.lastGet( _.unroll.make( [ 1, 2 ] ), options );
  // test.identical( got, 2 );
  // test.identical( options, undefined );
  //
  // test.case = 'options - empty map';
  // var options = {};
  // var got = _.cinterval.lastGet( _.unroll.make( [ 1, 2 ] ), options );
  // test.identical( got, 2 );
  // test.identical( options, { increment : 1 } );
  //
  // test.case = 'options - map with own field increment';
  // var options = { increment : 2 };
  // var got = _.cinterval.lastGet( _.unroll.make( [ 1, 2 ] ), options );
  // test.identical( got, 2 );
  // test.identical( options, { increment : 2 } );
  //
  // test.case = 'options - map with own field increment - undefined';
  // var options = { increment : undefined };
  // var got = _.cinterval.lastGet( _.unroll.make( [ 1, 2 ] ), options );
  // test.identical( got, 2 );
  // test.identical( options, { increment : 1 } );

  test.close( 'cinterval - unroll' );

  /* - */

  test.open( 'cinterval - argumentsArray' );

  test.case = 'without options';
  var got = _.cinterval.lastGet( _.argumentsArray.make( [ 1, 2 ] ) );
  test.identical( got, 2 );

  // test.case = 'options - undefined';
  // var options = undefined;
  // var got = _.cinterval.lastGet( _.argumentsArray.make( [ 1, 2 ] ), options );
  // test.identical( got, 2 );
  // test.identical( options, undefined );
  //
  // test.case = 'options - empty map';
  // var options = {};
  // var got = _.cinterval.lastGet( _.argumentsArray.make( [ 1, 2 ] ), options );
  // test.identical( got, 2 );
  // test.identical( options, { increment : 1 } );
  //
  // test.case = 'options - map with own field increment';
  // var options = { increment : 2 };
  // var got = _.cinterval.lastGet( _.argumentsArray.make( [ 1, 2 ] ), options );
  // test.identical( got, 2 );
  // test.identical( options, { increment : 2 } );
  //
  // test.case = 'options - map with own field increment - undefined';
  // var options = { increment : undefined };
  // var got = _.cinterval.lastGet( _.argumentsArray.make( [ 1, 2 ] ), options );
  // test.identical( got, 2 );
  // test.identical( options, { increment : 1 } );

  test.close( 'cinterval - argumentsArray' );

  /* - */

  test.open( 'cinterval - BufferTyped' );

  test.case = 'without options';
  var got = _.cinterval.lastGet( new U8x( [ 1, 2 ] ) );
  test.identical( got, 2 );

  // test.case = 'options - undefined';
  // var options = undefined;
  // var got = _.cinterval.lastGet( new I16x( [ 1, 2 ] ), options );
  // test.identical( got, 2 );
  // test.identical( options, undefined );
  //
  // test.case = 'options - empty map';
  // var options = {};
  // var got = _.cinterval.lastGet( new F32x( [ 1, 2 ] ), options );
  // test.identical( got, 2 );
  // test.identical( options, { increment : 1 } );
  //
  // test.case = 'options - map with own field increment';
  // var options = { increment : 2 };
  // var got = _.cinterval.lastGet( new F64x( [ 1, 2 ] ), options );
  // test.identical( got, 2 );
  // test.identical( options, { increment : 2 } );
  //
  // test.case = 'options - map with own field increment - undefined';
  // var options = { increment : undefined };
  // var got = _.cinterval.lastGet( new I8x( [ 1, 2 ] ), options );
  // test.identical( got, 2 );
  // test.identical( options, { increment : 1 } );

  test.close( 'cinterval - BufferTyped' );

  /* - */

  test.open( 'cinterval - map' );

  test.case = 'without options, last - undefined';
  var got = _.cinterval.lastGet( { first : undefined, last : undefined } );
  test.identical( got, undefined );

  test.case = 'without options, last - not exists';
  var got = _.cinterval.lastGet( { first : undefined } );
  test.identical( got, undefined );

  test.case = 'without options, last - number';
  var got = _.cinterval.lastGet( { first : 1, last : 2 } );
  test.identical( got, 2 );

  /* */

  // test.case = 'options - undefined, last - undefined';
  // var options = undefined;
  // var got = _.cinterval.lastGet( { first : undefined, last : undefined }, options );
  // test.identical( got, undefined );
  // test.identical( options, undefined );
  //
  // test.case = 'options - undefined, last - not exists';
  // var options = undefined;
  // var got = _.cinterval.lastGet( { first : undefined }, options );
  // test.identical( got, undefined );
  // test.identical( options, undefined );
  //
  // test.case = 'options - undefined, last - number';
  // var options = undefined;
  // var got = _.cinterval.lastGet( { first : 1, last : 2 }, options );
  // test.identical( got, 2 );
  // test.identical( options, undefined );
  //
  // /* */
  //
  // test.case = 'options - empty map, last - undefined';
  // var options = {};
  // var got = _.cinterval.lastGet( { first : undefined, last : undefined }, options );
  // test.identical( got, undefined );
  // test.identical( options, { increment : 1 } );
  //
  // test.case = 'options - empty map, last - not exists';
  // var options = {};
  // var got = _.cinterval.lastGet( { first : undefined }, options );
  // test.identical( got, undefined );
  // test.identical( options, { increment : 1 } );
  //
  // test.case = 'options - empty map, last - number';
  // var options = {};
  // var got = _.cinterval.lastGet( { first : 1, last : 2 }, options );
  // test.identical( got, 2 );
  // test.identical( options, { increment : 1 } );
  //
  // /* */
  //
  // test.case = 'options - map with own field increment, last - undefined';
  // var options = { increment : 2 };
  // var got = _.cinterval.lastGet( { first : undefined, last : undefined }, options );
  // test.identical( got, undefined );
  // test.identical( options, { increment : 2 } );
  //
  // test.case = 'options - map with own field increment, last - not exists';
  // var options = { increment : 2 };
  // var got = _.cinterval.lastGet( { first : undefined }, options );
  // test.identical( got, undefined );
  // test.identical( options, { increment : 2 } );
  //
  // test.case = 'options - map with own field increment, last - number';
  // var options = { increment : 2 };
  // var got = _.cinterval.lastGet( { first : 1, last : 2 }, options );
  // test.identical( got, 2 );
  // test.identical( options, { increment : 2 } );
  //
  // /* */
  //
  // test.case = 'options - map with own field increment - undefined, last - undefined';
  // var options = { increment : undefined };
  // var got = _.cinterval.lastGet( { first : undefined, last : undefined }, options );
  // test.identical( got, undefined );
  // test.identical( options, { increment : 1 } );
  //
  // test.case = 'options - map with own field increment - undefined, last - not exists';
  // var options = { increment : undefined };
  // var got = _.cinterval.lastGet( { first : undefined }, options );
  // test.identical( got, undefined );
  // test.identical( options, { increment : 1 } );
  //
  // test.case = 'options - map with own field increment - undefined, last - number';
  // var options = { increment : undefined };
  // var got = _.cinterval.lastGet( { first : 1, last : 2 }, options );
  // test.identical( got, 2 );
  // test.identical( options, { increment : 1 } );

  test.close( 'cinterval - map' );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.cinterval.lastGet() );

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.cinterval.lastGet( [ 1, 2 ], {}, 'extra' ) );

  test.case = 'cinterval is a long, but is not a cinterval';
  test.shouldThrowErrorSync( () => _.cinterval.lastGet( [ 1, 2, 3 ] ) );
  test.shouldThrowErrorSync( () => _.cinterval.lastGet( [ 1, 'wrong' ] ) );
  test.shouldThrowErrorSync( () => _.cinterval.lastGet( [ undefined, 1 ] ) );

  test.case = 'wrong type of cinterval';
  test.shouldThrowErrorSync( () => _.cinterval.lastGet( new Set( [ 1, 2 ] ) ) );
  test.shouldThrowErrorSync( () => _.cinterval.lastGet( new HashMap( [ [ 1, 2 ] ] ) ) );
}

// --
// linterval
// --

function lintervalIsEmpty( test )
{
  test.case = 'undefined';
  var got = _.linterval.isEmpty( undefined );
  var expected = false;
  test.identical( got, expected );

  test.case = 'null';
  var got = _.linterval.isEmpty( null );
  var expected = false;
  test.identical( got, expected );

  test.case = 'false';
  var got = _.linterval.isEmpty( false );
  var expected = false;
  test.identical( got, expected );

  test.case = 'empty string';
  var got = _.linterval.isEmpty( '' );
  var expected = false;
  test.identical( got, expected );

  test.case = 'zero';
  var got = _.linterval.isEmpty( 0 );
  var expected = false;
  test.identical( got, expected );

  test.case = 'NaN';
  var got = _.linterval.isEmpty( NaN );
  var expected = false;
  test.identical( got, expected );

  test.case = 'a boolean';
  var got = _.linterval.isEmpty( true );
  var expected = false;
  test.identical( got, expected );

  test.case = 'a number';
  var got = _.linterval.isEmpty( 13 );
  var expected = false;
  test.identical( got, expected );

  test.case = 'a function';
  var got = _.linterval.isEmpty( function() {} );
  var expected = false;
  test.identical( got, expected );

  test.case = 'constructor';
  function Constr( x )
  {
    this.x = x;
    return this;
  }
  var got = _.linterval.isEmpty( new Constr( 0 ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'a string';
  var got = _.linterval.isEmpty( 'str' );
  var expected = false;
  test.identical( got, expected );

  test.case = 'BufferRaw';
  var got = _.linterval.isEmpty( new BufferRaw( 5 ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'BufferView';
  var got = _.linterval.isEmpty( new BufferView( new BufferRaw( 5 ) ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'Set';
  var got = _.linterval.isEmpty( new Set( [ 5 ] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'Map';
  var got = _.linterval.isEmpty( new HashMap( [ [ 1, 2 ] ] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'pure empty map';
  var got = _.linterval.isEmpty( Object.create( null ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'pure map';
  var src = Object.create( null );
  src.x = 1;
  var got = _.linterval.isEmpty( src );
  var expected = false;
  test.identical( got, expected );

  test.case = 'map from pure map';
  var src = Object.create( Object.create( null ) );
  var got = _.linterval.isEmpty( src );
  var expected = false;
  test.identical( got, expected );

  test.case = 'an empty object';
  var got = _.linterval.isEmpty( {} );
  var expected = false;
  test.identical( got, expected );

  test.case = 'an object';
  var got = _.linterval.isEmpty( { a : 7, b : 13 } );
  var expected = false;
  test.identical( got, expected );

  /* */

  test.case = 'array.length = 0';
  var got = _.linterval.isEmpty( [] );
  var expected = false;
  test.identical( got, expected );

  test.case = 'array.length = 1';
  var got = _.linterval.isEmpty( [ 1 ] );
  var expected = false;
  test.identical( got, expected );

  test.case = 'array.length = 2, different numbers';
  var got = _.linterval.isEmpty( [ 1, 2 ] );
  var expected = false;
  test.identical( got, expected );

  test.case = 'array.length = 2, equal numbers';
  var got = _.linterval.isEmpty( [ 1, 1 ] );
  var expected = false;
  test.identical( got, expected );

  test.case = 'array.length = 2, linterval[ 0 ] - linterval[ 1 ] === 1';
  var got = _.linterval.isEmpty( [ 1, 0 ] );
  var expected = true;
  test.identical( got, expected );

  test.case = 'array.length = 2, number and undefined';
  var got = _.linterval.isEmpty( [ 1, undefined ] );
  var expected = false;
  test.identical( got, expected );

  test.case = 'array.length > 2';
  var got = _.linterval.isEmpty( [ 1, 2, 3 ] );
  var expected = false;
  test.identical( got, expected );

  test.case = 'unroll.length = 0';
  var got = _.linterval.isEmpty( _.unroll.make( [] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'unroll.length = 1';
  var got = _.linterval.isEmpty( _.unroll.make( [ 1 ] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'unroll.length = 2, different numbers';
  var got = _.linterval.isEmpty( _.unroll.make( [ 1, 2 ] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'unroll.length = 2, equal numbers';
  var got = _.linterval.isEmpty( _.unroll.make( [ 10, 10 ] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'unroll.length = 2, linterval[ 1 ] === 0';
  var got = _.linterval.isEmpty( _.unroll.make( [ 10, 0 ] ) );
  var expected = true;
  test.identical( got, expected );

  test.case = 'unroll.length = 2, number and undefined';
  var got = _.linterval.isEmpty( _.unroll.make( [ 1, undefined ] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'unroll.length > 2';
  var got = _.linterval.isEmpty( _.unroll.make( [ 1, 2, 3 ] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'argumentsArray.length = 0';
  var got = _.linterval.isEmpty( _.argumentsArray.make( [] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'argumentsArray.length = 1';
  var got = _.linterval.isEmpty( _.argumentsArray.make( [ 1 ] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'argumentsArray.length = 2, different numbers';
  var got = _.linterval.isEmpty( _.argumentsArray.make( [ 1, 2 ] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'argumentsArray.length = 2, equal numbers';
  var got = _.linterval.isEmpty( _.argumentsArray.make( [ -2, -2 ] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'argumentsArray.length = 2, linterval[ 1 ] === 0';
  var got = _.linterval.isEmpty( _.argumentsArray.make( [ -1, 0 ] ) );
  var expected = true;
  test.identical( got, expected );

  test.case = 'argumentsArray.length = 2, number and undefined';
  var got = _.linterval.isEmpty( _.argumentsArray.make( [ 1, undefined ] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'argumentsArray.length > 2';
  var got = _.linterval.isEmpty( _.argumentsArray.make( [ 1, 2, 3 ] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'BufferTyped.length = 0';
  var got = _.linterval.isEmpty( new U8x() );
  var expected = false;
  test.identical( got, expected );

  test.case = 'BufferTyped.length = 1';
  var got = _.linterval.isEmpty( new I16x( 1 ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'BufferTyped.length = 2, different numbers';
  var got = _.linterval.isEmpty( new F32x( [ 1, 3 ] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'BufferTyped.length = 2, equal numbers';
  var got = _.linterval.isEmpty( new F32x( [ 1, 1 ] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'BufferTyped.length = 2, linterval[ 1 ] === 0';
  var got = _.linterval.isEmpty( new F32x( [ 0, 0 ] ) );
  var expected = true;
  test.identical( got, expected );

  test.case = 'BufferTyped.length > 2';
  var got = _.linterval.isEmpty( new F32x( 4 ) );
  var expected = false;
  test.identical( got, expected );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.linterval.isEmpty() );

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.linterval.isEmpty( [ 1, 2 ], 'extra' ) );
}

//

function lintervalIsPopulated( test )
{
  test.case = 'undefined';
  var got = _.linterval.isPopulated( undefined );
  var expected = false;
  test.identical( got, expected );

  test.case = 'null';
  var got = _.linterval.isPopulated( null );
  var expected = false;
  test.identical( got, expected );

  test.case = 'false';
  var got = _.linterval.isPopulated( false );
  var expected = false;
  test.identical( got, expected );

  test.case = 'empty string';
  var got = _.linterval.isPopulated( '' );
  var expected = false;
  test.identical( got, expected );

  test.case = 'zero';
  var got = _.linterval.isPopulated( 0 );
  var expected = false;
  test.identical( got, expected );

  test.case = 'NaN';
  var got = _.linterval.isPopulated( NaN );
  var expected = false;
  test.identical( got, expected );

  test.case = 'a boolean';
  var got = _.linterval.isPopulated( true );
  var expected = false;
  test.identical( got, expected );

  test.case = 'a number';
  var got = _.linterval.isPopulated( 13 );
  var expected = false;
  test.identical( got, expected );

  test.case = 'a function';
  var got = _.linterval.isPopulated( function() {} );
  var expected = false;
  test.identical( got, expected );

  test.case = 'constructor';
  function Constr( x )
  {
    this.x = x;
    return this;
  }
  var got = _.linterval.isPopulated( new Constr( 0 ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'a string';
  var got = _.linterval.isPopulated( 'str' );
  var expected = false;
  test.identical( got, expected );

  test.case = 'BufferRaw';
  var got = _.linterval.isPopulated( new BufferRaw( 5 ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'BufferView';
  var got = _.linterval.isPopulated( new BufferView( new BufferRaw( 5 ) ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'Set';
  var got = _.linterval.isPopulated( new Set( [ 5 ] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'Map';
  var got = _.linterval.isPopulated( new HashMap( [ [ 1, 2 ] ] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'pure empty map';
  var got = _.linterval.isPopulated( Object.create( null ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'pure map';
  var src = Object.create( null );
  src.x = 1;
  var got = _.linterval.isPopulated( src );
  var expected = false;
  test.identical( got, expected );

  test.case = 'map from pure map';
  var src = Object.create( Object.create( null ) );
  var got = _.linterval.isPopulated( src );
  var expected = false;
  test.identical( got, expected );

  test.case = 'an empty object';
  var got = _.linterval.isPopulated( {} );
  var expected = false;
  test.identical( got, expected );

  test.case = 'an object';
  var got = _.linterval.isPopulated( { a : 7, b : 13 } );
  var expected = false;
  test.identical( got, expected );

  /* */

  test.case = 'array.length = 0';
  var got = _.linterval.isPopulated( [] );
  var expected = false;
  test.identical( got, expected );

  test.case = 'array.length = 1';
  var got = _.linterval.isPopulated( [ 1 ] );
  var expected = false;
  test.identical( got, expected );

  test.case = 'array.length = 2, different numbers';
  var got = _.linterval.isPopulated( [ 1, 2 ] );
  var expected = true;
  test.identical( got, expected );

  test.case = 'array.length = 2, equal numbers';
  var got = _.linterval.isPopulated( [ 1, 1 ] );
  var expected = true;
  test.identical( got, expected );

  test.case = 'array.length = 2, linterval[ 0 ] - linterval[ 1 ] === 1';
  var got = _.linterval.isPopulated( [ 1, 0 ] );
  var expected = false;
  test.identical( got, expected );

  test.case = 'array.length = 2, number and undefined';
  var got = _.linterval.isPopulated( [ 1, undefined ] );
  var expected = false;
  test.identical( got, expected );

  test.case = 'array.length > 2';
  var got = _.linterval.isPopulated( [ 1, 2, 3 ] );
  var expected = false;
  test.identical( got, expected );

  test.case = 'unroll.length = 0';
  var got = _.linterval.isPopulated( _.unroll.make( [] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'unroll.length = 1';
  var got = _.linterval.isPopulated( _.unroll.make( [ 1 ] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'unroll.length = 2, different numbers';
  var got = _.linterval.isPopulated( _.unroll.make( [ 1, 2 ] ) );
  var expected = true;
  test.identical( got, expected );

  test.case = 'unroll.length = 2, equal numbers';
  var got = _.linterval.isPopulated( _.unroll.make( [ 10, 10 ] ) );
  var expected = true;
  test.identical( got, expected );

  test.case = 'unroll.length = 2, linterval[ 0 ] - linterval[ 1 ] === 1';
  var got = _.linterval.isPopulated( _.unroll.make( [ 10, 9 ] ) );
  var expected = true;
  test.identical( got, expected );

  test.case = 'unroll.length = 2, number and undefined';
  var got = _.linterval.isPopulated( _.unroll.make( [ 1, undefined ] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'unroll.length > 2';
  var got = _.linterval.isPopulated( _.unroll.make( [ 1, 2, 3 ] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'argumentsArray.length = 0';
  var got = _.linterval.isPopulated( _.argumentsArray.make( [] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'argumentsArray.length = 1';
  var got = _.linterval.isPopulated( _.argumentsArray.make( [ 1 ] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'argumentsArray.length = 2, different numbers';
  var got = _.linterval.isPopulated( _.argumentsArray.make( [ 1, 2 ] ) );
  var expected = true;
  test.identical( got, expected );

  test.case = 'argumentsArray.length = 2, equal numbers';
  var got = _.linterval.isPopulated( _.argumentsArray.make( [ -2, -2 ] ) );
  var expected = true;
  test.identical( got, expected );

  test.case = 'argumentsArray.length = 2, linterval[ 0 ] - linterval[ 1 ] === 1';
  var got = _.linterval.isPopulated( _.argumentsArray.make( [ -1, -2 ] ) );
  var expected = true;
  test.identical( got, expected );

  test.case = 'argumentsArray.length = 2, number and undefined';
  var got = _.linterval.isPopulated( _.argumentsArray.make( [ 1, undefined ] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'argumentsArray.length > 2';
  var got = _.linterval.isPopulated( _.argumentsArray.make( [ 1, 2, 3 ] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'BufferTyped.length = 0';
  var got = _.linterval.isPopulated( new U8x() );
  var expected = false;
  test.identical( got, expected );

  test.case = 'BufferTyped.length = 1';
  var got = _.linterval.isPopulated( new I16x( 1 ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'BufferTyped.length = 2, different numbers';
  var got = _.linterval.isPopulated( new F32x( [ 1, 3 ] ) );
  var expected = true;
  test.identical( got, expected );

  test.case = 'BufferTyped.length = 2, equal numbers';
  var got = _.linterval.isPopulated( new F32x( [ 1, 1 ] ) );
  var expected = true;
  test.identical( got, expected );

  test.case = 'BufferTyped.length = 2, linterval[ 0 ] - linterval[ 1 ] === 1';
  var got = _.linterval.isPopulated( new F32x( [ 1, 0 ] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'BufferTyped.length > 2';
  var got = _.linterval.isPopulated( new F32x( 4 ) );
  var expected = false;
  test.identical( got, expected );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.linterval.isPopulated() );

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.linterval.isPopulated( [ 1, 2 ], 'extra' ) );
}

//

// function lintervalInInclusive( test )
// {
//   test.case = 'srcNumber - number, srcNumber < linterval[ 0 ]';
//   var got = _.linterval.inInclusive( [ 2, 5 ], 1 );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - number, srcNumber = linterval[ 0 ]';
//   var got = _.linterval.inInclusive( [ 2, 5 ], 2 );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - number, srcNumber > linterval[ 1 ], srcNumber > linterval[ 0 ] + linterval[ 1 ]';
//   var got = _.linterval.inInclusive( [ 2, 5 ], 7 );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - number, srcNumber > linterval[ 1 ], srcNumber === linterval[ 0 ] + linterval[ 1 ]';
//   var got = _.linterval.inInclusive( [ 2, 5 ], 6 );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - number, srcNumber = linterval[ 1 ]';
//   var got = _.linterval.inInclusive( [ 2, 5 ], 5 );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - number, linterval[ 0 ] < srcNumber < linterval[ 1 ]';
//   var got = _.linterval.inInclusive( [ 2, 5 ], 4 );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - array, srcNumber < linterval[ 0 ]';
//   var got = _.linterval.inInclusive( [ 2, 5 ], [ 0 ] );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - array, srcNumber = linterval[ 0 ]';
//   var got = _.linterval.inInclusive( [ 2, 5 ], [ 0, 0 ] );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - array, srcNumber > linterval[ 1 ], srcNumber > linterval[ 0 ] + linterval[ 1 ]';
//   var got = _.linterval.inInclusive( [ 2, 5 ], [ 0, 0, 0, 0, 0, 0, 0 ] );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - array, srcNumber > linterval[ 1 ], srcNumber === linterval[ 0 ] + linterval[ 1 ]';
//   var got = _.linterval.inInclusive( [ 2, 5 ], [ 0, 0, 0, 0, 0, 0 ] );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - array, srcNumber = linterval[ 1 ]';
//   var got = _.linterval.inInclusive( [ 2, 5 ], [ 0, 0, 0, 0, 0 ] );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - array, linterval[ 0 ] < srcNumber < linterval[ 1 ]';
//   var got = _.linterval.inInclusive( [ 2, 5 ], [ 0, 0, 0, 0 ] );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - unroll, srcNumber < linterval[ 0 ]';
//   var got = _.linterval.inInclusive( [ 2, 5 ], _.unroll.make( [ 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - unroll, srcNumber = linterval[ 0 ]';
//   var got = _.linterval.inInclusive( [ 2, 5 ], _.unroll.make( [ 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - unroll, srcNumber > linterval[ 1 ], srcNumber > linterval[ 0 ] + linterval[ 1 ]';
//   var got = _.linterval.inInclusive( [ 2, 5 ], _.unroll.make( [ 0, 0, 0, 0, 0, 0, 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - unroll, srcNumber > linterval[ 1 ], srcNumber === linterval[ 0 ] + linterval[ 1 ]';
//   var got = _.linterval.inInclusive( [ 2, 5 ], _.unroll.make( [ 0, 0, 0, 0, 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - unroll, srcNumber = linterval[ 1 ]';
//   var got = _.linterval.inInclusive( [ 2, 5 ], _.unroll.make( [ 0, 0, 0, 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - unroll, linterval[ 0 ] < srcNumber < linterval[ 1 ]';
//   var got = _.linterval.inInclusive( [ 2, 5 ], _.unroll.make( [ 0, 0, 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - argumentsArray, srcNumber < linterval[ 0 ]';
//   var got = _.linterval.inInclusive( [ 2, 5 ], _.argumentsArray.make( [ 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - argumentsArray, srcNumber = linterval[ 0 ]';
//   var got = _.linterval.inInclusive( [ 2, 5 ], _.argumentsArray.make( [ 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - argumentsArray, srcNumber > linterval[ 1 ], srcNumber > linterval[ 0 ] + linterval[ 1 ]';
//   var got = _.linterval.inInclusive( [ 2, 5 ], _.argumentsArray.make( [ 0, 0, 0, 0, 0, 0, 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - argumentsArray, srcNumber > linterval[ 1 ], srcNumber === linterval[ 0 ] + linterval[ 1 ]';
//   var got = _.linterval.inInclusive( [ 2, 5 ], _.argumentsArray.make( [ 0, 0, 0, 0, 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - argumentsArray, srcNumber = linterval[ 1 ]';
//   var got = _.linterval.inInclusive( [ 2, 5 ], _.argumentsArray.make( [ 0, 0, 0, 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - argumentsArray, linterval[ 0 ] < srcNumber < linterval[ 1 ]';
//   var got = _.linterval.inInclusive( [ 2, 5 ], _.argumentsArray.make( [ 0, 0, 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - BufferTyped, srcNumber < linterval[ 0 ]';
//   var got = _.linterval.inInclusive( [ 2, 5 ], new I8x( [ 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - BufferTyped, srcNumber = linterval[ 0 ]';
//   var got = _.linterval.inInclusive( [ 2, 5 ], new I8x( [ 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - BufferTyped, srcNumber > linterval[ 1 ], srcNumber > linterval[ 0 ] + linterval[ 1 ]';
//   var got = _.linterval.inInclusive( [ 2, 5 ], new U16x( [ 0, 0, 0, 0, 0, 0, 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - BufferTyped, srcNumber > linterval[ 1 ], srcNumber === linterval[ 0 ] + linterval[ 1 ]';
//   var got = _.linterval.inInclusive( [ 2, 5 ], new U16x( [ 0, 0, 0, 0, 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - BufferTyped, srcNumber = linterval[ 1 ]';
//   var got = _.linterval.inInclusive( [ 2, 5 ], new U16x( [ 0, 0, 0, 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - BufferTyped, linterval[ 0 ] < srcNumber < linterval[ 1 ]';
//   var got = _.linterval.inInclusive( [ 2, 5 ], new F32x( [ 0, 0, 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   /* - */
//
//   if( !Config.debug )
//   return;
//
//   test.case = 'without arguments';
//   test.shouldThrowErrorSync( () => _.linterval.inInclusive() );
//
//   test.case = 'not enough arguments';
//   test.shouldThrowErrorSync( () => _.linterval.inInclusive( [ 1, 2 ] ) );
//
//   test.case = 'extra arguments';
//   test.shouldThrowErrorSync( () => _.linterval.inInclusive( [ 1, 2 ], 3, 'extra' ) );
//
//   test.case = 'linterval is not linterval';
//   test.shouldThrowErrorSync( () => _.linterval.inInclusive( 'wrong', 3 ) );
//
//   test.case = 'srcNumber is not Long, not Number';
//   test.shouldThrowErrorSync( () => _.linterval.inInclusive( [ 1, 2 ], 'wrong' ) );
// }
//
// //
//
// function lintervalInExclusive( test )
// {
//   test.case = 'srcNumber - number, srcNumber < linterval[ 0 ]';
//   var got = _.linterval.inExclusive( [ 2, 5 ], 1 );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - number, srcNumber = linterval[ 0 ]';
//   var got = _.linterval.inExclusive( [ 2, 5 ], 2 );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - number, srcNumber > linterval[ 0 ] + linterval[ 1 ]';
//   var got = _.linterval.inExclusive( [ 2, 5 ], 8 );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - number, srcNumber === linterval[ 0 ] + linterval[ 1 ]';
//   var got = _.linterval.inExclusive( [ 2, 5 ], 7 );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - number, srcNumber = linterval[ 1 ]';
//   var got = _.linterval.inExclusive( [ 2, 5 ], 5 );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - number, linterval[ 0 ] < srcNumber < linterval[ 1 ]';
//   var got = _.linterval.inExclusive( [ 2, 5 ], 4 );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - array, srcNumber < linterval[ 0 ]';
//   var got = _.linterval.inExclusive( [ 2, 5 ], [ 0 ] );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - array, srcNumber = linterval[ 0 ]';
//   var got = _.linterval.inExclusive( [ 2, 5 ], [ 0, 0 ] );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - array, srcNumber > linterval[ 0 ] + linterval[ 1 ]';
//   var got = _.linterval.inExclusive( [ 2, 5 ], [ 0, 0, 0, 0, 0, 0, 0, 0 ] );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - array, srcNumber === linterval[ 0 ] + linterval[ 1 ]';
//   var got = _.linterval.inExclusive( [ 2, 5 ], [ 0, 0, 0, 0, 0, 0, 0 ] );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - array, srcNumber = linterval[ 1 ]';
//   var got = _.linterval.inExclusive( [ 2, 5 ], [ 0, 0, 0, 0, 0 ] );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - array, linterval[ 0 ] < srcNumber < linterval[ 1 ]';
//   var got = _.linterval.inExclusive( [ 2, 5 ], [ 0, 0, 0, 0 ] );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - unroll, srcNumber < linterval[ 0 ]';
//   var got = _.linterval.inExclusive( [ 2, 5 ], _.unroll.make( [ 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - unroll, srcNumber = linterval[ 0 ]';
//   var got = _.linterval.inExclusive( [ 2, 5 ], _.unroll.make( [ 0, 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - unroll, srcNumber > linterval[ 1 ], srcNumber > linterval[ 0 ] + linterval[ 1 ]';
//   var got = _.linterval.inExclusive( [ 2, 5 ], _.unroll.make( [ 0, 0, 0, 0, 0, 0, 0, 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - unroll, srcNumber > linterval[ 1 ], srcNumber === linterval[ 0 ] + linterval[ 1 ]';
//   var got = _.linterval.inExclusive( [ 2, 5 ], _.unroll.make( [ 0, 0, 0, 0, 0, 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - unroll, srcNumber = linterval[ 1 ]';
//   var got = _.linterval.inExclusive( [ 2, 5 ], _.unroll.make( [ 0, 0, 0, 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - unroll, linterval[ 0 ] < srcNumber < linterval[ 1 ]';
//   var got = _.linterval.inExclusive( [ 2, 5 ], _.unroll.make( [ 0, 0, 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - argumentsArray, srcNumber < linterval[ 0 ]';
//   var got = _.linterval.inExclusive( [ 2, 5 ], _.argumentsArray.make( [ 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - argumentsArray, srcNumber = linterval[ 0 ]';
//   var got = _.linterval.inExclusive( [ 2, 5 ], _.argumentsArray.make( [ 0, 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - argumentsArray, srcNumber > linterval[ 0 ] + linterval[ 1 ]';
//   var got = _.linterval.inExclusive( [ 2, 5 ], _.argumentsArray.make( [ 0, 0, 0, 0, 0, 0, 0, 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - argumentsArray, srcNumber === linterval[ 0 ] + linterval[ 1 ]';
//   var got = _.linterval.inExclusive( [ 2, 5 ], _.argumentsArray.make( [ 0, 0, 0, 0, 0, 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - argumentsArray, srcNumber = linterval[ 1 ]';
//   var got = _.linterval.inExclusive( [ 2, 5 ], _.argumentsArray.make( [ 0, 0, 0, 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - argumentsArray, linterval[ 0 ] < srcNumber < linterval[ 1 ]';
//   var got = _.linterval.inExclusive( [ 2, 5 ], _.argumentsArray.make( [ 0, 0, 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - BufferTyped, srcNumber < linterval[ 0 ]';
//   var got = _.linterval.inExclusive( [ 2, 5 ], new I8x( [ 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - BufferTyped, srcNumber = linterval[ 0 ]';
//   var got = _.linterval.inExclusive( [ 2, 5 ], new I8x( [ 0, 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - BufferTyped, srcNumber > linterval[ 0 ] + linterval[ 1 ]';
//   var got = _.linterval.inExclusive( [ 2, 5 ], new U16x( [ 0, 0, 0, 0, 0, 0, 0, 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - BufferTyped, srcNumber === linterval[ 0 ] + linterval[ 1 ]';
//   var got = _.linterval.inExclusive( [ 2, 5 ], new U16x( [ 0, 0, 0, 0, 0, 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - BufferTyped, srcNumber = linterval[ 1 ]';
//   var got = _.linterval.inExclusive( [ 2, 5 ], new U16x( [ 0, 0, 0, 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - BufferTyped, linterval[ 0 ] < srcNumber < linterval[ 1 ]';
//   var got = _.linterval.inExclusive( [ 2, 5 ], new F32x( [ 0, 0, 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   /* - */
//
//   if( !Config.debug )
//   return;
//
//   test.case = 'without arguments';
//   test.shouldThrowErrorSync( () => _.linterval.inExclusive() );
//
//   test.case = 'not enough arguments';
//   test.shouldThrowErrorSync( () => _.linterval.inExclusive( [ 1, 2 ] ) );
//
//   test.case = 'extra arguments';
//   test.shouldThrowErrorSync( () => _.linterval.inExclusive( [ 1, 2 ], 3, 'extra' ) );
//
//   test.case = 'linterval is not linterval';
//   test.shouldThrowErrorSync( () => _.linterval.inExclusive( 'wrong', 3 ) );
//
//   test.case = 'srcNumber is not Long, not Number';
//   test.shouldThrowErrorSync( () => _.linterval.inExclusive( [ 1, 2 ], 'wrong' ) );
// }
//
// //
//
// function lintervalInInclusiveLeft( test )
// {
//   test.case = 'srcNumber - number, srcNumber < linterval[ 0 ]';
//   var got = _.linterval.inInclusiveLeft( [ 2, 5 ], 1 );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - number, srcNumber = linterval[ 0 ]';
//   var got = _.linterval.inInclusiveLeft( [ 2, 5 ], 2 );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - number, srcNumber > linterval[ 1 ], srcNumber > linterval[ 0 ] + linterval[ 1 ]';
//   var got = _.linterval.inInclusiveLeft( [ 2, 5 ], 7 );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - number, srcNumber > linterval[ 1 ], srcNumber === linterval[ 0 ] + linterval[ 1 ]';
//   var got = _.linterval.inInclusiveLeft( [ 2, 5 ], 6 );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - number, srcNumber = linterval[ 1 ]';
//   var got = _.linterval.inInclusiveLeft( [ 2, 5 ], 5 );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - number, linterval[ 0 ] < srcNumber < linterval[ 1 ]';
//   var got = _.linterval.inInclusiveLeft( [ 2, 5 ], 4 );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - array, srcNumber < linterval[ 0 ]';
//   var got = _.linterval.inInclusiveLeft( [ 2, 5 ], [ 0 ] );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - array, srcNumber = linterval[ 0 ]';
//   var got = _.linterval.inInclusiveLeft( [ 2, 5 ], [ 0, 0 ] );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - array, srcNumber > linterval[ 1 ], srcNumber > linterval[ 0 ] + linterval[ 1 ]';
//   var got = _.linterval.inInclusiveLeft( [ 2, 5 ], [ 0, 0, 0, 0, 0, 0, 0 ] );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - array, srcNumber > linterval[ 1 ], srcNumber === linterval[ 0 ] + linterval[ 1 ]';
//   var got = _.linterval.inInclusiveLeft( [ 2, 5 ], [ 0, 0, 0, 0, 0, 0 ] );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - array, srcNumber = linterval[ 1 ]';
//   var got = _.linterval.inInclusiveLeft( [ 2, 5 ], [ 0, 0, 0, 0, 0 ] );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - array, linterval[ 0 ] < srcNumber < linterval[ 1 ]';
//   var got = _.linterval.inInclusiveLeft( [ 2, 5 ], [ 0, 0, 0, 0 ] );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - unroll, srcNumber < linterval[ 0 ]';
//   var got = _.linterval.inInclusiveLeft( [ 2, 5 ], _.unroll.make( [ 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - unroll, srcNumber = linterval[ 0 ]';
//   var got = _.linterval.inInclusiveLeft( [ 2, 5 ], _.unroll.make( [ 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - unroll, srcNumber > linterval[ 1 ], srcNumber > linterval[ 0 ] + linterval[ 1 ]';
//   var got = _.linterval.inInclusiveLeft( [ 2, 5 ], _.unroll.make( [ 0, 0, 0, 0, 0, 0, 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - unroll, srcNumber > linterval[ 1 ], srcNumber === linterval[ 0 ] + linterval[ 1 ]';
//   var got = _.linterval.inInclusiveLeft( [ 2, 5 ], _.unroll.make( [ 0, 0, 0, 0, 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - unroll, srcNumber = linterval[ 1 ]';
//   var got = _.linterval.inInclusiveLeft( [ 2, 5 ], _.unroll.make( [ 0, 0, 0, 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - unroll, linterval[ 0 ] < srcNumber < linterval[ 1 ]';
//   var got = _.linterval.inInclusiveLeft( [ 2, 5 ], _.unroll.make( [ 0, 0, 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - argumentsArray, srcNumber < linterval[ 0 ]';
//   var got = _.linterval.inInclusiveLeft( [ 2, 5 ], _.argumentsArray.make( [ 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - argumentsArray, srcNumber = linterval[ 0 ]';
//   var got = _.linterval.inInclusiveLeft( [ 2, 5 ], _.argumentsArray.make( [ 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - argumentsArray, srcNumber > linterval[ 1 ], srcNumber > linterval[ 0 ] + linterval[ 1 ]';
//   var got = _.linterval.inInclusiveLeft( [ 2, 5 ], _.argumentsArray.make( [ 0, 0, 0, 0, 0, 0, 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - argumentsArray, srcNumber > linterval[ 1 ], srcNumber === linterval[ 0 ] + linterval[ 1 ]';
//   var got = _.linterval.inInclusiveLeft( [ 2, 5 ], _.argumentsArray.make( [ 0, 0, 0, 0, 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - argumentsArray, srcNumber = linterval[ 1 ]';
//   var got = _.linterval.inInclusiveLeft( [ 2, 5 ], _.argumentsArray.make( [ 0, 0, 0, 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - argumentsArray, linterval[ 0 ] < srcNumber < linterval[ 1 ]';
//   var got = _.linterval.inInclusiveLeft( [ 2, 5 ], _.argumentsArray.make( [ 0, 0, 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - BufferTyped, srcNumber < linterval[ 0 ]';
//   var got = _.linterval.inInclusiveLeft( [ 2, 5 ], new I8x( [ 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - BufferTyped, srcNumber = linterval[ 0 ]';
//   var got = _.linterval.inInclusiveLeft( [ 2, 5 ], new I8x( [ 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - BufferTyped, srcNumber > linterval[ 1 ], srcNumber > linterval[ 0 ] + linterval[ 1 ]';
//   var got = _.linterval.inInclusiveLeft( [ 2, 5 ], new U16x( [ 0, 0, 0, 0, 0, 0, 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - BufferTyped, srcNumber > linterval[ 1 ], srcNumber === linterval[ 0 ] + linterval[ 1 ]';
//   var got = _.linterval.inInclusiveLeft( [ 2, 5 ], new U16x( [ 0, 0, 0, 0, 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - BufferTyped, srcNumber = linterval[ 1 ]';
//   var got = _.linterval.inInclusiveLeft( [ 2, 5 ], new U16x( [ 0, 0, 0, 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - BufferTyped, linterval[ 0 ] < srcNumber < linterval[ 1 ]';
//   var got = _.linterval.inInclusiveLeft( [ 2, 5 ], new F32x( [ 0, 0, 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   /* - */
//
//   if( !Config.debug )
//   return;
//
//   test.case = 'without arguments';
//   test.shouldThrowErrorSync( () => _.linterval.inInclusiveLeft() );
//
//   test.case = 'not enough arguments';
//   test.shouldThrowErrorSync( () => _.linterval.inInclusiveLeft( [ 1, 2 ] ) );
//
//   test.case = 'extra arguments';
//   test.shouldThrowErrorSync( () => _.linterval.inInclusiveLeft( [ 1, 2 ], 3, 'extra' ) );
//
//   test.case = 'linterval is not linterval';
//   test.shouldThrowErrorSync( () => _.linterval.inInclusiveLeft( 'wrong', 3 ) );
//
//   test.case = 'srcNumber is not Long, not Number';
//   test.shouldThrowErrorSync( () => _.linterval.inInclusiveLeft( [ 1, 2 ], 'wrong' ) );
// }
//
// //
//
// function lintervalInInclusiveRight( test )
// {
//   test.case = 'srcNumber - number, srcNumber < linterval[ 0 ]';
//   var got = _.linterval.inInclusiveRight( [ 2, 5 ], 1 );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - number, srcNumber = linterval[ 0 ]';
//   var got = _.linterval.inInclusiveRight( [ 2, 5 ], 2 );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - number, srcNumber > linterval[ 1 ], srcNumber > linterval[ 0 ] + linterval[ 1 ]';
//   var got = _.linterval.inInclusiveRight( [ 2, 5 ], 7 );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - number, srcNumber > linterval[ 1 ], srcNumber === linterval[ 0 ] + linterval[ 1 ]';
//   var got = _.linterval.inInclusiveRight( [ 2, 5 ], 6 );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - number, srcNumber = linterval[ 1 ]';
//   var got = _.linterval.inInclusiveRight( [ 2, 5 ], 5 );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - number, linterval[ 0 ] < srcNumber < linterval[ 1 ]';
//   var got = _.linterval.inInclusiveRight( [ 2, 5 ], 4 );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - array, srcNumber < linterval[ 0 ]';
//   var got = _.linterval.inInclusiveRight( [ 2, 5 ], [ 0 ] );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - array, srcNumber = linterval[ 0 ]';
//   var got = _.linterval.inInclusiveRight( [ 2, 5 ], [ 0, 0 ] );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - array, srcNumber > linterval[ 1 ], srcNumber > linterval[ 0 ] + linterval[ 1 ]';
//   var got = _.linterval.inInclusiveRight( [ 2, 5 ], [ 0, 0, 0, 0, 0, 0, 0 ] );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - array, srcNumber > linterval[ 1 ], srcNumber === linterval[ 0 ] + linterval[ 1 ]';
//   var got = _.linterval.inInclusiveRight( [ 2, 5 ], [ 0, 0, 0, 0, 0, 0 ] );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - array, srcNumber = linterval[ 1 ]';
//   var got = _.linterval.inInclusiveRight( [ 2, 5 ], [ 0, 0, 0, 0, 0 ] );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - array, linterval[ 0 ] < srcNumber < linterval[ 1 ]';
//   var got = _.linterval.inInclusiveRight( [ 2, 5 ], [ 0, 0, 0, 0 ] );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - unroll, srcNumber < linterval[ 0 ]';
//   var got = _.linterval.inInclusiveRight( [ 2, 5 ], _.unroll.make( [ 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - unroll, srcNumber = linterval[ 0 ]';
//   var got = _.linterval.inInclusiveRight( [ 2, 5 ], _.unroll.make( [ 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - unroll, srcNumber > linterval[ 1 ], srcNumber > linterval[ 0 ] + linterval[ 1 ]';
//   var got = _.linterval.inInclusiveRight( [ 2, 5 ], _.unroll.make( [ 0, 0, 0, 0, 0, 0, 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - unroll, srcNumber > linterval[ 1 ], srcNumber === linterval[ 0 ] + linterval[ 1 ]';
//   var got = _.linterval.inInclusiveRight( [ 2, 5 ], _.unroll.make( [ 0, 0, 0, 0, 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - unroll, srcNumber = linterval[ 1 ]';
//   var got = _.linterval.inInclusiveRight( [ 2, 5 ], _.unroll.make( [ 0, 0, 0, 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - unroll, linterval[ 0 ] < srcNumber < linterval[ 1 ]';
//   var got = _.linterval.inInclusiveRight( [ 2, 5 ], _.unroll.make( [ 0, 0, 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - argumentsArray, srcNumber < linterval[ 0 ]';
//   var got = _.linterval.inInclusiveRight( [ 2, 5 ], _.argumentsArray.make( [ 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - argumentsArray, srcNumber = linterval[ 0 ]';
//   var got = _.linterval.inInclusiveRight( [ 2, 5 ], _.argumentsArray.make( [ 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - argumentsArray, srcNumber > linterval[ 1 ], srcNumber > linterval[ 0 ] + linterval[ 1 ]';
//   var got = _.linterval.inInclusiveRight( [ 2, 5 ], _.argumentsArray.make( [ 0, 0, 0, 0, 0, 0, 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - argumentsArray, srcNumber > linterval[ 1 ], srcNumber === linterval[ 0 ] + linterval[ 1 ]';
//   var got = _.linterval.inInclusiveRight( [ 2, 5 ], _.argumentsArray.make( [ 0, 0, 0, 0, 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - argumentsArray, srcNumber = linterval[ 1 ]';
//   var got = _.linterval.inInclusiveRight( [ 2, 5 ], _.argumentsArray.make( [ 0, 0, 0, 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - argumentsArray, linterval[ 0 ] < srcNumber < linterval[ 1 ]';
//   var got = _.linterval.inInclusiveRight( [ 2, 5 ], _.argumentsArray.make( [ 0, 0, 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - BufferTyped, srcNumber < linterval[ 0 ]';
//   var got = _.linterval.inInclusiveRight( [ 2, 5 ], new I8x( [ 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - BufferTyped, srcNumber = linterval[ 0 ]';
//   var got = _.linterval.inInclusiveRight( [ 2, 5 ], new I8x( [ 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - BufferTyped, srcNumber > linterval[ 1 ], srcNumber > linterval[ 0 ] + linterval[ 1 ]';
//   var got = _.linterval.inInclusiveRight( [ 2, 5 ], new U16x( [ 0, 0, 0, 0, 0, 0, 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - BufferTyped, srcNumber > linterval[ 1 ], srcNumber === linterval[ 0 ] + linterval[ 1 ]';
//   var got = _.linterval.inInclusiveRight( [ 2, 5 ], new U16x( [ 0, 0, 0, 0, 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - BufferTyped, srcNumber = linterval[ 1 ]';
//   var got = _.linterval.inInclusiveRight( [ 2, 5 ], new U16x( [ 0, 0, 0, 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - BufferTyped, linterval[ 0 ] < srcNumber < linterval[ 1 ]';
//   var got = _.linterval.inInclusiveRight( [ 2, 5 ], new F32x( [ 0, 0, 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   /* - */
//
//   if( !Config.debug )
//   return;
//
//   test.case = 'without arguments';
//   test.shouldThrowErrorSync( () => _.linterval.inInclusiveRight() );
//
//   test.case = 'not enough arguments';
//   test.shouldThrowErrorSync( () => _.linterval.inInclusiveRight( [ 1, 2 ] ) );
//
//   test.case = 'extra arguments';
//   test.shouldThrowErrorSync( () => _.linterval.inInclusiveRight( [ 1, 2 ], 3, 'extra' ) );
//
//   test.case = 'linterval is not linterval';
//   test.shouldThrowErrorSync( () => _.linterval.inInclusiveRight( 'wrong', 3 ) );
//
//   test.case = 'srcNumber is not Long, not Number';
//   test.shouldThrowErrorSync( () => _.linterval.inInclusiveRight( [ 1, 2 ], 'wrong' ) );
// }

//

function lintervalHas( test )
{
  test.open( 'src - number' );

  test.case = 'src < linterval[ 0 ]';
  var got = _.linterval.has( [ 2, 5 ], 1 );
  var expected = false;
  test.identical( got, expected );

  test.case = 'src === linterval[ 0 ]';
  var got = _.linterval.has( [ 2, 5 ], 2 );
  var expected = true;
  test.identical( got, expected );

  test.case = 'src > linterval[ 0 ] + linterval[ 1 ]';
  var got = _.linterval.has( [ 2, 5 ], 8 );
  var expected = false;
  test.identical( got, expected );

  test.case = 'src === linterval[ 0 ] + linterval[ 1 ]';
  var got = _.linterval.has( [ 2, 5 ], 7 );
  var expected = true;
  test.identical( got, expected );

  test.case = 'linterval[ 0 ] < src < linterval[ 0 ] + linterval[ 1 ]';
  var got = _.linterval.has( [ 2, 5 ], 4 );
  var expected = true;
  test.identical( got, expected );

  test.close( 'src - number' );

  /* - */

  test.open( 'src - interval' );

  test.case = 'src[ 0 ] < linterval[ 0 ]';
  var got = _.linterval.has( [ 2, 5 ], [ 1, 4 ] );
  var expected = false;
  test.identical( got, expected );

  test.case = 'src[ 0 ] === linterval[ 0 ]';
  var got = _.linterval.has( [ 2, 5 ], [ 2, 4 ] );
  var expected = true;
  test.identical( got, expected );

  test.case = 'src[ 1 ] > linterval[ 1 ]';
  var got = _.linterval.has( [ 2, 5 ], [ 2, 6 ] );
  var expected = false;
  test.identical( got, expected );

  test.case = 'src[ 1 ] === linterval[ 1 ]';
  var got = _.linterval.has( [ 2, 5 ], [ 2, 5 ] );
  var expected = true;
  test.identical( got, expected );

  test.case = 'linterval[ 0 ] < src[ 0 ] && src[ 1 ] < linterval[ 1 ]';
  var got = _.linterval.has( [ 2, 5 ], [ 3, 4 ] );
  var expected = true;
  test.identical( got, expected );

  test.close( 'src - interval' );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.linterval.has() );

  test.case = 'not enough arguments';
  test.shouldThrowErrorSync( () => _.linterval.has( [ 1, 2 ] ) );

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.linterval.has( [ 1, 2 ], 3, 'extra' ) );

  test.case = 'linterval is not linterval';
  test.shouldThrowErrorSync( () => _.linterval.has( 'wrong', 3 ) );

  test.case = 'src is not Interval, not Number';
  test.shouldThrowErrorSync( () => _.linterval.has( [ 1, 2 ], 'wrong' ) );
}

//

function lintervalSureInRange( test )
{
  test.case = 'two arguments, src - number, in linterval';
  var got = _.linterval.sureIn( 3, [ 1, 5 ] );
  var expected = true;
  test.identical( got, expected );

  test.case = 'two arguments, src - array, in linterval';
  var got = _.linterval.sureIn( [ 0, 0, 0 ], [ 1, 5 ] );
  var expected = true;
  test.identical( got, expected );

  test.case = 'more then two arguments, src - number, in linterval';
  var got = _.linterval.sureIn( 3, [ 1, 5 ], 'extra', [ 'next' ] );
  var expected = true;
  test.identical( got, expected );

  test.case = 'more then two arguments, src - array, in linterval';
  var got = _.linterval.sureIn( [ 0, 0, 0 ], [ 1, 5 ], 'extra', [ 'next' ] );
  var expected = true;
  test.identical( got, expected );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.linterval.sureIn() );

  test.case = 'not enough arguments';
  test.shouldThrowErrorSync( () => _.linterval.sureIn( 2 ) );

  test.case = 'src out of linterval';
  test.shouldThrowErrorSync( () => _.linterval.sureIn( 1, [ 2, 5 ] ) );
  test.shouldThrowErrorSync( () => _.linterval.sureIn( 8, [ 2, 5 ] ) );
  test.shouldThrowErrorSync( () => _.linterval.sureIn( 1, [ 2, 5 ], [] ) );
  test.shouldThrowErrorSync( () => _.linterval.sureIn( 8, [ 2, 5 ], undefined ) );
}

//

function lintervalAssertInRange( test )
{
  test.case = 'two arguments, src - number, in linterval';
  var got = _.linterval.assertIn( 3, [ 1, 5 ] );
  var expected = true;
  test.identical( got, expected );

  test.case = 'two arguments, src - array, in linterval';
  var got = _.linterval.assertIn( [ 0, 0, 0 ], [ 1, 5 ] );
  var expected = true;
  test.identical( got, expected );

  test.case = 'more then two arguments, src - number, in linterval';
  var got = _.linterval.assertIn( 3, [ 1, 5 ], 'extra', [ 'next' ] );
  var expected = true;
  test.identical( got, expected );

  test.case = 'more then two arguments, src - array, in linterval';
  var got = _.linterval.assertIn( [ 0, 0, 0 ], [ 1, 5 ], 'extra', [ 'next' ] );
  var expected = true;
  test.identical( got, expected );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.linterval.assertIn() );

  test.case = 'not enough arguments';
  test.shouldThrowErrorSync( () => _.linterval.assertIn( 2 ) );

  test.case = 'src out of linterval';
  test.shouldThrowErrorSync( () => _.linterval.assertIn( 1, [ 2, 5 ] ) );
  test.shouldThrowErrorSync( () => _.linterval.assertIn( 8, [ 2, 5 ] ) );
  test.shouldThrowErrorSync( () => _.linterval.assertIn( 1, [ 2, 5 ], [] ) );
  test.shouldThrowErrorSync( () => _.linterval.assertIn( 8, [ 2, 5 ], undefined ) );
}

//

function lintervalFromSingle( test )
{
  test.case = 'linterval - number';
  var got = _.linterval.fromSingle( 1 );
  test.identical( got, [ 1, 1 ] );

  /* */

  test.case = 'linterval.length === 1';
  var src = [ 1 ];
  var got = _.linterval.fromSingle( src );
  test.identical( got, [ 1, 1 ] );
  test.true( got !== src );

  test.case = 'linterval[ 0 ] - undefined';
  var src = [ undefined, 1 ];
  var got = _.linterval.fromSingle( src );
  test.identical( got, [ 0, 1 ] );
  test.true( got !== src );

  test.case = 'linterval[ 1 ] - undefined';
  var src = [ 1, undefined ];
  var got = _.linterval.fromSingle( src );
  test.identical( got, [ 1, 1 ] );
  test.true( got !== src );

  test.case = 'linterval[ 0 ] - undefined, linterval[ 1 ] - undefined';
  var src = [ undefined, undefined ];
  var got = _.linterval.fromSingle( src );
  test.identical( got, [ 0, 1 ] );
  test.true( got !== src );

  test.case = 'linterval[ 0 ] - number, linterval[ 1 ] - number';
  var src = [ 1, 5 ];
  var got = _.linterval.fromSingle( src );
  test.identical( got, [ 1, 5 ] );
  test.true( got === src );

  /* */

  test.case = 'linterval.length === 1';
  var src = _.unroll.make( [ 1 ] );
  var got = _.linterval.fromSingle( src );
  test.identical( got, [ 1, 1 ] );
  test.true( got !== src );

  test.case = 'linterval[ 0 ] - undefined';
  var src = _.unroll.make( [ undefined, 1 ] );
  var got = _.linterval.fromSingle( src );
  test.identical( got, [ 0, 1 ] );
  test.true( got !== src );

  test.case = 'linterval[ 1 ] - undefined';
  var src = _.unroll.make( [ 1, undefined ] );
  var got = _.linterval.fromSingle( src );
  test.identical( got, [ 1, 1 ] );
  test.true( got !== src );

  test.case = 'linterval[ 0 ] - undefined, linterval[ 1 ] - undefined';
  var src = _.unroll.make( [ undefined, undefined ] );
  var got = _.linterval.fromSingle( src );
  test.identical( got, [ 0, 1 ] );
  test.true( got !== src );

  test.case = 'linterval[ 0 ] - number, linterval[ 1 ] - number';
  var src = _.unroll.make( [ 1, 5 ] );
  var got = _.linterval.fromSingle( src );
  test.identical( got, _.unroll.make( [ 1, 5 ] ) );
  test.true( got === src );

  /* */

  test.case = 'linterval.length === 1';
  var src = _.argumentsArray.make( [ 1 ] );
  var got = _.linterval.fromSingle( src );
  test.identical( got, [ 1, 1 ] );
  test.true( got !== src );

  test.case = 'linterval[ 0 ] - undefined';
  var src = _.argumentsArray.make( [ undefined, 1 ] );
  var got = _.linterval.fromSingle( src );
  test.identical( got, [ 0, 1 ] );
  test.true( got !== src );

  test.case = 'linterval[ 1 ] - undefined';
  var src = _.argumentsArray.make( [ 1, undefined ] );
  var got = _.linterval.fromSingle( src );
  test.identical( got, [ 1, 1 ] );
  test.true( got !== src );

  test.case = 'linterval[ 0 ] - undefined, linterval[ 1 ] - undefined';
  var src = _.argumentsArray.make( [ undefined, undefined ] );
  var got = _.linterval.fromSingle( src );
  test.identical( got, [ 0, 1 ] );
  test.true( got !== src );

  test.case = 'linterval[ 0 ] - number, linterval[ 1 ] - number';
  var src = _.argumentsArray.make( [ 1, 5 ] );
  var got = _.linterval.fromSingle( src );
  test.identical( got, _.argumentsArray.make( [ 1, 5 ] ) );
  test.true( got === src );

  /* */

  test.case = 'linterval.length === 1';
  var src = new U8x( [ 1 ] );
  var got = _.linterval.fromSingle( src );
  test.identical( got, [ 1, 1 ] );
  test.true( got !== src );

  test.case = 'linterval[ 0 ] - number, linterval[ 1 ] - number';
  var src = new I16x( [ 1, 5 ] );
  var got = _.linterval.fromSingle( src );
  test.identical( got, new I16x( [ 1, 5 ] ) );
  test.true( got === src );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.linterval.fromSingle() );

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.linterval.fromSingle( [ 1, 2 ], 1 ) );

  test.case = 'wrong type of linterval';
  test.shouldThrowErrorSync( () => _.linterval.fromSingle( { 0 : 1, 1 : 2 } ) );

  test.case = 'wrong linterval length';
  test.shouldThrowErrorSync( () => _.linterval.fromSingle( [ 0, 1, 2 ] ) );

  test.case = 'wrong elements in linterval';
  test.shouldThrowErrorSync( () => _.linterval.fromSingle( [ null, 2 ] ) );
  test.shouldThrowErrorSync( () => _.linterval.fromSingle( [ 2, 'abc' ] ) );
}

//

function lintervalClamp( test )
{
  test.open( 'array' );

  test.case = 'identical ranges';
  var dst = [ 2, 2 ];
  var clamp = [ 2, 2 ];
  var got = _.linterval.clamp( dst, clamp );
  test.identical( got, [ 2, 2 ] );
  test.true( got === dst );

  test.case = 'dst[ 0 ] < clamp[ 0 ], dst[ 1 ] < clamp[ 0 ]';
  var dst = [ 2, 2 ];
  var clamp = [ 3, 6 ];
  var got = _.linterval.clamp( dst, clamp );
  test.identical( got, [ 2, 2 ] );
  test.true( got === dst );

  test.case = 'dst[ 0 ] < clamp[ 0 ], clamp[ 1 ] > dst[ 1 ] > clamp[ 0 ]';
  var dst = [ 2, 4 ];
  var clamp = [ 3, 6 ];
  var got = _.linterval.clamp( dst, clamp );
  test.identical( got, [ 2, 4 ] );
  test.true( got === dst );

  test.case = 'clamp[ 1 ] > dst[ 0 ] > clamp[ 0 ], dst[ 1 ] < clamp[ 0 ]';
  var dst = [ 4, 2 ];
  var clamp = [ 3, 6 ];
  var got = _.linterval.clamp( dst, clamp );
  test.identical( got, [ 3, 2 ] );
  test.true( got === dst );

  test.case = 'clamp[ 1 ] > dst[ 0 ] > clamp[ 0 ], clamp[ 1 ] > dst[ 1 ] > clamp[ 0 ]';
  var dst = [ 4, 5 ];
  var clamp = [ 3, 6 ];
  var got = _.linterval.clamp( dst, clamp );
  test.identical( got, [ 3, 5 ] );
  test.true( got === dst );

  test.case = 'dst[ 0 ] > clamp[ 1 ], clamp[ 1 ] > dst[ 1 ] > clamp[ 0 ]';
  var dst = [ 7, 5 ];
  var clamp = [ 3, 6 ];
  var got = _.linterval.clamp( dst, clamp );
  test.identical( got, [ 3, 5 ] );
  test.true( got === dst );

  test.case = 'clamp[ 1 ] > dst[ 0 ] > clamp[ 0 ], dst[ 1 ] > clamp[ 1 ]';
  var dst = [ 4, 7 ];
  var clamp = [ 3, 6 ];
  var got = _.linterval.clamp( dst, clamp );
  test.identical( got, [ 3, 6 ] );
  test.true( got === dst );

  test.case = 'dst[ 0 ] > clamp[ 1 ], dst[ 1 ] > clamp[ 1 ]';
  var dst = [ 8, 7 ];
  var clamp = [ 3, 6 ];
  var got = _.linterval.clamp( dst, clamp );
  test.identical( got, [ 3, 6 ] );
  test.true( got === dst );

  test.case = 'dst[ 0 ] < clamp[ 0 ], dst[ 1 ] > clamp[ 1 ]';
  var dst = [ 2, 7 ];
  var clamp = [ 3, 6 ];
  var got = _.linterval.clamp( dst, clamp );
  test.identical( got, [ 2, 6 ] );
  test.true( got === dst );

  test.close( 'array' );

  /* - */

  test.open( 'unroll' );

  test.case = 'identical ranges';
  var dst = _.unroll.make( [ 2, 2 ] );
  var clamp = [ 2, 2 ];
  var got = _.linterval.clamp( dst, clamp );
  test.identical( got, _.unroll.make( [ 2, 2 ] ) );
  test.true( got === dst );

  test.case = 'dst[ 0 ] < clamp[ 0 ], dst[ 1 ] < clamp[ 0 ]';
  var dst = _.unroll.make( [ 2, 2 ] );
  var clamp = [ 3, 6 ];
  var got = _.linterval.clamp( dst, clamp );
  test.identical( got, _.unroll.make( [ 2, 2 ] ) );
  test.true( got === dst );

  test.case = 'dst[ 0 ] < clamp[ 0 ], clamp[ 1 ] > dst[ 1 ] > clamp[ 0 ]';
  var dst = _.unroll.make( [ 2, 4 ] );
  var clamp = [ 3, 6 ];
  var got = _.linterval.clamp( dst, clamp );
  test.identical( got, _.unroll.make( [ 2, 4 ] ) );
  test.true( got === dst );

  test.case = 'clamp[ 1 ] > dst[ 0 ] > clamp[ 0 ], dst[ 1 ] < clamp[ 0 ]';
  var dst = _.unroll.make( [ 4, 2 ] );
  var clamp = [ 3, 6 ];
  var got = _.linterval.clamp( dst, clamp );
  test.identical( got, _.unroll.make( [ 3, 2 ] ) );
  test.true( got === dst );

  test.case = 'clamp[ 1 ] > dst[ 0 ] > clamp[ 0 ], clamp[ 1 ] > dst[ 1 ] > clamp[ 0 ]';
  var dst = _.unroll.make( [ 4, 5 ] );
  var clamp = [ 3, 6 ];
  var got = _.linterval.clamp( dst, clamp );
  test.identical( got, _.unroll.make( [ 3, 5 ] ) );
  test.true( got === dst );

  test.case = 'dst[ 0 ] > clamp[ 1 ], clamp[ 1 ] > dst[ 1 ] > clamp[ 0 ]';
  var dst = _.unroll.make( [ 7, 5 ] );
  var clamp = [ 3, 6 ];
  var got = _.linterval.clamp( dst, clamp );
  test.identical( got, _.unroll.make( [ 3, 5 ] ) );
  test.true( got === dst );

  test.case = 'clamp[ 1 ] > dst[ 0 ] > clamp[ 0 ], dst[ 1 ] > clamp[ 1 ]';
  var dst = _.unroll.make( [ 4, 7 ] );
  var clamp = [ 3, 6 ];
  var got = _.linterval.clamp( dst, clamp );
  test.identical( got, _.unroll.make( [ 3, 6 ] ) );
  test.true( got === dst );

  test.case = 'dst[ 0 ] > clamp[ 1 ], dst[ 1 ] > clamp[ 1 ]';
  var dst = _.unroll.make( [ 8, 7 ] );
  var clamp = [ 3, 6 ];
  var got = _.linterval.clamp( dst, clamp );
  test.identical( got, _.unroll.make( [ 3, 6 ] ) );
  test.true( got === dst );

  test.case = 'dst[ 0 ] < clamp[ 0 ], dst[ 1 ] > clamp[ 1 ]';
  var dst = _.unroll.make( [ 2, 7 ] );
  var clamp = [ 3, 6 ];
  var got = _.linterval.clamp( dst, clamp );
  test.identical( got, _.unroll.make( [ 2, 6 ] ) );
  test.true( got === dst );

  test.close( 'unroll' );

  /* - */

  test.open( 'argumentsArray' );

  test.case = 'identical ranges';
  var dst = _.argumentsArray.make( [ 2, 2 ] );
  var clamp = [ 2, 2 ];
  var got = _.linterval.clamp( dst, clamp );
  test.identical( got, _.argumentsArray.make( [ 2, 2 ] ) );
  test.true( got === dst );

  test.case = 'dst[ 0 ] < clamp[ 0 ], dst[ 1 ] < clamp[ 0 ]';
  var dst = _.argumentsArray.make( [ 2, 2 ] );
  var clamp = [ 3, 6 ];
  var got = _.linterval.clamp( dst, clamp );
  test.identical( got, _.argumentsArray.make( [ 2, 2 ] ) );
  test.true( got === dst );

  test.case = 'dst[ 0 ] < clamp[ 0 ], clamp[ 1 ] > dst[ 1 ] > clamp[ 0 ]';
  var dst = _.argumentsArray.make( [ 2, 4 ] );
  var clamp = [ 3, 6 ];
  var got = _.linterval.clamp( dst, clamp );
  test.identical( got, _.argumentsArray.make( [ 2, 4 ] ) );
  test.true( got === dst );

  test.case = 'clamp[ 1 ] > dst[ 0 ] > clamp[ 0 ], dst[ 1 ] < clamp[ 0 ]';
  var dst = _.argumentsArray.make( [ 4, 2 ] );
  var clamp = [ 3, 6 ];
  var got = _.linterval.clamp( dst, clamp );
  test.identical( got, _.argumentsArray.make( [ 3, 2 ] ) );
  test.true( got === dst );

  test.case = 'clamp[ 1 ] > dst[ 0 ] > clamp[ 0 ], clamp[ 1 ] > dst[ 1 ] > clamp[ 0 ]';
  var dst = _.argumentsArray.make( [ 4, 5 ] );
  var clamp = [ 3, 6 ];
  var got = _.linterval.clamp( dst, clamp );
  test.identical( got, _.argumentsArray.make( [ 3, 5 ] ) );
  test.true( got === dst );

  test.case = 'dst[ 0 ] > clamp[ 1 ], clamp[ 1 ] > dst[ 1 ] > clamp[ 0 ]';
  var dst = _.argumentsArray.make( [ 7, 5 ] );
  var clamp = [ 3, 6 ];
  var got = _.linterval.clamp( dst, clamp );
  test.identical( got, _.argumentsArray.make( [ 3, 5 ] ) );
  test.true( got === dst );

  test.case = 'clamp[ 1 ] > dst[ 0 ] > clamp[ 0 ], dst[ 1 ] > clamp[ 1 ]';
  var dst = _.argumentsArray.make( [ 4, 7 ] );
  var clamp = [ 3, 6 ];
  var got = _.linterval.clamp( dst, clamp );
  test.identical( got, _.argumentsArray.make( [ 3, 6 ] ) );
  test.true( got === dst );

  test.case = 'dst[ 0 ] > clamp[ 1 ], dst[ 1 ] > clamp[ 1 ]';
  var dst = _.argumentsArray.make( [ 8, 7 ] );
  var clamp = [ 3, 6 ];
  var got = _.linterval.clamp( dst, clamp );
  test.identical( got, _.argumentsArray.make( [ 3, 6 ] ) );
  test.true( got === dst );

  test.case = 'dst[ 0 ] < clamp[ 0 ], dst[ 1 ] > clamp[ 1 ]';
  var dst = _.argumentsArray.make( [ 2, 7 ] );
  var clamp = [ 3, 6 ];
  var got = _.linterval.clamp( dst, clamp );
  test.identical( got, _.argumentsArray.make( [ 2, 6 ] ) );
  test.true( got === dst );

  test.close( 'argumentsArray' );

  /* - */

  test.open( 'bufferTyped' );

  test.case = 'identical ranges';
  var dst = new U8x( [ 2, 2 ] );
  var clamp = [ 2, 2 ];
  var got = _.linterval.clamp( dst, clamp );
  test.identical( got, new U8x( [ 2, 2 ] ) );
  test.true( got === dst );

  test.case = 'dst[ 0 ] < clamp[ 0 ], dst[ 1 ] < clamp[ 0 ]';
  var dst = new I8x( [ 2, 2 ] );
  var clamp = [ 3, 6 ];
  var got = _.linterval.clamp( dst, clamp );
  test.identical( got, new I8x( [ 2, 2 ] ) );
  test.true( got === dst );

  test.case = 'dst[ 0 ] < clamp[ 0 ], clamp[ 1 ] > dst[ 1 ] > clamp[ 0 ]';
  var dst = new U16x( [ 2, 4 ] );
  var clamp = [ 3, 6 ];
  var got = _.linterval.clamp( dst, clamp );
  test.identical( got, new U16x( [ 2, 4 ] ) );
  test.true( got === dst );

  test.case = 'clamp[ 1 ] > dst[ 0 ] > clamp[ 0 ], dst[ 1 ] < clamp[ 0 ]';
  var dst = new I16x( [ 4, 2 ] );
  var clamp = [ 3, 6 ];
  var got = _.linterval.clamp( dst, clamp );
  test.identical( got, new I16x( [ 3, 2 ] ) );
  test.true( got === dst );

  test.case = 'clamp[ 1 ] > dst[ 0 ] > clamp[ 0 ], clamp[ 1 ] > dst[ 1 ] > clamp[ 0 ]';
  var dst = new U32x( [ 4, 5 ] );
  var clamp = [ 3, 6 ];
  var got = _.linterval.clamp( dst, clamp );
  test.identical( got, new U32x( [ 3, 5 ] ) );
  test.true( got === dst );

  test.case = 'dst[ 0 ] > clamp[ 1 ], clamp[ 1 ] > dst[ 1 ] > clamp[ 0 ]';
  var dst = new I32x( [ 7, 5 ] );
  var clamp = [ 3, 6 ];
  var got = _.linterval.clamp( dst, clamp );
  test.identical( got, new I32x( [ 3, 5 ] ) );
  test.true( got === dst );

  test.case = 'clamp[ 1 ] > dst[ 0 ] > clamp[ 0 ], dst[ 1 ] > clamp[ 1 ]';
  var dst = new F32x( [ 4, 7 ] );
  var clamp = [ 3, 6 ];
  var got = _.linterval.clamp( dst, clamp );
  test.identical( got, new F32x( [ 3, 6 ] ) );
  test.true( got === dst );

  test.case = 'dst[ 0 ] > clamp[ 1 ], dst[ 1 ] > clamp[ 1 ]';
  var dst = new F64x( [ 8, 7 ] );
  var clamp = [ 3, 6 ];
  var got = _.linterval.clamp( dst, clamp );
  test.identical( got, new F64x( [ 3, 6 ] ) );
  test.true( got === dst );

  test.case = 'dst[ 0 ] < clamp[ 0 ], dst[ 1 ] > clamp[ 1 ]';
  var dst = new U8x( [ 2, 7 ] );
  var clamp = [ 3, 6 ];
  var got = _.linterval.clamp( dst, clamp );
  test.identical( got, new U8x( [ 2, 6 ] ) );
  test.true( got === dst );

  test.close( 'bufferTyped' );

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.linterval.clamp() );

  test.case = 'not enough arguments';
  test.shouldThrowErrorSync( () => _.linterval.clamp( [ 1, 2 ] ) );

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.linterval.clamp( [ 1, 2 ], [ 3, 4 ], [ 5, 6 ] ) );

  test.case = 'dstRange is not a linterval';
  test.shouldThrowErrorSync( () => _.linterval.clamp( [ 1, 2, 3 ], [ 3, 4 ] ) );
  test.shouldThrowErrorSync( () => _.linterval.clamp( [ 1, 'wrong' ], [ 3, 4 ] ) );
  test.shouldThrowErrorSync( () => _.linterval.clamp( [ undefined, 1 ], [ 3, 4 ] ) );

  test.case = 'clampRange is not a linterval';
  test.shouldThrowErrorSync( () => _.linterval.clamp( [ 3, 4 ], [ 1, 2, 3 ] ) );
  test.shouldThrowErrorSync( () => _.linterval.clamp( [ 3, 4 ], [ 1, 'wrong' ] ) );
  test.shouldThrowErrorSync( () => _.linterval.clamp( [ 3, 4 ], [ undefined, 1 ] ) );
}

//

function lintervalCountElements( test )
{
  test.case = 'increment - zero, positive substruction result';
  var got = _.linterval.countElements( [ 1, 6 ], 0 );
  test.identical( got, 0 );

  test.case = 'increment - zero, negative substruction result';
  var got = _.linterval.countElements( [ 6, 1 ], 0 );
  test.identical( got, 0 );

  test.case = 'increment - zero, zero substruction result';
  var got = _.linterval.countElements( [ 5, 5 ], 0 );
  test.identical( got, 0 );

  /* */

  test.case = 'increment - 1, positive substruction result';
  var got = _.linterval.countElements( [ 1, 6 ], 1 );
  test.identical( got, 6 );

  test.case = 'increment - 1, negative substruction result';
  var got = _.linterval.countElements( [ 6, 1 ], 1 );
  test.identical( got, 1 );

  test.case = 'increment - 1, zero substruction result';
  var got = _.linterval.countElements( [ 5, 5 ], 1 );
  test.identical( got, 5 );

  /* */

  test.case = 'increment - undefined, positive substruction result';
  var got = _.linterval.countElements( [ 1, 6 ], undefined );
  test.identical( got, 6 );

  test.case = 'increment - undefined, negative substruction result';
  var got = _.linterval.countElements( [ 6, 1 ], undefined );
  test.identical( got, 1 );

  test.case = 'increment - undefined, zero substruction result';
  var got = _.linterval.countElements( [ 5, 5 ], undefined );
  test.identical( got, 5 );

  /* */

  test.case = 'increment - not passed, positive substruction result';
  var got = _.linterval.countElements( [ 1, 6 ] );
  test.identical( got, 6 );

  test.case = 'increment - not passed, negative substruction result';
  var got = _.linterval.countElements( [ 6, 1 ] );
  test.identical( got, 1 );

  test.case = 'increment - not passed, zero substruction result';
  var got = _.linterval.countElements( [ 5, 5 ] );
  test.identical( got, 5 );

  /* */

  test.case = 'increment - -1, positive substruction result';
  var got = _.linterval.countElements( [ 1, 6 ], -1 );
  test.identical( got, -6 );

  test.case = 'increment - -1, negative substruction result';
  var got = _.linterval.countElements( [ 6, 1 ], -1 );
  test.identical( got, -1 );

  test.case = 'increment - -1, zero substruction result';
  var got = _.linterval.countElements( [ 5, 5 ], -1 );
  test.identical( got, -5 );

  /* */

  test.case = 'increment - 5, positive substruction result';
  var got = _.linterval.countElements( [ 1, 6 ], 5 );
  test.identical( got, 1 );

  test.case = 'increment - 5, negative substruction result';
  var got = _.linterval.countElements( [ 6, 1 ], 5 );
  test.identical( got, 1 );

  test.case = 'increment - 5, zero substruction result';
  var got = _.linterval.countElements( [ 5, 5 ], 5 );
  test.identical( got, 1 );

  /* */

  test.case = 'increment - -5, positive substruction result';
  var got = _.linterval.countElements( [ 1, 6 ], -5 );
  test.identical( got, -1 );

  test.case = 'increment - -5, negative substruction result';
  var got = _.linterval.countElements( [ 6, 1 ], -5 );
  test.identical( got, -1 );

  test.case = 'increment - 5, zero substruction result';
  var got = _.linterval.countElements( [ 5, 5 ], -5 );
  test.identical( got, -1 );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.linterval.countElements() );

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.linterval.countElements( [ 1, 2 ], 1, 'extra' ) );

  test.case = 'linterval is not a linterval';
  test.shouldThrowErrorSync( () => _.linterval.countElements( [ 1, 2, 3 ], [ 3, 4 ] ) );
  test.shouldThrowErrorSync( () => _.linterval.countElements( [ 1, 'wrong' ], [ 3, 4 ] ) );
  test.shouldThrowErrorSync( () => _.linterval.countElements( [ undefined, 1 ], [ 3, 4 ] ) );

  test.case = 'increment is not a number and not undefined';
  test.shouldThrowErrorSync( () => _.linterval.countElements( [ 1, 2 ], 'wrong' ) );
}

//

function lintervalLastGet( test )
{
  test.open( 'linterval - array' );

  test.case = 'without options';
  var got = _.linterval.lastGet( [ 2, 3 ] );
  test.identical( got, 4 );

  test.case = 'options - undefined';
  var options = undefined;
  var got = _.linterval.lastGet( [ 2, 3 ], options );
  test.identical( got, 4 );
  test.identical( options, undefined );

  test.case = 'options - empty map';
  var options = {};
  var got = _.linterval.lastGet( [ 2, 3 ], options );
  test.identical( got, 4 );
  test.identical( options, { increment : 1 } );

  test.case = 'options - map with own field increment';
  var options = { increment : 2 };
  var got = _.linterval.lastGet( [ 2, 3 ], options );
  test.identical( got, 4 );
  test.identical( options, { increment : 2 } );

  test.case = 'options - map with own field increment - undefined';
  var options = { increment : undefined };
  var got = _.linterval.lastGet( [ 2, 3 ], options );
  test.identical( got, 4 );
  test.identical( options, { increment : 1 } );

  test.close( 'linterval - array' );

  /* - */

  test.open( 'linterval - unroll' );

  test.case = 'without options';
  var got = _.linterval.lastGet( _.unroll.make( [ 2, 3 ] ) );
  test.identical( got, 4 );

  test.case = 'options - undefined';
  var options = undefined;
  var got = _.linterval.lastGet( _.unroll.make( [ 2, 3 ] ), options );
  test.identical( got, 4 );
  test.identical( options, undefined );

  test.case = 'options - empty map';
  var options = {};
  var got = _.linterval.lastGet( _.unroll.make( [ 2, 3 ] ), options );
  test.identical( got, 4 );
  test.identical( options, { increment : 1 } );

  test.case = 'options - map with own field increment';
  var options = { increment : 2 };
  var got = _.linterval.lastGet( _.unroll.make( [ 2, 3 ] ), options );
  test.identical( got, 4 );
  test.identical( options, { increment : 2 } );

  test.case = 'options - map with own field increment - undefined';
  var options = { increment : undefined };
  var got = _.linterval.lastGet( _.unroll.make( [ 2, 3 ] ), options );
  test.identical( got, 4 );
  test.identical( options, { increment : 1 } );

  test.close( 'linterval - unroll' );

  /* - */

  test.open( 'linterval - argumentsArray' );

  test.case = 'without options';
  var got = _.linterval.lastGet( _.argumentsArray.make( [ 2, 3 ] ) );
  test.identical( got, 4 );

  test.case = 'options - undefined';
  var options = undefined;
  var got = _.linterval.lastGet( _.argumentsArray.make( [ 2, 3 ] ), options );
  test.identical( got, 4 );
  test.identical( options, undefined );

  test.case = 'options - empty map';
  var options = {};
  var got = _.linterval.lastGet( _.argumentsArray.make( [ 2, 3 ] ), options );
  test.identical( got, 4 );
  test.identical( options, { increment : 1 } );

  test.case = 'options - map with own field increment';
  var options = { increment : 2 };
  var got = _.linterval.lastGet( _.argumentsArray.make( [ 2, 3 ] ), options );
  test.identical( got, 4 );
  test.identical( options, { increment : 2 } );

  test.case = 'options - map with own field increment - undefined';
  var options = { increment : undefined };
  var got = _.linterval.lastGet( _.argumentsArray.make( [ 2, 3 ] ), options );
  test.identical( got, 4 );
  test.identical( options, { increment : 1 } );

  test.close( 'linterval - argumentsArray' );

  /* - */

  test.open( 'linterval - BufferTyped' );

  test.case = 'without options';
  var got = _.linterval.lastGet( new U8x( [ 2, 3 ] ) );
  test.identical( got, 4 );

  test.case = 'options - undefined';
  var options = undefined;
  var got = _.linterval.lastGet( new I16x( [ 2, 3 ] ), options );
  test.identical( got, 4 );
  test.identical( options, undefined );

  test.case = 'options - empty map';
  var options = {};
  var got = _.linterval.lastGet( new F32x( [ 2, 3 ] ), options );
  test.identical( got, 4 );
  test.identical( options, { increment : 1 } );

  test.case = 'options - map with own field increment';
  var options = { increment : 2 };
  var got = _.linterval.lastGet( new F64x( [ 2, 3 ] ), options );
  test.identical( got, 4 );
  test.identical( options, { increment : 2 } );

  test.case = 'options - map with own field increment - undefined';
  var options = { increment : undefined };
  var got = _.linterval.lastGet( new I8x( [ 2, 3 ] ), options );
  test.identical( got, 4 );
  test.identical( options, { increment : 1 } );

  test.close( 'linterval - BufferTyped' );

  /* - */

  test.open( 'linterval - map' );

  test.case = 'without options, last - undefined';
  var got = _.linterval.lastGet( { first : undefined, last : undefined } );
  test.identical( got, undefined );

  test.case = 'without options, last - not exists';
  var got = _.linterval.lastGet( { first : undefined } );
  test.identical( got, undefined );

  test.case = 'without options, last - number';
  var got = _.linterval.lastGet( { first : 1, last : 2 } );
  test.identical( got, 2 );

  /* */

  test.case = 'options - undefined, last - undefined';
  var options = undefined;
  var got = _.linterval.lastGet( { first : undefined, last : undefined }, options );
  test.identical( got, undefined );
  test.identical( options, undefined );

  test.case = 'options - undefined, last - not exists';
  var options = undefined;
  var got = _.linterval.lastGet( { first : undefined }, options );
  test.identical( got, undefined );
  test.identical( options, undefined );

  test.case = 'options - undefined, last - number';
  var options = undefined;
  var got = _.linterval.lastGet( { first : 1, last : 2 }, options );
  test.identical( got, 2 );
  test.identical( options, undefined );

  /* */

  test.case = 'options - empty map, last - undefined';
  var options = {};
  var got = _.linterval.lastGet( { first : undefined, last : undefined }, options );
  test.identical( got, undefined );
  test.identical( options, { increment : 1 } );

  test.case = 'options - empty map, last - not exists';
  var options = {};
  var got = _.linterval.lastGet( { first : undefined }, options );
  test.identical( got, undefined );
  test.identical( options, { increment : 1 } );

  test.case = 'options - empty map, last - number';
  var options = {};
  var got = _.linterval.lastGet( { first : 1, last : 2 }, options );
  test.identical( got, 2 );
  test.identical( options, { increment : 1 } );

  /* */

  test.case = 'options - map with own field increment, last - undefined';
  var options = { increment : 2 };
  var got = _.linterval.lastGet( { first : undefined, last : undefined }, options );
  test.identical( got, undefined );
  test.identical( options, { increment : 2 } );

  test.case = 'options - map with own field increment, last - not exists';
  var options = { increment : 2 };
  var got = _.linterval.lastGet( { first : undefined }, options );
  test.identical( got, undefined );
  test.identical( options, { increment : 2 } );

  test.case = 'options - map with own field increment, last - number';
  var options = { increment : 2 };
  var got = _.linterval.lastGet( { first : 1, last : 2 }, options );
  test.identical( got, 2 );
  test.identical( options, { increment : 2 } );

  /* */

  test.case = 'options - map with own field increment - undefined, last - undefined';
  var options = { increment : undefined };
  var got = _.linterval.lastGet( { first : undefined, last : undefined }, options );
  test.identical( got, undefined );
  test.identical( options, { increment : 1 } );

  test.case = 'options - map with own field increment - undefined, last - not exists';
  var options = { increment : undefined };
  var got = _.linterval.lastGet( { first : undefined }, options );
  test.identical( got, undefined );
  test.identical( options, { increment : 1 } );

  test.case = 'options - map with own field increment - undefined, last - number';
  var options = { increment : undefined };
  var got = _.linterval.lastGet( { first : 1, last : 2 }, options );
  test.identical( got, 2 );
  test.identical( options, { increment : 1 } );

  test.close( 'linterval - map' );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.linterval.lastGet() );

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.linterval.lastGet( [ 2, 3 ], {}, 'extra' ) );

  test.case = 'linterval is a long, but is not a linterval';
  test.shouldThrowErrorSync( () => _.linterval.lastGet( [ 1, 2, 3 ] ) );
  test.shouldThrowErrorSync( () => _.linterval.lastGet( [ 1, 'wrong' ] ) );
  test.shouldThrowErrorSync( () => _.linterval.lastGet( [ undefined, 1 ] ) );

  test.case = 'wrong type of linterval';
  test.shouldThrowErrorSync( () => _.linterval.lastGet( new Set( [ 1, 2 ] ) ) );
  test.shouldThrowErrorSync( () => _.linterval.lastGet( new HashMap( [ [ 1, 2 ] ] ) ) );
}

// --
// ointerval
// --

function ointervalIsEmpty( test )
{
  test.case = 'undefined';
  var got = _.ointerval.isEmpty( undefined );
  var expected = false;
  test.identical( got, expected );

  test.case = 'null';
  var got = _.ointerval.isEmpty( null );
  var expected = false;
  test.identical( got, expected );

  test.case = 'false';
  var got = _.ointerval.isEmpty( false );
  var expected = false;
  test.identical( got, expected );

  test.case = 'empty string';
  var got = _.ointerval.isEmpty( '' );
  var expected = false;
  test.identical( got, expected );

  test.case = 'zero';
  var got = _.ointerval.isEmpty( 0 );
  var expected = false;
  test.identical( got, expected );

  test.case = 'NaN';
  var got = _.ointerval.isEmpty( NaN );
  var expected = false;
  test.identical( got, expected );

  test.case = 'a boolean';
  var got = _.ointerval.isEmpty( true );
  var expected = false;
  test.identical( got, expected );

  test.case = 'a number';
  var got = _.ointerval.isEmpty( 13 );
  var expected = false;
  test.identical( got, expected );

  test.case = 'a function';
  var got = _.ointerval.isEmpty( function() {} );
  var expected = false;
  test.identical( got, expected );

  test.case = 'constructor';
  function Constr( x )
  {
    this.x = x;
    return this;
  }
  var got = _.ointerval.isEmpty( new Constr( 0 ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'a string';
  var got = _.ointerval.isEmpty( 'str' );
  var expected = false;
  test.identical( got, expected );

  test.case = 'BufferRaw';
  var got = _.ointerval.isEmpty( new BufferRaw( 5 ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'BufferView';
  var got = _.ointerval.isEmpty( new BufferView( new BufferRaw( 5 ) ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'Set';
  var got = _.ointerval.isEmpty( new Set( [ 5 ] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'Map';
  var got = _.ointerval.isEmpty( new HashMap( [ [ 1, 2 ] ] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'pure empty map';
  var got = _.ointerval.isEmpty( Object.create( null ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'pure map';
  var src = Object.create( null );
  src.x = 1;
  var got = _.ointerval.isEmpty( src );
  var expected = false;
  test.identical( got, expected );

  test.case = 'map from pure map';
  var src = Object.create( Object.create( null ) );
  var got = _.ointerval.isEmpty( src );
  var expected = false;
  test.identical( got, expected );

  test.case = 'an empty object';
  var got = _.ointerval.isEmpty( {} );
  var expected = false;
  test.identical( got, expected );

  test.case = 'an object';
  var got = _.ointerval.isEmpty( { a : 7, b : 13 } );
  var expected = false;
  test.identical( got, expected );

  /* */

  test.case = 'array.length = 0';
  var got = _.ointerval.isEmpty( [] );
  var expected = false;
  test.identical( got, expected );

  test.case = 'array.length = 1';
  var got = _.ointerval.isEmpty( [ 1 ] );
  var expected = false;
  test.identical( got, expected );

  test.case = 'array.length = 2, different numbers';
  var got = _.ointerval.isEmpty( [ 1, 2 ] );
  var expected = false;
  test.identical( got, expected );

  test.case = 'array.length = 2, equal numbers';
  var got = _.ointerval.isEmpty( [ 1, 1 ] );
  var expected = true;
  test.identical( got, expected );

  test.case = 'array.length = 2, number and undefined';
  var got = _.ointerval.isEmpty( [ 1, undefined ] );
  var expected = false;
  test.identical( got, expected );

  test.case = 'array.length > 2';
  var got = _.ointerval.isEmpty( [ 1, 2, 3 ] );
  var expected = false;
  test.identical( got, expected );

  test.case = 'unroll.length = 0';
  var got = _.ointerval.isEmpty( _.unroll.make( [] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'unroll.length = 1';
  var got = _.ointerval.isEmpty( _.unroll.make( [ 1 ] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'unroll.length = 2, different numbers';
  var got = _.ointerval.isEmpty( _.unroll.make( [ 1, 2 ] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'unroll.length = 2, equal numbers';
  var got = _.ointerval.isEmpty( _.unroll.make( [ 10, 10 ] ) );
  var expected = true;
  test.identical( got, expected );

  test.case = 'unroll.length = 2, number and undefined';
  var got = _.ointerval.isEmpty( _.unroll.make( [ 1, undefined ] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'unroll.length > 2';
  var got = _.ointerval.isEmpty( _.unroll.make( [ 1, 2, 3 ] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'argumentsArray.length = 0';
  var got = _.ointerval.isEmpty( _.argumentsArray.make( [] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'argumentsArray.length = 1';
  var got = _.ointerval.isEmpty( _.argumentsArray.make( [ 1 ] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'argumentsArray.length = 2, different numbers';
  var got = _.ointerval.isEmpty( _.argumentsArray.make( [ 1, 2 ] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'argumentsArray.length = 2, equal numbers';
  var got = _.ointerval.isEmpty( _.argumentsArray.make( [ -2, -2 ] ) );
  var expected = true;
  test.identical( got, expected );

  test.case = 'argumentsArray.length = 2, number and undefined';
  var got = _.ointerval.isEmpty( _.argumentsArray.make( [ 1, undefined ] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'argumentsArray.length > 2';
  var got = _.ointerval.isEmpty( _.argumentsArray.make( [ 1, 2, 3 ] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'BufferTyped.length = 0';
  var got = _.ointerval.isEmpty( new U8x() );
  var expected = false;
  test.identical( got, expected );

  test.case = 'BufferTyped.length = 1';
  var got = _.ointerval.isEmpty( new I16x( 1 ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'BufferTyped.length = 2, different numbers';
  var got = _.ointerval.isEmpty( new F32x( [ 1, 3 ] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'BufferTyped.length = 2, equal numbers';
  var got = _.ointerval.isEmpty( new F32x( [ 1, 1 ] ) );
  var expected = true;
  test.identical( got, expected );

  test.case = 'BufferTyped.length > 2';
  var got = _.ointerval.isEmpty( new F32x( 4 ) );
  var expected = false;
  test.identical( got, expected );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.ointerval.isEmpty() );

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.ointerval.isEmpty( [ 1, 2 ], 'extra' ) );
}

//

function ointervalIsPopulated( test )
{
  test.case = 'undefined';
  var got = _.ointerval.isPopulated( undefined );
  var expected = false;
  test.identical( got, expected );

  test.case = 'null';
  var got = _.ointerval.isPopulated( null );
  var expected = false;
  test.identical( got, expected );

  test.case = 'false';
  var got = _.ointerval.isPopulated( false );
  var expected = false;
  test.identical( got, expected );

  test.case = 'empty string';
  var got = _.ointerval.isPopulated( '' );
  var expected = false;
  test.identical( got, expected );

  test.case = 'zero';
  var got = _.ointerval.isPopulated( 0 );
  var expected = false;
  test.identical( got, expected );

  test.case = 'NaN';
  var got = _.ointerval.isPopulated( NaN );
  var expected = false;
  test.identical( got, expected );

  test.case = 'a boolean';
  var got = _.ointerval.isPopulated( true );
  var expected = false;
  test.identical( got, expected );

  test.case = 'a number';
  var got = _.ointerval.isPopulated( 13 );
  var expected = false;
  test.identical( got, expected );

  test.case = 'a function';
  var got = _.ointerval.isPopulated( function() {} );
  var expected = false;
  test.identical( got, expected );

  test.case = 'constructor';
  function Constr( x )
  {
    this.x = x;
    return this;
  }
  var got = _.ointerval.isPopulated( new Constr( 0 ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'a string';
  var got = _.ointerval.isPopulated( 'str' );
  var expected = false;
  test.identical( got, expected );

  test.case = 'BufferRaw';
  var got = _.ointerval.isPopulated( new BufferRaw( 5 ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'BufferView';
  var got = _.ointerval.isPopulated( new BufferView( new BufferRaw( 5 ) ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'Set';
  var got = _.ointerval.isPopulated( new Set( [ 5 ] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'Map';
  var got = _.ointerval.isPopulated( new HashMap( [ [ 1, 2 ] ] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'pure empty map';
  var got = _.ointerval.isPopulated( Object.create( null ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'pure map';
  var src = Object.create( null );
  src.x = 1;
  var got = _.ointerval.isPopulated( src );
  var expected = false;
  test.identical( got, expected );

  test.case = 'map from pure map';
  var src = Object.create( Object.create( null ) );
  var got = _.ointerval.isPopulated( src );
  var expected = false;
  test.identical( got, expected );

  test.case = 'an empty object';
  var got = _.ointerval.isPopulated( {} );
  var expected = false;
  test.identical( got, expected );

  test.case = 'an object';
  var got = _.ointerval.isPopulated( { a : 7, b : 13 } );
  var expected = false;
  test.identical( got, expected );

  /* */

  test.case = 'array.length = 0';
  var got = _.ointerval.isPopulated( [] );
  var expected = false;
  test.identical( got, expected );

  test.case = 'array.length = 1';
  var got = _.ointerval.isPopulated( [ 1 ] );
  var expected = false;
  test.identical( got, expected );

  test.case = 'array.length = 2, different numbers';
  var got = _.ointerval.isPopulated( [ 1, 2 ] );
  var expected = true;
  test.identical( got, expected );

  test.case = 'array.length = 2, equal numbers';
  var got = _.ointerval.isPopulated( [ 1, 1 ] );
  var expected = false;
  test.identical( got, expected );

  test.case = 'array.length = 2, number and undefined';
  var got = _.ointerval.isPopulated( [ 1, undefined ] );
  var expected = false;
  test.identical( got, expected );

  test.case = 'array.length > 2';
  var got = _.ointerval.isPopulated( [ 1, 2, 3 ] );
  var expected = false;
  test.identical( got, expected );

  test.case = 'unroll.length = 0';
  var got = _.ointerval.isPopulated( _.unroll.make( [] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'unroll.length = 1';
  var got = _.ointerval.isPopulated( _.unroll.make( [ 1 ] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'unroll.length = 2, different numbers';
  var got = _.ointerval.isPopulated( _.unroll.make( [ 1, 2 ] ) );
  var expected = true;
  test.identical( got, expected );

  test.case = 'unroll.length = 2, equal numbers';
  var got = _.ointerval.isPopulated( _.unroll.make( [ 10, 10 ] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'unroll.length = 2, number and undefined';
  var got = _.ointerval.isPopulated( _.unroll.make( [ 1, undefined ] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'unroll.length > 2';
  var got = _.ointerval.isPopulated( _.unroll.make( [ 1, 2, 3 ] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'argumentsArray.length = 0';
  var got = _.ointerval.isPopulated( _.argumentsArray.make( [] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'argumentsArray.length = 1';
  var got = _.ointerval.isPopulated( _.argumentsArray.make( [ 1 ] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'argumentsArray.length = 2, different numbers';
  var got = _.ointerval.isPopulated( _.argumentsArray.make( [ 1, 2 ] ) );
  var expected = true;
  test.identical( got, expected );

  test.case = 'argumentsArray.length = 2, equal numbers';
  var got = _.ointerval.isPopulated( _.argumentsArray.make( [ -2, -2 ] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'argumentsArray.length = 2, number and undefined';
  var got = _.ointerval.isPopulated( _.argumentsArray.make( [ 1, undefined ] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'argumentsArray.length > 2';
  var got = _.ointerval.isPopulated( _.argumentsArray.make( [ 1, 2, 3 ] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'BufferTyped.length = 0';
  var got = _.ointerval.isPopulated( new U8x() );
  var expected = false;
  test.identical( got, expected );

  test.case = 'BufferTyped.length = 1';
  var got = _.ointerval.isPopulated( new I16x( 1 ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'BufferTyped.length = 2, different numbers';
  var got = _.ointerval.isPopulated( new F32x( [ 1, 3 ] ) );
  var expected = true;
  test.identical( got, expected );

  test.case = 'BufferTyped.length = 2, equal numbers';
  var got = _.ointerval.isPopulated( new F32x( [ 1, 1 ] ) );
  var expected = false;
  test.identical( got, expected );

  test.case = 'BufferTyped.length > 2';
  var got = _.ointerval.isPopulated( new F32x( 4 ) );
  var expected = false;
  test.identical( got, expected );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.ointerval.isPopulated() );

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.ointerval.isPopulated( [ 1, 2 ], 'extra' ) );
}

//

// function ointervalInInclusive( test )
// {
//   test.case = 'srcNumber - number, srcNumber < ointerval[ 0 ]';
//   var got = _.ointerval.inInclusive( [ 2, 5 ], 1 );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - number, srcNumber = ointerval[ 0 ]';
//   var got = _.ointerval.inInclusive( [ 2, 5 ], 2 );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - number, srcNumber > ointerval[ 1 ]';
//   var got = _.ointerval.inInclusive( [ 2, 5 ], 7 );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - number, srcNumber = ointerval[ 1 ]';
//   var got = _.ointerval.inInclusive( [ 2, 5 ], 5 );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - number, ointerval[ 0 ] < srcNumber < range[ 1 ]';
//   var got = _.ointerval.inInclusive( [ 2, 5 ], 4 );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - array, srcNumber < ointerval[ 0 ]';
//   var got = _.ointerval.inInclusive( [ 2, 5 ], [ 0 ] );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - array, srcNumber = ointerval[ 0 ]';
//   var got = _.ointerval.inInclusive( [ 2, 5 ], [ 0, 0 ] );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - array, srcNumber > ointerval[ 1 ]';
//   var got = _.ointerval.inInclusive( [ 2, 5 ], [ 0, 0, 0, 0, 0, 0, 0 ] );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - array, srcNumber = ointerval[ 1 ]';
//   var got = _.ointerval.inInclusive( [ 2, 5 ], [ 0, 0, 0, 0, 0 ] );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - array, ointerval[ 0 ] < srcNumber < range[ 1 ]';
//   var got = _.ointerval.inInclusive( [ 2, 5 ], [ 0, 0, 0, 0 ] );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - unroll, srcNumber < ointerval[ 0 ]';
//   var got = _.ointerval.inInclusive( [ 2, 5 ], _.unroll.make( [ 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - unroll, srcNumber = ointerval[ 0 ]';
//   var got = _.ointerval.inInclusive( [ 2, 5 ], _.unroll.make( [ 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - unroll, srcNumber > ointerval[ 1 ]';
//   var got = _.ointerval.inInclusive( [ 2, 5 ], _.unroll.make( [ 0, 0, 0, 0, 0, 0, 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - unroll, srcNumber = ointerval[ 1 ]';
//   var got = _.ointerval.inInclusive( [ 2, 5 ], _.unroll.make( [ 0, 0, 0, 0, 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - unroll, ointerval[ 0 ] < srcNumber < range[ 1 ]';
//   var got = _.ointerval.inInclusive( [ 2, 5 ], _.unroll.make( [ 0, 0, 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - argumentsArray, srcNumber < ointerval[ 0 ]';
//   var got = _.ointerval.inInclusive( [ 2, 5 ], _.argumentsArray.make( [ 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - argumentsArray, srcNumber = ointerval[ 0 ]';
//   var got = _.ointerval.inInclusive( [ 2, 5 ], _.argumentsArray.make( [ 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - argumentsArray, srcNumber > ointerval[ 1 ]';
//   var got = _.ointerval.inInclusive( [ 2, 5 ], _.argumentsArray.make( [ 0, 0, 0, 0, 0, 0, 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - argumentsArray, srcNumber = ointerval[ 1 ]';
//   var got = _.ointerval.inInclusive( [ 2, 5 ], _.argumentsArray.make( [ 0, 0, 0, 0, 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - argumentsArray, ointerval[ 0 ] < srcNumber < range[ 1 ]';
//   var got = _.ointerval.inInclusive( [ 2, 5 ], _.argumentsArray.make( [ 0, 0, 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - BufferTyped, srcNumber < ointerval[ 0 ]';
//   var got = _.ointerval.inInclusive( [ 2, 5 ], new I8x( [ 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - BufferTyped, srcNumber = ointerval[ 0 ]';
//   var got = _.ointerval.inInclusive( [ 2, 5 ], new I8x( [ 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - BufferTyped, srcNumber > ointerval[ 1 ]';
//   var got = _.ointerval.inInclusive( [ 2, 5 ], new U16x( [ 0, 0, 0, 0, 0, 0, 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - BufferTyped, srcNumber = ointerval[ 1 ]';
//   var got = _.ointerval.inInclusive( [ 2, 5 ], new U16x( [ 0, 0, 0, 0, 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - BufferTyped, ointerval[ 0 ] < srcNumber < range[ 1 ]';
//   var got = _.ointerval.inInclusive( [ 2, 5 ], new F32x( [ 0, 0, 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   /* - */
//
//   if( !Config.debug )
//   return;
//
//   test.case = 'without arguments';
//   test.shouldThrowErrorSync( () => _.ointerval.inInclusive() );
//
//   test.case = 'not enough arguments';
//   test.shouldThrowErrorSync( () => _.ointerval.inInclusive( [ 1, 2 ] ) );
//
//   test.case = 'extra arguments';
//   test.shouldThrowErrorSync( () => _.ointerval.inInclusive( [ 1, 2 ], 3, 'extra' ) );
//
//   test.case = 'ointerval is not Range';
//   test.shouldThrowErrorSync( () => _.ointerval.inInclusive( 'wrong', 3 ) );
//
//   test.case = 'srcNumber is not Long, not Number';
//   test.shouldThrowErrorSync( () => _.ointerval.inInclusive( [ 1, 2 ], 'wrong' ) );
// }
//
// //
//
// function ointervalInExclusive( test )
// {
//   test.case = 'srcNumber - number, srcNumber < ointerval[ 0 ]';
//   var got = _.ointerval.inExclusive( [ 2, 5 ], 1 );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - number, srcNumber = ointerval[ 0 ]';
//   var got = _.ointerval.inExclusive( [ 2, 5 ], 2 );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - number, srcNumber > ointerval[ 1 ]';
//   var got = _.ointerval.inExclusive( [ 2, 5 ], 7 );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - number, srcNumber = ointerval[ 1 ]';
//   var got = _.ointerval.inExclusive( [ 2, 5 ], 5 );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - number, ointerval[ 0 ] < srcNumber < ointerval[ 1 ]';
//   var got = _.ointerval.inExclusive( [ 2, 5 ], 4 );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - array, srcNumber < ointerval[ 0 ]';
//   var got = _.ointerval.inExclusive( [ 2, 5 ], [ 0 ] );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - array, srcNumber = ointerval[ 0 ]';
//   var got = _.ointerval.inExclusive( [ 2, 5 ], [ 0, 0 ] );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - array, srcNumber > ointerval[ 1 ]';
//   var got = _.ointerval.inExclusive( [ 2, 5 ], [ 0, 0, 0, 0, 0, 0, 0 ] );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - array, srcNumber = ointerval[ 1 ]';
//   var got = _.ointerval.inExclusive( [ 2, 5 ], [ 0, 0, 0, 0, 0 ] );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - array, ointerval[ 0 ] < srcNumber < ointerval[ 1 ]';
//   var got = _.ointerval.inExclusive( [ 2, 5 ], [ 0, 0, 0, 0 ] );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - unroll, srcNumber < ointerval[ 0 ]';
//   var got = _.ointerval.inExclusive( [ 2, 5 ], _.unroll.make( [ 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - unroll, srcNumber = ointerval[ 0 ]';
//   var got = _.ointerval.inExclusive( [ 2, 5 ], _.unroll.make( [ 0, 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - unroll, srcNumber > ointerval[ 1 ]';
//   var got = _.ointerval.inExclusive( [ 2, 5 ], _.unroll.make( [ 0, 0, 0, 0, 0, 0, 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - unroll, srcNumber = ointerval[ 1 ]';
//   var got = _.ointerval.inExclusive( [ 2, 5 ], _.unroll.make( [ 0, 0, 0, 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - unroll, ointerval[ 0 ] < srcNumber < ointerval[ 1 ]';
//   var got = _.ointerval.inExclusive( [ 2, 5 ], _.unroll.make( [ 0, 0, 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - argumentsArray, srcNumber < ointerval[ 0 ]';
//   var got = _.ointerval.inExclusive( [ 2, 5 ], _.argumentsArray.make( [ 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - argumentsArray, srcNumber = ointerval[ 0 ]';
//   var got = _.ointerval.inExclusive( [ 2, 5 ], _.argumentsArray.make( [ 0, 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - argumentsArray, srcNumber > ointerval[ 1 ]';
//   var got = _.ointerval.inExclusive( [ 2, 5 ], _.argumentsArray.make( [ 0, 0, 0, 0, 0, 0, 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - argumentsArray, srcNumber = ointerval[ 1 ]';
//   var got = _.ointerval.inExclusive( [ 2, 5 ], _.argumentsArray.make( [ 0, 0, 0, 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - argumentsArray, ointerval[ 0 ] < srcNumber < ointerval[ 1 ]';
//   var got = _.ointerval.inExclusive( [ 2, 5 ], _.argumentsArray.make( [ 0, 0, 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - BufferTyped, srcNumber < ointerval[ 0 ]';
//   var got = _.ointerval.inExclusive( [ 2, 5 ], new I8x( [ 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - BufferTyped, srcNumber = ointerval[ 0 ]';
//   var got = _.ointerval.inExclusive( [ 2, 5 ], new I8x( [ 0, 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - BufferTyped, srcNumber > ointerval[ 1 ]';
//   var got = _.ointerval.inExclusive( [ 2, 5 ], new U16x( [ 0, 0, 0, 0, 0, 0, 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - BufferTyped, srcNumber = ointerval[ 1 ]';
//   var got = _.ointerval.inExclusive( [ 2, 5 ], new U16x( [ 0, 0, 0, 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - BufferTyped, ointerval[ 0 ] < srcNumber < ointerval[ 1 ]';
//   var got = _.ointerval.inExclusive( [ 2, 5 ], new F32x( [ 0, 0, 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   /* - */
//
//   if( !Config.debug )
//   return;
//
//   test.case = 'without arguments';
//   test.shouldThrowErrorSync( () => _.ointerval.inExclusive() );
//
//   test.case = 'not enough arguments';
//   test.shouldThrowErrorSync( () => _.ointerval.inExclusive( [ 1, 2 ] ) );
//
//   test.case = 'extra arguments';
//   test.shouldThrowErrorSync( () => _.ointerval.inExclusive( [ 1, 2 ], 3, 'extra' ) );
//
//   test.case = 'ointerval is not ointerval';
//   test.shouldThrowErrorSync( () => _.ointerval.inExclusive( 'wrong', 3 ) );
//
//   test.case = 'srcNumber is not Long, not Number';
//   test.shouldThrowErrorSync( () => _.ointerval.inExclusive( [ 1, 2 ], 'wrong' ) );
// }
//
// //
//
// function ointervalInInclusiveLeft( test )
// {
//   test.case = 'srcNumber - number, srcNumber < ointerval[ 0 ]';
//   var got = _.ointerval.inInclusiveLeft( [ 2, 5 ], 1 );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - number, srcNumber = ointerval[ 0 ]';
//   var got = _.ointerval.inInclusiveLeft( [ 2, 5 ], 2 );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - number, srcNumber > ointerval[ 1 ]';
//   var got = _.ointerval.inInclusiveLeft( [ 2, 5 ], 7 );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - number, srcNumber = ointerval[ 1 ]';
//   var got = _.ointerval.inInclusiveLeft( [ 2, 5 ], 5 );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - number, ointerval[ 0 ] < srcNumber < range[ 1 ]';
//   var got = _.ointerval.inInclusiveLeft( [ 2, 5 ], 4 );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - array, srcNumber < ointerval[ 0 ]';
//   var got = _.ointerval.inInclusiveLeft( [ 2, 5 ], [ 0 ] );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - array, srcNumber = ointerval[ 0 ]';
//   var got = _.ointerval.inInclusiveLeft( [ 2, 5 ], [ 0, 0 ] );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - array, srcNumber > ointerval[ 1 ]';
//   var got = _.ointerval.inInclusiveLeft( [ 2, 5 ], [ 0, 0, 0, 0, 0, 0, 0 ] );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - array, srcNumber = ointerval[ 1 ]';
//   var got = _.ointerval.inInclusiveLeft( [ 2, 5 ], [ 0, 0, 0, 0, 0 ] );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - array, ointerval[ 0 ] < srcNumber < range[ 1 ]';
//   var got = _.ointerval.inInclusiveLeft( [ 2, 5 ], [ 0, 0, 0, 0 ] );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - unroll, srcNumber < ointerval[ 0 ]';
//   var got = _.ointerval.inInclusiveLeft( [ 2, 5 ], _.unroll.make( [ 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - unroll, srcNumber = ointerval[ 0 ]';
//   var got = _.ointerval.inInclusiveLeft( [ 2, 5 ], _.unroll.make( [ 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - unroll, srcNumber > ointerval[ 1 ]';
//   var got = _.ointerval.inInclusiveLeft( [ 2, 5 ], _.unroll.make( [ 0, 0, 0, 0, 0, 0, 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - unroll, srcNumber = ointerval[ 1 ]';
//   var got = _.ointerval.inInclusiveLeft( [ 2, 5 ], _.unroll.make( [ 0, 0, 0, 0, 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - unroll, ointerval[ 0 ] < srcNumber < range[ 1 ]';
//   var got = _.ointerval.inInclusiveLeft( [ 2, 5 ], _.unroll.make( [ 0, 0, 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - argumentsArray, srcNumber < ointerval[ 0 ]';
//   var got = _.ointerval.inInclusiveLeft( [ 2, 5 ], _.argumentsArray.make( [ 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - argumentsArray, srcNumber = ointerval[ 0 ]';
//   var got = _.ointerval.inInclusiveLeft( [ 2, 5 ], _.argumentsArray.make( [ 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - argumentsArray, srcNumber > ointerval[ 1 ]';
//   var got = _.ointerval.inInclusiveLeft( [ 2, 5 ], _.argumentsArray.make( [ 0, 0, 0, 0, 0, 0, 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - argumentsArray, srcNumber = ointerval[ 1 ]';
//   var got = _.ointerval.inInclusiveLeft( [ 2, 5 ], _.argumentsArray.make( [ 0, 0, 0, 0, 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - argumentsArray, ointerval[ 0 ] < srcNumber < range[ 1 ]';
//   var got = _.ointerval.inInclusiveLeft( [ 2, 5 ], _.argumentsArray.make( [ 0, 0, 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - BufferTyped, srcNumber < ointerval[ 0 ]';
//   var got = _.ointerval.inInclusiveLeft( [ 2, 5 ], new I8x( [ 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - BufferTyped, srcNumber = ointerval[ 0 ]';
//   var got = _.ointerval.inInclusiveLeft( [ 2, 5 ], new I8x( [ 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - BufferTyped, srcNumber > ointerval[ 1 ]';
//   var got = _.ointerval.inInclusiveLeft( [ 2, 5 ], new U16x( [ 0, 0, 0, 0, 0, 0, 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - BufferTyped, srcNumber = ointerval[ 1 ]';
//   var got = _.ointerval.inInclusiveLeft( [ 2, 5 ], new U16x( [ 0, 0, 0, 0, 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - BufferTyped, ointerval[ 0 ] < srcNumber < range[ 1 ]';
//   var got = _.ointerval.inInclusiveLeft( [ 2, 5 ], new F32x( [ 0, 0, 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   /* - */
//
//   if( !Config.debug )
//   return;
//
//   test.case = 'without arguments';
//   test.shouldThrowErrorSync( () => _.ointerval.inInclusiveLeft() );
//
//   test.case = 'not enough arguments';
//   test.shouldThrowErrorSync( () => _.ointerval.inInclusiveLeft( [ 1, 2 ] ) );
//
//   test.case = 'extra arguments';
//   test.shouldThrowErrorSync( () => _.ointerval.inInclusiveLeft( [ 1, 2 ], 3, 'extra' ) );
//
//   test.case = 'ointerval is not Range';
//   test.shouldThrowErrorSync( () => _.ointerval.inInclusiveLeft( 'wrong', 3 ) );
//
//   test.case = 'srcNumber is not Long, not Number';
//   test.shouldThrowErrorSync( () => _.ointerval.inInclusiveLeft( [ 1, 2 ], 'wrong' ) );
// }
//
// //
//
// function ointervalInInclusiveRight( test )
// {
//   test.case = 'srcNumber - number, srcNumber < ointerval[ 0 ]';
//   var got = _.ointerval.inInclusiveRight( [ 2, 5 ], 1 );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - number, srcNumber = ointerval[ 0 ]';
//   var got = _.ointerval.inInclusiveRight( [ 2, 5 ], 2 );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - number, srcNumber > ointerval[ 1 ]';
//   var got = _.ointerval.inInclusiveRight( [ 2, 5 ], 7 );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - number, srcNumber = ointerval[ 1 ]';
//   var got = _.ointerval.inInclusiveRight( [ 2, 5 ], 5 );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - number, ointerval[ 0 ] < srcNumber < range[ 1 ]';
//   var got = _.ointerval.inInclusiveRight( [ 2, 5 ], 4 );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - array, srcNumber < ointerval[ 0 ]';
//   var got = _.ointerval.inInclusiveRight( [ 2, 5 ], [ 0 ] );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - array, srcNumber = ointerval[ 0 ]';
//   var got = _.ointerval.inInclusiveRight( [ 2, 5 ], [ 0, 0 ] );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - array, srcNumber > ointerval[ 1 ]';
//   var got = _.ointerval.inInclusiveRight( [ 2, 5 ], [ 0, 0, 0, 0, 0, 0, 0 ] );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - array, srcNumber = ointerval[ 1 ]';
//   var got = _.ointerval.inInclusiveRight( [ 2, 5 ], [ 0, 0, 0, 0, 0 ] );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - array, ointerval[ 0 ] < srcNumber < range[ 1 ]';
//   var got = _.ointerval.inInclusiveRight( [ 2, 5 ], [ 0, 0, 0, 0 ] );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - unroll, srcNumber < ointerval[ 0 ]';
//   var got = _.ointerval.inInclusiveRight( [ 2, 5 ], _.unroll.make( [ 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - unroll, srcNumber = ointerval[ 0 ]';
//   var got = _.ointerval.inInclusiveRight( [ 2, 5 ], _.unroll.make( [ 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - unroll, srcNumber > ointerval[ 1 ]';
//   var got = _.ointerval.inInclusiveRight( [ 2, 5 ], _.unroll.make( [ 0, 0, 0, 0, 0, 0, 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - unroll, srcNumber = ointerval[ 1 ]';
//   var got = _.ointerval.inInclusiveRight( [ 2, 5 ], _.unroll.make( [ 0, 0, 0, 0, 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - unroll, ointerval[ 0 ] < srcNumber < range[ 1 ]';
//   var got = _.ointerval.inInclusiveRight( [ 2, 5 ], _.unroll.make( [ 0, 0, 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - argumentsArray, srcNumber < ointerval[ 0 ]';
//   var got = _.ointerval.inInclusiveRight( [ 2, 5 ], _.argumentsArray.make( [ 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - argumentsArray, srcNumber = ointerval[ 0 ]';
//   var got = _.ointerval.inInclusiveRight( [ 2, 5 ], _.argumentsArray.make( [ 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - argumentsArray, srcNumber > ointerval[ 1 ]';
//   var got = _.ointerval.inInclusiveRight( [ 2, 5 ], _.argumentsArray.make( [ 0, 0, 0, 0, 0, 0, 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - argumentsArray, srcNumber = ointerval[ 1 ]';
//   var got = _.ointerval.inInclusiveRight( [ 2, 5 ], _.argumentsArray.make( [ 0, 0, 0, 0, 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - argumentsArray, ointerval[ 0 ] < srcNumber < range[ 1 ]';
//   var got = _.ointerval.inInclusiveRight( [ 2, 5 ], _.argumentsArray.make( [ 0, 0, 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - BufferTyped, srcNumber < ointerval[ 0 ]';
//   var got = _.ointerval.inInclusiveRight( [ 2, 5 ], new I8x( [ 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - BufferTyped, srcNumber = ointerval[ 0 ]';
//   var got = _.ointerval.inInclusiveRight( [ 2, 5 ], new I8x( [ 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - BufferTyped, srcNumber > ointerval[ 1 ]';
//   var got = _.ointerval.inInclusiveRight( [ 2, 5 ], new U16x( [ 0, 0, 0, 0, 0, 0, 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - BufferTyped, srcNumber = ointerval[ 1 ]';
//   var got = _.ointerval.inInclusiveRight( [ 2, 5 ], new U16x( [ 0, 0, 0, 0, 0 ] ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'srcNumber - BufferTyped, ointerval[ 0 ] < srcNumber < range[ 1 ]';
//   var got = _.ointerval.inInclusiveRight( [ 2, 5 ], new F32x( [ 0, 0, 0, 0 ] ) );
//   var expected = true;
//   test.identical( got, expected );
//
//   /* - */
//
//   if( !Config.debug )
//   return;
//
//   test.case = 'without arguments';
//   test.shouldThrowErrorSync( () => _.ointerval.inInclusiveRight() );
//
//   test.case = 'not enough arguments';
//   test.shouldThrowErrorSync( () => _.ointerval.inInclusiveRight( [ 1, 2 ] ) );
//
//   test.case = 'extra arguments';
//   test.shouldThrowErrorSync( () => _.ointerval.inInclusiveRight( [ 1, 2 ], 3, 'extra' ) );
//
//   test.case = 'ointerval is not Range';
//   test.shouldThrowErrorSync( () => _.ointerval.inInclusiveRight( 'wrong', 3 ) );
//
//   test.case = 'srcNumber is not Long, not Number';
//   test.shouldThrowErrorSync( () => _.ointerval.inInclusiveRight( [ 1, 2 ], 'wrong' ) );
// }

//

function ointervalHas( test )
{
  test.open( 'src - number' );

  test.case = 'src < ointerval[ 0 ]';
  var got = _.ointerval.has( [ 2, 5 ], 1 );
  var expected = false;
  test.identical( got, expected );

  test.case = 'src === ointerval[ 0 ]';
  var got = _.ointerval.has( [ 2, 5 ], 2 );
  var expected = true;
  test.identical( got, expected );

  test.case = 'src > ointerval[ 1 ]';
  var got = _.ointerval.has( [ 2, 5 ], 7 );
  var expected = false;
  test.identical( got, expected );

  test.case = 'src === ointerval[ 1 ]';
  var got = _.ointerval.has( [ 2, 5 ], 5 );
  var expected = false;
  test.identical( got, expected );

  test.case = 'ointerval[ 0 ] < src < ointerval[ 1 ]';
  var got = _.ointerval.has( [ 2, 5 ], 4 );
  var expected = true;
  test.identical( got, expected );

  test.close( 'src - number' );

  /* - */

  test.open( 'src - interval' );

  test.case = 'src[ 0 ] < ointerval[ 0 ]';
  var got = _.ointerval.has( [ 2, 5 ], [ 1, 4 ] );
  var expected = false;
  test.identical( got, expected );

  test.case = 'src[ 0 ] === ointerval[ 0 ]';
  var got = _.ointerval.has( [ 2, 5 ], [ 2, 4 ] );
  var expected = true;
  test.identical( got, expected );

  test.case = 'src[ 1 ] > ointerval[ 1 ]';
  var got = _.ointerval.has( [ 2, 5 ], [ 2, 6 ] );
  var expected = false;
  test.identical( got, expected );

  test.case = 'src[ 1 ] === ointerval[ 1 ]';
  var got = _.ointerval.has( [ 2, 5 ], [ 2, 5 ] );
  var expected = true;
  test.identical( got, expected );

  test.case = 'ointerval[ 0 ] < src[ 0 ] && src[ 1 ] < ointerval[ 1 ]';
  var got = _.ointerval.has( [ 2, 5 ], [ 3, 4 ] );
  var expected = true;
  test.identical( got, expected );

  test.close( 'src - interval' );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.ointerval.has() );

  test.case = 'not enough arguments';
  test.shouldThrowErrorSync( () => _.ointerval.has( [ 1, 2 ] ) );

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.ointerval.has( [ 1, 2 ], 3, 'extra' ) );

  test.case = 'ointerval is not ointerval';
  test.shouldThrowErrorSync( () => _.ointerval.has( 'wrong', 3 ) );

  test.case = 'src is not Interval, not Number';
  test.shouldThrowErrorSync( () => _.ointerval.has( [ 1, 2 ], 'wrong' ) );
}

//

function ointervalSureInRange( test )
{
  test.case = 'two arguments, src - number, in ointerval';
  var got = _.ointerval.sureIn( 3, [ 1, 5 ] );
  var expected = true;
  test.identical( got, expected );

  test.case = 'two arguments, src - array, in ointerval';
  var got = _.ointerval.sureIn( [ 0, 0, 0 ], [ 1, 5 ] );
  var expected = true;
  test.identical( got, expected );

  test.case = 'more then two arguments, src - number, in ointerval';
  var got = _.ointerval.sureIn( 3, [ 1, 5 ], 'extra', [ 'next' ] );
  var expected = true;
  test.identical( got, expected );

  test.case = 'more then two arguments, src - array, in ointerval';
  var got = _.ointerval.sureIn( [ 0, 0, 0 ], [ 1, 5 ], 'extra', [ 'next' ] );
  var expected = true;
  test.identical( got, expected );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.ointerval.sureIn() );

  test.case = 'not enough arguments';
  test.shouldThrowErrorSync( () => _.ointerval.sureIn( 2 ) );

  test.case = 'src out of ointerval';
  test.shouldThrowErrorSync( () => _.ointerval.sureIn( 1, [ 2, 5 ] ) );
  test.shouldThrowErrorSync( () => _.ointerval.sureIn( 5, [ 2, 5 ] ) );
  test.shouldThrowErrorSync( () => _.ointerval.sureIn( 1, [ 2, 5 ], [] ) );
  test.shouldThrowErrorSync( () => _.ointerval.sureIn( 7, [ 2, 5 ], undefined ) );
}

//

function ointervalAssertInRange( test )
{
  test.case = 'two arguments, src - number, in ointerval';
  var got = _.ointerval.assertIn( 3, [ 1, 5 ] );
  var expected = true;
  test.identical( got, expected );

  test.case = 'two arguments, src - array, in ointerval';
  var got = _.ointerval.assertIn( 3, [ 1, 5 ] );
  var expected = true;
  test.identical( got, expected );

  test.case = 'more then two arguments, src - number, in ointerval';
  var got = _.ointerval.assertIn( 3, [ 1, 5 ], 'extra', [ 'next' ] );
  var expected = true;
  test.identical( got, expected );

  test.case = 'more then two arguments, src - array, in ointerval';
  var got = _.ointerval.assertIn( 3, [ 1, 5 ], 'extra', [ 'next' ] );
  var expected = true;
  test.identical( got, expected );

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.ointerval.assertIn() );

  test.case = 'not enough arguments';
  test.shouldThrowErrorSync( () => _.ointerval.assertIn( 2 ) );

  test.case = 'src out of ointerval';
  test.shouldThrowErrorSync( () => _.ointerval.assertIn( 1, [ 2, 5 ] ) );
  test.shouldThrowErrorSync( () => _.ointerval.assertIn( 5, [ 2, 5 ] ) );
  test.shouldThrowErrorSync( () => _.ointerval.assertIn( 7, [ 2, 5 ] ) );
  test.shouldThrowErrorSync( () => _.ointerval.assertIn( 1, [ 2, 5 ], [] ) );
  test.shouldThrowErrorSync( () => _.ointerval.assertIn( 5, [ 2, 5 ], {} ) );
  test.shouldThrowErrorSync( () => _.ointerval.assertIn( 7, [ 2, 5 ], undefined ) );
}

//

function ointervalFromSingle( test )
{
  test.case = 'ointerval - number';
  var got = _.ointerval.fromSingle( 1 );
  test.identical( got, [ 1, 2 ] );

  /* */

  test.case = 'ointerval.length === 1';
  var src = [ 1 ];
  var got = _.ointerval.fromSingle( src );
  test.identical( got, [ 1, 2 ] );
  test.true( got !== src );

  test.case = 'ointerval[ 0 ] - undefined';
  var src = [ undefined, 2 ];
  var got = _.ointerval.fromSingle( src );
  test.identical( got, [ 1, 2 ] );
  test.true( got !== src );

  test.case = 'ointerval[ 1 ] - undefined';
  var src = [ 1, undefined ];
  var got = _.ointerval.fromSingle( src );
  test.identical( got, [ 1, 2 ] );
  test.true( got !== src );

  test.case = 'ointerval[ 0 ] - undefined, ointerval[ 1 ] - undefined';
  var src = [ undefined, undefined ];
  var got = _.ointerval.fromSingle( src );
  test.identical( got, [ 0, 1 ] );
  test.true( got !== src );

  test.case = 'ointerval[ 0 ] - number, ointerval[ 1 ] - number';
  var src = [ 1, 5 ];
  var got = _.ointerval.fromSingle( src );
  test.identical( got, [ 1, 5 ] );
  test.true( got === src );

  /* */

  test.case = 'ointerval.length === 1';
  var src = _.unroll.make( [ 1 ] );
  var got = _.ointerval.fromSingle( src );
  test.identical( got, [ 1, 2 ] );
  test.true( got !== src );

  test.case = 'ointerval[ 0 ] - undefined';
  var src = _.unroll.make( [ undefined, 2 ] );
  var got = _.ointerval.fromSingle( src );
  test.identical( got, [ 1, 2 ] );
  test.true( got !== src );

  test.case = 'ointerval[ 1 ] - undefined';
  var src = _.unroll.make( [ 1, undefined ] );
  var got = _.ointerval.fromSingle( src );
  test.identical( got, [ 1, 2 ] );
  test.true( got !== src );

  test.case = 'ointerval[ 0 ] - undefined, ointerval[ 1 ] - undefined';
  var src = _.unroll.make( [ undefined, undefined ] );
  var got = _.ointerval.fromSingle( src );
  test.identical( got, [ 0, 1 ] );
  test.true( got !== src );

  test.case = 'ointerval[ 0 ] - number, ointerval[ 1 ] - number';
  var src = _.unroll.make( [ 1, 5 ] );
  var got = _.ointerval.fromSingle( src );
  test.identical( got, _.unroll.make( [ 1, 5 ] ) );
  test.true( got === src );

  /* */

  test.case = 'ointerval.length === 1';
  var src = _.argumentsArray.make( [ 1 ] );
  var got = _.ointerval.fromSingle( src );
  test.identical( got, [ 1, 2 ] );
  test.true( got !== src );

  test.case = 'ointerval[ 0 ] - undefined';
  var src = _.argumentsArray.make( [ undefined, 2 ] );
  var got = _.ointerval.fromSingle( src );
  test.identical( got, [ 1, 2 ] );
  test.true( got !== src );

  test.case = 'ointerval[ 1 ] - undefined';
  var src = _.argumentsArray.make( [ 1, undefined ] );
  var got = _.ointerval.fromSingle( src );
  test.identical( got, [ 1, 2 ] );
  test.true( got !== src );

  test.case = 'ointerval[ 0 ] - undefined, ointerval[ 1 ] - undefined';
  var src = _.argumentsArray.make( [ undefined, undefined ] );
  var got = _.ointerval.fromSingle( src );
  test.identical( got, [ 0, 1 ] );
  test.true( got !== src );

  test.case = 'ointerval[ 0 ] - number, ointerval[ 1 ] - number';
  var src = _.argumentsArray.make( [ 1, 5 ] );
  var got = _.ointerval.fromSingle( src );
  test.identical( got, _.argumentsArray.make( [ 1, 5 ] ) );
  test.true( got === src );

  /* */

  test.case = 'ointerval.length === 1';
  var src = new U8x( [ 1 ] );
  var got = _.ointerval.fromSingle( src );
  test.identical( got, [ 1, 2 ] );
  test.true( got !== src );

  test.case = 'ointerval[ 0 ] - number, ointerval[ 1 ] - number';
  var src = new I16x( [ 1, 5 ] );
  var got = _.ointerval.fromSingle( src );
  test.identical( got, new I16x( [ 1, 5 ] ) );
  test.true( got === src );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.ointerval.fromSingle() );

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.ointerval.fromSingle( [ 1, 2 ], 1 ) );

  test.case = 'wrong type of ointerval';
  test.shouldThrowErrorSync( () => _.ointerval.fromSingle( { 0 : 1, 1 : 2 } ) );

  test.case = 'wrong ointerval length';
  test.shouldThrowErrorSync( () => _.ointerval.fromSingle( [ 0, 1, 2 ] ) );

  test.case = 'wrong elements in ointerval';
  test.shouldThrowErrorSync( () => _.ointerval.fromSingle( [ null, 2 ] ) );
  test.shouldThrowErrorSync( () => _.ointerval.fromSingle( [ 2, 'abc' ] ) );
}

//

function ointervalClamp( test )
{
  test.open( 'array' );

  test.case = 'identical ranges';
  var dst = [ 2, 2 ];
  var clamp = [ 2, 2 ];
  var got = _.ointerval.clamp( dst, clamp );
  test.identical( got, [ 2, 2 ] );
  test.true( got === dst );

  test.case = 'dst[ 0 ] < clamp[ 0 ], dst[ 1 ] < clamp[ 0 ]';
  var dst = [ 2, 2 ];
  var clamp = [ 3, 6 ];
  var got = _.ointerval.clamp( dst, clamp );
  test.identical( got, [ 3, 3 ] );
  test.true( got === dst );

  test.case = 'dst[ 0 ] < clamp[ 0 ], clamp[ 1 ] > dst[ 1 ] > clamp[ 0 ]';
  var dst = [ 2, 4 ];
  var clamp = [ 3, 6 ];
  var got = _.ointerval.clamp( dst, clamp );
  test.identical( got, [ 3, 4 ] );
  test.true( got === dst );

  test.case = 'clamp[ 1 ] > dst[ 0 ] > clamp[ 0 ], dst[ 1 ] < clamp[ 0 ]';
  var dst = [ 4, 2 ];
  var clamp = [ 3, 6 ];
  var got = _.ointerval.clamp( dst, clamp );
  test.identical( got, [ 4, 3 ] );
  test.true( got === dst );

  test.case = 'clamp[ 1 ] > dst[ 0 ] > clamp[ 0 ], clamp[ 1 ] > dst[ 1 ] > clamp[ 0 ]';
  var dst = [ 4, 5 ];
  var clamp = [ 3, 6 ];
  var got = _.ointerval.clamp( dst, clamp );
  test.identical( got, [ 4, 5 ] );
  test.true( got === dst );

  test.case = 'dst[ 0 ] > clamp[ 1 ], clamp[ 1 ] > dst[ 1 ] > clamp[ 0 ]';
  var dst = [ 7, 5 ];
  var clamp = [ 3, 6 ];
  var got = _.ointerval.clamp( dst, clamp );
  test.identical( got, [ 6, 5 ] );
  test.true( got === dst );

  test.case = 'clamp[ 1 ] > dst[ 0 ] > clamp[ 0 ], dst[ 1 ] > clamp[ 1 ]';
  var dst = [ 4, 7 ];
  var clamp = [ 3, 6 ];
  var got = _.ointerval.clamp( dst, clamp );
  test.identical( got, [ 4, 6 ] );
  test.true( got === dst );

  test.case = 'dst[ 0 ] > clamp[ 1 ], dst[ 1 ] > clamp[ 1 ]';
  var dst = [ 8, 7 ];
  var clamp = [ 3, 6 ];
  var got = _.ointerval.clamp( dst, clamp );
  test.identical( got, [ 6, 6 ] );
  test.true( got === dst );

  test.case = 'dst[ 0 ] < clamp[ 0 ], dst[ 1 ] > clamp[ 1 ]';
  var dst = [ 2, 7 ];
  var clamp = [ 3, 6 ];
  var got = _.ointerval.clamp( dst, clamp );
  test.identical( got, [ 3, 6 ] );
  test.true( got === dst );

  test.close( 'array' );

  /* - */

  test.open( 'unroll' );

  test.case = 'identical ranges';
  var dst = _.unroll.make( [ 2, 2 ] );
  var clamp = [ 2, 2 ];
  var got = _.ointerval.clamp( dst, clamp );
  test.identical( got, _.unroll.make( [ 2, 2 ] ) );
  test.true( got === dst );

  test.case = 'dst[ 0 ] < clamp[ 0 ], dst[ 1 ] < clamp[ 0 ]';
  var dst = _.unroll.make( [ 2, 2 ] );
  var clamp = [ 3, 6 ];
  var got = _.ointerval.clamp( dst, clamp );
  test.identical( got, _.unroll.make( [ 3, 3 ] ) );
  test.true( got === dst );

  test.case = 'dst[ 0 ] < clamp[ 0 ], clamp[ 1 ] > dst[ 1 ] > clamp[ 0 ]';
  var dst = _.unroll.make( [ 2, 4 ] );
  var clamp = [ 3, 6 ];
  var got = _.ointerval.clamp( dst, clamp );
  test.identical( got, _.unroll.make( [ 3, 4 ] ) );
  test.true( got === dst );

  test.case = 'clamp[ 1 ] > dst[ 0 ] > clamp[ 0 ], dst[ 1 ] < clamp[ 0 ]';
  var dst = _.unroll.make( [ 4, 2 ] );
  var clamp = [ 3, 6 ];
  var got = _.ointerval.clamp( dst, clamp );
  test.identical( got, _.unroll.make( [ 4, 3 ] ) );
  test.true( got === dst );

  test.case = 'clamp[ 1 ] > dst[ 0 ] > clamp[ 0 ], clamp[ 1 ] > dst[ 1 ] > clamp[ 0 ]';
  var dst = _.unroll.make( [ 4, 5 ] );
  var clamp = [ 3, 6 ];
  var got = _.ointerval.clamp( dst, clamp );
  test.identical( got, _.unroll.make( [ 4, 5 ] ) );
  test.true( got === dst );

  test.case = 'dst[ 0 ] > clamp[ 1 ], clamp[ 1 ] > dst[ 1 ] > clamp[ 0 ]';
  var dst = _.unroll.make( [ 7, 5 ] );
  var clamp = [ 3, 6 ];
  var got = _.ointerval.clamp( dst, clamp );
  test.identical( got, _.unroll.make( [ 6, 5 ] ) );
  test.true( got === dst );

  test.case = 'clamp[ 1 ] > dst[ 0 ] > clamp[ 0 ], dst[ 1 ] > clamp[ 1 ]';
  var dst = _.unroll.make( [ 4, 7 ] );
  var clamp = [ 3, 6 ];
  var got = _.ointerval.clamp( dst, clamp );
  test.identical( got, _.unroll.make( [ 4, 6 ] ) );
  test.true( got === dst );

  test.case = 'dst[ 0 ] > clamp[ 1 ], dst[ 1 ] > clamp[ 1 ]';
  var dst = _.unroll.make( [ 8, 7 ] );
  var clamp = [ 3, 6 ];
  var got = _.ointerval.clamp( dst, clamp );
  test.identical( got, _.unroll.make( [ 6, 6 ] ) );
  test.true( got === dst );

  test.case = 'dst[ 0 ] < clamp[ 0 ], dst[ 1 ] > clamp[ 1 ]';
  var dst = _.unroll.make( [ 2, 7 ] );
  var clamp = [ 3, 6 ];
  var got = _.ointerval.clamp( dst, clamp );
  test.identical( got, _.unroll.make( [ 3, 6 ] ) );
  test.true( got === dst );

  test.close( 'unroll' );

  /* - */

  test.open( 'argumentsArray' );

  test.case = 'identical ranges';
  var dst = _.argumentsArray.make( [ 2, 2 ] );
  var clamp = [ 2, 2 ];
  var got = _.ointerval.clamp( dst, clamp );
  test.identical( got, _.argumentsArray.make( [ 2, 2 ] ) );
  test.true( got === dst );

  test.case = 'dst[ 0 ] < clamp[ 0 ], dst[ 1 ] < clamp[ 0 ]';
  var dst = _.argumentsArray.make( [ 2, 2 ] );
  var clamp = [ 3, 6 ];
  var got = _.ointerval.clamp( dst, clamp );
  test.identical( got, _.argumentsArray.make( [ 3, 3 ] ) );
  test.true( got === dst );

  test.case = 'dst[ 0 ] < clamp[ 0 ], clamp[ 1 ] > dst[ 1 ] > clamp[ 0 ]';
  var dst = _.argumentsArray.make( [ 2, 4 ] );
  var clamp = [ 3, 6 ];
  var got = _.ointerval.clamp( dst, clamp );
  test.identical( got, _.argumentsArray.make( [ 3, 4 ] ) );
  test.true( got === dst );

  test.case = 'clamp[ 1 ] > dst[ 0 ] > clamp[ 0 ], dst[ 1 ] < clamp[ 0 ]';
  var dst = _.argumentsArray.make( [ 4, 2 ] );
  var clamp = [ 3, 6 ];
  var got = _.ointerval.clamp( dst, clamp );
  test.identical( got, _.argumentsArray.make( [ 4, 3 ] ) );
  test.true( got === dst );

  test.case = 'clamp[ 1 ] > dst[ 0 ] > clamp[ 0 ], clamp[ 1 ] > dst[ 1 ] > clamp[ 0 ]';
  var dst = _.argumentsArray.make( [ 4, 5 ] );
  var clamp = [ 3, 6 ];
  var got = _.ointerval.clamp( dst, clamp );
  test.identical( got, _.argumentsArray.make( [ 4, 5 ] ) );
  test.true( got === dst );

  test.case = 'dst[ 0 ] > clamp[ 1 ], clamp[ 1 ] > dst[ 1 ] > clamp[ 0 ]';
  var dst = _.argumentsArray.make( [ 7, 5 ] );
  var clamp = [ 3, 6 ];
  var got = _.ointerval.clamp( dst, clamp );
  test.identical( got, _.argumentsArray.make( [ 6, 5 ] ) );
  test.true( got === dst );

  test.case = 'clamp[ 1 ] > dst[ 0 ] > clamp[ 0 ], dst[ 1 ] > clamp[ 1 ]';
  var dst = _.argumentsArray.make( [ 4, 7 ] );
  var clamp = [ 3, 6 ];
  var got = _.ointerval.clamp( dst, clamp );
  test.identical( got, _.argumentsArray.make( [ 4, 6 ] ) );
  test.true( got === dst );

  test.case = 'dst[ 0 ] > clamp[ 1 ], dst[ 1 ] > clamp[ 1 ]';
  var dst = _.argumentsArray.make( [ 8, 7 ] );
  var clamp = [ 3, 6 ];
  var got = _.ointerval.clamp( dst, clamp );
  test.identical( got, _.argumentsArray.make( [ 6, 6 ] ) );
  test.true( got === dst );

  test.case = 'dst[ 0 ] < clamp[ 0 ], dst[ 1 ] > clamp[ 1 ]';
  var dst = _.argumentsArray.make( [ 2, 7 ] );
  var clamp = [ 3, 6 ];
  var got = _.ointerval.clamp( dst, clamp );
  test.identical( got, _.argumentsArray.make( [ 3, 6 ] ) );
  test.true( got === dst );

  test.close( 'argumentsArray' );

  /* - */

  test.open( 'bufferTyped' );

  test.case = 'identical ranges';
  var dst = new U8x( [ 2, 2 ] );
  var clamp = [ 2, 2 ];
  var got = _.ointerval.clamp( dst, clamp );
  test.identical( got, new U8x( [ 2, 2 ] ) );
  test.true( got === dst );

  test.case = 'dst[ 0 ] < clamp[ 0 ], dst[ 1 ] < clamp[ 0 ]';
  var dst = new I8x( [ 2, 2 ] );
  var clamp = [ 3, 6 ];
  var got = _.ointerval.clamp( dst, clamp );
  test.identical( got, new I8x( [ 3, 3 ] ) );
  test.true( got === dst );

  test.case = 'dst[ 0 ] < clamp[ 0 ], clamp[ 1 ] > dst[ 1 ] > clamp[ 0 ]';
  var dst = new U16x( [ 2, 4 ] );
  var clamp = [ 3, 6 ];
  var got = _.ointerval.clamp( dst, clamp );
  test.identical( got, new U16x( [ 3, 4 ] ) );
  test.true( got === dst );

  test.case = 'clamp[ 1 ] > dst[ 0 ] > clamp[ 0 ], dst[ 1 ] < clamp[ 0 ]';
  var dst = new I16x( [ 4, 2 ] );
  var clamp = [ 3, 6 ];
  var got = _.ointerval.clamp( dst, clamp );
  test.identical( got, new I16x( [ 4, 3 ] ) );
  test.true( got === dst );

  test.case = 'clamp[ 1 ] > dst[ 0 ] > clamp[ 0 ], clamp[ 1 ] > dst[ 1 ] > clamp[ 0 ]';
  var dst = new U32x( [ 4, 5 ] );
  var clamp = [ 3, 6 ];
  var got = _.ointerval.clamp( dst, clamp );
  test.identical( got, new U32x( [ 4, 5 ] ) );
  test.true( got === dst );

  test.case = 'dst[ 0 ] > clamp[ 1 ], clamp[ 1 ] > dst[ 1 ] > clamp[ 0 ]';
  var dst = new I32x( [ 7, 5 ] );
  var clamp = [ 3, 6 ];
  var got = _.ointerval.clamp( dst, clamp );
  test.identical( got, new I32x( [ 6, 5 ] ) );
  test.true( got === dst );

  test.case = 'clamp[ 1 ] > dst[ 0 ] > clamp[ 0 ], dst[ 1 ] > clamp[ 1 ]';
  var dst = new F32x( [ 4, 7 ] );
  var clamp = [ 3, 6 ];
  var got = _.ointerval.clamp( dst, clamp );
  test.identical( got, new F32x( [ 4, 6 ] ) );
  test.true( got === dst );

  test.case = 'dst[ 0 ] > clamp[ 1 ], dst[ 1 ] > clamp[ 1 ]';
  var dst = new F64x( [ 8, 7 ] );
  var clamp = [ 3, 6 ];
  var got = _.ointerval.clamp( dst, clamp );
  test.identical( got, new F64x( [ 6, 6 ] ) );
  test.true( got === dst );

  test.case = 'dst[ 0 ] < clamp[ 0 ], dst[ 1 ] > clamp[ 1 ]';
  var dst = new U8x( [ 2, 7 ] );
  var clamp = [ 3, 6 ];
  var got = _.ointerval.clamp( dst, clamp );
  test.identical( got, new U8x( [ 3, 6 ] ) );
  test.true( got === dst );

  test.close( 'bufferTyped' );

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.ointerval.clamp() );

  test.case = 'not enough arguments';
  test.shouldThrowErrorSync( () => _.ointerval.clamp( [ 1, 2 ] ) );

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.ointerval.clamp( [ 1, 2 ], [ 3, 4 ], [ 5, 6 ] ) );

  test.case = 'dstRange is not a ointerval';
  test.shouldThrowErrorSync( () => _.ointerval.clamp( [ 1, 2, 3 ], [ 3, 4 ] ) );
  test.shouldThrowErrorSync( () => _.ointerval.clamp( [ 1, 'wrong' ], [ 3, 4 ] ) );
  test.shouldThrowErrorSync( () => _.ointerval.clamp( [ undefined, 1 ], [ 3, 4 ] ) );

  test.case = 'clampRange is not a ointerval';
  test.shouldThrowErrorSync( () => _.ointerval.clamp( [ 3, 4 ], [ 1, 2, 3 ] ) );
  test.shouldThrowErrorSync( () => _.ointerval.clamp( [ 3, 4 ], [ 1, 'wrong' ] ) );
  test.shouldThrowErrorSync( () => _.ointerval.clamp( [ 3, 4 ], [ undefined, 1 ] ) );
}

//

function ointervalCountElements( test )
{
  test.case = 'increment - zero, positive substruction result';
  var got = _.ointerval.countElements( [ 1, 6 ], 0 );
  test.identical( got, 0 );

  test.case = 'increment - zero, negative substruction result';
  var got = _.ointerval.countElements( [ 6, 1 ], 0 );
  test.identical( got, 0 );

  test.case = 'increment - zero, zero substruction result';
  var got = _.ointerval.countElements( [ 5, 5 ], 0 );
  test.identical( got, 0 );

  /* */

  test.case = 'increment - 1, positive substruction result';
  var got = _.ointerval.countElements( [ 1, 6 ], 1 );
  test.identical( got, 5 );

  test.case = 'increment - 1, negative substruction result';
  var got = _.ointerval.countElements( [ 6, 1 ], 1 );
  test.identical( got, -5 );

  test.case = 'increment - 1, zero substruction result';
  var got = _.ointerval.countElements( [ 5, 5 ], 1 );
  test.identical( got, 0 );

  /* */

  test.case = 'increment - undefined, positive substruction result';
  var got = _.ointerval.countElements( [ 1, 6 ], undefined );
  test.identical( got, 5 );

  test.case = 'increment - undefined, negative substruction result';
  var got = _.ointerval.countElements( [ 6, 1 ], undefined );
  test.identical( got, -5 );

  test.case = 'increment - undefined, zero substruction result';
  var got = _.ointerval.countElements( [ 5, 5 ], undefined );
  test.identical( got, 0 );

  /* */

  test.case = 'increment - not passed, positive substruction result';
  var got = _.ointerval.countElements( [ 1, 6 ] );
  test.identical( got, 5 );

  test.case = 'increment - not passed, negative substruction result';
  var got = _.ointerval.countElements( [ 6, 1 ] );
  test.identical( got, -5 );

  test.case = 'increment - not passed, zero substruction result';
  var got = _.ointerval.countElements( [ 5, 5 ] );
  test.identical( got, 0 );

  /* */

  test.case = 'increment - -1, positive substruction result';
  var got = _.ointerval.countElements( [ 1, 6 ], -1 );
  test.identical( got, -5 );

  test.case = 'increment - -1, negative substruction result';
  var got = _.ointerval.countElements( [ 6, 1 ], -1 );
  test.identical( got, 5 );

  test.case = 'increment - -1, zero substruction result';
  var got = _.ointerval.countElements( [ 5, 5 ], -1 );
  test.identical( got, 0 );

  /* */

  test.case = 'increment - 5, positive substruction result';
  var got = _.ointerval.countElements( [ 1, 6 ], 5 );
  test.identical( got, 1 );

  test.case = 'increment - 5, negative substruction result';
  var got = _.ointerval.countElements( [ 6, 1 ], 5 );
  test.identical( got, -1 );

  test.case = 'increment - 5, zero substruction result';
  var got = _.ointerval.countElements( [ 5, 5 ], 5 );
  test.identical( got, 0 );

  /* */

  test.case = 'increment - -5, positive substruction result';
  var got = _.ointerval.countElements( [ 1, 6 ], -5 );
  test.identical( got, -1 );

  test.case = 'increment - -5, negative substruction result';
  var got = _.ointerval.countElements( [ 6, 1 ], -5 );
  test.identical( got, 1 );

  test.case = 'increment - 5, zero substruction result';
  var got = _.ointerval.countElements( [ 5, 5 ], -5 );
  test.identical( got, 0 );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.ointerval.countElements() );

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.ointerval.countElements( [ 1, 2 ], 1, 'extra' ) );

  test.case = 'ointerval is not a ointerval';
  test.shouldThrowErrorSync( () => _.ointerval.countElements( [ 1, 2, 3 ], [ 3, 4 ] ) );
  test.shouldThrowErrorSync( () => _.ointerval.countElements( [ 1, 'wrong' ], [ 3, 4 ] ) );
  test.shouldThrowErrorSync( () => _.ointerval.countElements( [ undefined, 1 ], [ 3, 4 ] ) );

  test.case = 'increment is not a number and not undefined';
  test.shouldThrowErrorSync( () => _.ointerval.countElements( [ 1, 2 ], 'wrong' ) );
}

//

function ointervalLastGet( test )
{
  test.open( 'ointerval - array' );

  test.case = 'without options';
  var got = _.ointerval.lastGet( [ 1, 2 ] );
  test.identical( got, 1 );

  test.case = 'options - undefined';
  var options = undefined;
  var got = _.ointerval.lastGet( [ 1, 2 ], options );
  test.identical( got, 1 );
  test.identical( options, undefined );

  test.case = 'options - empty map';
  var options = {};
  var got = _.ointerval.lastGet( [ 1, 2 ], options );
  test.identical( got, 1 );
  test.identical( options, { increment : 1 } );

  test.case = 'options - map with own field increment';
  var options = { increment : 2 };
  var got = _.ointerval.lastGet( [ 1, 2 ], options );
  test.identical( got, 1 );
  test.identical( options, { increment : 2 } );

  test.case = 'options - map with own field increment - undefined';
  var options = { increment : undefined };
  var got = _.ointerval.lastGet( [ 1, 2 ], options );
  test.identical( got, 1 );
  test.identical( options, { increment : 1 } );

  test.close( 'ointerval - array' );

  /* - */

  test.open( 'ointerval - unroll' );

  test.case = 'without options';
  var got = _.ointerval.lastGet( _.unroll.make( [ 1, 2 ] ) );
  test.identical( got, 1 );

  test.case = 'options - undefined';
  var options = undefined;
  var got = _.ointerval.lastGet( _.unroll.make( [ 1, 2 ] ), options );
  test.identical( got, 1 );
  test.identical( options, undefined );

  test.case = 'options - empty map';
  var options = {};
  var got = _.ointerval.lastGet( _.unroll.make( [ 1, 2 ] ), options );
  test.identical( got, 1 );
  test.identical( options, { increment : 1 } );

  test.case = 'options - map with own field increment';
  var options = { increment : 2 };
  var got = _.ointerval.lastGet( _.unroll.make( [ 1, 2 ] ), options );
  test.identical( got, 1 );
  test.identical( options, { increment : 2 } );

  test.case = 'options - map with own field increment - undefined';
  var options = { increment : undefined };
  var got = _.ointerval.lastGet( _.unroll.make( [ 1, 2 ] ), options );
  test.identical( got, 1 );
  test.identical( options, { increment : 1 } );

  test.close( 'ointerval - unroll' );

  /* - */

  test.open( 'ointerval - argumentsArray' );

  test.case = 'without options';
  var got = _.ointerval.lastGet( _.argumentsArray.make( [ 1, 2 ] ) );
  test.identical( got, 1 );

  test.case = 'options - undefined';
  var options = undefined;
  var got = _.ointerval.lastGet( _.argumentsArray.make( [ 1, 2 ] ), options );
  test.identical( got, 1 );
  test.identical( options, undefined );

  test.case = 'options - empty map';
  var options = {};
  var got = _.ointerval.lastGet( _.argumentsArray.make( [ 1, 2 ] ), options );
  test.identical( got, 1 );
  test.identical( options, { increment : 1 } );

  test.case = 'options - map with own field increment';
  var options = { increment : 2 };
  var got = _.ointerval.lastGet( _.argumentsArray.make( [ 1, 2 ] ), options );
  test.identical( got, 1 );
  test.identical( options, { increment : 2 } );

  test.case = 'options - map with own field increment - undefined';
  var options = { increment : undefined };
  var got = _.ointerval.lastGet( _.argumentsArray.make( [ 1, 2 ] ), options );
  test.identical( got, 1 );
  test.identical( options, { increment : 1 } );

  test.close( 'ointerval - argumentsArray' );

  /* - */

  test.open( 'ointerval - BufferTyped' );

  test.case = 'without options';
  var got = _.ointerval.lastGet( new U8x( [ 1, 2 ] ) );
  test.identical( got, 1 );

  test.case = 'options - undefined';
  var options = undefined;
  var got = _.ointerval.lastGet( new I16x( [ 1, 2 ] ), options );
  test.identical( got, 1 );
  test.identical( options, undefined );

  test.case = 'options - empty map';
  var options = {};
  var got = _.ointerval.lastGet( new F32x( [ 1, 2 ] ), options );
  test.identical( got, 1 );
  test.identical( options, { increment : 1 } );

  test.case = 'options - map with own field increment';
  var options = { increment : 2 };
  var got = _.ointerval.lastGet( new F64x( [ 1, 2 ] ), options );
  test.identical( got, 1 );
  test.identical( options, { increment : 2 } );

  test.case = 'options - map with own field increment - undefined';
  var options = { increment : undefined };
  var got = _.ointerval.lastGet( new I8x( [ 1, 2 ] ), options );
  test.identical( got, 1 );
  test.identical( options, { increment : 1 } );

  test.close( 'ointerval - BufferTyped' );

  /* - */

  test.open( 'ointerval - map' );

  test.case = 'without options, last - undefined';
  var got = _.ointerval.lastGet( { first : undefined, last : undefined } );
  test.identical( got, undefined );

  test.case = 'without options, last - not exists';
  var got = _.ointerval.lastGet( { first : undefined } );
  test.identical( got, undefined );

  test.case = 'without options, last - number';
  var got = _.ointerval.lastGet( { first : 1, last : 2 } );
  test.identical( got, 2 );

  /* */

  test.case = 'options - undefined, last - undefined';
  var options = undefined;
  var got = _.ointerval.lastGet( { first : undefined, last : undefined }, options );
  test.identical( got, undefined );
  test.identical( options, undefined );

  test.case = 'options - undefined, last - not exists';
  var options = undefined;
  var got = _.ointerval.lastGet( { first : undefined }, options );
  test.identical( got, undefined );
  test.identical( options, undefined );

  test.case = 'options - undefined, last - number';
  var options = undefined;
  var got = _.ointerval.lastGet( { first : 1, last : 2 }, options );
  test.identical( got, 2 );
  test.identical( options, undefined );

  /* */

  test.case = 'options - empty map, last - undefined';
  var options = {};
  var got = _.ointerval.lastGet( { first : undefined, last : undefined }, options );
  test.identical( got, undefined );
  test.identical( options, { increment : 1 } );

  test.case = 'options - empty map, last - not exists';
  var options = {};
  var got = _.ointerval.lastGet( { first : undefined }, options );
  test.identical( got, undefined );
  test.identical( options, { increment : 1 } );

  test.case = 'options - empty map, last - number';
  var options = {};
  var got = _.ointerval.lastGet( { first : 1, last : 2 }, options );
  test.identical( got, 2 );
  test.identical( options, { increment : 1 } );

  /* */

  test.case = 'options - map with own field increment, last - undefined';
  var options = { increment : 2 };
  var got = _.ointerval.lastGet( { first : undefined, last : undefined }, options );
  test.identical( got, undefined );
  test.identical( options, { increment : 2 } );

  test.case = 'options - map with own field increment, last - not exists';
  var options = { increment : 2 };
  var got = _.ointerval.lastGet( { first : undefined }, options );
  test.identical( got, undefined );
  test.identical( options, { increment : 2 } );

  test.case = 'options - map with own field increment, last - number';
  var options = { increment : 2 };
  var got = _.ointerval.lastGet( { first : 1, last : 2 }, options );
  test.identical( got, 2 );
  test.identical( options, { increment : 2 } );

  /* */

  test.case = 'options - map with own field increment - undefined, last - undefined';
  var options = { increment : undefined };
  var got = _.ointerval.lastGet( { first : undefined, last : undefined }, options );
  test.identical( got, undefined );
  test.identical( options, { increment : 1 } );

  test.case = 'options - map with own field increment - undefined, last - not exists';
  var options = { increment : undefined };
  var got = _.ointerval.lastGet( { first : undefined }, options );
  test.identical( got, undefined );
  test.identical( options, { increment : 1 } );

  test.case = 'options - map with own field increment - undefined, last - number';
  var options = { increment : undefined };
  var got = _.ointerval.lastGet( { first : 1, last : 2 }, options );
  test.identical( got, 2 );
  test.identical( options, { increment : 1 } );

  test.close( 'ointerval - map' );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.ointerval.lastGet() );

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.ointerval.lastGet( [ 1, 2 ], {}, 'extra' ) );

  test.case = 'ointerval is a long, but is not a ointerval';
  test.shouldThrowErrorSync( () => _.ointerval.lastGet( [ 1, 2, 3 ] ) );
  test.shouldThrowErrorSync( () => _.ointerval.lastGet( [ 1, 'wrong' ] ) );
  test.shouldThrowErrorSync( () => _.ointerval.lastGet( [ undefined, 1 ] ) );

  test.case = 'wrong type of ointerval';
  test.shouldThrowErrorSync( () => _.ointerval.lastGet( new Set( [ 1, 2 ] ) ) );
  test.shouldThrowErrorSync( () => _.ointerval.lastGet( new HashMap( [ [ 1, 2 ] ] ) ) );
}

// --
// declaration
// --

const Proto =
{

  name : 'Tools.Interval.l0.l9',
  silencing : 1,
  enabled : 1,

  tests :
  {

    // l0/l3/Interval.s

    intervalIs,
    intervalIsValid,

    // common

    is,
    isValid,

    fromLeft,
    fromRight, /* aaa : fix */ /* Dmytro : fixed */

    firstGet,
    toStr,

    // cinterval

    cintervalIsEmpty,
    cintervalIsPopulated,
    // cintervalInInclusive,
    // cintervalInExclusive,
    // cintervalInInclusiveLeft,
    // cintervalInInclusiveRight,
    /* aaa2 for Dmytro : test for routine _.cinterval.has() */ /* Dmytro : added, improved routine */
    cintervalHas,
    cintervalSureInRange,
    cintervalAssertInRange,

    cintervalFromSingle,
    cintervalClamp,
    cintervalCountElements,
    cintervalLastGet,

    // linterval

    lintervalIsEmpty,
    lintervalIsPopulated,
    // lintervalInInclusive,
    // lintervalInExclusive,
    // lintervalInInclusiveLeft,
    // lintervalInInclusiveRight,
    /* aaa2 for Dmytro : test for routine _.linterval.has() */ /* Dmytro : added, improved routine */
    lintervalHas,
    lintervalSureInRange,
    lintervalAssertInRange,

    lintervalFromSingle,
    lintervalClamp,
    lintervalCountElements,
    lintervalLastGet,

    // ointerval

    ointervalIsEmpty,
    ointervalIsPopulated,
    // ointervalInInclusive,
    // ointervalInExclusive,
    // ointervalInInclusiveLeft,
    // ointervalInInclusiveRight,
    /* aaa2 for Dmytro : test for routine _.ointerval.has() */ /* Dmytro : added, improved routine */
    ointervalHas,
    ointervalSureInRange,
    ointervalAssertInRange,

    ointervalFromSingle,
    ointervalClamp,
    ointervalCountElements,
    ointervalLastGet,

  }

}

const Self = wTestSuite( Proto );
if( typeof module !== 'undefined' && !module.parent )
wTester.test( Self.name );

})();
