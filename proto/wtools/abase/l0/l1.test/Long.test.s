( function _l0_l1_Long_test_s_()
{

'use strict';

if( typeof module !== 'undefined' )
{
  const _ = require( '../Include1.s' );
  require( 'wTesting' );
}

const _global = _global_;
const _ = _global_.wTools;
const __ = _globals_.testing.wTools;

//--
// long
//--

/* qqq : for junior : merge routines is and like to routine dichotomy and extend */
/* qqq2 : for Ragul : redo please */
function dichotomy( test )
{

  act({ method : 'is' });
  act({ method : 'isOld' });
  act({ method : 'isCompact' });
  act({ method : 'isUnfolded' });
  act({ method : 'isUnfoldedSmartOrder' });
  act({ method : 'isUnfoldedSmarter' });
  act({ method : 'like' });

  function act( env )
  {
    test.case = `${__.entity.exportStringSolo( env )}, an empty array`;
    var got = _.long[ env.method ]( [] );
    var expected = true;
    test.identical( got, expected );

    test.case = `${__.entity.exportStringSolo( env )}, an array`;
    var got = _.long[ env.method ]( [ 1, 2, 3 ] );
    var expected  = true;
    test.identical( got, expected );

    test.case = `${__.entity.exportStringSolo( env )}, a pseudo array`;
    var got = _.long[ env.method ]( arguments );
    var expected = true;
    test.identical( got, expected );

    test.case = `${__.entity.exportStringSolo( env )}, raw array buffer`;
    var got = _.long[ env.method ]( new BufferRaw( 10 ) );
    var expected = false;
    test.identical( got, expected );

    test.case = `${__.entity.exportStringSolo( env )}, typed array buffer`;
    var got = _.long[ env.method ]( new F32x( 10 ) );
    var expected = true;
    test.identical( got, expected );

    test.case = `${__.entity.exportStringSolo( env )}, no arguments`;
    var got = _.long[ env.method ]();
    var expected  = false;
    test.identical( got, expected );

    test.case = `${__.entity.exportStringSolo( env )}, null`;
    var got = _.long[ env.method ]( null );
    var expected  = false;
    test.identical( got, expected );

    test.case = `${__.entity.exportStringSolo( env )}, function`;
    var got = _.long[ env.method ]( function() {} );
    var expected  = false;
    test.identical( got, expected );

    test.case = `${__.entity.exportStringSolo( env )}, string`;
    var got = _.long[ env.method ]( 'x' );
    var expected  = false;
    test.identical( got, expected );

    test.case = `${__.entity.exportStringSolo( env )}, number`;
    var got = _.long[ env.method ]( 1 );
    var expected  = false;
    test.identical( got, expected );

    test.case = `${__.entity.exportStringSolo( env )}, boolean`;
    var got = _.long[ env.method ]( true );
    var expected  = false;
    test.identical( got, expected );

    test.case = `${__.entity.exportStringSolo( env )}, empty map`;
    var got = _.long[ env.method ]( {} );
    var expected  = false;
    test.identical( got, expected );

    test.case = `${__.entity.exportStringSolo( env )}, object with fields and iteraor method`;
    var src = new function()
    {
      this[ Symbol.iterator ] = function ()
      {
        return { next() { return { done : true } } }
      }
    }
    var got = _.long[ env.method ]( src );
    var expected  = false;
    test.identical( got, expected );

    test.case = 'Testing for overlaps';
    test.identical( _.arrayIs( src ), false );
    test.identical( _.argumentsArray.like( src ), false );
    test.identical( _.long.like( src ), false );
    test.identical( _.vector.is( src ), false );
    test.identical( _.vector.like( src ), false );
    test.identical( _.countable.is( src ), true );
    test.identical( _.countable.like( src ), true );
  }
}

//

function dichotomyPerformance( test )
{
   /*
    Average of 10 runs of 1 million iteration of 24 _.long.is variations
    Values below are in seconds
    ╔════════════════════════╤═════╤═════╤═════════╤══════════╤════════════════════╤═════════════════╗
    ║                        │  is │isOld│isCompact│isUnfolded│isUnfoldedSmartOrder│isUnfoldedSmarter║
    ╟────────────────────────┼─────┼─────┼─────────┼──────────┼────────────────────┼─────────────────╢
    ║Windows-10-21H1, 10.24.1│1.296│5.490│  1.973  │  1.907   │     1.756          │    1.190        ║
    ╟────────────────────────┼─────┼─────┼─────────┼──────────┼────────────────────┼─────────────────╢
    ║Windows-10-21H1, 14.17.0│1.189│6.644│  1.902  │  1.989   │     1.657          │    1.342        ║
    ╟────────────────────────┼─────┼─────┼─────────┼──────────┼────────────────────┼─────────────────╢
    ║    Linux-Kos, 12.9.1   │     │     │         │          │                    │                 ║
    ╚════════════════════════╧═════╧═════╧═════════╧══════════╧════════════════════╧═════════════════╝
    qqq : for Rahul : update the table please
  */

  debugger; /* eslint-disable-line no-debugger */
  var debugFlag = Config.debug;
  Config.debug = false;

  /* */

  act({ method : 'is' });
  act({ method : 'isOld' });
  act({ method : 'isCompact' });
  act({ method : 'isUnfolded' });
  act({ method : 'isUnfoldedSmartOrder' });
  act({ method : 'isUnfoldedSmarter' });

  /* */

  Config.debug = debugFlag;
  debugger; /* eslint-disable-line no-debugger */

  /* */

  function act( data )
  {
    test.case = `${data.method}`;
    var took, time;
    var env = initializeVariables();
    __.time.sleep( 100 );

    time = _.time.now();
    for( let i = env.times; i > 0; i-- )
    {
      env.name = data.method;
      run( env );
    }
    took = __.time.spent( time );

    console.log( `${env.times} iterations of ${test.case} took : ${took} on ${process.version}` );
    test.identical( true, true );
  }

  /* - */

  function initializeVariables()
  {
    var env = {};
    env.times = 1000000;
    env.nonEmptyArray = [ 1, 2, 3 ];
    env.rawBuffer = new BufferRaw( 10 );
    env.float32Array = new F32x( 10 );
    env.int32Array = new I32x( 10 );
    env.uint16Array = new U16x( 10 );
    env.uint8Array = new U8x( 10 );
    env.uint8ClampedArray = new U8xClamped( 10 );
    env.derivedArray = 'abc'.match( /[a-z]/g );
    env.arrayProtoType = Array.prototype;
    env.multiDimensionalArray = [ [ 'Divyanshu', '32' ], [ 'Shankar', '34' ], [ 'Rahul', '29' ] ];
    env.constructedArray = new Array( 3 );
    env.anEmptyRoutine = new function() {};
    env.aSymbol = Symbol( 'a' );
    env.aString = 'x';
    env.aNumber = 1;
    env.aBoolean = false;
    env.anEmptyObject = {};
    env.routine = new function()
    {
      this[ Symbol.iterator ] = function ()
      {
        return { next() { return { done : true } } }
      }
    }

    return env;
  }

  function run( env )
  {
    let r = [];
    r.push( _.long[ env.name ]( [] ) );
    r.push( _.long[ env.name ]( env.nonEmptyArray ) );
    r.push( _.long[ env.name ]( arguments ) );
    r.push( _.long[ env.name ]( env.float32Array ) );
    r.push( _.long[ env.name ]( env.int32Array ) );
    r.push( _.long[ env.name ]( env.uint16Array ) );
    r.push( _.long[ env.name ]( env.uint8Array ) );
    r.push( _.long[ env.name ]( env.uint8ClampedArray ) );
    r.push( _.long[ env.name ]( env.derivedArray ) );
    r.push( _.long[ env.name ]( env.arrayProtoType ) );
    r.push( _.long[ env.name ]( env.multiDimensionalArray ) );
    r.push( _.long[ env.name ]( env.constructedArray ) );
    r.push( _.long[ env.name ]( env.rawBuffer ) );
    r.push( _.long[ env.name ]() );
    r.push( _.long[ env.name ]( null ) );
    r.push( _.long[ env.name ]( undefined ) );
    r.push( _.long[ env.name ]( NaN ) );
    r.push( _.long[ env.name ]( new Date() ) );
    r.push( _.long[ env.name ]( env.anEmptyRoutine ) );
    r.push( _.long[ env.name ]( env.aString ) );
    r.push( _.long[ env.name ]( env.aNumber ) );
    r.push( _.long[ env.name ]( env.aBoolean ) );
    r.push( _.long[ env.name ]( env.anEmptyObject ) );
    r.push( _.long[ env.name ]( env.routine ) );

    return r;
  }
}

dichotomyPerformance.timeOut = 1e7;
dichotomyPerformance.experimental = true;

// --
//
// --

function makeCommon( test )
{

  methodEach({ tools : 'default', type : 'Array' });
  methodEach({ tools : 'Array', type : 'Array' });
  methodEach({ tools : 'F32x', type : 'F32x' });
  methodEach({ tools : 'bufferTyped', type : 'Array' });

  /* - */

  function methodEach( env )
  {
    env.method = 'make';
    act( env );
    if( env.tools !== 'bufferTyped' )
    {
      env.method = 'cloneShallow';
      act( env );
    }
  }

  /* */

  function act( env )
  {
    test.open( `${__.entity.exportStringSolo( env )}` );

    const long = namespaceGet( env );
    const Constructor = defaultConstructorGet( env );
    const InstanceConstructor = env.tools === 'bufferTyped' ? Constructor : Array;

    /* */

    if( env.method !== 'cloneShallow' )
    {
      test.case = `no args`;
      var got = long[ env.method ]();
      test.true( got instanceof Constructor );
      test.identical( got.length, 0 );

      test.case = `length`;
      var got = long[ env.method ]( 3 );
      test.true( got instanceof Constructor );
      test.true( got.length === 3 );
    }

    /* */

    test.case = `empty array`;
    var src = [];
    var got = long[ env.method ]( src );
    test.true( got instanceof InstanceConstructor );
    test.identical( got.length, 0 );

    test.case = `filled array`;
    var src = [ 2, 3 ];
    var got = long[ env.method ]( src );
    test.true( got instanceof InstanceConstructor );
    test.identical( got, InstanceConstructor.from([ 2, 3 ]) );

    /* */

    test.case = `empty unroll`;
    var src = _.unroll.make( [] );
    var got = long[ env.method ]( src );
    test.true( got instanceof InstanceConstructor );
    test.identical( got.length, 0 );

    test.case = `filled unroll`;
    var src = _.unroll.make([ 2, 3 ]);
    var got = long[ env.method ]( src );
    test.true( got instanceof InstanceConstructor );
    test.identical( got, InstanceConstructor.from([ 2, 3 ]) );

    /* */

    test.case = `empty argumentsArray`;
    var src = _.argumentsArray.make( [] );
    var got = long[ env.method ]( src );
    if( env.tools === 'bufferTyped' )
    test.true( got instanceof Constructor );
    else
    test.true( _.argumentsArray.is( got ) );
    test.identical( got.length, 0 );

    test.case = `filled argumentsArray`;
    var src = _.argumentsArray.make([ 2, 3 ]);
    var got = long[ env.method ]( src );
    if( env.tools === 'bufferTyped' )
    {
      test.true( got instanceof Constructor );
      test.identical( got, InstanceConstructor.from([ 2, 3 ]) );
    }
    else
    {
      test.true( _.argumentsArray.is( got ) );
      test.identical( got, _.argumentsArray.make([ 2, 3 ]) );
    }

    /* */

    test.case = `empty typed buffer`;
    var src = _.u8x.make( [] );
    var got = long[ env.method ]( src );
    test.true( got instanceof U8x );
    test.identical( got.length, 0 );

    test.case = `filled typed buffer`;
    var src = _.f32x.make([ 2, 3 ]);
    var got = long[ env.method ]( src );
    test.true( got instanceof F32x );
    test.identical( got, _.f32x.make([ 2, 3 ]) );

    /* */

    if( env.method !== 'cloneShallow' )
    {
      test.case = `filled countable`;
      var src = __.diagnostic.objectMake({ elements : [ 2, 3 ], countable : 1 });
      var got = long[ env.method ]( src );
      test.true( got instanceof Constructor );
      test.identical( got, Constructor.from([ 2, 3 ]) );

      /* */

      test.case = `null and length - number`;
      var got = long[ env.method ]( null, 2 );
      test.true( got instanceof Constructor );
      test.identical( got.length, 2 );

      test.case = `null and length - array`;
      var got = long[ env.method ]( null, [ 1, 2 ] );
      test.true( got instanceof Constructor );
      test.identical( got.length, 2 );

      test.case = `null and countable`;
      var length = __.diagnostic.objectMake({ elements : [ 1, 2, 3 ], countable : 1 });
      var got = long[ env.method ]( null, length );
      test.true( got instanceof Constructor );
      test.identical( got.length, 3 );

      /* */

      test.case = `empty array and length`;
      var src = [];
      var got = long[ env.method ]( src, 2 );
      test.true( got instanceof InstanceConstructor );
      test.identical( got.length, 2 );

      test.case = `empty unroll and length`;
      var src = _.unroll.make( [] );
      var got = long[ env.method ]( src, 2 );
      test.true( got instanceof InstanceConstructor );
      test.identical( got.length, 2 );

      test.case = `empty argumentsArray and length`;
      var src = _.argumentsArray.make( [] );
      var got = long[ env.method ]( src, 2 );
      if( env.tools === 'bufferTyped' )
      test.true( got instanceof Constructor );
      else
      test.true( _.argumentsArray.is( got ) );
      test.identical( got.length, 2 );

      test.case = `empty typed buffer and length`;
      var src = _.u8x.make( [] );
      var got = long[ env.method ]( src, 2 );
      test.true( got instanceof U8x );
      test.identical( got.length, 2 );

      /* */

      test.case = `non-empty array and length longer`;
      var src = [ 3, 4 ];
      var got = long[ env.method ]( src, 3 );
      test.true( got instanceof InstanceConstructor );
      var value = env.tools === 'bufferTyped' ? 0 : undefined;
      test.identical( got, InstanceConstructor.from([ 3, 4, value ]) );

      test.case = `non-empty array and length shorter`;
      var src = [ 3, 4 ];
      var got = long[ env.method ]( src, 1 );
      test.true( got instanceof InstanceConstructor );
      test.identical( got, InstanceConstructor.from([ 3 ]) );

      /* */

      test.case = `non-empty unroll and length longer`;
      var src = _.unroll.make([ 3, 4 ]);
      var got = long[ env.method ]( src, 3 );
      test.true( got instanceof InstanceConstructor );
      var value = env.tools === 'bufferTyped' ? 0 : undefined;
      test.identical( got, InstanceConstructor.from([ 3, 4, value ]) );

      test.case = `non-empty unroll and length shorter`;
      var src = _.unroll.make([ 3, 4 ]);
      var got = long[ env.method ]( src, 1 );
      test.true( got instanceof InstanceConstructor );
      test.identical( got, InstanceConstructor.from([ 3 ]) );

      /* */

      test.case = `non-empty argumentsArray and length longer`;
      var src = _.argumentsArray.make([ 3, 4 ]);
      var got = long[ env.method ]( src, 3 );
      if( env.tools === 'bufferTyped' )
      {
        test.true( got instanceof Constructor );
        test.identical( got, InstanceConstructor.from([ 3, 4, 0 ]) );
      }
      else
      {
        test.true( _.argumentsArray.is( got ) );
        test.identical( got, _.argumentsArray.make([ 3, 4, undefined ]) );
      }

      test.case = `non-empty argumentsArray and length shorter`;
      var src = _.argumentsArray.make([ 3, 4 ]);
      var got = long[ env.method ]( src, 1 );
      if( env.tools === 'bufferTyped' )
      {
        test.true( got instanceof Constructor );
        test.identical( got, InstanceConstructor.from([ 3 ]) );
      }
      else
      {
        test.true( _.argumentsArray.is( got ) );
        test.identical( got, _.argumentsArray.make([ 3 ]) );
      }

      /* */

      test.case = `non-empty typed buffer and length longer`;
      var src = _.u8x.make([ 3, 4 ]);
      var got = long[ env.method ]( src, 3 );
      test.true( got instanceof U8x );
      var value = env.tools === 'bufferTyped' ? 0 : undefined;
      test.identical( got, _.u8x.make([ 3, 4, value ]) );

      test.case = `non-empty typed buffer and length shorter`;
      var src = _.f32x.make([ 3, 4 ]);
      var got = long[ env.method ]( src, 1 );
      test.true( got instanceof F32x );
      test.identical( got, _.f32x.make([ 3 ]) );

      /* */

      test.case = `array and countable`;
      var length = __.diagnostic.objectMake({ elements : [ 1, 2, 3 ], countable : 1 });
      var got = long[ env.method ]( [ 0, 0, 0, 0 ], length );
      test.true( got instanceof InstanceConstructor );
      test.identical( got, InstanceConstructor.from([ 1, 2, 3 ]) );
    }

    /* */

    if( Config.debug )
    {
      if( env.method === 'cloneShallow' )
      {
        test.case = 'without arguments';
        test.shouldThrowErrorSync( () => long[ env.method ]() );

        test.case = 'extra arguments';
        test.shouldThrowErrorSync( () => long[ env.method ]( [], 1 ) );
      }
      else
      {
        test.case = 'extra arguments';
        test.shouldThrowErrorSync( () => long[ env.method ]( [], 1, 1 ) );

        test.case = 'wrong type of length';
        test.shouldThrowErrorSync( () => long[ env.method ]( [ 1, 2, 3 ], null ) );
        test.shouldThrowErrorSync( () => long[ env.method ]( 3, null ) );
        test.shouldThrowErrorSync( () => long[ env.method ]( Array, null ) );
        test.shouldThrowErrorSync( () => long[ env.method ]( null, null ) );
        test.shouldThrowErrorSync( () => long[ env.method ]( [ 1, 2, 3 ], undefined ) );
        test.shouldThrowErrorSync( () => long[ env.method ]( 3, undefined ) );
        test.shouldThrowErrorSync( () => long[ env.method ]( Array, undefined ) );
        test.shouldThrowErrorSync( () => long[ env.method ]( null, undefined ) );

        test.case = 'wrong type of src';
        test.shouldThrowErrorSync( () => long[ env.method ]( 'wrong', 3 ) );
      }

      test.case = 'wrong type of src';
      test.shouldThrowErrorSync( () => long[ env.method ]( undefined ) );
    }

    test.close( `${__.entity.exportStringSolo( env )}` );
  }

  function namespaceGet( env )
  {
    if( env.tools === 'default' )
    return _.long;
    if( env.tools === 'bufferTyped' )
    return _.bufferTyped;
    return _.withLong[ env.tools ].long;
  }

  /* */

  function defaultConstructorGet( env )
  {
    if( env.tools === 'default' )
    return _.long.default.InstanceConstructor;
    if( env.tools === 'bufferTyped' )
    return _.bufferTyped.default.InstanceConstructor;
    return _.withLong[ env.tools ].long.default.InstanceConstructor;
  }
}

//

function makeCommonWithLongDescriptor( test )
{
  let times = 4;
  for( let k in _.long.namespaces )
  {
    let namespace = _.long.namespaces[ k ];
    let type = namespace.TypeName;

    if( type === 'ArgumentsArray' )
    continue;

    test.open( `long - ${ type }` );
    methodEach({ tools : 'default', type });
    methodEach({ tools : 'Array', type });
    methodEach({ tools : 'F32x', type });
    methodEach({ tools : 'bufferTyped', type });
    test.close( `long - ${ type }` );

    if( times < 1 )
    break;
    times--;
  }

  /* - */

  function methodEach( env )
  {
    env.method = 'make';
    act( env );
    if( env.tools !== 'bufferTyped' )
    {
      env.method = 'cloneShallow';
      act( env );
    }
  }

  /* */

  function act( env )
  {
    test.open( `${__.entity.exportStringSolo( env )}` );

    const long = namespaceGet( env );
    const Constructor = defaultConstructorGet( env );
    const InstanceConstructor = env.tools === 'bufferTyped' ? Constructor : Array;

    /* */

    if( env.method !== 'cloneShallow' )
    {
      test.case = `no args`;
      var got = long[ env.method ]();
      test.true( got instanceof Constructor );
      test.identical( got.length, 0 );

      test.case = `length`;
      var got = long[ env.method ]( 3 );
      test.true( got instanceof Constructor );
      test.true( got.length === 3 );
    }

    /* */

    test.case = `empty array`;
    var src = [];
    var got = long[ env.method ]( src );
    test.true( got instanceof InstanceConstructor );
    test.identical( got.length, 0 );

    test.case = `filled array`;
    var src = [ 2, 3 ];
    var got = long[ env.method ]( src );
    test.true( got instanceof InstanceConstructor );
    test.identical( got.length, 2 );
    test.identical( got, InstanceConstructor.from([ 2, 3 ]) );

    /* */

    test.case = `empty unroll`;
    var src = _.unroll.make( [] );
    var got = long[ env.method ]( src );
    test.true( got instanceof InstanceConstructor );
    test.identical( got.length, 0 );

    test.case = `filled unroll`;
    var src = _.unroll.make([ 2, 3 ]);
    var got = long[ env.method ]( src );
    test.true( got instanceof InstanceConstructor );
    test.identical( got.length, 2 );
    test.identical( got, InstanceConstructor.from([ 2, 3 ]) );

    /* */

    test.case = `empty argumentsArray`;
    var src = _.argumentsArray.make( [] );
    var got = long[ env.method ]( src );
    if( env.tools === 'bufferTyped' )
    test.true( got instanceof Constructor );
    else
    test.true( _.argumentsArray.is( got ) );
    test.identical( got.length, 0 );

    test.case = `filled argumentsArray`;
    var src = _.argumentsArray.make([ 2, 3 ]);
    var got = long[ env.method ]( src );
    if( env.tools === 'bufferTyped' )
    {
      test.true( got instanceof Constructor );
      test.identical( got, InstanceConstructor.from([ 2, 3 ]) );
    }
    else
    {
      test.true( _.argumentsArray.is( got ) );
      test.identical( got, _.argumentsArray.make([ 2, 3 ]) );
    }

    /* */

    test.case = `empty typed buffer`;
    var src = _.u8x.make( [] );
    var got = long[ env.method ]( src );
    test.true( got instanceof U8x );
    test.identical( got.length, 0 );

    test.case = `filled typed buffer`;
    var src = _.f32x.make([ 2, 3 ]);
    var got = long[ env.method ]( src );
    test.true( got instanceof F32x );
    test.identical( got, _.f32x.make([ 2, 3 ]) );

    /* */

    if( env.method !== 'cloneShallow' )
    {
      test.case = `filled countable`;
      var src = __.diagnostic.objectMake({ elements : [ 2, 3 ], countable : 1 });
      var got = long[ env.method ]( src );
      test.true( got instanceof Constructor );
      test.identical( got, Constructor.from([ 2, 3 ]) );

      /* */

      test.case = `null and length - number`;
      var got = long[ env.method ]( null, 2 );
      test.true( got instanceof Constructor );
      test.identical( got.length, 2 );

      test.case = `null and length - array`;
      var got = long[ env.method ]( null, [ 1, 2 ] );
      test.true( got instanceof Constructor );
      test.identical( got.length, 2 );

      test.case = `null and countable`;
      var length = __.diagnostic.objectMake({ elements : [ 1, 2, 3 ], countable : 1 });
      var got = long[ env.method ]( null, length );
      test.true( got instanceof Constructor );
      test.identical( got.length, 3 );

      /* */

      test.case = `empty array and length`;
      var src = [];
      var got = long[ env.method ]( src, 2 );
      test.true( got instanceof InstanceConstructor );
      test.identical( got.length, 2 );

      test.case = `empty unroll and length`;
      var src = _.unroll.make( [] );
      var got = long[ env.method ]( src, 2 );
      test.true( got instanceof InstanceConstructor );
      test.identical( got.length, 2 );

      test.case = `empty argumentsArray and length`;
      var src = _.argumentsArray.make( [] );
      var got = long[ env.method ]( src, 2 );
      if( env.tools === 'bufferTyped' )
      test.true( got instanceof Constructor );
      else
      test.true( _.argumentsArray.is( got ) );
      test.identical( got.length, 2 );

      test.case = `empty typed buffer and length`;
      var src = _.u8x.make( [] );
      var got = long[ env.method ]( src, 2 );
      test.true( got instanceof U8x );
      test.identical( got.length, 2 );

      /* */

      test.case = `non-empty array and length longer`;
      var src = [ 3, 4 ];
      var got = long[ env.method ]( src, 3 );
      test.true( got instanceof InstanceConstructor );
      var value = env.tools === 'bufferTyped' ? 0 : undefined;
      test.identical( got, InstanceConstructor.from([ 3, 4, value ]) );

      test.case = `non-empty array and length shorter`;
      var src = [ 3, 4 ];
      var got = long[ env.method ]( src, 1 );
      test.true( got instanceof InstanceConstructor );
      test.identical( got, InstanceConstructor.from([ 3 ]) );

      /* */

      test.case = `non-empty unroll and length longer`;
      var src = _.unroll.make([ 3, 4 ]);
      var got = long[ env.method ]( src, 3 );
      test.true( got instanceof InstanceConstructor );
      var value = env.tools === 'bufferTyped' ? 0 : undefined;
      test.identical( got, InstanceConstructor.from([ 3, 4, value ]) );

      test.case = `non-empty unroll and length shorter`;
      var src = _.unroll.make([ 3, 4 ]);
      var got = long[ env.method ]( src, 1 );
      test.true( got instanceof InstanceConstructor );
      test.identical( got, InstanceConstructor.from([ 3 ]) );

      /* */

      test.case = `non-empty argumentsArray and length longer`;
      var src = _.argumentsArray.make([ 3, 4 ]);
      var got = long[ env.method ]( src, 3 );
      if( env.tools === 'bufferTyped' )
      {
        test.true( got instanceof Constructor );
        test.identical( got, InstanceConstructor.from([ 3, 4, 0 ]) );
      }
      else
      {
        test.true( _.argumentsArray.is( got ) );
        test.identical( got, _.argumentsArray.make([ 3, 4, undefined ]) );
      }

      test.case = `non-empty argumentsArray and length shorter`;
      var src = _.argumentsArray.make([ 3, 4 ]);
      var got = long[ env.method ]( src, 1 );
      if( env.tools === 'bufferTyped' )
      {
        test.true( got instanceof Constructor );
        test.identical( got, InstanceConstructor.from([ 3 ]) );
      }
      else
      {
        test.true( _.argumentsArray.is( got ) );
        test.identical( got, _.argumentsArray.make([ 3 ]) );
      }

      /* */

      test.case = `non-empty typed buffer and length longer`;
      var src = _.u8x.make([ 3, 4 ]);
      var got = long[ env.method ]( src, 3 );
      test.true( got instanceof U8x );
      var value = env.tools === 'bufferTyped' ? 0 : undefined;
      test.identical( got, _.u8x.make([ 3, 4, value ]) );

      test.case = `non-empty typed buffer and length shorter`;
      var src = _.f32x.make([ 3, 4 ]);
      var got = long[ env.method ]( src, 1 );
      test.true( got instanceof F32x );
      test.identical( got, _.f32x.make([ 3 ]) );

      /* */

      test.case = `array and countable`;
      var length = __.diagnostic.objectMake({ elements : [ 1, 2, 3 ], countable : 1 });
      var got = long[ env.method ]( [ 0, 0, 0, 0 ], length );
      test.true( got instanceof InstanceConstructor );
      test.identical( got, InstanceConstructor.from([ 1, 2, 3 ]) );
    }

    test.close( `${__.entity.exportStringSolo( env )}` );
  }

  function namespaceGet( env )
  {
    if( env.tools === 'default' )
    return _.long;
    if( env.tools === 'bufferTyped' )
    return _.bufferTyped;
    return _.withLong[ env.type ].long;
  }

  /* */

  function defaultConstructorGet( env )
  {
    if( env.tools === 'default' )
    return _.long.default.InstanceConstructor;
    if( env.tools === 'bufferTyped' )
    return _.bufferTyped.default.InstanceConstructor;
    return _.withLong[ env.type ].long.default.InstanceConstructor;
  }
}

makeCommonWithLongDescriptor.timeOut = 30000;

//

function makeLongFilledCommon( test )
{
  methodEach({ tools : 'default', type : 'Array' });
  methodEach({ tools : 'Array', type : 'Array' });
  methodEach({ tools : 'F32x', type : 'F32x' });
  methodEach({ tools : 'bufferTyped', type : 'Array' });

  /* - */

  function methodEach( env )
  {
    env.method = 'makeUndefined';
    act( env );
    env.method = 'makeZeroed';
    act( env );
  }

  /* */

  function act( env )
  {
    const long = namespaceGet( env );
    const Constructor = defaultConstructorGet( env );
    const InstanceConstructor = env.tools === 'bufferTyped' ? Constructor : Array;
    let value = env.method === 'makeUndefined' ? undefined : 0;
    if( env.tools === 'bufferTyped' )
    value = 0;

    /* */

    test.case = `${__.entity.exportStringSolo( env )}, no args`;
    var got = long[ env.method ]();
    test.true( got instanceof Constructor );
    test.identical( got.length, 0 );

    test.case = `${__.entity.exportStringSolo( env )}, length`;
    var got = long[ env.method ]( 3 );
    test.true( got instanceof Constructor );
    var _value = _.buffer.typedIs( got ) ? 0 : value;
    test.identical( got, Constructor.from([ _value, _value, _value ]) );

    test.case = `${__.entity.exportStringSolo( env )}, filled countable`;
    var src = __.diagnostic.objectMake({ elements : [ 2, 3 ], countable : 1 });
    var got = long[ env.method ]( src );
    test.true( got instanceof Constructor );
    var _value = _.buffer.typedIs( got ) ? 0 : value;
    test.identical( got, Constructor.from([ _value, _value ]) );

    /* */

    test.case = `${__.entity.exportStringSolo( env )}, empty array`;
    var got = long[ env.method ]( [] );
    test.true( got instanceof InstanceConstructor );
    test.identical( got.length, 0 );

    test.case = `${__.entity.exportStringSolo( env )}, 1 element`;
    var got = long[ env.method ]( [ 2 ] );
    test.true( got instanceof InstanceConstructor );
    test.identical( got, InstanceConstructor.from([ value ]) );

    test.case = `${__.entity.exportStringSolo( env )}, 2 elements`;
    var got = long[ env.method ]( [ 2, 3 ] );
    test.true( got instanceof InstanceConstructor );
    test.identical( got, InstanceConstructor.from([ value, value ]) );

    /* */

    test.case = `${__.entity.exportStringSolo( env )}, empty and length`;
    var got = long[ env.method ]( [], 2 );
    test.true( got instanceof InstanceConstructor );
    test.identical( got, InstanceConstructor.from([ value, value ]) );

    test.case = `${__.entity.exportStringSolo( env )}, non-empty and length longer`;
    var got = long[ env.method ]( [ 3, 4 ], 3 );
    test.true( got instanceof InstanceConstructor );
    test.identical( got, InstanceConstructor.from([ value, value, value ]) );

    test.case = `${__.entity.exportStringSolo( env )}, non-empty and length shorter`;
    var got = long[ env.method ]( [ 3, 4 ], 1 );
    test.true( got instanceof InstanceConstructor );
    test.identical( got, InstanceConstructor.from([ value ]) );

    /* */

    test.case = `${__.entity.exportStringSolo( env )}, non-empty and ins longer`;
    var got = long[ env.method ]( [ 3, 4 ], [ 2, 3, 4 ] );
    test.true( got instanceof InstanceConstructor );
    test.identical( got, InstanceConstructor.from([ value, value, value ]) );

    test.case = `${__.entity.exportStringSolo( env )}, non-empty and ins shorter`;
    var got = long[ env.method ]( [ 3, 4 ], [ 2 ] );
    test.true( got instanceof InstanceConstructor );
    test.identical( got, InstanceConstructor.from([ value ]) );

    /* */

    test.case = `${__.entity.exportStringSolo( env )}, null and countable`;
    var length = __.diagnostic.objectMake({ elements : [ 1, 2, 3 ], countable : 1 });
    var got = long[ env.method ]( null, length );
    test.true( got instanceof Constructor );
    var _value = _.buffer.typedIs( got ) ? 0 : undefined;
    if( env.method === 'makeZeroed' )
    _value = 0;
    test.identical( got, Constructor.from([ _value, _value, _value ]) );

    test.case = `${__.entity.exportStringSolo( env )}, array and countable`;
    var length = __.diagnostic.objectMake({ elements : [ 1, 2, 3 ], countable : 1 });
    var got = long[ env.method ]( [ 1, 1, 1, 1 ], length );
    test.true( got instanceof InstanceConstructor );
    test.identical( got, InstanceConstructor.from([ value, value, value ]) );

    /* */

    if( Config.debug )
    {
      test.case = 'extra arguments';
      test.shouldThrowErrorSync( () => long[ env.method ]( [], 1, 1 ) );

      test.case = 'wrong type of src';
      test.shouldThrowErrorSync( () => long[ env.method ]( undefined ) );
      test.shouldThrowErrorSync( () => long[ env.method ]( 3, 3 ) );

      test.case = 'wrong type of length';
      test.shouldThrowErrorSync( () => long[ env.method ]( [ 1, 2, 3 ], null ) );
      test.shouldThrowErrorSync( () => long[ env.method ]( 3, null ) );
      test.shouldThrowErrorSync( () => long[ env.method ]( Array, null ) );
      test.shouldThrowErrorSync( () => long[ env.method ]( null, null ) );
      test.shouldThrowErrorSync( () => long[ env.method ]( [ 1, 2, 3 ], undefined ) );
      test.shouldThrowErrorSync( () => long[ env.method ]( 3, undefined ) );
      test.shouldThrowErrorSync( () => long[ env.method ]( Array, undefined ) );
      test.shouldThrowErrorSync( () => long[ env.method ]( null, undefined ) );
    }
  }

  /* */

  function namespaceGet( env )
  {
    if( env.tools === 'default' )
    return _.long;
    if( env.tools === 'bufferTyped' )
    return _.bufferTyped;
    return _.withLong[ env.tools ].long;
  }

  /* */

  function defaultConstructorGet( env )
  {
    if( env.tools === 'default' )
    return _.long.default.InstanceConstructor;
    if( env.tools === 'bufferTyped' )
    return _.bufferTyped.default.InstanceConstructor;
    return _.withLong[ env.tools ].long.default.InstanceConstructor;
  }
}

//

function makeLongFilledCommonWithLongDescriptor( test )
{
  let times = 4;
  for( let k in _.long.namespaces )
  {
    let namespace = _.long.namespaces[ k ];
    let type = namespace.TypeName;

    if( type === 'ArgumentsArray' )
    continue;

    test.open( `long - ${ type }` );
    methodEach({ tools : 'default', type });
    methodEach({ tools : 'Array', type });
    methodEach({ tools : 'F32x', type });
    methodEach({ tools : 'bufferTyped', type });
    test.close( `long - ${ type }` );

    if( times < 1 )
    break;
    times--;
  }

  /* - */

  function methodEach( env )
  {
    env.method = 'makeUndefined';
    act( env );
    env.method = 'makeZeroed';
    act( env );
  }

  /* */

  function act( env )
  {
    const long = namespaceGet( env );
    const Constructor = defaultConstructorGet( env );
    const InstanceConstructor = env.tools === 'bufferTyped' ? Constructor : Array;
    let value = env.method === 'makeUndefined' ? undefined : 0;
    if( env.tools === 'bufferTyped' )
    value = 0;

    /* */

    test.case = `${__.entity.exportStringSolo( env )}, no args`;
    var got = long[ env.method ]();
    test.true( got instanceof Constructor );
    test.identical( got.length, 0 );

    test.case = `${__.entity.exportStringSolo( env )}, length`;
    var got = long[ env.method ]( 3 );
    test.true( got instanceof Constructor );
    var _value = _.buffer.typedIs( got ) ? 0 : value;
    test.identical( got, Constructor.from([ _value, _value, _value ]) );

    test.case = `${__.entity.exportStringSolo( env )}, filled countable`;
    var src = __.diagnostic.objectMake({ elements : [ 2, 3 ], countable : 1 });
    var got = long[ env.method ]( src );
    test.true( got instanceof Constructor );
    var _value = _.buffer.typedIs( got ) ? 0 : value;
    test.identical( got, Constructor.from([ _value, _value ]) );

    /* */

    test.case = `${__.entity.exportStringSolo( env )}, empty array`;
    var got = long[ env.method ]( [] );
    test.true( got instanceof InstanceConstructor );
    test.identical( got.length, 0 );

    test.case = `${__.entity.exportStringSolo( env )}, 1 element`;
    var got = long[ env.method ]( [ 2 ] );
    test.true( got instanceof InstanceConstructor );
    test.identical( got, InstanceConstructor.from([ value ]) );

    test.case = `${__.entity.exportStringSolo( env )}, 2 elements`;
    var got = long[ env.method ]( [ 2, 3 ] );
    test.true( got instanceof InstanceConstructor );
    test.identical( got, InstanceConstructor.from([ value, value ]) );

    /* */

    test.case = `${__.entity.exportStringSolo( env )}, empty and length`;
    var got = long[ env.method ]( [], 2 );
    test.true( got instanceof InstanceConstructor );
    test.identical( got, InstanceConstructor.from([ value, value ]) );

    test.case = `${__.entity.exportStringSolo( env )}, non-empty and length longer`;
    var got = long[ env.method ]( [ 3, 4 ], 3 );
    test.true( got instanceof InstanceConstructor );
    test.identical( got, InstanceConstructor.from([ value, value, value ]) );

    test.case = `${__.entity.exportStringSolo( env )}, non-empty and length shorter`;
    var got = long[ env.method ]( [ 3, 4 ], 1 );
    test.true( got instanceof InstanceConstructor );
    test.identical( got, InstanceConstructor.from([ value ]) );

    /* */

    test.case = `${__.entity.exportStringSolo( env )}, non-empty and ins longer`;
    var got = long[ env.method ]( [ 3, 4 ], [ 2, 3, 4 ] );
    test.true( got instanceof InstanceConstructor );
    test.identical( got, InstanceConstructor.from([ value, value, value ]) );

    test.case = `${__.entity.exportStringSolo( env )}, non-empty and ins shorter`;
    var got = long[ env.method ]( [ 3, 4 ], [ 2 ] );
    test.true( got instanceof InstanceConstructor );
    test.identical( got, InstanceConstructor.from([ value ]) );

    /* */

    test.case = `${__.entity.exportStringSolo( env )}, null and countable`;
    var length = __.diagnostic.objectMake({ elements : [ 1, 2, 3 ], countable : 1 });
    var got = long[ env.method ]( null, length );
    test.true( got instanceof Constructor );
    var _value = _.buffer.typedIs( got ) ? 0 : undefined;
    if( env.method === 'makeZeroed' )
    _value = 0;
    test.identical( got, Constructor.from([ _value, _value, _value ]) );

    test.case = `${__.entity.exportStringSolo( env )}, array and countable`;
    var length = __.diagnostic.objectMake({ elements : [ 1, 2, 3 ], countable : 1 });
    var got = long[ env.method ]( [ 1, 1, 1, 1 ], length );
    test.true( got instanceof InstanceConstructor );
    test.identical( got, InstanceConstructor.from([ value, value, value ]) );
  }

  /* */

  function namespaceGet( env )
  {
    if( env.tools === 'default' )
    return _.long;
    if( env.tools === 'bufferTyped' )
    return _.bufferTyped;
    return _.withLong[ env.type ].long;
  }

  /* */

  function defaultConstructorGet( env )
  {
    if( env.tools === 'default' )
    return _.long.default.InstanceConstructor;
    if( env.tools === 'bufferTyped' )
    return _.bufferTyped.default.InstanceConstructor;
    return _.withLong[ env.type ].long.default.InstanceConstructor;
  }
}

makeLongFilledCommonWithLongDescriptor.timeOut = 10000;

//

function makeSrcIsNullWithLongNamespaces( test )
{
  for( let name in _.long.namespaces )
  {
    let namespace = _.long.namespaces[ name ];

    test.open( `${ namespace.TypeName }` );
    run( namespace );
    test.close( `${ namespace.TypeName }` );
  }

  function run( long )
  {
    test.case = 'only src - null';
    var got = long.make( null );
    test.identical( got, long.make( [] ) );

    test.case = 'src - null, length - number';
    var got = long.make( null, 0 );
    test.identical( got, long.make( [] ) );

    test.case = 'src - null, length - number';
    var got = long.make( null, 2 );
    var value = _.buffer.is( got ) ? 0 : undefined
    test.identical( got, long.make([ value, value ]) );

    test.case = 'src - null, length - long empty array';
    var got = long.make( null, [] );
    test.identical( got, long.make( [] ) );

    test.case = 'src - null, length - filled array';
    var got = long.make( null, [ 3, 4 ] );
    test.identical( got, long.make([ 3, 4 ]) );

    test.case = 'src - null, length - empty array';
    var got = long.make( null, new F32x() );
    test.identical( got, long.make([]) );

    test.case = 'src - null, length - filled typed array';
    var got = long.make( null, new U8x([ 3, 4 ]) );
    test.identical( got, long.make([ 3, 4 ]) );

    /* - */

    if( Config.debug )
    {
      test.case = 'src - number, length - number';
      test.shouldThrowErrorSync( () => long.make( 1, 1 ) );
    }
  }
}

//

// function longMakeWithArrayAndUnroll( test )
// {
//   var array = ( src ) => _.array.make( src );
//   var unroll = ( src ) => _.unroll.make( src );
//   var longConstr = ( src ) =>
//   {
//     if( src )
//     return _.long.default.make( src );
//     return _.long.default.make( 0 );
//   }
//   var list =
//   [
//     array,
//     unroll,
//     longConstr,
//   ];
//
//   /* tests */
//
//   for( let i = 0; i < list.length; i++ )
//   {
//     test.open( list[ i ].name );
//     testRun( list[ i ] );
//     test.close( list[ i ].name );
//   }
//
//   /* test subroutine */
//
//   function testRun( makeLong )
//   {
//     test.case = 'src = null, not ins';
//     var got = _.long.make( null );
//     var expected = [];
//     test.identical( got, expected );
//
//     test.case = 'src = number, not ins';
//     var got = _.long.make( 5 );
//     var expected = _.long.default.make( 5 );
//     test.identical( got, expected );
//
//     /* aaa : should throw error : add test cases */ /* Dmytro : added in new test routines */
//     // test.case = 'src = number, ins = null';
//     // var got = _.long.make( 5, null );
//     // var expected = _.long.default.make( 5 );
//     // test.identical( got, expected );
//
//     test.case = 'src = null, ins = number';
//     var got = _.long.make( null, 5 );
//     var expected = _.long.default.make( 5 );
//     test.identical( got, expected );
//
//     /* aaa : should throw error : add test cases */ /* Dmytro : added in new test routines */
//     // test.case = 'src = number, ins = undefined';
//     // var got = _.long.make( 5, undefined );
//     // var expected = _.long.default.make( 5 );
//     // test.identical( got, expected );
//
//     test.case = 'src = empty long, not ins';
//     var src = makeLong( [] );
//     var got = _.long.make( src );
//     var expected = makeLong( [] );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src.length - 1, positive number';
//     var src = makeLong( [ 2 ] );
//     var got = _.long.make( src );
//     var exp = makeLong( [ 2 ] );
//     test.identical( got, exp );
//     test.true( got !== src );
//
//     test.case = 'src.length - 1, negative number';
//     var src = makeLong( [ -1 ] );
//     var got = _.long.make( src );
//     var exp = makeLong( [ -1 ] );
//     test.identical( got, exp );
//     test.true( got !== src );
//
//     test.case = 'the irrational numbers in src';
//     var src = makeLong( [ 1.5 ] );
//     var got = _.long.make( src );
//     var exp = makeLong( [ 1.5 ] );
//     test.identical( got, exp );
//     test.true( got !== src );
//
//     test.case = 'src = long, not ins';
//     var src = makeLong([ 1, 2, 3 ]);
//     var got = _.long.make( src );
//     var expected = makeLong( [ 1, 2, 3 ] );
//     test.identical( got, expected );
//     test.identical( got.length, 3 );
//     test.true( got !== src );
//
//     test.case = 'src = empty long, ins = number';
//     var src = makeLong( [] );
//     var got = _.long.make( src, 2 );
//     var expected = makeLong( 2 );
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( _.arrayIs( got ) );
//
//     /* aaa : should throw error : add test cases */ /* Dmytro : added in new test routines */
//     // test.case = 'src = long, ins = null';
//     // var src = makeLong( [ 1, 2, 3 ] );
//     // var got = _.long.make( src, null );
//     // var expected = makeLong( [ 1, 2, 3 ] );
//     // test.identical( got, expected );
//     // test.true( got !== src );
//     // test.true( _.arrayIs( got ) );
//
//     test.case = 'ins = number, ins < src.length';
//     var src = makeLong( [ 1, 2, 3 ] );
//     var got = _.long.make( src, 2 );
//     var expected = makeLong( [ 1, 2 ] );
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( _.arrayIs( got ) );
//
//     test.case = 'ins = number, ins > src.length';
//     var src = makeLong( [ 1, 2, 3 ] );
//     var got = _.long.make( src, 4 );
//     var expected = makeLong( [ 1, 2, 3, undefined ] )
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( _.arrayIs( got ) );
//
//     test.case = 'ins = long, ins.length > src.length';
//     var src = makeLong( [ 0, 1 ] );
//     var ins = [ 1, 2, 3 ];
//     var got = _.long.make( src, ins );
//     var expected = makeLong( [ 1, 2, 3 ] );
//     test.identical( got, expected );
//     test.true( got !== ins );
//     test.true( got !== src );
//     test.true( _.arrayIs( got ) );
//
//     test.case = 'src = new long, ins = array'
//     var src = makeLong( 2 );
//     var ins = [ 1, 2, 3, 4, 5 ];
//     var got = _.long.make( src, ins );
//     var expected = makeLong( [ 1, 2, 3, 4, 5 ] );
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( _.arrayIs( got ) );
//
//     /* aaa : should throw error : add test cases */ /* Dmytro : added in new test routines */
//     // test.case = 'src = Array constructor, ins = null';
//     // var got = _.long.make( Array, null );
//     // var expected = [];
//     // test.identical( got, expected );
//     // test.true( _.arrayIs( got ) );
//
//     test.case = 'src = Array constructor, ins = number';
//     var got = _.long.make( Array, 3 );
//     var expected = [ undefined, undefined, undefined ];
//     test.identical( got, expected );
//     test.true( _.arrayIs( got ) );
//
//     test.case = 'src = BufferTyped constructor, ins = long';
//     var ins = makeLong( [ 1, 1, 1, 1, 1 ] );
//     var got = _.long.make( U32x, ins );
//     var expected = new U32x( [ 1, 1, 1, 1, 1 ] );
//     test.identical( got, expected );
//     test.true( _.buffer.typedIs(  got ) );
//     test.true( got !== ins );
//
//     test.case = 'src = long constructor, ins = number';
//     var src = makeLong( 5 );
//     var got = _.long.make( src.constructor, 2 );
//     var expected = makeLong([ undefined, undefined ]);
//     test.identical( got, expected );
//     test.true( got !== ins );
//
//     test.case = 'njs buffer';
//     var src = BufferNode.alloc( 3 );
//     var got = _.long.make( null, src );
//     var expected = [ 0, 0, 0 ];
//     test.identical( got, expected );
//
//     test.case = 'constructor and njs buffer';
//     var src = BufferNode.alloc( 3 );
//     var got = _.long.make( Array, src );
//     var expected = [ 0, 0, 0 ];
//     test.identical( got, expected );
//
//     test.case = 'countable';
//     var src = __.diagnostic.objectMake({ elements : [ 1, 2, 3 ], countable : 1 });
//     var got = _.long.make( null, src );
//     var expected = [ 1, 2, 3 ];
//     test.identical( got, expected );
//
//     // test.case = 'src = long constructor, ins = null';
//     // var src = makeLong( 5 );
//     // var got = _.long.make( src.constructor, null );
//     // var expected = makeLong();
//     // test.identical( got, expected );
//     // test.true( got !== ins );
//     //
//     // test.case = 'src = long constructor, ins = undefined';
//     // var src = makeLong( 5 );
//     // var got = _.long.make( src.constructor, undefined );
//     // var expected = makeLong( 0 );
//     // test.identical( got, expected );
//     // test.true( got !== ins );
//
//   }
//
//   /* - */
//
//   if( !Config.debug )
//   return;
//
//   // aaa : for Dmytro : add test case /* Dmytro : used in the new routines */
//   // test.case = 'without arguments';
//   // test.shouldThrowErrorSync( () => _.long.make() );
//
//   test.case = 'extra argument';
//   test.shouldThrowErrorSync( () => _.long.make( [ 1, 2, 3 ], 4, 'extra argument' ) );
//
//   test.case = 'wrong type of src';
//   test.shouldThrowErrorSync( () => _.long.make( 'wrong argument', 1 ) );
//   test.shouldThrowErrorSync( () => _.long.make( -1 ) );
//   test.shouldThrowErrorSync( () => _.long.make( 1, 1 ) );
//   test.shouldThrowErrorSync( () => _.long.make( new BufferRaw( 3 ), 2 ) );
//   test.shouldThrowErrorSync( () => _.long.make( ( e ) => { return { [ e ] : e } }, 5 ) );
//   if( Config.interpreter === 'njs' )
//   {
//     // test.shouldThrowErrorSync( () => _.long.make( Array, BufferNode.from( [ 3 ] ) ) );
//     test.shouldThrowErrorSync( () => _.long.make( BufferNode.alloc( 3 ), 2 ) );
//   }
//
//   test.case = 'wrong type of ins';
//   test.shouldThrowErrorSync( () => _.long.make( [ 1, 2, 3 ], 'wrong type of argument' ) );
//   test.shouldThrowErrorSync( () => _.long.make( [ 1, 2, 3 ], Infinity  ) );
//
//   /* aaa : for Dmytro : duplicate this cases in other tests routines */ /* Dmytro : added in new test routines */
//   test.case = 'bad second argument';
//   test.shouldThrowErrorSync( () => _.long.make( [ 1, 2, 3 ], null ) );
//   test.shouldThrowErrorSync( () => _.long.make( 3, null ) );
//   test.shouldThrowErrorSync( () => _.long.make( Array, null ) );
//   test.shouldThrowErrorSync( () => _.long.make( null, null ) );
//   test.shouldThrowErrorSync( () => _.long.make( [ 1, 2, 3 ], undefined ) );
//   test.shouldThrowErrorSync( () => _.long.make( 3, undefined ) );
//   test.shouldThrowErrorSync( () => _.long.make( Array, undefined ) );
//   test.shouldThrowErrorSync( () => _.long.make( null, undefined ) );
//
// }
//
// //
//
// /* aaa : for Dmytro : for junior : rewrite properly */ /* Dmytro : all coverage in the new routines */
// function longMakeWithArgumentsArray( test )
// {
//   test.case = 'src = null, not ins';
//   var got = _.long.make( null );
//   var expected = [];
//   test.identical( got, expected );
//
//   test.case = 'src = number, not ins';
//   var got = _.long.make( 5 );
//   var expected = _.long.default.make( 5 );
//   test.identical( got, expected );
//
//   /* aaa : should throw error : add test cases */ /* Dmytro : added in new test routines */
//   // test.case = 'src = number, ins = null';
//   // var got = _.long.make( 5, null );
//   // var expected = _.long.default.make( 5 );
//   // test.identical( got, expected );
//
//   test.case = 'src = null, ins = number';
//   var got = _.long.make( null, 5 );
//   var expected = _.long.default.make( 5 );
//   test.identical( got, expected );
//
//   /* aaa : should throw error : add test cases */ /* Dmytro : added in new test routines */
//   // test.case = 'src = number, ins = undefined';
//   // var got = _.long.make( 5, undefined );
//   // var expected = _.long.default.make( 5 );
//   // test.identical( got, expected );
//
//   test.case = 'src = empty long, not ins';
//   var src = _.argumentsArray.make( [] );
//   var got = _.long.make( src );
//   var expected = _.argumentsArray.make( [] );
//   test.identical( got, expected );
//   test.true( got !== src );
//
//   test.case = 'src = long, not ins';
//   var src = _.argumentsArray.make( [ 1, 2, 3 ] );
//   var got = _.long.make( src );
//   var expected = _.argumentsArray.make( [ 1, 2, 3 ] );
//   test.identical( got, expected );
//   test.identical( got.length, 3 );
//   test.true( got !== src );
//
//   test.case = 'src = empty long, ins = number';
//   var src = _.argumentsArray.make( [] );
//   var got = _.long.make( src, 2 );
//   var expected = _.argumentsArray.make( 2 );
//   test.identical( got, expected );
//   test.true( got !== src );
//
//   /* aaa : should throw error : add test cases */ /* Dmytro : added in new test routines */
//   // test.case = 'src = long, ins = null';
//   // var src = _.argumentsArray.make( [ 1, 2, 3 ] );
//   // var got = _.long.make( src, null );
//   // var expected = _.long.default.make( [ 1, 2, 3 ] );
//   // test.identical( got, expected );
//   // test.true( got !== src );
//   // test.true( _.arrayIs( got ) );
//
//   test.case = 'ins = number, ins < src.length';
//   var src = _.argumentsArray.make( [ 1, 2, 3 ] );
//   var got = _.long.make( src, 2 );
//   var expected = _.argumentsArray.make( [ 1, 2 ] );
//   test.identical( got, expected );
//   test.true( got !== src );
//
//   test.case = 'ins = number, ins > src.length';
//   var src = _.argumentsArray.make( [ 1, 2, 3 ] );
//   var got = _.long.make( src, 4 );
//   var expected = _.argumentsArray.make( [ 1, 2, 3, undefined ] );
//   test.identical( got, expected );
//   test.true( got !== src );
//
//   test.case = 'ins = long, ins.length > src.length';
//   var src = _.argumentsArray.make( [ 0, 1 ] );
//   var ins = [ 1, 2, 3 ];
//   var got = _.long.make( src, ins );
//   var expected = _.argumentsArray.make( [ 1, 2, 3 ] );
//   test.identical( got, expected );
//   test.true( got !== ins );
//   test.true( got !== src );
//
//   test.case = 'src = new long, ins = array'
//   var src = _.argumentsArray.make( 2 );
//   var ins = [ 1, 2, 3, 4, 5 ];
//   var got = _.long.make( src, ins );
//   var expected = _.argumentsArray.make( [ 1, 2, 3, 4, 5 ] );
//   test.identical( got, expected );
//   test.true( got !== src );
//
//   test.case = 'src = Array constructor, ins = number';
//   var got = _.long.make( Array, 3 );
//   var expected = [ undefined, undefined, undefined ];
//   test.identical( got, expected );
//   test.true( _.arrayIs( got ) );
//
//   test.case = 'src = BufferTyped constructor, ins = long';
//   var ins = _.argumentsArray.make( [ 1, 1, 1, 1, 1 ] );
//   var got = _.long.make( U32x, ins );
//   var expected = new U32x( [ 1, 1, 1, 1, 1 ] );
//   test.identical( got, expected );
//   test.true( _.buffer.typedIs(  got ) );
//   test.true( got !== ins );
// }
//
// //
//
// function longMakeWithBufferTyped( test )
// {
//   var list =
//   [
//     I8x,
//     U16x,
//     F32x,
//     F64x,
//   ];
//
//   /* tests */
//
//   for( let i = 0; i < list.length; i++ )
//   {
//     test.open( list[ i ].name );
//     testRun( list[ i ] );
//     test.close( list[ i ].name );
//   }
//
//   /* test subroutine */
//
//   function testRun( makeLong )
//   {
//     test.case = 'src = null, not ins';
//     var got = _.long.make( null );
//     var expected = [];
//     test.identical( got, expected );
//
//     test.case = 'src = number, not ins';
//     var got = _.long.make( 5 );
//     var expected = _.long.default.make( 5 );
//     test.identical( got, expected );
//
//     /* aaa : should throw error : add test cases */ /* Dmytro : added in new test routines */
//     // test.case = 'src = number, ins = null';
//     // var got = _.long.make( 5, null );
//     // var expected = _.long.default.make( 5 );
//     // test.identical( got, expected );
//
//     test.case = 'src = null, ins = number';
//     var got = _.long.make( null, 5 );
//     var expected = _.long.default.make( 5 );
//     test.identical( got, expected );
//
//     /* aaa : should throw error : add test cases */ /* Dmytro : added in new test routines */
//     // test.case = 'src = number, ins = undefined';
//     // var got = _.long.make( 5, undefined );
//     // var expected = _.long.default.make( 5 );
//     // test.identical( got, expected );
//
//     test.case = 'src = empty long, not ins';
//     var src = new makeLong( [] );
//     var got = _.long.make( src );
//     var expected = new makeLong( [] );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src = long, not ins';
//     var src = new makeLong( [ 1, 2, 3 ] );
//     var got = _.long.make( src );
//     var expected = new makeLong( [ 1, 2, 3 ] );
//     test.identical( got, expected );
//     test.identical( got.length, 3 );
//     test.true( got !== src );
//
//     test.case = 'src = empty long, ins = number';
//     var src = new makeLong( [] );
//     var got = _.long.make( src, 2 );
//     var expected = new makeLong( 2 );
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( _.buffer.typedIs( got ) );
//
//     /* aaa : should throw error : add test cases */ /* Dmytro : added in new test routines */
//     // test.case = 'src = long, ins = null';
//     // var src = new makeLong( [ 1, 2, 3 ] );
//     // var got = _.long.make( src, null );
//     // var expected = new makeLong( [ 1, 2, 3 ] );
//     // test.identical( got, expected );
//     // test.true( got !== src );
//     // test.true( _.buffer.typedIs( got ) );
//
//     test.case = 'ins = number, ins < src.length';
//     var src = new makeLong( [ 1, 2, 3 ] );
//     var got = _.long.make( src, 2 );
//     var expected = new makeLong( [ 1, 2 ] );
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( _.buffer.typedIs( got ) );
//
//     test.case = 'ins = number, ins > src.length';
//     var src = new makeLong( [ 1, 2, 3 ] );
//     var got = _.long.make( src, 4 );
//     // var expected = new long( [ 1, 2, 3, 0 ] )
//     var expected = new makeLong( [ 1, 2, 3, 0 ] )
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( _.buffer.typedIs( got ) );
//
//     test.case = 'ins = long, ins.length > src.length';
//     var src = new makeLong( [ 0, 1 ] );
//     var ins = [ 1, 2, 3 ];
//     var got = _.long.make( src, ins );
//     var expected = new makeLong( [ 1, 2, 3 ] );
//     test.identical( got, expected );
//     test.true( got !== ins );
//     test.true( got !== src );
//     test.true( _.buffer.typedIs( got ) );
//
//     test.case = 'src = new long, ins = array'
//     var src = new makeLong( 2 );
//     var ins = [ 1, 2, 3, 4, 5 ];
//     var got = _.long.make( src, ins );
//     var expected = new makeLong( [ 1, 2, 3, 4, 5 ] );
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( _.buffer.typedIs( got ) );
//
//     // test.case = 'src = Array constructor, ins = null';
//     // var got = _.long.make( Array, null );
//     // var expected = [];
//     // test.identical( got, expected );
//     // test.true( _.arrayIs( got ) );
//
//     test.case = 'src = Array constructor, ins = number';
//     var got = _.long.make( Array, 3 );
//     var expected = [ undefined, undefined, undefined ];
//     test.identical( got, expected );
//     test.true( _.arrayIs( got ) );
//
//     test.case = 'src = BufferTyped constructor, ins = long';
//     var ins = new makeLong( [ 1, 1, 1, 1, 1 ] );
//     var got = _.long.make( U32x, ins );
//     var expected = new U32x( [ 1, 1, 1, 1, 1 ] );
//     test.identical( got, expected );
//     test.true( _.buffer.typedIs(  got ) );
//     test.true( got !== ins );
//
//     test.case = 'src = long constructor, ins = number';
//     var src = new makeLong( 5 );
//     var got = _.long.make( src.constructor, 2 );
//     var expected = new makeLong([ undefined, undefined ]);
//     test.identical( got, expected );
//     test.true( got !== ins );
//
//     // test.case = 'src = long constructor, ins = null';
//     // var src = new makeLong( 5 );
//     // var got = _.long.make( src.constructor, null );
//     // var expected = new makeLong();
//     // test.identical( got, expected );
//     // test.true( got !== ins );
//     //
//     // test.case = 'src = long constructor, ins = undefined';
//     // var src = new makeLong( 5 );
//     // var got = _.long.make( src.constructor, undefined );
//     // var expected = new makeLong( 0 );
//     // test.identical( got, expected );
//     // test.true( got !== ins );
//   }
// }
//
// //
//
// function longMakeWithArrayAndUnrollLongDescriptor( test )
// {
//   var array = ( src ) => _.array.make( src );
//   var unroll = ( src ) => _.unroll.make( src );
//   var longConstr = ( src ) =>
//   {
//     if( src )
//     return _.long.default.make( src );
//     return _.long.default.make( 0 );
//   }
//   var list =
//   [
//     array,
//     unroll,
//     longConstr,
//   ];
//
//   /* tests */
//
//   let times = 4;
//   // for( let e in _.LongDescriptors )
//   // {
//   //   let name = _.LongDescriptors[ e ].name;
//   //   let long = _.withLong[ name ];
//
//   for( let k in _.long.namespaces )
//   {
//     let namespace = _.long.namespaces[ k ];
//     let name = namespace.TypeName;
//     let long = _.withLong[ name ];
//
//     for( let i = 0; i < list.length; i++ )
//     {
//       test.open( `long - ${ name }, makeLong - ${ list[ i ].name }` );
//       testRun( long, list[ i ] );
//       test.close( `long - ${ name }, makeLong - ${ list[ i ].name }` );
//     }
//
//     if( times < 1 )
//     break;
//     times--;
//   }
//
//   /* test subroutine */
//
//   function testRun( long, makeLong )
//   {
//
//     test.case = 'no args';
//     var got = long.long.make();
//     var expected = long.long.default.make();
//     test.identical( got, expected );
//
//     test.case = 'src = null, not ins';
//     var got = long.long.make( null );
//     var expected = long.long.default.make( 0 );
//     test.identical( got, expected );
//
//     test.case = 'src = number, not ins';
//     var got = long.long.make( 5 );
//     var expected = long.long.default.make( 5 );
//     test.identical( got, expected );
//
//     /* aaa : should throw error : add test cases */ /* Dmytro : added in new test routines */
//     // test.case = 'src = number, ins = null';
//     // var got = long.long.make( 5, null );
//     // var expected = long.long.default.make( 5 );
//     // test.identical( got, expected );
//
//     test.case = 'src = null, ins = number';
//     var got = long.long.make( null, 5 );
//     var expected = long.long.default.make( 5 );
//     test.identical( got, expected );
//
//     /* aaa : should throw error : add test cases */ /* Dmytro : added in new test routines */
//     // test.case = 'src = number, ins = undefined';
//     // var got = long.long.make( 5, undefined );
//     // var expected = long.long.default.make( 5 );
//     // test.identical( got, expected );
//
//     test.case = 'src = empty long, not ins';
//     var src = makeLong( [] );
//     var got = long.long.make( src );
//     var expected = makeLong( [] );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src.length - 1, positive number';
//     var src = makeLong( [ 2 ] );
//     var got = long.long.make( src );
//     var exp = makeLong( [ 2 ] );
//     test.identical( got, exp );
//     test.true( got !== src );
//
//     test.case = 'src.length - 1, negative number';
//     var src = makeLong( [ -1 ] );
//     var got = long.long.make( src );
//     var exp = makeLong( [ -1 ] );
//     test.identical( got, exp );
//     test.true( got !== src );
//
//     test.case = 'the irrational numbers in src';
//     var src = makeLong( [ 1.5 ] );
//     var got = long.long.make( src );
//     var exp = makeLong( [ 1.5 ] );
//     test.identical( got, exp );
//     test.true( got !== src );
//
//     test.case = 'src = long, not ins';
//     var src = makeLong( [ 1, 2, 3 ] );
//     var got = long.long.make( src );
//     var expected = makeLong( [ 1, 2, 3 ] );
//     test.identical( got, expected );
//     test.identical( got.length, 3 );
//     test.true( got !== src );
//
//     test.case = 'src = empty long, ins = number';
//     var src = makeLong( [] );
//     var got = long.long.make( src, 2 );
//     var expected = makeLong( 2 );
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( _.arrayIs( got ) );
//
//     /* aaa : should throw error : add test cases */ /* Dmytro : added in new test routines */
//     // test.case = 'src = long, ins = null';
//     // var src = makeLong( [ 1, 2, 3 ] );
//     // var got = long.long.make( src, null );
//     // var expected = makeLong( [ 1, 2, 3 ] );
//     // test.identical( got, expected );
//     // test.true( got !== src );
//     // test.true( _.arrayIs( got ) );
//
//     test.case = 'ins = number, ins < src.length';
//     var src = makeLong( [ 1, 2, 3 ] );
//     var got = long.long.make( src, 2 );
//     var expected = makeLong( [ 1, 2 ] );
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( _.arrayIs( got ) );
//
//     test.case = 'ins = number, ins > src.length';
//     var src = makeLong( [ 1, 2, 3 ] );
//     var got = long.long.make( src, 4 );
//     // var expected = long( [ 1, 2, 3, undefined ] )
//     var expected = makeLong( [ 1, 2, 3, undefined ] )
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( _.arrayIs( got ) );
//
//     test.case = 'ins = long, ins.length > src.length';
//     var src = makeLong( [ 0, 1 ] );
//     var ins = [ 1, 2, 3 ];
//     var got = long.long.make( src, ins );
//     var expected = makeLong( [ 1, 2, 3 ] );
//     test.identical( got, expected );
//     test.true( got !== ins );
//     test.true( got !== src );
//     test.true( _.arrayIs( got ) );
//
//     test.case = 'src = new long, ins = array'
//     var src = makeLong( 2 );
//     var ins = [ 1, 2, 3, 4, 5 ];
//     var got = long.long.make( src, ins );
//     var expected = makeLong( [ 1, 2, 3, 4, 5 ] );
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( _.arrayIs( got ) );
//
//     // test.case = 'src = Array constructor, ins = null';
//     // var got = long.long.make( Array, null );
//     // var expected = [];
//     // test.identical( got, expected );
//     // test.true( _.arrayIs( got ) );
//
//     test.case = 'src = Array constructor, ins = number';
//     var got = long.long.make( Array, 3 );
//     var expected = [ undefined, undefined, undefined ];
//     test.identical( got, expected );
//     test.true( _.arrayIs( got ) );
//
//     test.case = 'src = BufferTyped constructor, ins = long';
//     var ins = makeLong( [ 1, 1, 1, 1, 1 ] );
//     var got = long.long.make( U32x, ins );
//     var expected = new U32x( [ 1, 1, 1, 1, 1 ] );
//     test.identical( got, expected );
//     test.true( _.buffer.typedIs(  got ) );
//     test.true( got !== ins );
//
//     test.case = 'src = long constructor, ins = number';
//     var src = makeLong( 5 );
//     var got = long.long.make( src.constructor, 2 );
//     var expected = makeLong([ undefined, undefined ]);
//     test.identical( got, expected );
//     test.true( got !== ins );
//
//     test.case = 'njs buffer';
//     var src = BufferNode.alloc( 3 );
//     var got = long.long.make( null, src );
//     var expected = makeLong([ 0, 0, 0 ]);
//     test.identical( got, expected );
//
//     test.case = 'constructor and njs buffer';
//     var src = BufferNode.alloc( 3 );
//     var got = long.long.make( Array, src );
//     var expected = makeLong([ 0, 0, 0 ]);
//     test.identical( got, expected );
//
//     test.case = 'countable';
//     var src = __.diagnostic.objectMake({ elements : [ 1, 2, 3 ], countable : 1 });
//     var got = long.long.make( null, src );
//     var expected = makeLong([ 1, 2, 3 ]);
//     test.identical( got, expected );
//
//     // test.case = 'src = long constructor, ins = null';
//     // var src = makeLong( 5 );
//     // var got = long.long.make( src.constructor, null );
//     // var expected = makeLong();
//     // test.identical( got, expected );
//     // test.true( got !== ins );
//     //
//     // test.case = 'src = long constructor, ins = undefined';
//     // var src = makeLong( 5 );
//     // var got = long.long.make( src.constructor, undefined );
//     // var expected = makeLong( 0 );
//     // test.identical( got, expected );
//     // test.true( got !== ins );
//
//     /* - */
//
//     if( Config.debug )
//     {
//       // test.case = 'without arguments';
//       // test.shouldThrowErrorSync( () => long.long.make() );
//
//       test.case = 'extra argument';
//       test.shouldThrowErrorSync( () => long.long.make( [ 1, 2, 3 ], 4, 'extra argument' ) );
//
//       test.case = 'wrong type of src';
//       test.shouldThrowErrorSync( () => long.long.make( 'wrong argument', 1 ) );
//       test.shouldThrowErrorSync( () => long.long.make( 1, 1 ) );
//       test.shouldThrowErrorSync( () => long.long.make( new BufferRaw( 3 ), 2 ) );
//       test.shouldThrowErrorSync( () => long.long.make( ( e ) => { return { [ e ] : e } }, 5 ) );
//       test.shouldThrowErrorSync( () => long.long.make( undefined ) );
//       if( Config.interpreter === 'njs' )
//       {
//         // test.shouldThrowErrorSync( () => long.long.make( Array, BufferNode.from( [ 3 ] ) ) );
//         test.shouldThrowErrorSync( () => long.long.make( BufferNode.alloc( 3 ), 2 ) );
//       }
//
//       test.case = 'wrong type of ins';
//       test.shouldThrowErrorSync( () => long.long.make( [ 1, 2, 3 ], 'wrong type of argument' ) );
//       test.shouldThrowErrorSync( () => long.long.make( [ 1, 2, 3 ], Infinity  ) );
//     }
//   }
// }
//
// longMakeWithArrayAndUnrollLongDescriptor.timeOut = 15000;
//
// //
//
// function longMakeWithArgumentsArrayLongDescriptor( test )
// {
//   let times = 4;
//
//   // for( let e in _.LongDescriptors )
//   // {
//   //   let name = _.LongDescriptors[ e ].name;
//   //   let long = _.withLong[ name ];
//
//   for( let k in _.long.namespaces )
//   {
//     let namespace = _.long.namespaces[ k ];
//     let name = namespace.TypeName;
//     let long = _.withLong[ name ];
//
//     test.open( `long - ${ name }` );
//     testRun( long );
//     test.close( `long - ${ name }` );
//
//     if( times < 1 )
//     break;
//     times--;
//   }
//
//   /* - */
//
//   function testRun( long )
//   {
//     test.case = 'src = null, not ins';
//     var got = long.long.make( null );
//     var expected = long.long.default.make( 0 );
//     test.identical( got, expected );
//
//     test.case = 'src = number, not ins';
//     var got = long.long.make( 5 );
//     var expected = long.long.default.make( 5 );
//     test.identical( got, expected );
//
//     /* aaa : should throw error : add test cases */ /* Dmytro : added in new test routines */
//     // test.case = 'src = number, ins = null';
//     // var got = long.long.make( 5, null );
//     // var expected = long.long.default.make( 5 );
//     // test.identical( got, expected );
//
//     test.case = 'src = null, ins = number';
//     var got = long.long.make( null, 5 );
//     var expected = long.long.default.make( 5 );
//     test.identical( got, expected );
//
//     /* aaa : should throw error : add test cases */ /* Dmytro : added in new test routines */
//     // test.case = 'src = number, ins = undefined';
//     // var got = long.long.make( 5, undefined );
//     // var expected = long.long.default.make( 5 );
//     // test.identical( got, expected );
//
//     test.case = 'src = empty long, not ins';
//     var src = _.argumentsArray.make( [] );
//     var got = long.long.make( src );
//     var expected = long.long.default.make( [] );
//     test.identical( got, expected );
//
//     test.case = 'src = long, not ins';
//     var src = _.argumentsArray.make( [ 1, 2, 3 ] );
//     var got = long.long.make( src );
//     var expected = long.long.default.make( [ 1, 2, 3 ] );
//     test.identical( got, expected );
//
//     test.case = 'src = empty long, ins = number';
//     var src = _.argumentsArray.make( [] );
//     var got = long.long.make( src, 2 );
//     var expected = long.long.default.make( 2 );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     /* aaa : should throw error : add test cases */ /* Dmytro : added in new test routines */
//     // test.case = 'src = long, ins = null';
//     // var src = _.argumentsArray.make( [ 1, 2, 3 ] );
//     // var got = long.long.make( src, null );
//     // var expected = long.long.default.make( [ 1, 2, 3 ] );
//     // test.identical( got, expected );
//
//     test.case = 'ins = number, ins < src.length';
//     var src = _.argumentsArray.make( [ 1, 2, 3 ] );
//     var got = long.long.make( src, 2 );
//     var expected = long.long.default.make( [ 1, 2 ] );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'ins = number, ins > src.length';
//     var src = _.argumentsArray.make( [ 1, 2, 3 ] );
//     var got = long.long.make( src, 4 );
//     var expected = long.long.default.make( [ 1, 2, 3, undefined ] )
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'ins = long, ins.length > src.length';
//     var src = _.argumentsArray.make( [ 0, 1 ] );
//     var ins = [ 1, 2, 3 ];
//     var got = long.long.make( src, ins );
//     var expected = long.long.default.make( [ 1, 2, 3 ] );
//     test.identical( got, expected );
//     test.true( got !== ins );
//     test.true( got !== src );
//
//     test.case = 'src = new long, ins = array'
//     var src = _.argumentsArray.make( 2 );
//     var ins = [ 1, 2, 3, 4, 5 ];
//     var got = long.long.make( src, ins );
//     var expected = long.long.default.make( [ 1, 2, 3, 4, 5 ] );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     // test.case = 'src = Array constructor, ins = null';
//     // var got = long.long.make( Array, null );
//     // var expected = [];
//     // test.identical( got, expected );
//     // test.true( _.arrayIs( got ) );
//
//     test.case = 'src = Array constructor, ins = number';
//     var got = long.long.make( Array, 3 );
//     var expected = [ undefined, undefined, undefined ];
//     test.identical( got, expected );
//     test.true( _.arrayIs( got ) );
//
//     test.case = 'src = BufferTyped constructor, ins = long';
//     var ins = _.argumentsArray.make( [ 1, 1, 1, 1, 1 ] );
//     var got = long.long.make( U32x, ins );
//     var expected = new U32x( [ 1, 1, 1, 1, 1 ] );
//     test.identical( got, expected );
//     test.true( _.buffer.typedIs(  got ) );
//     test.true( got !== ins );
//   }
// }
//
// longMakeWithArgumentsArrayLongDescriptor.timeOut = 10000;
//
// //
//
// function longMakeWithBufferTypedLongDescriptor( test )
// {
//   var list =
//   [
//     I8x,
//     U16x,
//     F32x,
//     F64x,
//   ];
//
//   /* tests */
//
//   let times = 4;
//   // for( let e in _.LongDescriptors )
//   // {
//   //   let name = _.LongDescriptors[ e ].name;
//   //   let long = _.withLong[ name ];
//
//   for( let k in _.long.namespaces )
//   {
//     let namespace = _.long.namespaces[ k ];
//     let name = namespace.TypeName;
//     let long = _.withLong[ name ];
//
//     for( let i = 0; i < list.length; i++ )
//     {
//       test.open( `long - ${ name }, makeLong - ${ list[ i ].name }` );
//       testRun( long, list[ i ] );
//       test.close( `long - ${ name }, makeLong - ${ list[ i ].name }` );
//     }
//
//     if( times < 1 )
//     break;
//     times--;
//   }
//
//   /* test subroutine */
//
//   function testRun( long, makeLong )
//   {
//     test.case = 'src = null, not ins';
//     var got = long.long.make( null );
//     var expected = long.long.default.make( 0 );
//     test.identical( got, expected );
//
//     test.case = 'src = number, not ins';
//     var got = long.long.make( 5 );
//     var expected = long.long.default.make( 5 );
//     test.identical( got, expected );
//
//     /* aaa : should throw error : add test cases */ /* Dmytro : added in new test routines */
//     // test.case = 'src = number, ins = null';
//     // var got = long.long.make( 5, null );
//     // var expected = long.long.default.make( 5 );
//     // test.identical( got, expected );
//
//     test.case = 'src = null, ins = number';
//     var got = long.long.make( null, 5 );
//     var expected = long.long.default.make( 5 );
//     test.identical( got, expected );
//
//     /* aaa : should throw error : add test cases */ /* Dmytro : added in new test routines */
//     // test.case = 'src = number, ins = undefined';
//     // var got = long.long.make( 5, undefined );
//     // var expected = long.long.default.make( 5 );
//     // test.identical( got, expected );
//
//     test.case = 'src = empty long, not ins';
//     var src = new makeLong( [] );
//     var got = long.long.make( src );
//     var expected = new makeLong( [] );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src = long, not ins';
//     var src = new makeLong( [ 1, 2, 3 ] );
//     var got = long.long.make( src );
//     var expected = new makeLong( [ 1, 2, 3 ] );
//     test.identical( got, expected );
//     test.identical( got.length, 3 );
//     test.true( got !== src );
//
//     test.case = 'src = empty long, ins = number';
//     var src = new makeLong( [] );
//     var got = long.long.make( src, 2 );
//     var expected = new makeLong( 2 );
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( _.buffer.typedIs( got ) );
//
//     /* aaa : should throw error : add test cases */ /* Dmytro : added in new test routines */
//     // test.case = 'src = long, ins = null';
//     // var src = new makeLong( [ 1, 2, 3 ] );
//     // var got = long.long.make( src, null );
//     // var expected = new makeLong( [ 1, 2, 3 ] );
//     // test.identical( got, expected );
//     // test.true( got !== src );
//     // test.true( _.buffer.typedIs( got ) );
//
//     test.case = 'ins = number, ins < src.length';
//     var src = new makeLong( [ 1, 2, 3 ] );
//     var got = long.long.make( src, 2 );
//     var expected = new makeLong( [ 1, 2 ] );
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( _.buffer.typedIs( got ) );
//
//     test.case = 'ins = number, ins > src.length';
//     var src = new makeLong( [ 1, 2, 3 ] );
//     var got = long.long.make( src, 4 );
//     var expected = new makeLong( [ 1, 2, 3, 0 ] )
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( _.buffer.typedIs( got ) );
//
//     test.case = 'ins = long, ins.length > src.length';
//     var src = new makeLong( [ 0, 1 ] );
//     var ins = [ 1, 2, 3 ];
//     var got = long.long.make( src, ins );
//     var expected = new makeLong( [ 1, 2, 3 ] );
//     test.identical( got, expected );
//     test.true( got !== ins );
//     test.true( got !== src );
//     test.true( _.buffer.typedIs( got ) );
//
//     test.case = 'src = new long, ins = array'
//     var src = new makeLong( 2 );
//     var ins = [ 1, 2, 3, 4, 5 ];
//     var got = long.long.make( src, ins );
//     var expected = new makeLong( [ 1, 2, 3, 4, 5 ] );
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( _.buffer.typedIs( got ) );
//
//     // test.case = 'src = Array constructor, ins = null';
//     // var got = long.long.make( Array, null );
//     // var expected = [];
//     // test.identical( got, expected );
//     // test.true( _.arrayIs( got ) );
//
//     test.case = 'src = Array constructor, ins = number';
//     var got = long.long.make( Array, 3 );
//     var expected = [ undefined, undefined, undefined ];
//     test.identical( got, expected );
//     test.true( _.arrayIs( got ) );
//
//     test.case = 'src = BufferTyped constructor, ins = long';
//     var ins = new makeLong( [ 1, 1, 1, 1, 1 ] );
//     var got = long.long.make( U32x, ins );
//     var expected = new U32x( [ 1, 1, 1, 1, 1 ] );
//     test.identical( got, expected );
//     test.true( _.buffer.typedIs(  got ) );
//     test.true( got !== ins );
//
//     test.case = 'src = long constructor, ins = number';
//     var src = makeLong( 5 );
//     var got = long.long.make( src.constructor, 2 );
//     var expected = new makeLong([ undefined, undefined ]);
//     test.identical( got, expected );
//     test.true( got !== ins );
//
//     // test.case = 'src = long constructor, ins = null';
//     // var src = new makeLong( 5 );
//     // var got = long.long.make( src.constructor, null );
//     // var expected = new makeLong();
//     // test.identical( got, expected );
//     // test.true( got !== ins );
//     //
//     // test.case = 'src = long constructor, ins = undefined';
//     // var src = new makeLong( 5 );
//     // var got = long.long.make( src.constructor, undefined );
//     // var expected = new makeLong( 0 );
//     // test.identical( got, expected );
//     // test.true( got !== ins );
//
//   }
// }
//
// longMakeWithBufferTypedLongDescriptor.timeOut = 20000;

//

function makeEmptyCommon( test )
{
  act({ tools : 'default', type : 'Array' });
  act({ tools : 'Array', type : 'Array' });
  act({ tools : 'F32x', type : 'F32x' });
  act({ tools : 'bufferTyped', type : 'Array' });

  /* */

  function act( env )
  {
    test.open( `${__.entity.exportStringSolo( env )}` );

    const long = namespaceGet( env );
    const Constructor = defaultConstructorGet( env );

    /* */

    test.case = `no args`;
    var got = long.makeEmpty();
    test.true( got instanceof Constructor );
    test.identical( got.length, 0 );

    /* */

    test.case = `empty array`;
    var src = [];
    var got = long.makeEmpty( src );
    if( env.tools === 'bufferTyped' )
    test.true( got instanceof Constructor );
    else
    test.true( _.array.is( got ) );
    test.identical( got.length, 0 );
    test.true( got !== src );

    test.case = `array - filled`;
    var src = [ 2, 2 ];
    var got = long.makeEmpty( src );
    if( env.tools === 'bufferTyped' )
    test.true( got instanceof Constructor );
    else
    test.true( _.array.is( got ) );
    test.identical( got.length, 0 );
    test.true( got !== src );

    /* */

    test.case = `empty unroll`;
    var src = _.unroll.make( [] );
    var got = long.makeEmpty( src );
    if( env.tools === 'bufferTyped' )
    test.true( got instanceof Constructor );
    else
    test.true( _.unroll.is( got ) );
    test.identical( got.length, 0 );
    test.true( got !== src );

    test.case = `unroll - filled`;
    var src = _.unroll.make( [ 2, 2 ] );
    var got = long.makeEmpty( src );
    if( env.tools === 'bufferTyped' )
    test.true( got instanceof Constructor );
    else
    test.true( _.unroll.is( got ) );
    test.identical( got.length, 0 );
    test.true( got !== src );

    /* */

    test.case = `empty argumentsArray`;
    var src = _.argumentsArray.make( [] );
    var got = long.makeEmpty( src );
    if( env.tools === 'bufferTyped' )
    test.true( got instanceof Constructor );
    else
    test.true( _.argumentsArray.is( got ) );
    test.identical( got.length, 0 );
    test.true( got !== src );

    test.case = `argumentsArray - filled`;
    var src = _.argumentsArray.make( [ 2, 2 ] );
    var got = long.makeEmpty( src );
    if( env.tools === 'bufferTyped' )
    test.true( got instanceof Constructor );
    else
    test.true( _.argumentsArray.is( got ) );
    test.identical( got.length, 0 );
    test.true( got !== src );

    /* */

    test.case = `empty typed buffer`;
    var src = _.u8x.make( [] );
    var got = long.makeEmpty( src );
    test.true( got instanceof U8x );
    test.identical( got.length, 0 );
    test.true( got !== src );

    test.case = `typed buffer - filled`;
    var src = _.f32x.make( [ 2, 2 ] );
    var got = long.makeEmpty( src );
    test.true( got instanceof F32x );
    test.identical( got.length, 0 );
    test.true( got !== src );

    /* */

    test.case = `filled countable`;
    var src = __.diagnostic.objectMake({ elements : [ 2, 3 ], countable : 1 });
    var got = long.makeEmpty( src );
    test.true( got instanceof Constructor );
    test.identical( got.length, 0 );

    /* */

    if( Config.debug )
    {
      test.case = 'extra arguments';
      test.shouldThrowErrorSync( () => long.makeEmpty( [], 1 ) );

      test.case = 'wrong type of src';
      test.shouldThrowErrorSync( () => long.makeEmpty( undefined ) );
      test.shouldThrowErrorSync( () => long.makeEmpty( null ) );
    }

    test.close( `${__.entity.exportStringSolo( env )}` );
  }

  /* */

  function namespaceGet( env )
  {
    if( env.tools === 'default' )
    return _.long;
    if( env.tools === 'bufferTyped' )
    return _.bufferTyped;
    return _.withLong[ env.tools ].long;
  }

  /* */

  function defaultConstructorGet( env )
  {
    if( env.tools === 'default' )
    return _.long.default.InstanceConstructor;
    if( env.tools === 'bufferTyped' )
    return _.bufferTyped.default.InstanceConstructor;
    return _.withLong[ env.tools ].long.default.InstanceConstructor;
  }
}

//

function makeEmptyCommonWithLongDescriptor( test )
{
  let times = 4;
  for( let k in _.long.namespaces )
  {
    let namespace = _.long.namespaces[ k ];
    let type = namespace.TypeName;

    if( type === 'ArgumentsArray' )
    continue;

    test.open( `long - ${ type }` );
    act({ tools : 'default', type });
    act({ tools : 'Array', type });
    act({ tools : 'F32x', type });
    act({ tools : 'bufferTyped', type });
    test.close( `long - ${ type }` );

    if( times < 1 )
    break;
    times--;
  }

  /* */

  function act( env )
  {
    test.open( `${__.entity.exportStringSolo( env )}` );

    const long = namespaceGet( env );
    const Constructor = defaultConstructorGet( env );

    /* */

    test.case = `no args`;
    var got = long.makeEmpty();
    test.true( got instanceof Constructor );
    test.identical( got.length, 0 );

    /* */

    test.case = `empty array`;
    var src = [];
    var got = long.makeEmpty( src );
    if( env.tools === 'bufferTyped' )
    test.true( got instanceof Constructor );
    else
    test.true( _.array.is( got ) );
    test.identical( got.length, 0 );
    test.true( got !== src );

    test.case = `array - filled`;
    var src = [ 2, 2 ];
    var got = long.makeEmpty( src );
    if( env.tools === 'bufferTyped' )
    test.true( got instanceof Constructor );
    else
    test.true( _.array.is( got ) );
    test.identical( got.length, 0 );
    test.true( got !== src );

    /* */

    test.case = `empty unroll`;
    var src = _.unroll.make( [] );
    var got = long.makeEmpty( src );
    if( env.tools === 'bufferTyped' )
    test.true( got instanceof Constructor );
    else
    test.true( _.unroll.is( got ) );
    test.identical( got.length, 0 );
    test.true( got !== src );

    test.case = `unroll - filled`;
    var src = _.unroll.make( [ 2, 2 ] );
    var got = long.makeEmpty( src );
    if( env.tools === 'bufferTyped' )
    test.true( got instanceof Constructor );
    else
    test.true( _.unroll.is( got ) );
    test.identical( got.length, 0 );
    test.true( got !== src );

    /* */

    test.case = `empty argumentsArray`;
    var src = _.argumentsArray.make( [] );
    var got = long.makeEmpty( src );
    if( env.tools === 'bufferTyped' )
    test.true( got instanceof Constructor );
    else
    test.true( _.argumentsArray.is( got ) );
    test.identical( got.length, 0 );
    test.true( got !== src );

    test.case = `argumentsArray - filled`;
    var src = _.argumentsArray.make( [ 2, 2 ] );
    var got = long.makeEmpty( src );
    if( env.tools === 'bufferTyped' )
    test.true( got instanceof Constructor );
    else
    test.true( _.argumentsArray.is( got ) );
    test.identical( got.length, 0 );
    test.true( got !== src );

    /* */

    test.case = `empty typed buffer`;
    var src = _.u8x.make( [] );
    var got = long.makeEmpty( src );
    test.true( got instanceof U8x );
    test.identical( got.length, 0 );
    test.true( got !== src );

    test.case = `typed buffer - filled`;
    var src = _.f32x.make( [ 2, 2 ] );
    var got = long.makeEmpty( src );
    test.true( got instanceof F32x );
    test.identical( got.length, 0 );
    test.true( got !== src );

    /* */

    test.case = `filled countable`;
    var src = __.diagnostic.objectMake({ elements : [ 2, 3 ], countable : 1 });
    var got = long.makeEmpty( src );
    test.true( got instanceof Constructor );
    test.identical( got.length, 0 );

    test.close( `${__.entity.exportStringSolo( env )}` );
  }

  /* */

  function namespaceGet( env )
  {
    if( env.tools === 'default' )
    return _.long;
    if( env.tools === 'bufferTyped' )
    return _.bufferTyped;
    return _.withLong[ env.tools ].long;
  }

  /* */

  function defaultConstructorGet( env )
  {
    if( env.tools === 'default' )
    return _.long.default.InstanceConstructor;
    if( env.tools === 'bufferTyped' )
    return _.bufferTyped.default.InstanceConstructor;
    return _.withLong[ env.tools ].long.default.InstanceConstructor;
  }
}

// function longMakeEmptyWithArrayAndUnroll( test )
// {
//   var array = ( src ) => _.array.make( src );
//   var unroll = ( src ) => _.unroll.make( src );
//   var longConstr = ( src ) =>
//   {
//     if( src )
//     return _.long.default.make( src );
//     return _.long.default.make( 0 );
//   };
//   var list =
//   [
//     array,
//     unroll,
//     longConstr,
//     Array,
//   ];
//
//   /* tests */
//
//   for( let i = 0; i < list.length; i++ )
//   {
//     test.open( list[ i ].name );
//     testRun( list[ i ] );
//     test.close( list[ i ].name );
//   }
//
//   /* test subroutine */
//
//   function testRun( makeLong )
//   {
//     test.case = 'without arguments';
//     var got = _.long.makeEmpty();
//     var expected = _.long.default.make( 0 );
//     test.identical( got, expected );
//
//     // test.case = 'src - null';
//     // var got = _.long.makeEmpty( null );
//     // var expected = _.long.default.make( 0 );
//     // test.identical( got, expected );
//
//     test.case = 'src - empty long';
//     var src = makeLong( [] );
//     var got = _.long.makeEmpty( src );
//     var expected = makeLong( 0 );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src - filled long';
//     var src = makeLong( [ 1, 2, 3, 4, 5 ] );
//     var got = _.long.makeEmpty( src );
//     var expected = makeLong( 0 );
//     test.identical( got, expected );
//     test.true( got !== src );
//   }
//
//   test.case = 'src - Array, constructor';
//   var got = _.long.makeEmpty( Array );
//   var expected = [];
//   test.identical( got, expected );
//
//   /* - */
//
//   if( !Config.debug )
//   return;
//
//   test.case = 'extra argument';
//   test.shouldThrowErrorSync( () => _.long.makeEmpty( [ 1, 2, 3 ], 'extra argument' ) );
//
//   test.case = 'wrong type of src';
//   test.shouldThrowErrorSync( () => _.long.makeEmpty( 1 ) );
//   test.shouldThrowErrorSync( () => _.long.makeEmpty( undefined ) );
//   test.shouldThrowErrorSync( () => _.long.makeEmpty( 'wrong argument' ) );
//   test.shouldThrowErrorSync( () => _.long.makeEmpty( new BufferRaw( 3 ) ) );
//   if( Config.interpreter === 'njs' )
//   test.shouldThrowErrorSync( () => _.long.makeEmpty( BufferNode.alloc( 3 ) ) );
//
//   test.case = 'src is function without constructor';
//   test.shouldThrowErrorSync( () => _.long.makeEmpty( ( e ) => _.array.make( e ) ) );
// }
//
// //
//
// function longMakeEmptyWithArgumentsArray( test )
// {
//   test.case = 'without arguments';
//   var got = _.long.makeEmpty();
//   var expected = _.long.default.make( 0 );
//   test.identical( got, expected );
//
//   // test.case = 'src - null';
//   // var got = _.long.makeEmpty( null );
//   // var expected = _.long.default.make( 0 );
//   // test.identical( got, expected );
//
//   test.case = 'src - empty long';
//   var src = _.argumentsArray.make( [] );
//   var got = _.long.makeEmpty( src );
//   var expected = _.argumentsArray.make( 0 );
//   test.identical( got, expected );
//   test.true( got !== src );
//
//   test.case = 'src - filled long';
//   var src = _.argumentsArray.make( [ 1, 2, 3, 4, 5 ] );
//   var got = _.long.makeEmpty( src );
//   var expected = _.argumentsArray.make( 0 );
//   test.identical( got, expected );
//   test.true( got !== src );
// }
//
// //
//
// function longMakeEmptyWithBufferTyped( test )
// {
//   var list =
//   [
//     I8x,
//     U16x,
//     F32x,
//     F64x,
//   ];
//
//   /* tests */
//
//   for( let i = 0; i < list.length; i++ )
//   {
//     test.open( list[ i ].name );
//     testRun( list[ i ] );
//     test.close( list[ i ].name );
//   }
//
//   /* test subroutine */
//
//   function testRun( makeLong )
//   {
//     test.case = 'without arguments';
//     var got = _.long.makeEmpty();
//     var expected = _.long.default.make( 0 );
//     test.identical( got, expected );
//
//     // test.case = 'src - null';
//     // var got = _.long.makeEmpty( null );
//     // var expected = _.long.default.make( 0 );
//     // test.identical( got, expected );
//
//     test.case = 'src - empty long';
//     var src = new makeLong( [] );
//     var got = _.long.makeEmpty( src );
//     var expected = new makeLong( 0 );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src - filled long';
//     var src = new makeLong( [ 1, 2, 3, 4, 5 ] );
//     var got = _.long.makeEmpty( src );
//     var expected = new makeLong( 0 );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src - constructor';
//     var got = _.long.makeEmpty( makeLong );
//     var expected = new makeLong( 0 );
//     test.identical( got, expected );
//   }
// }
//
// //
//
// function longMakeEmptyWithArrayAndUnrollLongDescriptor( test )
// {
//   var array = ( src ) => _.array.make( src );
//   var unroll = ( src ) => _.unroll.make( src );
//   var longConstr = ( src ) =>
//   {
//     if( src )
//     return _.long.default.make( src );
//     return _.long.default.make( 0 );
//   }
//   var list =
//   [
//     array,
//     unroll,
//     longConstr,
//     Array,
//   ];
//
//   /* tests */
//
//   let times = 4;
//   // for( let e in _.LongDescriptors )
//   // {
//   //   let name = _.LongDescriptors[ e ].name;
//   //   let long = _.withLong[ name ];
//
//   for( let k in _.long.namespaces )
//   {
//     let namespace = _.long.namespaces[ k ];
//     let name = namespace.TypeName;
//     let long = _.withLong[ name ];
//
//     for( let i = 0; i < list.length; i++ )
//     {
//       test.open( `long - ${ name }, makeLong - ${ list[ i ].name }` );
//       testRun( long, list[ i ] );
//       test.close( `long - ${ name }, makeLong - ${ list[ i ].name }` );
//     }
//
//     if( times < 1 )
//     break;
//     times--;
//   }
//
//   /* test subroutine */
//
//   function testRun( long, makeLong )
//   {
//     test.case = 'without arguments';
//     var got = long.long.makeEmpty();
//     var expected = long.long.default.make( 0 );
//     test.identical( got, expected );
//
//     test.case = 'src - null';
//     var got = long.long.makeEmpty( null );
//     var expected = long.long.default.make( 0 );
//     test.identical( got, expected );
//
//     test.case = 'src - empty long';
//     var src = makeLong( [] );
//     var got = long.long.makeEmpty( src );
//     var expected = makeLong( 0 );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src - filled long';
//     var src = makeLong( [ 1, 2, 3, 4, 5 ] );
//     var got = long.long.makeEmpty( src );
//     var expected = makeLong( 0 );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src - Array, constructor';
//     var got = long.long.makeEmpty( Array );
//     var expected = [];
//     test.identical( got, expected );
//
//     /* - */
//
//     if( Config.debug )
//     {
//       test.case = 'extra argument';
//       test.shouldThrowErrorSync( () => long.long.makeEmpty( [ 1, 2, 3 ], 'extra argument' ) );
//
//       test.case = 'wrong type of src';
//       test.shouldThrowErrorSync( () => long.long.makeEmpty( 1 ) );
//       test.shouldThrowErrorSync( () => long.long.makeEmpty( undefined ) );
//       test.shouldThrowErrorSync( () => long.long.makeEmpty( 'wrong argument' ) );
//       test.shouldThrowErrorSync( () => long.long.makeEmpty( new BufferRaw( 3 ) ) );
//       if( Config.interpreter === 'njs' )
//       test.shouldThrowErrorSync( () => long.long.makeEmpty( BufferNode.alloc( 3 ) ) );
//     }
//   }
// }
//
// longMakeEmptyWithArrayAndUnrollLongDescriptor.timeOut = 20000;
//
// //
//
// function longMakeEmptyWithArgumentsArrayLongDescriptor( test )
// {
//   let times = 4;
//
//   // for( let e in _.LongDescriptors )
//   // {
//   //   let name = _.LongDescriptors[ e ].name;
//   //   let long = _.withLong[ name ];
//
//   for( let k in _.long.namespaces )
//   {
//     let namespace = _.long.namespaces[ k ];
//     let name = namespace.TypeName;
//     let long = _.withLong[ name ];
//
//     test.open( `long - ${ name }` );
//     testRun( long );
//     test.close( `long - ${ name }` );
//
//     if( times < 1 )
//     break;
//     times--;
//   }
//
//   /* - */
//
//   function testRun( long )
//   {
//     test.case = 'without arguments';
//     var got = long.long.makeEmpty();
//     var expected = long.long.default.make( 0 );
//     test.identical( got, expected );
//
//     test.case = 'src - null';
//     var got = long.long.makeEmpty( null );
//     var expected = long.long.default.make( 0 );
//     test.identical( got, expected );
//
//     test.case = 'src - empty long';
//     var src = _.argumentsArray.make( [] );
//     var got = long.long.makeEmpty( src );
//     var expected = long.long.default.make( 0 );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src - filled long';
//     var src = _.argumentsArray.make( [ 1, 2, 3, 4, 5 ] );
//     var got = long.long.makeEmpty( src );
//     var expected = long.long.default.make( 0 );
//     test.identical( got, expected );
//     test.true( got !== src );
//   }
// }
//
// //
//
// function longMakeEmptyWithBufferTypedLongDescriptor( test )
// {
//   var list =
//   [
//     I8x,
//     U16x,
//     F32x,
//     F64x,
//   ];
//
//   /* tests */
//
//   let times = 4;
//
//   // for( let e in _.LongDescriptors )
//   // {
//   //   let name = _.LongDescriptors[ e ].name;
//   //   let long = _.withLong[ name ];
//
//   for( let k in _.long.namespaces )
//   {
//     let namespace = _.long.namespaces[ k ];
//     let name = namespace.TypeName;
//     let long = _.withLong[ name ];
//
//     for( let i = 0; i < list.length; i++ )
//     {
//       test.open( `long - ${ name }, makeLong - ${ list[ i ].name }` );
//       testRun( long, list[ i ] );
//       test.close( `long - ${ name }, makeLong - ${ list[ i ].name }` );
//     }
//
//     if( times < 1 )
//     break;
//     times--;
//   }
//
//   /* test subroutine */
//
//   function testRun( long, makeLong )
//   {
//     test.case = 'without arguments';
//     var got = long.long.makeEmpty();
//     var expected = long.long.default.make( 0 );
//     test.identical( got, expected );
//
//     test.case = 'src - null';
//     var got = long.long.makeEmpty( null );
//     var expected = long.long.default.make( 0 );
//     test.identical( got, expected );
//
//     test.case = 'src - empty long';
//     var src = new makeLong( [] );
//     var got = long.long.makeEmpty( src );
//     var expected = new makeLong( 0 );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src - filled long';
//     var src = new makeLong( [ 1, 2, 3, 4, 5 ] );
//     var got = long.long.makeEmpty( src );
//     var expected = new makeLong( 0 );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src - filled long';
//     var got = long.long.makeEmpty( makeLong );
//     var expected = new makeLong( 0 );
//     test.identical( got, expected );
//   }
// }
//
// longMakeEmptyWithBufferTypedLongDescriptor.timeOut = 20000;

//

// function _longMakeOfLengthWithArrayAndUnroll( test )
// {
//   var array = ( src ) => _.array.make( src );
//   var unroll = ( src ) => _.unroll.make( src );
//   var longConstr = ( src ) =>
//   {
//     if( src )
//     return _.long.default.make( src );
//     return _.long.default.make( 0 );
//   }
//   var list =
//   [
//     array,
//     unroll,
//     longConstr,
//   ];
//
//   /* tests */
//
//   for( let t = 0; t < list.length; t++ )
//   {
//     test.open( list[ t ].name );
//     testRun( list[ t ] );
//     test.close( list[ t ].name );
//   }
//
//   /* test subroutine */
//
//   function testRun( makeLong )
//   {
//     test.case = 'src = null, not ins';
//     var got = _._longMakeOfLength( null );
//     var expected = [];
//     test.identical( got, expected );
//
//     test.case = 'src = number, not ins';
//     var got = _._longMakeOfLength( 5 );
//     var expected = _.long.default.make( 5 );
//     test.identical( got, expected );
//
//     /* aaa : should throw error : add test cases */ /* Dmytro : added in new test routines */
//     // test.case = 'src = number, ins = null';
//     // var got = _._longMakeOfLength( 5, null );
//     // var expected = _.long.default.make( 5 );
//     // test.identical( got, expected );
//     //
//     // test.case = 'src = number, ins = undefined';
//     // var got = _._longMakeOfLength( 5, undefined );
//     // var expected = _.long.default.make( 5 );
//     // test.identical( got, expected );
//
//     test.case = 'src = null, ins - number';
//     var got = _._longMakeOfLength( null, 5 );
//     var expected = _.long.default.make( 5 );
//     test.identical( got, expected );
//
//     test.case = 'src = null, ins - long';
//     var got = _._longMakeOfLength( null, makeLong( [ 1, 2, 3, 4, 5 ] ) );
//     var expected = _.long.default.make( 5 );
//     test.identical( got, expected );
//
//     test.case = 'src = empty long, not ins';
//     var src = makeLong( [] );
//     var got = _._longMakeOfLength( src );
//     var expected = makeLong( [] );
//     test.identical( got, expected );
//
//     test.case = 'src = long, not ins';
//     var src = makeLong( [ 1, 2, 3 ] );
//     var got = _._longMakeOfLength( src );
//     var expected = makeLong([ 1, 2, 3 ]);
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( src.constructor.name === got.constructor.name );
//
//     test.case = 'src = empty long, ins = null';
//     var src = makeLong( [] );
//     var got = _._longMakeOfLength( src, null );
//     var expected = makeLong( 0 );
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( src.constructor.name === got.constructor.name );
//
//     test.case = 'src = long, ins = number';
//     var src = makeLong( 10 );
//     var got = _._longMakeOfLength( src.constructor, 4 );
//     var expected = makeLong( 4 );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src = empty long, ins = number';
//     var src = makeLong( [] );
//     var got = _._longMakeOfLength( src, 2 );
//     var expected = makeLong( 2 );
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( src.constructor.name === got.constructor.name );
//
//     test.case = 'src = long, ins = number, ins < src.length';
//     var src = makeLong( [ 1, 2, 3 ] );
//     var got = _._longMakeOfLength( src, 2 );
//     var expected = makeLong( [ 1, 2 ] );
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( src.constructor.name === got.constructor.name );
//
//     test.case = 'src = long with an element, ins = empty array';
//     var src = new F64x( 10 );
//     var got = _._longMakeOfLength( src, [] );
//     var expected = new F64x( 0 );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src = long, ins = number, ins > src.length';
//     var src = makeLong( [ 1, 2, 3 ] );
//     var got = _._longMakeOfLength( src, 4 );
//     var expected = makeLong( [ 1, 2, 3, undefined ] );
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( src.constructor.name === got.constructor.name );
//
//     test.case = 'src = long, ins = array, ins.length > src.length';
//     var src = makeLong( [ 0, 1 ] );
//     var ins = [ 1, 2, 3 ];
//     var got = _._longMakeOfLength( src, ins );
//     var expected = makeLong( [ 0, 1, undefined ] );
//     test.identical( got, expected );
//     test.true( got !== ins );
//     test.true( got !== src );
//     test.true( src.constructor.name === got.constructor.name );
//
//     test.case = 'src = long, ins = array, ins.length === src.length'
//     var src = makeLong( 5 );
//     var ins = [ 1, 2, 3, 4, 5 ];
//     var got = _._longMakeOfLength( src, ins );
//     var expected = makeLong( 5 );
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( src.constructor.name === got.constructor.name );
//
//     // test.case = 'src = Array constructor, ins = null';
//     // var got = _._longMakeOfLength( Array, null );
//     // var expected = [];
//     // test.identical( got, expected );
//     // test.true( _.arrayIs( got ) );
//
//     test.case = 'src = BufferTyped constructor, ins = number';
//     var got = _._longMakeOfLength( U32x, 5 );
//     var expected = new U32x( 5 );
//     test.identical( got, expected );
//     test.true( _.buffer.typedIs(  got ) );
//
//     test.case = 'src = Array constructor, ins = long';
//     var ins = makeLong( [ 1, 2, 3 ] );
//     var got = _._longMakeOfLength( Array, ins );
//     var expected = [ undefined, undefined, undefined ];
//     test.identical( got, expected );
//     test.true( _.arrayIs( got ) );
//     test.true( got !== ins );
//   }
//
//   /* - */
//
//   if( !Config.debug )
//   return;
//
//   test.case = 'without arguments';
//   test.shouldThrowErrorSync( () => _._longMakeOfLength() );
//
//   test.case = 'extra arguments';
//   test.shouldThrowErrorSync( () => _._longMakeOfLength( [ 1, 2, 3 ], 4, 'extra argument' ) );
//
//   test.case = 'wrong type of ins';
//   test.shouldThrowErrorSync( () => _._longMakeOfLength( 'wrong argument', 1 ) );
//   test.shouldThrowErrorSync( () => _._longMakeOfLength( 1, 1 ) );
//   test.shouldThrowErrorSync( () => _._longMakeOfLength( BufferNode.alloc( 3 ), 2 ) );
//   test.shouldThrowErrorSync( () => _._longMakeOfLength( new BufferRaw( 3 ), 2 ) );
//   test.shouldThrowErrorSync( () => _._longMakeOfLength( Array, BufferNode.from( [ 3 ] ) ) );
//
//   test.case = 'wrong type of len';
//   test.shouldThrowErrorSync( () => _._longMakeOfLength( [ 1, 2, 3 ], 'wrong type of argument' ) );
//   test.shouldThrowErrorSync( () => _._longMakeOfLength( [ 1, 2, 3 ], Infinity ) );
// }
//
// //
//
// function _longMakeOfLengthWithArgumentsArray( test )
// {
//   test.case = 'src = null, not ins';
//   var got = _._longMakeOfLength( null );
//   var expected = _.long.default.make( 0 );
//   test.identical( got, expected );
//
//   test.case = 'src = number, not ins';
//   var got = _._longMakeOfLength( 5 );
//   var expected = _.long.default.make( 5 );
//   test.identical( got, expected );
//
//   /* aaa : should throw error : add test cases */ /* Dmytro : added in new test routines */
//   // test.case = 'src = number, ins = null';
//   // var got = _._longMakeOfLength( 5, null );
//   // var expected = _.long.default.make( 5 );
//   // test.identical( got, expected );
//   //
//   // test.case = 'src = number, ins = undefined';
//   // var got = _._longMakeOfLength( 5, undefined );
//   // var expected = _.long.default.make( 5 );
//   // test.identical( got, expected );
//
//   test.case = 'src = null, ins - number';
//   var got = _._longMakeOfLength( null, 5 );
//   var expected = _.long.default.make( 5 );
//   test.identical( got, expected );
//
//   test.case = 'src = null, ins - long';
//   var got = _._longMakeOfLength( null, _.argumentsArray.make( [ 1, 2, 3, 4, 5 ] ) );
//   var expected = _.long.default.make( 5 );
//   test.identical( got, expected );
//
//   test.case = 'src = empty long, not ins';
//   var src = _.argumentsArray.make( [] );
//   var got = _._longMakeOfLength( src );
//   var expected = _.long.default.make( [] );
//   test.identical( got, expected );
//
//   test.case = 'src = long, not ins';
//   var src = _.argumentsArray.make( [ 1, 2, 3 ] );
//   var got = _._longMakeOfLength( src );
//   var expected = _.long.default.make( 3 );
//   test.identical( got, expected );
//   test.true( got !== src );
//
//   test.case = 'src = empty long, ins = null';
//   var src = _.argumentsArray.make( [] );
//   var got = _._longMakeOfLength( src, null );
//   var expected = _.long.default.make( 0 );
//   test.identical( got, expected );
//   test.true( got !== src );
//
//   test.case = 'src = empty long, ins = number';
//   var src = _.argumentsArray.make( [] );
//   var got = _._longMakeOfLength( src, 2 );
//   var expected = _.long.default.make( 2 );
//   test.identical( got, expected );
//   test.true( got !== src );
//
//   test.case = 'src = long, ins = number, ins < src.length';
//   var src = _.argumentsArray.make( [ 1, 2, 3 ] );
//   var got = _._longMakeOfLength( src, 2 );
//   var expected = _.long.default.make( 2 );
//   test.identical( got, expected );
//   test.true( got !== src );
//
//   test.case = 'src = long with an element, ins = empty array';
//   var src = new F64x( 10 );
//   var got = _._longMakeOfLength( src, [] );
//   var expected = new F64x( 0 );
//   test.identical( got, expected );
//   test.true( got !== src );
//
//   test.case = 'src = long, ins = number, ins > src.length';
//   var src = _.argumentsArray.make( [ 1, 2, 3 ] );
//   var got = _._longMakeOfLength( src, 4 );
//   var expected = _.long.default.make( 4 );
//   test.identical( got, expected );
//   test.true( got !== src );
//
//   test.case = 'src = long, ins = array, ins.length > src.length';
//   var src = _.argumentsArray.make( [ 0, 1 ] );
//   var ins = [ 1, 2, 3 ];
//   var got = _._longMakeOfLength( src, ins );
//   var expected = _.long.default.make( 3 );
//   test.identical( got, expected );
//   test.true( got !== ins );
//   test.true( got !== src );
//
//   test.case = 'src = long, ins = array, ins.length === src.length'
//   var src = _.argumentsArray.make( 5 );
//   var ins = [ 1, 2, 3, 4, 5 ];
//   var got = _._longMakeOfLength( src, ins );
//   var expected = _.long.default.make( 5 );
//   test.identical( got, expected );
//   test.true( got !== src );
//
//   // test.case = 'src = Array constructor, ins = null';
//   // var got = _._longMakeOfLength( Array, null );
//   // var expected = [];
//   // test.identical( got, expected );
//   // test.true( _.arrayIs( got ) );
//
//   test.case = 'src = BufferTyped constructor, ins = number';
//   var got = _._longMakeOfLength( U32x, 5 );
//   var expected = new U32x( 5 );
//   test.identical( got, expected );
//   test.true( _.buffer.typedIs(  got ) );
//
//   test.case = 'src = Array constructor, ins = long';
//   var ins = _.argumentsArray.make( [ 1, 2, 3 ] );
//   var got = _._longMakeOfLength( Array, ins );
//   var expected = [ undefined, undefined, undefined ];
//   test.identical( got, expected );
//   test.true( _.arrayIs( got ) );
//   test.true( got !== ins );
// }
//
// //
//
// function _longMakeOfLengthWithBufferTyped( test )
// {
//   var list =
//   [
//     I8x,
//     U16x,
//     U16x,
//     F32x,
//   ];
//
//   /* tests */
//
//   for( let t = 0; t < list.length; t++ )
//   {
//     test.open( list[ t ].name );
//     testRun( list[ t ] );
//     test.close( list[ t ].name );
//   }
//
//   /* test subroutine */
//
//   function testRun( makeLong )
//   {
//     test.case = 'src = null, not ins';
//     var got = _._longMakeOfLength( null );
//     var expected = [];
//     test.identical( got, expected );
//
//     test.case = 'src = number, not ins';
//     var got = _._longMakeOfLength( 5 );
//     var expected = _.long.default.make( 5 );
//     test.identical( got, expected );
//
//     /* aaa : should throw error : add test cases */ /* Dmytro : added in new test routines */
//     // test.case = 'src = number, ins = null';
//     // var got = _._longMakeOfLength( 5, null );
//     // var expected = _.long.default.make( 5 );
//     // test.identical( got, expected );
//     //
//     // test.case = 'src = number, ins = undefined';
//     // var got = _._longMakeOfLength( 5, undefined );
//     // var expected = _.long.default.make( 5 );
//     // test.identical( got, expected );
//
//     test.case = 'src = null, ins - number';
//     var got = _._longMakeOfLength( null, 5 );
//     var expected = _.long.default.make( 5 );
//     test.identical( got, expected );
//
//     test.case = 'src = null, ins - long';
//     var got = _._longMakeOfLength( null, new makeLong( [ 1, 2, 3, 4, 5 ] ) );
//     var expected = _.long.default.make( 5 );
//     test.identical( got, expected );
//
//     test.case = 'src = empty long, not ins';
//     var src = new makeLong( [] );
//     var got = _._longMakeOfLength( src );
//     var expected = new makeLong( [] );
//     test.identical( got, expected );
//
//     test.case = 'src = long, not ins';
//     var src = new makeLong( [ 1, 2, 3 ] );
//     var got = _._longMakeOfLength( src );
//     var expected = new makeLong([ 1, 2, 3 ]);
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( src.constructor.name === got.constructor.name );
//
//     test.case = 'src = empty long, ins = null';
//     var src = new makeLong( [] );
//     var got = _._longMakeOfLength( src, null );
//     var expected = new makeLong( 0 );
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( src.constructor.name === got.constructor.name );
//
//     test.case = 'src = long, ins = number';
//     var src = new makeLong( 10 );
//     var got = _._longMakeOfLength( src.constructor, 4 );
//     var expected = new makeLong( 4 );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src = empty long, ins = number';
//     var src = new makeLong( [] );
//     var got = _._longMakeOfLength( src, 2 );
//     var expected = new makeLong( 2 );
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( src.constructor.name === got.constructor.name );
//
//     test.case = 'src = long, ins = number, ins < src.length';
//     var src = new makeLong( [ 1, 2, 3 ] );
//     var got = _._longMakeOfLength( src, 2 );
//     var expected = new makeLong( [ 1, 2 ] );
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( src.constructor.name === got.constructor.name );
//
//     test.case = 'src = long with an element, ins = empty array';
//     var src = new F64x( 10 );
//     var got = _._longMakeOfLength( src, [] );
//     var expected = new F64x( 0 );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src = long, ins = number, ins > src.length';
//     var src = new makeLong( [ 1, 2, 3 ] );
//     var got = _._longMakeOfLength( src, 4 );
//     var expected = new makeLong( [ 1, 2, 3, 0 ] );
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( src.constructor.name === got.constructor.name );
//
//     test.case = 'src = long, ins = array, ins.length === src.length'
//     var src = new makeLong( 5 );
//     var ins = [ 1, 2, 3, 4, 5 ];
//     var got = _._longMakeOfLength( src, ins );
//     var expected = new makeLong([ 1, 2, 3, 4, 5 ]);
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( src.constructor.name === got.constructor.name );
//
//     // test.case = 'src = Array constructor, ins = null';
//     // var got = _._longMakeOfLength( Array, null );
//     // var expected = [];
//     // test.identical( got, expected );
//     // test.true( _.arrayIs( got ) );
//
//     test.case = 'src = BufferTyped constructor, ins = number';
//     var got = _._longMakeOfLength( U32x, 5 );
//     var expected = new U32x( 5 );
//     test.identical( got, expected );
//     test.true( _.buffer.typedIs(  got ) );
//
//     test.case = 'src = Array constructor, ins = long';
//     var ins = new makeLong( [ 1, 2, 3 ] );
//     var got = _._longMakeOfLength( Array, ins );
//     var expected = [ undefined, undefined, undefined ];
//     test.identical( got, expected );
//     test.true( _.arrayIs( got ) );
//     test.true( got !== ins );
//   }
// }
//
// //
//
// function _longMakeOfLengthWithArrayAndUnrollLongDescriptor( test )
// {
//   var array = ( src ) => _.array.make( src );
//   var unroll = ( src ) => _.unroll.make( src );
//   var longConstr = ( src ) =>
//   {
//     if( src )
//     return _.long.default.make( src );
//     return _.long.default.make( 0 );
//   }
//   var list =
//   [
//     array,
//     unroll,
//     longConstr,
//   ];
//
//   /* tests */
//
//   let times = 4;
//   for( let e in _.LongDescriptors )
//   {
//     let name = _.LongDescriptors[ e ].name;
//     let long = _.withLong[ name ];
//
//     for( let i = 0; i < list.length; i++ )
//     {
//       test.open( `long - ${ name }, makeLong - ${ list[ i ].name }` );
//       testRun( long, list[ i ] );
//       test.close( `long - ${ name }, makeLong - ${ list[ i ].name }` );
//     }
//
//     if( times < 1 )
//     break;
//     times--;
//   }
//
//   /* test subroutine */
//
//   function testRun( long, makeLong )
//   {
//     test.case = 'src = null, not ins';
//     var got = long._longMakeOfLength( null );
//     var expected = long.long.default.make( 0 );
//     test.identical( got, expected );
//
//     test.case = 'src = number, not ins';
//     var got = long._longMakeOfLength( 5 );
//     var expected = long.long.default.make( 5 );
//     test.identical( got, expected );
//
//     /* aaa : should throw error : add test cases */ /* Dmytro : added in new test routines */
//     // test.case = 'src = number, ins = null';
//     // var got = long._longMakeOfLength( 5, null );
//     // var expected = long.long.default.make( 5 );
//     // test.identical( got, expected );
//     //
//     // test.case = 'src = number, ins = undefined';
//     // var got = long._longMakeOfLength( 5, undefined );
//     // var expected = long.long.default.make( 5 );
//     // test.identical( got, expected );
//
//     test.case = 'src = null, ins - number';
//     var got = long._longMakeOfLength( null, 5 );
//     var expected = long.long.default.make( 5 );
//     test.identical( got, expected );
//
//     test.case = 'src = null, ins - long';
//     var got = long._longMakeOfLength( null, makeLong( [ 1, 2, 3, 4, 5 ] ) );
//     var expected = long.long.default.make( 5 );
//     test.identical( got, expected );
//
//     test.case = 'src = empty long, not ins';
//     var src = makeLong( [] );
//     var got = long._longMakeOfLength( src );
//     var expected = makeLong( [] );
//     test.identical( got, expected );
//
//     test.case = 'src = long, not ins';
//     var src = makeLong( [ 1, 2, 3 ] );
//     var got = long._longMakeOfLength( src );
//     var expected = makeLong([ 1, 2, 3 ]);
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src = empty long, ins = null';
//     var src = makeLong( [] );
//     var got = long._longMakeOfLength( src, null );
//     var expected = makeLong( 0 );
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( src.constructor.name === got.constructor.name );
//
//     test.case = 'src = long, ins = number';
//     var src = makeLong( 10 );
//     var got = long._longMakeOfLength( src.constructor, 4 );
//     var expected = makeLong( 4 );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src = empty long, ins = number';
//     var src = makeLong( [] );
//     var got = long._longMakeOfLength( src, 2 );
//     var expected = makeLong( 2 );
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( src.constructor.name === got.constructor.name );
//
//     test.case = 'src = long, ins = number, ins < src.length';
//     var src = makeLong( [ 1, 2, 3 ] );
//     var got = long._longMakeOfLength( src, 2 );
//     var expected = makeLong( [ 1, 2 ] );
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( src.constructor.name === got.constructor.name );
//
//     test.case = 'src = long with an element, ins = empty array';
//     var src = new F64x( 10 );
//     var got = long._longMakeOfLength( src, [] );
//     var expected = new F64x( 0 );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src = long, ins = number, ins > src.length';
//     var src = makeLong( [ 1, 2, 3 ] );
//     var got = long._longMakeOfLength( src, 4 );
//     var expected = makeLong( [ 1, 2, 3, undefined ] );
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( src.constructor.name === got.constructor.name );
//
//     test.case = 'src = long, ins = array, ins.length > src.length';
//     var src = makeLong( [ 0, 1 ] );
//     var ins = [ 1, 2, 3 ];
//     var got = long._longMakeOfLength( src, ins );
//     var expected = makeLong( [ 0, 1, undefined ] );
//     test.identical( got, expected );
//     test.true( got !== ins );
//     test.true( got !== src );
//     test.true( src.constructor.name === got.constructor.name );
//
//     test.case = 'src = long, ins = array, ins.length === src.length'
//     var src = makeLong( 5 );
//     var ins = [ 1, 2, 3, 4, 5 ];
//     var got = long._longMakeOfLength( src, ins );
//     var expected = makeLong( 5 );
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( src.constructor.name === got.constructor.name );
//
//     // test.case = 'src = Array constructor, ins = null';
//     // var got = long._longMakeOfLength( Array, null );
//     // var expected = [];
//     // test.identical( got, expected );
//     // test.true( _.arrayIs( got ) );
//
//     test.case = 'src = BufferTyped constructor, ins = number';
//     var got = long._longMakeOfLength( U32x, 5 );
//     var expected = new U32x( 5 );
//     test.identical( got, expected );
//     test.true( _.buffer.typedIs(  got ) );
//
//     test.case = 'src = Array constructor, ins = long';
//     var ins = makeLong( [ 1, 2, 3 ] );
//     var got = long._longMakeOfLength( Array, ins );
//     var expected = [ undefined, undefined, undefined ];
//     test.identical( got, expected );
//     test.true( _.arrayIs( got ) );
//     test.true( got !== ins );
//
//     /* - */
//
//     if( Config.debug )
//     {
//       test.case = 'without arguments';
//       test.shouldThrowErrorSync( () => long._longMakeOfLength() );
//
//       test.case = 'extra arguments';
//       test.shouldThrowErrorSync( () => long._longMakeOfLength( [ 1, 2, 3 ], 4, 'extra argument' ) );
//
//       test.case = 'wrong type of ins';
//       test.shouldThrowErrorSync( () => long._longMakeOfLength( 'wrong argument', 1 ) );
//       test.shouldThrowErrorSync( () => long._longMakeOfLength( 1, 1 ) );
//       test.shouldThrowErrorSync( () => long._longMakeOfLength( BufferNode.alloc( 3 ), 2 ) );
//       test.shouldThrowErrorSync( () => long._longMakeOfLength( new BufferRaw( 3 ), 2 ) );
//       test.shouldThrowErrorSync( () => long._longMakeOfLength( Array, BufferNode.from( [ 3 ] ) ) );
//
//       test.case = 'wrong type of len';
//       test.shouldThrowErrorSync( () => long._longMakeOfLength( [ 1, 2, 3 ], 'wrong type of argument' ) );
//       test.shouldThrowErrorSync( () => long._longMakeOfLength( [ 1, 2, 3 ], Infinity ) );
//     }
//   }
// }
//
// _longMakeOfLengthWithArrayAndUnrollLongDescriptor.timeOut = 20000;
//
// //
//
// function _longMakeOfLengthWithArgumentsArrayLongDescriptor( test )
// {
//   let times = 4;
//   for( let e in _.LongDescriptors )
//   {
//     let name = _.LongDescriptors[ e ].name;
//     let long = _.withLong[ name ];
//
//     test.open( `long - ${ name }` );
//     testRun( long );
//     test.close( `long - ${ name }` );
//
//     if( times < 1 )
//     break;
//     times--;
//   }
//
//   /* - */
//
//   function testRun( long )
//   {
//     test.case = 'src = null, not ins';
//     var got = long._longMakeOfLength( null );
//     var expected = long.long.default.make( 0 );
//     test.identical( got, expected );
//
//     test.case = 'src = number, not ins';
//     var got = long._longMakeOfLength( 5 );
//     var expected = long.long.default.make( 5 );
//     test.identical( got, expected );
//
//     /* aaa : should throw error : add test cases */ /* Dmytro : added in new test routines */
//     // test.case = 'src = number, ins = null';
//     // var got = long._longMakeOfLength( 5, null );
//     // var expected = long.long.default.make( 5 );
//     // test.identical( got, expected );
//     //
//     // test.case = 'src = number, ins = undefined';
//     // var got = long._longMakeOfLength( 5, undefined );
//     // var expected = long.long.default.make( 5 );
//     // test.identical( got, expected );
//
//     test.case = 'src = null, ins - number';
//     var got = long._longMakeOfLength( null, 5 );
//     var expected = long.long.default.make( 5 );
//     test.identical( got, expected );
//
//     test.case = 'src = null, ins - long';
//     var got = long._longMakeOfLength( null, _.argumentsArray.make( [ 1, 2, 3, 4, 5 ] ) );
//     var expected = long.long.default.make( 5 );
//     test.identical( got, expected );
//
//     test.case = 'src = empty long, not ins';
//     var src = _.argumentsArray.make( [] );
//     var got = long._longMakeOfLength( src );
//     var expected = long.long.default.make( [] );
//     test.identical( got, expected );
//
//     test.case = 'src = long, not ins';
//     var src = _.argumentsArray.make( [ 1, 2, 3 ] );
//     var got = long._longMakeOfLength( src );
//     var expected = long.long.default.make( 3 );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src = empty long, ins = null';
//     var src = _.argumentsArray.make( [] );
//     var got = long._longMakeOfLength( src, null );
//     var expected = long.long.default.make( 0 );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src = empty long, ins = number';
//     var src = _.argumentsArray.make( [] );
//     var got = long._longMakeOfLength( src, 2 );
//     var expected = long.long.default.make( 2 );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src = long, ins = number, ins < src.length';
//     var src = _.argumentsArray.make( [ 1, 2, 3 ] );
//     var got = long._longMakeOfLength( src, 2 );
//     var expected = long.long.default.make( 2 );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src = long with an element, ins = empty array';
//     var src = new F64x( 10 );
//     var got = long._longMakeOfLength( src, [] );
//     var expected = new F64x( 0 );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src = long, ins = number, ins > src.length';
//     var src = _.argumentsArray.make( [ 1, 2, 3 ] );
//     var got = long._longMakeOfLength( src, 4 );
//     var expected = long.long.default.make( 4 );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src = long, ins = array, ins.length > src.length';
//     var src = _.argumentsArray.make( [ 0, 1 ] );
//     var ins = [ 1, 2, 3 ];
//     var got = long._longMakeOfLength( src, ins );
//     var expected = long.long.default.make( 3 );
//     test.identical( got, expected );
//     test.true( got !== ins );
//     test.true( got !== src );
//
//     test.case = 'src = long, ins = array, ins.length === src.length'
//     var src = _.argumentsArray.make( 5 );
//     var ins = [ 1, 2, 3, 4, 5 ];
//     var got = long._longMakeOfLength( src, ins );
//     var expected = long.long.default.make( 5 );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     // test.case = 'src = Array constructor, ins = null';
//     // var got = long._longMakeOfLength( Array, null );
//     // var expected = [];
//     // test.identical( got, expected );
//     // test.true( _.arrayIs( got ) );
//
//     test.case = 'src = BufferTyped constructor, ins = number';
//     var got = long._longMakeOfLength( U32x, 5 );
//     var expected = new U32x( 5 );
//     test.identical( got, expected );
//     test.true( _.buffer.typedIs(  got ) );
//
//     test.case = 'src = Array constructor, ins = long';
//     var ins = _.argumentsArray.make( [ 1, 2, 3 ] );
//     var got = long._longMakeOfLength( Array, ins );
//     var expected = [ undefined, undefined, undefined ];
//     test.identical( got, expected );
//     test.true( _.arrayIs( got ) );
//     test.true( got !== ins );
//   }
// }
//
// //
//
// function _longMakeOfLengthWithBufferTypedLongDescriptor( test )
// {
//   var list =
//   [
//     I8x,
//     U16x,
//     F32x,
//     F64x,
//   ];
//
//   /* tests */
//
//   let times = 4;
//   for( let e in _.LongDescriptors )
//   {
//     let name = _.LongDescriptors[ e ].name;
//     let long = _.withLong[ name ];
//
//     for( let i = 0; i < list.length; i++ )
//     {
//       test.open( `long - ${ name }, makeLong - ${ list[ i ].name }` );
//       testRun( long, list[ i ] );
//       test.close( `long - ${ name }, makeLong - ${ list[ i ].name }` );
//     }
//
//     if( times < 1 )
//     break;
//     times--;
//   }
//
//   /* test subroutine */
//
//   function testRun( long, makeLong )
//   {
//     test.case = 'src = null, not ins';
//     var got = long._longMakeOfLength( null );
//     var expected = long.long.default.make( 0 );
//     test.identical( got, expected );
//
//     test.case = 'src = number, not ins';
//     var got = long._longMakeOfLength( 5 );
//     var expected = long.long.default.make( 5 );
//     test.identical( got, expected );
//
//     /* aaa : should throw error : add test cases */ /* Dmytro : added in new test routines */
//     // test.case = 'src = number, ins = null';
//     // var got = long._longMakeOfLength( 5, null );
//     // var expected = long.long.default.make( 5 );
//     // test.identical( got, expected );
//     //
//     // test.case = 'src = number, ins = undefined';
//     // var got = long._longMakeOfLength( 5, undefined );
//     // var expected = long.long.default.make( 5 );
//     // test.identical( got, expected );
//
//     test.case = 'src = null, ins - number';
//     var got = long._longMakeOfLength( null, 5 );
//     var expected = long.long.default.make( 5 );
//     test.identical( got, expected );
//
//     test.case = 'src = null, ins - long';
//     var got = long._longMakeOfLength( null, new makeLong( [ 1, 2, 3, 4, 5 ] ) );
//     var expected = long.long.default.make( 5 );
//     test.identical( got, expected );
//
//     test.case = 'src = empty long, not ins';
//     var src = new makeLong( [] );
//     var got = long._longMakeOfLength( src );
//     var expected = new makeLong( [] );
//     test.identical( got, expected );
//
//     test.case = 'src = long, not ins';
//     var src = new makeLong( [ 1, 2, 3 ] );
//     var got = long._longMakeOfLength( src );
//     var expected = new makeLong([ 1, 2, 3 ]);
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src = empty long, ins = null';
//     var src = new makeLong( [] );
//     var got = long._longMakeOfLength( src, null );
//     var expected = new makeLong( 0 );
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( src.constructor.name === got.constructor.name );
//
//     test.case = 'src = long, ins = number';
//     var src = new makeLong( 10 );
//     var got = long._longMakeOfLength( src.constructor, 4 );
//     var expected = new makeLong( 4 );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src = empty long, ins = number';
//     var src = new makeLong( [] );
//     var got = long._longMakeOfLength( src, 2 );
//     var expected = new makeLong( 2 );
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( src.constructor.name === got.constructor.name );
//
//     test.case = 'src = long, ins = number, ins < src.length';
//     var src = new makeLong( [ 1, 2, 3 ] );
//     var got = long._longMakeOfLength( src, 2 );
//     var expected = new makeLong( [ 1, 2 ] );
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( src.constructor.name === got.constructor.name );
//
//     test.case = 'src = long with an element, ins = empty array';
//     var src = new F64x( 10 );
//     var got = long._longMakeOfLength( src, [] );
//     var expected = new F64x( 0 );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src = long, ins = number, ins > src.length';
//     var src = new makeLong( [ 1, 2, 3 ] );
//     var got = long._longMakeOfLength( src, 4 );
//     var expected = new makeLong( [ 1, 2, 3, 0 ] );
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( src.constructor.name === got.constructor.name );
//
//     test.case = 'src = long, ins = array, ins.length === src.length'
//     var src = new makeLong( 5 );
//     var ins = [ 1, 2, 3, 4, 5 ];
//     var got = long._longMakeOfLength( src, ins );
//     var expected = new makeLong([ 1, 2, 3, 4, 5 ]);
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( src.constructor.name === got.constructor.name );
//
//     // test.case = 'src = Array constructor, ins = null';
//     // var got = long._longMakeOfLength( Array, null );
//     // var expected = [];
//     // test.identical( got, expected );
//     // test.true( _.arrayIs( got ) );
//
//     test.case = 'src = BufferTyped constructor, ins = number';
//     var got = long._longMakeOfLength( U32x, 5 );
//     var expected = new U32x( 5 );
//     test.identical( got, expected );
//     test.true( _.buffer.typedIs(  got ) );
//
//     test.case = 'src = Array constructor, ins = long';
//     var ins = new makeLong( [ 1, 2, 3 ] );
//     var got = long._longMakeOfLength( Array, ins );
//     var expected = [ undefined, undefined, undefined ];
//     test.identical( got, expected );
//     test.true( _.arrayIs( got ) );
//     test.true( got !== ins );
//   }
// }
//
// _longMakeOfLengthWithBufferTypedLongDescriptor.timeOut = 20000;

//

/* aaa : implement */
/* Dmytro : implemented */
/* aaa : no! */
/* Dmytro : improved test routines, split on separate types, tested with default longDescriptor and with no default */

// function longMakeUndefinedWithArrayAndUnroll( test )
// {
//   var array = ( src ) => _.array.make( src );
//   var unroll = ( src ) => _.unroll.make( src );
//   var longConstr = ( src ) =>
//   {
//     if( src )
//     return _.long.default.make( src );
//     return _.long.default.make( 0 );
//   }
//   var list =
//   [
//     array,
//     unroll,
//     longConstr,
//   ];
//
//   /* tests */
//
//   for( let t = 0; t < list.length; t++ )
//   {
//     test.open( list[ t ].name );
//     testRun( list[ t ] );
//     test.close( list[ t ].name );
//   }
//
//   /* test subroutine */
//
//   function testRun( makeLong )
//   {
//     /* aaa : thouse routines are too makeLong code to be such routine. use function nameOfRoutine(){} an move it out of run */
//     /* aaa : bad names of routines. not clear what it does */
//     /* aaa : bad names of routines. don't use nouns for routines */
//     /* Dmytro : improved test routines, split on separate types, tested with default longDescriptor and with no default, removed unnacessary function and variables */
//
//     test.case = 'without arguments';
//     var got = _.long.makeUndefined();
//     var expected = [];
//     test.identical( got, expected );
//
//     test.case = 'src = null, not ins';
//     var got = _.long.makeUndefined( null );
//     var expected = [];
//     test.identical( got, expected );
//
//     test.case = 'src = number, not ins';
//     var got = _.long.makeUndefined( 5 );
//     var expected = _.long.default.make( 5 );
//     test.identical( got, expected );
//
//     /* aaa : should throw error : add test cases */ /* Dmytro : added in new test routines */
//     // test.case = 'src = number, ins = null';
//     // var got = _.long.makeUndefined( 5, null );
//     // var expected = _.long.default.make( 5 );
//     // test.identical( got, expected );
//     //
//     // test.case = 'src = number, ins = undefined';
//     // var got = _.long.makeUndefined( 5, undefined );
//     // var expected = _.long.default.make( 5 );
//     // test.identical( got, expected );
//
//     test.case = 'src = null, ins - number';
//     var got = _.long.makeUndefined( null, 5 );
//     var expected = _.long.default.make( 5 );
//     test.identical( got, expected );
//
//     test.case = 'src = null, ins - long';
//     var got = _.long.makeUndefined( null, makeLong( [ 1, 2, 3, 4, 5 ] ) );
//     var expected = _.long.default.make( 5 );
//     test.identical( got, expected );
//
//     test.case = 'src = empty long, not ins';
//     var src = makeLong( [] );
//     var got = _.long.makeUndefined( src );
//     var expected = makeLong( [] );
//     test.identical( got, expected );
//     /* aaa : analyze and fix that */
//     /* Dmytro : this test case is the same as above. That is src defines only length, but longDescriptor defines type */
//     // test.false( src.constructor.name === got.constructor.name );
//
//     test.case = 'src = long, not ins';
//     var src = makeLong( [ 1, 2, 3 ] );
//     var got = _.long.makeUndefined( src );
//     var expected = makeLong( 3 );
//     test.identical( got, expected );
//     test.true( got !== src );
//     /* aaa : improve this line. make it proepr + redable */
//     /* Dmytro : line below not use strange subroutine */
//     test.true( src.constructor.name === got.constructor.name );
//
//     /* aaa : make sure each _.longMake, _.longFrom, _.array.make, _.array.from test routine has the same test case */
//     /* Dmytro : each test routine that accepts two arguments has this test case */
//     // test.case = 'src = empty long, ins = null';
//     // var src = makeLong( [] );
//     // var got = _.long.makeUndefined( src, null );
//     // var expected = makeLong( 0 );
//     // test.identical( got, expected );
//     // test.true( got !== src );
//     // test.true( src.constructor.name === got.constructor.name );
//
//     /* aaa : make sure each _.longMake, _.longFrom, _.array.make, _.array.from test routine has the same test case */
//     /* Dmytro : each test routine that accepts two arguments has this test case */
//     test.case = 'src = long, ins = number';
//     var src = makeLong( 10 );
//     var got = _.long.makeUndefined( src, 4 );
//     var expected = makeLong( 4 );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src = empty long, ins = number';
//     var src = makeLong( [] );
//     var got = _.long.makeUndefined( src, 2 );
//     var expected = makeLong( 2 );
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( src.constructor.name === got.constructor.name );
//
//     test.case = 'src = long, ins = number, ins < src.length';
//     var src = makeLong( [ 1, 2, 3 ] );
//     var got = _.long.makeUndefined( src, 2 );
//     var expected = makeLong( 2 );
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( src.constructor.name === got.constructor.name );
//
//     /* aaa : make sure each _.longMake, _.longFrom, _.array.make, _.array.from test routine has the same test case */
//     /* Dmytro : each test routine that accepts two arguments has this test case */
//     test.case = 'src = long with an element, ins = empty array';
//     var src = new F64x( 10 );
//     var got = _.long.makeUndefined( src, [] );
//     var expected = new F64x( 0 );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     /* aaa : make sure each _.longMake, _.longFrom, _.array.make, _.array.from test routine has the same test case */
//     /* Dmytro : each test routine that accepts two arguments has this test case */
//     test.case = 'src = long, ins = number, ins > src.length';
//     var src = makeLong( [ 1, 2, 3 ] );
//     var got = _.long.makeUndefined( src, 4 );
//     var expected = makeLong( 4 );
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( src.constructor.name === got.constructor.name );
//
//     test.case = 'src = long, ins = array, ins.length > src.length';
//     var src = makeLong( [ 0, 1 ] );
//     var ins = [ 1, 2, 3 ];
//     var got = _.long.makeUndefined( src, ins );
//     var expected = makeLong( 3 );
//     test.identical( got, expected );
//     test.true( got !== ins );
//     test.true( got !== src );
//     test.true( src.constructor.name === got.constructor.name );
//
//     test.case = 'src = long, ins = array, ins.length === src.length'
//     var src = makeLong( 5 );
//     var ins = [ 1, 2, 3, 4, 5 ];
//     var got = _.long.makeUndefined( src, ins );
//     var expected = makeLong( 5 );
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( src.constructor.name === got.constructor.name );
//
//     // test.case = 'src = Array constructor, ins = null';
//     // var got = _.long.makeUndefined( Array, null );
//     // var expected = [];
//     // test.identical( got, expected );
//     // test.true( _.arrayIs( got ) );
//
//     test.case = 'src = BufferTyped constructor, ins = number';
//     var got = _.long.makeUndefined( U32x, 5 );
//     var expected = new U32x( 5 );
//     test.identical( got, expected );
//     test.true( _.buffer.typedIs(  got ) );
//
//     test.case = 'src = Array constructor, ins = long';
//     var ins = makeLong( [ 1, 2, 3 ] );
//     var got = _.long.makeUndefined( Array, ins );
//     var expected = [ undefined, undefined, undefined ];
//     test.identical( got, expected );
//     test.true( _.arrayIs( got ) );
//     test.true( got !== ins );
//
//     test.case = 'njs buffer';
//     var src = BufferNode.alloc( 3 );
//     var got = _.long.makeUndefined( null, src );
//     var expected = [ undefined, undefined, undefined ];
//     test.identical( got, expected );
//
//     test.case = 'constructor and njs buffer';
//     var src = BufferNode.alloc( 3 );
//     var got = _.long.makeUndefined( Array, src );
//     var expected = [ undefined, undefined, undefined ];
//     test.identical( got, expected );
//
//     test.case = 'countable';
//     var src = __.diagnostic.objectMake({ elements : [ 1, 2, 3 ], countable : 1 });
//     var got = _.long.makeUndefined( null, src );
//     var expected = [ undefined, undefined, undefined ];
//     test.identical( got, expected );
//
//   }
//
//   /* - */
//
//   if( !Config.debug )
//   return;
//
//   test.case = 'extra arguments';
//   test.shouldThrowErrorSync( () => _.long.makeUndefined( [ 1, 2, 3 ], 4, 'extra argument' ) );
//
//   test.case = 'wrong type of ins';
//   test.shouldThrowErrorSync( () => _.long.makeUndefined( 'wrong argument', 1 ) );
//   test.shouldThrowErrorSync( () => _.long.makeUndefined( 1, 1 ) );
//   test.shouldThrowErrorSync( () => _.long.makeUndefined( BufferNode.alloc( 3 ), 2 ) );
//   test.shouldThrowErrorSync( () => _.long.makeUndefined( new BufferRaw( 3 ), 2 ) );
//   // test.shouldThrowErrorSync( () => _.long.makeUndefined( Array, BufferNode.from( [ 3 ] ) ) );
//
//   test.case = 'wrong type of len';
//   test.shouldThrowErrorSync( () => _.long.makeUndefined( [ 1, 2, 3 ], 'wrong type of argument' ) );
//   test.shouldThrowErrorSync( () => _.long.makeUndefined( [ 1, 2, 3 ], Infinity ) );
//
//   test.case = 'src - long, ins - null';
//   test.shouldThrowErrorSync( () => _.long.makeUndefined( [ 1, 2, 3 ], null ) );
// }
//
// //
//
// function longMakeUndefinedWithArgumentsArray( test )
// {
//   test.case = 'src = null, not ins';
//   var got = _.long.makeUndefined( null );
//   var expected = _.long.default.make( 0 );
//   test.identical( got, expected );
//
//   test.case = 'src = number, not ins';
//   var got = _.long.makeUndefined( 5 );
//   var expected = _.long.default.make( 5 );
//   test.identical( got, expected );
//
//   /* aaa : should throw error : add test cases */ /* Dmytro : added in new test routines */
//   // test.case = 'src = number, ins = null';
//   // var got = _.long.makeUndefined( 5, null );
//   // var expected = _.long.default.make( 5 );
//   // test.identical( got, expected );
//   //
//   // test.case = 'src = number, ins = undefined';
//   // var got = _.long.makeUndefined( 5, undefined );
//   // var expected = _.long.default.make( 5 );
//   // test.identical( got, expected );
//
//   test.case = 'src = null, ins - number';
//   var got = _.long.makeUndefined( null, 5 );
//   var expected = _.long.default.make( 5 );
//   test.identical( got, expected );
//
//   test.case = 'src = null, ins - long';
//   var got = _.long.makeUndefined( null, _.argumentsArray.make( [ 1, 2, 3, 4, 5 ] ) );
//   var expected = _.long.default.make( 5 );
//   test.identical( got, expected );
//
//   test.case = 'src = empty long, not ins';
//   var src = _.argumentsArray.make( [] );
//   var got = _.long.makeUndefined( src );
//   var expected = _.argumentsArray.make( [] );
//   test.identical( got, expected );
//
//   test.case = 'src = long, not ins';
//   var src = _.argumentsArray.make( [ 1, 2, 3 ] );
//   var got = _.long.makeUndefined( src );
//   var expected = _.argumentsArray.make( 3 );
//   test.identical( got, expected );
//   test.true( got !== src );
//
//   // test.case = 'src = empty long, ins = null';
//   // var src = _.argumentsArray.make( [] );
//   // var got = _.long.makeUndefined( src, null );
//   // var expected = _.long.default.make( 0 );
//   // test.identical( got, expected );
//   // test.true( got !== src );
//
//   test.case = 'src = empty long, ins = number';
//   var src = _.argumentsArray.make( [] );
//   var got = _.long.makeUndefined( src, 2 );
//   var expected = _.argumentsArray.make( 2 );
//   test.identical( got, expected );
//   test.true( got !== src );
//
//   test.case = 'src = long, ins = number, ins < src.length';
//   var src = _.argumentsArray.make( [ 1, 2, 3 ] );
//   var got = _.long.makeUndefined( src, 2 );
//   var expected = _.argumentsArray.make( 2 );
//   test.identical( got, expected );
//   test.true( got !== src );
//
//   test.case = 'src = long, ins = number, ins > src.length';
//   var src = _.argumentsArray.make( [ 1, 2, 3 ] );
//   var got = _.long.makeUndefined( src, 4 );
//   var expected = _.argumentsArray.make( 4 );
//   test.identical( got, expected );
//   test.true( got !== src );
//
//   test.case = 'src = long, ins = array, ins.length > src.length';
//   var src = _.argumentsArray.make( [ 0, 1 ] );
//   var ins = [ 1, 2, 3 ];
//   var got = _.long.makeUndefined( src, ins );
//   var expected = _.argumentsArray.make( 3 );
//   test.identical( got, expected );
//   test.true( got !== ins );
//   test.true( got !== src );
//
//   test.case = 'src = long, ins = array, ins.length === src.length'
//   var src = _.argumentsArray.make( 5 );
//   var ins = [ 1, 2, 3, 4, 5 ];
//   var got = _.long.makeUndefined( src, ins );
//   var expected = _.argumentsArray.make( 5 );
//   test.identical( got, expected );
//   test.true( got !== src );
//
//   test.case = 'src = BufferTyped constructor, ins = number';
//   var got = _.long.makeUndefined( U32x, 5 );
//   var expected = new U32x( 5 );
//   test.identical( got, expected );
//   test.true( _.buffer.typedIs(  got ) );
//
//   test.case = 'src = Array constructor, ins = long';
//   var ins = _.argumentsArray.make( [ 1, 2, 3 ] );
//   var got = _.long.makeUndefined( Array, ins );
//   var expected = [ undefined, undefined, undefined ];
//   test.identical( got, expected );
//   test.true( _.arrayIs( got ) );
//   test.true( got !== ins );
// }
//
// //
//
// function longMakeUndefinedWithBufferTyped( test )
// {
//   var list =
//   [
//     I8x,
//     U16x,
//     U16x,
//     F32x,
//   ];
//
//   /* tests */
//
//   for( let t = 0; t < list.length; t++ )
//   {
//     test.open( list[ t ].name );
//     testRun( list[ t ] );
//     test.close( list[ t ].name );
//   }
//
//   /* test subroutine */
//
//   function testRun( makeLong )
//   {
//     test.case = 'src = null, not ins';
//     var got = _.long.makeUndefined( null );
//     var expected = [];
//     test.identical( got, expected );
//
//     test.case = 'src = number, not ins';
//     var got = _.long.makeUndefined( 5 );
//     var expected = _.long.default.make( 5 );
//     test.identical( got, expected );
//
//     /* aaa : should throw error : add test cases */ /* Dmytro : added in new test routines */
//     // test.case = 'src = number, ins = null';
//     // var got = _.long.makeUndefined( 5, null );
//     // var expected = _.long.default.make( 5 );
//     // test.identical( got, expected );
//     //
//     // test.case = 'src = number, ins = undefined';
//     // var got = _.long.makeUndefined( 5, undefined );
//     // var expected = _.long.default.make( 5 );
//     // test.identical( got, expected );
//
//     test.case = 'src = null, ins - number';
//     var got = _.long.makeUndefined( null, 5 );
//     var expected = _.long.default.make( 5 );
//     test.identical( got, expected );
//
//     test.case = 'src = null, ins - long';
//     var got = _.long.makeUndefined( null, new makeLong( [ 1, 2, 3, 4, 5 ] ) );
//     var expected = _.long.default.make( 5 );
//     test.identical( got, expected );
//
//     test.case = 'src = empty long, not ins';
//     var src = new makeLong( [] );
//     var got = _.long.makeUndefined( src );
//     var expected = new makeLong( [] );
//     test.identical( got, expected );
//
//     test.case = 'src = long, not ins';
//     var src = new makeLong( [ 1, 2, 3 ] );
//     var got = _.long.makeUndefined( src );
//     var expected = new makeLong( 3 );
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( src.constructor.name === got.constructor.name );
//
//     // test.case = 'src = empty long, ins = null';
//     // var src = new makeLong( [] );
//     // var got = _.long.makeUndefined( src, null );
//     // var expected = new makeLong( 0 );
//     // test.identical( got, expected );
//     // test.true( got !== src );
//     // test.true( src.constructor.name === got.constructor.name );
//
//     test.case = 'src = long, ins = number';
//     var src = new makeLong( 10 );
//     var got = _.long.makeUndefined( src.constructor, 4 );
//     var expected = new makeLong( 4 );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src = empty long, ins = number';
//     var src = new makeLong( [] );
//     var got = _.long.makeUndefined( src, 2 );
//     var expected = new makeLong( 2 );
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( src.constructor.name === got.constructor.name );
//
//     test.case = 'src = long, ins = number, ins < src.length';
//     var src = new makeLong( [ 1, 2, 3 ] );
//     var got = _.long.makeUndefined( src, 2 );
//     var expected = new makeLong( 2 );
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( src.constructor.name === got.constructor.name );
//
//     test.case = 'src = long with an element, ins = empty array';
//     var src = new F64x( 10 );
//     var got = _.long.makeUndefined( src, [] );
//     var expected = new F64x( 0 );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src = long, ins = number, ins > src.length';
//     var src = new makeLong( [ 1, 2, 3 ] );
//     var got = _.long.makeUndefined( src, 4 );
//     var expected = new makeLong( 4 );
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( src.constructor.name === got.constructor.name );
//
//     test.case = 'src = long, ins = array, ins.length > src.length';
//     var src = new makeLong( [ 0, 1 ] );
//     var ins = [ 1, 2, 3 ];
//     var got = _.long.makeUndefined( src, ins );
//     var expected = new makeLong( 3 );
//     test.identical( got, expected );
//     test.true( got !== ins );
//     test.true( got !== src );
//     test.true( src.constructor.name === got.constructor.name );
//
//     test.case = 'src = long, ins = array, ins.length === src.length'
//     var src = new makeLong( 5 );
//     var ins = [ 1, 2, 3, 4, 5 ];
//     var got = _.long.makeUndefined( src, ins );
//     var expected = new makeLong( 5 );
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( src.constructor.name === got.constructor.name );
//
//     // test.case = 'src = Array constructor, ins = null';
//     // var got = _.long.makeUndefined( Array, null );
//     // var expected = [];
//     // test.identical( got, expected );
//     // test.true( _.arrayIs( got ) );
//
//     test.case = 'src = BufferTyped constructor, ins = number';
//     var got = _.long.makeUndefined( U32x, 5 );
//     var expected = new U32x( 5 );
//     test.identical( got, expected );
//     test.true( _.buffer.typedIs(  got ) );
//
//     test.case = 'src = Array constructor, ins = long';
//     var ins = new makeLong( [ 1, 2, 3 ] );
//     var got = _.long.makeUndefined( Array, ins );
//     var expected = [ undefined, undefined, undefined ];
//     test.identical( got, expected );
//     test.true( _.arrayIs( got ) );
//     test.true( got !== ins );
//   }
// }
//
// //
//
// function longMakeUndefinedWithArrayAndUnrollLongDescriptor( test )
// {
//   var array = ( src ) => _.array.make( src );
//   var unroll = ( src ) => _.unroll.make( src );
//   var longConstr = ( src ) =>
//   {
//     if( src )
//     return _.long.default.make( src );
//     return _.long.default.make( 0 );
//   }
//   var list =
//   [
//     array,
//     unroll,
//     longConstr,
//   ];
//
//   /* tests */
//
//   let times = 4;
//   // for( let e in _.LongDescriptors )
//   // {
//   //   let name = _.LongDescriptors[ e ].name;
//   //   let long = _.withLong[ name ];
//
//   for( let k in _.long.namespaces )
//   {
//     let namespace = _.long.namespaces[ k ];
//     let name = namespace.TypeName;
//     let long = _.withLong[ name ];
//
//     for( let i = 0; i < list.length; i++ )
//     {
//       test.open( `long - ${ name }, makeLong - ${ list[ i ].name }` );
//       testRun( long, list[ i ] );
//       test.close( `long - ${ name }, makeLong - ${ list[ i ].name }` );
//     }
//
//     if( times < 1 )
//     break;
//     times--;
//   }
//
//   /* test subroutine */
//
//   function testRun( long, makeLong )
//   {
//     test.case = 'src = null, not ins';
//     var got = long.long.makeUndefined( null );
//     var expected = long.long.default.make( 0 );
//     test.identical( got, expected );
//
//     test.case = 'src = number, not ins';
//     var got = long.long.makeUndefined( 5 );
//     var expected = long.long.default.make( 5 );
//     test.identical( got, expected );
//
//     /* aaa : should throw error : add test cases */ /* Dmytro : added in new test routines */
//     // test.case = 'src = number, ins = null';
//     // var got = long.long.makeUndefined( 5, null );
//     // var expected = long.long.default.make( 5 );
//     // test.identical( got, expected );
//     //
//     // test.case = 'src = number, ins = undefined';
//     // var got = long.long.makeUndefined( 5, undefined );
//     // var expected = long.long.default.make( 5 );
//     // test.identical( got, expected );
//
//     test.case = 'src = null, ins - number';
//     var got = long.long.makeUndefined( null, 5 );
//     var expected = long.long.default.make( 5 );
//     test.identical( got, expected );
//
//     test.case = 'src = null, ins - long';
//     var got = long.long.makeUndefined( null, makeLong( [ 1, 2, 3, 4, 5 ] ) );
//     var expected = long.long.default.make( 5 );
//     test.identical( got, expected );
//
//     test.case = 'src = empty long, not ins';
//     var src = makeLong( [] );
//     var got = long.long.makeUndefined( src );
//     var expected = makeLong( [] );
//     test.identical( got, expected );
//
//     test.case = 'src = long, not ins';
//     var src = makeLong( [ 1, 2, 3 ] );
//     var got = long.long.makeUndefined( src );
//     var expected = makeLong( 3 );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src = empty long, ins = null';
//     var src = makeLong( [] );
//     var got = long.long.makeUndefined( src, null );
//     var expected = makeLong( 0 );
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( src.constructor.name === got.constructor.name );
//
//     test.case = 'src = long, ins = number';
//     var src = makeLong( 10 );
//     var got = long.long.makeUndefined( src.constructor, 4 );
//     var expected = makeLong( 4 );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src = empty long, ins = number';
//     var src = makeLong( [] );
//     var got = long.long.makeUndefined( src, 2 );
//     var expected = makeLong( 2 );
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( src.constructor.name === got.constructor.name );
//
//     test.case = 'src = long, ins = number, ins < src.length';
//     var src = makeLong( [ 1, 2, 3 ] );
//     var got = long.long.makeUndefined( src, 2 );
//     var expected = makeLong( 2 );
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( src.constructor.name === got.constructor.name );
//
//     test.case = 'src = long with an element, ins = empty array';
//     var src = new F64x( 10 );
//     var got = long.long.makeUndefined( src, [] );
//     var expected = new F64x( 0 );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src = long, ins = number, ins > src.length';
//     var src = makeLong( [ 1, 2, 3 ] );
//     var got = long.long.makeUndefined( src, 4 );
//     var expected = makeLong( 4 );
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( src.constructor.name === got.constructor.name );
//
//     test.case = 'src = long, ins = array, ins.length > src.length';
//     var src = makeLong( [ 0, 1 ] );
//     var ins = [ 1, 2, 3 ];
//     var got = long.long.makeUndefined( src, ins );
//     var expected = makeLong( 3 );
//     test.identical( got, expected );
//     test.true( got !== ins );
//     test.true( got !== src );
//     test.true( src.constructor.name === got.constructor.name );
//
//     test.case = 'src = long, ins = array, ins.length === src.length'
//     var src = makeLong( 5 );
//     var ins = [ 1, 2, 3, 4, 5 ];
//     var got = long.long.makeUndefined( src, ins );
//     var expected = makeLong( 5 );
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( src.constructor.name === got.constructor.name );
//
//     // test.case = 'src = Array constructor, ins = null';
//     // var got = long.long.makeUndefined( Array, null );
//     // var expected = [];
//     // test.identical( got, expected );
//     // test.true( _.arrayIs( got ) );
//
//     test.case = 'src = BufferTyped constructor, ins = number';
//     var got = long.long.makeUndefined( U32x, 5 );
//     var expected = new U32x( 5 );
//     test.identical( got, expected );
//     test.true( _.buffer.typedIs(  got ) );
//
//     test.case = 'src = Array constructor, ins = long';
//     var ins = makeLong( [ 1, 2, 3 ] );
//     var got = long.long.makeUndefined( Array, ins );
//     var expected = [ undefined, undefined, undefined ];
//     test.identical( got, expected );
//     test.true( _.arrayIs( got ) );
//     test.true( got !== ins );
//
//     /* - */
//
//     if( Config.debug )
//     {
//       test.case = 'without arguments';
//       test.shouldThrowErrorSync( () => long.long.makeUndefined() );
//
//       test.case = 'extra arguments';
//       test.shouldThrowErrorSync( () => long.long.makeUndefined( [ 1, 2, 3 ], 4, 'extra argument' ) );
//
//       test.case = 'wrong type of ins';
//       test.shouldThrowErrorSync( () => long.long.makeUndefined( 'wrong argument', 1 ) );
//       test.shouldThrowErrorSync( () => long.long.makeUndefined( 1, 1 ) );
//       test.shouldThrowErrorSync( () => long.long.makeUndefined( BufferNode.alloc( 3 ), 2 ) );
//       test.shouldThrowErrorSync( () => long.long.makeUndefined( new BufferRaw( 3 ), 2 ) );
//       test.shouldThrowErrorSync( () => long.long.makeUndefined( Array, BufferNode.from( [ 3 ] ) ) );
//
//       test.case = 'wrong type of len';
//       test.shouldThrowErrorSync( () => long.long.makeUndefined( [ 1, 2, 3 ], 'wrong type of argument' ) );
//       test.shouldThrowErrorSync( () => long.long.makeUndefined( [ 1, 2, 3 ], Infinity ) );
//     }
//   }
// }
//
// longMakeUndefinedWithArrayAndUnrollLongDescriptor.timeOut = 20000;
//
// //
//
// function longMakeUndefinedWithArgumentsArrayLongDescriptor( test )
// {
//   let times = 4;
//   // for( let e in _.LongDescriptors )
//   // {
//   //   let name = _.LongDescriptors[ e ].name;
//   //   let long = _.withLong[ name ];
//
//   for( let k in _.long.namespaces )
//   {
//     let namespace = _.long.namespaces[ k ];
//     let name = namespace.TypeName;
//     let long = _.withLong[ name ];
//
//     test.open( `long - ${ name }` );
//     testRun( long );
//     test.close( `long - ${ name }` );
//
//     if( times < 1 )
//     break;
//     times--;
//   }
//
//   /* - */
//
//   function testRun( long )
//   {
//     test.case = 'src = null, not ins';
//     var got = long.long.makeUndefined( null );
//     var expected = long.long.default.make( 0 );
//     test.identical( got, expected );
//
//     test.case = 'src = number, not ins';
//     var got = long.long.makeUndefined( 5 );
//     var expected = long.long.default.make( 5 );
//     test.identical( got, expected );
//
//     /* aaa : should throw error : add test cases */ /* Dmytro : added in new test routines */
//     // test.case = 'src = number, ins = null';
//     // var got = long.long.makeUndefined( 5, null );
//     // var expected = long.long.default.make( 5 );
//     // test.identical( got, expected );
//     //
//     // test.case = 'src = number, ins = undefined';
//     // var got = long.long.makeUndefined( 5, undefined );
//     // var expected = long.long.default.make( 5 );
//     // test.identical( got, expected );
//
//     test.case = 'src = null, ins - number';
//     var got = long.long.makeUndefined( null, 5 );
//     var expected = long.long.default.make( 5 );
//     test.identical( got, expected );
//
//     test.case = 'src = null, ins - long';
//     var got = long.long.makeUndefined( null, _.argumentsArray.make( [ 1, 2, 3, 4, 5 ] ) );
//     var expected = long.long.default.make( 5 );
//     test.identical( got, expected );
//
//     test.case = 'src = empty long, not ins';
//     var src = _.argumentsArray.make( [] );
//     var got = long.long.makeUndefined( src );
//     var expected = long.long.default.make( [] );
//     test.identical( got, expected );
//
//     test.case = 'src = long, not ins';
//     var src = _.argumentsArray.make( [ 1, 2, 3 ] );
//     var got = long.long.makeUndefined( src );
//     var expected = long.long.default.make( 3 );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src = empty long, ins = null';
//     var src = _.argumentsArray.make( [] );
//     var got = long.long.makeUndefined( src, null );
//     var expected = long.long.default.make( 0 );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src = empty long, ins = number';
//     var src = _.argumentsArray.make( [] );
//     var got = long.long.makeUndefined( src, 2 );
//     var expected = long.long.default.make( 2 );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src = long, ins = number, ins < src.length';
//     var src = _.argumentsArray.make( [ 1, 2, 3 ] );
//     var got = long.long.makeUndefined( src, 2 );
//     var expected = long.long.default.make( 2 );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src = long with an element, ins = empty array';
//     var src = new F64x( 10 );
//     var got = long.long.makeUndefined( src, [] );
//     var expected = new F64x( 0 );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src = long, ins = number, ins > src.length';
//     var src = _.argumentsArray.make( [ 1, 2, 3 ] );
//     var got = long.long.makeUndefined( src, 4 );
//     var expected = long.long.default.make( 4 );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src = long, ins = array, ins.length > src.length';
//     var src = _.argumentsArray.make( [ 0, 1 ] );
//     var ins = [ 1, 2, 3 ];
//     var got = long.long.makeUndefined( src, ins );
//     var expected = long.long.default.make( 3 );
//     test.identical( got, expected );
//     test.true( got !== ins );
//     test.true( got !== src );
//
//     test.case = 'src = long, ins = array, ins.length === src.length'
//     var src = _.argumentsArray.make( 5 );
//     var ins = [ 1, 2, 3, 4, 5 ];
//     var got = long.long.makeUndefined( src, ins );
//     var expected = long.long.default.make( 5 );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     // test.case = 'src = Array constructor, ins = null';
//     // var got = long.long.makeUndefined( Array, null );
//     // var expected = [];
//     // test.identical( got, expected );
//     // test.true( _.arrayIs( got ) );
//
//     test.case = 'src = BufferTyped constructor, ins = number';
//     var got = long.long.makeUndefined( U32x, 5 );
//     var expected = new U32x( 5 );
//     test.identical( got, expected );
//     test.true( _.buffer.typedIs(  got ) );
//
//     test.case = 'src = Array constructor, ins = long';
//     var ins = _.argumentsArray.make( [ 1, 2, 3 ] );
//     var got = long.long.makeUndefined( Array, ins );
//     var expected = [ undefined, undefined, undefined ];
//     test.identical( got, expected );
//     test.true( _.arrayIs( got ) );
//     test.true( got !== ins );
//   }
// }
//
// //
//
// function longMakeUndefinedWithBufferTypedLongDescriptor( test )
// {
//   var list =
//   [
//     I8x,
//     U16x,
//     F32x,
//     F64x,
//   ];
//
//   /* tests */
//
//   let times = 4;
//   // for( let e in _.LongDescriptors )
//   // {
//   //   let name = _.LongDescriptors[ e ].name;
//   //   let long = _.withLong[ name ];
//
//   for( let k in _.long.namespaces )
//   {
//     let namespace = _.long.namespaces[ k ];
//     let name = namespace.TypeName;
//     let long = _.withLong[ name ];
//
//     for( let i = 0; i < list.length; i++ )
//     {
//       test.open( `long - ${ name }, makeLong - ${ list[ i ].name }` );
//       testRun( long, list[ i ] );
//       test.close( `long - ${ name }, makeLong - ${ list[ i ].name }` );
//     }
//
//     if( times < 1 )
//     break;
//     times--;
//   }
//
//   /* test subroutine */
//
//   function testRun( long, makeLong )
//   {
//     test.case = 'src = null, not ins';
//     var got = long.long.makeUndefined( null );
//     var expected = long.long.default.make( 0 );
//     test.identical( got, expected );
//
//     test.case = 'src = number, not ins';
//     var got = long.long.makeUndefined( 5 );
//     var expected = long.long.default.make( 5 );
//     test.identical( got, expected );
//
//     /* aaa : should throw error : add test cases */ /* Dmytro : added in new test routines */
//     // test.case = 'src = number, ins = null';
//     // var got = long.long.makeUndefined( 5, null );
//     // var expected = long.long.default.make( 5 );
//     // test.identical( got, expected );
//     //
//     // test.case = 'src = number, ins = undefined';
//     // var got = long.long.makeUndefined( 5, undefined );
//     // var expected = long.long.default.make( 5 );
//     // test.identical( got, expected );
//
//     test.case = 'src = null, ins - number';
//     var got = long.long.makeUndefined( null, 5 );
//     var expected = long.long.default.make( 5 );
//     test.identical( got, expected );
//
//     test.case = 'src = null, ins - long';
//     var got = long.long.makeUndefined( null, new makeLong( [ 1, 2, 3, 4, 5 ] ) );
//     var expected = long.long.default.make( 5 );
//     test.identical( got, expected );
//
//     test.case = 'src = empty long, not ins';
//     var src = new makeLong( [] );
//     var got = long.long.makeUndefined( src );
//     var expected = new makeLong( [] );
//     test.identical( got, expected );
//
//     test.case = 'src = long, not ins';
//     var src = new makeLong( [ 1, 2, 3 ] );
//     var got = long.long.makeUndefined( src );
//     var expected = new makeLong( 3 );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src = empty long, ins = null';
//     var src = new makeLong( [] );
//     var got = long.long.makeUndefined( src, null );
//     var expected = new makeLong( 0 );
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( src.constructor.name === got.constructor.name );
//
//     test.case = 'src = long, ins = number';
//     var src = new makeLong( 10 );
//     var got = long.long.makeUndefined( src.constructor, 4 );
//     var expected = new makeLong( 4 );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src = empty long, ins = number';
//     var src = new makeLong( [] );
//     var got = long.long.makeUndefined( src, 2 );
//     var expected = new makeLong( 2 );
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( src.constructor.name === got.constructor.name );
//
//     test.case = 'src = long, ins = number, ins < src.length';
//     var src = new makeLong( [ 1, 2, 3 ] );
//     var got = long.long.makeUndefined( src, 2 );
//     var expected = new makeLong( 2 );
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( src.constructor.name === got.constructor.name );
//
//     test.case = 'src = long with an element, ins = empty array';
//     var src = new F64x( 10 );
//     var got = long.long.makeUndefined( src, [] );
//     var expected = new F64x( 0 );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src = long, ins = number, ins > src.length';
//     var src = new makeLong( [ 1, 2, 3 ] );
//     var got = long.long.makeUndefined( src, 4 );
//     var expected = new makeLong( 4 );
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( src.constructor.name === got.constructor.name );
//
//     test.case = 'src = long, ins = array, ins.length > src.length';
//     var src = new makeLong( [ 0, 1 ] );
//     var ins = [ 1, 2, 3 ];
//     var got = long.long.makeUndefined( src, ins );
//     var expected = new makeLong( 3 );
//     test.identical( got, expected );
//     test.true( got !== ins );
//     test.true( got !== src );
//     test.true( src.constructor.name === got.constructor.name );
//
//     test.case = 'src = long, ins = array, ins.length === src.length'
//     var src = new makeLong( 5 );
//     var ins = [ 1, 2, 3, 4, 5 ];
//     var got = long.long.makeUndefined( src, ins );
//     var expected = new makeLong( 5 );
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( src.constructor.name === got.constructor.name );
//
//     // test.case = 'src = Array constructor, ins = null';
//     // var got = long.long.makeUndefined( Array, null );
//     // var expected = [];
//     // test.identical( got, expected );
//     // test.true( _.arrayIs( got ) );
//
//     test.case = 'src = BufferTyped constructor, ins = number';
//     var got = long.long.makeUndefined( U32x, 5 );
//     var expected = new U32x( 5 );
//     test.identical( got, expected );
//     test.true( _.buffer.typedIs(  got ) );
//
//     test.case = 'src = Array constructor, ins = long';
//     var ins = new makeLong( [ 1, 2, 3 ] );
//     var got = long.long.makeUndefined( Array, ins );
//     var expected = [ undefined, undefined, undefined ];
//     test.identical( got, expected );
//     test.true( _.arrayIs( got ) );
//     test.true( got !== ins );
//   }
// }
//
// longMakeUndefinedWithBufferTypedLongDescriptor.timeOut = 20000;


//

// function longMakeZeroedBasic( test )
// {
//
//   test.case = 'no args';
//   var got = _.long.makeZeroed();
//   var expected = [];
//   test.identical( got, expected );
//
//   test.case = 'null';
//   var got = _.long.makeZeroed();
//   var expected = [];
//   test.identical( got, expected );
//
//   test.case = 'njs buffer';
//   var src = BufferNode.alloc( 3 );
//   var got = _.long.makeZeroed( null, src );
//   var expected = [ 0, 0, 0 ];
//   test.identical( got, expected );
//
//   test.case = 'constructor and njs buffer';
//   var src = BufferNode.alloc( 3 );
//   var got = _.long.makeZeroed( Array, src );
//   var expected = [ 0, 0, 0 ];
//   test.identical( got, expected );
//
//   test.case = 'countable';
//   var src = __.diagnostic.objectMake({ elements : [ 1, 2, 3 ], countable : 1 });
//   var got = _.long.makeZeroed( null, src );
//   var expected = [ 0, 0, 0 ];
//   test.identical( got, expected );
//
//   /* - */
//
//   if( !Config.debug )
//   return;
//
//   // test.case = 'without arguments';
//   // test.shouldThrowErrorSync( () => _.long.makeZeroed() );
//
//   test.case = 'extra arguments';
//   test.shouldThrowErrorSync( () => _.long.makeZeroed( [ 1, 2, 3 ], 4, 'extra argument' ) );
//
//   test.case = 'wrong type of ins';
//   test.shouldThrowErrorSync( () => _.long.makeZeroed( undefiend ) );
//   test.shouldThrowErrorSync( () => _.long.makeZeroed( 'wrong argument', 1 ) );
//   test.shouldThrowErrorSync( () => _.long.makeZeroed( 1, 1 ) );
//   test.shouldThrowErrorSync( () => _.long.makeZeroed( BufferNode.alloc( 3 ), 2 ) );
//   // test.shouldThrowErrorSync( () => _.long.makeZeroed( new BufferRaw( 3 ), 2 ) );
//   // test.shouldThrowErrorSync( () => _.long.makeZeroed( Array, BufferNode.from( [ 3 ] ) ) );
//
//   test.case = 'wrong type of len';
//   test.shouldThrowErrorSync( () => _.long.makeZeroed( [ 1, 2, 3 ], 'wrong type of argument' ) );
//   test.shouldThrowErrorSync( () => _.long.makeZeroed( [ 1, 2, 3 ], Infinity ) );
//   test.shouldThrowErrorSync( () => _.long.makeZeroed( [ 1, 2, 3 ], null ) );
//   test.shouldThrowErrorSync( () => _.long.makeZeroed( null, null ) );
//
// }
//
// //
//
// function longMakeZeroedWithArrayAndUnroll( test )
// {
//   var array = ( src ) => _.array.make( src );
//   var unroll = ( src ) => _.unroll.make( src );
//   var longConstr = ( src ) =>
//   {
//     if( src )
//     return _.long.default.make( src );
//     return _.long.default.make( 0 );
//   }
//   var list =
//   [
//     array,
//     unroll,
//     longConstr,
//   ];
//
//   /* tests */
//
//   for( let t = 0; t < list.length; t++ )
//   {
//     test.open( list[ t ].name );
//     testRun( list[ t ] );
//     test.close( list[ t ].name );
//   }
//
//   /* test subroutine */
//
//   function testRun( makeLong )
//   {
//
//     test.case = 'no args';
//     var got = _.long.makeZeroed();
//     var expected = _.long.default.make( 0 );
//     test.identical( got, expected );
//
//     test.case = 'njs buffer';
//     var src = BufferNode.alloc( 3 );
//     var got = _.long.makeZeroed( null, src );
//     var expected = [ 0, 0, 0 ];
//     test.identical( got, expected );
//
//     test.case = 'constructor and njs buffer';
//     var src = BufferNode.alloc( 3 );
//     var got = _.long.makeZeroed( Array, src );
//     var expected = [ 0, 0, 0 ];
//     test.identical( got, expected );
//
//     /* aaa2 : for Dmytro : add more such test cases */ /* Dmytro : added */
//     /* aaa2 : for Dmytro : add such cases to ALL similar test routines */ /* Dmytro : added */
//     test.case = 'countable';
//     var src = __.diagnostic.objectMake({ elements : [ 1, 2, 3 ], countable : 1 });
//     var got = _.long.makeZeroed( null, src );
//     var expected = [ 0, 0, 0 ];
//     test.identical( got, expected );
//
//     test.case = 'src = null, not ins';
//     var got = _.long.makeZeroed( null );
//     var expected = _.long.default.make( 0 );
//     test.identical( got, expected );
//
//     test.case = 'src = number, not ins';
//     var got = _.long.makeZeroed( 5 );
//     var expected = _.long.default.make( [ 0, 0, 0, 0, 0 ] );
//     test.identical( got, expected );
//
//     /* aaa : should throw error : add test cases */ /* Dmytro : added in new test routines */
//     // test.case = 'src = number, ins = null';
//     // var got = _.long.makeZeroed( 5, null );
//     // var expected = _.long.default.make( [ 0, 0, 0, 0, 0 ] );
//     // test.identical( got, expected );
//     //
//     // test.case = 'src = number, ins = undefined';
//     // var got = _.long.makeZeroed( 5, undefined );
//     // var expected = _.long.default.make( [ 0, 0, 0, 0, 0 ] );
//     // test.identical( got, expected );
//
//     test.case = 'src = null, ins - number';
//     var got = _.long.makeZeroed( null, 5 );
//     var expected = _.long.default.make( [ 0, 0, 0, 0, 0 ] );
//     test.identical( got, expected );
//
//     test.case = 'src = null, ins - long';
//     var got = _.long.makeZeroed( null, makeLong( [ 1, 2, 3, 4, 5 ] ) );
//     var expected = _.long.default.make( [ 0, 0, 0, 0, 0 ] );
//     test.identical( got, expected );
//
//     test.case = 'src = empty long, not ins';
//     var src = makeLong( [] );
//     var got = _.long.makeZeroed( src );
//     var expected = makeLong( [] );
//     test.identical( got, expected );
//
//     test.case = 'src = long, not ins';
//     var src = makeLong( [ 1, 2, 3 ] );
//     var got = _.long.makeZeroed( src );
//     var expected = makeLong( [ 0, 0, 0 ] );
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( src.constructor.name === got.constructor.name );
//
//     // test.case = 'src = empty long, ins = null';
//     // var src = makeLong( [] );
//     // var got = _.long.makeZeroed( src, null );
//     // var expected = makeLong( 0 );
//     // test.identical( got, expected );
//     // test.true( got !== src );
//     // test.true( src.constructor.name === got.constructor.name );
//
//     test.case = 'src = long, ins = number';
//     var src = makeLong( 10 );
//     var got = _.long.makeZeroed( src.constructor, 4 );
//     var expected = makeLong( [ 0, 0, 0, 0 ] );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src = empty long, ins = number';
//     var src = makeLong( [] );
//     var got = _.long.makeZeroed( src, 2 );
//     var expected = makeLong( [ 0, 0 ] );
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( src.constructor.name === got.constructor.name );
//
//     test.case = 'src = long, ins = number, ins < src.length';
//     var src = makeLong( [ 1, 2, 3 ] );
//     var got = _.long.makeZeroed( src, 2 );
//     var expected = makeLong( [ 0, 0 ] );
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( src.constructor.name === got.constructor.name );
//
//     test.case = 'src = long with an element, ins = empty array';
//     var src = new F64x( 10 );
//     var got = _.long.makeZeroed( src, [] );
//     var expected = new F64x( 0 );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src = long, ins = number, ins > src.length';
//     var src = makeLong( [ 1, 2, 3 ] );
//     var got = _.long.makeZeroed( src, 4 );
//     var expected = makeLong( [ 0, 0, 0, 0 ] );
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( src.constructor.name === got.constructor.name );
//
//     test.case = 'src = long, ins = array, ins.length > src.length';
//     var src = makeLong( [ 0, 1 ] );
//     var ins = [ 1, 2, 3 ];
//     var got = _.long.makeZeroed( src, ins );
//     var expected = makeLong( [ 0, 0, 0 ] );
//     test.identical( got, expected );
//     test.true( got !== ins );
//     test.true( got !== src );
//     test.true( src.constructor.name === got.constructor.name );
//
//     test.case = 'src = long, ins = array, ins.length === src.length'
//     var src = makeLong( 5 );
//     var ins = [ 1, 2, 3, 4, 5 ];
//     var got = _.long.makeZeroed( src, ins );
//     var expected = makeLong( [ 0, 0, 0, 0, 0 ] );
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( src.constructor.name === got.constructor.name );
//
//     // test.case = 'src = Array constructor, ins = null';
//     // var got = _.long.makeZeroed( Array, null );
//     // var expected = [];
//     // test.identical( got, expected );
//     // test.true( _.arrayIs( got ) );
//
//     test.case = 'src = BufferTyped constructor, ins = number';
//     var got = _.long.makeZeroed( U32x, 5 );
//     var expected = new U32x( [ 0, 0, 0, 0, 0 ] );
//     test.identical( got, expected );
//     test.true( _.buffer.typedIs(  got ) );
//
//     test.case = 'src = Array constructor, ins = long';
//     var ins = makeLong( [ 1, 2, 3 ] );
//     var got = _.long.makeZeroed( Array, ins );
//     var expected = [ 0, 0, 0 ];
//     test.identical( got, expected );
//     test.true( _.arrayIs( got ) );
//     test.true( got !== ins );
//
//   }
//
// }
//
// //
//
// function longMakeZeroedWithArgumentsArray( test )
// {
//
//   test.case = 'no args';
//   var got = _.long.makeZeroed();
//   var expected = _.long.default.make( 0 );
//   test.identical( got, expected );
//
//   test.case = 'src = null, not ins';
//   var got = _.long.makeZeroed( null );
//   var expected = _.long.default.make( 0 );
//   test.identical( got, expected );
//
//   test.case = 'src = number, not ins';
//   var got = _.long.makeZeroed( 5 );
//   var expected = _.long.default.make( [ 0, 0, 0, 0, 0 ] );
//   test.identical( got, expected );
//
//   /* aaa : should throw error : add test cases */ /* Dmytro : added in new test routines */
//   // test.case = 'src = number, ins = null';
//   // var got = _.long.makeZeroed( 5, null );
//   // var expected = _.long.default.make( [ 0, 0, 0, 0, 0 ] );
//   // test.identical( got, expected );
//   //
//   // test.case = 'src = number, ins = undefined';
//   // var got = _.long.makeZeroed( 5, undefined );
//   // var expected = _.long.default.make( [ 0, 0, 0, 0, 0 ] );
//   // test.identical( got, expected );
//
//   test.case = 'src = null, ins - number';
//   var got = _.long.makeZeroed( null, 5 );
//   var expected = _.long.default.make( [ 0, 0, 0, 0, 0 ] );
//   test.identical( got, expected );
//
//   test.case = 'src = null, ins - long';
//   var got = _.long.makeZeroed( null, _.argumentsArray.make( [ 1, 2, 3, 4, 5 ] ) );
//   var expected = _.long.default.make( [ 0, 0, 0, 0, 0 ] );
//   test.identical( got, expected );
//
//   test.case = 'src = empty long, not ins';
//   var src = _.argumentsArray.make( [] );
//   var got = _.long.makeZeroed( src );
//   var expected = _.argumentsArray.make( [] );
//   test.identical( got, expected );
//
//   test.case = 'src = long, not ins';
//   var src = _.argumentsArray.make( [ 1, 2, 3 ] );
//   var got = _.long.makeZeroed( src );
//   var expected = _.argumentsArray.make( [ 0, 0, 0 ] );
//   test.identical( got, expected );
//   test.true( got !== src );
//
//   // test.case = 'src = empty long, ins = null';
//   // var src = _.argumentsArray.make( [] );
//   // var got = _.long.makeZeroed( src, null );
//   // var expected = _.long.default.make( 0 );
//   // test.identical( got, expected );
//   // test.true( got !== src );
//
//   test.case = 'src = empty long, ins = number';
//   var src = _.argumentsArray.make( [] );
//   var got = _.long.makeZeroed( src, 2 );
//   var expected = _.argumentsArray.make( [ 0, 0 ] );
//   test.identical( got, expected );
//   test.true( got !== src );
//
//   test.case = 'src = long, ins = number, ins < src.length';
//   var src = _.argumentsArray.make( [ 1, 2, 3 ] );
//   var got = _.long.makeZeroed( src, 2 );
//   var expected = _.argumentsArray.make( [ 0, 0 ] );
//   test.identical( got, expected );
//   test.true( got !== src );
//
//   test.case = 'src = long with an element, ins = empty array';
//   var src = new F64x( 10 );
//   var got = _.long.makeZeroed( src, [] );
//   var expected = new F64x( 0 );
//   test.identical( got, expected );
//   test.true( got !== src );
//
//   test.case = 'src = long, ins = number, ins > src.length';
//   var src = _.argumentsArray.make( [ 1, 2, 3 ] );
//   var got = _.long.makeZeroed( src, 4 );
//   var expected = _.argumentsArray.make( [ 0, 0, 0, 0 ] );
//   test.identical( got, expected );
//   test.true( got !== src );
//
//   test.case = 'src = long, ins = array, ins.length > src.length';
//   var src = _.argumentsArray.make( [ 0, 1 ] );
//   var ins = [ 1, 2, 3 ];
//   var got = _.long.makeZeroed( src, ins );
//   var expected = _.argumentsArray.make( [ 0, 0, 0 ] );
//   test.identical( got, expected );
//   test.true( got !== ins );
//   test.true( got !== src );
//
//   test.case = 'src = long, ins = array, ins.length === src.length'
//   var src = _.argumentsArray.make( 5 );
//   var ins = [ 1, 2, 3, 4, 5 ];
//   var got = _.long.makeZeroed( src, ins );
//   var expected = _.argumentsArray.make( [ 0, 0, 0, 0, 0 ] );
//   test.identical( got, expected );
//   test.true( got !== src );
//
//   // test.case = 'src = Array constructor, ins = null';
//   // var got = _.long.makeZeroed( Array, null );
//   // var expected = [];
//   // test.identical( got, expected );
//   // test.true( _.arrayIs( got ) );
//
//   test.case = 'src = BufferTyped constructor, ins = number';
//   var got = _.long.makeZeroed( U32x, 5 );
//   var expected = new U32x( [ 0, 0, 0, 0, 0 ] );
//   test.identical( got, expected );
//   test.true( _.buffer.typedIs(  got ) );
//
//   test.case = 'src = Array constructor, ins = long';
//   var ins = _.argumentsArray.make( [ 1, 2, 3 ] );
//   var got = _.long.makeZeroed( Array, ins );
//   var expected = [ 0, 0, 0 ];
//   test.identical( got, expected );
//   test.true( _.arrayIs( got ) );
//   test.true( got !== ins );
//
//   if( Config.debug )
//   {
//
//     test.case = 'extra arguments';
//     test.shouldThrowErrorSync( () => _.long.makeZeroed( [ 1, 2, 3 ], 4, 'extra argument' ) );
//
//     test.case = 'wrong type of ins';
//     test.shouldThrowErrorSync( () => _.long.makeZeroed( 'wrong argument', 1 ) );
//     test.shouldThrowErrorSync( () => _.long.makeZeroed( 1, 1 ) );
//     test.shouldThrowErrorSync( () => _.long.makeZeroed( BufferNode.alloc( 3 ), 2 ) );
//     test.shouldThrowErrorSync( () => _.long.makeZeroed( new BufferRaw( 3 ), 2 ) );
//     // test.shouldThrowErrorSync( () => _.long.makeZeroed( Array, BufferNode.from( [ 3 ] ) ) );
//
//     test.case = 'wrong type of len';
//     test.shouldThrowErrorSync( () => _.long.makeZeroed( [ 1, 2, 3 ], 'wrong type of argument' ) );
//     test.shouldThrowErrorSync( () => _.long.makeZeroed( [ 1, 2, 3 ], Infinity ) );
//     test.shouldThrowErrorSync( () => _.long.makeZeroed( [ 1, 2, 3 ], null ) );
//     test.shouldThrowErrorSync( () => _.long.makeZeroed( null, null ) );
//     test.shouldThrowErrorSync( () => _.long.makeZeroed( [ 1, 2, 3 ], undefined ) );
//
//   }
//
// }
//
// //
//
// function longMakeZeroedWithBufferTyped( test )
// {
//   var list =
//   [
//     I8x,
//     U16x,
//     U16x,
//     F32x,
//   ];
//
//   /* tests */
//
//   for( let t = 0; t < list.length; t++ )
//   {
//     test.open( list[ t ].name );
//     testRun( list[ t ] );
//     test.close( list[ t ].name );
//   }
//
//   /* test subroutine */
//
//   function testRun( makeLong )
//   {
//     test.case = 'src = null, not ins';
//     var got = _.long.makeZeroed( null );
//     var expected = _.long.default.make( 0 );
//     test.identical( got, expected );
//
//     test.case = 'src = number, not ins';
//     var got = _.long.makeZeroed( 5 );
//     var expected = _.long.default.make( [ 0, 0, 0, 0, 0 ] );
//     test.identical( got, expected );
//
//     /* aaa : should throw error : add test cases */ /* Dmytro : added in new test routines */
//     // test.case = 'src = number, ins = null';
//     // var got = _.long.makeZeroed( 5, null );
//     // var expected = _.long.default.make( [ 0, 0, 0, 0, 0 ] );
//     // test.identical( got, expected );
//     //
//     // test.case = 'src = number, ins = undefined';
//     // var got = _.long.makeZeroed( 5, undefined );
//     // var expected = _.long.default.make( [ 0, 0, 0, 0, 0 ] );
//     // test.identical( got, expected );
//
//     test.case = 'src = null, ins - number';
//     var got = _.long.makeZeroed( null, 5 );
//     var expected = _.long.default.make( [ 0, 0, 0, 0, 0 ] );
//     test.identical( got, expected );
//
//     test.case = 'src = null, ins - long';
//     var got = _.long.makeZeroed( null, new makeLong( [ 1, 2, 3, 4, 5 ] ) );
//     var expected = _.long.default.make( [ 0, 0, 0, 0, 0 ] );
//     test.identical( got, expected );
//
//     test.case = 'src = empty long, not ins';
//     var src = new makeLong( [] );
//     var got = _.long.makeZeroed( src );
//     var expected = new makeLong( [] );
//     test.identical( got, expected );
//
//     test.case = 'src = long, not ins';
//     var src = new makeLong( [ 1, 2, 3 ] );
//     var got = _.long.makeZeroed( src );
//     var expected = new makeLong( [ 0, 0, 0 ] );
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( src.constructor.name === got.constructor.name );
//
//     test.case = 'src = empty long, ins = null';
//     var src = new makeLong( [] );
//     var got = _.long.makeZeroed( src, null );
//     var expected = new makeLong( 0 );
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( src.constructor.name === got.constructor.name );
//
//     test.case = 'src = long, ins = number';
//     var src = new makeLong( 10 );
//     var got = _.long.makeZeroed( src.constructor, 4 );
//     var expected = new makeLong( [ 0, 0, 0, 0 ] );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src = empty long, ins = number';
//     var src = new makeLong( [] );
//     var got = _.long.makeZeroed( src, 2 );
//     var expected = new makeLong( [ 0, 0 ] );
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( src.constructor.name === got.constructor.name );
//
//     test.case = 'src = long, ins = number, ins < src.length';
//     var src = new makeLong( [ 1, 2, 3 ] );
//     var got = _.long.makeZeroed( src, 2 );
//     var expected = new makeLong( [ 0, 0 ] );
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( src.constructor.name === got.constructor.name );
//
//     test.case = 'src = long with an element, ins = empty array';
//     var src = new F64x( 10 );
//     var got = _.long.makeZeroed( src, [] );
//     var expected = new F64x( 0 );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src = long, ins = number, ins > src.length';
//     var src = new makeLong( [ 1, 2, 3 ] );
//     var got = _.long.makeZeroed( src, 4 );
//     var expected = new makeLong( [ 0, 0, 0, 0 ] );
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( src.constructor.name === got.constructor.name );
//
//     test.case = 'src = long, ins = array, ins.length > src.length';
//     var src = new makeLong( [ 0, 1 ] );
//     var ins = [ 1, 2, 3 ];
//     var got = _.long.makeZeroed( src, ins );
//     var expected = new makeLong( [ 0, 0, 0 ] );
//     test.identical( got, expected );
//     test.true( got !== ins );
//     test.true( got !== src );
//     test.true( src.constructor.name === got.constructor.name );
//
//     test.case = 'src = long, ins = array, ins.length === src.length'
//     var src = new makeLong( 5 );
//     var ins = [ 1, 2, 3, 4, 5 ];
//     var got = _.long.makeZeroed( src, ins );
//     var expected = new makeLong( [ 0, 0, 0, 0, 0 ] );
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( src.constructor.name === got.constructor.name );
//
//     // test.case = 'src = Array constructor, ins = null';
//     // var got = _.long.makeZeroed( Array, null );
//     // var expected = [];
//     // test.identical( got, expected );
//     // test.true( _.arrayIs( got ) );
//
//     test.case = 'src = BufferTyped constructor, ins = number';
//     var got = _.long.makeZeroed( U32x, 5 );
//     var expected = new U32x( [ 0, 0, 0, 0, 0 ] );
//     test.identical( got, expected );
//     test.true( _.buffer.typedIs(  got ) );
//
//     test.case = 'src = Array constructor, ins = long';
//     var ins = new makeLong( [ 1, 2, 3 ] );
//     var got = _.long.makeZeroed( Array, ins );
//     var expected = [ 0, 0, 0 ];
//     test.identical( got, expected );
//     test.true( _.arrayIs( got ) );
//     test.true( got !== ins );
//   }
// }
//
// //
//
// function longMakeZeroedWithArrayAndUnrollLongDescriptor( test )
// {
//   var array = ( src ) => _.array.make( src );
//   var unroll = ( src ) => _.unroll.make( src );
//   var longConstr = ( src ) =>
//   {
//     if( src )
//     return _.long.default.make( src );
//     return _.long.default.make( 0 );
//   }
//   var list =
//   [
//     array,
//     unroll,
//     longConstr,
//   ];
//
//   /* tests */
//
//   let times = 4;
//   for( let e in _.LongDescriptors )
//   {
//     let name = _.LongDescriptors[ e ].name;
//     let long = _.withLong[ name ];
//
//     for( let i = 0; i < list.length; i++ )
//     {
//       test.open( `long - ${ name }, makeLong - ${ list[ i ].name }` );
//       testRun( long, list[ i ] );
//       test.close( `long - ${ name }, makeLong - ${ list[ i ].name }` );
//     }
//
//     if( times < 1 )
//     break;
//     times--;
//   }
//
//   /* test subroutine */
//
//   function testRun( long, makeLong )
//   {
//     test.case = 'src = null, not ins';
//     var got = long.long.makeZeroed( null );
//     var expected = long.long.default.make( 0 );
//     test.identical( got, expected );
//
//     test.case = 'src = number, not ins';
//     var got = long.long.makeZeroed( 5 );
//     var expected = long.long.default.make( [ 0, 0, 0, 0, 0 ] );
//     test.identical( got, expected );
//
//     /* aaa : should throw error : add test cases */ /* Dmytro : added in new test routines */
//     // test.case = 'src = number, ins = null';
//     // var got = long.long.makeZeroed( 5, null );
//     // var expected = long.long.default.make( [ 0, 0, 0, 0, 0 ] );
//     // test.identical( got, expected );
//     //
//     // test.case = 'src = number, ins = undefined';
//     // var got = long.long.makeZeroed( 5, undefined );
//     // var expected = long.long.default.make( [ 0, 0, 0, 0, 0 ] );
//     // test.identical( got, expected );
//
//     test.case = 'src = null, ins - number';
//     var got = long.long.makeZeroed( null, 5 );
//     var expected = long.long.default.make( [ 0, 0, 0, 0, 0 ] );
//     test.identical( got, expected );
//
//     test.case = 'src = null, ins - long';
//     var got = long.long.makeZeroed( null, makeLong( [ 1, 2, 3, 4, 5 ] ) );
//     var expected = long.long.default.make( [ 0, 0, 0, 0, 0 ] );
//     test.identical( got, expected );
//
//     test.case = 'src = empty long, not ins';
//     var src = makeLong( [] );
//     var got = long.long.makeZeroed( src );
//     var expected = makeLong( [] );
//     test.identical( got, expected );
//
//     test.case = 'src = long, not ins';
//     var src = makeLong( [ 1, 2, 3 ] );
//     var got = long.long.makeZeroed( src );
//     var expected = makeLong( [ 0, 0, 0 ] );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src = empty long, ins = null';
//     var src = makeLong( [] );
//     var got = long.long.makeZeroed( src, null );
//     var expected = makeLong( 0 );
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( src.constructor.name === got.constructor.name );
//
//     test.case = 'src = long, ins = number';
//     var src = makeLong( 10 );
//     var got = long.long.makeZeroed( src.constructor, 4 );
//     var expected = makeLong( [ 0, 0, 0, 0 ] );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src = empty long, ins = number';
//     var src = makeLong( [] );
//     var got = long.long.makeZeroed( src, 2 );
//     var expected = makeLong( [ 0, 0 ] );
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( src.constructor.name === got.constructor.name );
//
//     test.case = 'src = long, ins = number, ins < src.length';
//     var src = makeLong( [ 1, 2, 3 ] );
//     var got = long.long.makeZeroed( src, 2 );
//     var expected = makeLong( [ 0, 0 ] );
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( src.constructor.name === got.constructor.name );
//
//     test.case = 'src = long with an element, ins = empty array';
//     var src = new F64x( 10 );
//     var got = long.long.makeZeroed( src, [] );
//     var expected = new F64x( 0 );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src = long, ins = number, ins > src.length';
//     var src = makeLong( [ 1, 2, 3 ] );
//     var got = long.long.makeZeroed( src, 4 );
//     var expected = makeLong( [ 0, 0, 0, 0 ] );
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( src.constructor.name === got.constructor.name );
//
//     test.case = 'src = long, ins = array, ins.length > src.length';
//     var src = makeLong( [ 0, 1 ] );
//     var ins = [ 1, 2, 3 ];
//     var got = long.long.makeZeroed( src, ins );
//     var expected = makeLong( [ 0, 0, 0 ] );
//     test.identical( got, expected );
//     test.true( got !== ins );
//     test.true( got !== src );
//     test.true( src.constructor.name === got.constructor.name );
//
//     test.case = 'src = long, ins = array, ins.length === src.length'
//     var src = makeLong( 5 );
//     var ins = [ 1, 2, 3, 4, 5 ];
//     var got = long.long.makeZeroed( src, ins );
//     var expected = makeLong( [ 0, 0, 0, 0, 0 ] );
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( src.constructor.name === got.constructor.name );
//
//     // test.case = 'src = Array constructor, ins = null';
//     // var got = long.long.makeZeroed( Array, null );
//     // var expected = [];
//     // test.identical( got, expected );
//     // test.true( _.arrayIs( got ) );
//
//     test.case = 'src = BufferTyped constructor, ins = number';
//     var got = long.long.makeZeroed( U32x, 5 );
//     var expected = new U32x( [ 0, 0, 0, 0, 0 ] );
//     test.identical( got, expected );
//     test.true( _.buffer.typedIs(  got ) );
//
//     test.case = 'src = Array constructor, ins = long';
//     var ins = makeLong( [ 1, 2, 3 ] );
//     var got = long.long.makeZeroed( Array, ins );
//     var expected = [ 0, 0, 0 ];
//     test.identical( got, expected );
//     test.true( _.arrayIs( got ) );
//     test.true( got !== ins );
//
//     /* - */
//
//     if( Config.debug )
//     {
//       test.case = 'without arguments';
//       test.shouldThrowErrorSync( () => long.long.makeZeroed() );
//
//       test.case = 'extra arguments';
//       test.shouldThrowErrorSync( () => long.long.makeZeroed( [ 1, 2, 3 ], 4, 'extra argument' ) );
//
//       test.case = 'wrong type of ins';
//       test.shouldThrowErrorSync( () => long.long.makeZeroed( 'wrong argument', 1 ) );
//       test.shouldThrowErrorSync( () => long.long.makeZeroed( 1, 1 ) );
//       test.shouldThrowErrorSync( () => long.long.makeZeroed( BufferNode.alloc( 3 ), 2 ) );
//       test.shouldThrowErrorSync( () => long.long.makeZeroed( new BufferRaw( 3 ), 2 ) );
//       test.shouldThrowErrorSync( () => long.long.makeZeroed( Array, BufferNode.from( [ 3 ] ) ) );
//
//       test.case = 'wrong type of len';
//       test.shouldThrowErrorSync( () => long.long.makeZeroed( [ 1, 2, 3 ], 'wrong type of argument' ) );
//       test.shouldThrowErrorSync( () => long.long.makeZeroed( [ 1, 2, 3 ], Infinity ) );
//     }
//   }
// }
//
// longMakeZeroedWithArrayAndUnrollLongDescriptor.timeOut = 20000;
//
// //
//
// function longMakeZeroedWithArgumentsArrayLongDescriptor( test )
// {
//   let times = 4;
//   for( let e in _.LongDescriptors )
//   {
//     let name = _.LongDescriptors[ e ].name;
//     let long = _.withLong[ name ];
//
//     test.open( `long - ${ name }` );
//     testRun( long );
//     test.close( `long - ${ name }` );
//
//     if( times < 1 )
//     break;
//     times--;
//   }
//
//   /* - */
//
//   function testRun( long )
//   {
//     test.case = 'src = null, not ins';
//     var got = long.long.makeZeroed( null );
//     var expected = long.long.default.make( 0 );
//     test.identical( got, expected );
//
//     test.case = 'src = number, not ins';
//     var got = long.long.makeZeroed( 5 );
//     var expected = long.long.default.make( [ 0, 0, 0, 0, 0 ] );
//     test.identical( got, expected );
//
//     /* aaa : should throw error : add test cases */ /* Dmytro : added in new test routines */
//     // test.case = 'src = number, ins = null';
//     // var got = long.long.makeZeroed( 5, null );
//     // var expected = long.long.default.make( [ 0, 0, 0, 0, 0 ] );
//     // test.identical( got, expected );
//     //
//     // test.case = 'src = number, ins = undefined';
//     // var got = long.long.makeZeroed( 5, undefined );
//     // var expected = long.long.default.make( [ 0, 0, 0, 0, 0 ] );
//     // test.identical( got, expected );
//
//     test.case = 'src = null, ins - number';
//     var got = long.long.makeZeroed( null, 5 );
//     var expected = long.long.default.make( [ 0, 0, 0, 0, 0 ] );
//     test.identical( got, expected );
//
//     test.case = 'src = null, ins - long';
//     var got = long.long.makeZeroed( null, _.argumentsArray.make( [ 1, 2, 3, 4, 5 ] ) );
//     var expected = long.long.default.make( [ 0, 0, 0, 0, 0 ] );
//     test.identical( got, expected );
//
//     test.case = 'src = empty long, not ins';
//     var src = _.argumentsArray.make( [] );
//     var got = long.long.makeZeroed( src );
//     var expected = long.long.default.make( [] );
//     test.identical( got, expected );
//
//     test.case = 'src = long, not ins';
//     var src = _.argumentsArray.make( [ 1, 2, 3 ] );
//     var got = long.long.makeZeroed( src );
//     var expected = long.long.default.make( [ 0, 0, 0 ] );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src = empty long, ins = null';
//     var src = _.argumentsArray.make( [] );
//     var got = long.long.makeZeroed( src, null );
//     var expected = long.long.default.make( 0 );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src = empty long, ins = number';
//     var src = _.argumentsArray.make( [] );
//     var got = long.long.makeZeroed( src, 2 );
//     var expected = long.long.default.make( [ 0, 0 ] );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src = long, ins = number, ins < src.length';
//     var src = _.argumentsArray.make( [ 1, 2, 3 ] );
//     var got = long.long.makeZeroed( src, 2 );
//     var expected = long.long.default.make( [ 0, 0 ] );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src = long with an element, ins = empty array';
//     var src = new F64x( 10 );
//     var got = long.long.makeZeroed( src, [] );
//     var expected = new F64x( 0 );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src = long, ins = number, ins > src.length';
//     var src = _.argumentsArray.make( [ 1, 2, 3 ] );
//     var got = long.long.makeZeroed( src, 4 );
//     var expected = long.long.default.make( [ 0, 0, 0, 0 ] );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src = long, ins = array, ins.length > src.length';
//     var src = _.argumentsArray.make( [ 0, 1 ] );
//     var ins = [ 1, 2, 3 ];
//     var got = long.long.makeZeroed( src, ins );
//     var expected = long.long.default.make( [ 0, 0, 0 ] );
//     test.identical( got, expected );
//     test.true( got !== ins );
//     test.true( got !== src );
//
//     test.case = 'src = long, ins = array, ins.length === src.length'
//     var src = _.argumentsArray.make( 5 );
//     var ins = [ 1, 2, 3, 4, 5 ];
//     var got = long.long.makeZeroed( src, ins );
//     var expected = long.long.default.make( [ 0, 0, 0, 0, 0 ] );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     // test.case = 'src = Array constructor, ins = null';
//     // var got = long.long.makeZeroed( Array, null );
//     // var expected = [];
//     // test.identical( got, expected );
//     // test.true( _.arrayIs( got ) );
//
//     test.case = 'src = BufferTyped constructor, ins = number';
//     var got = long.long.makeZeroed( U32x, 5 );
//     var expected = new U32x( [ 0, 0, 0, 0, 0 ] );
//     test.identical( got, expected );
//     test.true( _.buffer.typedIs(  got ) );
//
//     test.case = 'src = Array constructor, ins = long';
//     var ins = _.argumentsArray.make( [ 1, 2, 3 ] );
//     var got = long.long.makeZeroed( Array, ins );
//     var expected = [ 0, 0, 0 ];
//     test.identical( got, expected );
//     test.true( _.arrayIs( got ) );
//     test.true( got !== ins );
//   }
// }
//
// //
//
// function longMakeZeroedWithBufferTypedLongDescriptor( test )
// {
//   var list =
//   [
//     I8x,
//     U16x,
//     F32x,
//     F64x,
//   ];
//
//   /* tests */
//
//   let times = 4;
//   for( let e in _.LongDescriptors )
//   {
//     let name = _.LongDescriptors[ e ].name;
//     let long = _.withLong[ name ];
//
//     for( let i = 0; i < list.length; i++ )
//     {
//       test.open( `long - ${ name }, makeLong - ${ list[ i ].name }` );
//       testRun( long, list[ i ] );
//       test.close( `long - ${ name }, makeLong - ${ list[ i ].name }` );
//     }
//
//     if( times < 1 )
//     break;
//     times--;
//   }
//
//   /* test subroutine */
//
//   function testRun( long, makeLong )
//   {
//     test.case = 'src = null, not ins';
//     var got = long.long.makeZeroed( null );
//     var expected = long.long.default.make( 0 );
//     test.identical( got, expected );
//
//     test.case = 'src = number, not ins';
//     var got = long.long.makeZeroed( 5 );
//     var expected = long.long.default.make( [ 0, 0, 0, 0, 0 ] );
//     test.identical( got, expected );
//
//     /* aaa : should throw error : add test cases */ /* Dmytro : added in new test routines */
//     // test.case = 'src = number, ins = null';
//     // var got = long.long.makeZeroed( 5, null );
//     // var expected = long.long.default.make( [ 0, 0, 0, 0, 0 ] );
//     // test.identical( got, expected );
//     //
//     // test.case = 'src = number, ins = undefined';
//     // var got = long.long.makeZeroed( 5, undefined );
//     // var expected = long.long.default.make( [ 0, 0, 0, 0, 0 ] );
//     // test.identical( got, expected );
//
//     test.case = 'src = null, ins - number';
//     var got = long.long.makeZeroed( null, 5 );
//     var expected = long.long.default.make( [ 0, 0, 0, 0, 0 ] );
//     test.identical( got, expected );
//
//     test.case = 'src = null, ins - long';
//     var got = long.long.makeZeroed( null, new makeLong( [ 1, 2, 3, 4, 5 ] ) );
//     var expected = long.long.default.make( [ 0, 0, 0, 0, 0 ] );
//     test.identical( got, expected );
//
//     test.case = 'src = empty long, not ins';
//     var src = new makeLong( [] );
//     var got = long.long.makeZeroed( src );
//     var expected = new makeLong( [] );
//     test.identical( got, expected );
//
//     test.case = 'src = long, not ins';
//     var src = new makeLong( [ 1, 2, 3 ] );
//     var got = long.long.makeZeroed( src );
//     var expected = new makeLong( [ 0, 0, 0 ] );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src = empty long, ins = null';
//     var src = new makeLong( [] );
//     var got = long.long.makeZeroed( src, null );
//     var expected = new makeLong( 0 );
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( src.constructor.name === got.constructor.name );
//
//     test.case = 'src = long, ins = number';
//     var src = new makeLong( 10 );
//     var got = long.long.makeZeroed( src.constructor, 4 );
//     var expected = new makeLong( [ 0, 0, 0, 0 ] );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src = empty long, ins = number';
//     var src = new makeLong( [] );
//     var got = long.long.makeZeroed( src, 2 );
//     var expected = new makeLong( [ 0, 0 ] );
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( src.constructor.name === got.constructor.name );
//
//     test.case = 'src = long, ins = number, ins < src.length';
//     var src = new makeLong( [ 1, 2, 3 ] );
//     var got = long.long.makeZeroed( src, 2 );
//     var expected = new makeLong( [ 0, 0 ] );
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( src.constructor.name === got.constructor.name );
//
//     test.case = 'src = long with an element, ins = empty array';
//     var src = new F64x( 10 );
//     var got = long.long.makeZeroed( src, [] );
//     var expected = new F64x( 0 );
//     test.identical( got, expected );
//     test.true( got !== src );
//
//     test.case = 'src = long, ins = number, ins > src.length';
//     var src = new makeLong( [ 1, 2, 3 ] );
//     var got = long.long.makeZeroed( src, 4 );
//     var expected = new makeLong( [ 0, 0, 0, 0 ] );
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( src.constructor.name === got.constructor.name );
//
//     test.case = 'src = long, ins = array, ins.length > src.length';
//     var src = new makeLong( [ 0, 1 ] );
//     var ins = [ 1, 2, 3 ];
//     var got = long.long.makeZeroed( src, ins );
//     var expected = new makeLong( [ 0, 0, 0 ] );
//     test.identical( got, expected );
//     test.true( got !== ins );
//     test.true( got !== src );
//     test.true( src.constructor.name === got.constructor.name );
//
//     test.case = 'src = long, ins = array, ins.length === src.length'
//     var src = new makeLong( 5 );
//     var ins = [ 1, 2, 3, 4, 5 ];
//     var got = long.long.makeZeroed( src, ins );
//     var expected = new makeLong( [ 0, 0, 0, 0, 0 ] );
//     test.identical( got, expected );
//     test.true( got !== src );
//     test.true( src.constructor.name === got.constructor.name );
//
//     // test.case = 'src = Array constructor, ins = null';
//     // var got = long.long.makeZeroed( Array, null );
//     // var expected = [];
//     // test.identical( got, expected );
//     // test.true( _.arrayIs( got ) );
//
//     test.case = 'src = BufferTyped constructor, ins = number';
//     var got = long.long.makeZeroed( U32x, 5 );
//     var expected = new U32x( [ 0, 0, 0, 0, 0 ] );
//     test.identical( got, expected );
//     test.true( _.buffer.typedIs(  got ) );
//
//     test.case = 'src = Array constructor, ins = long';
//     var ins = new makeLong( [ 1, 2, 3 ] );
//     var got = long.long.makeZeroed( Array, ins );
//     var expected = [ 0, 0, 0 ];
//     test.identical( got, expected );
//     test.true( _.arrayIs( got ) );
//     test.true( got !== ins );
//   }
// }
//
// longMakeZeroedWithBufferTypedLongDescriptor.timeOut = 20000;

//

function makeFillingCommon( test )
{
  act({ tools : 'default', type : 'Array' });
  act({ tools : 'Array', type : 'Array' });
  act({ tools : 'F32x', type : 'F32x' });
  act({ tools : 'bufferTyped', type : 'Array' });

  /* */

  function act( env )
  {
    test.open( `${ __.entity.exportStringSolo( env ) }` );

    const long = namespaceGet( env );
    const Constructor = defaultConstructorGet( env );

    /* */

    test.case = 'value - null, length - number';
    var got = long.makeFilling( null, 3 );
    var expected = Constructor.from([ null, null, null ]);
    test.identical( got, expected );

    test.case = `value - number, length - filled array`;
    var length = [ 1, 2, 3 ];
    var got = long.makeFilling( 1, length );
    if( env.tools === 'bufferTyped' )
    var expected = Constructor.from([ 1, 1, 1 ]);
    else
    var expected = [ 1, 1, 1 ];
    test.identical( got, expected );

    test.case = `value - number, length - countable`;
    var length = __.diagnostic.objectMake({ elements : [ 1, 2, 3 ], countable : 1 });
    var got = long.makeFilling( 1, length );
    var expected = Constructor.from([ 1, 1, 1 ]);
    test.identical( got, expected );

    /* */

    test.case = 'type - null, value - number, length - number';
    var got = long.makeFilling( null, 1, 3 );
    var expected = Constructor.from([ 1, 1, 1 ]);
    test.identical( got, expected );

    test.case = 'type - null, value - number, length - long';
    var got = long.makeFilling( null, 1, new U8x( 3 ) );
    var expected = Constructor.from([ 1, 1, 1 ]);
    test.identical( got, expected );

    test.case = 'type - null, value - number, length - countable';
    var length = __.diagnostic.objectMake({ elements : [ 1, 2, 3 ], countable : 1 });
    var got = long.makeFilling( null, 1, length );
    var expected = Constructor.from([ 1, 1, 1 ]);
    test.identical( got, expected );

    /* */

    test.case = `type - Array constructor, value - number, length - number`;
    var got = long.makeFilling( Array, 10, 3 );
    if( env.tools === 'bufferTyped' )
    {
      var expected = Constructor.from([ 10, 10, 10 ]);
      test.identical( got, expected );
    }
    else
    {
      var expected = [ 10, 10, 10 ];
      test.identical( got, expected );
    }

    test.case = `type - Array constructor, value - number, length - long`;
    var got = long.makeFilling( Array, 10, new I16x( 3 ) );
    if( env.tools === 'bufferTyped' )
    {
      var expected = Constructor.from([ 10, 10, 10 ]);
      test.identical( got, expected );
    }
    else
    {
      var expected = [ 10, 10, 10 ];
      test.identical( got, expected );
    }

    /* */

    test.case = `type - empty array, value - number, length - number`;
    var got = long.makeFilling( [], 10, 3 );
    if( env.tools === 'bufferTyped' )
    {
      var expected = Constructor.from([ 10, 10, 10 ]);
      test.identical( got, expected );
    }
    else
    {
      var expected = [ 10, 10, 10 ];
      test.identical( got, expected );
    }

    test.case = `type - filled array, value - number, length - long`;
    var got = long.makeFilling( [ 1, 2, 3, 4 ], 10, new I16x( 3 ) );
    if( env.tools === 'bufferTyped' )
    {
      var expected = Constructor.from([ 10, 10, 10 ]);
      test.identical( got, expected );
    }
    else
    {
      var expected = [ 10, 10, 10 ];
      test.identical( got, expected );
    }

    /* */

    test.case = `type - empty unroll, value - number, length - number`;
    var got = long.makeFilling( _.unroll.make( [] ), 10, 3 );
    if( env.tools === 'bufferTyped' )
    {
      var expected = Constructor.from([ 10, 10, 10 ]);
      test.identical( got, expected );
    }
    else
    {
      var expected = _.unroll.make([ 10, 10, 10 ]);
      test.identical( got, expected );
    }

    test.case = `type - filled unroll, value - number, length - long`;
    var got = long.makeFilling( _.unroll.make([ 1, 2, 3, 4 ]), 10, new I16x( 3 ) );
    if( env.tools === 'bufferTyped' )
    {
      var expected = Constructor.from([ 10, 10, 10 ]);
      test.identical( got, expected );
    }
    else
    {
      var expected = _.unroll.make([ 10, 10, 10 ]);
      test.identical( got, expected );
    }

    /* */

    test.case = `type - empty argumentsArray, value - number, length - number`;
    var got = long.makeFilling( _.argumentsArray.make( [] ), 10, 3 );
    if( env.tools === 'bufferTyped' )
    {
      var expected = Constructor.from([ 10, 10, 10 ]);
      test.identical( got, expected );
    }
    else
    {
      var expected = _.argumentsArray.make([ 10, 10, 10 ]);
      test.identical( got, expected );
    }

    test.case = `type - filled argumentsArray, value - number, length - long`;
    var got = long.makeFilling( _.argumentsArray.make([ 1, 2, 3, 4 ]), 10, new I16x( 3 ) );
    if( env.tools === 'bufferTyped' )
    {
      var expected = Constructor.from([ 10, 10, 10 ]);
      test.identical( got, expected );
    }
    else
    {
      var expected = _.argumentsArray.make([ 10, 10, 10 ]);
      test.identical( got, expected );
    }

    /* */

    test.case = `type - empty typed array, value - number, length - number`;
    var got = long.makeFilling( _.u8x.make( [] ), 10, 3 );
    var expected = _.u8x.make([ 10, 10, 10 ]);
    test.identical( got, expected );

    test.case = `type - filled typed array, value - number, length - long`;
    var got = long.makeFilling( _.f32x.make([ 1, 2, 3, 4 ]), 10, new I16x( 3 ) );
    var expected = _.f32x.make([ 10, 10, 10 ]);
    test.identical( got, expected );

    /* */

    test.case = `type - empty typed array, value - number, length - countable`;
    var length = __.diagnostic.objectMake({ elements : [ 1, 2, 3 ], countable : 1 });
    var got = long.makeFilling( _.u8x.make( [] ), 10, length );
    var expected = _.u8x.make([ 10, 10, 10 ]);
    test.identical( got, expected );

    /* */

    if( Config.debug )
    {
      test.case = 'without arguments';
      test.shouldThrowErrorSync( () => long.makeFilling() );

      test.case = 'not enough arguments';
      test.shouldThrowErrorSync( () => long.makeFilling( null ) );

      test.case = 'extra arguments';
      test.shouldThrowErrorSync( () => long.makeFilling( [ 1, 2, 3 ], 4, 4, 'extra' ) );

      test.case = 'wrong type of type argument';
      test.shouldThrowErrorSync( () => long.makeFilling( {}, 2, 2 ) );
      test.shouldThrowErrorSync( () => long.makeFilling( undefined, 2, 2 ) );

      test.case = 'wrong type of value';
      test.shouldThrowErrorSync( () => long.makeFilling( undefined, 1 ) );
      test.shouldThrowErrorSync( () => long.makeFilling( [], undefined, 1 ) );

      test.case = 'wrong type of length';
      test.shouldThrowErrorSync( () => long.makeFilling( [ 1, 2 ], 'wrong' ) );
      test.shouldThrowErrorSync( () => long.makeFilling( 1, 'wrong' ) );
      test.shouldThrowErrorSync( () => long.makeFilling( Array, 'wrong' ) );
      test.shouldThrowErrorSync( () => long.makeFilling( [ 1, 2 ], null ) );
      test.shouldThrowErrorSync( () => long.makeFilling( 1, null ) );
      test.shouldThrowErrorSync( () => long.makeFilling( Array, null ) );
      test.shouldThrowErrorSync( () => long.makeFilling( [ 1, 2 ], undefined ) );
      test.shouldThrowErrorSync( () => long.makeFilling( [ 1, 2 ], 2, undefined ) );
    }

    test.close( `${ __.entity.exportStringSolo( env ) }` );
  }

  /* */

  function namespaceGet( env )
  {
    if( env.tools === 'default' )
    return _.long;
    if( env.tools === 'bufferTyped' )
    return _.bufferTyped;
    return _.withLong[ env.tools ].long;
  }

  /* */

  function defaultConstructorGet( env )
  {
    if( env.tools === 'default' )
    return _.long.default.InstanceConstructor;
    if( env.tools === 'bufferTyped' )
    return _.bufferTyped.default.InstanceConstructor;
    return _.withLong[ env.tools ].long.default.InstanceConstructor;
  }
}

//

function makeFillingCommonWithLongDescriptor( test )
{
  let times = 4;
  for( let k in _.long.namespaces )
  {
    let namespace = _.long.namespaces[ k ];
    let type = namespace.TypeName;

    if( type === 'ArgumentsArray' )
    continue;

    test.open( `long - ${ type }` );
    act({ tools : 'default', type });
    act({ tools : 'Array', type });
    act({ tools : 'F32x', type });
    act({ tools : 'bufferTyped', type });
    test.close( `long - ${ type }` );

    if( times < 1 )
    break;
    times--;
  }

  /* */

  function act( env )
  {
    test.open( `${ __.entity.exportStringSolo( env ) }` );

    const long = namespaceGet( env );
    const Constructor = defaultConstructorGet( env );

    /* */

    test.case = 'value - null, length - number';
    var got = long.makeFilling( null, 3 );
    var expected = Constructor.from([ null, null, null ]);
    test.identical( got, expected );

    test.case = `value - number, length - filled array`;
    var length = [ 1, 2, 3 ];
    var got = long.makeFilling( 1, length );
    if( env.tools === 'bufferTyped' )
    var expected = Constructor.from([ 1, 1, 1 ]);
    else
    var expected = [ 1, 1, 1 ];
    test.identical( got, expected );

    test.case = `value - number, length - countable`;
    var length = __.diagnostic.objectMake({ elements : [ 1, 2, 3 ], countable : 1 });
    var got = long.makeFilling( 1, length );
    var expected = Constructor.from([ 1, 1, 1 ]);
    test.identical( got, expected );

    /* */

    test.case = 'type - null, value - number, length - number';
    var got = long.makeFilling( null, 1, 3 );
    var expected = Constructor.from([ 1, 1, 1 ]);
    test.identical( got, expected );

    test.case = 'type - null, value - number, length - long';
    var got = long.makeFilling( null, 1, new U8x( 3 ) );
    var expected = Constructor.from([ 1, 1, 1 ]);
    test.identical( got, expected );

    test.case = 'type - null, value - number, length - countable';
    var length = __.diagnostic.objectMake({ elements : [ 1, 2, 3 ], countable : 1 });
    var got = long.makeFilling( null, 1, length );
    var expected = Constructor.from([ 1, 1, 1 ]);
    test.identical( got, expected );

    /* */

    test.case = `type - Array constructor, value - number, length - number`;
    var got = long.makeFilling( Array, 10, 3 );
    if( env.tools === 'bufferTyped' )
    {
      var expected = Constructor.from([ 10, 10, 10 ]);
      test.identical( got, expected );
    }
    else
    {
      var expected = [ 10, 10, 10 ];
      test.identical( got, expected );
    }

    test.case = `type - Array constructor, value - number, length - long`;
    var got = long.makeFilling( Array, 10, new I16x( 3 ) );
    if( env.tools === 'bufferTyped' )
    {
      var expected = Constructor.from([ 10, 10, 10 ]);
      test.identical( got, expected );
    }
    else
    {
      var expected = [ 10, 10, 10 ];
      test.identical( got, expected );
    }

    /* */

    test.case = `type - empty array, value - number, length - number`;
    var got = long.makeFilling( [], 10, 3 );
    if( env.tools === 'bufferTyped' )
    {
      var expected = Constructor.from([ 10, 10, 10 ]);
      test.identical( got, expected );
    }
    else
    {
      var expected = [ 10, 10, 10 ];
      test.identical( got, expected );
    }

    test.case = `type - filled array, value - number, length - long`;
    var got = long.makeFilling( [ 1, 2, 3, 4 ], 10, new I16x( 3 ) );
    if( env.tools === 'bufferTyped' )
    {
      var expected = Constructor.from([ 10, 10, 10 ]);
      test.identical( got, expected );
    }
    else
    {
      var expected = [ 10, 10, 10 ];
      test.identical( got, expected );
    }

    /* */

    test.case = `type - empty unroll, value - number, length - number`;
    var got = long.makeFilling( _.unroll.make( [] ), 10, 3 );
    if( env.tools === 'bufferTyped' )
    {
      var expected = Constructor.from([ 10, 10, 10 ]);
      test.identical( got, expected );
    }
    else
    {
      var expected = _.unroll.make([ 10, 10, 10 ]);
      test.identical( got, expected );
    }

    test.case = `type - filled unroll, value - number, length - long`;
    var got = long.makeFilling( _.unroll.make([ 1, 2, 3, 4 ]), 10, new I16x( 3 ) );
    if( env.tools === 'bufferTyped' )
    {
      var expected = Constructor.from([ 10, 10, 10 ]);
      test.identical( got, expected );
    }
    else
    {
      var expected = _.unroll.make([ 10, 10, 10 ]);
      test.identical( got, expected );
    }

    /* */

    test.case = `type - empty argumentsArray, value - number, length - number`;
    var got = long.makeFilling( _.argumentsArray.make( [] ), 10, 3 );
    if( env.tools === 'bufferTyped' )
    {
      var expected = Constructor.from([ 10, 10, 10 ]);
      test.identical( got, expected );
    }
    else
    {
      var expected = _.argumentsArray.make([ 10, 10, 10 ]);
      test.identical( got, expected );
    }

    test.case = `type - filled argumentsArray, value - number, length - long`;
    var got = long.makeFilling( _.argumentsArray.make([ 1, 2, 3, 4 ]), 10, new I16x( 3 ) );
    if( env.tools === 'bufferTyped' )
    {
      var expected = Constructor.from([ 10, 10, 10 ]);
      test.identical( got, expected );
    }
    else
    {
      var expected = _.argumentsArray.make([ 10, 10, 10 ]);
      test.identical( got, expected );
    }

    /* */

    test.case = `type - empty typed array, value - number, length - number`;
    var got = long.makeFilling( _.u8x.make( [] ), 10, 3 );
    var expected = _.u8x.make([ 10, 10, 10 ]);
    test.identical( got, expected );

    test.case = `type - filled typed array, value - number, length - long`;
    var got = long.makeFilling( _.f32x.make([ 1, 2, 3, 4 ]), 10, new I16x( 3 ) );
    var expected = _.f32x.make([ 10, 10, 10 ]);
    test.identical( got, expected );

    /* */

    test.case = `type - empty typed array, value - number, length - countable`;
    var length = __.diagnostic.objectMake({ elements : [ 1, 2, 3 ], countable : 1 });
    var got = long.makeFilling( _.u8x.make( [] ), 10, length );
    var expected = _.u8x.make([ 10, 10, 10 ]);
    test.identical( got, expected );

    /* */

    if( Config.debug )
    {
      test.case = 'without arguments';
      test.shouldThrowErrorSync( () => long.makeFilling() );

      test.case = 'not enough arguments';
      test.shouldThrowErrorSync( () => long.makeFilling( null ) );

      test.case = 'extra arguments';
      test.shouldThrowErrorSync( () => long.makeFilling( [ 1, 2, 3 ], 4, 4, 'extra' ) );

      test.case = 'wrong type of type argument';
      test.shouldThrowErrorSync( () => long.makeFilling( {}, 2, 2 ) );
      test.shouldThrowErrorSync( () => long.makeFilling( undefined, 2, 2 ) );

      test.case = 'wrong type of value';
      test.shouldThrowErrorSync( () => long.makeFilling( undefined, 1 ) );
      test.shouldThrowErrorSync( () => long.makeFilling( [], undefined, 1 ) );

      test.case = 'wrong type of length';
      test.shouldThrowErrorSync( () => long.makeFilling( [ 1, 2, 3 ], 'wrong' ) );
      test.shouldThrowErrorSync( () => long.makeFilling( [ 1, 2, 3 ], 2, undefined ) );
    }

    test.close( `${ __.entity.exportStringSolo( env ) }` );
  }

  /* */

  function namespaceGet( env )
  {
    if( env.tools === 'default' )
    return _.long;
    if( env.tools === 'bufferTyped' )
    return _.bufferTyped;
    return _.withLong[ env.tools ].long;
  }

  /* */

  function defaultConstructorGet( env )
  {
    if( env.tools === 'default' )
    return _.long.default.InstanceConstructor;
    if( env.tools === 'bufferTyped' )
    return _.bufferTyped.default.InstanceConstructor;
    return _.withLong[ env.tools ].long.default.InstanceConstructor;
  }
}

//

// function longMakeFillingWithArrayAndUnroll( test )
// {
//   var array = ( src ) => _.array.make( src );
//   var unroll = ( src ) => _.unroll.make( src );
//   var longConstr = ( src ) =>
//   {
//     if( src )
//     return _.long.default.make( src );
//     return _.long.default.make( 0 );
//   }
//   var list =
//   [
//     array,
//     unroll,
//     longConstr,
//   ];
//
//   /* tests */
//
//   for( let t = 0; t < list.length; t++ )
//   {
//     test.open( list[ t ].name );
//     testRun( list[ t ] );
//     test.close( list[ t ].name );
//   }
//
//   /* test subroutine */
//
//   function testRun( makeLong )
//   {
//     test.case = 'value - null, length - number';
//     var got = _.longMakeFilling( null, 5 );
//     var expected = _.long.default.make( [ null, null, null, null, null ] );
//     test.identical( got, expected );
//
//     test.case = `value - zero, length - ${ makeLong.name }`;
//     var got = _.longMakeFilling( 0, makeLong( 5 ) );
//     var expected = makeLong( [ 0, 0, 0, 0, 0 ] );
//     test.identical( got, expected );
//
//     /* */
//
//     test.case = 'type - null, value - string, length - number';
//     var got = _.longMakeFilling( null, 'str', 5 );
//     var expected = _.long.default.make( [ 'str', 'str', 'str', 'str', 'str' ] );
//     test.identical( got, expected );
//
//     test.case = 'type - null, value - string, length - BufferTyped';
//     var got = _.longMakeFilling( null, 'str', new U8x( 5 ) );
//     var expected = _.long.default.make( [ 'str', 'str', 'str', 'str', 'str' ] );
//     test.identical( got, expected );
//
//     test.case = `type - ${ makeLong.name } constructor, value - array, length - number`;
//     var got = _.longMakeFilling( makeLong( 0 ).constructor, [ 1 ], 3 );
//     var expected = makeLong( [ [ 1 ], [ 1 ], [ 1 ] ] );
//     test.identical( got, expected );
//
//     test.case = `type - ${ makeLong.name } constructor, value - array, length - empty ${ makeLong.name }`;
//     var got = _.longMakeFilling( makeLong( 0 ).constructor, [ 1 ], makeLong( 0 ) );
//     var expected = makeLong( [] );
//     test.identical( got, expected );
//
//     test.case = `type - ${ makeLong.name } instance, value - map, length - number`;
//     var got = _.longMakeFilling( makeLong( 0 ), { a : 1 }, 3 );
//     var expected = makeLong( [ { a : 1 }, { a : 1 }, { a : 1 } ] );
//     test.identical( got, expected );
//
//     test.case = `type - ${ makeLong.name } instance, value - map, length - ${ makeLong.name }`;
//     var got = _.longMakeFilling( makeLong( 0 ), { a : 1 }, makeLong( 3 ) );
//     var expected = makeLong( [ { a : 1 }, { a : 1 }, { a : 1 } ] );
//     test.identical( got, expected );
//
//     test.case = `type - U8x, value - number, length - number`;
//     var got = _.longMakeFilling( U8x, 10, 3 );
//     var expected = new U8x( [ 10, 10, 10 ] );
//     test.identical( got, expected );
//
//     test.case = `type - I16x, value - number, length - ${ makeLong.name }`;
//     var got = _.longMakeFilling( I16x, 10, makeLong( 3 ) );
//     var expected = new I16x( [ 10, 10, 10 ] );
//     test.identical( got, expected );
//
//     test.case = `type - F32x instance, value - number, length - number`;
//     var got = _.longMakeFilling( new F32x( 10 ), 10, 3 );
//     var expected = new F32x( [ 10, 10, 10 ] );
//     test.identical( got, expected );
//
//     test.case = `type - F32x instance, value - number, length - ${ makeLong.name }`;
//     var got = _.longMakeFilling( new F32x( 10 ), 10, makeLong( 3 ) );
//     var expected = new F32x( [ 10, 10, 10 ] );
//     test.identical( got, expected );
//   }
//
//   /* - */
//
//   if( !Config.debug )
//   return;
//
//   test.case = 'without arguments';
//   test.shouldThrowErrorSync( () => _.longMakeFilling() );
//
//   test.case = 'not enough arguments';
//   test.shouldThrowErrorSync( () => _.longMakeFilling( null ) );
//
//   test.case = 'extra arguments';
//   test.shouldThrowErrorSync( () => _.longMakeFilling( [ 1, 2, 3 ], 4, 4, 'extra' ) );
//
//   test.case = 'wrong type of type argument';
//   test.shouldThrowErrorSync( () => _.longMakeFilling( {}, 2, 2 ) );
//   test.shouldThrowErrorSync( () => _.longMakeFilling( undefined, 2, 2 ) );
//
//   test.case = 'wrong type of value';
//   test.shouldThrowErrorSync( () => _.longMakeFilling( undefined, 1 ) );
//   test.shouldThrowErrorSync( () => _.longMakeFilling( [], undefined, 1 ) );
//
//   test.case = 'wrong type of length';
//   test.shouldThrowErrorSync( () => _.longMakeFilling( [ 1, 2, 3 ], 'wrong' ) );
//   test.shouldThrowErrorSync( () => _.longMakeFilling( [ 1, 2, 3 ], 2, undefined ) );
// }
//
// //
//
// function longMakeFillingWithArgumentsArray( test )
// {
//   test.case = 'value - null, length - number';
//   var got = _.longMakeFilling( null, 5 );
//   var expected = _.long.default.make( [ null, null, null, null, null ] );
//   test.identical( got, expected );
//
//   test.case = `value - zero, length - ArgumentsArray`;
//   var got = _.longMakeFilling( 0, _.argumentsArray.make( 5 ) );
//   var expected = _.long.default.make( [ 0, 0, 0, 0, 0 ] );
//   test.identical( got, expected );
//
//   /* */
//
//   test.case = 'type - null, value - string, length - number';
//   var got = _.longMakeFilling( null, 'str', 5 );
//   var expected = _.long.default.make( [ 'str', 'str', 'str', 'str', 'str' ] );
//   test.identical( got, expected );
//
//   test.case = 'type - null, value - string, length - ArgumentsArray';
//   var got = _.longMakeFilling( null, 'str', _.argumentsArray.make( 5 ) );
//   var expected = _.long.default.make( [ 'str', 'str', 'str', 'str', 'str' ] );
//   test.identical( got, expected );
//
//   test.case = `type - ArgumentsArray instance, value - map, length - number`;
//   var got = _.longMakeFilling( _.argumentsArray.make( 0 ), { a : 1 }, 3 );
//   var expected = _.long.default.make( [ { a : 1 }, { a : 1 }, { a : 1 } ] );
//   test.identical( got, expected );
//
//   test.case = `type - ArgumentsArray instance, value - map, length - ArgumentsArray`;
//   var got = _.longMakeFilling( _.argumentsArray.make( 0 ), { a : 1 }, _.argumentsArray.make( 3 ) );
//   var expected = _.long.default.make( [ { a : 1 }, { a : 1 }, { a : 1 } ] );
//   test.identical( got, expected );
//
//   test.case = `type - U8x, value - number, length - number`;
//   var got = _.longMakeFilling( U8x, 10, 3 );
//   var expected = new U8x( [ 10, 10, 10 ] );
//   test.identical( got, expected );
//
//   test.case = `type - I16x, value - number, length - ArgumentsArray`;
//   var got = _.longMakeFilling( I16x, 10, _.argumentsArray.make( 3 ) );
//   var expected = new I16x( [ 10, 10, 10 ] );
//   test.identical( got, expected );
//
//   test.case = `type - F32x instance, value - number, length - number`;
//   var got = _.longMakeFilling( new F32x( 10 ), 10, 3 );
//   var expected = new F32x( [ 10, 10, 10 ] );
//   test.identical( got, expected );
//
//   test.case = `type - F32x instance, value - number, length - ArgumentsArray`;
//   var got = _.longMakeFilling( new F32x( 10 ), 10, _.argumentsArray.make( 3 ) );
//   var expected = new F32x( [ 10, 10, 10 ] );
//   test.identical( got, expected );
//
//   /* */
//
//   if( !Config.debug )
//   return;
//
//   test.case = `type - ArgumentsArray constructor, value - array, length - number`;
//   test.shouldThrowErrorSync( () => _.longMakeFilling( _.argumentsArray.make( 0 ).constructor, [ 1 ], 3 ) );
// }
//
// //
//
// function longMakeFillingWithBufferTyped( test )
// {
//   var list =
//   [
//     I8x,
//     U16x,
//     U16x,
//     F32x,
//   ];
//
//   /* tests */
//
//   for( let t = 0; t < list.length; t++ )
//   {
//     test.open( list[ t ].name );
//     testRun( list[ t ] );
//     test.close( list[ t ].name );
//   }
//
//   /* test subroutine */
//
//
//   function testRun( makeLong )
//   {
//     test.case = 'value - null, length - number';
//     var got = _.longMakeFilling( null, 5 );
//     var expected = _.long.default.make( [ null, null, null, null, null ] );
//     test.identical( got, expected );
//
//     test.case = `value - zero, length - ${ makeLong.name }`;
//     var got = _.longMakeFilling( 0, new makeLong( 5 ) );
//     var expected = new makeLong( [ 0, 0, 0, 0, 0 ] );
//     test.identical( got, expected );
//
//     /* */
//
//     test.case = 'type - null, value - string, length - number';
//     var got = _.longMakeFilling( null, 'str', 5 );
//     var expected = _.long.default.make( [ 'str', 'str', 'str', 'str', 'str' ] );
//     test.identical( got, expected );
//
//     test.case = 'type - null, value - string, length - BufferTyped';
//     var got = _.longMakeFilling( null, 'str', new U8x( 5 ) );
//     var expected = _.long.default.make( [ 'str', 'str', 'str', 'str', 'str' ] );
//     test.identical( got, expected );
//
//     test.case = `type - ${ makeLong.name } constructor, value - array, length - number`;
//     var got = _.longMakeFilling( new makeLong, [ 1 ], 3 );
//     var expected = new makeLong( [ [ 1 ], [ 1 ], [ 1 ] ] );
//     test.identical( got, expected );
//
//     test.case = `type - ${ makeLong.name } constructor, value - array, length - empty ${ makeLong.name }`;
//     var got = _.longMakeFilling( new makeLong, [ 1 ], new makeLong( 0 ) );
//     var expected = new makeLong( [] );
//     test.identical( got, expected );
//
//     test.case = `type - ${ makeLong.name } instance, value - map, length - number`;
//     var got = _.longMakeFilling( new makeLong( 0 ), { a : 1 }, 3 );
//     var expected = new makeLong( [ { a : 1 }, { a : 1 }, { a : 1 } ] );
//     test.identical( got, expected );
//
//     test.case = `type - ${ makeLong.name } instance, value - map, length - ${ makeLong.name }`;
//     var got = _.longMakeFilling( new makeLong( 0 ), { a : 1 }, new makeLong( 3 ) );
//     var expected = new makeLong( [ { a : 1 }, { a : 1 }, { a : 1 } ] );
//     test.identical( got, expected );
//
//     test.case = `type - Array, value - number, length - number`;
//     var got = _.longMakeFilling( Array, 10, 3 );
//     var expected = new Array( 10, 10, 10 );
//     test.identical( got, expected );
//
//     test.case = `type - Array, value - number, length - ${ makeLong.name }`;
//     var got = _.longMakeFilling( Array, 10, new makeLong( 3 ) );
//     var expected = new Array( 10, 10, 10 );
//     test.identical( got, expected );
//
//     test.case = `type - Array instance, value - number, length - number`;
//     var got = _.longMakeFilling( new Array( 10 ), 10, 3 );
//     var expected = new Array( 10, 10, 10 );
//     test.identical( got, expected );
//
//     test.case = `type - Array instance, value - number, length - ${ makeLong.name }`;
//     var got = _.longMakeFilling( new Array( 10 ), 10, new makeLong( 3 ) );
//     var expected = new Array( 10, 10, 10 );
//     test.identical( got, expected );
//   }
// }
//
// //
//
// function longMakeFillingWithArrayAndUnrollLongDescriptor( test )
// {
//   var array = ( src ) => _.array.make( src );
//   var unroll = ( src ) => _.unroll.make( src );
//   var longConstr = ( src ) =>
//   {
//     if( src )
//     return _.long.default.make( src );
//     return _.long.default.make( 0 );
//   }
//   var list =
//   [
//     array,
//     unroll,
//     longConstr,
//   ];
//
//   /* tests */
//
//   let times = 4;
//   for( let e in _.LongDescriptors )
//   {
//     let name = _.LongDescriptors[ e ].name;
//     let long = _.withLong[ name ];
//
//     for( let i = 0; i < list.length; i++ )
//     {
//       test.open( `long - ${ name }, long - ${ list[ i ].name }` );
//       testRun( long, list[ i ] );
//       test.close( `long - ${ name }, long - ${ list[ i ].name }` );
//     }
//
//     if( times < 1 )
//     break;
//     times--;
//   }
//
//   /* test subroutine */
//
//   function testRun( long, makeLong )
//   {
//     test.case = 'value - null, length - number';
//     var got = long.longMakeFilling( null, 5 );
//     var expected = long.long.default.make( [ null, null, null, null, null ] );
//     test.identical( got, expected );
//
//     test.case = `value - zero, length - ${ makeLong.name }`;
//     var got = long.longMakeFilling( 0, makeLong( 5 ) );
//     var expected = makeLong( [ 0, 0, 0, 0, 0 ] );
//     test.identical( got, expected );
//
//     /* */
//
//     test.case = 'type - null, value - string, length - number';
//     var got = long.longMakeFilling( null, 'str', 5 );
//     var expected = long.long.default.make( [ 'str', 'str', 'str', 'str', 'str' ] );
//     test.identical( got, expected );
//
//     test.case = 'type - null, value - string, length - BufferTyped';
//     var got = long.longMakeFilling( null, 'str', new U8x( 5 ) );
//     var expected = long.long.default.make( [ 'str', 'str', 'str', 'str', 'str' ] );
//     test.identical( got, expected );
//
//     test.case = `type - ${ makeLong.name } constructor, value - array, length - number`;
//     var got = long.longMakeFilling( makeLong( 0 ).constructor, [ 1 ], 3 );
//     var expected = makeLong( [ [ 1 ], [ 1 ], [ 1 ] ] );
//     test.identical( got, expected );
//
//     test.case = `type - ${ makeLong.name } constructor, value - array, length - empty ${ makeLong.name }`;
//     var got = long.longMakeFilling( makeLong( 0 ).constructor, [ 1 ], makeLong( 0 ) );
//     var expected = makeLong( [] );
//     test.identical( got, expected );
//
//     test.case = `type - ${ makeLong.name } instance, value - map, length - number`;
//     var got = long.longMakeFilling( makeLong( 0 ), { a : 1 }, 3 );
//     var expected = makeLong( [ { a : 1 }, { a : 1 }, { a : 1 } ] );
//     test.identical( got, expected );
//
//     test.case = `type - ${ makeLong.name } instance, value - map, length - ${ makeLong.name }`;
//     var got = long.longMakeFilling( makeLong( 0 ), { a : 1 }, makeLong( 3 ) );
//     var expected = makeLong( [ { a : 1 }, { a : 1 }, { a : 1 } ] );
//     test.identical( got, expected );
//
//     test.case = `type - U8x, value - number, length - number`;
//     var got = long.longMakeFilling( U8x, 10, 3 );
//     var expected = new U8x( [ 10, 10, 10 ] );
//     test.identical( got, expected );
//
//     test.case = `type - I16x, value - number, length - ${ makeLong.name }`;
//     var got = long.longMakeFilling( I16x, 10, makeLong( 3 ) );
//     var expected = new I16x( [ 10, 10, 10 ] );
//     test.identical( got, expected );
//
//     test.case = `type - F32x instance, value - number, length - number`;
//     var got = long.longMakeFilling( new F32x( 10 ), 10, 3 );
//     var expected = new F32x( [ 10, 10, 10 ] );
//     test.identical( got, expected );
//
//     test.case = `type - F32x instance, value - number, length - ${ makeLong.name }`;
//     var got = long.longMakeFilling( new F32x( 10 ), 10, makeLong( 3 ) );
//     var expected = new F32x( [ 10, 10, 10 ] );
//     test.identical( got, expected );
//
//     /* - */
//
//     if( Config.debug )
//     {
//       test.case = 'without arguments';
//       test.shouldThrowErrorSync( () => long.longMakeFilling() );
//
//       test.case = 'not enough arguments';
//       test.shouldThrowErrorSync( () => long.longMakeFilling( null ) );
//
//       test.case = 'extra arguments';
//       test.shouldThrowErrorSync( () => long.longMakeFilling( [ 1, 2, 3 ], 4, 4, 'extra' ) );
//
//       test.case = 'wrong type of type argument';
//       test.shouldThrowErrorSync( () => long.longMakeFilling( {}, 2, 2 ) );
//       test.shouldThrowErrorSync( () => long.longMakeFilling( undefined, 2, 2 ) );
//
//       test.case = 'wrong type of value';
//       test.shouldThrowErrorSync( () => long.longMakeFilling( undefined, 1 ) );
//       test.shouldThrowErrorSync( () => long.longMakeFilling( [], undefined, 1 ) );
//
//       test.case = 'wrong type of length';
//       test.shouldThrowErrorSync( () => long.longMakeFilling( [ 1, 2, 3 ], 'wrong' ) );
//       test.shouldThrowErrorSync( () => long.longMakeFilling( [ 1, 2, 3 ], 2, undefined ) );
//     }
//   }
// }
//
// longMakeFillingWithArrayAndUnrollLongDescriptor.timeOut = 15000;
//
// //
//
// function longMakeFillingWithArgumentsArrayLongDescriptor( test )
// {
//   let times = 4;
//   for( let e in _.LongDescriptors )
//   {
//     let name = _.LongDescriptors[ e ].name;
//     let long = _.withLong[ name ];
//
//     test.open( `long - ${ name }` );
//     testRun( long );
//     test.close( `long - ${ name }` );
//
//     if( times < 1 )
//     break;
//     times--;
//   }
//
//   /* test subroutine */
//
//   function testRun( long )
//   {
//     test.case = 'value - null, length - number';
//     var got = long.longMakeFilling( null, 5 );
//     var expected = long.long.default.make( [ null, null, null, null, null ] );
//     test.identical( got, expected );
//
//     test.case = `value - zero, length - ArgumentsArray`;
//     var got = long.longMakeFilling( 0, _.argumentsArray.make( 5 ) );
//     var expected = long.long.default.make( [ 0, 0, 0, 0, 0 ] );
//     test.identical( got, expected );
//
//     /* */
//
//     test.case = 'type - null, value - string, length - number';
//     var got = long.longMakeFilling( null, 'str', 5 );
//     var expected = long.long.default.make( [ 'str', 'str', 'str', 'str', 'str' ] );
//     test.identical( got, expected );
//
//     test.case = 'type - null, value - string, length - ArgumentsArray';
//     var got = long.longMakeFilling( null, 'str', _.argumentsArray.make( 5 ) );
//     var expected = long.long.default.make( [ 'str', 'str', 'str', 'str', 'str' ] );
//     test.identical( got, expected );
//
//     test.case = `type - ArgumentsArray instance, value - map, length - number`;
//     var got = long.longMakeFilling( _.argumentsArray.make( 0 ), { a : 1 }, 3 );
//     var expected = long.long.default.make( [ { a : 1 }, { a : 1 }, { a : 1 } ] );
//     test.identical( got, expected );
//
//     test.case = `type - ArgumentsArray instance, value - map, length - ArgumentsArray`;
//     var got = long.longMakeFilling( _.argumentsArray.make( 0 ), { a : 1 }, _.argumentsArray.make( 3 ) );
//     var expected = long.long.default.make( [ { a : 1 }, { a : 1 }, { a : 1 } ] );
//     test.identical( got, expected );
//
//     test.case = `type - U8x, value - number, length - number`;
//     var got = long.longMakeFilling( U8x, 10, 3 );
//     var expected = new U8x( [ 10, 10, 10 ] );
//     test.identical( got, expected );
//
//     test.case = `type - I16x, value - number, length - ArgumentsArray`;
//     var got = long.longMakeFilling( I16x, 10, _.argumentsArray.make( 3 ) );
//     var expected = new I16x( [ 10, 10, 10 ] );
//     test.identical( got, expected );
//
//     test.case = `type - F32x instance, value - number, length - number`;
//     var got = long.longMakeFilling( new F32x( 10 ), 10, 3 );
//     var expected = new F32x( [ 10, 10, 10 ] );
//     test.identical( got, expected );
//
//     test.case = `type - F32x instance, value - number, length - ArgumentsArray`;
//     var got = long.longMakeFilling( new F32x( 10 ), 10, _.argumentsArray.make( 3 ) );
//     var expected = new F32x( [ 10, 10, 10 ] );
//     test.identical( got, expected );
//
//     /* */
//
//     if( Config.debug )
//     {
//       test.case = `type - ArgumentsArray constructor, value - array, length - number`;
//       test.shouldThrowErrorSync( () => long.longMakeFilling( _.argumentsArray.make( 0 ).constructor, [ 1 ], 3 ) );
//     }
//   }
// }
//
// //
//
// function longMakeFillingWithBufferTypedLongDescriptor( test )
// {
//   var list =
//   [
//     I8x,
//     U16x,
//     U16x,
//     F32x,
//   ];
//
//   /* tests */
//
//   let times = 4;
//   for( let e in _.LongDescriptors )
//   {
//     let name = _.LongDescriptors[ e ].name;
//     let long = _.withLong[ name ];
//
//     for( let i = 0; i < list.length; i++ )
//     {
//       test.open( `long - ${ name }, long - ${ list[ i ].name }` );
//       testRun( long, list[ i ] );
//       test.close( `long - ${ name }, long - ${ list[ i ].name }` );
//     }
//
//     if( times < 1 )
//     break;
//     times--;
//   }
//
//   /* test subroutine */
//
//   function testRun( long, makeLong )
//   {
//     test.case = 'value - null, length - number';
//     var got = long.longMakeFilling( null, 5 );
//     var expected = long.long.default.make( [ null, null, null, null, null ] );
//     test.identical( got, expected );
//
//     test.case = `value - zero, length - ${ makeLong.name }`;
//     var got = long.longMakeFilling( 0, new makeLong( 5 ) );
//     var expected = new makeLong( [ 0, 0, 0, 0, 0 ] );
//     test.identical( got, expected );
//
//     /* */
//
//     test.case = 'type - null, value - string, length - number';
//     var got = long.longMakeFilling( null, 'str', 5 );
//     var expected = long.long.default.make( [ 'str', 'str', 'str', 'str', 'str' ] );
//     test.identical( got, expected );
//
//     test.case = 'type - null, value - string, length - BufferTyped';
//     var got = long.longMakeFilling( null, 'str', new U8x( 5 ) );
//     var expected = long.long.default.make( [ 'str', 'str', 'str', 'str', 'str' ] );
//     test.identical( got, expected );
//
//     test.case = `type - ${ makeLong.name } constructor, value - array, length - number`;
//     var got = long.longMakeFilling( new makeLong, [ 1 ], 3 );
//     var expected = new makeLong( [ [ 1 ], [ 1 ], [ 1 ] ] );
//     test.identical( got, expected );
//
//     test.case = `type - ${ makeLong.name } constructor, value - array, length - empty ${ makeLong.name }`;
//     var got = long.longMakeFilling( new makeLong, [ 1 ], new makeLong( 0 ) );
//     var expected = new makeLong( [] );
//     test.identical( got, expected );
//
//     test.case = `type - ${ makeLong.name } instance, value - map, length - number`;
//     var got = long.longMakeFilling( new makeLong( 0 ), { a : 1 }, 3 );
//     var expected = new makeLong( [ { a : 1 }, { a : 1 }, { a : 1 } ] );
//     test.identical( got, expected );
//
//     test.case = `type - ${ makeLong.name } instance, value - map, length - ${ makeLong.name }`;
//     var got = long.longMakeFilling( new makeLong( 0 ), { a : 1 }, new makeLong( 3 ) );
//     var expected = new makeLong( [ { a : 1 }, { a : 1 }, { a : 1 } ] );
//     test.identical( got, expected );
//
//     test.case = `type - Array, value - number, length - number`;
//     var got = long.longMakeFilling( Array, 10, 3 );
//     var expected = new Array( 10, 10, 10 );
//     test.identical( got, expected );
//
//     test.case = `type - Array, value - number, length - ${ makeLong.name }`;
//     var got = long.longMakeFilling( Array, 10, new makeLong( 3 ) );
//     var expected = new Array( 10, 10, 10 );
//     test.identical( got, expected );
//
//     test.case = `type - Array instance, value - number, length - number`;
//     var got = long.longMakeFilling( new Array( 10 ), 10, 3 );
//     var expected = new Array( 10, 10, 10 );
//     test.identical( got, expected );
//
//     test.case = `type - Array instance, value - number, length - ${ makeLong.name }`;
//     var got = long.longMakeFilling( new Array( 10 ), 10, new makeLong( 3 ) );
//     var expected = new Array( 10, 10, 10 );
//     test.identical( got, expected );
//   }
// }
//
// longMakeFillingWithBufferTypedLongDescriptor.timeOut = 15000;

//

function fromCommon( test )
{
  act({ tools : 'default', type : 'Array' });
  act({ tools : 'Array', type : 'Array' });
  act({ tools : 'F32x', type : 'F32x' });
  act({ tools : 'bufferTyped', type : 'Array' });

  /* */

  function act( env )
  {
    test.open( `${ __.entity.exportStringSolo( env ) }` );

    const long = namespaceGet( env );
    const Constructor = defaultConstructorGet( env );

    /* */

    test.case = 'null';
    var got = long.from( null );
    test.identical( got, Constructor.from( [] ) );
    test.true( got instanceof Constructor );

    test.case = 'number';
    var src = 2;
    var got = long.from( src );
    var value = _.buffer.typedIs( Constructor.from( [] ) ) ? 0 : undefined;
    test.identical( got, Constructor.from([ value, value ]) );
    test.true( got instanceof Constructor );

    /* */

    test.case = 'empty array';
    var src = [];
    var got = long.from( src );
    if( env.tools === 'bufferTyped' )
    {
      test.identical( got, Constructor.from( [] ) );
      test.true( got instanceof Constructor );
      test.true( got !== src );
    }
    else
    {
      test.identical( got, [] );
      test.true( _.array.is( got ) );
      test.true( got === src );
    }


    test.case = 'filled array';
    var src = [ 1, 2, 3 ];
    var got = long.from( src );
    if( env.tools === 'bufferTyped' )
    {
      test.identical( got, Constructor.from([ 1, 2, 3 ]) );
      test.true( got instanceof Constructor );
      test.true( got !== src );
    }
    else
    {
      test.identical( got, [ 1, 2, 3 ] );
      test.true( _.array.is( got ) );
      test.true( got === src );
    }

    /* */

    test.case = 'empty unroll';
    var src = _.unroll.make( [] );
    var got = long.from( src );
    if( env.tools === 'bufferTyped' )
    {
      test.identical( got, Constructor.from( [] ) );
      test.true( got instanceof Constructor );
      test.true( got !== src );
    }
    else
    {
      test.identical( got, [] );
      test.true( _.unroll.is( got ) );
      test.true( got === src );
    }

    test.case = 'filled unroll';
    var src = _.unroll.make([ 1, 2, 3 ]);
    var got = long.from( src );
    if( env.tools === 'bufferTyped' )
    {
      test.identical( got, Constructor.from([ 1, 2, 3 ]) );
      test.true( got instanceof Constructor );
      test.true( got !== src );
    }
    else
    {
      test.identical( got, [ 1, 2, 3 ] );
      test.true( _.unroll.is( got ) );
      test.true( got === src );
    }

    /* */

    test.case = 'empty argumentsArray';
    var src = _.argumentsArray.make( [] );
    var got = long.from( src );
    if( env.tools === 'bufferTyped' )
    {
      test.identical( got, Constructor.from( [] ) );
      test.true( got instanceof Constructor );
      test.true( got !== src );
    }
    else
    {
      test.identical( got, _.argumentsArray.make( [] ) );
      test.true( _.argumentsArray.is( got ) );
      test.true( got === src );
    }

    test.case = 'filled argumentsArray';
    var src = _.argumentsArray.make([ 1, 2, 3 ]);
    var got = long.from( src );
    if( env.tools === 'bufferTyped' )
    {
      test.identical( got, Constructor.from([ 1, 2, 3 ]) );
      test.true( got instanceof Constructor );
      test.true( got !== src );
    }
    else
    {
      test.identical( got, _.argumentsArray.make([ 1, 2, 3 ]) );
      test.true( _.argumentsArray.is( got ) );
      test.true( got === src );
    }

    /* */

    test.case = 'empty BufferTyped';
    var src = new U8x( [] );
    var got = long.from( src );
    test.identical( got, new U8x( [] ) );
    test.true( _.u8x.is( got ) );
    test.true( got === src );

    var src = new I16x( [] );
    var got = long.from( src );
    test.identical( got, new I16x( [] ) );
    test.true( _.i16x.is( got ) );
    test.true( got === src );

    test.case = 'filled BufferTyped';
    var src = new F32x([ 1, 2, 3 ]);
    var got = long.from( src );
    test.identical( got, new F32x([ 1, 2, 3 ]) );
    test.true( _.f32x.is( got ) );
    test.true( got === src );

    var src = new F64x([ 1, 2, 3 ]);
    var got = long.from( src );
    test.identical( got, new F64x([ 1, 2, 3 ]) );
    test.true( _.f64x.is( got ) );
    test.true( got === src );

    /* */

    test.case = `countable`;
    var src = __.diagnostic.objectMake({ elements : [ 2, 3 ], countable : 1 });
    var got = long.from( src );
    test.true( got instanceof Constructor );
    test.identical( got, Constructor.from([ 2, 3 ]) );

    /* - */

    if( Config.debug )
    {
      test.case = 'without arguments';
      test.shouldThrowErrorSync( () => long.from() );

      test.case = 'extra arguments';
      test.shouldThrowErrorSync( () => long.from( 1, [] ) );

      test.case = 'wrong type of src';
      test.shouldThrowErrorSync( () => long.from( 'str' ) );
      test.shouldThrowErrorSync( () => long.from( { 1 : 2 } ) );
    }

    test.close( `${ __.entity.exportStringSolo( env ) }` );
  }

  /* */

  function namespaceGet( env )
  {
    if( env.tools === 'default' )
    return _.long;
    if( env.tools === 'bufferTyped' )
    return _.bufferTyped;
    return _.withLong[ env.tools ].long;
  }

  /* */

  function defaultConstructorGet( env )
  {
    if( env.tools === 'default' )
    return _.long.default.InstanceConstructor;
    if( env.tools === 'bufferTyped' )
    return _.bufferTyped.default.InstanceConstructor;
    return _.withLong[ env.tools ].long.default.InstanceConstructor;
  }
}

//

function fromCommonWithLongDescriptor( test )
{
  let times = 4;
  for( let k in _.long.namespaces )
  {
    let namespace = _.long.namespaces[ k ];
    let type = namespace.TypeName;

    if( type === 'ArgumentsArray' )
    continue;

    test.open( `long - ${ type }` );
    act({ tools : 'default', type });
    act({ tools : 'Array', type });
    act({ tools : 'F32x', type });
    act({ tools : 'bufferTyped', type });
    test.close( `long - ${ type }` );

    if( times < 1 )
    break;
    times--;
  }

  /* */

  function act( env )
  {
    test.open( `${ __.entity.exportStringSolo( env ) }` );

    const long = namespaceGet( env );
    const Constructor = defaultConstructorGet( env );

    /* */

    test.case = 'null';
    var got = long.from( null );
    test.identical( got, Constructor.from( [] ) );
    test.true( got instanceof Constructor );

    test.case = 'number';
    var src = 2;
    var got = long.from( src );
    var value = _.buffer.typedIs( Constructor.from( [] ) ) ? 0 : undefined;
    test.identical( got, Constructor.from([ value, value ]) );
    test.true( got instanceof Constructor );

    /* */

    test.case = 'empty array';
    var src = [];
    var got = long.from( src );
    if( env.tools === 'bufferTyped' )
    {
      test.identical( got, Constructor.from( [] ) );
      test.true( got instanceof Constructor );
      test.true( got !== src );
    }
    else
    {
      test.identical( got, [] );
      test.true( _.array.is( got ) );
      test.true( got === src );
    }


    test.case = 'filled array';
    var src = [ 1, 2, 3 ];
    var got = long.from( src );
    if( env.tools === 'bufferTyped' )
    {
      test.identical( got, Constructor.from([ 1, 2, 3 ]) );
      test.true( got instanceof Constructor );
      test.true( got !== src );
    }
    else
    {
      test.identical( got, [ 1, 2, 3 ] );
      test.true( _.array.is( got ) );
      test.true( got === src );
    }

    /* */

    test.case = 'empty unroll';
    var src = _.unroll.make( [] );
    var got = long.from( src );
    if( env.tools === 'bufferTyped' )
    {
      test.identical( got, Constructor.from( [] ) );
      test.true( got instanceof Constructor );
      test.true( got !== src );
    }
    else
    {
      test.identical( got, [] );
      test.true( _.unroll.is( got ) );
      test.true( got === src );
    }

    test.case = 'filled unroll';
    var src = _.unroll.make([ 1, 2, 3 ]);
    var got = long.from( src );
    if( env.tools === 'bufferTyped' )
    {
      test.identical( got, Constructor.from([ 1, 2, 3 ]) );
      test.true( got instanceof Constructor );
      test.true( got !== src );
    }
    else
    {
      test.identical( got, [ 1, 2, 3 ] );
      test.true( _.unroll.is( got ) );
      test.true( got === src );
    }

    /* */

    test.case = 'empty argumentsArray';
    var src = _.argumentsArray.make( [] );
    var got = long.from( src );
    if( env.tools === 'bufferTyped' )
    {
      test.identical( got, Constructor.from( [] ) );
      test.true( got instanceof Constructor );
      test.true( got !== src );
    }
    else
    {
      test.identical( got, _.argumentsArray.make( [] ) );
      test.true( _.argumentsArray.is( got ) );
      test.true( got === src );
    }

    test.case = 'filled argumentsArray';
    var src = _.argumentsArray.make([ 1, 2, 3 ]);
    var got = long.from( src );
    if( env.tools === 'bufferTyped' )
    {
      test.identical( got, Constructor.from([ 1, 2, 3 ]) );
      test.true( got instanceof Constructor );
      test.true( got !== src );
    }
    else
    {
      test.identical( got, _.argumentsArray.make([ 1, 2, 3 ]) );
      test.true( _.argumentsArray.is( got ) );
      test.true( got === src );
    }

    /* */

    test.case = 'empty BufferTyped';
    var src = new U8x( [] );
    var got = long.from( src );
    test.identical( got, new U8x( [] ) );
    test.true( _.u8x.is( got ) );
    test.true( got === src );

    var src = new I16x( [] );
    var got = long.from( src );
    test.identical( got, new I16x( [] ) );
    test.true( _.i16x.is( got ) );
    test.true( got === src );

    test.case = 'filled BufferTyped';
    var src = new F32x([ 1, 2, 3 ]);
    var got = long.from( src );
    test.identical( got, new F32x([ 1, 2, 3 ]) );
    test.true( _.f32x.is( got ) );
    test.true( got === src );

    var src = new F64x([ 1, 2, 3 ]);
    var got = long.from( src );
    test.identical( got, new F64x([ 1, 2, 3 ]) );
    test.true( _.f64x.is( got ) );
    test.true( got === src );

    /* */

    test.case = `countable`;
    var src = __.diagnostic.objectMake({ elements : [ 2, 3 ], countable : 1 });
    var got = long.from( src );
    test.true( got instanceof Constructor );
    test.identical( got, Constructor.from([ 2, 3 ]) );

    test.close( `${ __.entity.exportStringSolo( env ) }` );
  }

  /* */

  function namespaceGet( env )
  {
    if( env.tools === 'default' )
    return _.long;
    if( env.tools === 'bufferTyped' )
    return _.bufferTyped;
    return _.withLong[ env.type ].long;
  }

  /* */

  function defaultConstructorGet( env )
  {
    if( env.tools === 'default' )
    return _.long.default.InstanceConstructor;
    if( env.tools === 'bufferTyped' )
    return _.bufferTyped.default.InstanceConstructor;
    return _.withLong[ env.type ].long.default.InstanceConstructor;
  }
}

//

// function from( test )
// {
//   test.case = 'null';
//   var src = null;
//   var got = _.long.from( src );
//   test.identical( got, [] );
//   test.true( _.arrayIs( got ) );
//
//   test.case = 'number';
//   var src = 2;
//   var got = _.long.from( src );
//   test.identical( got, [ undefined, undefined ] );
//   test.true( _.arrayIs( got ) );
//
//   test.case = 'empty array';
//   var src = [];
//   var got = _.long.from( src );
//   test.identical( got, [] );
//   test.true( _.arrayIs( got ) );
//   test.true( got === src );
//
//   test.case = 'filled array';
//   var src = [ 1, '', 'abc', undefined, null, false, true, 0 ];
//   var got = _.long.from( src );
//   test.identical( got, [ 1, '', 'abc', undefined, null, false, true, 0 ] );
//   test.true( _.arrayIs( got ) );
//   test.true( got === src );
//
//   test.case = 'empty unroll';
//   var src = _.unroll.make( [] );
//   var got = _.long.from( src );
//   test.identical( got, [] );
//   test.true( _.arrayIs( got ) );
//   test.true( got === src );
//
//   test.case = 'filled unroll';
//   var src = _.unroll.make( [ 1, '', 'abc', undefined, null, false, true, 0 ] );
//   var got = _.long.from( src );
//   test.identical( got, [ 1, '', 'abc', undefined, null, false, true, 0 ] );
//   test.true( _.arrayIs( got ) );
//   test.true( got === src );
//
//   test.case = 'empty argumentsArray';
//   var src = _.argumentsArray.make( [] );
//   var got = _.long.from( src );
//   test.identical( got, _.argumentsArray.make( [] ) );
//   test.true( _.argumentsArray.is( got ) );
//   test.true( got === src );
//
//   test.case = 'filled argumentsArray';
//   var src = _.argumentsArray.make( [ 1, '', 'abc', undefined, null, false, true, 0 ] );
//   var got = _.long.from( src );
//   test.identical( got, _.argumentsArray.make( [ 1, '', 'abc', undefined, null, false, true, 0 ] ) );
//   test.true( _.argumentsArray.is( got ) );
//   test.true( got === src );
//
//   test.case = 'empty BufferTyped';
//   var src = new U8x( [] );
//   var got = _.long.from( src );
//   test.identical( got, new U8x( [] ) );
//   test.true( _.buffer.typedIs( got ) );
//   test.true( got === src );
//
//   var src = new I16x( [] );
//   var got = _.long.from( src );
//   test.identical( got, new I16x( [] ) );
//   test.true( _.buffer.typedIs( got ) );
//   test.true( got === src );
//
//   test.case = 'filled BufferTyped';
//   var src = new F32x( [ 1, 2, 3, 4, 0 ] );
//   var got = _.long.from( src );
//   test.identical( got, new F32x([ 1, 2, 3, 4, 0 ]) );
//   test.true( _.buffer.typedIs( got ) );
//   test.true( got === src );
//
//   var src = new F64x( [ 1, 2, 3, 4, 0 ] );
//   var got = _.long.from( src );
//   test.identical( got, new F64x([ 1, 2, 3, 4, 0 ]) );
//   test.true( _.buffer.typedIs( got ) );
//   test.true( got === src );
//
//   /* - */
//
//   if( !Config.debug )
//   return;
//
//   test.case = 'without arguments';
//   test.shouldThrowErrorSync( () => _.long.from() );
//
//   test.case = 'extra arguments';
//   test.shouldThrowErrorSync( () => _.long.from( 1, [] ) );
//
//   test.case = 'wrong type of src';
//   test.shouldThrowErrorSync( () => _.long.from( 'str' ) );
//   test.shouldThrowErrorSync( () => _.long.from( { 1 : 2 } ) );
// }
//
// //
//
// function fromWithLongDescriptor( test )
// {
//   let times = 4;
//   for( let k in _.long.namespaces )
//   {
//     let namespace = _.long.namespaces[ k ];
//     let name = namespace.TypeName;
//     let long = _.withLong[ name ];
//
//     /* aaa2 : for Dmytro : remove this if, please. or cover ArgumentsArray in separate routine */ /* Dmytro : kovered */
//     // if( name === 'ArgumentsArray' )
//     // continue;
//
//     test.open( `long - ${ name }` );
//     testRun( long );
//     test.close( `long - ${ name }` );
//
//     if( times < 1 )
//     break;
//     times--;
//   }
//
//   /* - */
//
//   function testRun( long )
//   {
//     test.case = 'null';
//     var src = null;
//     var got = long.long.from( src );
//     var exp = long.long.default.make( 0 );
//     test.identical( got, exp );
//     if( !_.argumentsArray.is( exp ) )
//     test.true( got instanceof long.long.default.InstanceConstructor );
//
//     test.case = 'number';
//     var src = 2;
//     var got = long.long.from( src );
//     var exp = long.long.default.make( 2 );
//     test.identical( got, exp );
//     if( !_.argumentsArray.is( exp ) )
//     test.true( got instanceof long.long.default.InstanceConstructor );
//
//     test.case = 'empty array';
//     var src = [];
//     var got = long.long.from( src );
//     var exp = [];
//     test.identical( got, exp );
//     test.true( src === got );
//
//     test.case = 'filled array';
//     var src = [ 1, 2, 3, 4, 0 ];
//     var got = long.long.from( src );
//     var exp = [ 1, 2, 3, 4, 0 ];
//     test.identical( got, exp );
//     test.true( src === got );
//
//     test.case = 'empty unroll';
//     var src = _.unroll.make( [] );
//     var got = long.long.from( src );
//     var exp = _.unroll.make( [] );
//     test.identical( got, exp );
//     test.true( src === got );
//
//     test.case = 'filled unroll';
//     var src = _.unroll.make( [ 1, 2, 3, 4, 0 ] );
//     var got = long.long.from( src );
//     var exp = _.unroll.make( [ 1, 2, 3, 4, 0 ] );
//     test.identical( got, exp );
//     test.true( src === got );
//     // test.true( src === got ); /* aaa : for Dmytro : add similar check to other cases */ /* Dmytro : added modified checks, longs do not change its type if constructor of src and long descriptor are identical */
//
//     test.case = 'empty argumentsArray';
//     var src = _.argumentsArray.make( [] );
//     var got = long.long.from( src );
//     var exp = _.argumentsArray.make( [] );
//     test.identical( got, exp );
//     test.true( src === got );
//
//     test.case = 'filled argumentsArray';
//     var src = _.argumentsArray.make( [ 1, 2, 3 ] );
//     var got = long.long.from( src );
//     var exp = _.argumentsArray.make( [ 1, 2, 3 ] );
//     test.identical( got, exp );
//     test.true( src === got );
//
//     test.case = 'empty BufferTyped';
//     var src = new U8x( [] );
//     var got = long.long.from( src );
//     var exp = new U8x( [] );
//     test.identical( got, exp );
//     test.true( src === got );
//
//     var src = new I16x( [] );
//     var got = long.long.from( src );
//     var exp = new I16x( [] );
//     test.identical( got, exp );
//     test.true( src === got );
//
//     test.case = 'filled BufferTyped';
//     var src = new F32x( [ 1, 2, 3, 4, 0 ] );
//     var got = long.long.from( src );
//     var exp = new F32x( [ 1, 2, 3, 4, 0 ] );
//     test.identical( got, exp );
//     test.true( src === got );
//
//     var src = new F64x( [ 1, 2, 3, 4, 0 ] );
//     var got = long.long.from( src );
//     var exp = new F64x( [ 1, 2, 3, 4, 0 ] );
//     test.identical( got, exp );
//     test.true( src === got );
//
//     /* - */
//
//     if( Config.debug )
//     {
//       test.case = 'without arguments';
//       test.shouldThrowErrorSync( () => long.long.from() );
//
//       test.case = 'extra arguments';
//       test.shouldThrowErrorSync( () => long.long.from( 1, [] ) );
//
//       test.case = 'wrong type of src';
//       test.shouldThrowErrorSync( () => long.long.from( undefined ) );
//       test.shouldThrowErrorSync( () => long.long.from( 'str' ) );
//       test.shouldThrowErrorSync( () => long.long.from( { 1 : 2 } ) );
//     }
//   }
// }

//

function longSlice( test )
{

  test.open( 'Array' );
  runFor( makeArray );
  test.close( 'Array' );

  /* - */

  test.open( 'ArgumentsArray' );
  runFor( makeU8 );
  test.close( 'ArgumentsArray' );

  /* - */

  test.open( 'F32x' );
  runFor( makeF32 );
  test.close( 'F32x' );

  /* - */

  test.open( 'U8x' );
  runFor( makeU8 );
  test.close( 'U8x' );

  /* instance makers */

  function makeArray( src )
  {
    return _.array.make( src );
  }

  function makeArgumentsArray( src )
  {
    return arguments;
  }

  function makeF32( src )
  {
    var result = new F32x( src );
    return result;
  }

  function makeU8( src )
  {
    var result = new U8x( src );
    return result;
  }

  /* test routine */

  function runFor( makeLong )
  {
    test.open( 'src - empty long' );

    test.case = 'not start and end';
    var srcLong = makeLong();
    var got = _.longSlice( srcLong );
    var expected = makeLong();
    test.identical( got, expected );
    test.true( srcLong !== got );

    test.case = 'start - 0';
    var srcLong = makeLong();
    var got = _.longSlice( srcLong, 0 );
    var expected = makeLong();
    test.identical( got, expected );
    test.true( srcLong !== got );

    test.case = 'start - 0, end - 0';
    var srcLong = makeLong();
    var got = _.longSlice( srcLong, 0, 0 );
    var expected = makeLong();
    test.identical( got, expected );
    test.true( srcLong !== got );

    test.case = 'start - 0, end > src.length';
    var srcLong = makeLong();
    var got = _.longSlice( srcLong, 0, 5 );
    var expected = makeLong();
    test.identical( got, expected );
    test.true( srcLong !== got );

    test.case = 'start - 0, end - -1';
    var srcLong = makeLong();
    var got = _.longSlice( srcLong, 0, -1 );
    var expected = makeLong();
    test.identical( got, expected );
    test.true( srcLong !== got );

    /* */

    test.case = 'start > 0';
    var srcLong = makeLong();
    var got = _.longSlice( srcLong, 2 );
    var expected = makeLong();
    test.identical( got, expected );
    test.true( srcLong !== got );

    test.case = 'start > 0, end === start';
    var srcLong = makeLong();
    var got = _.longSlice( srcLong, 2, 2 );
    var expected = makeLong();
    test.identical( got, expected );
    test.true( srcLong !== got );

    test.case = 'start > 0, end > src.length';
    var srcLong = makeLong();
    var got = _.longSlice( srcLong, 2, 5 );
    var expected = makeLong();
    test.identical( got, expected );
    test.true( srcLong !== got );

    test.case = 'start > 0, end < start';
    var srcLong = makeLong();
    var got = _.longSlice( srcLong, 2, -1 );
    var expected = makeLong();
    test.identical( got, expected );
    test.true( srcLong !== got );

    /* */

    test.case = 'start < 0';
    var srcLong = makeLong();
    var got = _.longSlice( srcLong, -2 );
    var expected = makeLong();
    test.identical( got, expected );
    test.true( srcLong !== got );

    test.case = 'start < 0, end === start';
    var srcLong = makeLong();
    var got = _.longSlice( srcLong, -2, -2 );
    var expected = makeLong();
    test.identical( got, expected );
    test.true( srcLong !== got );

    test.case = 'start < 0, end > src.length';
    var srcLong = makeLong();
    var got = _.longSlice( srcLong, -2, 5 );
    var expected = makeLong();
    test.identical( got, expected );
    test.true( srcLong !== got );

    test.case = 'start < 0, end < start';
    var srcLong = makeLong();
    var got = _.longSlice( srcLong, -2, -4 );
    var expected = makeLong();
    test.identical( got, expected );
    test.true( srcLong !== got );

    test.close( 'src - empty long' );

    /* - */

    test.open( 'src - filled long' );

    test.case = 'not start and end';
    var srcLong = makeLong( [ 1, 2, 3, 4 ] );
    var got = _.longSlice( srcLong );
    var expected = makeLong( [ 1, 2, 3, 4 ] );
    test.identical( got, expected );
    test.true( srcLong !== got );

    test.case = 'start - 0';
    var srcLong = makeLong( [ 1, 2, 3, 4 ] );
    var got = _.longSlice( srcLong, 0 );
    var expected = makeLong( [ 1, 2, 3, 4 ] );
    test.identical( got, expected );
    test.true( srcLong !== got );

    test.case = 'start - undefined, end - 0';
    var srcLong = makeLong( [ 1, 2, 3, 4 ] );
    var got = _.longSlice( srcLong, undefined, 0 );
    var expected = makeLong();
    test.identical( got, expected );
    test.true( srcLong !== got );

    test.case = 'start - 0, end - 0';
    var srcLong = makeLong( [ 1, 2, 3, 4 ] );
    var got = _.longSlice( srcLong, 0, 0 );
    var expected = makeLong();
    test.identical( got, expected );
    test.true( srcLong !== got );

    test.case = 'start - undefined, end < src.length';
    var srcLong = makeLong( [ 1, 2, 3, 4 ] );
    var got = _.longSlice( srcLong, undefined, 3 );
    var expected = makeLong( [ 1, 2, 3 ] );
    test.identical( got, expected );
    test.true( srcLong !== got );

    test.case = 'start - 0, end < src.length';
    var srcLong = makeLong( [ 1, 2, 3, 4 ] );
    var got = _.longSlice( srcLong, 0, 3 );
    var expected = makeLong( [ 1, 2, 3 ] );
    test.identical( got, expected );
    test.true( srcLong !== got );

    test.case = 'start - undefined, end > src.length';
    var srcLong = makeLong( [ 1, 2, 3, 4 ] );
    var got = _.longSlice( srcLong, undefined, 5 );
    var expected = makeLong( [ 1, 2, 3, 4 ] );
    test.identical( got, expected );
    test.true( srcLong !== got );

    test.case = 'start - 0, end > src.length';
    var srcLong = makeLong( [ 1, 2, 3, 4 ] );
    var got = _.longSlice( srcLong, 0, 5 );
    var expected = makeLong( [ 1, 2, 3, 4 ] );
    test.identical( got, expected );
    test.true( srcLong !== got );

    test.case = 'start - undefined, end - -1';
    var srcLong = makeLong( [ 1, 2, 3, 4 ] );
    var got = _.longSlice( srcLong, undefined, -1 );
    var expected = makeLong( [ 1, 2, 3 ] );
    test.identical( got, expected );
    test.true( srcLong !== got );

    test.case = 'start - 0, end - -1';
    var srcLong = makeLong( [ 1, 2, 3, 4 ] );
    var got = _.longSlice( srcLong, 0, -1 );
    var expected = makeLong( [ 1, 2, 3 ] );
    test.identical( got, expected );
    test.true( srcLong !== got );

    /* */

    test.case = '0 < start < src.length';
    var srcLong = makeLong( [ 1, 2, 3, 4 ] );
    var got = _.longSlice( srcLong, 2 );
    var expected = makeLong( [ 3, 4 ] );
    test.identical( got, expected );
    test.true( srcLong !== got );

    test.case = '0 < start > src.length';
    var srcLong = makeLong( [ 1, 2, 3, 4 ] );
    var got = _.longSlice( srcLong, 5 );
    var expected = makeLong();
    test.identical( got, expected );
    test.true( srcLong !== got );

    test.case = '0 < start < src.length, end === start';
    var srcLong = makeLong( [ 1, 2, 3, 4 ] );
    var got = _.longSlice( srcLong, 2, 2 );
    var expected = makeLong();
    test.identical( got, expected );
    test.true( srcLong !== got );

    test.case = '0 < start > src.length, end === start';
    var srcLong = makeLong( [ 1, 2, 3, 4 ] );
    var got = _.longSlice( srcLong, 5, 5 );
    var expected = makeLong();
    test.identical( got, expected );
    test.true( srcLong !== got );

    test.case = '0 < start < src.length, end < src.length';
    var srcLong = makeLong( [ 1, 2, 3, 4 ] );
    var got = _.longSlice( srcLong, 2, 3 );
    var expected = makeLong( [ 3 ] );
    test.identical( got, expected );
    test.true( srcLong !== got );

    test.case = '0 < start > src.length, end < src.length';
    var srcLong = makeLong( [ 1, 2, 3, 4 ] );
    var got = _.longSlice( srcLong, 5, 3 );
    var expected = makeLong();
    test.identical( got, expected );
    test.true( srcLong !== got );

    test.case = '0 < start < src.length, end > src.length';
    var srcLong = makeLong( [ 1, 2, 3, 4 ] );
    var got = _.longSlice( srcLong, 2, 5 );
    var expected = makeLong( [ 3, 4 ] );
    test.identical( got, expected );
    test.true( srcLong !== got );

    test.case = '0 < start > src.length, end > src.length';
    var srcLong = makeLong( [ 1, 2, 3, 4 ] );
    var got = _.longSlice( srcLong, 5, 7 );
    var expected = makeLong();
    test.identical( got, expected );
    test.true( srcLong !== got );

    test.case = '0 < start < src.length, end < start';
    var srcLong = makeLong( [ 1, 2, 3, 4 ] );
    var got = _.longSlice( srcLong, 2, -1 );
    var expected = makeLong( [ 3 ] );
    test.identical( got, expected );
    test.true( srcLong !== got );

    test.case = '0 < start > src.length, end < start';
    var srcLong = makeLong( [ 1, 2, 3, 4 ] );
    var got = _.longSlice( srcLong, 5, -1 );
    var expected = makeLong();
    test.identical( got, expected );
    test.true( srcLong !== got );

    /* */

    test.case = 'start < 0, src.length > |start|';
    var srcLong = makeLong( [ 1, 2, 3, 4 ] );
    var got = _.longSlice( srcLong, -2 );
    var expected = makeLong( [ 3, 4 ] );
    test.identical( got, expected );
    test.true( srcLong !== got );

    test.case = 'start < 0, src.length < |start|';
    var srcLong = makeLong( [ 1, 2, 3, 4 ] );
    var got = _.longSlice( srcLong, -5 );
    var expected = makeLong( [ 1, 2, 3, 4 ] );
    test.identical( got, expected );
    test.true( srcLong !== got );

    test.case = 'start < 0, src.length > |start|, end === start';
    var srcLong = makeLong( [ 1, 2, 3, 4 ] );
    var got = _.longSlice( srcLong, -2, -2 );
    var expected = makeLong();
    test.identical( got, expected );
    test.true( srcLong !== got );

    test.case = 'start < 0, src.length < |start|, end === start';
    var srcLong = makeLong( [ 1, 2, 3, 4 ] );
    var got = _.longSlice( srcLong, -5, -5 );
    var expected = makeLong();
    test.identical( got, expected );
    test.true( srcLong !== got );

    test.case = 'start < 0, src.length > |start|, end < src.length';
    var srcLong = makeLong( [ 1, 2, 3, 4 ] );
    var got = _.longSlice( srcLong, -2, 3 );
    var expected = makeLong( [ 3 ] );
    test.identical( got, expected );
    test.true( srcLong !== got );

    test.case = 'start < 0, src.length < |start|, end < src.length';
    var srcLong = makeLong( [ 1, 2, 3, 4 ] );
    var got = _.longSlice( srcLong, -5, 3 );
    var expected = makeLong( [ 1, 2, 3 ] );
    test.identical( got, expected );
    test.true( srcLong !== got );

    test.case = 'start < 0, src.length > |start|, end > src.length';
    var srcLong = makeLong( [ 1, 2, 3, 4 ] );
    var got = _.longSlice( srcLong, -2, 5 );
    var expected = makeLong( [ 3, 4 ] );
    test.identical( got, expected );
    test.true( srcLong !== got );

    test.case = 'start < 0, src.length < |start|, end > src.length';
    var srcLong = makeLong( [ 1, 2, 3, 4 ] );
    var got = _.longSlice( srcLong, -5, 5 );
    var expected = makeLong( [ 1, 2, 3, 4 ] );
    test.identical( got, expected );
    test.true( srcLong !== got );

    test.case = 'start < 0, src.length > |start|, end < start';
    var srcLong = makeLong( [ 1, 2, 3, 4 ] );
    var got = _.longSlice( srcLong, -2, -4 );
    var expected = makeLong();
    test.identical( got, expected );
    test.true( srcLong !== got );

    test.case = 'start < 0, src.length < |start|, end < start';
    var srcLong = makeLong( [ 1, 2, 3, 4 ] );
    var got = _.longSlice( srcLong, -5, -7 );
    var expected = makeLong();
    test.identical( got, expected );
    test.true( srcLong !== got );

    test.close( 'src - filled long' );

    /* - */

    if( !Config.debug )
    return;

    test.case = 'without arguments';
    test.shouldThrowErrorSync( () => _.longSlice() );

    test.case = 'extra arguments';
    test.shouldThrowErrorSync( () => _.longSlice( [ 1, 2, 3 ], 1, 2, 'extra' ) );

    test.case = 'array is not long';
    test.shouldThrowErrorSync( () => _.longSlice( 'x' ) );
    test.shouldThrowErrorSync( () => _.longSlice( new BufferRaw() ) );

    test.case = 'f is not number';
    test.shouldThrowErrorSync( () => _.longSlice( [ 1 ], 'x', 1 ) );

    test.case = 'l is not number';
    test.shouldThrowErrorSync( () => _.longSlice( [ 1 ], 0, 'x' ) );
  }

}

longSlice.timeOut = 20000;

//

// function identical( test )
// {
//   test.case = 'empty arrays';
//   var got = _.long.identical( [], [] );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'arrays are equal';
//   var got = _.long.identical( [ 1, 2, 3 ], [ 1, 2, 3 ] );
//   var expected = true;
//   test.identical( got, expected );
//
//   test.case = 'array-like arguments';
//   function src1()
//   {
//     return arguments;
//   };
//   function src2()
//   {
//     return arguments;
//   };
//   var got = _.long.identical( src1( 3, 7, 33 ), src2( 3, 7, 13 ) );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'arrays are not equal';
//   var got = _.long.identical( [ 1, 2, 3, 'Hi!' ], [ 1, 2, 3, 'Hello there!' ] );
//   var expected = false;
//   test.identical( got, expected );
//
//   test.case = 'arrays length are not equal';
//   var got = _.long.identical( [ 1, 2, 3 ], [ 1, 2 ] );
//   var expected = false;
//   test.identical( got, expected );
//
//   /**/
//
//   if( !Config.debug )
//   return;
//
//   test.case = 'no arguments';
//   test.shouldThrowErrorSync( function()
//   {
//     _.long.identical();
//   });
//
//   test.case = 'not enough arguments';
//   test.shouldThrowErrorSync( function()
//   {
//     _.long.identical( [ 1, 2, 3 ] );
//   });
//
//   test.case = 'extra argument';
//   test.shouldThrowErrorSync( function()
//   {
//     _.long.identical( [ 1, 2, 3 ], [ 1, 2 ], 'redundant argument' );
//   });
// };

//

function longLeftIndex( test )
{
  test.case = 'nothing';
  var got = _.longLeftIndex( [], 3 );
  var expected = -1;
  test.identical( got, expected );

  test.case = 'second index';
  var got = _.longLeftIndex( [ 1, 2, 3 ], 3 );
  var expected = 2;
  test.identical( got, expected );

  test.case = 'zero index';
  var got = _.longLeftIndex( [ 1, 2, 3 ], 3, ( el, ins ) => el < ins );
  var expected = 0;
  test.identical( got, expected );

  test.case = 'nothing';
  var got = _.longLeftIndex( [ 1, 2, 3 ], 4 );
  var expected = -1;
  test.identical( got, expected );

  test.case = 'nothing';
  var got = _.longLeftIndex( [ 1, 2, 3 ], 3, ( el, ins ) => el > ins );
  var expected = -1;
  test.identical( got, expected );

  test.case = 'array-like arguments';
  function arr()
  {
    return arguments;
  }
  var _arr = arr( 3, 7, 13 );
  var got = _.longLeftIndex( _arr, 13 );
  var expected = 2;
  test.identical( got, expected );

  test.case = 'fromIndex';
  var got = _.longLeftIndex( [ 0, 0, 0, 0 ], 0, 0 );
  var expected = 0;
  test.identical( got, expected );

  test.case = 'fromIndex';
  var got = _.longLeftIndex( [ 0, 0, 0, 0 ], 0, 3 );
  var expected = 3;
  test.identical( got, expected );

  test.case = 'fromIndex';
  var got = _.longLeftIndex( [ 0, 0, 0, 0 ], 0, -1 );
  var expected = 3;
  test.identical( got, expected );

  test.case = 'fromIndex + evaluator';
  var got = _.longLeftIndex( [ 1, 1, 2, 2, 3, 3 ], 3, 2, ( el, ins ) => el < ins );
  var expected = 2;
  test.identical( got, expected );

  test.case = 'fromIndex + evaluator x2';
  var evaluator1 = ( el ) => el + 1;
  var evaluator2 = ( ins ) => ins * 2;
  var got = _.longLeftIndex( [ 6, 6, 5, 5 ], 3, 2, evaluator1, evaluator2 );
  var expected = 2;
  test.identical( got, expected );

  test.case = 'evaluator search first element of array';
  var evaluator = ( e ) => e[ 0 ];
  var got = _.longLeftIndex( [ 1, 2, 3, [ 2 ], 3, [ 4 ] ], [ 2 ], evaluator );
  var expected = 3;
  test.identical( got, expected );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'one argument';
  test.shouldThrowErrorSync( function()
  {
    var got = _.longLeftIndex( [ 1, 2, 3 ] );
  });

  test.case = 'no arguments';
  test.shouldThrowErrorSync( function()
  {
    _.longLeftIndex();
  });

  test.case = 'third argument is wrong';
  test.shouldThrowErrorSync( function()
  {
    _.longLeftIndex( [ 1, 2, 3 ], 2, 'wrong argument' );
  });

};

//

function longRightIndex( test )
{

  test.case = 'nothing';
  var got = _.longRightIndex( [], 3 );
  var expected = -1;
  test.identical( got, expected );

  test.case = 'second index';
  var got = _.longRightIndex( [ 1, 2, 3 ], 3 );
  var expected = 2;
  test.identical( got, expected );

  test.case = 'zero index';
  var got = _.longRightIndex( [ 1, 2, 3 ], 3, function( el, ins ) { return el < ins } );
  var expected = 1;
  test.identical( got, expected );

  test.case = 'nothing';
  var got = _.longRightIndex( [ 1, 2, 3 ], 4 );
  var expected = -1;
  test.identical( got, expected );

  test.case = 'nothing';
  var got = _.longRightIndex( [ 1, 2, 3 ], 3, function( el, ins ) { return el > ins } );
  var expected = -1;
  test.identical( got, expected );

  test.case = 'array-like arguments';
  function arr()
  {
    return arguments;
  }
  var _arr = arr( 3, 7, 13 );
  var got = _.longRightIndex( _arr, 13 );
  var expected = 2;
  test.identical( got, expected );

  test.case = 'fifth index';
  var got = _.longRightIndex( 'abcdef', 'e', function( el, ins ) { return el > ins } );
  var expected = 5;
  test.identical( got, expected );

  test.case = 'third index';
  var got = _.longRightIndex( 'abcdef', 'd' );
  var expected = 3;
  test.identical( got, expected );

  test.case = 'second index';
  var got = _.longRightIndex( 'abcdef', 'c', function( el ) { return el; } );
  var expected = 2;
  test.identical( got, expected );

  test.case = 'fromIndex';
  var got = _.longRightIndex( [ 0, 0, 0, 0 ], 0, 0 );
  var expected = 0;
  test.identical( got, expected );

  test.case = 'fromIndex';
  var got = _.longRightIndex( [ 0, 0, 0, 0 ], 0, 3 );
  var expected = 3;
  test.identical( got, expected );

  test.case = 'fromIndex';
  var got = _.longRightIndex( [ 0, 1, 1, 0 ], 0, 1 );
  var expected = 0;
  test.identical( got, expected );

  test.case = 'fromIndex';
  var got = _.longRightIndex( [ 0, 1, 1, 0 ], 1, 2 );
  var expected = 2;
  test.identical( got, expected );

  test.case = 'fromIndex + evaluator';
  var got = _.longRightIndex( [ 1, 1, 2, 2, 3, 3 ], 3, 4, ( el, ins ) => el < ins );
  var expected = 3;
  test.identical( got, expected );

  test.case = 'fromIndex + evaluator x2';
  var evaluator1 = ( el ) => el + 1;
  var evaluator2 = ( ins ) => ins * 2;
  var got = _.longRightIndex( [ 6, 6, 5, 5 ], 3, 2, evaluator1, evaluator2 );
  var expected = 2;
  test.identical( got, expected );

  /**/

  if( !Config.debug )
  return;

  test.case = 'one argument';
  test.shouldThrowErrorSync( function()
  {
    var got = _.longRightIndex( [ 1, 2, 3 ] );
  });

  test.case = 'no arguments';
  test.shouldThrowErrorSync( function()
  {
    _.longRightIndex();
  });

  test.case = 'third argument is wrong';
  test.shouldThrowErrorSync( function()
  {
    _.longRightIndex( [ 1, 2, 3 ], 2, 'wrong argument' );
  });

};

//

function longLeft( test )
{
  test.case = 'empty array';
  var src = [];
  var got = _.longLeft( src, 1 );
  test.identical( got, { index : -1 } );

  test.case = 'array has not searched element';
  var src = [ 1, 2, 'str', [ 3 ], { a : 2 } ];
  var got = _.longLeft( src, 3 );
  test.identical( got, { index : -1 } );

  test.case = 'array has duplicated searched element';
  var src = [ 1, 2, 3, 'str', [ 3 ], 3, { a : 2 } ];
  var got = _.longLeft( src, 3 );
  test.identical( got, { index : 2, element : 3 } );

  test.case = 'searches complex data without evaluators';
  var src = [ 1, 2, 'str', [ 3 ], { a : 2 } ];
  var got = _.longLeft( src, [ 3 ] );
  test.identical( got, { index : -1 } );

  /* */

  test.case = 'array has not searched element, fromIndex';
  var src = [ 1, 2, 3, 'str', [ 3 ], { a : 2 } ];
  var got = _.longLeft( src, 3, 4 );
  test.identical( got, { index : -1 } );

  test.case = 'array has duplicated searched element, fromIndex';
  var src = [ 1, 2, 3, 'str', [ 3 ], 3, 'str', { a : 2 } ];
  var got = _.longLeft( src, 'str', 4 );
  test.identical( got, { index : 6, element : 'str' } );

  test.case = 'searches complex data, fromIndex';
  var src = [ 1, 2, 'str', [ 3 ], { a : 2 } ];
  var got = _.longLeft( src, [ 3 ], 2 );
  test.identical( got, { index : -1 } );

  /* */

  test.case = 'array has not searched element, onEvaluate1';
  var src = [ 1, 2, 'str', [ 3 ], { a : 2 } ];
  var got = _.longLeft( src, 3, ( e ) => typeof e );
  test.identical( got, { index : 0, element : 1 } );

  test.case = 'array has duplicated searched element, onEvaluate1';
  var src = [ 1, 2, 3, 'str', [ 3 ], 3, 'str', { a : 2 } ];
  var got = _.longLeft( src, 'str', ( e ) => e );
  test.identical( got, { index : 3, element : 'str' } );

  test.case = 'searches complex data, onEvaluate1';
  var src = [ 1, 2, 'str', [ 3 ], { a : 2 } ];
  var got = _.longLeft( src, [ 3 ], ( e ) => e[ 0 ] );
  test.identical( got, { index : 3, element : [ 3 ] } );

  /* */

  test.case = 'array has not searched element, fromIndex, onEvaluate1';
  var src = [ 1, 2, 'str', [ 3 ], { a : 2 } ];
  var got = _.longLeft( src, 3, 2, ( e ) => typeof e );
  test.identical( got, { index : -1 } );

  test.case = 'array has duplicated searched element, fromIndex, onEvaluate1';
  var src = [ 1, 2, 3, 'str', [ 3 ], 3, 'str', { a : 2 } ];
  var got = _.longLeft( src, 'str', 4, ( e ) => typeof e );
  test.identical( got, { index : 6, element : 'str' } );

  test.case = 'searches complex data, onEvaluate1';
  var src = [ 1, 2, 'str', [ 3 ], { a : 2 } ];
  var got = _.longLeft( src, [ 3 ], 4, ( e ) => e[ 0 ] );
  test.identical( got, { index : -1 } );

  /* */

  test.case = 'array has not searched element, onEvaluate1, onEvaluate2';
  var src =[ 1, 2, 'str', [ 3 ], { a : 2 } ];
  var got = _.longLeft( src, 3, ( e ) => e[ 0 ], ( ins ) => ins );
  test.identical( got, { index : 3, element : [ 3 ] } );

  test.case = 'array has duplicated searched element, onEvaluate1, onEvaluate2';
  var src = [ 1, 2, 3, 'str', [ 3 ], 3, 'str', { a : 2 } ];
  var got = _.longLeft( src, 2, ( e ) => e.a, ( ins ) => ins );
  test.identical( got, { index : 7, element : { a : 2 } } );

  test.case = 'searches complex data, onEvaluate, onEvaluate2';
  var src = [ 1, 2, 'str', [ 3 ], { a : 2 } ];
  var got = _.longLeft( src, 3, ( e ) => e[ 0 ], ( ins ) => ins );
  test.identical( got, { index : 3, element : [ 3 ] } );

  /* */

  test.case = 'array has not searched element, fromIndex, onEvaluate1, onEvaluate2';
  var src = [ 1, 2, 'str', [ 3 ], { a : 2 } ];
  var got = _.longLeft( src, 3, 2, ( e ) => e[ 0 ], ( ins ) => ins );
  test.identical( got, { index : 3, element : [ 3 ] } );

  test.case = 'array has duplicated searched element, fromIndex, onEvaluate1, onEvaluate2';
  var src = [ 1, 2, 3, 'str', [ 3 ], 3, 'str', { a : 2 } ];
  var got = _.longLeft( src, 2, 7, ( e ) => e.a, ( ins ) => ins );
  test.identical( got, { index : 7, element : { a : 2 } } );

  test.case = 'searches complex data, fromIndex, onEvaluate, onEvaluate2';
  var src = [ 1, 2, 'str', [ 3 ], { a : 2 } ];
  var got = _.longLeft( src, 3, 4, ( e ) => e[ 0 ], ( ins ) => ins );
  test.identical( got, { index : -1 } );

  /* */

  test.case = 'array has not searched element, equalizer';
  var src = [ 1, 2, 'str', [ 3 ], { a : 2 } ];
  var got = _.longLeft( src, 3, ( e, ins ) => e[ 0 ] === ins );
  test.identical( got, { index : 3, element : [ 3 ] } );

  test.case = 'array has duplicated searched element, equalizer';
  var src = [ 1, 2, 3, 'str', [ 3 ], 3, 'str', { a : 2 } ];
  var got = _.longLeft( src, 2, ( e, ins ) => e.a === ins );
  test.identical( got, { index : 7, element : { a : 2 } } );

  test.case = 'searches complex data, equalizer';
  var src = [ 1, 2, 'str', [ 3 ], { a : 2 } ];
  var got = _.longLeft( src, 3, ( e, ins ) => e[ 0 ] ===  ins );
  test.identical( got, { index : 3, element : [ 3 ] } );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.longLeft() );

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.longLeft( [ 1, 2 ], 1, 0, ( e ) => e, ( ins ) => ins, 'extra' ) );

  test.case = 'fromIndex is not a number';
  test.shouldThrowErrorSync( () => _.longLeft( [ 1, 2 ], 1, 'wrong' ) );

  test.case = 'onEvaluate1 is not a routine, not a number';
  test.shouldThrowErrorSync( () => _.longLeft( [ 1, 2 ], 1, 0, 'wrong' ) );

  test.case = 'onEvaluate1 has wrong length';
  test.shouldThrowErrorSync( () => _.longLeft( [ 1, 2 ], 1, 0, () => 1 ) );

  test.case = 'onEvaluate2 has wrong length';
  test.shouldThrowErrorSync( () => _.longLeft( [ 1, 2 ], 1, 0, ( e ) => e, () => 1 ) );
}

//

function longRight( test )
{
  test.case = 'empty array';
  var src = [];
  var got = _.longRight( src, 1 );
  test.identical( got, { index : -1 } );

  test.case = 'array has not searched element';
  var src = [ 1, 2, 'str', [ 3 ], { a : 2 } ];
  var got = _.longRight( src, 3 );
  test.identical( got, { index : -1 } );

  test.case = 'array has duplicated searched element';
  var src = [ 1, 2, 3, 'str', [ 3 ], 3, { a : 2 } ];
  var got = _.longRight( src, 3 );
  test.identical( got, { index : 5, element : 3 } );

  test.case = 'searches complex data without evaluators';
  var src = [ 1, 2, 'str', [ 3 ], { a : 2 } ];
  var got = _.longRight( src, [ 3 ] );
  test.identical( got, { index : -1 } );

  /* */

  test.case = 'array has not searched element, fromIndex';
  var src = [ 1, 2, 3, 'str', [ 3 ], { a : 2 } ];
  var got = _.longRight( src, 3, 4 );
  test.identical( got, { index : 2, element : 3 } );

  test.case = 'array has duplicated searched element, fromIndex';
  var src = [ 1, 2, 3, 'str', [ 3 ], 3, 'str', { a : 2 } ];
  var got = _.longRight( src, 'str', 4 );
  test.identical( got, { index : 3, element : 'str' } );

  test.case = 'searches complex data, fromIndex';
  var src = [ 1, 2, 'str', [ 3 ], { a : 2 } ];
  var got = _.longRight( src, [ 3 ], 2 );
  test.identical( got, { index : -1 } );

  /* */

  test.case = 'array has not searched element, onEvaluate1';
  var src = [ 1, 2, 'str', [ 3 ], { a : 2 } ];
  var got = _.longRight( src, 3, ( e ) => typeof e );
  test.identical( got, { index : 1, element : 2 } );

  test.case = 'array has duplicated searched element, onEvaluate1';
  var src = [ 1, 2, 3, 'str', [ 3 ], 3, 'str', { a : 2 } ];
  var got = _.longRight( src, 'str', ( e ) => e );
  test.identical( got, { index : 6, element : 'str' } );

  test.case = 'searches complex data, onEvaluate1';
  var src = [ 1, 2, [ 3 ], 'str', [ 3 ], { a : 2 } ];
  var got = _.longRight( src, [ 3 ], ( e ) => e[ 0 ] );
  test.identical( got, { index : 4, element : [ 3 ] } );

  /* */

  test.case = 'array has not searched element, fromIndex, onEvaluate1';
  var src = [ 1, 2, 'str', [ 3 ], { a : 2 } ];
  var got = _.longRight( src, 3, 2, ( e ) => typeof e );
  test.identical( got, { index : 1, element : 2 } );

  test.case = 'array has duplicated searched element, fromIndex, onEvaluate1';
  var src = [ 1, 2, 3, 'str', [ 3 ], 3, 'str', { a : 2 } ];
  var got = _.longRight( src, 'str', 4, ( e ) => typeof e );
  test.identical( got, { index : 3, element : 'str' } );

  test.case = 'searches complex data, onEvaluate1';
  var src = [ 1, 2, 'str', [ 3 ], { a : 2 } ];
  var got = _.longRight( src, [ 3 ], 4, ( e ) => e[ 0 ] );
  test.identical( got, { index : 3, element : [ 3 ] } );

  /* */

  test.case = 'array has not searched element, onEvaluate1, onEvaluate2';
  var src =[ 1, 2, [ 3 ], 'str', [ 3 ], { a : 2 } ];
  var got = _.longRight( src, 3, ( e ) => e[ 0 ], ( ins ) => ins );
  test.identical( got, { index : 4, element : [ 3 ] } );

  test.case = 'array has duplicated searched element, onEvaluate1, onEvaluate2';
  var src = [ 1, 2, 3, { a : 2 }, 'str', [ 3 ], 3, 'str', { a : 2 } ];
  var got = _.longRight( src, 2, ( e ) => e.a, ( ins ) => ins );
  test.identical( got, { index : 8, element : { a : 2 } } );

  test.case = 'searches complex data, onEvaluate, onEvaluate2';
  var src = [ 1, 2, [ 3 ], 'str', [ 3 ], { a : 2 } ];
  var got = _.longRight( src, 3, ( e ) => e[ 0 ], ( ins ) => ins );
  test.identical( got, { index : 4, element : [ 3 ] } );

  /* */

  test.case = 'array has not searched element, fromIndex, onEvaluate1, onEvaluate2';
  var src = [ 1, 2, [ 3 ], 'str', [ 3 ], { a : 2 } ];
  var got = _.longRight( src, 3, 2, ( e ) => e[ 0 ], ( ins ) => ins );
  test.identical( got, { index : 2, element : [ 3 ] } );

  test.case = 'array has duplicated searched element, fromIndex, onEvaluate1, onEvaluate2';
  var src = [ 1, 2, 3, 'str', [ 3 ], 3, 'str', { a : 2 } ];
  var got = _.longRight( src, 2, 7, ( e ) => e.a, ( ins ) => ins );
  test.identical( got, { index : 7, element : { a : 2 } } );

  test.case = 'searches complex data, fromIndex, onEvaluate, onEvaluate2';
  var src = [ 1, 2, [ 3, 4 ], 'str', [ 3 ], { a : 2 } ];
  var got = _.longRight( src, 3, 4, ( e ) => e[ 0 ], ( ins ) => ins );
  test.identical( got, { index : 4, element : [ 3 ] } );

  /* */

  test.case = 'array has not searched element, equalizer';
  var src = [ 1, 2, [ 3 ], 'str', [ 3 ], { a : 2 } ];
  var got = _.longRight( src, 3, ( e, ins ) => e[ 0 ] === ins );
  test.identical( got, { index : 4, element : [ 3 ] } );

  test.case = 'array has duplicated searched element, equalizer';
  var src = [ 1, 2, 3, 'str', [ 3 ], 3, 'str', { a : 2 } ];
  var got = _.longRight( src, 2, ( e, ins ) => e.a === ins );
  test.identical( got, { index : 7, element : { a : 2 } } );

  test.case = 'searches complex data, equalizer';
  var src = [ 1, 2, [ 3 ], 'str', [ 3 ], { a : 2 } ];
  var got = _.longRight( src, 3, ( e, ins ) => e[ 0 ] ===  ins );
  test.identical( got, { index : 4, element : [ 3 ] } );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () => _.longRight() );

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.longRight( [ 1, 2 ], 1, 0, ( e ) => e, ( ins ) => ins, 'extra' ) );

  test.case = 'fromIndex is not a number';
  test.shouldThrowErrorSync( () => _.longRight( [ 1, 2 ], 1, 'wrong' ) );

  test.case = 'onEvaluate1 is not a routine, not a number';
  test.shouldThrowErrorSync( () => _.longRight( [ 1, 2 ], 1, 0, 'wrong' ) );

  test.case = 'onEvaluate1 has wrong length';
  test.shouldThrowErrorSync( () => _.longRight( [ 1, 2 ], 1, 0, () => 1 ) );

  test.case = 'onEvaluate2 has wrong length';
  test.shouldThrowErrorSync( () => _.longRight( [ 1, 2 ], 1, 0, ( e ) => e, () => 1 ) );
}

// --
//
// --

const Proto =
{

  name : 'Tools.Long.l0.l1',
  silencing : 1,
  enabled : 1,
  routineTimeOut : 20000,

  tests :
  {

    // dichotomy

    dichotomy,
    dichotomyPerformance,

    // make

    makeCommon,
    makeCommonWithLongDescriptor,

    makeLongFilledCommon,
    makeLongFilledCommonWithLongDescriptor,

    makeSrcIsNullWithLongNamespaces,

    // longMakeWithArrayAndUnroll, /* Dmytro : full coverage in routines makeCommon* */
    // longMakeWithArgumentsArray,/* Dmytro : full coverage in routines makeCommon* */
    // longMakeWithBufferTyped, /* Dmytro : full coverage in routines makeCommon* */
    /* Dmytro : longDescriptor does not exists anymore. Kos : exists */
    // longMakeWithArrayAndUnrollLongDescriptor, /* aaa2 : for Dmytro : make proper fix */ /* Dmytro : fixed, full coverage in routines makeCommon* */
    // longMakeWithArgumentsArrayLongDescriptor, /* aaa2 : for Dmytro : make proper fix */ /* Dmytro : fixed, full coverage in routines makeCommon* */
    // longMakeWithBufferTypedLongDescriptor, /* aaa2 : for Dmytro : make proper fix */ /* Dmytro : fixed, full coverage in routines makeCommon* */

    makeEmptyCommon,
    makeEmptyCommonWithLongDescriptor,
    // longMakeEmptyWithArrayAndUnroll, /* Dmytro : full coverage in routines makeEmptyCommon* */
    // longMakeEmptyWithArgumentsArray, /* Dmytro : full coverage in routines makeEmptyCommon* */
    // longMakeEmptyWithBufferTyped, /* Dmytro : full coverage in routines makeEmptyCommon* */
    // longMakeEmptyWithArrayAndUnrollLongDescriptor, /* aaa2 : for Dmytro : make proper fix */ /* Dmytro : fixed, full coverage in routines makeEmptyCommon* */
    // longMakeEmptyWithArgumentsArrayLongDescriptor, /* aaa2 : for Dmytro : make proper fix */ /* Dmytro : fixed, full coverage in routines makeEmptyCommon* */
    // longMakeEmptyWithBufferTypedLongDescriptor, /* aaa2 : for Dmytro : make proper fix */ /* Dmytro : fixed, full coverage in routines makeEmptyCommon* */

    // _longMakeOfLengthWithArrayAndUnroll, /* Dmytro : should be removed, routines do not exist and not used */
    // _longMakeOfLengthWithArgumentsArray,
    // _longMakeOfLengthWithBufferTyped,
    // _longMakeOfLengthWithArrayAndUnrollLongDescriptor,
    // _longMakeOfLengthWithArgumentsArrayLongDescriptor,
    // _longMakeOfLengthWithBufferTypedLongDescriptor,

    // longMakeUndefinedWithArrayAndUnroll, /* Dmytro : all coverage in routines makeLongFilledCommon* */
    // longMakeUndefinedWithArgumentsArray, /* Dmytro : all coverage in routines makeLongFilledCommon* */
    // longMakeUndefinedWithBufferTyped, /* Dmytro : all coverage in routines makeLongFilledCommon* */
    // longMakeUndefinedWithArrayAndUnrollLongDescriptor, /* aaa2 : for Dmytro : make proper fix */ /* Dmytro : all coverage in routines makeLongFilledCommon* */
    // longMakeUndefinedWithArgumentsArrayLongDescriptor, /* aaa2 : for Dmytro : make proper fix */ /* Dmytro : all coverage in routines makeLongFilledCommon* */
    // longMakeUndefinedWithBufferTypedLongDescriptor, /* aaa2 : for Dmytro : make proper fix */ /* Dmytro : all coverage in routines makeLongFilledCommon* */

    /* xxx : aaa2 : for Dmytro : try to move this routines on l1 */ /* Dmytro : moved */
    // longMakeZeroedBasic, /* aaa2 : for Dmytro : extend */ /* Dmytro : all coverage in routines makeLongFilledCommon* */
    // longMakeZeroedWithArrayAndUnroll, /* Dmytro : all coverage in routines makeLongFilledCommon* */
    // longMakeZeroedWithArgumentsArray, /* Dmytro : all coverage in routines makeLongFilledCommon* */
    // longMakeZeroedWithBufferTyped, /* aaa2 : for Dmytro : enable */ /* Dmytro : all coverage in routines makeLongFilledCommon* */
    // longMakeZeroedWithArrayAndUnrollLongDescriptor, /* aaa2 : for Dmytro : enable */ /* Dmytro : all coverage in routines makeLongFilledCommon* */
    // longMakeZeroedWithArgumentsArrayLongDescriptor, /* aaa2 : for Dmytro : enable */ /* Dmytro : all coverage in routines makeLongFilledCommon* */
    // longMakeZeroedWithBufferTypedLongDescriptor, /* aaa2 : for Dmytro : enable */ /* Dmytro : all coverage in routines makeLongFilledCommon* */

    makeFillingCommon, /* aaa2 : for Dmytro : enable */ /* Dmytro : wrote, this routine aggregates tests fro routines `longMakeFillingWithArrayAndUnroll` `longMakeFillingWithArgumentsArray`, `longMakeFillingWithBufferTyped` */
    makeFillingCommonWithLongDescriptor,
    // longMakeFillingWithArrayAndUnroll, /* aaa2 : for Dmytro : enable */ /* Dmytro : all test in aggregated routine makeFillingCommon */
    // longMakeFillingWithArgumentsArray, /* aaa2 : for Dmytro : enable */ /* Dmytro : all test in aggregated routine makeFillingCommon */
    // longMakeFillingWithBufferTyped, /* aaa2 : for Dmytro : enable */ /* Dmytro : all test in aggregated routine makeFillingCommon */

    // longMakeFillingWithArrayAndUnrollLongDescriptor, /* aaa2 : for Dmytro : enable */ /* Dmytro : fixed, full coverage in routines makeFillingCommonWithLongDescriptor */
    // longMakeFillingWithArgumentsArrayLongDescriptor, /* aaa2 : for Dmytro : enable */ /* Dmytro : fixed, full coverage in routines makeFillingCommonWithLongDescriptor */
    // longMakeFillingWithBufferTypedLongDescriptor, /* aaa2 : for Dmytro : enable */ /* Dmytro : fixed, full coverage in routines makeFillingCommonWithLongDescriptor */

    //

    fromCommon,
    fromCommonWithLongDescriptor,
    // from, /* Dmytro : wrote test routine fromCommon for namespaces long and bufferTyped */
    // fromWithLongDescriptor, /* aaa2 : for Dmytro : enable */ /* Dmytro : wrote test routine fromCommonWithLongDescriptor for namespaces long and bufferTyped */

    // longSlice,

    // identical,

    longLeftIndex,
    longRightIndex,

    longLeft,
    longRight,

  }

}

const Self = wTestSuite( Proto );
if( typeof module !== 'undefined' && !module.parent )
wTester.test( Self.name );

})();
