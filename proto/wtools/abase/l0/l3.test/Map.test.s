( function _l0_l3_Map_test_s()
{

'use strict';

if( typeof module !== 'undefined' )
{
  const _ = require( '../Include3.s' );
  require( 'wTesting' );
}

const _global = _global_;
const _ = _global_.wTools;
const __ = _globals_.testing.wTools;

// --
//
// --

function exportStringDiagnosticShallow( test )
{

  test.case = 'pure empty map';
  var src = Object.create( null );
  var expected = '{- Map.pure with 0 elements -}';
  var got = _.map.exportStringDiagnosticShallow( src );
  test.identical( got, expected );

  test.case = 'pure map';
  var src = Object.create( null );
  var expected = '{- Map.pure with 1 elements -}';
  src.x = 1;
  var got = _.map.exportStringDiagnosticShallow( src );
  test.identical( got, expected );

  test.case = 'empty polluted map';
  var src = {};
  var expected = '{- Map.polluted with 0 elements -}';
  var got = _.map.exportStringDiagnosticShallow( src );
  test.identical( got, expected );

  test.case = 'polluted map';
  var src = { a : 7, b : 13 };
  var expected = '{- Map.polluted with 2 elements -}';
  var got = _.map.exportStringDiagnosticShallow( src );
  test.identical( got, expected );

  // test.case = 'prototyped from pure map';
  // var prototype = Object.create( null );
  // var src = Object.create( prototype );
  // var expected = '{- Aux.pure.prototyped with 0 elements -}';
  // var got = _.map.exportStringDiagnosticShallow( src );
  // test.identical( got, expected );
  //
  // test.case = 'prototyped from pure map deep';
  // var prototype1 = Object.create( null );
  // var prototype2 = Object.create( prototype1 );
  // var src = Object.create( prototype1 );
  // var expected = '{- Aux.pure.prototyped with 0 elements -}';
  // var got = _.map.exportStringDiagnosticShallow( src );
  // test.identical( got, expected );
  //
  // test.case = 'prototyped from pure map deep with props';
  // var prototype1 = Object.create( null );
  // prototype1.a = 1;
  // var prototype2 = Object.create( prototype1 );
  // prototype2.b = 1;
  // var src = Object.create( prototype1 );
  // src.c = 1;
  // var expected = '{- Aux.pure.prototyped with 2 elements -}';
  // var got = _.map.exportStringDiagnosticShallow( src );
  // test.identical( got, expected );
  //
  // test.case = 'prototyped from polluted map';
  // var prototype = {};
  // var src = Object.create( prototype );
  // var expected = '{- Aux.polluted.prototyped with 0 elements -}';
  // var got = _.map.exportStringDiagnosticShallow( src );
  // test.identical( got, expected );
  //
  // test.case = 'prototyped from polluted map deep';
  // var prototype1 = {};
  // var prototype2 = Object.create( prototype1 );
  // var src = Object.create( prototype1 );
  // var expected = '{- Aux.polluted.prototyped with 0 elements -}';
  // var got = _.map.exportStringDiagnosticShallow( src );
  // test.identical( got, expected );
  //
  // test.case = 'prototyped from polluted map deep with props';
  // var prototype1 = {};
  // prototype1.a = 1;
  // var prototype2 = Object.create( prototype1 );
  // prototype2.b = 1;
  // var src = Object.create( prototype1 );
  // src.c = 1;
  // var expected = '{- Aux.polluted.prototyped with 2 elements -}';
  // var got = _.map.exportStringDiagnosticShallow( src );
  // test.identical( got, expected );

  if( !Config.debug )
  return;

  test.case = 'no args'
  test.shouldThrowErrorSync( () => _.map.exportStringDiagnosticShallow() );

  test.case = 'redundant args'
  test.shouldThrowErrorSync( () => _.map.exportStringDiagnosticShallow( {}, 13 ) );

  test.case = 'not aux'
  test.shouldThrowErrorSync( () => _.map.exportStringDiagnosticShallow( [] ) );
}

exportStringDiagnosticShallow.timeOut = 10000;

//

function identicalShallow( test )
{

  test.case = 'same map';
  var src = { a : 1 }
  var got = _.map.identicalShallow( src, src );
  test.identical( got, true );

  test.case = 'same values';
  var got = _.map.identicalShallow( { a : 7, b : 13 }, { a : 7, b : 13 } );
  var expected = true;
  test.identical( got, expected );

  test.case = 'same values with nested objects';
  var got = _.map.identicalShallow( { a : 7, b : { c : 13 } }, { a : 7, b : { c : 13 } } );
  var expected = false;
  test.identical( got, expected );

  test.case = 'not the same values in'
  var got = _.map.identicalShallow( { 'a' : 7, 'b' : 13 }, { 'a' : 7, 'b' : 14 } );
  var expected = false;
  test.identical( got, expected );

  test.case = 'different number of keys, more in the first argument'
  var got = _.map.identicalShallow( { 'a' : 7, 'b' : 13, 'с' : 15 }, { 'a' : 7, 'b' : 13 } );
  var expected = false;
  test.identical( got, expected );

  test.case = 'different number of keys, more in the second argument'
  var got = _.map.identicalShallow( { 'a' : 7, 'b' : 13 }, { 'a' : 7, 'b' : 13, 'с' : 15 } );
  var expected = false;
  test.identical( got, expected );

  test.case = 'empty maps, standrard'
  var got = _.map.identicalShallow( {}, {} );
  var expected = true;
  test.identical( got, expected );

  test.case = 'empty maps, pure'
  var got = _.map.identicalShallow( Object.create( null ), Object.create( null ) );
  var expected = true;
  test.identical( got, expected );

  test.case = 'empty maps, one standard another pure'
  var got = _.map.identicalShallow( {}, Object.create( null ) );
  var expected = true;
  test.identical( got, expected );

  test.case = 'not object-like 1 arg';
  var got = _.map.identicalShallow( [ 'a', 7, 'b', 14 ], { a : 7, b : 14 } );
  var expected = false;
  test.identical( got, expected );
  var got = _.map.identicalShallow( { a : 7, b : 14 }, [ 'a', 7, 'b', 14 ] );
  var expected = false;
  test.identical( got, expected );

  test.case = 'not object-like arguments';
  var got = _.map.identicalShallow( [ 'a', 7, 'b', 14 ], [ 'a', 7, 'b', 14 ] );
  var expected = false;
  test.identical( got, expected );

  test.case = 'same values';
  var got = _.map.identicalShallow( { a : 7, b : 13 }, { a : 7, b : 13 } );
  var expected = true;
  test.identical( got, expected );

  test.case = 'not the same values in'
  var got = _.map.identicalShallow( { 'a' : 7, 'b' : 13 }, { 'a' : 7, 'b' : 14 } );
  var expected = false;
  test.identical( got, expected );

  test.case = 'different number of keys, more in the first argument'
  var got = _.map.identicalShallow( { 'a' : 7, 'b' : 13, 'с' : 15 }, { 'a' : 7, 'b' : 13 } );
  var expected = false;
  test.identical( got, expected );

  test.case = 'different number of keys, more in the second argument'
  var got = _.map.identicalShallow( { 'a' : 7, 'b' : 13 }, { 'a' : 7, 'b' : 13, 'с' : 15 } );
  var expected = false;
  test.identical( got, expected );

  /* special cases */

  test.case = 'empty maps, standrard'
  var got = _.map.identicalShallow( {}, {} );
  var expected = true;
  test.identical( got, expected );

  test.case = 'empty maps, pure'
  var got = _.map.identicalShallow( Object.create( null ), Object.create( null ) );
  var expected = true;
  test.identical( got, expected );

  test.case = 'empty maps, one standard another pure'
  var got = _.map.identicalShallow( {}, Object.create( null ) );
  var expected = true;
  test.identical( got, expected );

  test.case = 'strings'
  var got = _.map.identicalShallow( 'a', 'a' );
  test.identical( got, false );

  test.case = 'map and not map';
  var got = _.map.identicalShallow( { 'a' : 1 }, 'b' );
  test.identical( got, false );

  test.case = 'nulls';
  var got = _.map.identicalShallow( null, null );
  test.identical( got, false );

  test.case = 'undefined';
  var got = _.map.identicalShallow( undefined, undefined );
  test.identical( got, false );

  /* bad arguments */

  if( !Config.debug )
  return;

  test.case = 'no arguments';
  test.shouldThrowErrorSync( function()
  {
    _.map.identicalShallow();
  });

  test.case = 'no arguments';
  test.shouldThrowErrorSync( function()
  {
    _.map.identicalShallow( {} );
  });

  test.case = 'redundant arguments';
  test.shouldThrowErrorSync( function()
  {
    _.map.identicalShallow( {}, {}, {}, 'redundant argument' );
  });

  test.case = 'no arguments';
  test.shouldThrowErrorSync( function()
  {
    _.map.identicalShallow();
  });

  test.case = 'not object-like arguments';
  // test.shouldThrowErrorSync( function() /* qqq : for Dmytro : need to investigate, two different namespaces */
  // {
  //   _.map.identicalShallow( [ 'a', 7, 'b', 13 ], [ 'a', 7, 'b', 14 ] );
  // });
  // test.shouldThrowErrorSync( function()
  // {
  //   _.map.identicalShallow( 'a', 'b' );
  // });
  // test.shouldThrowErrorSync( function()
  // {
  //   _.map.identicalShallow( { 'a' : 1 }, 'b' );
  // });
  // test.shouldThrowErrorSync( function()
  // {
  //   _.map.identicalShallow( 1, 3 );
  // });
  // test.shouldThrowErrorSync( function()
  // {
  //   _.map.identicalShallow( null, null );
  // });
  // test.shouldThrowErrorSync( function()
  // {
  //   _.map.identicalShallow( undefined, undefined );
  // });

  test.case = 'too few arguments';
  test.shouldThrowErrorSync( function()
  {
    _.map.identicalShallow( {} );
  });
}

//

function contain( test )
{

  test.case = 'first has same keys like second';
  var got = _.map.contain( { a : 7, b : 13, c : 15 }, { a : 7, b : 13 } );
  var expected = true;
  test.identical( got, expected );

  test.case = 'in the array';
  var got = _.map.contain( [ 'a', 7, 'b', 13, 'c', 15 ], [ 'a', 7, 'b', 13 ] );
  var expected = false;
  test.identical( got, expected );

  test.case = 'number of keys in first not equal';
  var got = _.map.contain( { a : 1 }, { a : 1, b : 2 } );
  var expected = true;
  test.identical( got, expected );

  test.case = 'strings'
  var got = _.map.contain( 'a', 'a' );
  test.identical( got, false );

  test.case = 'map and not map';
  var got = _.map.contain( { 'a' : 1 }, 'b' );
  test.identical( got, false );

  test.case = 'nulls';
  var got = _.map.contain( null, null );
  test.identical( got, false );

  test.case = 'undefined';
  var got = _.map.contain( undefined, undefined );
  test.identical( got, false );

  /* */

  if( !Config.debug )
  return;

  test.case = 'no arguments';
  test.shouldThrowErrorSync( function()
  {
    _.map.contain();
  });

  test.case = 'few arguments';
  test.shouldThrowErrorSync( function()
  {
    _.map.contain( {} );
  });

  test.case = 'too many arguments';
  test.shouldThrowErrorSync( function()
  {
    _.map.contain( {}, {}, 'redundant argument' );
  });

}

//

function aptLeft( test )
{

  /* */

  test.case = 'left';
  var src = { a : 1, b : 2, c : 3 };
  var ops = [];
  var got = _.map.aptLeft( src, function( val )
  {
    ops.push( Array.prototype.slice.call( arguments ) );
    if( val === 1 )
    return val + 10;
  });
  var exp = [ 11, 'a', 0, true ];
  test.identical( got, exp );
  var exp =
  [
    [
      1,
      'a',
      0,
      src,
    ],
  ]
  test.identical( ops, exp );

  /* */

  test.case = 'middle';
  var src = { a : 1, b : 2, c : 3 };
  var ops = [];
  var got = _.map.aptLeft( src, function( val )
  {
    ops.push( Array.prototype.slice.call( arguments ) );
    if( val === 2 )
    return val + 10;
  });
  var exp = [ 12, 'b', 1, true ];
  test.identical( got, exp );
  var exp =
  [
    [
      1,
      'a',
      0,
      src,
    ],
    [
      2,
      'b',
      1,
      src,
    ]
  ]
  test.identical( ops, exp );

  /* */

  test.case = 'right';
  var src = { a : 1, b : 2, c : 3 };
  var ops = [];
  var got = _.map.aptLeft( src, function( val )
  {
    ops.push( Array.prototype.slice.call( arguments ) );
    if( val === 3 )
    return val + 10;
  });
  var exp = [ 13, 'c', 2, true ];
  test.identical( got, exp );
  var exp =
  [
    [
      1,
      'a',
      0,
      src,
    ],
    [
      2,
      'b',
      1,
      src,
    ],
    [
      3,
      'c',
      2,
      src,
    ]
  ]
  test.identical( ops, exp );

  /* */

  test.case = 'no';
  var src = { a : 1, b : 2, c : 3 };
  var ops = [];
  var got = _.map.aptLeft( src, function( val )
  {
    ops.push( Array.prototype.slice.call( arguments ) );
    if( val === 4 )
    return val + 10;
  });
  var exp = [ 3, 'c', 2, false ];
  test.identical( got, exp );
  var exp =
  [
    [
      1,
      'a',
      0,
      src,
    ],
    [
      2,
      'b',
      1,
      src,
    ],
    [
      3,
      'c',
      2,
      src,
    ]
  ]
  test.identical( ops, exp );

  /* */

  test.case = 'empty';
  var src = Object.create( null );
  var ops = [];
  var got = _.map.aptLeft( src, function( val )
  {
    ops.push( Array.prototype.slice.call( arguments ) );
    if( val === 4 )
    return val + 10;
  });
  var exp = [ undefined, undefined, -1, false ];
  test.identical( got, exp );
  var exp =
  [
  ]
  test.identical( ops, exp );

  /* */

  test.case = 'without callback';
  var src = { a : 1, b : 2, c : 3 };
  var exp = [ 1, 'a', 0, true ];
  var got = _.map.aptLeft( src );
  test.identical( got, exp );
  var exp = [ 1, 'a', 0, true ];
  var got = _.map.first( src );
  test.identical( got, exp );

  /* */

  test.case = 'without callback, empty';
  var src = Object.create( null );
  var exp = [ undefined, undefined, -1, false ];
  var got = _.map.aptLeft( src );
  test.identical( got, exp );
  var exp = [ undefined, undefined, -1, false ];
  var got = _.map.first( src );
  test.identical( got, exp );

  /* */

}

//

function aptRight( test )
{

  /* */

  test.case = 'left';
  var src = { a : 1, b : 2, c : 3 };
  var ops = [];
  debugger;
  var got = _.map.aptRight( src, function( val )
  {
    ops.push( Array.prototype.slice.call( arguments ) );
    if( val === 1 )
    return val + 10;
  });
  debugger;
  var exp = [ 11, 'a', 0, true ];
  test.identical( got, exp );
  var exp =
  [
    [
      3,
      'c',
      2,
      src,
    ],
    [
      2,
      'b',
      1,
      src,
    ],
    [
      1,
      'a',
      0,
      src,
    ]
  ]
  test.identical( ops, exp );

  /* */

  test.case = 'middle';
  var src = { a : 1, b : 2, c : 3 };
  var ops = [];
  var got = _.map.aptRight( src, function( val )
  {
    ops.push( Array.prototype.slice.call( arguments ) );
    if( val === 2 )
    return val + 10;
  });
  var exp = [ 12, 'b', 1, true ];
  test.identical( got, exp );
  var exp =
  [
    [
      3,
      'c',
      2,
      src,
    ],
    [
      2,
      'b',
      1,
      src,
    ]
  ]
  test.identical( ops, exp );

  /* */

  test.case = 'right';
  var src = { a : 1, b : 2, c : 3 };
  var ops = [];
  var got = _.map.aptRight( src, function( val )
  {
    ops.push( Array.prototype.slice.call( arguments ) );
    if( val === 3 )
    return val + 10;
  });
  var exp = [ 13, 'c', 2, true ];
  test.identical( got, exp );
  var exp =
  [
    [
      3,
      'c',
      2,
      src,
    ]
  ]
  test.identical( ops, exp );

  /* */

  test.case = 'no';
  var src = { a : 1, b : 2, c : 3 };
  var ops = [];
  var got = _.map.aptRight( src, function( val )
  {
    ops.push( Array.prototype.slice.call( arguments ) );
    if( val === 4 )
    return val + 10;
  });
  var exp = [ 1, 'a', 0, false ];
  test.identical( got, exp );
  var exp =
  [
    [
      3,
      'c',
      2,
      src,
    ],
    [
      2,
      'b',
      1,
      src,
    ],
    [
      1,
      'a',
      0,
      src,
    ],
  ]
  test.identical( ops, exp );

  /* */

  test.case = 'empty';
  var src = Object.create( null );
  var ops = [];
  var got = _.map.aptRight( src, function( val )
  {
    ops.push( Array.prototype.slice.call( arguments ) );
    if( val === 4 )
    return val + 10;
  });
  var exp = [ undefined, undefined, -1, false ];
  test.identical( got, exp );
  var exp =
  [
  ]
  test.identical( ops, exp );

  /* */

  test.case = 'without callback';
  var src = { a : 1, b : 2, c : 3 };
  var exp = [ 3, 'c', 2, true ];
  var got = _.map.aptRight( src );
  test.identical( got, exp );
  var exp = [ 3, 'c', 2, true ];
  var got = _.map.last( src );
  test.identical( got, exp );

  /* */

  test.case = 'without callback, empty';
  var src = Object.create( null );
  var exp = [ undefined, undefined, -1, false ];
  var got = _.map.aptRight( src );
  test.identical( got, exp );
  var exp = [ undefined, undefined, -1, false ];
  var got = _.map.last( src );
  test.identical( got, exp );

  /* */

}

//

function filterLeftBasic( test )
{

  dstEach({ method : 'filterWithoutEscapeLeft', escaping : 0 });
  dstEach({ method : 'filterWithEscapeLeft', escaping : 1 });

  /* - */

  function dstEach( env )
  {
    env.dst = 'src';
    act( env );
    env.dst = 'self';
    act( env );
    env.dst = 'null';
    act( env );
    env.dst = 'another';
    act( env );
  }

  /* - */

  function act( env )
  {

    /* */

    test.case = `${__.entity.exportStringSolo( env )}, basic`;
    var ops = [];
    var src = { a : 1, b : 2, c : 3 }
    var dst = dstFor( env, src );
    var got = _.map[ env.method ]( dst, src, function( val ) { ops.push( Array.prototype.slice.call( arguments ) ); return val + 10 } );
    /* xxx : qqq : implement and use _.long.toArray() */
    var exp = { 'a' : 11, 'b' : 12, 'c' : 13 };
    test.identical( got, exp );
    if( env.dst === 'self' )
    dst = src;
    if( env.dst === 'null' )
    test.true( got !== dst );
    else
    test.true( got === dst );
    var exp =
    [
      [
        1,
        'a',
        0,
        ( env.dst === 'self' || env.dst === 'src' ) ? { 'a' : 11, 'b' : 12, 'c' : 13 } : { 'a' : 1, 'b' : 2, 'c' : 3 },
        got,
      ],
      [
        2,
        'b',
        1,
        ( env.dst === 'self' || env.dst === 'src' ) ? { 'a' : 11, 'b' : 12, 'c' : 13 } : { 'a' : 1, 'b' : 2, 'c' : 3 },
        got,
      ],
      [
        3,
        'c',
        2,
        ( env.dst === 'self' || env.dst === 'src' ) ? { 'a' : 11, 'b' : 12, 'c' : 13 } : { 'a' : 1, 'b' : 2, 'c' : 3 },
        got,
      ],
    ]
    test.identical( ops, exp );

    /* */

  }

  /* - */

  function dstFor( env, src )
  {
    if( env.dst === 'src' )
    return src;
    if( env.dst === 'self' )
    return _.self;
    if( env.dst === 'null' )
    return null;
    if( env.dst === 'another' )
    return {};
    _.assert( 0 );
  }

  /* - */

}

//

function filterRightBasic( test )
{

  dstEach({ method : 'filterWithoutEscapeRight', escaping : 0 });
  dstEach({ method : 'filterWithEscapeRight', escaping : 1 });

  /* - */

  function dstEach( env )
  {
    env.dst = 'src';
    act( env );
    env.dst = 'self';
    act( env );
    env.dst = 'null';
    act( env );
    env.dst = 'another';
    act( env );
  }

  /* - */

  function act( env )
  {

    /* */

    test.case = `${__.entity.exportStringSolo( env )}, basic`;
    var ops = [];
    var src = { a : 1, b : 2, c : 3 }
    var dst = dstFor( env, src );
    var got = _.map[ env.method ]( dst, src, function( val ) { ops.push( Array.prototype.slice.call( arguments ) ); return val + 10 } );
    /* xxx : qqq : implement and use _.long.toArray() */
    var exp = { 'a' : 11, 'b' : 12, 'c' : 13 };
    test.identical( got, exp );
    if( env.dst === 'self' )
    dst = src;
    if( env.dst === 'null' )
    test.true( got !== dst );
    else
    test.true( got === dst );
    var exp =
    [
      [
        3,
        'c',
        2,
        ( env.dst === 'self' || env.dst === 'src' ) ? { 'a' : 11, 'b' : 12, 'c' : 13 } : { 'a' : 1, 'b' : 2, 'c' : 3 },
        got,
      ],
      [
        2,
        'b',
        1,
        ( env.dst === 'self' || env.dst === 'src' ) ? { 'a' : 11, 'b' : 12, 'c' : 13 } : { 'a' : 1, 'b' : 2, 'c' : 3 },
        got,
      ],
      [
        1,
        'a',
        0,
        ( env.dst === 'self' || env.dst === 'src' ) ? { 'a' : 11, 'b' : 12, 'c' : 13 } : { 'a' : 1, 'b' : 2, 'c' : 3 },
        got,
      ],
    ]
    test.identical( ops, exp );

    /* */

  }

  /* - */

  function dstFor( env, src )
  {
    if( env.dst === 'src' )
    return src;
    if( env.dst === 'self' )
    return _.self;
    if( env.dst === 'null' )
    return null;
    if( env.dst === 'another' )
    return {};
    _.assert( 0 );
  }

  /* - */

}

//

function filterBlackBox( test )
{

  dstEach({ method : 'filterWithoutEscapeLeft', escaping : 0 });
  dstEach({ method : 'filterWithoutEscapeRight', escaping : 0 });
  dstEach({ method : 'filterWithEscapeLeft', escaping : 1 });
  dstEach({ method : 'filterWithEscapeRight', escaping : 1 });

  /* - */

  function dstEach( env )
  {
    env.dst = 'src';
    act( env );
    env.dst = 'self';
    act( env );
    env.dst = 'null';
    act( env );
    env.dst = 'another';
    act( env );
  }

  /* - */

  function act( env )
  {

    /* */

    test.case = `${__.entity.exportStringSolo( env )}, basic`;
    var src = { a : 1, b : 2, c : 3 }
    var dst = dstFor( env, src );
    var got = _.map[ env.method ]( dst, src, function( val ) { return val + 10 } );
    var exp = { 'a' : 11, 'b' : 12, 'c' : 13 };
    test.identical( got, exp );
    if( env.dst === 'self' )
    dst = src;
    if( env.dst === 'null' )
    test.true( got !== dst );
    else
    test.true( got === dst );

    /* */

    test.case = `${__.entity.exportStringSolo( env )}, drop first`;
    var src = { a : 1, b : 2, c : 3 };
    var dst = dstFor( env, src );
    var got = _.map[ env.method ]( dst, src, function( val ) { if( val !== 1 ) return val + 10 } );
    var exp = { 'b' : 12, 'c' : 13 };
    test.identical( got, exp );
    if( env.dst === 'self' )
    dst = src;
    if( env.dst === 'null' )
    test.true( got !== dst );
    else
    test.true( got === dst );

    /* */

    test.case = `${__.entity.exportStringSolo( env )}, drop mid`;
    var src = { a : 1, b : 2, c : 3 };
    var dst = dstFor( env, src );
    var got = _.map[ env.method ]( dst, src, function( val ) { if( val !== 2 ) return val + 10 } );
    var exp = { 'a' : 11, 'c' : 13 };
    test.identical( got, exp );
    if( env.dst === 'self' )
    dst = src;
    if( env.dst === 'null' )
    test.true( got !== dst );
    else
    test.true( got === dst );

    /* */

    test.case = `${__.entity.exportStringSolo( env )}, drop last`;
    var src = { a : 1, b : 2, c : 3 };
    var dst = dstFor( env, src );
    var got = _.map[ env.method ]( dst, src, function( val ) { if( val !== 3 ) return val + 10 } );
    var exp = { 'a' : 11, 'b' : 12 };
    test.identical( got, exp );
    if( env.dst === 'self' )
    dst = src;
    if( env.dst === 'null' )
    test.true( got !== dst );
    else
    test.true( got === dst );

    /* */

    test.case = `${__.entity.exportStringSolo( env )}, val=undefined, leaving`;
    var src = { a : 1, b : undefined, c : 3 };
    var dst = dstFor( env, src );
    var got = _.map[ env.method ]( dst, src, function( val ) { return val } );
    var exp = { 'a' : 1, 'c' : 3 };
    test.identical( got, exp );
    if( env.dst === 'self' )
    dst = src;
    if( env.dst === 'null' )
    test.true( got !== dst );
    else
    test.true( got === dst );

    /* */

    test.case = `${__.entity.exportStringSolo( env )}, val=undefined, removing`;
    var src = { a : 1, b : undefined, c : 3 };
    var dst = dstFor( env, src );
    var got = _.map[ env.method ]( dst, src, function( val ) {} );
    var exp = {};
    test.identical( got, exp );
    if( env.dst === 'self' )
    dst = src;
    if( env.dst === 'null' )
    test.true( got !== dst );
    else
    test.true( got === dst );

    /* */

    test.case = `${__.entity.exportStringSolo( env )}, val=_.undefined, leaving`;
    var src = { a : 1, b : _.undefined, c : 3 };
    var dst = dstFor( env, src );
    var got = _.map[ env.method ]( dst, src, function( val ) { return val } );
    var exp = { 'a' : 1, 'b' : _.undefined, 'c' : 3 };
    if( env.escaping )
    exp.b = undefined;
    test.identical( got, exp );
    if( env.dst === 'self' )
    dst = src;
    if( env.dst === 'null' )
    test.true( got !== dst );
    else
    test.true( got === dst );

    /* */

    test.case = `${__.entity.exportStringSolo( env )}, val=_.undefined, removing`;
    var src = { a : 1, b : _.undefined, c : 3 };
    var dst = dstFor( env, src );
    var got = _.map[ env.method ]( dst, src, function( val ) {} );
    var exp = {};
    test.identical( got, exp );
    if( env.dst === 'self' )
    dst = src;
    if( env.dst === 'null' )
    test.true( got !== dst );
    else
    test.true( got === dst );

    /* */

    test.case = `${__.entity.exportStringSolo( env )}, wrapped`;
    var src = { a : _.escape.left( undefined ), b : _.escape.left( null ), c : _.escape.wrap( 3 ) };
    var dst = dstFor( env, src );
    var got = _.map[ env.method ]( dst, src, function( val ) { return val } );
    var exp = { a : _.escape.left( undefined ), b : _.escape.left( null ), c : _.escape.wrap( 3 ) };
    if( env.escaping )
    exp = { 'a' : undefined, 'b' : null, 'c' : 3 };
    test.identical( got, exp );
    if( env.dst === 'self' )
    dst = src;
    if( env.dst === 'null' )
    test.true( got !== dst );
    else
    test.true( got === dst );

    /* */

  }

  /* - */

  function dstFor( env, src )
  {
    if( env.dst === 'src' )
    return src;
    if( env.dst === 'self' )
    return _.self;
    if( env.dst === 'null' )
    return null;
    if( env.dst === 'another' )
    return {};
    _.assert( 0 );
  }

  /* - */

}

// --
// define test suite
// --

const Proto =
{

  name : 'Tools.Map.l0.l3',
  silencing : 1,

  tests :
  {

    exportStringDiagnosticShallow,
    identicalShallow,
    contain,

    aptLeft,
    aptRight,

    filterLeftBasic,
    filterRightBasic,
    filterBlackBox,

  }

}

const Self = wTestSuite( Proto );
if( typeof module !== 'undefined' && !module.parent )
wTester.test( Self.name );

})();
