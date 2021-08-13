( function _ContainerAdapter_test_s_()
{

'use strict';

// return; // xxx

if( typeof module !== 'undefined' )
{
  const _ = require( 'Tools' );
  _.include( 'wTesting' );
  _.include( 'wSelector' );
}

const _ = _global_.wTools;

// --
// base tests
// --

function is( test )
{
  test.case = 'empty';
  var got = _.containerAdapter.is();
  test.identical( got, false );

  test.case = 'src - null';
  var got = _.containerAdapter.is( null );
  test.identical( got, false );

  test.case = 'src - undefined';
  var got = _.containerAdapter.is( undefined );
  test.identical( got, false );

  test.case = 'src - number';
  var got = _.containerAdapter.is( 1 );
  test.identical( got, false );

  test.case = 'src - string';
  var got = _.containerAdapter.is( 'str' );
  test.identical( got, false );

  test.case = 'src - boolean, true';
  var got = _.containerAdapter.is( true );
  test.identical( got, false );

  test.case = 'src - boolean, false';
  var got = _.containerAdapter.is( false );
  test.identical( got, false );

  test.case = 'src - array';
  var got = _.containerAdapter.is( [ 1, 2 ] );
  test.identical( got, false );

  test.case = 'src - unroll';
  var got = _.containerAdapter.is( _.unroll.make( [ 1, 2 ] ) );
  test.identical( got, false );

  test.case = 'src - argumentsArray';
  var got = _.containerAdapter.is( _.argumentsArray.make( [ 1, 2 ] ) );
  test.identical( got, false );

  test.case = 'src - BufferRaw';
  var got = _.containerAdapter.is( new BufferRaw( 5 ) );
  test.identical( got, false );

  test.case = 'src - BufferTyped';
  var got = _.containerAdapter.is( new U8x( [ 1, 2 ] ) );
  test.identical( got, false );

  test.case = 'src - map';
  var got = _.containerAdapter.is( { a : 0 } );
  test.identical( got, false );

  test.case = 'src - Map';
  var got = _.containerAdapter.is( new HashMap() );
  test.identical( got, false );

  test.case = 'src - Set';
  var got = _.containerAdapter.is( new Set() );
  test.identical( got, false );

  test.case = 'src - Symbol';
  var got = _.containerAdapter.is( Symbol( 'a' ) );
  test.identical( got, false );

  test.case = 'src - instance of constructor';
  function Constr()
  {
    this.x = 1;
    return this;
  }
  var got = _.containerAdapter.is( new Constr() );
  test.identical( got, false );

  /* */

  test.case = 'check instance of ContainerAdapter';
  var src = _.containerAdapter.make( [ 1, 2 ] );
  var got = _.containerAdapter.is( src );
  test.identical( got, true );

  test.case = 'check instance of ContainerAdapter';
  var src = _.containerAdapter.make( new Set( [ 1, 2 ] ) );
  var got = _.containerAdapter.is( src );
  test.identical( got, true );
}

//

function make( test )
{
  test.case = 'from empty array';
  var src = [];
  var exp = [];
  var got = _.containerAdapter.make( src );
  test.true( got !== src );
  test.true( _.arrayIs( got.original ) );
  test.identical( got.original, exp );

  test.case = 'from array';
  var src = [ 1, 2, '', {}, [], null, undefined ];
  var exp = [ 1, 2, '', {}, [], null, undefined ];
  var got = _.containerAdapter.make( src );
  test.true( got !== src );
  test.true( _.arrayIs( got.original ) );
  test.identical( got.original, exp );

  test.case = 'from empty unroll';
  var src = _.unroll.make( [] );
  var exp = [];
  var got = _.containerAdapter.make( src );
  test.true( got !== src );
  test.true( _.arrayIs( got.original ) );
  test.identical( got.original, exp );

  test.case = 'from unroll';
  var src = _.unroll.make( [ 1, 2, '', {}, [], null, undefined ] );
  var exp = [ 1, 2, '', {}, [], null, undefined ];
  var got = _.containerAdapter.make( src );
  test.true( got !== src );
  test.true( _.arrayIs( got.original ) );
  test.identical( got.original, exp );

  test.case = 'from empty Set';
  var src = new Set();
  var exp = [];
  var got = _.containerAdapter.make( src );
  test.true( got !== src );
  test.true( _.set.is( got.original ) );
  test.identical( [ ... got.original ], exp );

  test.case = 'from Set';
  var src = new Set( [ 1, 2, '', {}, [], null, undefined ] );
  var exp = [ 1, 2, '', {}, [], null, undefined ];
  var got = _.containerAdapter.make( src );
  test.true( got !== src );
  test.true( _.set.is( got.original ) );
  test.identical( [ ... got.original ], exp );

  test.case = 'from empty arrayAdapterContainer';
  var src = _.containerAdapter.make( [] );
  var exp = [];
  var got = _.containerAdapter.make( src );
  test.true( got !== src );
  test.true( _.arrayIs( got.original ) );
  test.identical( got.original, exp );

  test.case = 'from arrayAdapterContainer';
  var src = _.containerAdapter.make( [ 1, 2, '', {}, [], null, undefined ] );
  var exp = [ 1, 2, '', {}, [], null, undefined ];
  var got = _.containerAdapter.make( src );
  test.true( got !== src );
  test.true( _.arrayIs( got.original ) );
  test.identical( got.original, exp );

  test.case = 'from empty setAdapterContainer';
  var src = _.containerAdapter.make( new Set( [] ) );
  var exp = [];
  var got = _.containerAdapter.make( src );
  test.true( got !== src );
  test.true( _.set.is( got.original ) );
  test.identical( [ ... got.original ], exp );

  test.case = 'from setAdapterContainer';
  var src = _.containerAdapter.make( new Set( [ 1, 2, '', {}, [], null, undefined ] ) );
  var exp = [ 1, 2, '', {}, [], null, undefined ];
  var got = _.containerAdapter.make( src );
  test.true( got !== src );
  test.true( _.set.is( got.original ) );
  test.identical( [ ... got.original ], exp );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.containerAdapter.make( [ 1, 2 ], [ 1, 2 ] ) );

  test.case = 'container is not an array or a set';
  test.shouldThrowErrorSync( () => _.containerAdapter.make( _.argumentsArray.make( 10 ) ) );
  test.shouldThrowErrorSync( () => _.containerAdapter.make( new U8x( 10 ) ) );
  test.shouldThrowErrorSync( () => _.containerAdapter.make( { a : 0 } ) );
  test.shouldThrowErrorSync( () => _.containerAdapter.make( 'str' ) );
  test.shouldThrowErrorSync( () => _.containerAdapter.make( 0 ) );
  test.shouldThrowErrorSync( () => _.containerAdapter.make( null ) );
  test.shouldThrowErrorSync( () => _.containerAdapter.make( undefined ) );
}

//

function makeByConstructor( test )
{
  test.case = 'from empty array';
  var src = [];
  var exp = [];
  var got = new _.containerAdapter( src );
  test.true( got !== src );
  test.true( _.arrayIs( got.original ) );
  test.identical( got.original, exp );

  test.case = 'from array';
  var src = [ 1, 2, '', {}, [], null, undefined ];
  var exp = [ 1, 2, '', {}, [], null, undefined ];
  var got = new _.containerAdapter( src );
  test.true( got !== src );
  test.true( _.arrayIs( got.original ) );
  test.identical( got.original, exp );

  test.case = 'from empty unroll';
  var src = _.unroll.make( [] );
  var exp = [];
  var got = new _.containerAdapter( src );
  test.true( got !== src );
  test.true( _.arrayIs( got.original ) );
  test.identical( got.original, exp );

  test.case = 'from unroll';
  var src = _.unroll.make( [ 1, 2, '', {}, [], null, undefined ] );
  var exp = [ 1, 2, '', {}, [], null, undefined ];
  var got = new _.containerAdapter( src );
  test.true( got !== src );
  test.true( _.arrayIs( got.original ) );
  test.identical( got.original, exp );

  test.case = 'from empty Set';
  var src = new Set();
  var exp = [];
  var got = new _.containerAdapter( src );
  test.true( got !== src );
  test.true( _.set.is( got.original ) );
  test.identical( [ ... got.original ], exp );

  test.case = 'from Set';
  var src = new Set( [ 1, 2, '', {}, [], null, undefined ] );
  var exp = [ 1, 2, '', {}, [], null, undefined ];
  var got = new _.containerAdapter( src );
  test.true( got !== src );
  test.true( _.set.is( got.original ) );
  test.identical( [ ... got.original ], exp );

  test.case = 'from empty arrayAdapterContainer';
  var src = _.containerAdapter.make( [] );
  var exp = [];
  var got = new _.containerAdapter( src );
  test.true( got !== src );
  test.true( _.arrayIs( got.original ) );
  test.identical( got.original, exp );

  test.case = 'from arrayAdapterContainer';
  var src = _.containerAdapter.make( [ 1, 2, '', {}, [], null, undefined ] );
  var exp = [ 1, 2, '', {}, [], null, undefined ];
  var got = new _.containerAdapter( src );
  test.true( got !== src );
  test.true( _.arrayIs( got.original ) );
  test.identical( got.original, exp );

  test.case = 'from empty setAdapterContainer';
  var src = _.containerAdapter.make( new Set( [] ) );
  var exp = [];
  var got = new _.containerAdapter( src );
  test.true( got !== src );
  test.true( _.set.is( got.original ) );
  test.identical( [ ... got.original ], exp );

  test.case = 'from setAdapterContainer';
  var src = _.containerAdapter.make( new Set( [ 1, 2, '', {}, [], null, undefined ] ) );
  var exp = [ 1, 2, '', {}, [], null, undefined ];
  var got = new _.containerAdapter( src );
  test.true( got !== src );
  test.true( _.set.is( got.original ) );
  test.identical( [ ... got.original ], exp );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => new _.containerAdapter( [ 1, 2 ], [ 1, 2 ] ) );

  test.case = 'container is not an array or a set';
  test.shouldThrowErrorSync( () => new _.containerAdapter( _.argumentsArray.make( 10 ) ) );
  test.shouldThrowErrorSync( () => new _.containerAdapter( new U8x( 10 ) ) );
  test.shouldThrowErrorSync( () => new _.containerAdapter( { a : 0 } ) );
  test.shouldThrowErrorSync( () => new _.containerAdapter( 'str' ) );
  test.shouldThrowErrorSync( () => new _.containerAdapter( 0 ) );
  test.shouldThrowErrorSync( () => new _.containerAdapter( null ) );
  test.shouldThrowErrorSync( () => new _.containerAdapter( undefined ) );
}

//

function from( test )
{
  test.case = 'from empty array';
  var src = [];
  var exp = [];
  var got = _.containerAdapter.from( src );
  test.true( got !== src );
  test.true( _.arrayIs( got.original ) );
  test.identical( got.original, exp );

  test.case = 'from array';
  var src = [ 1, 2, '', {}, [], null, undefined ];
  var exp = [ 1, 2, '', {}, [], null, undefined ];
  var got = _.containerAdapter.from( src );
  test.true( got !== src );
  test.true( _.arrayIs( got.original ) );
  test.identical( got.original, exp );

  test.case = 'from empty unroll';
  var src = _.unroll.from( [] );
  var exp = [];
  var got = _.containerAdapter.from( src );
  test.true( got !== src );
  test.true( _.arrayIs( got.original ) );
  test.identical( got.original, exp );

  test.case = 'from unroll';
  var src = _.unroll.from( [ 1, 2, '', {}, [], null, undefined ] );
  var exp = [ 1, 2, '', {}, [], null, undefined ];
  var got = _.containerAdapter.from( src );
  test.true( got !== src );
  test.true( _.arrayIs( got.original ) );
  test.identical( got.original, exp );

  test.case = 'from empty Set';
  var src = new Set();
  var exp = [];
  var got = _.containerAdapter.from( src );
  test.true( got !== src );
  test.true( _.set.is( got.original ) );
  test.identical( [ ... got.original ], exp );

  test.case = 'from Set';
  var src = new Set( [ 1, 2, '', {}, [], null, undefined ] );
  var exp = [ 1, 2, '', {}, [], null, undefined ];
  var got = _.containerAdapter.from( src );
  test.true( got !== src );
  test.true( _.set.is( got.original ) );
  test.identical( [ ... got.original ], exp );

  test.case = 'from empty arrayAdapterContainer';
  var src = _.containerAdapter.from( [] );
  var exp = [];
  var got = _.containerAdapter.from( src );
  test.true( got === src );
  test.true( _.arrayIs( got.original ) );
  test.identical( got.original, exp );

  test.case = 'from arrayAdapterContainer';
  var src = _.containerAdapter.from( [ 1, 2, '', {}, [], null, undefined ] );
  var exp = [ 1, 2, '', {}, [], null, undefined ];
  var got = _.containerAdapter.from( src );
  test.true( got === src );
  test.true( _.arrayIs( got.original ) );
  test.identical( got.original, exp );

  test.case = 'from empty setAdapterContainer';
  var src = _.containerAdapter.from( new Set( [] ) );
  var exp = [];
  var got = _.containerAdapter.from( src );
  test.true( got === src );
  test.true( _.set.is( got.original ) );
  test.identical( [ ... got.original ], exp );

  test.case = 'from setAdapterContainer';
  var src = _.containerAdapter.from( new Set( [ 1, 2, '', {}, [], null, undefined ] ) );
  var exp = [ 1, 2, '', {}, [], null, undefined ];
  var got = _.containerAdapter.from( src );
  test.true( got === src );
  test.true( _.set.is( got.original ) );
  test.identical( [ ... got.original ], exp );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () => _.containerAdapter.from( [ 1, 2 ], [ 1, 2 ] ) );

  test.case = 'container is not an array or a set';
  test.shouldThrowErrorSync( () => _.containerAdapter.from( _.argumentsArray.make( 10 ) ) );
  test.shouldThrowErrorSync( () => _.containerAdapter.from( new U8x( 10 ) ) );
  test.shouldThrowErrorSync( () => _.containerAdapter.from( { a : 0 } ) );
  test.shouldThrowErrorSync( () => _.containerAdapter.from( 'str' ) );
  test.shouldThrowErrorSync( () => _.containerAdapter.from( 0 ) );
  test.shouldThrowErrorSync( () => _.containerAdapter.from( null ) );
  test.shouldThrowErrorSync( () => _.containerAdapter.from( undefined ) );
}

//

function toOriginal( test )
{
  test.case = 'empty';
  var exp = undefined;
  var got = _.containerAdapter.toOriginal();
  test.identical( got, exp );

  test.case = 'from null';
  var src = null;
  var exp = null;
  var got = _.containerAdapter.toOriginal( src );
  test.true( got === src );
  test.identical( got, exp );

  test.case = 'from undefined';
  var src = undefined;
  var exp = undefined;
  var got = _.containerAdapter.toOriginal( src );
  test.true( got === src );
  test.identical( got, exp );

  test.case = 'from number';
  var src = 5;
  var exp = 5;
  var got = _.containerAdapter.toOriginal( src );
  test.true( got === src );
  test.identical( got, exp );

  test.case = 'from string';
  var src = 'str';
  var exp = 'str';
  var got = _.containerAdapter.toOriginal( src );
  test.true( got === src );
  test.identical( got, exp );

  test.case = 'from boolean';
  var src = true;
  var exp = true;
  var got = _.containerAdapter.toOriginal( src );
  test.true( got === src );
  test.identical( got, exp );

  test.case = 'from array';
  var src = [ 1, 2, 'str' ];
  var exp = [ 1, 2, 'str' ];
  var got = _.containerAdapter.toOriginal( src );
  test.true( got === src );
  test.identical( got, exp );

  test.case = 'from unroll';
  var src = _.unroll.make( [ 1, 2, 'str' ] );
  var exp = _.unroll.make( [ 1, 2, 'str' ] );
  var got = _.containerAdapter.toOriginal( src );
  test.true( got === src );
  test.identical( got, exp );

  test.case = 'from argumentsArray';
  var src = _.argumentsArray.make( [ 1, 2, 'str' ] );
  var exp = _.argumentsArray.make( [ 1, 2, 'str' ] );
  var got = _.containerAdapter.toOriginal( src );
  test.true( got === src );
  test.identical( got, exp );

  test.case = 'from BufferRaw';
  var src = new BufferRaw( [ 1, 2, 'str' ] );
  var exp = new BufferRaw( [ 1, 2, 'str' ] );
  var got = _.containerAdapter.toOriginal( src );
  test.true( got === src );
  test.identical( got, exp );

  test.case = 'from BufferTyped';
  var src = new I16x( [ 1, 2, 'str' ] );
  var exp = new I16x( [ 1, 2, 'str' ] );
  var got = _.containerAdapter.toOriginal( src );
  test.true( got === src );
  test.identical( got, exp );

  test.case = 'from map';
  var src = { a : 0 };
  var exp = { a : 0 };
  var got = _.containerAdapter.toOriginal( src );
  test.true( got === src );
  test.identical( got, exp );

  test.case = 'from Map';
  var src = new HashMap( [ [ 1, 2 ] ] );
  var exp = new HashMap( [ [ 1, 2 ] ] );
  var got = _.containerAdapter.toOriginal( src );
  test.true( got === src );
  test.identical( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'from Set';
  var src = new Set();
  var exp = new Set();
  var got = _.containerAdapter.toOriginal( src );
  test.true( got === src );
  test.identical( [ ... got ], [ ... exp ] );

  test.case = 'from Symbol';
  var src = Symbol( 'a' );
  var exp = src;
  var got = _.containerAdapter.toOriginal( src );
  test.true( got === src );
  test.identical( got, exp );

  test.case = 'from instance of constructor';
  function Constr()
  {
    this.x = 1;
    return this;
  }
  var src = new Constr();
  var got = _.containerAdapter.toOriginal( src );
  test.true( got === src );
  test.identical( got.x, 1 );

  /* */

  test.case = 'from ContainerAdapterArray';
  var src = _.containerAdapter.make( [ 1, 2, '', {}, [], null, undefined ] );
  var got = _.containerAdapter.toOriginal( src );
  test.true( got !== src );
  test.identical( got, src.original );

  test.case = 'from ContainerAdapterSet';
  var src = _.containerAdapter.make( new Set( [ 1, 2, '', {}, [], null, undefined ] ) );
  var got = _.containerAdapter.toOriginal( src );
  test.true( got !== src );
  test.identical( got, src.original );
}

//

function toOriginals( test )
{
  test.open( 'dsts' );

  test.case = 'empty';
  var exp = undefined;
  var got = _.containerAdapter.toOriginals();
  test.identical( got, exp );

  test.case = 'from null';
  var src = null;
  var exp = null;
  var got = _.containerAdapter.toOriginals( src );
  test.true( got === src );
  test.identical( got, exp );

  test.case = 'dsts - null, from null';
  var src = null;
  var exp = null;
  var got = _.containerAdapter.toOriginals( null, src );
  test.true( got === src );
  test.identical( got, exp );

  test.case = 'from undefined';
  var src = undefined;
  var exp = undefined;
  var got = _.containerAdapter.toOriginals( src );
  test.true( got === src );
  test.identical( got, exp );

  test.case = 'dsts - null, from undefined';
  var src = undefined;
  var exp = null;
  var got = _.containerAdapter.toOriginals( null, src );
  test.true( got !== src );
  test.identical( got, exp );

  test.case = 'from number';
  var src = 5;
  var exp = 5;
  var got = _.containerAdapter.toOriginals( src );
  test.true( got === src );
  test.identical( got, exp );

  test.case = 'dsts - null, from number';
  var src = 5;
  var exp = [ 5 ];
  var got = _.containerAdapter.toOriginals( null, src );
  test.true( got !== src );
  test.identical( got, exp );

  test.case = 'from string';
  var src = 'str';
  var exp = 'str';
  var got = _.containerAdapter.toOriginals( src );
  test.true( got === src );
  test.identical( got, exp );

  test.case = 'dsts - null, from string';
  var src = 'str';
  var exp = [ 'str' ];
  var got = _.containerAdapter.toOriginals( null, src );
  test.true( got !== src );
  test.identical( got, exp );

  test.case = 'from boolean';
  var src = true;
  var exp = true;
  var got = _.containerAdapter.toOriginals( src );
  test.true( got === src );
  test.identical( got, exp );

  test.case = 'dsts - null, from boolean';
  var src = true;
  var exp = [ true ];
  var got = _.containerAdapter.toOriginals( null, src );
  test.true( got !== src );
  test.identical( got, exp );

  test.case = 'from array';
  var src = [ 1, 2, 'str' ];
  var exp = [ 1, 2, 'str' ];
  var got = _.containerAdapter.toOriginals( src );
  test.true( got === src );
  test.identical( got, exp );

  test.case = 'dsts - null, from array';
  var src = [ 1, 2, 'str' ];
  var exp = [ 1, 2, 'str' ];
  var got = _.containerAdapter.toOriginals( null, src );
  test.true( got !== src );
  test.identical( got, exp );

  test.case = 'from empty array';
  var src = [];
  var exp = [];
  var got = _.containerAdapter.toOriginals( src );
  test.true( got === src );
  test.identical( got, exp );

  test.case = 'dsts - null, from empty array';
  var src = [];
  var exp = [];
  var got = _.containerAdapter.toOriginals( null, src );
  test.true( got !== src );
  test.identical( got, exp );

  test.case = 'from unroll';
  var src = _.unroll.make( [ 1, 2, 'str' ] );
  var exp = _.unroll.make( [ 1, 2, 'str' ] );
  var got = _.containerAdapter.toOriginals( src );
  test.true( got === src );
  test.identical( got, exp );

  test.case = 'dsts - null, from unroll';
  var src = _.unroll.make( [ 1, 2, 'str' ] );
  var exp = _.unroll.make( [ 1, 2, 'str' ] );
  var got = _.containerAdapter.toOriginals( null, src );
  test.true( got !== src );
  test.identical( got, exp );

  test.case = 'from argumentsArray';
  var src = _.argumentsArray.make( [ 1, 2, 'str' ] );
  var exp = _.argumentsArray.make( [ 1, 2, 'str' ] );
  var got = _.containerAdapter.toOriginals( src );
  test.true( got === src );
  test.identical( got, exp );

  test.case = 'dsts - null, from argumentsArray';
  var src = _.argumentsArray.make( [ 1, 2, 'str' ] );
  var exp = [ 1, 2, 'str' ];
  var got = _.containerAdapter.toOriginals( null, src );
  test.true( got !== src );
  test.identical( got, exp );

  test.case = 'from BufferRaw';
  var src = new BufferRaw( [ 1, 2, 'str' ] );
  var exp = new BufferRaw( [ 1, 2, 'str' ] );
  var got = _.containerAdapter.toOriginals( src );
  test.true( got === src );
  test.identical( got, exp );

  test.case = 'dsts - null, from BufferRaw';
  var src = new BufferRaw( [ 1, 2, 'str' ] );
  var exp = [ new BufferRaw( [ 1, 2, 'str' ] ) ];
  var got = _.containerAdapter.toOriginals( null, src );
  test.true( got !== src );
  test.identical( got, exp );

  test.case = 'from BufferTyped';
  var src = new I16x( [ 1, 2, 'str' ] );
  var exp = new I16x( [ 1, 2, 'str' ] );
  var got = _.containerAdapter.toOriginals( src );
  test.true( got === src );
  test.identical( got, exp );

  test.case = 'from map';
  var src = { a : 0 };
  var exp = { a : 0 };
  var got = _.containerAdapter.toOriginals( src );
  test.true( got === src );
  test.identical( got, exp );

  test.case = 'dsts - null, from map';
  var src = { a : 0 };
  var exp = [ { a : 0 } ];
  var got = _.containerAdapter.toOriginals( null, src );
  test.true( got !== src );
  test.identical( got, exp );

  test.case = 'from Map';
  var src = new HashMap( [ [ 1, 2 ] ] );
  var exp = new HashMap( [ [ 1, 2 ] ] );
  var got = _.containerAdapter.toOriginals( src );
  test.true( got === src );
  test.identical( [ ... got.entries() ], [ ... exp.entries() ] );

  test.case = 'dsts - null, from Map';
  var src = new HashMap( [ [ 1, 2 ] ] );
  var exp = new HashMap( [ [ 1, 2 ] ] );
  var got = _.containerAdapter.toOriginals( null, src );
  test.true( got !== src );
  test.identical( [ ... got[ 0 ].entries() ], [ ... exp.entries() ] );

  test.case = 'from Set';
  var src = new Set();
  var exp = new Set();
  var got = _.containerAdapter.toOriginals( src );
  test.true( got === src );
  test.identical( [ ... got ], [ ... exp ] );

  test.case = 'dsts - null, from Set';
  var src = new Set();
  var exp = new Set();
  var got = _.containerAdapter.toOriginals( null, src );
  test.true( got !== src );
  test.identical( [ ... got[ 0 ] ], [ ... exp ] );

  test.case = 'from Symbol';
  var src = Symbol( 'a' );
  var exp = src;
  var got = _.containerAdapter.toOriginals( src );
  test.true( got === src );
  test.identical( got, exp );

  test.case = 'from instance of constructor';
  function Constr()
  {
    this.x = 1;
    return this;
  }
  var src = new Constr();
  var got = _.containerAdapter.toOriginals( src );
  test.true( got === src );
  test.identical( got.x, 1 );

  /* */

  test.case = 'from ContainerAdapterArray';
  var src = _.containerAdapter.make( [ 1, 2, '', {}, [], null, undefined ] );
  var got = _.containerAdapter.toOriginals( src );
  test.true( got !== src );
  test.identical( got, src.original );

  test.case = 'dsts - null, from ContainerAdapterArray';
  var src = _.containerAdapter.make( [ 1, 2, '', {}, [], null, undefined ] );
  var got = _.containerAdapter.toOriginals( null, src );
  test.true( got !== src );
  test.identical( got[ 0 ], src.original );

  test.case = 'from ContainerAdapterSet';
  var src = _.containerAdapter.make( new Set( [ 1, 2, '', {}, [], null, undefined ] ) );
  var got = _.containerAdapter.toOriginals( src );
  test.true( got !== src );
  test.identical( got, src.original );

  test.case = 'dsts - null, from ContainerAdapterSet';
  var src = _.containerAdapter.make( new Set( [ 1, 2, '', {}, [], null, undefined ] ) );
  var got = _.containerAdapter.toOriginals( null, src );
  test.true( got !== src );
  test.identical( got[ 0 ], src.original );

  /* */

  test.case = 'array contains ContainerAdapterArray';
  var src = _.containerAdapter.make( [ 1, 2, 'str' ] );
  var srcs = [ src, 1, src, 'str', src, undefined, null, false ];
  var got = _.containerAdapter.toOriginals( srcs );
  test.true( got === srcs );
  test.identical( got.length, 8 );
  test.identical( got[ 0 ], src.original );
  test.identical( got[ 2 ], src.original );

  test.case = 'dsts - null, array contains ContainerAdapterArray';
  var src = _.containerAdapter.make( [ 1, 2, 'str' ] );
  var srcs = [ src, 1, src, 'str', src, undefined, null, false ];
  var got = _.containerAdapter.toOriginals( null, srcs );
  test.true( got !== srcs );
  test.identical( got.length, 8 );
  test.identical( got[ 0 ], src.original );
  test.identical( got[ 2 ], src.original );

  test.case = 'array contains ContainerAdapterSet';
  var src = _.containerAdapter.make( new Set( [ 1, 2, 'str' ] ) );
  var srcs = [ src, 1, src, 'str', src, undefined, null, false ];
  var got = _.containerAdapter.toOriginals( srcs );
  test.true( got === srcs );
  test.identical( got.length, 8 );
  test.identical( [ ... got[ 0 ] ], [ ... src.original ] );
  test.identical( [ ... got[ 2 ] ], [ ... src.original ] );

  test.case = 'array contains ContainerAdapterArray and ContainerAdapterSet';
  var src1 = _.containerAdapter.make( [ 1, 2, 'str' ] );
  var src2 = _.containerAdapter.make( new Set( [ 1, 2, 'str' ] ) );
  var srcs = [ src1, 1, src2, 'str', src1, undefined, null, false ];
  var got = _.containerAdapter.toOriginals( srcs );
  test.true( got === srcs );
  test.identical( got.length, 8 );
  test.identical( got[ 0 ], src1.original );
  test.identical( [ ... got[ 2 ] ], [ ... src2.original ] );

  test.case = 'array contains ContainerAdapterArray and ContainerAdapterSet';
  var src1 = _.containerAdapter.make( [ 1, 2, 'str' ] );
  var src2 = _.containerAdapter.make( new Set( [ 1, 2, 'str' ] ) );
  var srcs = [ src1, 1, src2, 'str', src1, undefined, null, false ];
  var got = _.containerAdapter.toOriginals( null, srcs );
  test.true( got !== srcs );
  test.identical( got.length, 8 );
  test.identical( got[ 0 ], src1.original );
  test.identical( [ ... got[ 2 ] ], [ ... src2.original ] );

  test.close( 'dsts' );

  /* - */

  test.case = 'from two primitive';
  var dst = 1;
  var src = 'str';
  var exp = [ 1, 'str' ];
  var got = _.containerAdapter.toOriginals( dst, src );
  test.true( got !== dst );
  test.true( got !== src );
  test.identical( got, exp );

  test.case = 'dsts - constructor, src - primitive';
  var dst = new Constr();
  var src = true;
  var exp = [ dst, true ];
  var got = _.containerAdapter.toOriginals( dst, src );
  test.true( got !== dst );
  test.true( got !== src );
  test.identical( got, exp );

  test.case = 'dsts - primitive, src - array';
  var dst = 1;
  var src = [ [ 2, 'str' ], [ 1 ] ];
  var exp = [ 1, [ 2, 'str' ], [ 1 ] ];
  var got = _.containerAdapter.toOriginals( dst, src );
  test.true( got !== dst );
  test.true( got !== src );
  test.identical( got, exp );

  test.case = 'dsts - primitive, src - unroll';
  var dst = 1;
  var src = _.unroll.make( [ [ 2, 'str' ], [ 1 ] ] );
  var exp = [ 1, [ 2, 'str' ], [ 1 ] ];
  var got = _.containerAdapter.toOriginals( dst, src );
  test.true( got !== dst );
  test.true( got !== src );
  test.identical( got, exp );

  test.case = 'dsts - primitive, src - argumentsArray';
  var dst = 1;
  var src = _.argumentsArray.make( [ [ 2, 'str' ], [ 1 ] ] );
  var exp = [ 1, [ 2, 'str' ], [ 1 ] ];
  var got = _.containerAdapter.toOriginals( dst, src );
  test.true( got !== dst );
  test.true( got !== src );
  test.identical( got, exp );

  test.case = 'dsts - array, src - array';
  var dst = [ 1, { a : 0 } ];
  var src = [ [ 2, 'str' ], [ 1 ] ];
  var exp = [ 1, { a : 0 }, [ 2, 'str' ], [ 1 ] ];
  var got = _.containerAdapter.toOriginals( dst, src );
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got, exp );

  test.case = 'dsts - unroll, src - array';
  var dst = _.unroll.make( [ 1, { a : 0 } ] );
  var src = [ [ 2, 'str' ], [ 1 ] ];
  var exp = [ 1, { a : 0 }, [ 2, 'str' ], [ 1 ] ];
  var got = _.containerAdapter.toOriginals( dst, src );
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got, exp );

  test.case = 'dsts - argumentsArray, src - array';
  var dst = _.argumentsArray.make( [ 1, { a : 0 } ] );
  var src = [ [ 2, 'str' ], [ 1 ] ];
  var exp = [ [ 1, { a : 0 } ], [ 2, 'str' ], [ 1 ] ];
  var got = _.containerAdapter.toOriginals( dst, src );
  test.true( got !== dst );
  test.true( got !== src );
  test.equivalent( got, exp );
}

//--
// ContainerAdapterAbstract
//--

function ToOriginal( test )
{
  test.open( 'arrayContainerAdapter' );

  test.case = 'number';
  var src = _.containerAdapter.make( [] );
  var got = src.ToOriginal( 1 );
  test.identical( got, 1 );

  test.case = 'string';
  var src = _.containerAdapter.make( [] );
  var got = src.ToOriginal( 'str' );
  test.identical( got, 'str' );

  test.case = 'array';
  var src = _.containerAdapter.make( [] );
  var got = src.ToOriginal( [ 1, 2 ] );
  test.identical( got, [ 1, 2 ] );

  test.case = 'map';
  var src = _.containerAdapter.make( [] );
  var got = src.ToOriginal( { a : 0 } );
  test.identical( got, { a : 0 } );

  test.case = 'BufferTyped';
  var src = _.containerAdapter.make( [] );
  var got = src.ToOriginal( new U8x( 2 ) );
  test.identical( got, new U8x( 2 ) );

  test.case = 'BufferTyped';
  var src = _.containerAdapter.make( [] );
  var got = src.ToOriginal( new Set( [ 1, 2 ] ) );
  test.identical( [ ... got ], [ ... (  new Set( [ 1, 2 ] ) ) ] );

  test.case = 'arrayContainerAdapter';
  var container = _.containerAdapter.make( [ 1, 2 ] );
  var src = _.containerAdapter.make( [] );
  var got = src.ToOriginal( container );
  test.identical( got, container.original );

  test.case = 'setContainerAdapter';
  var container = _.containerAdapter.make( new Set( [ 1, 2 ] ) );
  var src = _.containerAdapter.make( [] );
  var got = src.ToOriginal( container );
  test.identical( [ ... got ], [ ... container.original ] );

  test.close( 'arrayContainerAdapter' );

  /* - */

  test.open( 'setContainerAdapter' );

  test.case = 'number';
  var src = _.containerAdapter.make( new Set() );
  var got = src.ToOriginal( 1 );
  test.identical( got, 1 );

  test.case = 'string';
  var src = _.containerAdapter.make( new Set() );
  var got = src.ToOriginal( 'str' );
  test.identical( got, 'str' );

  test.case = 'array';
  var src = _.containerAdapter.make( new Set() );
  var got = src.ToOriginal( [ 1, 2 ] );
  test.identical( got, [ 1, 2 ] );

  test.case = 'map';
  var src = _.containerAdapter.make( new Set() );
  var got = src.ToOriginal( { a : 0 } );
  test.identical( got, { a : 0 } );

  test.case = 'BufferTyped';
  var src = _.containerAdapter.make( new Set() );
  var got = src.ToOriginal( new U8x( 2 ) );
  test.identical( got, new U8x( 2 ) );

  test.case = 'BufferTyped';
  var src = _.containerAdapter.make( new Set() );
  var got = src.ToOriginal( new Set( [ 1, 2 ] ) );
  test.identical( [ ... got ], [ ... (  new Set( [ 1, 2 ] ) ) ] );

  test.case = 'arrayContainerAdapter';
  var container = _.containerAdapter.make( [ 1, 2 ] );
  var src = _.containerAdapter.make( new Set() );
  var got = src.ToOriginal( container );
  test.identical( got, container.original );

  test.case = 'setContainerAdapter';
  var container = _.containerAdapter.make( new Set( [ 1, 2 ] ) );
  var src = _.containerAdapter.make( new Set() );
  var got = src.ToOriginal( container );
  test.identical( [ ... got ], [ ... container.original ] );

  test.close( 'setContainerAdapter' );
}

//

function is_( test )
{
  test.open( 'arrayContainerAdapter' );

  test.case = 'undefined';
  var src = _.containerAdapter.make( [] );
  var got = src.Is( undefined );
  test.identical( got, false );

  test.case = 'null';
  var src = _.containerAdapter.make( [] );
  var got = src.Is( null );
  test.identical( got, false );

  test.case = 'number';
  var src = _.containerAdapter.make( [] );
  var got = src.Is( 1 );
  test.identical( got, false );

  test.case = 'zero';
  var src = _.containerAdapter.make( [] );
  var got = src.Is( 0 );
  test.identical( got, false );

  test.case = 'string';
  var src = _.containerAdapter.make( [] );
  var got = src.Is( 'str' );
  test.identical( got, false );

  test.case = 'empty string';
  var src = _.containerAdapter.make( [] );
  var got = src.Is( '' );
  test.identical( got, false );

  test.case = 'array';
  var src = _.containerAdapter.make( [] );
  var got = src.Is( [ 1, 2 ] );
  test.identical( got, false );

  test.case = 'map';
  var src = _.containerAdapter.make( [] );
  var got = src.Is( { a : 0 } );
  test.identical( got, false );

  test.case = 'BufferTyped';
  var src = _.containerAdapter.make( [] );
  var got = src.Is( new U8x( 2 ) );
  test.identical( got, false );

  test.case = 'Set';
  var src = _.containerAdapter.make( [] );
  var got = src.Is( new Set( [ 1, 2 ] ) );
  test.identical( got, false );

  test.case = 'arrayContainerAdapter';
  var container = _.containerAdapter.make( [ 1, 2 ] );
  var src = _.containerAdapter.make( [] );
  var got = src.Is( container );
  test.identical( got, true );

  test.case = 'setContainerAdapter';
  var container = _.containerAdapter.make( new Set( [ 1, 2 ] ) );
  var src = _.containerAdapter.make( [] );
  var got = src.Is( container );
  test.identical( got, true );

  test.close( 'arrayContainerAdapter' );

  /* - */

  test.open( 'setContainerAdapter' );

  test.case = 'undefined';
  var src = _.containerAdapter.make( new Set() );
  var got = src.Is( undefined );
  test.identical( got, false );

  test.case = 'null';
  var src = _.containerAdapter.make( new Set() );
  var got = src.Is( null );
  test.identical( got, false );

  test.case = 'number';
  var src = _.containerAdapter.make( new Set() );
  var got = src.Is( 1 );
  test.identical( got, false );

  test.case = 'zero';
  var src = _.containerAdapter.make( new Set() );
  var got = src.Is( 0 );
  test.identical( got, false );

  test.case = 'string';
  var src = _.containerAdapter.make( new Set() );
  var got = src.Is( 'str' );
  test.identical( got, false );

  test.case = 'empty string';
  var src = _.containerAdapter.make( new Set() );
  var got = src.Is( '' );
  test.identical( got, false );

  test.case = 'array';
  var src = _.containerAdapter.make( new Set() );
  var got = src.Is( [ 1, 2 ] );
  test.identical( got, false );

  test.case = 'map';
  var src = _.containerAdapter.make( new Set() );
  var got = src.Is( { a : 0 } );
  test.identical( got, false );

  test.case = 'BufferTyped';
  var src = _.containerAdapter.make( new Set() );
  var got = src.Is( new U8x( 2 ) );
  test.identical( got, false );

  test.case = 'Set';
  var src = _.containerAdapter.make( new Set() );
  var got = src.Is( new Set( [ 1, 2 ] ) );
  test.identical( got, false );

  test.case = 'arrayContainerAdapter';
  var container = _.containerAdapter.make( [ 1, 2 ] );
  var src = _.containerAdapter.make( new Set() );
  var got = src.Is( container );
  test.identical( got, true );

  test.case = 'setContainerAdapter';
  var container = _.containerAdapter.make( new Set( [ 1, 2 ] ) );
  var src = _.containerAdapter.make( new Set() );
  var got = src.Is( container );
  test.identical( got, true );

  test.close( 'setContainerAdapter' );
}

//

function isContainer( test )
{
  test.open( 'arrayContainerAdapter' );

  test.case = 'undefined';
  var src = _.containerAdapter.make( [] );
  var got = src.IsContainer( undefined );
  test.identical( got, false );

  test.case = 'null';
  var src = _.containerAdapter.make( [] );
  var got = src.IsContainer( null );
  test.identical( got, false );

  test.case = 'number';
  var src = _.containerAdapter.make( [] );
  var got = src.IsContainer( 1 );
  test.identical( got, false );

  test.case = 'zero';
  var src = _.containerAdapter.make( [] );
  var got = src.IsContainer( 0 );
  test.identical( got, false );

  test.case = 'string';
  var src = _.containerAdapter.make( [] );
  var got = src.IsContainer( 'str' );
  test.identical( got, false );

  test.case = 'empty string';
  var src = _.containerAdapter.make( [] );
  var got = src.IsContainer( '' );
  test.identical( got, false );

  test.case = 'array';
  var src = _.containerAdapter.make( [] );
  var got = src.IsContainer( [ 1, 2 ] );
  test.identical( got, true );

  test.case = 'unroll';
  var src = _.containerAdapter.make( [] );
  var got = src.IsContainer( _.unroll.make( [ 1, 2 ] ) );
  test.identical( got, true );

  test.case = 'argumentsArray';
  var src = _.containerAdapter.make( [] );
  var got = src.IsContainer( _.argumentsArray.make( [ 1, 2 ] ) );
  test.identical( got, true );

  test.case = 'map';
  var src = _.containerAdapter.make( [] );
  var got = src.IsContainer( { a : 0 } );
  test.identical( got, false );

  test.case = 'BufferTyped';
  var src = _.containerAdapter.make( [] );
  var got = src.IsContainer( new U8x( 2 ) );
  test.identical( got, true );

  test.case = 'Set';
  var src = _.containerAdapter.make( [] );
  var got = src.IsContainer( new Set( [ 1, 2 ] ) );
  test.identical( got, true );

  test.case = 'arrayContainerAdapter';
  var container = _.containerAdapter.make( [ 1, 2 ] );
  var src = _.containerAdapter.make( [] );
  var got = src.IsContainer( container );
  test.identical( got, true );

  test.case = 'setContainerAdapter';
  var container = _.containerAdapter.make( new Set( [ 1, 2 ] ) );
  var src = _.containerAdapter.make( [] );
  var got = src.IsContainer( container );
  test.identical( got, true );

  test.close( 'arrayContainerAdapter' );

  /* - */

  test.open( 'setContainerAdapter' );

  test.case = 'undefined';
  var src = _.containerAdapter.make( new Set() );
  var got = src.IsContainer( undefined );
  test.identical( got, false );

  test.case = 'null';
  var src = _.containerAdapter.make( new Set() );
  var got = src.IsContainer( null );
  test.identical( got, false );

  test.case = 'number';
  var src = _.containerAdapter.make( new Set() );
  var got = src.IsContainer( 1 );
  test.identical( got, false );

  test.case = 'zero';
  var src = _.containerAdapter.make( new Set() );
  var got = src.IsContainer( 0 );
  test.identical( got, false );

  test.case = 'string';
  var src = _.containerAdapter.make( new Set() );
  var got = src.IsContainer( 'str' );
  test.identical( got, false );

  test.case = 'empty string';
  var src = _.containerAdapter.make( new Set() );
  var got = src.IsContainer( '' );
  test.identical( got, false );

  test.case = 'array';
  var src = _.containerAdapter.make( new Set() );
  var got = src.IsContainer( [ 1, 2 ] );
  test.identical( got, true );

  test.case = 'unroll';
  var src = _.containerAdapter.make( new Set() );
  var got = src.IsContainer( _.unroll.make( [ 1, 2 ] ) );
  test.identical( got, true );

  test.case = 'argumentsArray';
  var src = _.containerAdapter.make( new Set() );
  var got = src.IsContainer( _.argumentsArray.make( [ 1, 2 ] ) );
  test.identical( got, true );

  test.case = 'map';
  var src = _.containerAdapter.make( new Set() );
  var got = src.IsContainer( { a : 0 } );
  test.identical( got, false );

  test.case = 'BufferTyped';
  var src = _.containerAdapter.make( new Set() );
  var got = src.IsContainer( new U8x( 2 ) );
  test.identical( got, true );

  test.case = 'Set';
  var src = _.containerAdapter.make( new Set() );
  var got = src.IsContainer( new Set( [ 1, 2 ] ) );
  test.identical( got, true );

  test.case = 'arrayContainerAdapter';
  var container = _.containerAdapter.make( [ 1, 2 ] );
  var src = _.containerAdapter.make( new Set() );
  var got = src.IsContainer( container );
  test.identical( got, true );

  test.case = 'setContainerAdapter';
  var container = _.containerAdapter.make( new Set( [ 1, 2 ] ) );
  var src = _.containerAdapter.make( new Set() );
  var got = src.IsContainer( container );
  test.identical( got, true );

  test.close( 'setContainerAdapter' );
}

//

function from_( test )
{
  test.case = 'from empty array';
  var container = _.containerAdapter.from( [] );
  var src = [];
  var exp = [];
  var got = container.From( src );
  test.true( got !== container );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'from array';
  var container = _.containerAdapter.from( [] );
  var src = [ 1, 2, '', {}, [], null, undefined ];
  var exp = [ 1, 2, '', {}, [], null, undefined ];
  var got = container.From( src );
  test.true( got !== container );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'from empty unroll';
  var container = _.containerAdapter.from( [] );
  var src = _.unroll.from( [] );
  var exp = [];
  var got = container.From( src );
  test.true( got !== container );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'from unroll';
  var container = _.containerAdapter.from( [] );
  var src = _.unroll.from( [ 1, 2, '', {}, [], null, undefined ] );
  var exp = [ 1, 2, '', {}, [], null, undefined ];
  var got = container.From( src );
  test.true( got !== container );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'from empty Set';
  var container = _.containerAdapter.from( [] );
  var src = new Set();
  var exp = [];
  var got = container.From( src );
  test.true( got !== container );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from Set';
  var container = _.containerAdapter.from( [] );
  var src = new Set( [ 1, 2, '', {}, [], null, undefined ] );
  var exp = [ 1, 2, '', {}, [], null, undefined ];
  var got = container.From( src );
  test.true( got !== container );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from empty arrayAdapterContainer';
  var container = _.containerAdapter.from( [] );
  var src = _.containerAdapter.from( [] );
  var exp = [];
  var got = container.From( src );
  test.true( got !== container );
  test.true( got === src );
  test.identical( got.original, exp );

  test.case = 'from arrayAdapterContainer';
  var container = _.containerAdapter.from( [] );
  var src = _.containerAdapter.from( [ 1, 2, '', {}, [], null, undefined ] );
  var exp = [ 1, 2, '', {}, [], null, undefined ];
  var got = container.From( src );
  test.true( got !== container );
  test.true( got === src );
  test.identical( got.original, exp );

  test.case = 'from empty setAdapterContainer';
  var container = _.containerAdapter.from( [] );
  var src = _.containerAdapter.from( new Set( [] ) );
  var exp = [];
  var got = container.From( src );
  test.true( got !== container );
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from setAdapterContainer';
  var container = _.containerAdapter.from( [] );
  var src = _.containerAdapter.from( new Set( [ 1, 2, '', {}, [], null, undefined ] ) );
  var exp = [ 1, 2, '', {}, [], null, undefined ];
  var got = container.From( src );
  test.true( got !== container );
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  /* */

  test.case = 'from arrayAdapterContainer';
  var container = _.containerAdapter.from( new Set() );
  var src = _.containerAdapter.from( [ 1, 2, '', {}, [], null, undefined ] );
  var exp = [ 1, 2, '', {}, [], null, undefined ];
  var got = container.From( src );
  test.true( got !== container );
  test.true( got === src );
  test.identical( got.original, exp );

  test.case = 'from empty setAdapterContainer';
  var container = _.containerAdapter.from( new Set() );
  var src = _.containerAdapter.from( new Set( [] ) );
  var exp = [];
  var got = container.From( src );
  test.true( got !== container );
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from setAdapterContainer';
  var container = _.containerAdapter.from( new Set() );
  var src = _.containerAdapter.from( new Set( [ 1, 2, '', {}, [], null, undefined ] ) );
  var exp = [ 1, 2, '', {}, [], null, undefined ];
  var got = container.From( src );
  test.true( got !== container );
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () =>
  {
    var container = _.containerAdapter.from( [] );
    container.From();
  });

  test.case = 'container is not an array or a set';
  test.shouldThrowErrorSync( () =>
  {
    var container = _.containerAdapter.from( [] );
    container.From( _.argumentsArray.make( 10 ) );
  });
  test.shouldThrowErrorSync( () =>
  {
    var container = _.containerAdapter.from( [] );
    container.From( _.argumentsArray.make( new U8x( 10 ) ) );
  });
  test.shouldThrowErrorSync( () =>
  {
    var container = _.containerAdapter.from( [] );
    container.From( _.argumentsArray.make( { a : 0 } ) );
  });
  test.shouldThrowErrorSync( () =>
  {
    var container = _.containerAdapter.from( [] );
    container.From( _.argumentsArray.make( 'str' ) );
  });
  test.shouldThrowErrorSync( () =>
  {
    var container = _.containerAdapter.from( [] );
    container.From( _.argumentsArray.make( 0 ) );
  });
  test.shouldThrowErrorSync( () =>
  {
    var container = _.containerAdapter.from( [] );
    container.From( _.argumentsArray.make( null ) );
  });
  test.shouldThrowErrorSync( () =>
  {
    var container = _.containerAdapter.from( [] );
    container.From( _.argumentsArray.make( undefined ) );
  });
}

//

function same( test )
{
  test.open( 'arrayContainerAdapter' );

  test.case = 'src - undefined';
  var container = _.containerAdapter.make( [] );
  var got = container.same( undefined );
  test.identical( got, false );

  test.case = 'src - null';
  var container = _.containerAdapter.make( [] );
  var got = container.same( null );
  test.identical( got, false );

  test.case = 'src - empty string';
  var container = _.containerAdapter.make( [] );
  var got = container.same( '' );
  test.identical( got, false );

  test.case = 'src - false';
  var container = _.containerAdapter.make( [] );
  var got = container.same( false );
  test.identical( got, false );

  test.case = 'src - zero';
  var container = _.containerAdapter.make( [] );
  var got = container.same( 0 );
  test.identical( got, false );

  test.case = 'src - number';
  var container = _.containerAdapter.make( [] );
  var got = container.same( 2 );
  test.identical( got, false );

  test.case = 'src - string';
  var container = _.containerAdapter.make( [] );
  var got = container.same( 'str' );
  test.identical( got, false );

  test.case = 'src - Map';
  var container = _.containerAdapter.make( [] );
  var got = container.same( new HashMap( [ [ 1, 2 ] ] ) );
  test.identical( got, false );

  test.case = 'src - map';
  var container = _.containerAdapter.make( [] );
  var got = container.same( { a : 0 } );
  test.identical( got, false );

  test.case = 'src - BufferTyped';
  var container = _.containerAdapter.make( [] );
  var got = container.same( new U8x( 5 ) );
  test.identical( got, false );

  test.case = 'src - Set, not same';
  var src = new Set( [ 1, 2 ] );
  var container = _.containerAdapter.make( [] );
  var got = container.same( src );
  test.identical( got, false );

  test.case = 'src - setContainerAdapter';
  var src = _.containerAdapter.make( new Set( [ 1, 2 ] ) );
  var container = _.containerAdapter.make( [] );
  var got = container.same( src );
  test.identical( got, false );

  /* */

  test.case = 'src - array, same';
  var src = [ 1, 2 ];
  var container = _.containerAdapter.make( src );
  var got = container.same( src );
  test.identical( got, true );

  test.case = 'src - array, not same';
  var src = [ 1, 2 ];
  var container = _.containerAdapter.make( [ 1, 2 ] );
  var got = container.same( src );
  test.identical( got, false );

  test.case = 'src - unroll, same';
  var src = _.unroll.make( [ 1, 2 ] );
  var container = _.containerAdapter.make( src );
  var got = container.same( src );
  test.identical( got, true );

  test.case = 'src - unroll, not same';
  var src = _.unroll.make( [ 1, 2 ] );
  var container = _.containerAdapter.make( _.unroll.make( [ 1, 2 ] ) );
  var got = container.same( src );
  test.identical( got, false );

  test.case = 'src - arrayContainerAdapter';
  var src = _.containerAdapter.make( [ 1, 2 ] );
  var container = _.containerAdapter.make( src );
  var got = container.same( src );
  test.identical( got, false );

  test.case = 'src - arrayContainerAdapter';
  var src = _.containerAdapter.make( [ 1, 2 ] );
  var container = _.containerAdapter.make( _.containerAdapter.make( [ 1, 2 ] ) );
  var got = container.same( src );
  test.identical( got, false );

  test.case = 'src - src';
  var container = _.containerAdapter.make( [] );
  var got = container.same( container );
  test.identical( got, true );

  test.case = 'two containers with same src';
  var src = [ 1, 2, 3 ];
  var src2 = _.containerAdapter.make( src );
  var container = _.containerAdapter.make( src );
  var got = container.same( src2 );
  test.identical( got, true );

  test.close( 'arrayContainerAdapter' );

  /* - */

  test.open( 'setContainerAdapter' );

  test.case = 'src - undefined';
  var container = _.containerAdapter.make( new Set( [] ) );
  var got = container.same( undefined );
  test.identical( got, false );

  test.case = 'src - null';
  var container = _.containerAdapter.make( new Set( [] ) );
  var got = container.same( null );
  test.identical( got, false );

  test.case = 'src - empty string';
  var container = _.containerAdapter.make( new Set( [] ) );
  var got = container.same( '' );
  test.identical( got, false );

  test.case = 'src - false';
  var container = _.containerAdapter.make( new Set( [] ) );
  var got = container.same( false );
  test.identical( got, false );

  test.case = 'src - zero';
  var container = _.containerAdapter.make( new Set( [] ) );
  var got = container.same( 0 );
  test.identical( got, false );

  test.case = 'src - number';
  var container = _.containerAdapter.make( new Set( [] ) );
  var got = container.same( 2 );
  test.identical( got, false );

  test.case = 'src - string';
  var container = _.containerAdapter.make( new Set( [] ) );
  var got = container.same( 'str' );
  test.identical( got, false );

  test.case = 'src - Map';
  var container = _.containerAdapter.make( new Set( [] ) );
  var got = container.same( new HashMap( [ [ 1, 2 ] ] ) );
  test.identical( got, false );

  test.case = 'src - map';
  var container = _.containerAdapter.make( new Set( [] ) );
  var got = container.same( { a : 0 } );
  test.identical( got, false );

  test.case = 'src - BufferTyped';
  var container = _.containerAdapter.make( new Set( [] ) );
  var got = container.same( new U8x( 5 ) );
  test.identical( got, false );

  test.case = 'src - array, not same';
  var src = [ 1, 2 ];
  var container = _.containerAdapter.make( new Set( [] ) );
  var got = container.same( src );
  test.identical( got, false );

  test.case = 'src - unroll, not same';
  var src = _.unroll.make( [ 1, 2 ] );
  var container = _.containerAdapter.make( _.unroll.make( new Set( [] ) ) );
  var got = container.same( src );
  test.identical( got, false );

  test.case = 'src - arrayContainerAdapter';
  var src = _.containerAdapter.make( [ 1, 2 ] );
  var container = _.containerAdapter.make( new Set( [] ) );
  var got = container.same( src );
  test.identical( got, false );

  /* */

  test.case = 'src - Set, same';
  var src = new Set( [ 1, 2 ] );
  var container = _.containerAdapter.make( src );
  var got = container.same( src );
  test.identical( got, true );

  test.case = 'src - Set, not same';
  var src = new Set( [ 1, 2 ] );
  var container = _.containerAdapter.make( new Set( [ 1, 2 ] ) );
  var got = container.same( src );
  test.identical( got, false );

  test.case = 'src - setContainerAdapter';
  var src = _.containerAdapter.make( new Set( [ 1, 2 ] ) );
  var container = _.containerAdapter.make( new Set( [ 1, 2 ] ) );
  var got = container.same( src );
  test.identical( got, false );

  test.case = 'src - setContainerAdapter, ';
  var src = _.containerAdapter.make( new Set( [ 1, 2 ] ) );
  var container = _.containerAdapter.make( src );
  var got = container.same( src );
  test.identical( got, false );

  test.case = 'src - src';
  var container = _.containerAdapter.make( new Set( [ 1, 2 ] ) );
  var got = container.same( container );
  test.identical( got, true );

  test.case = 'two containers with same src';
  var src = new Set( [ 1, 2, 3 ] );
  var src2 = _.containerAdapter.make( src );
  var container = _.containerAdapter.make( src );
  var got = container.same( src2 );
  test.identical( got, true );

  test.close( 'setContainerAdapter' );
}

//

function removedContainer( test )
{
  test.open( 'arrayContainerAdapter' );

  test.case = 'dst - empty container, src - empty array';
  var dst = _.containerAdapter.make( [] );
  var src = [];
  var got = dst.removedContainer( src );
  var exp = [];
  test.identical( got, 0 );
  test.identical( dst.original, exp );

  test.case = 'dst - empty container, src - empty Set';
  var dst = _.containerAdapter.make( [] );
  var src = new Set();
  var got = dst.removedContainer( src );
  var exp = [];
  test.identical( got, 0 );
  test.identical( dst.original, exp );

  test.case = 'dst - empty container, src - arrayContainerAdapter';
  var dst = _.containerAdapter.make( [] );
  var src = _.containerAdapter.make( [ 1, [ 2 ], { a : 3 } ] );
  var got = dst.removedContainer( src );
  var exp = [];
  test.identical( got, 0 );
  test.identical( dst.original, exp );

  test.case = 'dst - empty container, src - setContainerAdapter';
  var dst = _.containerAdapter.make( [] );
  var src = _.containerAdapter.make( new Set( [ 1, [ 2 ], { a : 3 } ] ) );
  var got = dst.removedContainer( src );
  var exp = [];
  test.identical( got, 0 );
  test.identical( dst.original, exp );

  /* */

  test.case = 'dst - container, src - empty array, no entries';
  var dst = _.containerAdapter.make( [ 'str', null, 0 ] );
  var src = [ 1, [ 2 ], { a : 3 } ];
  var got = dst.removedContainer( src );
  var exp = [ 'str', null, 0 ];
  test.identical( got, 0 );
  test.identical( dst.original, exp );

  test.case = 'dst - container, src - Set, no entries';
  var dst = _.containerAdapter.make( [ 'str', null, 0 ] );
  var src = new Set( [ 1, [ 2 ], { a : 3 } ] );
  var got = dst.removedContainer( src );
  var exp = [ 'str', null, 0 ];
  test.identical( got, 0 );
  test.identical( dst.original, exp );

  test.case = 'dst - container, src - arrayContainerAdapter, entries';
  var dst = _.containerAdapter.make( [ 1, 2, 'str', [ 2 ] ] );
  var src = _.containerAdapter.make( [ 1, [ 2 ], 2, 2, { a : 3 } ] );
  var got = dst.removedContainer( src );
  var exp = [ 'str', [ 2 ] ];
  test.identical( got, 2 );
  test.identical( dst.original, exp );

  test.case = 'dst - container, src - setContainerAdapter';
  var dst = _.containerAdapter.make( [ 1, 1, 2, 'str', [ 2 ], 1 ] );
  var src = _.containerAdapter.make( new Set( [ 1, [ 2 ], { a : 3 } ] ) );
  var got = dst.removedContainer( src );
  var exp = [ 2, 'str', [ 2 ] ];
  test.identical( got, 3 );
  test.identical( dst.original, exp );

  /* */

  test.case = 'dst === src';
  var dst = _.containerAdapter.make( [ 'str', null, 0, 1, [ 2 ], 'str', { a : 3 } ] );
  var src = dst;
  var got = dst.removedContainer( src );
  var exp = [];
  test.identical( got, 7 );
  test.identical( dst.original, exp );

  test.case = 'src identical to dst';
  var dst = _.containerAdapter.make( [ 'str', null, 0, 1, [ 2 ], 'str', { a : 3 } ] );
  var src = _.containerAdapter.make( [ 'str', null, 0, 1, [ 2 ], 'str', { a : 3 } ] );
  var got = dst.removedContainer( src );
  var exp = [ [ 2 ], { a : 3 } ];
  test.identical( got, 5 );
  test.identical( dst.original, exp );

  test.close( 'arrayContainerAdapter' );

  /* - */

  test.open( 'setContainerAdapter' );

  test.case = 'dst - empty container, src - empty array';
  var dst = _.containerAdapter.make( new Set() );
  var src = [];
  var got = dst.removedContainer( src );
  var exp = [];
  test.identical( got, 0 );
  test.identical( [ ... dst.original ], exp );

  test.case = 'dst - empty container, src - empty Set';
  var dst = _.containerAdapter.make( new Set() );
  var src = new Set();
  var got = dst.removedContainer( src );
  var exp = [];
  test.identical( got, 0 );
  test.identical( [ ... dst.original ], exp );

  test.case = 'dst - empty container, src - arrayContainerAdapter';
  var dst = _.containerAdapter.make( new Set() );
  var src = _.containerAdapter.make( [ 1, [ 2 ], { a : 3 } ] );
  var got = dst.removedContainer( src );
  var exp = [];
  test.identical( got, 0 );
  test.identical( [ ... dst.original ], exp );

  test.case = 'dst - empty container, src - setContainerAdapter';
  var dst = _.containerAdapter.make( new Set() );
  var src = _.containerAdapter.make( new Set( [ 1, [ 2 ], { a : 3 } ] ) );
  var got = dst.removedContainer( src );
  var exp = [];
  test.identical( got, 0 );
  test.identical( [ ... dst.original ], exp );

  /* */

  test.case = 'dst - container, src - empty array, no entries';
  var dst = _.containerAdapter.make( new Set( [ 'str', null, 0 ] ) );
  var src = [ 1, [ 2 ], { a : 3 } ];
  var got = dst.removedContainer( src );
  var exp = [ 'str', null, 0 ];
  test.identical( got, 0 );
  test.identical( [ ... dst.original ], exp );

  test.case = 'dst - container, src - Set, no entries';
  var dst = _.containerAdapter.make( new Set( [ 'str', null, 0 ] ) );
  var src = new Set( [ 1, [ 2 ], { a : 3 } ] );
  var got = dst.removedContainer( src );
  var exp = [ 'str', null, 0 ];
  test.identical( got, 0 );
  test.identical( [ ... dst.original ], exp );

  test.case = 'dst - container, src - arrayContainerAdapter, entries';
  var dst = _.containerAdapter.make( new Set( [ 1, 2, 'str', [ 2 ] ] ) );
  var src = _.containerAdapter.make( [ 1, [ 2 ], { a : 3 }, 'str' ] );
  var got = dst.removedContainer( src );
  var exp = [ 2, [ 2 ] ];
  test.identical( got, 2 );
  test.identical( [ ... dst.original ], exp );

  test.case = 'dst - container, src - setContainerAdapter';
  var dst = _.containerAdapter.make( new Set( [ 1, 2, 'str', [ 2 ] ] ) );
  var src = _.containerAdapter.make( new Set( [ 1, [ 2 ], { a : 3 } ] ) );
  var got = dst.removedContainer( src );
  var exp = [ 2, 'str', [ 2 ] ];
  test.identical( got, 1 );
  test.identical( [ ... dst.original ], exp );

  /* */

  test.case = 'dst === src';
  var dst = _.containerAdapter.make( new Set( [ 'str', null, 0, 1, [ 2 ], 'str', { a : 3 } ] ) );
  var src = dst;
  var got = dst.removedContainer( src );
  var exp = [];
  test.identical( got, 6 );
  test.identical( [ ... dst.original ], exp );

  test.case = 'src identical to dst';
  var dst = _.containerAdapter.make( new Set( [ 'str', null, 0, 1, [ 2 ], 'str', { a : 3 } ] ) );
  var src = _.containerAdapter.make( [ 'str', null, 0, 1, [ 2 ], 'str', { a : 3 } ] );
  var got = dst.removedContainer( src );
  var exp = [ [ 2 ], { a : 3 } ];
  test.identical( got, 4 );
  test.identical( [ ... dst.original ], exp );

  test.close( 'setContainerAdapter' );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () =>
  {
    var dst = _.containerAdapter.make( [] );
    dst.removedContainer();
  });

  test.case = 'wrong type of container';
  test.shouldThrowErrorSync( () =>
  {
    var dst = _.containerAdapter.make( [] );
    dst.removedContainer( 0 );
  });

  test.shouldThrowErrorSync( () =>
  {
    var dst = _.containerAdapter.make( [] );
    dst.removedContainer( new BufferRaw() );
  });

  test.shouldThrowErrorSync( () =>
  {
    var dst = _.containerAdapter.make( [] );
    dst.removedContainer( 'wrong' );
  });
}

//

function removedContainerOnce( test )
{
  test.open( 'arrayContainerAdapter' );

  test.case = 'dst - empty container, src - empty array';
  var dst = _.containerAdapter.make( [] );
  var src = [];
  var got = dst.removedContainerOnce( src );
  var exp = [];
  test.identical( got, 0 );
  test.identical( dst.original, exp );

  test.case = 'dst - empty container, src - empty Set';
  var dst = _.containerAdapter.make( [] );
  var src = new Set();
  var got = dst.removedContainerOnce( src );
  var exp = [];
  test.identical( got, 0 );
  test.identical( dst.original, exp );

  test.case = 'dst - empty container, src - arrayContainerAdapter';
  var dst = _.containerAdapter.make( [] );
  var src = _.containerAdapter.make( [ 1, [ 2 ], { a : 3 } ] );
  var got = dst.removedContainerOnce( src );
  var exp = [];
  test.identical( got, 0 );
  test.identical( dst.original, exp );

  test.case = 'dst - empty container, src - setContainerAdapter';
  var dst = _.containerAdapter.make( [] );
  var src = _.containerAdapter.make( new Set( [ 1, [ 2 ], { a : 3 } ] ) );
  var got = dst.removedContainerOnce( src );
  var exp = [];
  test.identical( got, 0 );
  test.identical( dst.original, exp );

  /* */

  test.case = 'dst - container, src - empty array, no entries';
  var dst = _.containerAdapter.make( [ 'str', null, 0 ] );
  var src = [ 1, [ 2 ], { a : 3 } ];
  var got = dst.removedContainerOnce( src );
  var exp = [ 'str', null, 0 ];
  test.identical( got, 0 );
  test.identical( dst.original, exp );

  test.case = 'dst - container, src - Set, no entries';
  var dst = _.containerAdapter.make( [ 'str', null, 0 ] );
  var src = new Set( [ 1, [ 2 ], { a : 3 } ] );
  var got = dst.removedContainerOnce( src );
  var exp = [ 'str', null, 0 ];
  test.identical( got, 0 );
  test.identical( dst.original, exp );

  test.case = 'dst - container, src - arrayContainerAdapter, entries';
  var dst = _.containerAdapter.make( [ 1, 1, 2, 'str', [ 2 ] ] );
  var src = _.containerAdapter.make( [ 1, [ 2 ], { a : 3 } ] );
  var got = dst.removedContainerOnce( src );
  var exp = [ 1, 2, 'str', [ 2 ] ];
  test.identical( got, 1 );
  test.identical( dst.original, exp );

  test.case = 'dst - container, src - setContainerAdapter';
  var dst = _.containerAdapter.make( [ 1, 1, 2, 'str', [ 2 ] ] );
  var src = _.containerAdapter.make( new Set( [ 1, [ 2 ], 2, { a : 3 } ] ) );
  var got = dst.removedContainerOnce( src );
  var exp = [ 1, 'str', [ 2 ] ];
  test.identical( got, 2 );
  test.identical( dst.original, exp );

  /* */

  test.case = 'dst === src';
  var dst = _.containerAdapter.make( [ 'str', null, 0, 1, [ 2 ], 'str', { a : 3 } ] );
  var src = dst;
  var got = dst.removedContainerOnce( src );
  var exp = [];
  test.identical( got, 7 );
  test.identical( dst.original, exp );

  test.case = 'src identical to dst';
  var dst = _.containerAdapter.make( [ 'str', null, 0, 1, [ 2 ], 'str', { a : 3 } ] );
  var src = _.containerAdapter.make( [ 'str', null, 0, 1, [ 2 ], 'str', { a : 3 } ] );
  var got = dst.removedContainerOnce( src );
  var exp = [ [ 2 ], { a : 3 } ];
  test.identical( got, 5 );
  test.identical( dst.original, exp );

  /* */

  test.case = 'dst and src has arrays, one evaluator';
  var dst = _.containerAdapter.make( [ 'str', 0, 1, [ 2 ], [ 3 ], [ 2 ], 'str', { a : 3 } ] );
  var src = _.containerAdapter.make( [ 'str', 0, 1, [ 2 ], 'str', { a : 3 } ] );
  var got = dst.removedContainerOnce( src, ( e ) => e[ 0 ] );
  var exp = [ [ 3 ], [ 2 ] ];
  test.identical( got, 6 );
  test.identical( dst.original, exp );

  test.case = 'dst and src has arrays, two evaluators';
  var dst = _.containerAdapter.make( [ 'str', 0, 1, [ 2 ], [ 3 ], [ 2 ], 'str', { a : 3 } ] );
  var src = _.containerAdapter.make( [ 'str', 0, 1, [ 2 ], 'str', { a : 3 } ] );
  var got = dst.removedContainerOnce( src, ( e ) => e[ 0 ], ( ins ) => ins[ 0 ] );
  var exp = [ [ 3 ], [ 2 ] ];
  test.identical( got, 6 );
  test.identical( dst.original, exp );

  test.case = 'dst and src has arrays, fromIndex and evaluator2';
  var dst = _.containerAdapter.make( [ 'str', 0, 1, [ 2 ], [ 3 ], [ 2 ], 'str', { a : 3 } ] );
  var src = _.containerAdapter.make( [ 'str', 0, 1, [ 2 ], 'str', { a : 3 } ] );
  var got = dst.removedContainerOnce( src, 3, ( e ) => e[ 0 ] );
  var exp = [ 'str', 0, 1, [ 3 ], [ 2 ] ];
  test.identical( got, 3 );
  test.identical( dst.original, exp );

  test.case = 'dst and src has arrays, two evaluators';
  var dst = _.containerAdapter.make( [ 'str', 0, 1, [ 2 ], [ 3 ], [ 2 ], 'str', { a : 3 } ] );
  var src = _.containerAdapter.make( [ 'str', 0, 1, [ 2 ], 'str', { a : 3 } ] );
  var got = dst.removedContainerOnce( src, ( e, ins ) => e[ 0 ] === ins[ 0 ] );
  var exp = [ [ 3 ], [ 2 ] ];
  test.identical( got, 6 );
  test.identical( dst.original, exp );

  test.close( 'arrayContainerAdapter' );

  /* - */

  test.open( 'setContainerAdapter' );

  test.case = 'dst - empty container, src - empty array';
  var dst = _.containerAdapter.make( new Set() );
  var src = [];
  var got = dst.removedContainerOnce( src );
  var exp = [];
  test.identical( got, 0 );
  test.identical( [ ... dst.original ], exp );

  test.case = 'dst - empty container, src - empty Set';
  var dst = _.containerAdapter.make( new Set() );
  var src = new Set();
  var got = dst.removedContainerOnce( src );
  var exp = [];
  test.identical( got, 0 );
  test.identical( [ ... dst.original ], exp );

  test.case = 'dst - empty container, src - arrayContainerAdapter';
  var dst = _.containerAdapter.make( new Set() );
  var src = _.containerAdapter.make( [ 1, [ 2 ], { a : 3 } ] );
  var got = dst.removedContainerOnce( src );
  var exp = [];
  test.identical( got, 0 );
  test.identical( [ ... dst.original ], exp );

  test.case = 'dst - empty container, src - setContainerAdapter';
  var dst = _.containerAdapter.make( new Set() );
  var src = _.containerAdapter.make( new Set( [ 1, [ 2 ], { a : 3 } ] ) );
  var got = dst.removedContainerOnce( src );
  var exp = [];
  test.identical( got, 0 );
  test.identical( [ ... dst.original ], exp );

  /* */

  test.case = 'dst - container, src - empty array, no entries';
  var dst = _.containerAdapter.make( new Set( [ 'str', null, 0 ] ) );
  var src = [ 1, [ 2 ], { a : 3 } ];
  var got = dst.removedContainerOnce( src );
  var exp = [ 'str', null, 0 ];
  test.identical( got, 0 );
  test.identical( [ ... dst.original ], exp );

  test.case = 'dst - container, src - Set, no entries';
  var dst = _.containerAdapter.make( new Set( [ 'str', null, 0 ] ) );
  var src = new Set( [ 1, [ 2 ], { a : 3 } ] );
  var got = dst.removedContainerOnce( src );
  var exp = [ 'str', null, 0 ];
  test.identical( got, 0 );
  test.identical( [ ... dst.original ], exp );

  test.case = 'dst - container, src - arrayContainerAdapter, entries';
  var dst = _.containerAdapter.make( new Set( [ 1, 1, 2, 'str', [ 2 ] ] ) );
  var src = _.containerAdapter.make( [ 1, [ 2 ], { a : 3 } ] );
  var got = dst.removedContainerOnce( src );
  var exp = [ 2, 'str', [ 2 ] ];
  test.identical( got, 1 );
  test.identical( [ ... dst.original ], exp );

  test.case = 'dst - container, src - setContainerAdapter';
  var dst = _.containerAdapter.make( new Set( [ 1, 1, 2, 'str', [ 2 ] ] ) );
  var src = _.containerAdapter.make( new Set( [ 1, [ 2 ], 'str', { a : 3 } ] ) );
  var got = dst.removedContainerOnce( src );
  var exp = [ 2, [ 2 ] ];
  test.identical( got, 2 );
  test.identical( [ ... dst.original ], exp );

  /* */

  test.case = 'dst === src';
  var dst = _.containerAdapter.make( new Set( [ 'str', null, 0, 1, [ 2 ], 'str', { a : 3 } ] ) );
  var src = dst;
  var got = dst.removedContainerOnce( src );
  var exp = [];
  test.identical( got, 6 );
  test.identical( [ ... dst.original ], exp );

  test.case = 'src identical to dst';
  var dst = _.containerAdapter.make( new Set( [ 'str', null, 0, 1, [ 2 ], 'str', { a : 3 } ] ) );
  var src = _.containerAdapter.make( [ 'str', null, 0, 1, [ 2 ], 'str', { a : 3 } ] );
  var got = dst.removedContainerOnce( src );
  var exp = [ [ 2 ], { a : 3 } ];
  test.identical( got, 4 );
  test.identical( [ ... dst.original ], exp );

  /* */

  test.case = 'dst and src has arrays, one evaluator';
  var dst = _.containerAdapter.make( new Set( [ 'str', 0, 1, [ 2 ], [ 3 ], [ 2 ], 'str', { a : 3 } ] ) );
  var src = _.containerAdapter.make( new Set( [ 'str', 0, 1, [ 2 ], 'str', { a : 3 } ] ) );
  var got = dst.removedContainerOnce( src, ( e ) => e[ 0 ] );
  var exp = [ [ 3 ], [ 2 ] ];
  test.identical( got, 5 );
  test.identical( [ ... dst.original ], exp );

  test.case = 'dst and src has arrays, two evaluators';
  var dst = _.containerAdapter.make( new Set( [ 'str', 0, 1, [ 2 ], [ 3 ], [ 2 ], 'str', { a : 3 } ] ) );
  var src = _.containerAdapter.make( new Set( [ 'str', 0, 1, [ 2 ], 'str', { a : 3 } ] ) );
  var got = dst.removedContainerOnce( src, ( e ) => e[ 0 ], ( ins ) => ins[ 0 ] );
  var exp = [ [ 3 ], [ 2 ] ];
  test.identical( got, 5 );
  test.identical( [ ... dst.original ], exp );

  test.case = 'dst and src has arrays, fromIndex and evaluator2';
  var dst = _.containerAdapter.make( new Set( [ 'str', 0, 1, [ 2 ], [ 3 ], [ 2 ], 'str', { a : 3 } ] ) );
  var src = _.containerAdapter.make( new Set( [ 'str', 0, 1, [ 2 ], 'str', { a : 3 } ] ) );
  var got = dst.removedContainerOnce( src, 3, ( e ) => e[ 0 ] );
  var exp = [ 'str', 0, 1, [ 3 ], [ 2 ] ];
  test.identical( got, 2 );
  test.identical( [ ... dst.original ], exp );

  test.case = 'dst and src has arrays, two evaluators';
  var dst = _.containerAdapter.make( new Set( [ 'str', 0, 1, [ 2 ], [ 3 ], [ 2 ], 'str', { a : 3 } ] ) );
  var src = _.containerAdapter.make( new Set( [ 'str', 0, 1, [ 2 ], 'str', { a : 3 } ] ) );
  var got = dst.removedContainerOnce( src, ( e, ins ) => e[ 0 ] === ins[ 0 ] );
  var exp = [ [ 3 ], [ 2 ] ];
  test.identical( got, 5 );
  test.identical( [ ... dst.original ], exp );

  test.close( 'setContainerAdapter' );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () =>
  {
    var dst = _.containerAdapter.make( [] );
    dst.removedContainerOnce();
  });

  test.case = 'wrong type of container';
  test.shouldThrowErrorSync( () =>
  {
    var dst = _.containerAdapter.make( [] );
    dst.removedContainerOnce( 0 );
  });

  test.shouldThrowErrorSync( () =>
  {
    var dst = _.containerAdapter.make( [] );
    dst.removedContainerOnce( new BufferRaw() );
  });

  test.shouldThrowErrorSync( () =>
  {
    var dst = _.containerAdapter.make( [] );
    dst.removedContainerOnce( 'wrong' );
  });
}

//

function removedContainerOnceStrictly( test )
{
  test.open( 'arrayContainerAdapter' );

  test.case = 'dst - container, src - arrayContainerAdapter, entries';
  var dst = _.containerAdapter.make( [ 1, 2, 'str', [ 2 ] ] );
  var src = _.containerAdapter.make( [ 1, 2, 'str' ] );
  var got = dst.removedContainerOnceStrictly( src );
  var exp = [ [ 2 ] ];
  test.identical( got, 3 );
  test.identical( dst.original, exp );

  test.case = 'dst - container, src - setContainerAdapter';
  var dst = _.containerAdapter.make( [ 1, 2, 'str', [ 2 ] ] );
  var src = _.containerAdapter.make( new Set( [ 1, 2 ] ) );
  var got = dst.removedContainerOnceStrictly( src );
  var exp = [ 'str', [ 2 ] ];
  test.identical( got, 2 );
  test.identical( dst.original, exp );

  /* */

  test.case = 'dst === src';
  var dst = _.containerAdapter.make( [ 'str', null, 0, 1, [ 2 ], { a : 3 } ] );
  var src = dst;
  var got = dst.removedContainerOnceStrictly( src );
  var exp = [];
  test.identical( got, 6 );
  test.identical( dst.original, exp );

  test.case = 'src identical to dst';
  var dst = _.containerAdapter.make( [ 'str', null, 0, 1, [ 2 ], { a : 3 } ] );
  var src = _.containerAdapter.make( [ 'str', null, 0, 1 ] );
  var got = dst.removedContainerOnceStrictly( src );
  var exp = [ [ 2 ], { a : 3 } ];
  test.identical( got, 4 );
  test.identical( dst.original, exp );

  /* */

  test.case = 'dst and src has arrays, one evaluator';
  var dst = _.containerAdapter.make( [ 'str', 0, 1, [ 2 ], [ 3 ], [ 4 ], 'str', { a : 3 } ] );
  var src = _.containerAdapter.make( [ [ 2 ], [ 4 ] ] );
  var got = dst.removedContainerOnceStrictly( src, ( e ) => e[ 0 ] );
  var exp = [ 'str', 0, 1, [ 3 ], 'str', { a : 3 } ];
  test.identical( got, 2 );
  test.identical( dst.original, exp );

  test.case = 'dst and src has arrays, two evaluators';
  var dst = _.containerAdapter.make( [ 'str', 0, 1, [ 2 ], [ 3 ], [ 4 ], 'str', { a : 3 } ] );
  var src = _.containerAdapter.make( [ [ 2 ], [ 4 ] ] );
  var got = dst.removedContainerOnceStrictly( src, ( e ) => e[ 0 ], ( ins ) => ins[ 0 ] );
  var exp = [ 'str', 0, 1, [ 3 ], 'str', { a : 3 } ];
  test.identical( got, 2 );
  test.identical( dst.original, exp );

  test.case = 'dst and src has arrays, fromIndex and evaluator2';
  var dst = _.containerAdapter.make( [ 'str', 0, 1, [ 2 ], [ 3 ], [ 2 ], 'str', { a : 3 } ] );
  var src = _.containerAdapter.make( [ [ 2 ], [ 3 ] ] );
  var got = dst.removedContainerOnceStrictly( src, 4, ( e ) => e[ 0 ] );
  var exp = [ 'str', 0, 1, [ 2 ], 'str', { a : 3 }  ];
  test.identical( got, 2 );
  test.identical( dst.original, exp );

  test.case = 'dst and src has arrays, two evaluators';
  var dst = _.containerAdapter.make( [ 'str', 0, 1, [ 2 ], [ 3 ], [ 4 ], 'str', { a : 3 } ] );
  var src = _.containerAdapter.make( [ [ 2 ], [ 4 ] ] );
  var got = dst.removedContainerOnceStrictly( src, ( e, ins ) => e[ 0 ] === ins[ 0 ] );
  var exp = [ 'str', 0, 1, [ 3 ], 'str', { a : 3 } ];
  test.identical( got, 2 );
  test.identical( dst.original, exp );

  test.close( 'arrayContainerAdapter' );

  /* - */

  test.open( 'setContainerAdapter' );

  test.case = 'dst - container, src - arrayContainerAdapter, entries';
  var dst = _.containerAdapter.make( new Set( [ 1, 2, 'str', [ 2 ] ] ) );
  var src = _.containerAdapter.make( [ 1 ] );
  var got = dst.removedContainerOnceStrictly( src );
  var exp = [ 2, 'str', [ 2 ] ];
  test.identical( got, 1 );
  test.identical( [ ... dst.original ], exp );

  test.case = 'dst - container, src - setContainerAdapter';
  var dst = _.containerAdapter.make( new Set( [ 1, 2, 'str', [ 2 ] ] ) );
  var src = _.containerAdapter.make( new Set( [ 1 ] ) );
  var got = dst.removedContainerOnceStrictly( src );
  var exp = [ 2, 'str', [ 2 ] ];
  test.identical( got, 1 );
  test.identical( [ ... dst.original ], exp );

  /* */

  test.case = 'dst === src';
  var dst = _.containerAdapter.make( new Set( [ 'str', null, 0, 1, [ 2 ], 'str', { a : 3 } ] ) );
  var src = dst;
  var got = dst.removedContainerOnceStrictly( src );
  var exp = [];
  test.identical( got, 6 );
  test.identical( [ ... dst.original ], exp );

  test.case = 'src identical to dst';
  var dst = _.containerAdapter.make( new Set( [ 'str', null, 0, 1, [ 2 ], { a : 3 } ] ) );
  var src = _.containerAdapter.make( [ 'str', null, 0, 1 ] );
  var got = dst.removedContainerOnceStrictly( src );
  var exp = [ [ 2 ], { a : 3 } ];
  test.identical( got, 4 );
  test.identical( [ ... dst.original ], exp );

  /* */

  test.case = 'dst and src has arrays, one evaluator';
  var dst = _.containerAdapter.make( new Set( [ 'str', 0, 1, [ 2 ], [ 3 ], [ 4 ], 'str', { a : 3 } ] ) );
  var src = _.containerAdapter.make( new Set( [ [ 2 ], [ 4 ] ] ) );
  var got = dst.removedContainerOnceStrictly( src, ( e ) => e[ 0 ] );
  var exp = [ 'str', 0, 1, [ 3 ], { a : 3 } ];
  test.identical( got, 2 );
  test.identical( [ ... dst.original ], exp );

  test.case = 'dst and src has arrays, two evaluators';
  var dst = _.containerAdapter.make( new Set( [ 'str', 0, 1, [ 2 ], [ 3 ], [ 4 ], 'str', { a : 3 } ] ) );
  var src = _.containerAdapter.make( new Set( [ [ 2 ], [ 4 ] ] ) );
  var got = dst.removedContainerOnceStrictly( src, ( e ) => e[ 0 ], ( ins ) => ins[ 0 ] );
  var exp = [ 'str', 0, 1, [ 3 ], { a : 3 } ];
  test.identical( got, 2 );
  test.identical( [ ... dst.original ], exp );

  test.case = 'dst and src has arrays, fromIndex and evaluator2';
  var dst = _.containerAdapter.make( new Set( [ 'str', 0, 1, [ 2 ], [ 3 ], [ 2 ], 'str', { a : 3 } ] ) );
  var src = _.containerAdapter.make( new Set( [ [ 2 ], [ 3 ] ] ) );
  var got = dst.removedContainerOnceStrictly( src, 4, ( e ) => e[ 0 ] );
  var exp = [ 'str', 0, 1, [ 2 ], { a : 3 }  ];
  test.identical( got, 2 );
  test.identical( [ ... dst.original ], exp );

  test.case = 'dst and src has arrays, two evaluators';
  var dst = _.containerAdapter.make( new Set( [ 'str', 0, 1, [ 2 ], [ 3 ], [ 4 ], 'str', { a : 3 } ] ) );
  var src = _.containerAdapter.make( new Set( [ [ 2 ], [ 4 ] ] ) );
  var got = dst.removedContainerOnceStrictly( src, ( e, ins ) => e[ 0 ] === ins[ 0 ] );
  var exp = [ 'str', 0, 1, [ 3 ], { a : 3 } ];
  test.identical( got, 2 );
  test.identical( [ ... dst.original ], exp );

  test.close( 'setContainerAdapter' );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () =>
  {
    var dst = _.containerAdapter.make( [] );
    dst.removedContainerOnceStrictly();
  });

  test.case = 'empty container';
  test.shouldThrowErrorSync( () =>
  {
    var dst = _.containerAdapter.make( [] );
    dst.removedContainerOnceStrictly( [ 1 ] );
  });

  test.case = 'a few entries';
  test.shouldThrowErrorSync( () =>
  {
    var dst = _.containerAdapter.make( [ 1, 1 ] );
    dst.removedContainerOnceStrictly( [ 1 ] );
  });

  test.case = 'no entries';
  test.shouldThrowErrorSync( () =>
  {
    var dst = _.containerAdapter.make( [ 2, 2 ] );
    dst.removedContainerOnceStrictly( [ 1 ] );
  });

  test.case = 'wrong type of container';
  test.shouldThrowErrorSync( () =>
  {
    var dst = _.containerAdapter.make( [] );
    dst.removedContainerOnceStrictly( 0 );
  });

  test.shouldThrowErrorSync( () =>
  {
    var dst = _.containerAdapter.make( [] );
    dst.removedContainerOnceStrictly( new BufferRaw() );
  });

  test.shouldThrowErrorSync( () =>
  {
    var dst = _.containerAdapter.make( [] );
    dst.removedContainerOnceStrictly( 'wrong' );
  });
}

//

function removeContainer( test )
{
  test.open( 'arrayContainerAdapter' );

  test.case = 'dst - empty container, src - empty array';
  var dst = _.containerAdapter.make( [] );
  var src = [];
  var got = dst.removeContainer( src );
  var exp = [];
  test.true( got === dst );
  test.identical( got.original, exp );

  test.case = 'dst - empty container, src - empty Set';
  var dst = _.containerAdapter.make( [] );
  var src = new Set();
  var got = dst.removeContainer( src );
  var exp = [];
  test.true( got === dst );
  test.identical( got.original, exp );

  test.case = 'dst - empty container, src - arrayContainerAdapter';
  var dst = _.containerAdapter.make( [] );
  var src = _.containerAdapter.make( [ 1, [ 2 ], { a : 3 } ] );
  var got = dst.removeContainer( src );
  var exp = [];
  test.true( got === dst );
  test.identical( got.original, exp );

  test.case = 'dst - empty container, src - setContainerAdapter';
  var dst = _.containerAdapter.make( [] );
  var src = _.containerAdapter.make( new Set( [ 1, [ 2 ], { a : 3 } ] ) );
  var got = dst.removeContainer( src );
  var exp = [];
  test.true( got === dst );
  test.identical( got.original, exp );

  /* */

  test.case = 'dst - container, src - empty array, no entries';
  var dst = _.containerAdapter.make( [ 'str', null, 0 ] );
  var src = [ 1, [ 2 ], { a : 3 } ];
  var got = dst.removeContainer( src );
  var exp = [ 'str', null, 0 ];
  test.true( got === dst );
  test.identical( got.original, exp );

  test.case = 'dst - container, src - Set, no entries';
  var dst = _.containerAdapter.make( [ 'str', null, 0 ] );
  var src = new Set( [ 1, [ 2 ], { a : 3 } ] );
  var got = dst.removeContainer( src );
  var exp = [ 'str', null, 0 ];
  test.true( got === dst );
  test.identical( got.original, exp );

  test.case = 'dst - container, src - arrayContainerAdapter, entries';
  var dst = _.containerAdapter.make( [ 1, 2, 'str', [ 2 ] ] );
  var src = _.containerAdapter.make( [ 1, [ 2 ], { a : 3 } ] );
  var got = dst.removeContainer( src );
  var exp = [ 2, 'str', [ 2 ] ];
  test.true( got === dst );
  test.identical( got.original, exp );

  test.case = 'dst - container, src - setContainerAdapter';
  var dst = _.containerAdapter.make( [ 1, 2, 'str', [ 2 ] ] );
  var src = _.containerAdapter.make( new Set( [ 1, [ 2 ], { a : 3 } ] ) );
  var got = dst.removeContainer( src );
  var exp = [ 2, 'str', [ 2 ] ];
  test.true( got === dst );
  test.identical( got.original, exp );

  /* */

  test.case = 'dst === src';
  var dst = _.containerAdapter.make( [ 'str', null, 0, 1, [ 2 ], 'str', { a : 3 } ] );
  var src = dst;
  var got = dst.removeContainer( src );
  var exp = [];
  test.true( got === dst );
  test.true( got === src );
  test.identical( got.original, exp );

  test.case = 'src identical to dst';
  var dst = _.containerAdapter.make( [ 'str', null, 0, 1, [ 2 ], 'str', { a : 3 } ] );
  var src = _.containerAdapter.make( [ 'str', null, 0, 1, [ 2 ], 'str', { a : 3 } ] );
  var got = dst.removeContainer( src );
  var exp = [ [ 2 ], { a : 3 } ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.close( 'arrayContainerAdapter' );

  /* - */

  test.open( 'setContainerAdapter' );

  test.case = 'dst - empty container, src - empty array';
  var dst = _.containerAdapter.make( new Set() );
  var src = [];
  var got = dst.removeContainer( src );
  var exp = [];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'dst - empty container, src - empty Set';
  var dst = _.containerAdapter.make( new Set() );
  var src = new Set();
  var got = dst.removeContainer( src );
  var exp = [];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'dst - empty container, src - arrayContainerAdapter';
  var dst = _.containerAdapter.make( new Set() );
  var src = _.containerAdapter.make( [ 1, [ 2 ], { a : 3 } ] );
  var got = dst.removeContainer( src );
  var exp = [];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'dst - empty container, src - setContainerAdapter';
  var dst = _.containerAdapter.make( new Set() );
  var src = _.containerAdapter.make( new Set( [ 1, [ 2 ], { a : 3 } ] ) );
  var got = dst.removeContainer( src );
  var exp = [];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  /* */

  test.case = 'dst - container, src - empty array, no entries';
  var dst = _.containerAdapter.make( new Set( [ 'str', null, 0 ] ) );
  var src = [ 1, [ 2 ], { a : 3 } ];
  var got = dst.removeContainer( src );
  var exp = [ 'str', null, 0 ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'dst - container, src - Set, no entries';
  var dst = _.containerAdapter.make( new Set( [ 'str', null, 0 ] ) );
  var src = new Set( [ 1, [ 2 ], { a : 3 } ] );
  var got = dst.removeContainer( src );
  var exp = [ 'str', null, 0 ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'dst - container, src - arrayContainerAdapter, entries';
  var dst = _.containerAdapter.make( new Set( [ 1, 2, 'str', [ 2 ] ] ) );
  var src = _.containerAdapter.make( [ 1, [ 2 ], { a : 3 } ] );
  var got = dst.removeContainer( src );
  var exp = [ 2, 'str', [ 2 ] ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'dst - container, src - setContainerAdapter';
  var dst = _.containerAdapter.make( new Set( [ 1, 2, 'str', [ 2 ] ] ) );
  var src = _.containerAdapter.make( new Set( [ 1, [ 2 ], { a : 3 } ] ) );
  var got = dst.removeContainer( src );
  var exp = [ 2, 'str', [ 2 ] ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  /* */

  test.case = 'dst === src';
  var dst = _.containerAdapter.make( new Set( [ 'str', null, 0, 1, [ 2 ], 'str', { a : 3 } ] ) );
  var src = dst;
  var got = dst.removeContainer( src );
  var exp = [];
  test.true( got === dst );
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src identical to dst';
  var dst = _.containerAdapter.make( new Set( [ 'str', null, 0, 1, [ 2 ], 'str', { a : 3 } ] ) );
  var src = _.containerAdapter.make( [ 'str', null, 0, 1, [ 2 ], 'str', { a : 3 } ] );
  var got = dst.removeContainer( src );
  var exp = [ [ 2 ], { a : 3 } ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.close( 'setContainerAdapter' );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () =>
  {
    var dst = _.containerAdapter.make( [] );
    dst.removeContainer();
  });

  test.case = 'wrong type of container';
  test.shouldThrowErrorSync( () =>
  {
    var dst = _.containerAdapter.make( [] );
    dst.removeContainer( 0 );
  });

  test.shouldThrowErrorSync( () =>
  {
    var dst = _.containerAdapter.make( [] );
    dst.removeContainer( new BufferRaw() );
  });

  test.shouldThrowErrorSync( () =>
  {
    var dst = _.containerAdapter.make( [] );
    dst.removeContainer( 'wrong' );
  });
}

//

function removeContainerOnce( test )
{
  test.open( 'arrayContainerAdapter' );

  test.case = 'dst - empty container, src - empty array';
  var dst = _.containerAdapter.make( [] );
  var src = [];
  var got = dst.removeContainerOnce( src );
  var exp = [];
  test.true( got === dst );
  test.identical( got.original, exp );

  test.case = 'dst - empty container, src - empty Set';
  var dst = _.containerAdapter.make( [] );
  var src = new Set();
  var got = dst.removeContainerOnce( src );
  var exp = [];
  test.true( got === dst );
  test.identical( got.original, exp );

  test.case = 'dst - empty container, src - arrayContainerAdapter';
  var dst = _.containerAdapter.make( [] );
  var src = _.containerAdapter.make( [ 1, [ 2 ], { a : 3 } ] );
  var got = dst.removeContainerOnce( src );
  var exp = [];
  test.true( got === dst );
  test.identical( got.original, exp );

  test.case = 'dst - empty container, src - setContainerAdapter';
  var dst = _.containerAdapter.make( [] );
  var src = _.containerAdapter.make( new Set( [ 1, [ 2 ], { a : 3 } ] ) );
  var got = dst.removeContainerOnce( src );
  var exp = [];
  test.true( got === dst );
  test.identical( got.original, exp );

  /* */

  test.case = 'dst - container, src - empty array, no entries';
  var dst = _.containerAdapter.make( [ 'str', null, 0 ] );
  var src = [ 1, [ 2 ], { a : 3 } ];
  var got = dst.removeContainerOnce( src );
  var exp = [ 'str', null, 0 ];
  test.true( got === dst );
  test.identical( got.original, exp );

  test.case = 'dst - container, src - Set, no entries';
  var dst = _.containerAdapter.make( [ 'str', null, 0 ] );
  var src = new Set( [ 1, [ 2 ], { a : 3 } ] );
  var got = dst.removeContainerOnce( src );
  var exp = [ 'str', null, 0 ];
  test.true( got === dst );
  test.identical( got.original, exp );

  test.case = 'dst - container, src - arrayContainerAdapter, entries';
  var dst = _.containerAdapter.make( [ 1, 1, 2, 'str', [ 2 ] ] );
  var src = _.containerAdapter.make( [ 1, [ 2 ], { a : 3 } ] );
  var got = dst.removeContainerOnce( src );
  var exp = [ 1, 2, 'str', [ 2 ] ];
  test.true( got === dst );
  test.identical( got.original, exp );

  test.case = 'dst - container, src - setContainerAdapter';
  var dst = _.containerAdapter.make( [ 1, 1, 2, 'str', [ 2 ] ] );
  var src = _.containerAdapter.make( new Set( [ 1, [ 2 ], { a : 3 } ] ) );
  var got = dst.removeContainerOnce( src );
  var exp = [ 1, 2, 'str', [ 2 ] ];
  test.true( got === dst );
  test.identical( got.original, exp );

  /* */

  test.case = 'dst === src';
  var dst = _.containerAdapter.make( [ 'str', null, 0, 1, [ 2 ], 'str', { a : 3 } ] );
  var src = dst;
  var got = dst.removeContainerOnce( src );
  var exp = [];
  test.true( got === dst );
  test.true( got === src );
  test.identical( got.original, exp );

  test.case = 'src identical to dst';
  var dst = _.containerAdapter.make( [ 'str', null, 0, 1, [ 2 ], 'str', { a : 3 } ] );
  var src = _.containerAdapter.make( [ 'str', null, 0, 1, [ 2 ], 'str', { a : 3 } ] );
  var got = dst.removeContainerOnce( src );
  var exp = [ [ 2 ], { a : 3 } ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  /* */

  test.case = 'dst and src has arrays, one evaluator';
  var dst = _.containerAdapter.make( [ 'str', 0, 1, [ 2 ], [ 3 ], [ 2 ], 'str', { a : 3 } ] );
  var src = _.containerAdapter.make( [ 'str', 0, 1, [ 2 ], 'str', { a : 3 } ] );
  var got = dst.removeContainerOnce( src, ( e ) => e[ 0 ] );
  var exp = [ [ 3 ], [ 2 ] ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'dst and src has arrays, two evaluators';
  var dst = _.containerAdapter.make( [ 'str', 0, 1, [ 2 ], [ 3 ], [ 2 ], 'str', { a : 3 } ] );
  var src = _.containerAdapter.make( [ 'str', 0, 1, [ 2 ], 'str', { a : 3 } ] );
  var got = dst.removeContainerOnce( src, ( e ) => e[ 0 ], ( ins ) => ins[ 0 ] );
  var exp = [ [ 3 ], [ 2 ] ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'dst and src has arrays, fromIndex and evaluator2';
  var dst = _.containerAdapter.make( [ 'str', 0, 1, [ 2 ], [ 3 ], [ 2 ], 'str', { a : 3 } ] );
  var src = _.containerAdapter.make( [ 'str', 0, 1, [ 2 ], 'str', { a : 3 } ] );
  var got = dst.removeContainerOnce( src, 3, ( e ) => e[ 0 ] );
  var exp = [ 'str', 0, 1, [ 3 ], [ 2 ] ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'dst and src has arrays, two evaluators';
  var dst = _.containerAdapter.make( [ 'str', 0, 1, [ 2 ], [ 3 ], [ 2 ], 'str', { a : 3 } ] );
  var src = _.containerAdapter.make( [ 'str', 0, 1, [ 2 ], 'str', { a : 3 } ] );
  var got = dst.removeContainerOnce( src, ( e, ins ) => e[ 0 ] === ins[ 0 ] );
  var exp = [ [ 3 ], [ 2 ] ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.close( 'arrayContainerAdapter' );

  /* - */

  test.open( 'setContainerAdapter' );

  test.case = 'dst - empty container, src - empty array';
  var dst = _.containerAdapter.make( new Set() );
  var src = [];
  var got = dst.removeContainerOnce( src );
  var exp = [];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'dst - empty container, src - empty Set';
  var dst = _.containerAdapter.make( new Set() );
  var src = new Set();
  var got = dst.removeContainerOnce( src );
  var exp = [];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'dst - empty container, src - arrayContainerAdapter';
  var dst = _.containerAdapter.make( new Set() );
  var src = _.containerAdapter.make( [ 1, [ 2 ], { a : 3 } ] );
  var got = dst.removeContainerOnce( src );
  var exp = [];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'dst - empty container, src - setContainerAdapter';
  var dst = _.containerAdapter.make( new Set() );
  var src = _.containerAdapter.make( new Set( [ 1, [ 2 ], { a : 3 } ] ) );
  var got = dst.removeContainerOnce( src );
  var exp = [];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  /* */

  test.case = 'dst - container, src - empty array, no entries';
  var dst = _.containerAdapter.make( new Set( [ 'str', null, 0 ] ) );
  var src = [ 1, [ 2 ], { a : 3 } ];
  var got = dst.removeContainerOnce( src );
  var exp = [ 'str', null, 0 ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'dst - container, src - Set, no entries';
  var dst = _.containerAdapter.make( new Set( [ 'str', null, 0 ] ) );
  var src = new Set( [ 1, [ 2 ], { a : 3 } ] );
  var got = dst.removeContainerOnce( src );
  var exp = [ 'str', null, 0 ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'dst - container, src - arrayContainerAdapter, entries';
  var dst = _.containerAdapter.make( new Set( [ 1, 1, 2, 'str', [ 2 ] ] ) );
  var src = _.containerAdapter.make( [ 1, [ 2 ], { a : 3 } ] );
  var got = dst.removeContainerOnce( src );
  var exp = [ 2, 'str', [ 2 ] ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'dst - container, src - setContainerAdapter';
  var dst = _.containerAdapter.make( new Set( [ 1, 1, 2, 'str', [ 2 ] ] ) );
  var src = _.containerAdapter.make( new Set( [ 1, [ 2 ], { a : 3 } ] ) );
  var got = dst.removeContainerOnce( src );
  var exp = [ 2, 'str', [ 2 ] ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  /* */

  test.case = 'dst === src';
  var dst = _.containerAdapter.make( new Set( [ 'str', null, 0, 1, [ 2 ], 'str', { a : 3 } ] ) );
  var src = dst;
  var got = dst.removeContainerOnce( src );
  var exp = [];
  test.true( got === dst );
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src identical to dst';
  var dst = _.containerAdapter.make( new Set( [ 'str', null, 0, 1, [ 2 ], 'str', { a : 3 } ] ) );
  var src = _.containerAdapter.make( [ 'str', null, 0, 1, [ 2 ], 'str', { a : 3 } ] );
  var got = dst.removeContainerOnce( src );
  var exp = [ [ 2 ], { a : 3 } ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  /* */

  test.case = 'dst and src has arrays, one evaluator';
  var dst = _.containerAdapter.make( new Set( [ 'str', 0, 1, [ 2 ], [ 3 ], [ 2 ], 'str', { a : 3 } ] ) );
  var src = _.containerAdapter.make( new Set( [ 'str', 0, 1, [ 2 ], 'str', { a : 3 } ] ) );
  var got = dst.removeContainerOnce( src, ( e ) => e[ 0 ] );
  var exp = [ [ 3 ], [ 2 ] ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'dst and src has arrays, two evaluators';
  var dst = _.containerAdapter.make( new Set( [ 'str', 0, 1, [ 2 ], [ 3 ], [ 2 ], 'str', { a : 3 } ] ) );
  var src = _.containerAdapter.make( new Set( [ 'str', 0, 1, [ 2 ], 'str', { a : 3 } ] ) );
  var got = dst.removeContainerOnce( src, ( e ) => e[ 0 ], ( ins ) => ins[ 0 ] );
  var exp = [ [ 3 ], [ 2 ] ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'dst and src has arrays, fromIndex and evaluator2';
  var dst = _.containerAdapter.make( new Set( [ 'str', 0, 1, [ 2 ], [ 3 ], [ 2 ], 'str', { a : 3 } ] ) );
  var src = _.containerAdapter.make( new Set( [ 'str', 0, 1, [ 2 ], 'str', { a : 3 } ] ) );
  var got = dst.removeContainerOnce( src, 3, ( e ) => e[ 0 ] );
  var exp = [ 'str', 0, 1, [ 3 ], [ 2 ] ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'dst and src has arrays, two evaluators';
  var dst = _.containerAdapter.make( new Set( [ 'str', 0, 1, [ 2 ], [ 3 ], [ 2 ], 'str', { a : 3 } ] ) );
  var src = _.containerAdapter.make( new Set( [ 'str', 0, 1, [ 2 ], 'str', { a : 3 } ] ) );
  var got = dst.removeContainerOnce( src, ( e, ins ) => e[ 0 ] === ins[ 0 ] );
  var exp = [ [ 3 ], [ 2 ] ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.close( 'setContainerAdapter' );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () =>
  {
    var dst = _.containerAdapter.make( [] );
    dst.removeContainerOnce();
  });

  test.case = 'wrong type of container';
  test.shouldThrowErrorSync( () =>
  {
    var dst = _.containerAdapter.make( [] );
    dst.removeContainerOnce( 0 );
  });

  test.shouldThrowErrorSync( () =>
  {
    var dst = _.containerAdapter.make( [] );
    dst.removeContainerOnce( new BufferRaw() );
  });

  test.shouldThrowErrorSync( () =>
  {
    var dst = _.containerAdapter.make( [] );
    dst.removeContainerOnce( 'wrong' );
  });
}

//

function removeContainerOnceStrictly( test )
{
  test.open( 'arrayContainerAdapter' );

  test.case = 'dst - container, src - arrayContainerAdapter, entries';
  var dst = _.containerAdapter.make( [ 1, 2, 'str', [ 2 ] ] );
  var src = _.containerAdapter.make( [ 1, 2, 'str' ] );
  var got = dst.removeContainerOnceStrictly( src );
  var exp = [ [ 2 ] ];
  test.true( got === dst );
  test.identical( got.original, exp );

  test.case = 'dst - container, src - setContainerAdapter';
  var dst = _.containerAdapter.make( [ 1, 2, 'str', [ 2 ] ] );
  var src = _.containerAdapter.make( new Set( [ 1, 2 ] ) );
  var got = dst.removeContainerOnceStrictly( src );
  var exp = [ 'str', [ 2 ] ];
  test.true( got === dst );
  test.identical( got.original, exp );

  /* */

  test.case = 'dst === src';
  var dst = _.containerAdapter.make( [ 'str', null, 0, 1, [ 2 ], { a : 3 } ] );
  var src = dst;
  var got = dst.removeContainerOnceStrictly( src );
  var exp = [];
  test.true( got === dst );
  test.true( got === src );
  test.identical( got.original, exp );

  test.case = 'src identical to dst';
  var dst = _.containerAdapter.make( [ 'str', null, 0, 1, [ 2 ], { a : 3 } ] );
  var src = _.containerAdapter.make( [ 'str', null, 0, 1 ] );
  var got = dst.removeContainerOnceStrictly( src );
  var exp = [ [ 2 ], { a : 3 } ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  /* */

  test.case = 'dst and src has arrays, one evaluator';
  var dst = _.containerAdapter.make( [ 'str', 0, 1, [ 2 ], [ 3 ], [ 4 ], 'str', { a : 3 } ] );
  var src = _.containerAdapter.make( [ [ 2 ], [ 4 ] ] );
  var got = dst.removeContainerOnceStrictly( src, ( e ) => e[ 0 ] );
  var exp = [ 'str', 0, 1, [ 3 ], 'str', { a : 3 } ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'dst and src has arrays, two evaluators';
  var dst = _.containerAdapter.make( [ 'str', 0, 1, [ 2 ], [ 3 ], [ 4 ], 'str', { a : 3 } ] );
  var src = _.containerAdapter.make( [ [ 2 ], [ 4 ] ] );
  var got = dst.removeContainerOnceStrictly( src, ( e ) => e[ 0 ], ( ins ) => ins[ 0 ] );
  var exp = [ 'str', 0, 1, [ 3 ], 'str', { a : 3 } ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'dst and src has arrays, fromIndex and evaluator2';
  var dst = _.containerAdapter.make( [ 'str', 0, 1, [ 2 ], [ 3 ], [ 2 ], 'str', { a : 3 } ] );
  var src = _.containerAdapter.make( [ [ 2 ], [ 3 ] ] );
  var got = dst.removeContainerOnceStrictly( src, 4, ( e ) => e[ 0 ] );
  var exp = [ 'str', 0, 1, [ 2 ], 'str', { a : 3 }  ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'dst and src has arrays, two evaluators';
  var dst = _.containerAdapter.make( [ 'str', 0, 1, [ 2 ], [ 3 ], [ 4 ], 'str', { a : 3 } ] );
  var src = _.containerAdapter.make( [ [ 2 ], [ 4 ] ] );
  var got = dst.removeContainerOnceStrictly( src, ( e, ins ) => e[ 0 ] === ins[ 0 ] );
  var exp = [ 'str', 0, 1, [ 3 ], 'str', { a : 3 } ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.close( 'arrayContainerAdapter' );

  /* - */

  test.open( 'setContainerAdapter' );

  test.case = 'dst - container, src - arrayContainerAdapter, entries';
  var dst = _.containerAdapter.make( new Set( [ 1, 2, 'str', [ 2 ] ] ) );
  var src = _.containerAdapter.make( [ 1 ] );
  var got = dst.removeContainerOnceStrictly( src );
  var exp = [ 2, 'str', [ 2 ] ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'dst - container, src - setContainerAdapter';
  var dst = _.containerAdapter.make( new Set( [ 1, 2, 'str', [ 2 ] ] ) );
  var src = _.containerAdapter.make( new Set( [ 1 ] ) );
  var got = dst.removeContainerOnceStrictly( src );
  var exp = [ 2, 'str', [ 2 ] ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  /* */

  test.case = 'dst === src';
  var dst = _.containerAdapter.make( new Set( [ 'str', null, 0, 1, [ 2 ], 'str', { a : 3 } ] ) );
  var src = dst;
  var got = dst.removeContainerOnceStrictly( src );
  var exp = [];
  test.true( got === dst );
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src identical to dst';
  var dst = _.containerAdapter.make( new Set( [ 'str', null, 0, 1, [ 2 ], { a : 3 } ] ) );
  var src = _.containerAdapter.make( [ 'str', null, 0, 1 ] );
  var got = dst.removeContainerOnceStrictly( src );
  var exp = [ [ 2 ], { a : 3 } ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  /* */

  test.case = 'dst and src has arrays, one evaluator';
  var dst = _.containerAdapter.make( new Set( [ 'str', 0, 1, [ 2 ], [ 3 ], [ 4 ], 'str', { a : 3 } ] ) );
  var src = _.containerAdapter.make( new Set( [ [ 2 ], [ 4 ] ] ) );
  var got = dst.removeContainerOnceStrictly( src, ( e ) => e[ 0 ] );
  var exp = [ 'str', 0, 1, [ 3 ], { a : 3 } ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'dst and src has arrays, two evaluators';
  var dst = _.containerAdapter.make( new Set( [ 'str', 0, 1, [ 2 ], [ 3 ], [ 4 ], 'str', { a : 3 } ] ) );
  var src = _.containerAdapter.make( new Set( [ [ 2 ], [ 4 ] ] ) );
  var got = dst.removeContainerOnceStrictly( src, ( e ) => e[ 0 ], ( ins ) => ins[ 0 ] );
  var exp = [ 'str', 0, 1, [ 3 ], { a : 3 } ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'dst and src has arrays, fromIndex and evaluator2';
  var dst = _.containerAdapter.make( new Set( [ 'str', 0, 1, [ 2 ], [ 3 ], [ 2 ], 'str', { a : 3 } ] ) );
  var src = _.containerAdapter.make( new Set( [ [ 2 ], [ 3 ] ] ) );
  var got = dst.removeContainerOnceStrictly( src, 4, ( e ) => e[ 0 ] );
  var exp = [ 'str', 0, 1, [ 2 ], { a : 3 }  ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'dst and src has arrays, two evaluators';
  var dst = _.containerAdapter.make( new Set( [ 'str', 0, 1, [ 2 ], [ 3 ], [ 4 ], 'str', { a : 3 } ] ) );
  var src = _.containerAdapter.make( new Set( [ [ 2 ], [ 4 ] ] ) );
  var got = dst.removeContainerOnceStrictly( src, ( e, ins ) => e[ 0 ] === ins[ 0 ] );
  var exp = [ 'str', 0, 1, [ 3 ], { a : 3 } ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.close( 'setContainerAdapter' );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () =>
  {
    var dst = _.containerAdapter.make( [] );
    dst.removeContainerOnceStrictly();
  });

  test.case = 'empty container';
  test.shouldThrowErrorSync( () =>
  {
    var dst = _.containerAdapter.make( [] );
    dst.removeContainerOnceStrictly( [ 1 ] );
  });

  test.case = 'a few entries';
  test.shouldThrowErrorSync( () =>
  {
    var dst = _.containerAdapter.make( [ 1, 1 ] );
    dst.removeContainerOnceStrictly( [ 1 ] );
  });

  test.case = 'no entries';
  test.shouldThrowErrorSync( () =>
  {
    var dst = _.containerAdapter.make( [ 2, 2 ] );
    dst.removeContainerOnceStrictly( [ 1 ] );
  });

  test.case = 'wrong type of container';
  test.shouldThrowErrorSync( () =>
  {
    var dst = _.containerAdapter.make( [] );
    dst.removeContainerOnceStrictly( 0 );
  });

  test.shouldThrowErrorSync( () =>
  {
    var dst = _.containerAdapter.make( [] );
    dst.removeContainerOnceStrictly( new BufferRaw() );
  });

  test.shouldThrowErrorSync( () =>
  {
    var dst = _.containerAdapter.make( [] );
    dst.removeContainerOnceStrictly( 'wrong' );
  });
}

//

function min( test )
{
  test.open( 'arrayContainerAdapter' );

  test.case = 'without onEach, has minimum';
  var src = _.containerAdapter.make( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
  var got = src.min();
  var exp = -20;
  test.identical( got, exp );

  test.case = 'without onEach, has minimum - -Infinity';
  var src = _.containerAdapter.make( [ -10, 5, -15, 4, -Infinity, 2, 15, 21, -20 ] );
  var got = src.min();
  var exp = -Infinity;
  test.identical( got, exp );

  test.case = 'without onEach, has minimum, small difference';
  var src = _.containerAdapter.make( [ -10, 5, -15, -20.0001, 4, 1, 2, 15, 21, -20 ] );
  var got = src.min();
  var exp = -20.0001;
  test.identical( got, exp );

  test.case = 'without onEach, all equal';
  var src = _.containerAdapter.make( [ -1, -1, -1, -1 ] );
  var got = src.min();
  var exp = -1;
  test.identical( got, exp );

  test.case = 'without onEach, all -Infinity';
  var src = _.containerAdapter.make( [ -Infinity, -Infinity, -Infinity, -Infinity ] );
  var got = src.min();
  var exp = -Infinity;
  test.identical( got, exp );

  test.case = 'without onEach, all Infinity';
  var src = _.containerAdapter.make( [ Infinity, Infinity, Infinity, Infinity ] );
  var got = src.min();
  var exp = Infinity;
  test.identical( got, exp );

  test.case = 'without onEach, all NaN';
  var src = _.containerAdapter.make( [ NaN, NaN, NaN, NaN ] );
  var got = src.min();
  var exp = NaN;
  test.identical( got, exp );

  test.case = 'without onEach, has minimum and NaN';
  var src = _.containerAdapter.make( [ -10, 5, -15, '4', 1, NaN, 15, -1, 21, '-20' ] );
  var got = src.min();
  var exp = NaN;
  test.identical( got, exp );

  /* */

  test.case = 'onEach, has minimum';
  var src = _.containerAdapter.make( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
  var got = src.min( ( e ) => e < 0 ? -e : e );
  var exp = 1;
  test.identical( got, exp );

  test.case = 'onEach, has minimum - -Infinity';
  var src = _.containerAdapter.make( [ -10, 5, -15, 4, -Infinity, 2, 15, 21, -20 ] );
  var got = src.min( ( e ) => e < 0 ? -e : e );
  var exp = 2;
  test.identical( got, exp );

  test.case = 'onEach, has minimum, small difference';
  var src = _.containerAdapter.make( [ -10, 5, -15, -20.0001, 4, 1, 2, 15, 21, -20 ] );
  var got = src.min( ( e ) => e < 0 ? -e : e );
  var exp = 1;
  test.identical( got, exp );

  test.case = 'onEach, all equal';
  var src = _.containerAdapter.make( [ -1, -1, -1, -1 ] );
  var got = src.min( ( e ) => e < 0 ? -e : e );
  var exp = 1;
  test.identical( got, exp );

  test.case = 'onEach, all -Infinity';
  var src = _.containerAdapter.make( [ -Infinity, -Infinity, -Infinity, -Infinity ] );
  var got = src.min( ( e ) => e < 0 ? -e : e );
  var exp = Infinity;
  test.identical( got, exp );

  test.case = 'onEach, all Infinity';
  var src = _.containerAdapter.make( [ Infinity, Infinity, Infinity, Infinity ] );
  var got = src.min( ( e ) => e < 0 ? -e : e );
  var exp = Infinity;
  test.identical( got, exp );

  test.case = 'onEach, all NaN';
  var src = _.containerAdapter.make( [ NaN, NaN, NaN, NaN ] );
  var got = src.min( ( e ) => isNaN( e ) ? 0 : e );
  var exp = 0;
  test.identical( got, exp );

  test.case = 'onEach, has minimum and NaN';
  var src = _.containerAdapter.make( [ -10, 5, -15, '4', 1, NaN, 15, -1, 21, '-20' ] );
  var got = src.min( ( e ) => isNaN( e ) || _.strIs( e ) ? 0 : e );
  var exp = -15;
  test.identical( got, exp );

  test.close( 'arrayContainerAdapter' );

  /* - */

  test.open( 'setContainerAdapter' );

  test.case = 'without onEach, has minimum';
  var src = _.containerAdapter.make( new Set( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] ) );
  var got = src.min();
  var exp = -20;
  test.identical( got, exp );

  test.case = 'without onEach, has minimum - -Infinity';
  var src = _.containerAdapter.make( new Set( [ -10, 5, -15, 4, -Infinity, 2, 15, 21, -20 ] ) );
  var got = src.min();
  var exp = -Infinity;
  test.identical( got, exp );

  test.case = 'without onEach, has minimum, small difference';
  var src = _.containerAdapter.make( new Set( [ -10, 5, -15, -20.0001, 4, 1, 2, 15, 21, -20 ] ) );
  var got = src.min();
  var exp = -20.0001;
  test.identical( got, exp );

  test.case = 'without onEach, all equal';
  var src = _.containerAdapter.make( new Set( [ -1, -1, -1, -1 ] ) );
  var got = src.min();
  var exp = -1;
  test.identical( got, exp );

  test.case = 'without onEach, all -Infinity';
  var src = _.containerAdapter.make( new Set( [ -Infinity, -Infinity, -Infinity, -Infinity ] ) );
  var got = src.min();
  var exp = -Infinity;
  test.identical( got, exp );

  test.case = 'without onEach, all Infinity';
  var src = _.containerAdapter.make( new Set( [ Infinity, Infinity, Infinity, Infinity ] ) );
  var got = src.min();
  var exp = Infinity;
  test.identical( got, exp );

  test.case = 'without onEach, all NaN';
  var src = _.containerAdapter.make( new Set( [ NaN, NaN, NaN, NaN ] ) );
  var got = src.min();
  var exp = NaN;
  test.identical( got, exp );

  test.case = 'without onEach, has minimum and NaN';
  var src = _.containerAdapter.make( new Set( [ -10, 5, -15, '4', 1, NaN, 15, -1, 21, '-20' ] ) );
  var got = src.min();
  var exp = NaN;
  test.identical( got, exp );

  /* */

  test.case = 'onEach, has minimum';
  var src = _.containerAdapter.make( new Set( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] ) );
  var got = src.min( ( e ) => e < 0 ? -e : e );
  var exp = 1;
  test.identical( got, exp );

  test.case = 'onEach, has minimum - -Infinity';
  var src = _.containerAdapter.make( new Set( [ -10, 5, -15, 4, -Infinity, 2, 15, 21, -20 ] ) );
  var got = src.min( ( e ) => e < 0 ? -e : e );
  var exp = 2;
  test.identical( got, exp );

  test.case = 'onEach, has minimum, small difference';
  var src = _.containerAdapter.make( new Set( [ -10, 5, -15, -20.0001, 4, 1, 2, 15, 21, -20 ] ) );
  var got = src.min( ( e ) => e < 0 ? -e : e );
  var exp = 1;
  test.identical( got, exp );

  test.case = 'onEach, all equal';
  var src = _.containerAdapter.make( new Set( [ -1, -1, -1, -1 ] ) );
  var got = src.min( ( e ) => e < 0 ? -e : e );
  var exp = 1;
  test.identical( got, exp );

  test.case = 'onEach, all -Infinity';
  var src = _.containerAdapter.make( new Set( [ -Infinity, -Infinity, -Infinity, -Infinity ] ) );
  var got = src.min( ( e ) => e < 0 ? -e : e );
  var exp = Infinity;
  test.identical( got, exp );

  test.case = 'onEach, all Infinity';
  var src = _.containerAdapter.make( new Set( [ Infinity, Infinity, Infinity, Infinity ] ) );
  var got = src.min( ( e ) => e < 0 ? -e : e );
  var exp = Infinity;
  test.identical( got, exp );

  test.case = 'onEach, all NaN';
  var src = _.containerAdapter.make( new Set( [ NaN, NaN, NaN, NaN ] ) );
  var got = src.min( ( e ) => isNaN( e ) ? 0 : e );
  var exp = 0;
  test.identical( got, exp );

  test.case = 'onEach, has minimum and NaN';
  var src = _.containerAdapter.make( new Set( [ -10, 5, -15, '4', 1, NaN, 15, -1, 21, '-20' ] ) );
  var got = src.min( ( e ) => isNaN( e ) || _.strIs( e ) ? 0 : e );
  var exp = -15;
  test.identical( got, exp );

  test.close( 'setContainerAdapter' );
}

//

function max( test )
{
  test.open( 'arrayContainerAdapter' );

  test.case = 'without onEach, has maximum';
  var src = _.containerAdapter.make( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
  var got = src.max();
  var exp = 21;
  test.identical( got, exp );

  test.case = 'without onEach, has maximum - Infinity';
  var src = _.containerAdapter.make( [ -10, 5, -15, 4, Infinity, 2, 15, 21, -20 ] );
  var got = src.max();
  var exp = Infinity;
  test.identical( got, exp );

  test.case = 'without onEach, has maximum, small difference';
  var src = _.containerAdapter.make( [ -10, 5, -15, 21.0001, 4, 1, 2, 15, 21, -20 ] );
  var got = src.max();
  var exp = 21.0001;
  test.identical( got, exp );

  test.case = 'without onEach, all equal';
  var src = _.containerAdapter.make( [ 1, 1, 1, 1 ] );
  var got = src.max();
  var exp = 1;
  test.identical( got, exp );

  test.case = 'without onEach, all -Infinity';
  var src = _.containerAdapter.make( [ -Infinity, -Infinity, -Infinity, -Infinity ] );
  var got = src.max();
  var exp = -Infinity;
  test.identical( got, exp );

  test.case = 'without onEach, all Infinity';
  var src = _.containerAdapter.make( [ Infinity, Infinity, Infinity, Infinity ] );
  var got = src.max();
  var exp = Infinity;
  test.identical( got, exp );

  test.case = 'without onEach, all NaN';
  var src = _.containerAdapter.make( [ NaN, NaN, NaN, NaN ] );
  var got = src.max();
  var exp = NaN;
  test.identical( got, exp );

  test.case = 'without onEach, has maximum and NaN';
  var src = _.containerAdapter.make( [ -10, 5, -15, '4', 1, NaN, 15, -1, 21, '-20' ] );
  var got = src.max();
  var exp = NaN;
  test.identical( got, exp );

  /* */

  test.case = 'onEach, has maximum';
  var src = _.containerAdapter.make( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
  var got = src.max( ( e ) => e < 0 ? -e : e );
  var exp = 21;
  test.identical( got, exp );

  test.case = 'onEach, has maximum - -Infinity';
  var src = _.containerAdapter.make( [ -10, 5, -15, 4, -Infinity, 2, 15, 21, -20 ] );
  var got = src.max( ( e ) => e < 0 ? -e : e );
  var exp = Infinity;
  test.identical( got, exp );

  test.case = 'onEach, has maximum, small difference';
  var src = _.containerAdapter.make( [ -10, 5, -15, -21.0001, 4, 1, 2, 15, 21, -20 ] );
  var got = src.max( ( e ) => e < 0 ? -e : e );
  var exp = 21.0001;
  test.identical( got, exp );

  test.case = 'onEach, all equal';
  var src = _.containerAdapter.make( [ -1, -1, -1, -1 ] );
  var got = src.max( ( e ) => e < 0 ? -e : e );
  var exp = 1;
  test.identical( got, exp );

  test.case = 'onEach, all -Infinity';
  var src = _.containerAdapter.make( [ -Infinity, -Infinity, -Infinity, -Infinity ] );
  var got = src.max( ( e ) => e < 0 ? -e : e );
  var exp = Infinity;
  test.identical( got, exp );

  test.case = 'onEach, all Infinity';
  var src = _.containerAdapter.make( [ Infinity, Infinity, Infinity, Infinity ] );
  var got = src.max( ( e ) => e < 0 ? -e : e );
  var exp = Infinity;
  test.identical( got, exp );

  test.case = 'onEach, all NaN';
  var src = _.containerAdapter.make( [ NaN, NaN, NaN, NaN ] );
  var got = src.max( ( e ) => isNaN( e ) ? 0 : e );
  var exp = 0;
  test.identical( got, exp );

  test.case = 'onEach, has maximum and NaN';
  var src = _.containerAdapter.make( [ -10, 5, -15, '4', 1, NaN, 15, -1, 21, '-20' ] );
  var got = src.max( ( e ) => isNaN( e ) || _.strIs( e ) ? 0 : e );
  var exp = 21;
  test.identical( got, exp );

  test.close( 'arrayContainerAdapter' );

  /* - */

  test.open( 'setContainerAdapter' );

  test.case = 'without onEach, has maximum';
  var src = _.containerAdapter.make( new Set( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] ) );
  var got = src.max();
  var exp = 21;
  test.identical( got, exp );

  test.case = 'without onEach, has maximum - Infinity';
  var src = _.containerAdapter.make( new Set( [ -10, 5, -15, 4, Infinity, 2, 15, 21, -20 ] ) );
  var got = src.max();
  var exp = Infinity;
  test.identical( got, exp );

  test.case = 'without onEach, has maximum, small difference';
  var src = _.containerAdapter.make( new Set( [ -10, 5, -15, 21.0001, 4, 1, 2, 15, 21, -20 ] ) );
  var got = src.max();
  var exp = 21.0001;
  test.identical( got, exp );

  test.case = 'without onEach, all equal';
  var src = _.containerAdapter.make( new Set( [ 1, 1, 1, 1 ] ) );
  var got = src.max();
  var exp = 1;
  test.identical( got, exp );

  test.case = 'without onEach, all -Infinity';
  var src = _.containerAdapter.make( new Set( [ -Infinity, -Infinity, -Infinity, -Infinity ] ) );
  var got = src.max();
  var exp = -Infinity;
  test.identical( got, exp );

  test.case = 'without onEach, all Infinity';
  var src = _.containerAdapter.make( new Set( [ Infinity, Infinity, Infinity, Infinity ] ) );
  var got = src.max();
  var exp = Infinity;
  test.identical( got, exp );

  test.case = 'without onEach, all NaN';
  var src = _.containerAdapter.make( new Set( [ NaN, NaN, NaN, NaN ] ) );
  var got = src.max();
  var exp = NaN;
  test.identical( got, exp );

  test.case = 'without onEach, has maximum and NaN';
  var src = _.containerAdapter.make( new Set( [ -10, 5, -15, '4', 1, NaN, 15, -1, 21, '-20' ] ) );
  var got = src.max();
  var exp = NaN;
  test.identical( got, exp );

  /* */

  test.case = 'onEach, has maximum';
  var src = _.containerAdapter.make( new Set( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] ) );
  var got = src.max( ( e ) => e < 0 ? -e : e );
  var exp = 21;
  test.identical( got, exp );

  test.case = 'onEach, has maximum - Infinity';
  var src = _.containerAdapter.make( new Set( [ -10, 5, -15, 4, -Infinity, 2, 15, 21, -20 ] ) );
  var got = src.max( ( e ) => e < 0 ? -e : e );
  var exp = Infinity;
  test.identical( got, exp );

  test.case = 'onEach, has maximum, small difference';
  var src = _.containerAdapter.make( new Set( [ -10, 5, -15, -21.0001, 4, 1, 2, 15, 21, -20 ] ) );
  var got = src.max( ( e ) => e < 0 ? -e : e );
  var exp = 21.0001;
  test.identical( got, exp );

  test.case = 'onEach, all equal';
  var src = _.containerAdapter.make( new Set( [ -1, -1, -1, -1 ] ) );
  var got = src.max( ( e ) => e < 0 ? -e : e );
  var exp = 1;
  test.identical( got, exp );

  test.case = 'onEach, all -Infinity';
  var src = _.containerAdapter.make( new Set( [ -Infinity, -Infinity, -Infinity, -Infinity ] ) );
  var got = src.max( ( e ) => e < 0 ? -e : e );
  var exp = Infinity;
  test.identical( got, exp );

  test.case = 'onEach, all Infinity';
  var src = _.containerAdapter.make( new Set( [ Infinity, Infinity, Infinity, Infinity ] ) );
  var got = src.max( ( e ) => e < 0 ? -e : e );
  var exp = Infinity;
  test.identical( got, exp );

  test.case = 'onEach, all NaN';
  var src = _.containerAdapter.make( new Set( [ NaN, NaN, NaN, NaN ] ) );
  var got = src.max( ( e ) => isNaN( e ) ? 0 : e );
  var exp = 0;
  test.identical( got, exp );

  test.case = 'onEach, has maximum and NaN';
  var src = _.containerAdapter.make( new Set( [ -10, 5, -15, '4', 1, NaN, 15, -1, 21, '-20' ] ) );
  var got = src.max( ( e ) => isNaN( e ) || _.strIs( e ) ? 0 : e );
  var exp = 21;
  test.identical( got, exp );

  test.close( 'setContainerAdapter' );
}

//

function least( test )
{
  test.open( 'arrayContainerAdapter' );

  test.open( 'without dst' );

  test.case = 'without onEach, has one minimum';
  var src = _.containerAdapter.make( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
  var got = src.least();
  var exp = [ -20 ];
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'without onEach, has minimum - -Infinity';
  var src = _.containerAdapter.make( [ -10, 5, -15, 4, -Infinity, 2, 15, 21, -20 ] );
  var got = src.least();
  var exp = [ -Infinity ];
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'without onEach, has minimum, small difference';
  var src = _.containerAdapter.make( [ -10, 5, -15, -20.0001, 4, 1, 2, 15, 21, -20 ] );
  var got = src.least();
  var exp = [ -20.0001 ];
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'without onEach, all equal';
  var src = _.containerAdapter.make( [ -1, -1, -1, -1 ] );
  var got = src.least();
  var exp = [ -1, -1, -1, -1 ];
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'without onEach, all -Infinity';
  var src = _.containerAdapter.make( [ -Infinity, -Infinity, -Infinity, -Infinity ] );
  var got = src.least();
  var exp = [ -Infinity, -Infinity, -Infinity, -Infinity ];
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'without onEach, all Infinity';
  var src = _.containerAdapter.make( [ Infinity, Infinity, Infinity, Infinity ] );
  var got = src.least();
  var exp = [ Infinity, Infinity, Infinity, Infinity ];
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'without onEach, all NaN';
  var src = _.containerAdapter.make( [ NaN, NaN, NaN, NaN ] );
  var got = src.least();
  var exp = [];
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'without onEach, has minimum and NaN';
  var src = _.containerAdapter.make( [ -10, 5, -15, '4', 1, NaN, 15, -1, 21, '-20' ] );
  var got = src.least();
  var exp = [];
  test.true( got !== src );
  test.identical( got.original, exp );

  /* */

  test.case = 'onEach, has minimum';
  var src = _.containerAdapter.make( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
  var got = src.least( ( e ) => e < 0 ? -e : e );
  var exp = [ 1 ];
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'onEach, has minimum - -Infinity';
  var src = _.containerAdapter.make( [ -10, 5, -15, 4, -Infinity, 2, 15, 21, -20 ] );
  var got = src.least( ( e ) => e < 0 ? -e : e );
  var exp = [ 2 ];
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'onEach, has minimum, small difference';
  var src = _.containerAdapter.make( [ -10, 5, -15, -20.0001, 4, 1, 2, 15, 21, -20 ] );
  var got = src.least( ( e ) => e < 0 ? -e : e );
  var exp = [ 1 ];
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'onEach, all equal';
  var src = _.containerAdapter.make( [ -1, -1, -1, -1 ] );
  var got = src.least( ( e ) => isNaN( e ) || _.strIs( e ) ? 0 : e );
  var exp = [ -1, -1, -1, -1 ];
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'onEach, all -Infinity';
  var src = _.containerAdapter.make( [ -Infinity, -Infinity, -Infinity, -Infinity ] );
  var got = src.least( ( e ) => isNaN( e ) || _.strIs( e ) ? 0 : e );
  var exp = [ -Infinity, -Infinity, -Infinity, -Infinity ];
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'onEach, all Infinity';
  var src = _.containerAdapter.make( [ Infinity, Infinity, Infinity, Infinity ] );
  var got = src.least( ( e ) => e < 0 ? -e : e );
  var exp = [ Infinity, Infinity, Infinity, Infinity ];
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'onEach, all NaN';
  var src = _.containerAdapter.make( [ NaN, NaN, NaN, NaN ] );
  var got = src.least( ( e ) => isNaN( e ) ? 0 : e );
  var exp = [ NaN, NaN, NaN, NaN ];
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'onEach, has minimum and NaN';
  var src = _.containerAdapter.make( [ -10, 5, -15, '4', 1, NaN, 15, -1, 21, '-20' ] );
  var got = src.least( ( e ) => isNaN( e ) || _.strIs( e ) ? 0 : e );
  var exp = [ -15 ];
  test.true( got !== src );
  test.identical( got.original, exp );

  test.close( 'without dst' );

  /* - */

  test.open( 'dst - null' );

  test.case = 'without onEach, has one minimum';
  var src = _.containerAdapter.make( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
  var got = src.least( null );
  var exp = [ -20 ];
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'without onEach, has minimum - -Infinity';
  var src = _.containerAdapter.make( [ -10, 5, -15, 4, -Infinity, 2, 15, 21, -20 ] );
  var got = src.least( null );
  var exp = [ -Infinity ];
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'without onEach, has minimum, small difference';
  var src = _.containerAdapter.make( [ -10, 5, -15, -20.0001, 4, 1, 2, 15, 21, -20 ] );
  var got = src.least( null );
  var exp = [ -20.0001 ];
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'without onEach, all equal';
  var src = _.containerAdapter.make( [ -1, -1, -1, -1 ] );
  var got = src.least( null );
  var exp = [ -1, -1, -1, -1 ];
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'without onEach, all -Infinity';
  var src = _.containerAdapter.make( [ -Infinity, -Infinity, -Infinity, -Infinity ] );
  var got = src.least( null );
  var exp = [ -Infinity, -Infinity, -Infinity, -Infinity ];
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'without onEach, all Infinity';
  var src = _.containerAdapter.make( [ Infinity, Infinity, Infinity, Infinity ] );
  var got = src.least( null );
  var exp = [ Infinity, Infinity, Infinity, Infinity ];
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'without onEach, all NaN';
  var src = _.containerAdapter.make( [ NaN, NaN, NaN, NaN ] );
  var got = src.least( null );
  var exp = [];
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'without onEach, has minimum and NaN';
  var src = _.containerAdapter.make( [ -10, 5, -15, '4', 1, NaN, 15, -1, 21, '-20' ] );
  var got = src.least( null );
  var exp = [];
  test.true( got !== src );
  test.identical( got.original, exp );

  /* */

  test.case = 'onEach, has minimum';
  var src = _.containerAdapter.make( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
  var got = src.least( null, ( e ) => e < 0 ? -e : e );
  var exp = [ 1 ];
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'onEach, has minimum - -Infinity';
  var src = _.containerAdapter.make( [ -10, 5, -15, 4, -Infinity, 2, 15, 21, -20 ] );
  var got = src.least( null, ( e ) => e < 0 ? -e : e );
  var exp = [ 2 ];
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'onEach, has minimum, small difference';
  var src = _.containerAdapter.make( [ -10, 5, -15, -20.0001, 4, 1, 2, 15, 21, -20 ] );
  var got = src.least( null, ( e ) => e < 0 ? -e : e );
  var exp = [ 1 ];
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'onEach, all equal';
  var src = _.containerAdapter.make( [ -1, -1, -1, -1 ] );
  var got = src.least( null, ( e ) => isNaN( e ) || _.strIs( e ) ? 0 : e );
  var exp = [ -1, -1, -1, -1 ];
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'onEach, all -Infinity';
  var src = _.containerAdapter.make( [ -Infinity, -Infinity, -Infinity, -Infinity ] );
  var got = src.least( null, ( e ) => isNaN( e ) || _.strIs( e ) ? 0 : e );
  var exp = [ -Infinity, -Infinity, -Infinity, -Infinity ];
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'onEach, all Infinity';
  var src = _.containerAdapter.make( [ Infinity, Infinity, Infinity, Infinity ] );
  var got = src.least( null, ( e ) => e < 0 ? -e : e );
  var exp = [ Infinity, Infinity, Infinity, Infinity ];
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'onEach, all NaN';
  var src = _.containerAdapter.make( [ NaN, NaN, NaN, NaN ] );
  var got = src.least( null, ( e ) => isNaN( e ) ? 0 : e );
  var exp = [ NaN, NaN, NaN, NaN ];
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'onEach, has minimum and NaN';
  var src = _.containerAdapter.make( [ -10, 5, -15, '4', 1, NaN, 15, -1, 21, '-20' ] );
  var got = src.least( null, ( e ) => isNaN( e ) || _.strIs( e ) ? 0 : e );
  var exp = [ -15 ];
  test.true( got !== src );
  test.identical( got.original, exp );

  test.close( 'dst - null' );

  /* - */

  test.open( 'dst - new container' );

  test.case = 'without onEach, has one minimum';
  var dst = _.containerAdapter.make( [] );
  var src = _.containerAdapter.make( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
  var got = src.least( dst );
  var exp = [ -20 ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'without onEach, has minimum - -Infinity';
  var dst = _.containerAdapter.make( [] );
  var src = _.containerAdapter.make( [ -10, 5, -15, 4, -Infinity, 2, 15, 21, -20 ] );
  var got = src.least( dst );
  var exp = [ -Infinity ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'without onEach, has minimum, small difference';
  var dst = _.containerAdapter.make( [] );
  var src = _.containerAdapter.make( [ -10, 5, -15, -20.0001, 4, 1, 2, 15, 21, -20 ] );
  var got = src.least( dst );
  var exp = [ -20.0001 ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'without onEach, all equal';
  var dst = _.containerAdapter.make( [] );
  var src = _.containerAdapter.make( [ -1, -1, -1, -1 ] );
  var got = src.least( dst );
  var exp = [ -1, -1, -1, -1 ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'without onEach, all -Infinity';
  var dst = _.containerAdapter.make( [] );
  var src = _.containerAdapter.make( [ -Infinity, -Infinity, -Infinity, -Infinity ] );
  var got = src.least( dst );
  var exp = [ -Infinity, -Infinity, -Infinity, -Infinity ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'without onEach, all Infinity';
  var dst = _.containerAdapter.make( [] );
  var src = _.containerAdapter.make( [ Infinity, Infinity, Infinity, Infinity ] );
  var got = src.least( dst );
  var exp = [ Infinity, Infinity, Infinity, Infinity ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'without onEach, all NaN';
  var dst = _.containerAdapter.make( [] );
  var src = _.containerAdapter.make( [ NaN, NaN, NaN, NaN ] );
  var got = src.least( dst );
  var exp = [];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'without onEach, has minimum and NaN';
  var dst = _.containerAdapter.make( [] );
  var src = _.containerAdapter.make( [ -10, 5, -15, '4', 1, NaN, 15, -1, 21, '-20' ] );
  var got = src.least( dst );
  var exp = [];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  /* */

  test.case = 'onEach, has minimum';
  var dst = _.containerAdapter.make( [] );
  var src = _.containerAdapter.make( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
  var got = src.least( dst, ( e ) => e < 0 ? -e : e );
  var exp = [ 1 ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'onEach, has minimum - -Infinity';
  var dst = _.containerAdapter.make( [] );
  var src = _.containerAdapter.make( [ -10, 5, -15, 4, -Infinity, 2, 15, 21, -20 ] );
  var got = src.least( dst, ( e ) => e < 0 ? -e : e );
  var exp = [ 2 ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'onEach, has minimum, small difference';
  var dst = _.containerAdapter.make( [] );
  var src = _.containerAdapter.make( [ -10, 5, -15, -20.0001, 4, 1, 2, 15, 21, -20 ] );
  var got = src.least( dst, ( e ) => e < 0 ? -e : e );
  var exp = [ 1 ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'onEach, all equal';
  var dst = _.containerAdapter.make( [] );
  var src = _.containerAdapter.make( [ -1, -1, -1, -1 ] );
  var got = src.least( dst, ( e ) => isNaN( e ) || _.strIs( e ) ? 0 : e );
  var exp = [ -1, -1, -1, -1 ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'onEach, all -Infinity';
  var dst = _.containerAdapter.make( [] );
  var src = _.containerAdapter.make( [ -Infinity, -Infinity, -Infinity, -Infinity ] );
  var got = src.least( dst, ( e ) => isNaN( e ) || _.strIs( e ) ? 0 : e );
  var exp = [ -Infinity, -Infinity, -Infinity, -Infinity ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'onEach, all Infinity';
  var dst = _.containerAdapter.make( [] );
  var src = _.containerAdapter.make( [ Infinity, Infinity, Infinity, Infinity ] );
  var got = src.least( dst, ( e ) => e < 0 ? -e : e );
  var exp = [ Infinity, Infinity, Infinity, Infinity ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'onEach, all NaN';
  var dst = _.containerAdapter.make( [] );
  var src = _.containerAdapter.make( [ NaN, NaN, NaN, NaN ] );
  var got = src.least( dst, ( e ) => isNaN( e ) ? 0 : e );
  var exp = [ NaN, NaN, NaN, NaN ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'onEach, has minimum and NaN';
  var dst = _.containerAdapter.make( [] );
  var src = _.containerAdapter.make( [ -10, 5, -15, '4', 1, NaN, 15, -1, 21, '-20' ] );
  var got = src.least( dst, ( e ) => isNaN( e ) || _.strIs( e ) ? 0 : e );
  var exp = [ -15 ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  /* */

  test.case = 'onEach, has minimum';
  var dst = _.containerAdapter.make( new Set() );
  var src = _.containerAdapter.make( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
  var got = src.least( dst, ( e ) => e < 0 ? -e : e );
  var exp = [ 1 ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'onEach, has minimum - -Infinity';
  var dst = _.containerAdapter.make( new Set() );
  var src = _.containerAdapter.make( [ -10, 5, -15, 4, -Infinity, 2, 15, 21, -20 ] );
  var got = src.least( dst, ( e ) => e < 0 ? -e : e );
  var exp = [ 2 ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'onEach, has minimum, small difference';
  var dst = _.containerAdapter.make( new Set() );
  var src = _.containerAdapter.make( [ -10, 5, -15, -20.0001, 4, 1, 2, 15, 21, -20 ] );
  var got = src.least( dst, ( e ) => e < 0 ? -e : e );
  var exp = [ 1 ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'onEach, all equal';
  var dst = _.containerAdapter.make( new Set() );
  var src = _.containerAdapter.make( [ -1, -1, -1, -1 ] );
  var got = src.least( dst, ( e ) => isNaN( e ) || _.strIs( e ) ? 0 : e );
  var exp = [ -1 ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'onEach, all -Infinity';
  var dst = _.containerAdapter.make( new Set() );
  var src = _.containerAdapter.make( [ -Infinity, -Infinity, -Infinity, -Infinity ] );
  var got = src.least( dst, ( e ) => isNaN( e ) || _.strIs( e ) ? 0 : e );
  var exp = [ -Infinity ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'onEach, all Infinity';
  var dst = _.containerAdapter.make( new Set() );
  var src = _.containerAdapter.make( [ Infinity, Infinity, Infinity, Infinity ] );
  var got = src.least( dst, ( e ) => e < 0 ? -e : e );
  var exp = [ Infinity ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'onEach, all NaN';
  var dst = _.containerAdapter.make( new Set() );
  var src = _.containerAdapter.make( [ NaN, NaN, NaN, NaN ] );
  var got = src.least( dst, ( e ) => isNaN( e ) ? 0 : e );
  var exp = [ NaN ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'onEach, has minimum and NaN';
  var dst = _.containerAdapter.make( new Set() );
  var src = _.containerAdapter.make( [ -10, 5, -15, '4', 1, NaN, 15, -1, 21, '-20' ] );
  var got = src.least( dst, ( e ) => isNaN( e ) || _.strIs( e ) ? 0 : e );
  var exp = [ -15 ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.close( 'dst - new container' );

  /* - */

  test.open( 'dst - src' );

  test.case = 'without onEach, has one minimum';
  var src = _.containerAdapter.make( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
  var got = src.least( src );
  var exp = [ -20 ];
  test.true( got === src );
  test.identical( got.original, exp );

  test.case = 'without onEach, has minimum - -Infinity';
  var src = _.containerAdapter.make( [ -10, 5, -15, 4, -Infinity, 2, 15, 21, -20 ] );
  var got = src.least( src );
  var exp = [ -Infinity ];
  test.true( got === src );
  test.identical( got.original, exp );

  test.case = 'without onEach, has minimum, small difference';
  var src = _.containerAdapter.make( [ -10, 5, -15, -20.0001, 4, 1, 2, 15, 21, -20 ] );
  var got = src.least( src );
  var exp = [ -20.0001 ];
  test.true( got === src );
  test.identical( got.original, exp );

  test.case = 'without onEach, all equal';
  var src = _.containerAdapter.make( [ -1, -1, -1, -1 ] );
  var got = src.least( src );
  var exp = [ -1, -1, -1, -1 ];
  test.true( got === src );
  test.identical( got.original, exp );

  test.case = 'without onEach, all -Infinity';
  var src = _.containerAdapter.make( [ -Infinity, -Infinity, -Infinity, -Infinity ] );
  var got = src.least( src );
  var exp = [ -Infinity, -Infinity, -Infinity, -Infinity ];
  test.true( got === src );
  test.identical( got.original, exp );

  test.case = 'without onEach, all Infinity';
  var src = _.containerAdapter.make( [ Infinity, Infinity, Infinity, Infinity ] );
  var got = src.least( src );
  var exp = [ Infinity, Infinity, Infinity, Infinity ];
  test.true( got === src );
  test.identical( got.original, exp );

  test.case = 'without onEach, all NaN';
  var src = _.containerAdapter.make( [ NaN, NaN, NaN, NaN ] );
  var got = src.least( src );
  var exp = [];
  test.true( got === src );
  test.identical( got.original, exp );

  test.case = 'without onEach, has minimum and NaN';
  var src = _.containerAdapter.make( [ -10, 5, -15, '4', 1, NaN, 15, -1, 21, '-20' ] );
  var got = src.least( src );
  var exp = [];
  test.true( got === src );
  test.identical( got.original, exp );

  /* */

  test.case = 'onEach, has minimum';
  var src = _.containerAdapter.make( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
  var got = src.least( src, ( e ) => e < 0 ? -e : e );
  var exp = [ 1 ];
  test.true( got === src );
  test.identical( got.original, exp );

  test.case = 'onEach, has minimum - -Infinity';
  var src = _.containerAdapter.make( [ -10, 5, -15, 4, -Infinity, 2, 15, 21, -20 ] );
  var got = src.least( src, ( e ) => e < 0 ? -e : e );
  var exp = [ 2 ];
  test.true( got === src );
  test.identical( got.original, exp );

  test.case = 'onEach, has minimum, small difference';
  var src = _.containerAdapter.make( [ -10, 5, -15, -20.0001, 4, 1, 2, 15, 21, -20 ] );
  var got = src.least( src, ( e ) => e < 0 ? -e : e );
  var exp = [ 1 ];
  test.true( got === src );
  test.identical( got.original, exp );

  test.case = 'onEach, all equal';
  var src = _.containerAdapter.make( [ -1, -1, -1, -1 ] );
  var got = src.least( src, ( e ) => isNaN( e ) || _.strIs( e ) ? 0 : e );
  var exp = [ -1, -1, -1, -1 ];
  test.true( got === src );
  test.identical( got.original, exp );

  test.case = 'onEach, all -Infinity';
  var src = _.containerAdapter.make( [ -Infinity, -Infinity, -Infinity, -Infinity ] );
  var got = src.least( src, ( e ) => isNaN( e ) || _.strIs( e ) ? 0 : e );
  var exp = [ -Infinity, -Infinity, -Infinity, -Infinity ];
  test.true( got === src );
  test.identical( got.original, exp );

  test.case = 'onEach, all Infinity';
  var src = _.containerAdapter.make( [ Infinity, Infinity, Infinity, Infinity ] );
  var got = src.least( src, ( e ) => e < 0 ? -e : e );
  var exp = [ Infinity, Infinity, Infinity, Infinity ];
  test.true( got === src );
  test.identical( got.original, exp );

  test.case = 'onEach, all NaN';
  var src = _.containerAdapter.make( [ NaN, NaN, NaN, NaN ] );
  var got = src.least( src, ( e ) => isNaN( e ) ? 0 : e );
  var exp = [ NaN, NaN, NaN, NaN ];
  test.true( got === src );
  test.identical( got.original, exp );

  test.case = 'onEach, has minimum and NaN';
  var src = _.containerAdapter.make( [ -10, 5, -15, '4', 1, NaN, 15, -1, 21, '-20' ] );
  var got = src.least( src, ( e ) => isNaN( e ) || _.strIs( e ) ? 0 : e );
  var exp = [ -15 ];
  test.true( got === src );
  test.identical( got.original, exp );

  test.close( 'dst - src' );

  test.close( 'arrayContainerAdapter' );

  /* - */

  test.open( 'setContainerAdapter' );

  test.open( 'without dst' );

  test.case = 'without onEach, has one minimum';
  var src = _.containerAdapter.make( new Set( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] ) );
  var got = src.least();
  var exp = [ -20 ];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'without onEach, has minimum - -Infinity';
  var src = _.containerAdapter.make( new Set( [ -10, 5, -15, 4, -Infinity, 2, 15, 21, -20 ] ) );
  var got = src.least();
  var exp = [ -Infinity ];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'without onEach, has minimum, small difference';
  var src = _.containerAdapter.make( new Set(  [ -10, 5, -15, -20.0001, 4, 1, 2, 15, 21, -20 ] ) );
  var got = src.least();
  var exp = [ -20.0001 ];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'without onEach, all equal';
  var src = _.containerAdapter.make( new Set( [ -1, -1, -1, -1 ] ) );
  var got = src.least();
  var exp = [ -1 ];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'without onEach, all -Infinity';
  var src = _.containerAdapter.make( new Set( [ -Infinity, -Infinity, -Infinity, -Infinity ] ) );
  var got = src.least();
  var exp = [ -Infinity ];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'without onEach, all Infinity';
  var src = _.containerAdapter.make( new Set( [ Infinity, Infinity, Infinity, Infinity ] ) );
  var got = src.least();
  var exp = [ Infinity ];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'without onEach, all NaN';
  var src = _.containerAdapter.make( new Set( [ NaN, NaN, NaN, NaN ] ) );
  var got = src.least();
  var exp = [];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'without onEach, has minimum and NaN';
  var src = _.containerAdapter.make( new Set( [ -10, 5, -15, '4', 1, NaN, 15, -1, 21, '-20' ] ) );
  var got = src.least();
  var exp = [];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  /* */

  test.case = 'onEach, has minimum';
  var src = _.containerAdapter.make( new Set( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] ) );
  var got = src.least( ( e ) => e < 0 ? -e : e );
  var exp = [ 1 ];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'onEach, has minimum - -Infinity';
  var src = _.containerAdapter.make( new Set( [ -10, 5, -15, 4, -Infinity, 2, 15, 21, -20 ] ) );
  var got = src.least( ( e ) => e < 0 ? -e : e );
  var exp = [ 2 ];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'onEach, has minimum, small difference';
  var src = _.containerAdapter.make( new Set( [ -10, 5, -15, -20.0001, 4, 1, 2, 15, 21, -20 ] ) );
  var got = src.least( ( e ) => e < 0 ? -e : e );
  var exp = [ 1 ];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'onEach, all equal';
  var src = _.containerAdapter.make( new Set( [ -1, -1, -1, -1 ] ) );
  var got = src.least( ( e ) => isNaN( e ) || _.strIs( e ) ? 0 : e );
  var exp = [ -1 ];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'onEach, all -Infinity';
  var src = _.containerAdapter.make( new Set( [ -Infinity, -Infinity, -Infinity, -Infinity ] ) );
  var got = src.least( ( e ) => isNaN( e ) || _.strIs( e ) ? 0 : e );
  var exp = [ -Infinity ];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'onEach, all Infinity';
  var src = _.containerAdapter.make( new Set( [ Infinity, Infinity, Infinity, Infinity ] ) );
  var got = src.least( ( e ) => e < 0 ? -e : e );
  var exp = [ Infinity ];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'onEach, all NaN';
  var src = _.containerAdapter.make( new Set( [ NaN, NaN, NaN, NaN ] ) );
  var got = src.least( ( e ) => isNaN( e ) ? 0 : e );
  var exp = [ NaN ];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'onEach, has minimum and NaN';
  var src = _.containerAdapter.make( new Set( [ -10, 5, -15, '4', 1, NaN, 15, -1, 21, '-20' ] ) );
  var got = src.least( ( e ) => isNaN( e ) || _.strIs( e ) ? 0 : e );
  var exp = [ -15 ];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.close( 'without dst' );

  /* - */

  test.open( 'dst - null' );

  test.case = 'without onEach, has one minimum';
  var src = _.containerAdapter.make( new Set( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] ) );
  var got = src.least( null );
  var exp = [ -20 ];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'without onEach, has minimum - -Infinity';
  var src = _.containerAdapter.make( new Set( [ -10, 5, -15, 4, -Infinity, 2, 15, 21, -20 ] ) );
  var got = src.least( null );
  var exp = [ -Infinity ];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'without onEach, has minimum, small difference';
  var src = _.containerAdapter.make( new Set( [ -10, 5, -15, -20.0001, 4, 1, 2, 15, 21, -20 ] ) );
  var got = src.least( null );
  var exp = [ -20.0001 ];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'without onEach, all equal';
  var src = _.containerAdapter.make( new Set( [ -1, -1, -1, -1 ] ) );
  var got = src.least( null );
  var exp = [ -1 ];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'without onEach, all -Infinity';
  var src = _.containerAdapter.make( new Set( [ -Infinity, -Infinity, -Infinity, -Infinity ] ) );
  var got = src.least( null );
  var exp = [ -Infinity ];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'without onEach, all Infinity';
  var src = _.containerAdapter.make( new Set( [ Infinity, Infinity, Infinity, Infinity ] ) );
  var got = src.least( null );
  var exp = [ Infinity ];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'without onEach, all NaN';
  var src = _.containerAdapter.make( new Set( [ NaN, NaN, NaN, NaN ] ) );
  var got = src.least( null );
  var exp = [];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'without onEach, has minimum and NaN';
  var src = _.containerAdapter.make( new Set( [ -10, 5, -15, '4', 1, NaN, 15, -1, 21, '-20' ] ) );
  var got = src.least( null );
  var exp = [];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  /* */

  test.case = 'onEach, has minimum';
  var src = _.containerAdapter.make( new Set( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] ) );
  var got = src.least( null, ( e ) => e < 0 ? -e : e );
  var exp = [ 1 ];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'onEach, has minimum - -Infinity';
  var src = _.containerAdapter.make( new Set( [ -10, 5, -15, 4, -Infinity, 2, 15, 21, -20 ] ) );
  var got = src.least( null, ( e ) => e < 0 ? -e : e );
  var exp = [ 2 ];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'onEach, has minimum, small difference';
  var src = _.containerAdapter.make( new Set( [ -10, 5, -15, -20.0001, 4, 1, 2, 15, 21, -20 ] ) );
  var got = src.least( null, ( e ) => e < 0 ? -e : e );
  var exp = [ 1 ];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'onEach, all equal';
  var src = _.containerAdapter.make( new Set( [ -1, -1, -1, -1 ] ) );
  var got = src.least( null, ( e ) => isNaN( e ) || _.strIs( e ) ? 0 : e );
  var exp = [ -1 ];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'onEach, all -Infinity';
  var src = _.containerAdapter.make( new Set( [ -Infinity, -Infinity, -Infinity, -Infinity ] ) );
  var got = src.least( null, ( e ) => isNaN( e ) || _.strIs( e ) ? 0 : e );
  var exp = [ -Infinity ];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'onEach, all Infinity';
  var src = _.containerAdapter.make( new Set( [ Infinity, Infinity, Infinity, Infinity ] ) );
  var got = src.least( null, ( e ) => e < 0 ? -e : e );
  var exp = [ Infinity ];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'onEach, all NaN';
  var src = _.containerAdapter.make( new Set( [ NaN, NaN, NaN, NaN ] ) );
  var got = src.least( null, ( e ) => isNaN( e ) ? 0 : e );
  var exp = [ NaN ];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'onEach, has minimum and NaN';
  var src = _.containerAdapter.make( new Set( [ -10, 5, -15, '4', 1, NaN, 15, -1, 21, '-20' ] ) );
  var got = src.least( null, ( e ) => isNaN( e ) || _.strIs( e ) ? 0 : e );
  var exp = [ -15 ];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.close( 'dst - null' );

  /* - */

  test.open( 'dst - new container' );

  test.case = 'without onEach, has one minimum';
  var dst = _.containerAdapter.make( new Set( [] ) );
  var src = _.containerAdapter.make( new Set( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] ) );
  var got = src.least( dst );
  var exp = [ -20 ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'without onEach, has minimum - -Infinity';
  var dst = _.containerAdapter.make( new Set( [] ) );
  var src = _.containerAdapter.make( new Set( [ -10, 5, -15, 4, -Infinity, 2, 15, 21, -20 ] ) );
  var got = src.least( dst );
  var exp = [ -Infinity ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'without onEach, has minimum, small difference';
  var dst = _.containerAdapter.make( new Set( [] ) );
  var src = _.containerAdapter.make( new Set( [ -10, 5, -15, -20.0001, 4, 1, 2, 15, 21, -20 ] ) );
  var got = src.least( dst );
  var exp = [ -20.0001 ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'without onEach, all equal';
  var dst = _.containerAdapter.make( new Set( [] ) );
  var src = _.containerAdapter.make( new Set( [ -1, -1, -1, -1 ] ) );
  var got = src.least( dst );
  var exp = [ -1 ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'without onEach, all -Infinity';
  var dst = _.containerAdapter.make( new Set( [] ) );
  var src = _.containerAdapter.make( new Set( [ -Infinity, -Infinity, -Infinity, -Infinity ] ) );
  var got = src.least( dst );
  var exp = [ -Infinity ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'without onEach, all Infinity';
  var dst = _.containerAdapter.make( new Set( [] ) );
  var src = _.containerAdapter.make( new Set( [ Infinity, Infinity, Infinity, Infinity ] ) );
  var got = src.least( dst );
  var exp = [ Infinity ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'without onEach, all NaN';
  var dst = _.containerAdapter.make( new Set( [] ) );
  var src = _.containerAdapter.make( new Set( [ NaN, NaN, NaN, NaN ] ) );
  var got = src.least( dst );
  var exp = [];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'without onEach, has minimum and NaN';
  var dst = _.containerAdapter.make( new Set( [] ) );
  var src = _.containerAdapter.make( new Set( [ -10, 5, -15, '4', 1, NaN, 15, -1, 21, '-20' ] ) );
  var got = src.least( dst );
  var exp = [];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  /* */

  test.case = 'onEach, has minimum';
  var dst = _.containerAdapter.make( new Set( [] ) );
  var src = _.containerAdapter.make( new Set( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] ) );
  var got = src.least( dst, ( e ) => e < 0 ? -e : e );
  var exp = [ 1 ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'onEach, has minimum - -Infinity';
  var dst = _.containerAdapter.make( new Set( [] ) );
  var src = _.containerAdapter.make( new Set( [ -10, 5, -15, 4, -Infinity, 2, 15, 21, -20 ] ) );
  var got = src.least( dst, ( e ) => e < 0 ? -e : e );
  var exp = [ 2 ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'onEach, has minimum, small difference';
  var dst = _.containerAdapter.make( new Set( [] ) );
  var src = _.containerAdapter.make( new Set( [ -10, 5, -15, -20.0001, 4, 1, 2, 15, 21, -20 ] ) );
  var got = src.least( dst, ( e ) => e < 0 ? -e : e );
  var exp = [ 1 ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'onEach, all equal';
  var dst = _.containerAdapter.make( new Set( [] ) );
  var src = _.containerAdapter.make( new Set( [ -1, -1, -1, -1 ] ) );
  var got = src.least( dst, ( e ) => isNaN( e ) || _.strIs( e ) ? 0 : e );
  var exp = [ -1 ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'onEach, all -Infinity';
  var dst = _.containerAdapter.make( new Set( [] ) );
  var src = _.containerAdapter.make( new Set( [ -Infinity, -Infinity, -Infinity, -Infinity ] ) );
  var got = src.least( dst, ( e ) => isNaN( e ) || _.strIs( e ) ? 0 : e );
  var exp = [ -Infinity ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'onEach, all Infinity';
  var dst = _.containerAdapter.make( new Set( [] ) );
  var src = _.containerAdapter.make( new Set( [ Infinity, Infinity, Infinity, Infinity ] ) );
  var got = src.least( dst, ( e ) => e < 0 ? -e : e );
  var exp = [ Infinity ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'onEach, all NaN';
  var dst = _.containerAdapter.make( new Set( [] ) );
  var src = _.containerAdapter.make( new Set( [ NaN, NaN, NaN, NaN ] ) );
  var got = src.least( dst, ( e ) => isNaN( e ) ? 0 : e );
  var exp = [ NaN ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'onEach, has minimum and NaN';
  var dst = _.containerAdapter.make( new Set( [] ) );
  var src = _.containerAdapter.make( [ -10, 5, -15, '4', 1, NaN, 15, -1, 21, '-20' ] );
  var got = src.least( dst, ( e ) => isNaN( e ) || _.strIs( e ) ? 0 : e );
  var exp = [ -15 ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  /* */

  test.case = 'onEach, has minimum';
  var dst = _.containerAdapter.make( [] );
  var src = _.containerAdapter.make( new Set( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] ) );
  var got = src.least( dst, ( e ) => e < 0 ? -e : e );
  var exp = [ 1 ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'onEach, has minimum - -Infinity';
  var dst = _.containerAdapter.make( [] );
  var src = _.containerAdapter.make( new Set( [ -10, 5, -15, 4, -Infinity, 2, 15, 21, -20 ] ) );
  var got = src.least( dst, ( e ) => e < 0 ? -e : e );
  var exp = [ 2 ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'onEach, has minimum, small difference';
  var dst = _.containerAdapter.make( [] );
  var src = _.containerAdapter.make( new Set( [ -10, 5, -15, -20.0001, 4, 1, 2, 15, 21, -20 ] ) );
  var got = src.least( dst, ( e ) => e < 0 ? -e : e );
  var exp = [ 1 ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'onEach, all equal';
  var dst = _.containerAdapter.make( [] );
  var src = _.containerAdapter.make( new Set( [ -1, -1, -1, -1 ] ) );
  var got = src.least( dst, ( e ) => isNaN( e ) || _.strIs( e ) ? 0 : e );
  var exp = [ -1 ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'onEach, all -Infinity';
  var dst = _.containerAdapter.make( [] );
  var src = _.containerAdapter.make( new Set( [ -Infinity, -Infinity, -Infinity, -Infinity ] ) );
  var got = src.least( dst, ( e ) => isNaN( e ) || _.strIs( e ) ? 0 : e );
  var exp = [ -Infinity ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'onEach, all Infinity';
  var dst = _.containerAdapter.make( [] );
  var src = _.containerAdapter.make( new Set( [ Infinity, Infinity, Infinity, Infinity ] ) );
  var got = src.least( dst, ( e ) => e < 0 ? -e : e );
  var exp = [ Infinity ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'onEach, all NaN';
  var dst = _.containerAdapter.make( [] );
  var src = _.containerAdapter.make( new Set( [ NaN, NaN, NaN, NaN ] ) );
  var got = src.least( dst, ( e ) => isNaN( e ) ? 0 : e );
  var exp = [ NaN ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'onEach, has minimum and NaN';
  var dst = _.containerAdapter.make( [] );
  var src = _.containerAdapter.make( new Set( [ -10, 5, -15, '4', 1, NaN, 15, -1, 21, '-20' ] ) );
  var got = src.least( dst, ( e ) => isNaN( e ) || _.strIs( e ) ? 0 : e );
  var exp = [ -15 ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.close( 'dst - new container' );

  /* - */

  test.open( 'dst - src' );

  test.case = 'without onEach, has one minimum';
  var src = _.containerAdapter.make( new Set( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] ) );
  var got = src.least( src );
  var exp = [ -20 ];
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.case = 'without onEach, has minimum - -Infinity';
  var src = _.containerAdapter.make( new Set( [ -10, 5, -15, 4, -Infinity, 2, 15, 21, -20 ] ) );
  var got = src.least( src );
  var exp = [ -Infinity ];
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.case = 'without onEach, has minimum, small difference';
  var src = _.containerAdapter.make( new Set( [ -10, 5, -15, -20.0001, 4, 1, 2, 15, 21, -20 ] ) );
  var got = src.least( src );
  var exp = [ -20.0001 ];
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.case = 'without onEach, all equal';
  var src = _.containerAdapter.make( new Set( [ -1, -1, -1, -1 ] ) );
  var got = src.least( src );
  var exp = [ -1 ];
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.case = 'without onEach, all -Infinity';
  var src = _.containerAdapter.make( new Set( [ -Infinity, -Infinity, -Infinity, -Infinity ] ) );
  var got = src.least( src );
  var exp = [ -Infinity ];
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.case = 'without onEach, all Infinity';
  var src = _.containerAdapter.make( new Set( [ Infinity, Infinity, Infinity, Infinity ] ) );
  var got = src.least( src );
  var exp = [ Infinity ];
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.case = 'without onEach, all NaN';
  var src = _.containerAdapter.make( new Set( [ NaN, NaN, NaN, NaN ] ) );
  var got = src.least( src );
  var exp = [];
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.case = 'without onEach, has minimum and NaN';
  var src = _.containerAdapter.make( new Set( [ -10, 5, -15, '4', 1, NaN, 15, -1, 21, '-20' ] ) );
  var got = src.least( src );
  var exp = [];
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  /* */

  test.case = 'onEach, has minimum';
  var src = _.containerAdapter.make( new Set( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] ) );
  var got = src.least( src, ( e ) => e < 0 ? -e : e );
  var exp = [ 1 ];
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.case = 'onEach, has minimum - -Infinity';
  var src = _.containerAdapter.make( new Set( [ -10, 5, -15, 4, -Infinity, 2, 15, 21, -20 ] ) );
  var got = src.least( src, ( e ) => e < 0 ? -e : e );
  var exp = [ 2 ];
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.case = 'onEach, has minimum, small difference';
  var src = _.containerAdapter.make( new Set( [ -10, 5, -15, -20.0001, 4, 1, 2, 15, 21, -20 ] ) );
  var got = src.least( src, ( e ) => e < 0 ? -e : e );
  var exp = [ 1 ];
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.case = 'onEach, all equal';
  var src = _.containerAdapter.make( new Set( [ -1, -1, -1, -1 ] ) );
  var got = src.least( src, ( e ) => isNaN( e ) || _.strIs( e ) ? 0 : e );
  var exp = [ -1 ];
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.case = 'onEach, all -Infinity';
  var src = _.containerAdapter.make( new Set( [ -Infinity, -Infinity, -Infinity, -Infinity ] ) );
  var got = src.least( src, ( e ) => isNaN( e ) || _.strIs( e ) ? 0 : e );
  var exp = [ -Infinity ];
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.case = 'onEach, all Infinity';
  var src = _.containerAdapter.make( new Set( [ Infinity, Infinity, Infinity, Infinity ] ) );
  var got = src.least( src, ( e ) => e < 0 ? -e : e );
  var exp = [ Infinity ];
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.case = 'onEach, all NaN';
  var src = _.containerAdapter.make( new Set( [ NaN, NaN, NaN, NaN ] ) );
  var got = src.least( src, ( e ) => isNaN( e ) ? 0 : e );
  var exp = [];
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.case = 'onEach, has minimum and NaN';
  var src = _.containerAdapter.make( new Set( [ -10, 5, -15, '4', 1, NaN, 15, -1, 21, '-20' ] ) );
  var got = src.least( src, ( e ) => isNaN( e ) || _.strIs( e ) ? 0 : e );
  var exp = [ -15 ];
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.close( 'dst - src' );

  test.close( 'setContainerAdapter' );
}

//

function most( test )
{
  test.open( 'arrayContainerAdapter' );

  test.open( 'without dst' );

  test.case = 'without onEach, has one maximun';
  var src = _.containerAdapter.make( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
  var got = src.most();
  var exp = [ 21 ];
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'without onEach, has maximun - -Infinity';
  var src = _.containerAdapter.make( [ -10, 5, -15, 4, Infinity, 2, 15, 21, -20 ] );
  var got = src.most();
  var exp = [ Infinity ];
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'without onEach, has maximun, small difference';
  var src = _.containerAdapter.make( [ -10, 5, -15, 21.0001, 4, 1, 2, 15, 21, -20 ] );
  var got = src.most();
  var exp = [ 21.0001 ];
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'without onEach, all equal';
  var src = _.containerAdapter.make( [ -1, -1, -1, -1 ] );
  var got = src.most();
  var exp = [ -1, -1, -1, -1 ];
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'without onEach, all -Infinity';
  var src = _.containerAdapter.make( [ -Infinity, -Infinity, -Infinity, -Infinity ] );
  var got = src.most();
  var exp = [ -Infinity, -Infinity, -Infinity, -Infinity ];
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'without onEach, all Infinity';
  var src = _.containerAdapter.make( [ Infinity, Infinity, Infinity, Infinity ] );
  var got = src.most();
  var exp = [ Infinity, Infinity, Infinity, Infinity ];
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'without onEach, all NaN';
  var src = _.containerAdapter.make( [ NaN, NaN, NaN, NaN ] );
  var got = src.most();
  var exp = [];
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'without onEach, has maximun and NaN';
  var src = _.containerAdapter.make( [ -10, 5, -15, '4', 1, NaN, 15, -1, 21, '-20' ] );
  var got = src.most();
  var exp = [];
  test.true( got !== src );
  test.identical( got.original, exp );

  /* */

  test.case = 'onEach, has maximun';
  var src = _.containerAdapter.make( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
  var got = src.most( ( e ) => e < 0 ? -e : e );
  var exp = [ 21 ];
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'onEach, has maximun - Infinity';
  var src = _.containerAdapter.make( [ -10, 5, -15, 4, Infinity, 2, 15, 21, -20 ] );
  var got = src.most( ( e ) => e < 0 ? -e : e );
  var exp = [ Infinity ];
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'onEach, has maximun, small difference';
  var src = _.containerAdapter.make( [ -10, 5, -15, 21.0001, 4, 1, 2, 15, 21, -20 ] );
  var got = src.most( ( e ) => e < 0 ? -e : e );
  var exp = [ 21.0001 ];
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'onEach, all equal';
  var src = _.containerAdapter.make( [ -1, -1, -1, -1 ] );
  var got = src.most( ( e ) => isNaN( e ) || _.strIs( e ) ? 0 : e );
  var exp = [ -1, -1, -1, -1 ];
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'onEach, all -Infinity';
  var src = _.containerAdapter.make( [ -Infinity, -Infinity, -Infinity, -Infinity ] );
  var got = src.most( ( e ) => isNaN( e ) || _.strIs( e ) ? 0 : e );
  var exp = [ -Infinity, -Infinity, -Infinity, -Infinity ];
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'onEach, all Infinity';
  var src = _.containerAdapter.make( [ Infinity, Infinity, Infinity, Infinity ] );
  var got = src.most( ( e ) => e < 0 ? -e : e );
  var exp = [ Infinity, Infinity, Infinity, Infinity ];
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'onEach, all NaN';
  var src = _.containerAdapter.make( [ NaN, NaN, NaN, NaN ] );
  var got = src.most( ( e ) => isNaN( e ) ? 0 : e );
  var exp = [ NaN, NaN, NaN, NaN ];
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'onEach, has maximun and NaN';
  var src = _.containerAdapter.make( [ -10, 5, -15, '4', 1, NaN, 15, -1, 21, '-20' ] );
  var got = src.most( ( e ) => isNaN( e ) || _.strIs( e ) ? 0 : e );
  var exp = [ 21 ];
  test.true( got !== src );
  test.identical( got.original, exp );

  test.close( 'without dst' );

  /* - */

  test.open( 'dst - null' );

  test.case = 'without onEach, has one maximun';
  var src = _.containerAdapter.make( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
  var got = src.most( null );
  var exp = [ 21 ];
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'without onEach, has maximun - Infinity';
  var src = _.containerAdapter.make( [ -10, 5, -15, 4, Infinity, 2, 15, 21, -20 ] );
  var got = src.most( null );
  var exp = [ Infinity ];
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'without onEach, has maximun, small difference';
  var src = _.containerAdapter.make( [ -10, 5, -15, 21.0001, 4, 1, 2, 15, 21, -20 ] );
  var got = src.most( null );
  var exp = [ 21.0001 ];
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'without onEach, all equal';
  var src = _.containerAdapter.make( [ -1, -1, -1, -1 ] );
  var got = src.most( null );
  var exp = [ -1, -1, -1, -1 ];
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'without onEach, all -Infinity';
  var src = _.containerAdapter.make( [ -Infinity, -Infinity, -Infinity, -Infinity ] );
  var got = src.most( null );
  var exp = [ -Infinity, -Infinity, -Infinity, -Infinity ];
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'without onEach, all Infinity';
  var src = _.containerAdapter.make( [ Infinity, Infinity, Infinity, Infinity ] );
  var got = src.most( null );
  var exp = [ Infinity, Infinity, Infinity, Infinity ];
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'without onEach, all NaN';
  var src = _.containerAdapter.make( [ NaN, NaN, NaN, NaN ] );
  var got = src.most( null );
  var exp = [];
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'without onEach, has maximun and NaN';
  var src = _.containerAdapter.make( [ -10, 5, -15, '4', 1, NaN, 15, -1, 21, '-20' ] );
  var got = src.most( null );
  var exp = [];
  test.true( got !== src );
  test.identical( got.original, exp );

  /* */

  test.case = 'onEach, has maximun';
  var src = _.containerAdapter.make( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
  var got = src.most( null, ( e ) => e < 0 ? -e : e );
  var exp = [ 21 ];
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'onEach, has maximun - Infinity';
  var src = _.containerAdapter.make( [ -10, 5, -15, 4, -Infinity, 2, 15, 21, -20 ] );
  var got = src.most( null, ( e ) => e < 0 ? -e : e );
  var exp = [ -Infinity ];
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'onEach, has maximun, small difference';
  var src = _.containerAdapter.make( [ -10, 5, -15, 21.0001, 4, 1, 2, 15, 21, -20 ] );
  var got = src.most( null, ( e ) => e < 0 ? -e : e );
  var exp = [ 21.0001 ];
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'onEach, all equal';
  var src = _.containerAdapter.make( [ -1, -1, -1, -1 ] );
  var got = src.most( null, ( e ) => isNaN( e ) || _.strIs( e ) ? 0 : e );
  var exp = [ -1, -1, -1, -1 ];
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'onEach, all -Infinity';
  var src = _.containerAdapter.make( [ -Infinity, -Infinity, -Infinity, -Infinity ] );
  var got = src.most( null, ( e ) => isNaN( e ) || _.strIs( e ) ? 0 : e );
  var exp = [ -Infinity, -Infinity, -Infinity, -Infinity ];
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'onEach, all Infinity';
  var src = _.containerAdapter.make( [ Infinity, Infinity, Infinity, Infinity ] );
  var got = src.most( null, ( e ) => e < 0 ? -e : e );
  var exp = [ Infinity, Infinity, Infinity, Infinity ];
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'onEach, all NaN';
  var src = _.containerAdapter.make( [ NaN, NaN, NaN, NaN ] );
  var got = src.most( null, ( e ) => isNaN( e ) ? 0 : e );
  var exp = [ NaN, NaN, NaN, NaN ];
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'onEach, has maximun and NaN';
  var src = _.containerAdapter.make( [ -10, 5, -15, '4', 1, NaN, 15, -1, 21, '-20' ] );
  var got = src.most( null, ( e ) => isNaN( e ) || _.strIs( e ) ? 0 : e );
  var exp = [ 21 ];
  test.true( got !== src );
  test.identical( got.original, exp );

  test.close( 'dst - null' );

  /* - */

  test.open( 'dst - new container' );

  test.case = 'without onEach, has one maximun';
  var dst = _.containerAdapter.make( [] );
  var src = _.containerAdapter.make( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
  var got = src.most( dst );
  var exp = [ 21 ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'without onEach, has maximun - Infinity';
  var dst = _.containerAdapter.make( [] );
  var src = _.containerAdapter.make( [ -10, 5, -15, 4, Infinity, 2, 15, 21, -20 ] );
  var got = src.most( dst );
  var exp = [ Infinity ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'without onEach, has maximun, small difference';
  var dst = _.containerAdapter.make( [] );
  var src = _.containerAdapter.make( [ -10, 5, -15, 21.0001, 4, 1, 2, 15, 21, -20 ] );
  var got = src.most( dst );
  var exp = [ 21.0001 ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'without onEach, all equal';
  var dst = _.containerAdapter.make( [] );
  var src = _.containerAdapter.make( [ -1, -1, -1, -1 ] );
  var got = src.most( dst );
  var exp = [ -1, -1, -1, -1 ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'without onEach, all -Infinity';
  var dst = _.containerAdapter.make( [] );
  var src = _.containerAdapter.make( [ -Infinity, -Infinity, -Infinity, -Infinity ] );
  var got = src.most( dst );
  var exp = [ -Infinity, -Infinity, -Infinity, -Infinity ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'without onEach, all Infinity';
  var dst = _.containerAdapter.make( [] );
  var src = _.containerAdapter.make( [ Infinity, Infinity, Infinity, Infinity ] );
  var got = src.most( dst );
  var exp = [ Infinity, Infinity, Infinity, Infinity ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'without onEach, all NaN';
  var dst = _.containerAdapter.make( [] );
  var src = _.containerAdapter.make( [ NaN, NaN, NaN, NaN ] );
  var got = src.most( dst );
  var exp = [];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'without onEach, has maximun and NaN';
  var dst = _.containerAdapter.make( [] );
  var src = _.containerAdapter.make( [ -10, 5, -15, '4', 1, NaN, 15, -1, 21, '-20' ] );
  var got = src.most( dst );
  var exp = [];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  /* */

  test.case = 'onEach, has maximun';
  var dst = _.containerAdapter.make( [] );
  var src = _.containerAdapter.make( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
  var got = src.most( dst, ( e ) => e < 0 ? -e : e );
  var exp = [ 21 ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'onEach, has maximun - -Infinity';
  var dst = _.containerAdapter.make( [] );
  var src = _.containerAdapter.make( [ -10, 5, -15, 4, -Infinity, 2, 15, 21, -20 ] );
  var got = src.most( dst, ( e ) => e < 0 ? -e : e );
  var exp = [ -Infinity ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'onEach, has maximun, small difference';
  var dst = _.containerAdapter.make( [] );
  var src = _.containerAdapter.make( [ -10, 5, -15, 21.0001, 4, 1, 2, 15, 21, -20 ] );
  var got = src.most( dst, ( e ) => e < 0 ? -e : e );
  var exp = [ 21.0001 ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'onEach, all equal';
  var dst = _.containerAdapter.make( [] );
  var src = _.containerAdapter.make( [ -1, -1, -1, -1 ] );
  var got = src.most( dst, ( e ) => isNaN( e ) || _.strIs( e ) ? 0 : e );
  var exp = [ -1, -1, -1, -1 ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'onEach, all -Infinity';
  var dst = _.containerAdapter.make( [] );
  var src = _.containerAdapter.make( [ -Infinity, -Infinity, -Infinity, -Infinity ] );
  var got = src.most( dst, ( e ) => isNaN( e ) || _.strIs( e ) ? 0 : e );
  var exp = [ -Infinity, -Infinity, -Infinity, -Infinity ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'onEach, all Infinity';
  var dst = _.containerAdapter.make( [] );
  var src = _.containerAdapter.make( [ Infinity, Infinity, Infinity, Infinity ] );
  var got = src.most( dst, ( e ) => e < 0 ? -e : e );
  var exp = [ Infinity, Infinity, Infinity, Infinity ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'onEach, all NaN';
  var dst = _.containerAdapter.make( [] );
  var src = _.containerAdapter.make( [ NaN, NaN, NaN, NaN ] );
  var got = src.most( dst, ( e ) => isNaN( e ) ? 0 : e );
  var exp = [ NaN, NaN, NaN, NaN ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'onEach, has maximun and NaN';
  var dst = _.containerAdapter.make( [] );
  var src = _.containerAdapter.make( [ -10, 5, -15, '4', 1, NaN, 15, -1, 21, '-20' ] );
  var got = src.most( dst, ( e ) => isNaN( e ) || _.strIs( e ) ? 0 : e );
  var exp = [ 21 ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  /* */

  test.case = 'onEach, has maximun';
  var dst = _.containerAdapter.make( new Set() );
  var src = _.containerAdapter.make( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
  var got = src.most( dst, ( e ) => e < 0 ? -e : e );
  var exp = [ 21 ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'onEach, has maximun - -Infinity';
  var dst = _.containerAdapter.make( new Set() );
  var src = _.containerAdapter.make( [ -10, 5, -15, 4, -Infinity, 2, 15, 21, -20 ] );
  var got = src.most( dst, ( e ) => e < 0 ? -e : e );
  var exp = [ -Infinity ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'onEach, has maximun, small difference';
  var dst = _.containerAdapter.make( new Set() );
  var src = _.containerAdapter.make( [ -10, 5, -15, 21.0001, 4, 1, 2, 15, 21, -20 ] );
  var got = src.most( dst, ( e ) => e < 0 ? -e : e );
  var exp = [ 21.0001 ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'onEach, all equal';
  var dst = _.containerAdapter.make( new Set() );
  var src = _.containerAdapter.make( [ -1, -1, -1, -1 ] );
  var got = src.most( dst, ( e ) => isNaN( e ) || _.strIs( e ) ? 0 : e );
  var exp = [ -1 ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'onEach, all -Infinity';
  var dst = _.containerAdapter.make( new Set() );
  var src = _.containerAdapter.make( [ -Infinity, -Infinity, -Infinity, -Infinity ] );
  var got = src.most( dst, ( e ) => isNaN( e ) || _.strIs( e ) ? 0 : e );
  var exp = [ -Infinity ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'onEach, all Infinity';
  var dst = _.containerAdapter.make( new Set() );
  var src = _.containerAdapter.make( [ Infinity, Infinity, Infinity, Infinity ] );
  var got = src.most( dst, ( e ) => e < 0 ? -e : e );
  var exp = [ Infinity ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'onEach, all NaN';
  var dst = _.containerAdapter.make( new Set() );
  var src = _.containerAdapter.make( [ NaN, NaN, NaN, NaN ] );
  var got = src.most( dst, ( e ) => isNaN( e ) ? 0 : e );
  var exp = [ NaN ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'onEach, has maximun and NaN';
  var dst = _.containerAdapter.make( new Set() );
  var src = _.containerAdapter.make( [ -10, 5, -15, '4', 1, NaN, 15, -1, 21, '-20' ] );
  var got = src.most( dst, ( e ) => isNaN( e ) || _.strIs( e ) ? 0 : e );
  var exp = [ 21 ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.close( 'dst - new container' );

  /* - */

  test.open( 'dst - src' );

  test.case = 'without onEach, has one maximun';
  var src = _.containerAdapter.make( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
  var got = src.most( src );
  var exp = [ 21 ];
  test.true( got === src );
  test.identical( got.original, exp );

  test.case = 'without onEach, has maximun - Infinity';
  var src = _.containerAdapter.make( [ -10, 5, -15, 4, Infinity, 2, 15, 21, -20 ] );
  var got = src.most( src );
  var exp = [ Infinity ];
  test.true( got === src );
  test.identical( got.original, exp );

  test.case = 'without onEach, has maximun, small difference';
  var src = _.containerAdapter.make( [ -10, 5, -15, 21.0001, 4, 1, 2, 15, 21, -20 ] );
  var got = src.most( src );
  var exp = [ 21.0001 ];
  test.true( got === src );
  test.identical( got.original, exp );

  test.case = 'without onEach, all equal';
  var src = _.containerAdapter.make( [ -1, -1, -1, -1 ] );
  var got = src.most( src );
  var exp = [ -1, -1, -1, -1 ];
  test.true( got === src );
  test.identical( got.original, exp );

  test.case = 'without onEach, all -Infinity';
  var src = _.containerAdapter.make( [ -Infinity, -Infinity, -Infinity, -Infinity ] );
  var got = src.most( src );
  var exp = [ -Infinity, -Infinity, -Infinity, -Infinity ];
  test.true( got === src );
  test.identical( got.original, exp );

  test.case = 'without onEach, all Infinity';
  var src = _.containerAdapter.make( [ Infinity, Infinity, Infinity, Infinity ] );
  var got = src.most( src );
  var exp = [ Infinity, Infinity, Infinity, Infinity ];
  test.true( got === src );
  test.identical( got.original, exp );

  test.case = 'without onEach, all NaN';
  var src = _.containerAdapter.make( [ NaN, NaN, NaN, NaN ] );
  var got = src.most( src );
  var exp = [];
  test.true( got === src );
  test.identical( got.original, exp );

  test.case = 'without onEach, has maximun and NaN';
  var src = _.containerAdapter.make( [ -10, 5, -15, '4', 1, NaN, 15, -1, 21, '-20' ] );
  var got = src.most( src );
  var exp = [];
  test.true( got === src );
  test.identical( got.original, exp );

  /* */

  test.case = 'onEach, has maximun';
  var src = _.containerAdapter.make( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
  var got = src.most( src, ( e ) => e < 0 ? -e : e );
  var exp = [ 21 ];
  test.true( got === src );
  test.identical( got.original, exp );

  test.case = 'onEach, has maximun - -Infinity';
  var src = _.containerAdapter.make( [ -10, 5, -15, 4, -Infinity, 2, 15, 21, -20 ] );
  var got = src.most( src, ( e ) => e < 0 ? -e : e );
  var exp = [ -Infinity ];
  test.true( got === src );
  test.identical( got.original, exp );

  test.case = 'onEach, has maximun, small difference';
  var src = _.containerAdapter.make( [ -10, 5, -15, 21.0001, 4, 1, 2, 15, 21, -20 ] );
  var got = src.most( src, ( e ) => e < 0 ? -e : e );
  var exp = [ 21.0001 ];
  test.true( got === src );
  test.identical( got.original, exp );

  test.case = 'onEach, all equal';
  var src = _.containerAdapter.make( [ -1, -1, -1, -1 ] );
  var got = src.most( src, ( e ) => isNaN( e ) || _.strIs( e ) ? 0 : e );
  var exp = [ -1, -1, -1, -1 ];
  test.true( got === src );
  test.identical( got.original, exp );

  test.case = 'onEach, all -Infinity';
  var src = _.containerAdapter.make( [ -Infinity, -Infinity, -Infinity, -Infinity ] );
  var got = src.most( src, ( e ) => isNaN( e ) || _.strIs( e ) ? 0 : e );
  var exp = [ -Infinity, -Infinity, -Infinity, -Infinity ];
  test.true( got === src );
  test.identical( got.original, exp );

  test.case = 'onEach, all Infinity';
  var src = _.containerAdapter.make( [ Infinity, Infinity, Infinity, Infinity ] );
  var got = src.most( src, ( e ) => e < 0 ? -e : e );
  var exp = [ Infinity, Infinity, Infinity, Infinity ];
  test.true( got === src );
  test.identical( got.original, exp );

  test.case = 'onEach, all NaN';
  var src = _.containerAdapter.make( [ NaN, NaN, NaN, NaN ] );
  var got = src.most( src, ( e ) => isNaN( e ) ? 0 : e );
  var exp = [ NaN, NaN, NaN, NaN ];
  test.true( got === src );
  test.identical( got.original, exp );

  test.case = 'onEach, has maximun and NaN';
  var src = _.containerAdapter.make( [ -10, 5, -15, '4', 1, NaN, 15, -1, 21, '-20' ] );
  var got = src.most( src, ( e ) => isNaN( e ) || _.strIs( e ) ? 0 : e );
  var exp = [ 21 ];
  test.true( got === src );
  test.identical( got.original, exp );

  test.close( 'dst - src' );

  test.close( 'arrayContainerAdapter' );

  /* - */

  test.open( 'setContainerAdapter' );

  test.open( 'without dst' );

  test.case = 'without onEach, has one maximun';
  var src = _.containerAdapter.make( new Set( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] ) );
  var got = src.most();
  var exp = [ 21 ];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'without onEach, has maximun - Infinity';
  var src = _.containerAdapter.make( new Set( [ -10, 5, -15, 4, Infinity, 2, 15, 21, -20 ] ) );
  var got = src.most();
  var exp = [ Infinity ];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'without onEach, has maximun, small difference';
  var src = _.containerAdapter.make( new Set(  [ -10, 5, -15, 21.0001, 4, 1, 2, 15, 21, -20 ] ) );
  var got = src.most();
  var exp = [ 21.0001 ];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'without onEach, all equal';
  var src = _.containerAdapter.make( new Set( [ -1, -1, -1, -1 ] ) );
  var got = src.most();
  var exp = [ -1 ];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'without onEach, all -Infinity';
  var src = _.containerAdapter.make( new Set( [ -Infinity, -Infinity, -Infinity, -Infinity ] ) );
  var got = src.most();
  var exp = [ -Infinity ];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'without onEach, all Infinity';
  var src = _.containerAdapter.make( new Set( [ Infinity, Infinity, Infinity, Infinity ] ) );
  var got = src.most();
  var exp = [ Infinity ];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'without onEach, all NaN';
  var src = _.containerAdapter.make( new Set( [ NaN, NaN, NaN, NaN ] ) );
  var got = src.most();
  var exp = [];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'without onEach, has maximun and NaN';
  var src = _.containerAdapter.make( new Set( [ -10, 5, -15, '4', 1, NaN, 15, -1, 21, '-20' ] ) );
  var got = src.most();
  var exp = [];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  /* */

  test.case = 'onEach, has maximun';
  var src = _.containerAdapter.make( new Set( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] ) );
  var got = src.most( ( e ) => e < 0 ? -e : e );
  var exp = [ 21 ];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'onEach, has maximun - -Infinity';
  var src = _.containerAdapter.make( new Set( [ -10, 5, -15, 4, -Infinity, 2, 15, 21, -20 ] ) );
  var got = src.most( ( e ) => e < 0 ? -e : e );
  var exp = [ -Infinity ];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'onEach, has maximun, small difference';
  var src = _.containerAdapter.make( new Set( [ -10, 5, -15, 21.0001, 4, 1, 2, 15, 21, -20 ] ) );
  var got = src.most( ( e ) => e < 0 ? -e : e );
  var exp = [ 21.0001 ];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'onEach, all equal';
  var src = _.containerAdapter.make( new Set( [ -1, -1, -1, -1 ] ) );
  var got = src.most( ( e ) => isNaN( e ) || _.strIs( e ) ? 0 : e );
  var exp = [ -1 ];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'onEach, all -Infinity';
  var src = _.containerAdapter.make( new Set( [ -Infinity, -Infinity, -Infinity, -Infinity ] ) );
  var got = src.most( ( e ) => isNaN( e ) || _.strIs( e ) ? 0 : e );
  var exp = [ -Infinity ];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'onEach, all Infinity';
  var src = _.containerAdapter.make( new Set( [ Infinity, Infinity, Infinity, Infinity ] ) );
  var got = src.most( ( e ) => e < 0 ? -e : e );
  var exp = [ Infinity ];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'onEach, all NaN';
  var src = _.containerAdapter.make( new Set( [ NaN, NaN, NaN, NaN ] ) );
  var got = src.most( ( e ) => isNaN( e ) ? 0 : e );
  var exp = [ NaN ];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'onEach, has maximun and NaN';
  var src = _.containerAdapter.make( new Set( [ -10, 5, -15, '4', 1, NaN, 15, -1, 21, '-20' ] ) );
  var got = src.most( ( e ) => isNaN( e ) || _.strIs( e ) ? 0 : e );
  var exp = [ 21 ];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.close( 'without dst' );

  /* - */

  test.open( 'dst - null' );

  test.case = 'without onEach, has one maximun';
  var src = _.containerAdapter.make( new Set( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] ) );
  var got = src.most( null );
  var exp = [ 21 ];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'without onEach, has maximun - Infinity';
  var src = _.containerAdapter.make( new Set( [ -10, 5, -15, 4, Infinity, 2, 15, 21, -20 ] ) );
  var got = src.most( null );
  var exp = [ Infinity ];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'without onEach, has maximun, small difference';
  var src = _.containerAdapter.make( new Set( [ -10, 5, -15, 21.0001, 4, 1, 2, 15, 21, -20 ] ) );
  var got = src.most( null );
  var exp = [ 21.0001 ];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'without onEach, all equal';
  var src = _.containerAdapter.make( new Set( [ -1, -1, -1, -1 ] ) );
  var got = src.most( null );
  var exp = [ -1 ];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'without onEach, all -Infinity';
  var src = _.containerAdapter.make( new Set( [ -Infinity, -Infinity, -Infinity, -Infinity ] ) );
  var got = src.most( null );
  var exp = [ -Infinity ];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'without onEach, all Infinity';
  var src = _.containerAdapter.make( new Set( [ Infinity, Infinity, Infinity, Infinity ] ) );
  var got = src.most( null );
  var exp = [ Infinity ];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'without onEach, all NaN';
  var src = _.containerAdapter.make( new Set( [ NaN, NaN, NaN, NaN ] ) );
  var got = src.most( null );
  var exp = [];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'without onEach, has maximun and NaN';
  var src = _.containerAdapter.make( new Set( [ -10, 5, -15, '4', 1, NaN, 15, -1, 21, '-20' ] ) );
  var got = src.most( null );
  var exp = [];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  /* */

  test.case = 'onEach, has maximun';
  var src = _.containerAdapter.make( new Set( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] ) );
  var got = src.most( null, ( e ) => e < 0 ? -e : e );
  var exp = [ 21 ];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'onEach, has maximun - -Infinity';
  var src = _.containerAdapter.make( new Set( [ -10, 5, -15, 4, -Infinity, 2, 15, 21, -20 ] ) );
  var got = src.most( null, ( e ) => e < 0 ? -e : e );
  var exp = [ -Infinity ];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'onEach, has maximun, small difference';
  var src = _.containerAdapter.make( new Set( [ -10, 5, -15, -21.0001, 4, 1, 2, 15, 21, -20 ] ) );
  var got = src.most( null, ( e ) => e < 0 ? -e : e );
  var exp = [ -21.0001 ];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'onEach, all equal';
  var src = _.containerAdapter.make( new Set( [ -1, -1, -1, -1 ] ) );
  var got = src.most( null, ( e ) => isNaN( e ) || _.strIs( e ) ? 0 : e );
  var exp = [ -1 ];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'onEach, all -Infinity';
  var src = _.containerAdapter.make( new Set( [ -Infinity, -Infinity, -Infinity, -Infinity ] ) );
  var got = src.most( null, ( e ) => isNaN( e ) || _.strIs( e ) ? 0 : e );
  var exp = [ -Infinity ];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'onEach, all Infinity';
  var src = _.containerAdapter.make( new Set( [ Infinity, Infinity, Infinity, Infinity ] ) );
  var got = src.most( null, ( e ) => e < 0 ? -e : e );
  var exp = [ Infinity ];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'onEach, all NaN';
  var src = _.containerAdapter.make( new Set( [ NaN, NaN, NaN, NaN ] ) );
  var got = src.most( null, ( e ) => isNaN( e ) ? 0 : e );
  var exp = [ NaN ];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'onEach, has maximun and NaN';
  var src = _.containerAdapter.make( new Set( [ -10, 5, -15, '4', 1, NaN, 15, -1, 21, '-20' ] ) );
  var got = src.most( null, ( e ) => isNaN( e ) || _.strIs( e ) ? 0 : e );
  var exp = [ 21 ];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.close( 'dst - null' );

  /* - */

  test.open( 'dst - new container' );

  test.case = 'without onEach, has one maximun';
  var dst = _.containerAdapter.make( new Set( [] ) );
  var src = _.containerAdapter.make( new Set( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] ) );
  var got = src.most( dst );
  var exp = [ 21 ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'without onEach, has maximun - Infinity';
  var dst = _.containerAdapter.make( new Set( [] ) );
  var src = _.containerAdapter.make( new Set( [ -10, 5, -15, 4, Infinity, 2, 15, 21, -20 ] ) );
  var got = src.most( dst );
  var exp = [ Infinity ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'without onEach, has maximun, small difference';
  var dst = _.containerAdapter.make( new Set( [] ) );
  var src = _.containerAdapter.make( new Set( [ -10, 5, -15, 21.0001, 4, 1, 2, 15, 21, -20 ] ) );
  var got = src.most( dst );
  var exp = [ 21.0001 ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'without onEach, all equal';
  var dst = _.containerAdapter.make( new Set( [] ) );
  var src = _.containerAdapter.make( new Set( [ -1, -1, -1, -1 ] ) );
  var got = src.most( dst );
  var exp = [ -1 ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'without onEach, all -Infinity';
  var dst = _.containerAdapter.make( new Set( [] ) );
  var src = _.containerAdapter.make( new Set( [ -Infinity, -Infinity, -Infinity, -Infinity ] ) );
  var got = src.most( dst );
  var exp = [ -Infinity ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'without onEach, all Infinity';
  var dst = _.containerAdapter.make( new Set( [] ) );
  var src = _.containerAdapter.make( new Set( [ Infinity, Infinity, Infinity, Infinity ] ) );
  var got = src.most( dst );
  var exp = [ Infinity ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'without onEach, all NaN';
  var dst = _.containerAdapter.make( new Set( [] ) );
  var src = _.containerAdapter.make( new Set( [ NaN, NaN, NaN, NaN ] ) );
  var got = src.most( dst );
  var exp = [];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'without onEach, has maximun and NaN';
  var dst = _.containerAdapter.make( new Set( [] ) );
  var src = _.containerAdapter.make( new Set( [ -10, 5, -15, '4', 1, NaN, 15, -1, 21, '-20' ] ) );
  var got = src.most( dst );
  var exp = [];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  /* */

  test.case = 'onEach, has maximun';
  var dst = _.containerAdapter.make( new Set( [] ) );
  var src = _.containerAdapter.make( new Set( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] ) );
  var got = src.most( dst, ( e ) => e < 0 ? -e : e );
  var exp = [ 21 ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'onEach, has maximun - -Infinity';
  var dst = _.containerAdapter.make( new Set( [] ) );
  var src = _.containerAdapter.make( new Set( [ -10, 5, -15, 4, -Infinity, 2, 15, 21, -20 ] ) );
  var got = src.most( dst, ( e ) => e < 0 ? -e : e );
  var exp = [ -Infinity ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'onEach, has maximun, small difference';
  var dst = _.containerAdapter.make( new Set( [] ) );
  var src = _.containerAdapter.make( new Set( [ -10, 5, -15, 21.0001, 4, 1, 2, 15, 21, -20 ] ) );
  var got = src.most( dst, ( e ) => e < 0 ? -e : e );
  var exp = [ 21.0001 ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'onEach, all equal';
  var dst = _.containerAdapter.make( new Set( [] ) );
  var src = _.containerAdapter.make( new Set( [ -1, -1, -1, -1 ] ) );
  var got = src.most( dst, ( e ) => isNaN( e ) || _.strIs( e ) ? 0 : e );
  var exp = [ -1 ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'onEach, all -Infinity';
  var dst = _.containerAdapter.make( new Set( [] ) );
  var src = _.containerAdapter.make( new Set( [ -Infinity, -Infinity, -Infinity, -Infinity ] ) );
  var got = src.most( dst, ( e ) => isNaN( e ) || _.strIs( e ) ? 0 : e );
  var exp = [ -Infinity ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'onEach, all Infinity';
  var dst = _.containerAdapter.make( new Set( [] ) );
  var src = _.containerAdapter.make( new Set( [ Infinity, Infinity, Infinity, Infinity ] ) );
  var got = src.most( dst, ( e ) => e < 0 ? -e : e );
  var exp = [ Infinity ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'onEach, all NaN';
  var dst = _.containerAdapter.make( new Set( [] ) );
  var src = _.containerAdapter.make( new Set( [ NaN, NaN, NaN, NaN ] ) );
  var got = src.most( dst, ( e ) => isNaN( e ) ? 0 : e );
  var exp = [ NaN ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'onEach, has maximun and NaN';
  var dst = _.containerAdapter.make( new Set( [] ) );
  var src = _.containerAdapter.make( [ -10, 5, -15, '4', 1, NaN, 15, -1, 21, '-20' ] );
  var got = src.most( dst, ( e ) => isNaN( e ) || _.strIs( e ) ? 0 : e );
  var exp = [ 21 ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  /* */

  test.case = 'onEach, has maximun';
  var dst = _.containerAdapter.make( [] );
  var src = _.containerAdapter.make( new Set( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] ) );
  var got = src.most( dst, ( e ) => e < 0 ? -e : e );
  var exp = [ 21 ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'onEach, has maximun - -Infinity';
  var dst = _.containerAdapter.make( [] );
  var src = _.containerAdapter.make( new Set( [ -10, 5, -15, 4, -Infinity, 2, 15, 21, -20 ] ) );
  var got = src.most( dst, ( e ) => e < 0 ? -e : e );
  var exp = [ -Infinity ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'onEach, has maximun, small difference';
  var dst = _.containerAdapter.make( [] );
  var src = _.containerAdapter.make( new Set( [ -10, 5, -15, 21.0001, 4, 1, 2, 15, 21, -20 ] ) );
  var got = src.most( dst, ( e ) => e < 0 ? -e : e );
  var exp = [ 21.0001 ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'onEach, all equal';
  var dst = _.containerAdapter.make( [] );
  var src = _.containerAdapter.make( new Set( [ -1, -1, -1, -1 ] ) );
  var got = src.most( dst, ( e ) => isNaN( e ) || _.strIs( e ) ? 0 : e );
  var exp = [ -1 ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'onEach, all -Infinity';
  var dst = _.containerAdapter.make( [] );
  var src = _.containerAdapter.make( new Set( [ -Infinity, -Infinity, -Infinity, -Infinity ] ) );
  var got = src.most( dst, ( e ) => isNaN( e ) || _.strIs( e ) ? 0 : e );
  var exp = [ -Infinity ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'onEach, all Infinity';
  var dst = _.containerAdapter.make( [] );
  var src = _.containerAdapter.make( new Set( [ Infinity, Infinity, Infinity, Infinity ] ) );
  var got = src.most( dst, ( e ) => e < 0 ? -e : e );
  var exp = [ Infinity ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'onEach, all NaN';
  var dst = _.containerAdapter.make( [] );
  var src = _.containerAdapter.make( new Set( [ NaN, NaN, NaN, NaN ] ) );
  var got = src.most( dst, ( e ) => isNaN( e ) ? 0 : e );
  var exp = [ NaN ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'onEach, has maximun and NaN';
  var dst = _.containerAdapter.make( [] );
  var src = _.containerAdapter.make( new Set( [ -10, 5, -15, '4', 1, NaN, 15, -1, 21, '-20' ] ) );
  var got = src.most( dst, ( e ) => isNaN( e ) || _.strIs( e ) ? 0 : e );
  var exp = [ 21 ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.close( 'dst - new container' );

  /* - */

  test.open( 'dst - src' );

  test.case = 'without onEach, has one maximun';
  var src = _.containerAdapter.make( new Set( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] ) );
  var got = src.most( src );
  var exp = [ 21 ];
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.case = 'without onEach, has maximun - Infinity';
  var src = _.containerAdapter.make( new Set( [ -10, 5, -15, 4, Infinity, 2, 15, 21, -20 ] ) );
  var got = src.most( src );
  var exp = [ Infinity ];
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.case = 'without onEach, has maximun, small difference';
  var src = _.containerAdapter.make( new Set( [ -10, 5, -15, 21.0001, 4, 1, 2, 15, 21, -20 ] ) );
  var got = src.most( src );
  var exp = [ 21.0001 ];
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.case = 'without onEach, all equal';
  var src = _.containerAdapter.make( new Set( [ -1, -1, -1, -1 ] ) );
  var got = src.most( src );
  var exp = [ -1 ];
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.case = 'without onEach, all -Infinity';
  var src = _.containerAdapter.make( new Set( [ -Infinity, -Infinity, -Infinity, -Infinity ] ) );
  var got = src.most( src );
  var exp = [ -Infinity ];
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.case = 'without onEach, all Infinity';
  var src = _.containerAdapter.make( new Set( [ Infinity, Infinity, Infinity, Infinity ] ) );
  var got = src.most( src );
  var exp = [ Infinity ];
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.case = 'without onEach, all NaN';
  var src = _.containerAdapter.make( new Set( [ NaN, NaN, NaN, NaN ] ) );
  var got = src.most( src );
  var exp = [];
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.case = 'without onEach, has maximun and NaN';
  var src = _.containerAdapter.make( new Set( [ -10, 5, -15, '4', 1, NaN, 15, -1, 21, '-20' ] ) );
  var got = src.most( src );
  var exp = [];
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  /* */

  test.case = 'onEach, has maximun';
  var src = _.containerAdapter.make( new Set( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] ) );
  var got = src.most( src, ( e ) => e < 0 ? -e : e );
  var exp = [ 21 ];
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.case = 'onEach, has maximun - -Infinity';
  var src = _.containerAdapter.make( new Set( [ -10, 5, -15, 4, -Infinity, 2, 15, 21, -20 ] ) );
  var got = src.most( src, ( e ) => e < 0 ? -e : e );
  var exp = [ -Infinity ];
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.case = 'onEach, has maximun, small difference';
  var src = _.containerAdapter.make( new Set( [ -10, 5, -15, 21.0001, 4, 1, 2, 15, 21, -20 ] ) );
  var got = src.most( src, ( e ) => e < 0 ? -e : e );
  var exp = [ 21.0001 ];
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.case = 'onEach, all equal';
  var src = _.containerAdapter.make( new Set( [ -1, -1, -1, -1 ] ) );
  var got = src.most( src, ( e ) => isNaN( e ) || _.strIs( e ) ? 0 : e );
  var exp = [ -1 ];
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.case = 'onEach, all -Infinity';
  var src = _.containerAdapter.make( new Set( [ -Infinity, -Infinity, -Infinity, -Infinity ] ) );
  var got = src.most( src, ( e ) => isNaN( e ) || _.strIs( e ) ? 0 : e );
  var exp = [ -Infinity ];
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.case = 'onEach, all Infinity';
  var src = _.containerAdapter.make( new Set( [ Infinity, Infinity, Infinity, Infinity ] ) );
  var got = src.most( src, ( e ) => e < 0 ? -e : e );
  var exp = [ Infinity ];
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.case = 'onEach, all NaN';
  var src = _.containerAdapter.make( new Set( [ NaN, NaN, NaN, NaN ] ) );
  var got = src.most( src, ( e ) => isNaN( e ) ? 0 : e );
  var exp = [];
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.case = 'onEach, has maximun and NaN';
  var src = _.containerAdapter.make( new Set( [ -10, 5, -15, '4', 1, NaN, 15, -1, 21, '-20' ] ) );
  var got = src.most( src, ( e ) => isNaN( e ) || _.strIs( e ) ? 0 : e );
  var exp = [ 21 ];
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.close( 'dst - src' );

  test.close( 'setContainerAdapter' );
}

//

function onlyWithoutCallbacks( test )
{
  test.open( 'arrayContainerAdapter' );
  run( ( e ) => _.containerAdapter.make( e ) );
  test.close( 'arrayContainerAdapter' );

  /* - */

  test.open( 'setContainerAdapter' );
  run( ( e ) => _.containerAdapter.make( new Set( e ) ) );
  test.close( 'setContainerAdapter' );

  /* - */

  function run( makeSrc )
  {
    test.open( 'without dst' );

    test.case = 'dst - null';
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.only( null );
    var exp = [];
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - empty array';
    var src2 = [];
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.only( src2 );
    var exp = [];
    test.true( got !== src2 );
    test.true( got !== src );
    test.identical( [ ... got ], exp );

    test.case = 'dst - empty arrayAdapter';
    var src2 = _.containerAdapter.make( [] );
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.only( dst );
    var exp = [];
    test.true( got !== src2 );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - empty Set';
    var src2 = new Set();
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.only( src2 );
    var exp = [];
    test.true( got !== src2 );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - setAdapter';
    var src2 = _.containerAdapter.make( new Set( [ -10, 5 ] ) );
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.only( src2 );
    var exp = [ -10, 5 ];
    test.true( got !== src2 );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - src';
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.only( src );
    var exp = [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ];
    test.true( got === src );
    test.identical( [ ... got.original ], exp );

    test.close( 'without dst' );

    /* - */

    test.open( 'src2 is empty' );

    test.case = 'dst - null';
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.only( null, [] );
    var exp = [];
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - empty array';
    var dst = [];
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.only( dst, new Set() );
    var exp = [];
    test.true( got !== dst );
    test.true( got !== src );
    test.identical( got.original, exp );

    test.case = 'dst - empty arrayAdapter';
    var dst = _.containerAdapter.make( [] );
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var src2 = _.containerAdapter.make( [] );
    var got = src.only( dst, src2 );
    var exp = [];
    test.true( got === dst );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - empty Set';
    var dst = new Set( [ 2 ] );
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.only( dst, new Set() );
    var exp = [ 2 ];
    test.true( got !== dst );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - empty setAdapter';
    var dst = _.containerAdapter.make( new Set() );
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.only( dst, [] );
    var exp = [];
    test.true( got === dst );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - src';
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.only( src, [] );
    var exp = [];
    test.true( got === src );
    test.identical( [ ... got.original ], exp );

    test.close( 'src2 is empty' );

    /* */

    test.open( 'src2 == dst' );

    test.case = 'dst - empty array';
    var dst = [];
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var src2 = _.containerAdapter.make( new Set( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] ) );
    var got = src.only( dst, src2 );
    var exp = [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ];
    test.true( got !== dst );
    test.true( got !== src );
    test.identical( got.original, exp );

    test.case = 'dst - empty arrayAdapter';
    var dst = _.containerAdapter.make( [] );
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var src2 = _.containerAdapter.make( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.only( dst, src2 );
    var exp = [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ];
    test.true( got === dst );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - empty Set';
    var dst = new Set();
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var src2 = new Set( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.only( dst, src2 );
    var exp = [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ];
    test.true( got !== dst );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - empty setAdapter';
    var dst = _.containerAdapter.make( new Set() );
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var src2 = [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ];
    var got = src.only( dst, src2 );
    var exp = [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ];
    test.true( got === dst );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - src';
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var src2 = [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ];
    var got = src.only( src, src2 );
    var exp = [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ];
    test.true( got === src );
    test.identical( [ ... got.original ], exp );

    test.close( 'src2 == dst' );

    /* - */

    test.open( 'src2 != dst' );

    test.case = 'dst - empty array';
    var dst = [];
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var src2 = _.containerAdapter.make( new Set( [ -10, 6, -14, 4, 1, 1, 15, 21, -1 ] ) );
    var got = src.only( dst, src2 );
    var exp = [ -10, 4, 1, 15, 21 ];
    test.true( got !== dst );
    test.true( got !== src );
    test.identical( got.original, exp );

    test.case = 'dst - arrayAdapter';
    var dst = _.containerAdapter.make( [ 1 ] );
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var src2 = _.containerAdapter.make( [ -10, 6, -14, 4, 1, 1, 15, 21, -1 ] );
    var got = src.only( dst, src2 );
    var exp = [ 1, -10, 4, 15, 21 ];
    test.true( got === dst );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - Set';
    var dst = new Set( [ 1 ] );
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var src2 = new Set( [ -10, 6, -14, 4, 1, 1, 15, 21, -1 ] );
    var got = src.only( dst, src2 );
    var exp = new Set( [ 1, -10, 4, 1, 15, 21 ] );
    test.true( got !== dst );
    test.true( got !== src );
    test.identical( got.original, exp );

    test.case = 'dst - empty setAdapter';
    var dst = _.containerAdapter.make( new Set() );
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var src2 = [ -10, 6, -14, 4, 1, 1, 15, 21, -1 ];
    var got = src.only( dst, src2 );
    var exp = [ -10, 4, 1, 15, 21 ];
    test.true( got === dst );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - src';
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var src2 = [ -10, 6, -14, 4, 1, 1, 15, 21, -1 ];
    var got = src.only( src, src2 );
    var exp = [ -10, 4, 1, 15, 21 ];
    test.true( got === src );
    test.identical( [ ... got.original ], exp );

    test.close( 'src2 != dst' );
  }
}

//

function onlyOneEvaluator( test )
{
  test.open( 'arrayContainerAdapter' );
  run( ( e ) => _.containerAdapter.make( e ) );
  test.close( 'arrayContainerAdapter' );

  /* - */

  test.open( 'setContainerAdapter' );
  run( ( e ) => _.containerAdapter.make( new Set( e ) ) );
  test.close( 'setContainerAdapter' );

  /* - */

  function run( makeSrc )
  {
    test.open( 'without dst' );

    test.case = 'dst - null';
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.only( null, ( e ) => e );
    var exp = [];
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - empty array';
    var src2 = [];
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.only( src2, ( e ) => e );
    var exp = [];
    test.true( got !== src2 );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - empty arrayAdapter';
    var src2 = _.containerAdapter.make( [] );
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.only( src2, ( e ) => e );
    var exp = [];
    test.true( got !== src2 );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - empty Set';
    var src2 = new Set();
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.only( src2, ( e ) => e );
    var exp = [];
    test.true( got !== src2 );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - empty setAdapter';
    var src2 = _.containerAdapter.make( new Set() );
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.only( src2, ( e ) => e );
    var exp = [];
    test.true( got !== src2 );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - src';
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.only( src, ( e ) => e );
    var exp = [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ];
    test.true( got === src );
    test.identical( [ ... got.original ], exp );

    test.close( 'without dst' );

    /* - */

    test.open( 'src2 is empty' );

    test.case = 'dst - null';
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.only( null, [], ( e ) => e );
    var exp = [];
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - empty array';
    var dst = [];
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.only( dst, new Set(), ( e ) => e );
    var exp = [];
    test.true( got !== dst );
    test.true( got !== src );
    test.identical( got.original, exp );

    test.case = 'dst - empty arrayAdapter';
    var dst = _.containerAdapter.make( [] );
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var src2 = _.containerAdapter.make( [] );
    var got = src.only( dst, src2, ( e ) => e );
    var exp = [];
    test.true( got === dst );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - empty Set';
    var dst = new Set();
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.only( dst, new Set(), ( e ) => e );
    var exp = [];
    test.true( got !== dst );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - empty setAdapter';
    var dst = _.containerAdapter.make( new Set() );
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.only( dst, [], ( e ) => e );
    var exp = [];
    test.true( got === dst );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - src';
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.only( src, [], ( e ) => e );
    var exp = [];
    test.true( got === src );
    test.identical( [ ... got.original ], exp );

    test.close( 'src2 is empty' );

    /* */

    test.open( 'src2 == dst' );

    test.case = 'dst - empty array';
    var dst = [];
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var src2 = _.containerAdapter.make( new Set( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] ) );
    var got = src.only( dst, src2, ( e ) => e );
    var exp = [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ];
    test.true( got !== dst );
    test.true( got !== src );
    test.identical( got.original, exp );

    test.case = 'dst - empty arrayAdapter';
    var dst = _.containerAdapter.make( [] );
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var src2 = _.containerAdapter.make( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.only( dst, src2, ( e ) => e );
    var exp = [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ];
    test.true( got === dst );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - empty Set';
    var dst = new Set();
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var src2 = new Set( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.only( dst, src2, ( e ) => e );
    var exp = [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ];
    test.true( got !== dst );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - setAdapter';
    var dst = _.containerAdapter.make( new Set( [ -20, 5 ] ) );
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20, [ 22 ], [ 20 ] ] );
    var src2 = [ -10, 5, -15, 4, 1, 2, 15, [ 21 ], [ -20 ], [ 22 ], [ 20 ] ];
    var got = src.only( dst, src2, ( e ) => e[ 0 ] );
    var exp = [ -20, 5, -10, -15, 4, 1, 2, 15, [ 22 ], [ 20 ] ];
    test.true( got === dst );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - src';
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var src2 = [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ];
    var got = src.only( src, src2, ( e ) => e );
    var exp = [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ];
    test.true( got === src );
    test.identical( [ ... got.original ], exp );

    test.close( 'src2 == dst' );

    /* - */

    test.open( 'src2 != dst' );

    test.case = 'dst - empty array';
    var dst = [];
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var src2 = _.containerAdapter.make( new Set( [ -10, 6, -14, 4, 1, 1, 15, 21, -1 ] ) );
    var got = src.only( dst, src2, ( e ) => e );
    var exp = [ -10, 4, 1, 15, 21 ];
    test.true( got !== dst );
    test.true( got !== src );
    test.identical( got.original, exp );

    test.case = 'dst - arrayAdapter';
    var dst = _.containerAdapter.make( [ 1 ] );
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var src2 = _.containerAdapter.make( [ -10, 6, -14, 4, 1, 1, 15, 21, -1 ] );
    var got = src.only( dst, src2, ( e ) => e );
    var exp = [ 1, -10, 4, 15, 21 ];
    test.true( got === dst );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - empty Set';
    var dst = new Set();
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20, [ 22 ] ] );
    var src2 = new Set( [ -10, 6, -14, 4, 1, 1, 15, 21, -1, [ 22 ] ] );
    var got = src.only( dst, src2, ( e ) => e[ 0 ] );
    var exp = [ -10, 6, -14, 4, 1, 15, 21, -1, [ 22 ] ];
    test.true( got !== dst );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - Set';
    var dst = new Set( [ [ 22 ] ] );
    var src = makeSrc( [ -15, 4, 1, 2, 15, 21, -20, [ 22 ] ] );
    var src2 = new Set( [ [ 22 ], -10, 6, -14, 4, 1, 1, 15, 21, -1 ] );
    var got = src.only( dst, src2, ( e ) => e[ 0 ] );
    var exp = [ [ 22 ], [ 22 ], -10, 6, -14, 4, 1, 15, 21, -1 ];
    test.true( got !== dst );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - empty setAdapter';
    var dst = _.containerAdapter.make( new Set() );
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var src2 = [ -10, 6, -14, 4, 1, 1, 15, 21, -1 ];
    var got = src.only( dst, src2, ( e ) => e );
    var exp = [ -10, 4, 1, 15, 21 ];
    test.true( got === dst );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - src';
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var src2 = [ -10, 6, -14, 4, 1, 1, 15, 21, -1 ];
    var got = src.only( src, src2, ( e ) => e );
    var exp = [ -10, 4, 1, 15, 21 ];
    test.true( got === src );
    test.identical( [ ... got.original ], exp );

    test.close( 'src2 != dst' );
  }
}

//

function onlyTwoEvaluators( test )
{
  test.open( 'arrayContainerAdapter' );
  run( ( e ) => _.containerAdapter.make( e ) );
  test.close( 'arrayContainerAdapter' );

  /* - */

  test.open( 'setContainerAdapter' );
  run( ( e ) => _.containerAdapter.make( new Set( e ) ) );
  test.close( 'setContainerAdapter' );

  /* - */

  function run( makeSrc )
  {
    test.open( 'without dst' );

    test.case = 'dst - null';
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.only( null, ( e ) => e, ( ins ) => ins );
    var exp = [];
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - empty array';
    var src2 = [];
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.only( src2, ( e ) => e, ( ins ) => ins );
    var exp = [];
    test.true( got !== src2 );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - empty arrayAdapter';
    var src2 = _.containerAdapter.make( [] );
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.only( src2, ( e ) => e, ( ins ) => ins );
    var exp = [];
    test.true( got !== src2 );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - empty Set';
    var src2 = new Set();
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.only( src2, ( e ) => e, ( ins ) => ins );
    var exp = [];
    test.true( got !== src2 );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - empty setAdapter';
    var src2 = _.containerAdapter.make( new Set() );
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.only( src2, ( e ) => e, ( ins ) => ins );
    var exp = [];
    test.true( got !== src2 );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - src';
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.only( src, ( e ) => e, ( ins ) => ins );
    var exp = [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ];
    test.true( got === src );
    test.identical( [ ... got.original ], exp );

    test.close( 'without dst' );

    /* - */

    test.open( 'src2 is empty' );

    test.case = 'dst - null';
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.only( null, [], ( e ) => e, ( ins ) => ins );
    var exp = [];
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - empty array';
    var dst = [];
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.only( dst, new Set(), ( e ) => e, ( ins ) => ins );
    var exp = [];
    test.true( got !== dst );
    test.true( got !== src );
    test.identical( got.original, exp );

    test.case = 'dst - empty arrayAdapter';
    var dst = _.containerAdapter.make( [] );
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var src2 = _.containerAdapter.make( [] );
    var got = src.only( dst, src2, ( e ) => e, ( ins ) => ins );
    var exp = [];
    test.true( got === dst );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - empty Set';
    var dst = new Set();
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.only( dst, new Set(), ( e ) => e, ( ins ) => ins );
    var exp = [];
    test.true( got !== dst );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - empty setAdapter';
    var dst = _.containerAdapter.make( new Set() );
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.only( dst, [], ( e ) => e, ( ins ) => ins );
    var exp = [];
    test.true( got === dst );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - src';
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.only( src, [], ( e ) => e, ( ins ) => ins );
    var exp = [];
    test.true( got === src );
    test.identical( [ ... got.original ], exp );

    test.close( 'src2 is empty' );

    /* */

    test.open( 'src2 == dst' );

    test.case = 'dst - empty array';
    var dst = [];
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var src2 = _.containerAdapter.make( new Set( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] ) );
    var got = src.only( dst, src2, ( e ) => e, ( ins ) => ins );
    var exp = [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ];
    test.true( got !== dst );
    test.true( got !== src );
    test.identical( got.original, exp );

    test.case = 'dst - empty arrayAdapter';
    var dst = _.containerAdapter.make( [] );
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var src2 = _.containerAdapter.make( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.only( dst, src2, ( e ) => e, ( ins ) => ins );
    var exp = [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ];
    test.true( got === dst );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - empty Set';
    var dst = new Set();
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var src2 = new Set( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.only( dst, src2, ( e ) => e, ( ins ) => ins );
    var exp = [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ];
    test.true( got !== dst );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - setAdapter';
    var dst = _.containerAdapter.make( new Set( [ -20, 5 ] ) );
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20, [ 22 ], [ 20 ] ] );
    var src2 = [ -10, 5, -15, 4, 1, 2, 15, [ 21 ], [ -20 ], [ 22 ], [ 20 ] ];
    var got = src.only( dst, src2, ( e ) => e, ( ins ) => ins[ 0 ] );
    var exp = [ -20, 5, [ 21 ], [ -20 ] ];
    test.true( got === dst );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - src';
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var src2 = [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ];
    var got = src.only( src, src2, ( e ) => e, ( ins ) => ins );
    var exp = [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ];
    test.true( got === src );
    test.identical( [ ... got.original ], exp );

    test.close( 'src2 == dst' );

    /* - */

    test.open( 'src2 != dst' );

    test.case = 'dst - empty array';
    var dst = [];
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var src2 = _.containerAdapter.make( new Set( [ -10, 6, -14, 4, 1, 1, 15, 21, -1 ] ) );
    var got = src.only( dst, src2, ( e ) => e, ( ins ) => ins );
    var exp = [ -10, 4, 1, 15, 21 ];
    test.true( got !== dst );
    test.true( got !== src );
    test.identical( got.original, exp );

    test.case = 'dst - arrayAdapter';
    var dst = _.containerAdapter.make( [ 1 ] );
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var src2 = _.containerAdapter.make( [ -10, 6, -14, 4, 1, 1, 15, 21, -1 ] );
    var got = src.only( dst, src2, ( e ) => e, ( ins ) => ins );
    var exp = [ 1, -10, 4, 15, 21 ];
    test.true( got === dst );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - empty Set';
    var dst = new Set();
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20, 22 ] );
    var src2 = new Set( [ -10, 6, -14, 4, 1, 1, 15, 21, -1, [ 22 ] ] );
    var got = src.only( dst, src2, ( e ) => e, ( ins ) => ins[ 0 ] );
    var exp = [ [ 22 ] ];
    test.true( got !== dst );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - Set';
    var dst = new Set( [ [ 22 ] ] );
    var src = makeSrc( [ -15, 4, 1, 2, 15, 21, -20, [ 22 ] ] );
    var src2 = new Set( [ [ 22 ], -10, 6, -14, 4, 1, 1, 15, 21, -1 ] );
    var got = src.only( dst, src2, ( e ) => e[ 0 ], ( ins ) => ins[ 0 ] );
    var exp = [ [ 22 ], [ 22 ], -10, 6, -14, 4, 1, 15, 21, -1 ];
    test.true( got !== dst );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - empty setAdapter';
    var dst = _.containerAdapter.make( new Set() );
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var src2 = [ -10, 6, -14, 4, 1, 1, 15, 21, -1 ];
    var got = src.only( dst, src2, ( e ) => e, ( ins ) => ins );
    var exp = [ -10, 4, 1, 15, 21 ];
    test.true( got === dst );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - src';
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var src2 = [ -10, 6, -14, 4, 1, 1, 15, 21, -1 ];
    var got = src.only( src, src2, ( e ) => e, ( ins ) => ins );
    var exp = [ -10, 4, 1, 15, 21 ];
    test.true( got === src );
    test.identical( [ ... got.original ], exp );

    test.close( 'src2 != dst' );
  }
}

//

function onlyEqualizer( test )
{
  test.open( 'arrayContainerAdapter' );
  run( ( e ) => _.containerAdapter.make( e ) );
  test.close( 'arrayContainerAdapter' );

  /* - */

  test.open( 'setContainerAdapter' );
  run( ( e ) => _.containerAdapter.make( new Set( e ) ) );
  test.close( 'setContainerAdapter' );

  /* - */

  function run( makeSrc )
  {
    test.open( 'without dst' );

    test.case = 'dst - null';
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.only( null, ( e, ins ) => e === ins );
    var exp = [];
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - empty array';
    var src2 = [];
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.only( src2, ( e, ins ) => e === ins );
    var exp = [];
    test.true( got !== src2 );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - empty arrayAdapter';
    var src2 = _.containerAdapter.make( [] );
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.only( src2, ( e, ins ) => e === ins );
    var exp = [];
    test.true( got !== src2 );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - empty Set';
    var src2 = new Set();
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.only( src2, ( e, ins ) => e === ins );
    var exp = [];
    test.true( got !== src2 );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - empty setAdapter';
    var src2 = _.containerAdapter.make( new Set() );
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.only( src2, ( e, ins ) => e === ins );
    var exp = [];
    test.true( got !== src2 );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - src';
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.only( src, ( e, ins ) => e === ins );
    var exp = [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ];
    test.true( got === src );
    test.identical( [ ... got.original ], exp );

    test.close( 'without dst' );

    /* - */

    test.open( 'src2 is empty' );

    test.case = 'dst - null';
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.only( null, [], ( e, ins ) => e === ins );
    var exp = [];
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - empty array';
    var dst = [];
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.only( dst, new Set(), ( e, ins ) => e === ins );
    var exp = [];
    test.true( got !== dst );
    test.true( got !== src );
    test.identical( got.original, exp );

    test.case = 'dst - empty arrayAdapter';
    var dst = _.containerAdapter.make( [] );
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var src2 = _.containerAdapter.make( [] );
    var got = src.only( dst, src2, ( e, ins ) => e === ins );
    var exp = [];
    test.true( got === dst );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - empty Set';
    var dst = new Set();
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.only( dst, new Set(), ( e, ins ) => e === ins );
    var exp = [];
    test.true( got !== dst );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - empty setAdapter';
    var dst = _.containerAdapter.make( new Set() );
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.only( dst, [], ( e, ins ) => e === ins );
    var exp = [];
    test.true( got === dst );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - src';
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.only( src, [], ( e, ins ) => e === ins );
    var exp = [];
    test.true( got === src );
    test.identical( [ ... got.original ], exp );

    test.close( 'src2 is empty' );

    /* */

    test.open( 'src2 == dst' );

    test.case = 'dst - empty array';
    var dst = [];
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var src2 = _.containerAdapter.make( new Set( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] ) );
    var got = src.only( dst, src2, ( e, ins ) => e === ins );
    var exp = [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ];
    test.true( got !== dst );
    test.true( got !== src );
    test.identical( got.original, exp );

    test.case = 'dst - empty arrayAdapter';
    var dst = _.containerAdapter.make( [] );
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var src2 = _.containerAdapter.make( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.only( dst, src2, ( e, ins ) => e === ins );
    var exp = [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ];
    test.true( got === dst );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - empty Set';
    var dst = new Set();
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var src2 = new Set( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.only( dst, src2, ( e, ins ) => e === ins );
    var exp = [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ];
    test.true( got !== dst );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - setAdapter';
    var dst = _.containerAdapter.make( new Set( [ -20, 5 ] ) );
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20, [ 22 ], [ 20 ] ] );
    var src2 = [ -10, 5, -15, 4, 1, 2, 15, [ 21 ], [ -20 ], [ 22 ], [ 20 ] ];
    var got = src.only( dst, src2, ( e, ins ) => e === ins[ 0 ] );
    var exp = [ -20, 5, [ 21 ], [ -20 ] ];
    test.true( got === dst );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - src';
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var src2 = [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ];
    var got = src.only( src, src2, ( e, ins ) => e === ins );
    var exp = [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ];
    test.true( got === src );
    test.identical( [ ... got.original ], exp );

    test.close( 'src2 == dst' );

    /* - */

    test.open( 'src2 != dst' );

    test.case = 'dst - empty array';
    var dst = [];
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var src2 = _.containerAdapter.make( new Set( [ -10, 6, -14, 4, 1, 1, 15, 21, -1 ] ) );
    var got = src.only( dst, src2, ( e, ins ) => e === ins );
    var exp = [ -10, 4, 1, 15, 21 ];
    test.true( got !== dst );
    test.true( got !== src );
    test.identical( got.original, exp );

    test.case = 'dst - arrayAdapter';
    var dst = _.containerAdapter.make( [ 1 ] );
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var src2 = _.containerAdapter.make( [ -10, 6, -14, 4, 1, 1, 15, 21, -1 ] );
    var got = src.only( dst, src2, ( e, ins ) => e === ins );
    var exp = [ 1, -10, 4, 15, 21 ];
    test.true( got === dst );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - empty Set';
    var dst = new Set();
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20, 22 ] );
    var src2 = new Set( [ -10, 6, -14, 4, 1, 1, 15, 21, -1, [ 22 ] ] );
    var got = src.only( dst, src2, ( e, ins ) => e === ins[ 0 ] );
    var exp = [ [ 22 ] ];
    test.true( got !== dst );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - Set';
    var dst = new Set( [ [ 22 ] ] );
    var src = makeSrc( [ -15, 4, 1, 2, 15, 21, -20, [ 22 ] ] );
    var src2 = new Set( [ [ 22 ], -10, 6, -14, 4, 1, 1, 15, 21, -1 ] );
    var got = src.only( dst, src2, ( e, ins ) => e[ 0 ] === ins[ 0 ] );
    var exp = [ [ 22 ], [ 22 ], -10, 6, -14, 4, 1, 15, 21, -1 ];
    test.true( got !== dst );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - empty setAdapter';
    var dst = _.containerAdapter.make( new Set() );
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var src2 = [ -10, 6, -14, 4, 1, 1, 15, 21, -1 ];
    var got = src.only( dst, src2, ( e, ins ) => e === ins );
    var exp = [ -10, 4, 1, 15, 21 ];
    test.true( got === dst );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - src';
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var src2 = [ -10, 6, -14, 4, 1, 1, 15, 21, -1 ];
    var got = src.only( src, src2, ( e, ins ) => e === ins );
    var exp = [ -10, 4, 1, 15, 21 ];
    test.true( got === src );
    test.identical( [ ... got.original ], exp );

    test.close( 'src2 != dst' );
  }
}

//

function onlyBugWithReturnedContainer( test )
{
  test.case = 'dst is array';
  var dst = _.self;
  var src = _.containerAdapter.make( [ 2, 3, 4 ] );
  var src2 = [ 2, 3, 5 ];
  var got = src.only( dst, src2 );
  var exp = [ 2, 3 ];
  test.true( got !== dst );
  test.true( got !== src2 );
  test.identical( got.original, exp );

  test.case = 'dst is Set';
  var dst = _.self;
  var src = _.containerAdapter.make( [ 2, 3, 4 ] );
  var src2 = [ 2, 3, 5 ];
  var got = src.only( dst, src2 );
  var exp = [ 2, 3 ];
  test.true( got !== dst );
  test.true( got !== src2 );
  test.identical( [ ... got.original ], exp );
}
onlyBugWithReturnedContainer.description =
`
  Description :
    - method returns Symbol _.self if it was provided, but should not
`
//

function butWithoutCallbacks( test )
{
  test.open( 'arrayContainerAdapter' );
  run( ( e ) => _.containerAdapter.make( e ) );
  test.close( 'arrayContainerAdapter' );

  /* - */

  test.open( 'setContainerAdapter' );
  run( ( e ) => _.containerAdapter.make( new Set( e ) ) );
  test.close( 'setContainerAdapter' );

  /* - */

  function run( makeSrc )
  {
    test.open( 'without dst' );

    test.case = 'dst - null';
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.but( null );
    var exp = [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ];
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - empty array';
    var src2 = [];
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.but( src2 );
    var exp = [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ];
    test.true( got !== src2 );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - empty arrayAdapter';
    var src2 = _.containerAdapter.make( [] );
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.but( src2 );
    var exp = [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ];
    test.true( got !== src2 );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - empty Set';
    var src2 = new Set();
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.but( src2 );
    var exp = [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ];
    test.true( got !== src2 );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - empty setAdapter';
    var src2 = _.containerAdapter.make( new Set() );
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.but( src2 );
    var exp = [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ];
    test.true( got !== src2 );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - src';
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.but( src );
    var exp = [];
    test.true( got === src );
    test.identical( [ ... got.original ], exp );

    test.close( 'without dst' );

    /* - */

    test.open( 'src2 is empty' );

    test.case = 'dst - null';
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.but( null, [] );
    var exp = [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ];
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - empty array';
    var dst = [];
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.but( dst, new Set() );
    var exp = [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ];
    test.true( got !== dst );
    test.true( got !== src );
    test.identical( got.original, exp );

    test.case = 'dst - empty arrayAdapter';
    var dst = _.containerAdapter.make( [] );
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var src2 = _.containerAdapter.make( [] );
    var got = src.but( dst, src2 );
    var exp = [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ];
    test.true( got === dst );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - empty Set';
    var dst = new Set();
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.but( dst, new Set() );
    var exp = [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ];
    test.true( got !== dst );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - empty setAdapter';
    var dst = _.containerAdapter.make( new Set() );
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.but( dst, [] );
    var exp = [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ];
    test.true( got === dst );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - src';
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.but( src, [] );
    var exp = [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ];
    test.true( got === src );
    test.identical( [ ... got.original ], exp );

    test.close( 'src2 is empty' );

    /* */

    test.open( 'src2 == dst' );

    test.case = 'dst - empty array';
    var dst = [];
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var src2 = _.containerAdapter.make( new Set( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] ) );
    var got = src.but( dst, src2 );
    var exp = [];
    test.true( got !== dst );
    test.true( got !== src );
    test.identical( got.original, exp );

    test.case = 'dst - empty arrayAdapter';
    var dst = _.containerAdapter.make( [] );
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var src2 = _.containerAdapter.make( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.but( dst, src2 );
    var exp = [];
    test.true( got === dst );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - empty Set';
    var dst = new Set();
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var src2 = new Set( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.but( dst, src2 );
    var exp = [];
    test.true( got !== dst );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - empty setAdapter';
    var dst = _.containerAdapter.make( new Set() );
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var src2 = [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ];
    var got = src.but( dst, src2 );
    var exp = [];
    test.true( got === dst );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - src';
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var src2 = [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ];
    var got = src.but( src, src2 );
    var exp = [];
    test.true( got === src );
    test.identical( [ ... got.original ], exp );

    test.close( 'src2 == dst' );

    /* - */

    test.open( 'src2 != dst' );

    test.case = 'dst - empty array';
    var dst = [];
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var src2 = _.containerAdapter.make( new Set( [ -10, 6, -14, 4, 1, 1, 15, 21, -1 ] ) );
    var got = src.but( dst, src2 );
    var exp = [ 5, -15, 2, -20 ];
    test.true( got !== dst );
    test.true( got !== src );
    test.identical( got.original, exp );

    test.case = 'dst - arrayAdapter';
    var dst = _.containerAdapter.make( [ 1 ] );
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var src2 = _.containerAdapter.make( [ -10, 6, -14, 4, 1, 1, 15, 21, -1 ] );
    var got = src.but( dst, src2 );
    var exp = [ 1, 5, -15, 2, -20 ];
    test.true( got === dst );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - empty Set';
    var dst = new Set();
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var src2 = new Set( [ -10, 6, -14, 4, 1, 1, 15, 21, -1 ] );
    var got = src.but( dst, src2 );
    var exp = [ 5, -15, 2, -20 ];
    test.true( got !== dst );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - empty setAdapter';
    var dst = _.containerAdapter.make( new Set() );
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var src2 = [ -10, 6, -14, 4, 1, 1, 15, 21, -1 ];
    var got = src.but( dst, src2 );
    var exp = [ 5, -15, 2, -20 ];
    test.true( got === dst );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - src';
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var src2 = [ -10, 6, -14, 4, 1, 1, 15, 21, -1 ];
    var got = src.but( src, src2 );
    var exp = [ 5, -15, 2, -20 ];
    test.true( got === src );
    test.identical( [ ... got.original ], exp );

    test.close( 'src2 != dst' );
  }
}

//

function butOneEvaluator( test )
{
  test.open( 'arrayContainerAdapter' );
  run( ( e ) => _.containerAdapter.make( e ) );
  test.close( 'arrayContainerAdapter' );

  /* - */

  test.open( 'setContainerAdapter' );
  run( ( e ) => _.containerAdapter.make( new Set( e ) ) );
  test.close( 'setContainerAdapter' );

  /* - */

  function run( makeSrc )
  {
    test.open( 'without dst' );

    test.case = 'dst - null';
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.but( null, ( e ) => e );
    var exp = [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ];
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - empty array';
    var src2 = [];
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.but( src2, ( e ) => e );
    var exp = [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ];
    test.true( got !== src2 );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - empty arrayAdapter';
    var src2 = _.containerAdapter.make( [] );
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.but( src2, ( e ) => e );
    var exp = [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ];
    test.true( got !== dst );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - empty Set';
    var src2 = new Set();
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.but( src2, ( e ) => e );
    var exp = [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ];
    test.true( got !== src2 );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - empty setAdapter';
    var src2 = _.containerAdapter.make( new Set() );
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.but( src2, ( e ) => e );
    var exp = [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ];
    test.true( got !== src2 );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - src';
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.but( src, ( e ) => e );
    var exp = [];
    test.true( got === src );
    test.identical( [ ... got.original ], exp );

    test.close( 'without dst' );

    /* - */

    test.open( 'src2 is empty' );

    test.case = 'dst - null';
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.but( null, [], ( e ) => e );
    var exp = [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ];
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - empty array';
    var dst = [];
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.but( dst, new Set(), ( e ) => e );
    var exp = [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ];
    test.true( got !== dst );
    test.true( got !== src );
    test.identical( got.original, exp );

    test.case = 'dst - empty arrayAdapter';
    var dst = _.containerAdapter.make( [] );
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var src2 = _.containerAdapter.make( [] );
    var got = src.but( dst, src2, ( e ) => e );
    var exp = [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ];
    test.true( got === dst );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - empty Set';
    var dst = new Set();
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.but( dst, new Set(), ( e ) => e );
    var exp = [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ];
    test.true( got !== dst );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - empty setAdapter';
    var dst = _.containerAdapter.make( new Set() );
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.but( dst, [], ( e ) => e );
    var exp = [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ];
    test.true( got === dst );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - src';
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.but( src, [], ( e ) => e );
    var exp = [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ];
    test.true( got === src );
    test.identical( [ ... got.original ], exp );

    test.close( 'src2 is empty' );

    /* */

    test.open( 'src2 == dst' );

    test.case = 'dst - empty array';
    var dst = [];
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var src2 = _.containerAdapter.make( new Set( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] ) );
    var got = src.but( dst, src2, ( e ) => e );
    var exp = [];
    test.true( got !== dst );
    test.true( got !== src );
    test.identical( got.original, exp );

    test.case = 'dst - empty arrayAdapter';
    var dst = _.containerAdapter.make( [] );
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var src2 = _.containerAdapter.make( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.but( dst, src2, ( e ) => e );
    var exp = [];
    test.true( got === dst );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - empty Set';
    var dst = new Set();
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var src2 = new Set( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.but( dst, src2, ( e ) => e );
    var exp = [];
    test.true( got !== dst );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - setAdapter';
    var dst = _.containerAdapter.make( new Set( [ -20, 5 ] ) );
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20, [ 22 ], [ 20 ] ] );
    var src2 = [ -10, 5, -15, 4, 1, 2, 15, [ 21 ], [ -20 ], [ 22 ], [ 20 ] ];
    var got = src.but( dst, src2, ( e ) => e[ 0 ] );
    var exp = [ -20, 5 ];
    test.true( got === dst );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - src';
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var src2 = [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ];
    var got = src.but( src, src2, ( e ) => e );
    var exp = [];
    test.true( got === src );
    test.identical( [ ... got.original ], exp );

    test.close( 'src2 == dst' );

    /* - */

    test.open( 'src2 != dst' );

    test.case = 'dst - empty array';
    var dst = [];
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var src2 = _.containerAdapter.make( new Set( [ -10, 6, -14, 4, 1, 1, 15, 21, -1 ] ) );
    var got = src.but( dst, src2, ( e ) => e );
    var exp = [ 5, -15, 2, -20 ];
    test.true( got !== dst );
    test.true( got !== src );
    test.identical( got.original, exp );

    test.case = 'dst - arrayAdapter';
    var dst = _.containerAdapter.make( [ 1 ] );
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var src2 = _.containerAdapter.make( [ -10, 6, -14, 4, 1, 1, 15, 21, -1 ] );
    var got = src.but( dst, src2, ( e ) => e );
    var exp = [ 1, 5, -15, 2, -20 ];
    test.true( got === dst );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - empty Set';
    var dst = new Set();
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20, [ 22 ] ] );
    var src2 = new Set( [ -10, 6, -14, 4, 1, 1, 15, 21, -1, [ 22 ] ] );
    var got = src.but( dst, src2, ( e ) => e[ 0 ] );
    var exp = [];
    test.true( got !== dst );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - Set';
    var dst = new Set( [ [ 22 ] ] );
    var src = makeSrc( [ -15, 4, 1, 2, 15, 21, -20, [ 22 ] ] );
    var src2 = new Set( [ [ 22 ], -10, 6, -14, 4, 1, 1, 15, 21, -1 ] );
    var got = src.but( dst, src2, ( e ) => e[ 0 ] );
    var exp = [ [ 22 ] ];
    test.true( got !== dst );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - empty setAdapter';
    var dst = _.containerAdapter.make( new Set() );
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var src2 = [ -10, 6, -14, 4, 1, 1, 15, 21, -1 ];
    var got = src.but( dst, src2, ( e ) => e );
    var exp = [ 5, -15, 2, -20 ];
    test.true( got === dst );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - src';
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var src2 = [ -10, 6, -14, 4, 1, 1, 15, 21, -1 ];
    var got = src.but( src, src2, ( e ) => e );
    var exp = [ 5, -15, 2, -20 ];
    test.true( got === src );
    test.identical( [ ... got.original ], exp );

    test.close( 'src2 != dst' );
  }
}

//

function butTwoEvaluators( test )
{
  test.open( 'arrayContainerAdapter' );
  run( ( e ) => _.containerAdapter.make( e ) );
  test.close( 'arrayContainerAdapter' );

  /* - */

  test.open( 'setContainerAdapter' );
  run( ( e ) => _.containerAdapter.make( new Set( e ) ) );
  test.close( 'setContainerAdapter' );

  /* - */

  function run( makeSrc )
  {
    test.open( 'without dst' );

    test.case = 'dst - null';
    var src = makeSrc( [] );
    var got = src.but( null, ( e ) => e, ( ins ) => ins );
    var exp = [];
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - empty array';
    var src2 = [];
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.but( src2, ( e ) => e, ( ins ) => ins );
    var exp = [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ];
    test.true( got !== src2 );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - empty arrayAdapter';
    var src2 = _.containerAdapter.make( [] );
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.but( src2, ( e ) => e, ( ins ) => ins );
    var exp = [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ];
    test.true( got !== src2 );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - empty Set';
    var src2 = new Set();
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.but( src2, ( e ) => e, ( ins ) => ins );
    var exp = [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ];
    test.true( got !== src2 );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - empty setAdapter';
    var src2 = _.containerAdapter.make( new Set() );
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.but( src2, ( e ) => e, ( ins ) => ins );
    var exp = [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ];
    test.true( got !== src2 );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - src';
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.but( src, ( e ) => e, ( ins ) => ins );
    var exp = [];
    test.true( got === src );
    test.identical( [ ... got.original ], exp );

    test.close( 'without dst' );

    /* - */

    test.open( 'src2 is empty' );

    test.case = 'dst - null';
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.but( null, [], ( e ) => e, ( ins ) => ins );
    var exp = [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ];
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - empty array';
    var dst = [];
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.but( dst, new Set(), ( e ) => e, ( ins ) => ins );
    var exp = [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ];
    test.true( got !== dst );
    test.true( got !== src );
    test.identical( got.original, exp );

    test.case = 'dst - empty arrayAdapter';
    var dst = _.containerAdapter.make( [] );
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var src2 = _.containerAdapter.make( [] );
    var got = src.but( dst, src2, ( e ) => e, ( ins ) => ins );
    var exp = [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ];
    test.true( got === dst );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - empty Set';
    var dst = new Set();
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.but( dst, new Set(), ( e ) => e, ( ins ) => ins );
    var exp = [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ];
    test.true( got !== dst );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - empty setAdapter';
    var dst = _.containerAdapter.make( new Set() );
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.but( dst, [], ( e ) => e, ( ins ) => ins );
    var exp = [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ];
    test.true( got === dst );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - src';
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.but( src, [], ( e ) => e, ( ins ) => ins );
    var exp = [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ];
    test.true( got === src );
    test.identical( [ ... got.original ], exp );

    test.close( 'src2 is empty' );

    /* */

    test.open( 'src2 == dst' );

    test.case = 'dst - empty array';
    var dst = [];
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var src2 = _.containerAdapter.make( new Set( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] ) );
    var got = src.but( dst, src2, ( e ) => e, ( ins ) => ins );
    var exp = [];
    test.true( got !== dst );
    test.true( got !== src );
    test.identical( got.original, exp );

    test.case = 'dst - empty arrayAdapter';
    var dst = _.containerAdapter.make( [] );
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var src2 = _.containerAdapter.make( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.but( dst, src2, ( e ) => e, ( ins ) => ins );
    var exp = [];
    test.true( got === dst );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - empty Set';
    var dst = new Set();
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var src2 = new Set( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.but( dst, src2, ( e ) => e, ( ins ) => ins );
    var exp = [];
    test.true( got !== dst );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - setAdapter';
    var dst = _.containerAdapter.make( new Set( [ -20, 5 ] ) );
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20, [ 22 ], [ 20 ] ] );
    var src2 = [ -10, 5, -15, 4, 1, 2, 15, [ 21 ], [ -20 ], [ 22 ], [ 20 ] ];
    var got = src.but( dst, src2, ( e ) => e, ( ins ) => ins[ 0 ] );
    var exp = [ -20, 5, -10, -15, 4, 1, 2, 15, 21, [ 22 ], [ 20 ] ];
    test.true( got === dst );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - src';
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var src2 = [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ];
    var got = src.but( src, src2, ( e ) => e, ( ins ) => ins );
    var exp = [];
    test.true( got === src );
    test.identical( [ ... got.original ], exp );

    test.close( 'src2 == dst' );

    /* - */

    test.open( 'src2 != dst' );

    test.case = 'dst - empty array';
    var dst = [];
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var src2 = _.containerAdapter.make( new Set( [ -10, 6, -14, 4, 1, 1, 15, 21, -1 ] ) );
    var got = src.but( dst, src2, ( e ) => e, ( ins ) => ins );
    var exp = [ 5, -15, 2, -20 ];
    test.true( got !== dst );
    test.true( got !== src );
    test.identical( got.original, exp );

    test.case = 'dst - arrayAdapter';
    var dst = _.containerAdapter.make( [ 1 ] );
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var src2 = _.containerAdapter.make( [ -10, 6, -14, 4, 1, 1, 15, 21, -1 ] );
    var got = src.but( dst, src2, ( e ) => e, ( ins ) => ins );
    var exp = [ 1, 5, -15, 2, -20 ];
    test.true( got === dst );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - empty Set';
    var dst = new Set();
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20, 22 ] );
    var src2 = new Set( [ -10, 6, -14, 4, 1, 1, 15, 21, -1, [ 22 ] ] );
    var got = src.but( dst, src2, ( e ) => e, ( ins ) => ins[ 0 ] );
    var exp = [ -10, 5, -15, 4, 1, 2, 15, 21, -20, 22 ];
    test.true( got !== dst );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - Set';
    var dst = new Set( [ [ 22 ] ] );
    var src = makeSrc( [ -15, 4, 1, 2, 15, 21, -20, [ 22 ] ] );
    var src2 = new Set( [ [ 22 ], -10, 6, -14, 4, 1, 1, 15, 21, -1 ] );
    var got = src.but( dst, src2, ( e ) => e[ 0 ], ( ins ) => ins[ 0 ] );
    var exp = [ [ 22 ] ];
    test.true( got !== dst );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - empty setAdapter';
    var dst = _.containerAdapter.make( new Set() );
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var src2 = [ -10, 6, -14, 4, 1, 1, 15, 21, -1 ];
    var got = src.but( dst, src2, ( e ) => e, ( ins ) => ins );
    var exp = [ 5, -15, 2, -20 ];
    test.true( got === dst );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - src';
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var src2 = [ -10, 6, -14, 4, 1, 1, 15, 21, -1 ];
    var got = src.but( src, src2, ( e ) => e, ( ins ) => ins );
    var exp = [ 5, -15, 2, -20 ];
    test.true( got === src );
    test.identical( [ ... got.original ], exp );

    test.close( 'src2 != dst' );
  }
}

//

function butEqualizer( test )
{
  test.open( 'arrayContainerAdapter' );
  run( ( e ) => _.containerAdapter.make( e ) );
  test.close( 'arrayContainerAdapter' );

  /* - */

  test.open( 'setContainerAdapter' );
  run( ( e ) => _.containerAdapter.make( new Set( e ) ) );
  test.close( 'setContainerAdapter' );

  /* - */

  function run( makeSrc )
  {
    test.open( 'without dst' );

    test.case = 'dst - null';
    var src = makeSrc( [] );
    var got = src.but( null, ( e, ins ) => e === ins );
    var exp = [];
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - empty array';
    var src2 = [];
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.but( src2, ( e, ins ) => e === ins );
    var exp = [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ];
    test.true( got !== src2 );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - empty arrayAdapter';
    var src2 = _.containerAdapter.make( [] );
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.but( src2, ( e, ins ) => e === ins );
    var exp = [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ];
    test.true( got !== src2 );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - empty Set';
    var src2 = new Set();
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.but( src2, ( e, ins ) => e === ins );
    var exp = [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ];
    test.true( got !== src2 );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - empty setAdapter';
    var src2 = _.containerAdapter.make( new Set() );
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.but( src2, ( e, ins ) => e === ins );
    var exp = [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ];
    test.true( got !== src2 );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - src';
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.but( src, ( e, ins ) => e === ins );
    var exp = [];
    test.true( got === src );
    test.identical( [ ... got.original ], exp );

    test.close( 'without dst' );

    /* - */

    test.open( 'src2 is empty' );

    test.case = 'dst - null';
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.but( null, [], ( e, ins ) => e === ins );
    var exp =[ -10, 5, -15, 4, 1, 2, 15, 21, -20 ];
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - empty array';
    var dst = [];
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.but( dst, new Set(), ( e, ins ) => e === ins );
    var exp = [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ];
    test.true( got !== dst );
    test.true( got !== src );
    test.identical( got.original, exp );

    test.case = 'dst - empty arrayAdapter';
    var dst = _.containerAdapter.make( [] );
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var src2 = _.containerAdapter.make( [] );
    var got = src.but( dst, src2, ( e, ins ) => e === ins );
    var exp = [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ];
    test.true( got === dst );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - empty Set';
    var dst = new Set();
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.but( dst, new Set(), ( e, ins ) => e === ins );
    var exp = [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ];
    test.true( got !== dst );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - empty setAdapter';
    var dst = _.containerAdapter.make( new Set() );
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.but( dst, [], ( e, ins ) => e === ins );
    var exp = [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ];
    test.true( got === dst );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - src';
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.but( src, [], ( e, ins ) => e === ins );
    var exp = [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ];
    test.true( got === src );
    test.identical( [ ... got.original ], exp );

    test.close( 'src2 is empty' );

    /* */

    test.open( 'src2 == dst' );

    test.case = 'dst - empty array';
    var dst = [];
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var src2 = _.containerAdapter.make( new Set( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] ) );
    var got = src.but( dst, src2, ( e, ins ) => e === ins );
    var exp = [];
    test.true( got !== dst );
    test.true( got !== src );
    test.identical( got.original, exp );

    test.case = 'dst - empty arrayAdapter';
    var dst = _.containerAdapter.make( [] );
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var src2 = _.containerAdapter.make( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.but( dst, src2, ( e, ins ) => e === ins );
    var exp = [];
    test.true( got === dst );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - empty Set';
    var dst = new Set();
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var src2 = new Set( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var got = src.but( dst, src2, ( e, ins ) => e === ins );
    var exp = [];
    test.true( got !== dst );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - setAdapter';
    var dst = _.containerAdapter.make( new Set( [ -20, 5 ] ) );
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20, [ 22 ], [ 20 ] ] );
    var src2 = [ -10, 5, -15, 4, 1, 2, 15, [ 21 ], [ -20 ], [ 22 ], [ 20 ] ];
    var got = src.but( dst, src2, ( e, ins ) => e === ins[ 0 ] );
    var exp = [ -20, 5, -10, -15, 4, 1, 2, 15, 21, [ 22 ], [ 20 ] ];
    test.true( got === dst );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - src';
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var src2 = [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ];
    var got = src.but( src, src2, ( e, ins ) => e === ins );
    var exp = [];
    test.true( got === src );
    test.identical( [ ... got.original ], exp );

    test.close( 'src2 == dst' );

    /* - */

    test.open( 'src2 != dst' );

    test.case = 'dst - empty array';
    var dst = [];
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var src2 = _.containerAdapter.make( new Set( [ -10, 6, -14, 4, 1, 1, 15, 21, -1 ] ) );
    var got = src.but( dst, src2, ( e, ins ) => e === ins );
    var exp = [ 5, -15, 2, -20 ];
    test.true( got !== dst );
    test.true( got !== src );
    test.identical( got.original, exp );

    test.case = 'dst - arrayAdapter';
    var dst = _.containerAdapter.make( [ 1 ] );
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var src2 = _.containerAdapter.make( [ -10, 6, -14, 4, 1, 1, 15, 21, -1 ] );
    var got = src.but( dst, src2, ( e, ins ) => e === ins );
    var exp = [ 1, 5, -15, 2, -20 ];
    test.true( got === dst );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - empty Set';
    var dst = new Set();
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20, 22 ] );
    var src2 = new Set( [ -10, 6, -14, 4, 1, 1, 15, 21, -1, [ 22 ] ] );
    var got = src.but( dst, src2, ( e, ins ) => e === ins[ 0 ] );
    var exp = [ -10, 5, -15, 4, 1, 2, 15, 21, -20, 22 ];
    test.true( got !== dst );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - Set';
    var dst = new Set( [ [ 22 ] ] );
    var src = makeSrc( [ -15, 4, 1, 2, 15, 21, -20, [ 22 ] ] );
    var src2 = new Set( [ [ 22 ], -10, 6, -14, 4, 1, 1, 15, 21, -1 ] );
    var got = src.but( dst, src2, ( e ) => e[ 0 ], ( ins ) => ins[ 0 ] );
    var exp = [ [ 22 ] ];
    test.true( got !== dst );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - empty setAdapter';
    var dst = _.containerAdapter.make( new Set() );
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var src2 = [ -10, 6, -14, 4, 1, 1, 15, 21, -1 ];
    var got = src.but( dst, src2, ( e, ins ) => e === ins );
    var exp = [ 5, -15, 2, -20 ];
    test.true( got === dst );
    test.true( got !== src );
    test.identical( [ ... got.original ], exp );

    test.case = 'dst - src';
    var src = makeSrc( [ -10, 5, -15, 4, 1, 2, 15, 21, -20 ] );
    var src2 = [ -10, 6, -14, 4, 1, 1, 15, 21, -1 ];
    var got = src.but( src, src2, ( e ) => e, ( ins ) => ins );
    var exp = [ 5, -15, 2, -20 ];
    test.true( got === src );
    test.identical( [ ... got.original ], exp );

    test.close( 'src2 != dst' );
  }
}

//

function butBugWithReturnedContainer( test )
{
  test.case = 'dst is array';
  var dst = _.self;
  var src = _.containerAdapter.make( [ 2, 3, 4 ] );
  var src2 = [ 2, 3, 5 ];
  var got = src.but( dst, src2 );
  var exp = [ 4 ];
  test.true( got !== dst );
  test.true( got !== src2 );
  test.identical( got.original, exp );

  test.case = 'dst is Set';
  var dst = _.self;
  var src = _.containerAdapter.make( [ 2, 3, 4 ] );
  var src2 = [ 2, 3, 5 ];
  var got = src.but( dst, src2 );
  var exp = [ 4 ];
  test.true( got !== dst );
  test.true( got !== src2 );
  test.identical( [ ... got.original ], exp );
}
onlyBugWithReturnedContainer.description =
`
  Description :
    - method returns Symbol _.self if it was provided, but should not
`

//

function select( test )
{
  test.case = 'arrayContainerAdapter, empty container';
  var src = _.containerAdapter.make( [] );
  var got = src.select( 'a' );
  var exp = [];
  test.identical( got, exp );

  test.case = 'arrayContainerAdapter, no selections';
  var src = _.containerAdapter.make( [ 'abc', 'bac', 'cad' ] );
  var got = src.select( 'a' );
  var exp = [];
  test.identical( got, exp );

  test.case = 'arrayContainerAdapter, no selections';
  var src = _.containerAdapter.make( [ { b : 1, d : 0 }, { c : 'c' }, { k : [ 1 ] } ] );
  var got = src.select( 'a' );
  var exp = [];
  test.identical( got, exp );

  test.case = 'arrayContainerAdapter, a few selections';
  var src = _.containerAdapter.make( [ { c : 3, a : 1 }, { d : 2, a : [ 2 ] }, { b : { a : [] } } ] );
  var got = src.select( 'a' );
  var exp = [ { c : 3, a : 1 }, { a : [ 2 ], d : 2 } ];
  test.identical( got, exp );

  /* */

  test.case = 'arrayContainerAdapter, empty container';
  var src = _.containerAdapter.make( new Set() );
  var got = src.select( 'a' );
  var exp = [];
  test.identical( got, exp );

  test.case = 'arrayContainerAdapter, no selections';
  var src = _.containerAdapter.make( new Set( [ 'abc', 'bac', 'cad' ] ) );
  var got = src.select( 'a' );
  var exp = [];
  test.identical( got, exp );

  test.case = 'arrayContainerAdapter, no selections';
  var src = _.containerAdapter.make( new Set( [ { b : 1, d : 0 }, [ 'a' ], { c : 'c' }, { k : [ 1 ] } ] ) );
  var got = src.select( 'a' );
  var exp = [];
  test.identical( got, exp );

  test.case = 'arrayContainerAdapter, a few selections';
  var src = _.containerAdapter.make( new Set( [ { c : 3, a : 1 }, { d : 2, a : [ 2 ] }, { b : { a : [] } } ] ) );
  var got = src.select( 'a' );
  var exp = [ { c : 3, a : 1 }, { a : [ 2 ], d : 2 } ];
  test.identical( got, exp );
}

//--
// ContainerAdapterSet
//--

function setAdapterMake_( test )
{
  test.case = 'from empty Set';
  var src = _.containerAdapter.make( new Set( [] ) );
  var got = src.make();
  var exp = [];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from Set';
  var src = _.containerAdapter.make( new Set( [ 1, { a : 2 }, [ 0 ], undefined ] ) );
  var got = src.make();
  var exp = [ 1, { a : 2 }, [ 0 ], undefined ];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from setContainerAdapter';
  var src = _.containerAdapter.make( _.containerAdapter.make( new Set( [ 1, { a : 2 }, [ 0 ], undefined ] ) ) );
  var got = src.make();
  var exp = [ 1, { a : 2 }, [ 0 ], undefined ];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  if( !Config.debug )
  return;

  test.case = 'argument is provided';
  test.shouldThrowErrorSync( () =>
  {
    var src = _.containerAdapter.make( new Set() );
    src.make( [ 1 ] );
  });
}

//

function setAdapterMakeEmpty( test )
{
  test.case = 'from empty Set';
  var src = _.containerAdapter.make( new Set( [] ) );
  var got = src.MakeEmpty();
  var exp = [];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from Set';
  var src = _.containerAdapter.make( new Set( [ 1, { a : 2 }, [ 0 ], undefined ] ) );
  var got = src.MakeEmpty();
  var exp = [];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from setContainerAdapter';
  var src = _.containerAdapter.make( _.containerAdapter.make( new Set( [ 1, { a : 2 }, [ 0 ], undefined ] ) ) );
  var got = src.MakeEmpty();
  var exp = [];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  if( !Config.debug )
  return;

  test.case = 'argument is provided';
  test.shouldThrowErrorSync( () =>
  {
    var src = _.containerAdapter.MakeEmpty( new Set() );
    src.make( [ 1 ] );
  });
}

//

function setAdapterMake( test )
{
  test.case = 'without arguments';
  var src = _.containerAdapter.make( new Set() );
  var got = src.Make();
  test.true( got !== src );
  test.identical( [ ... got.original ], [] );

  test.case = 'from undefined';
  var src = _.containerAdapter.make( new Set() );
  var got = src.Make( undefined );
  test.true( got !== src );
  test.identical( [ ... got.original ], [] );

  test.case = 'from null';
  var src = _.containerAdapter.make( new Set() );
  var got = src.Make( null );
  test.true( got !== src );
  test.identical( [ ... got.original ], [] );

  test.case = 'from zero';
  var src = _.containerAdapter.make( new Set() );
  var got = src.Make( 0 );
  test.true( got !== src );
  test.identical( [ ... got.original ], [] );

  test.case = 'from number';
  var src = _.containerAdapter.make( new Set() );
  var got = src.Make( 100 );
  test.true( got !== src );
  test.identical( [ ... got.original ], [] );

  test.case = 'from array';
  var container = [ 1, 2, 3 ];
  var src = _.containerAdapter.make( new Set() );
  var got = src.Make( container );
  test.true( got !== container );
  test.true( got !== src );
  test.identical( [ ... got.original ], [ 1, 2, 3 ] );

  test.case = 'from argumentsArray';
  var container = _.argumentsArray.make( [ 1, 2, 3 ] );
  var src = _.containerAdapter.make( new Set() );
  var got = src.Make( container );
  test.true( got !== container );
  test.true( got !== src );
  test.identical( [ ... got.original ], [ 1, 2, 3 ] );

  test.case = 'from BufferTyped';
  var container = new F32x( [ 1, 2, 3 ] );
  var src = _.containerAdapter.make( new Set() );
  var got = src.Make( container );
  test.true( got !== container );
  test.true( got !== src );
  test.identical( [ ... got.original ], [ 1, 2, 3 ] );

  test.case = 'from arrayContainerAdapter';
  var container = _.containerAdapter.make( [ 1, 2, 3 ] );
  var src = _.containerAdapter.make( new Set() );
  var got = src.Make( container );
  test.true( got !== container );
  test.true( got !== src );
  test.identical( [ ... got.original ], [ 1, 2, 3 ] );

  test.case = 'from setContainerAdapter';
  var container = _.containerAdapter.make( new Set( [ 1, 2, 3 ] ) );
  var src = _.containerAdapter.make( new Set() );
  var got = src.Make( container );
  test.true( got !== container );
  test.true( got !== src );
  test.identical( [ ... got.original ], [ 1, 2, 3 ] );

  test.case = 'from self';
  var src = _.containerAdapter.make( new Set() );
  var got = src.Make( src );
  test.true( got !== container );
  test.true( got !== src );
  test.identical( [ ... got.original ], [] );

  if( !Config.debug )
  return;

  test.case = 'from string';
  var src = _.containerAdapter.make( new Set() );
  var got = src.Make( 'str' );
  test.identical( got, undefined );

  test.case = 'from BufferRaw';
  var src = _.containerAdapter.make( new Set() );
  var got = src.Make( new BufferRaw( 10 ) );
  test.identical( got, undefined );
}

//

function setAdapterHas( test )
{
  test.case = 'empty container';
  var src = _.containerAdapter.make( new Set() );
  var got = src.has( 1 );
  test.identical( got, false );

  test.case = 'container, no entries';
  var src = _.containerAdapter.make( new Set( [ 1, 2, 3, 4 ] ) );
  var got = src.has( 0 );
  test.identical( got, false );

  test.case = 'container, entry';
  var src = _.containerAdapter.make( new Set( [ 1, 2, 3, 4 ] ) );
  var got = src.has( 3 );
  test.identical( got, true );

  test.case = 'container with arrays, no entries';
  var src = _.containerAdapter.make( new Set( [ [ 1 ], [ 2 ], [ 1 ] ] ) );
  var got = src.has( [ 1 ] );
  test.identical( [ ... src.original ], [ [ 1 ], [ 2 ], [ 1 ] ] );
  test.identical( got, false );

  test.case = 'container with maps, no entries';
  var src = _.containerAdapter.make( new Set( [ { a : 1 }, { b : 1 }, { a : 1 } ] ) );
  var got = src.has( { a : 1 } );
  test.identical( [ ... src.original ], [ { a : 1 }, { b : 1 }, { a : 1 } ] );
  test.identical( got, false );

  /* */

  test.case = 'container with arrays, evaluator is null';
  var src = _.containerAdapter.make( new Set( [ [ 1 ], [ 2 ], [ 1 ] ] ) );
  var got = src.has( [ 1 ], null );
  test.identical( [ ... src.original ], [ [ 1 ], [ 2 ], [ 1 ] ] );
  test.identical( got, false );

  test.case = 'container with arrays, one evaluator';
  var src = _.containerAdapter.make( new Set( [ [ 1 ], [ 2 ], [ 1 ] ] ) );
  var got = src.has( [ 1 ], ( e ) => e[ 0 ] );
  test.identical( [ ... src.original ], [ [ 1 ], [ 2 ], [ 1 ] ] );
  test.identical( got, true );

  test.case = 'container with maps, one evaluator';
  var src = _.containerAdapter.make( new Set( [ { a : 1 }, { b : 1 }, { a : 1 } ] ) );
  var got = src.has( { a : 1 }, ( e ) => e.a );
  test.identical( [ ... src.original ], [ { a : 1 }, { b : 1 }, { a : 1 } ] );
  test.identical( got, true );

  test.case = 'container with arrays, two evaluators';
  var src = _.containerAdapter.make( new Set( [ [ 1 ], [ 2 ], [ 1 ] ] ) );
  var got = src.has( 1, ( e ) => e[ 0 ], ( ins ) => ins );
  test.identical( [ ... src.original ], [ [ 1 ], [ 2 ], [ 1 ] ] );
  test.identical( got, true );

  test.case = 'container with maps, two evaluators';
  var src = _.containerAdapter.make( new Set( [ { a : 1 }, { b : 1 }, { a : 1 } ] ) );
  var got = src.has( 1, ( e ) => e.a, ( ins ) => ins );
  test.identical( [ ... src.original ], [ { a : 1 }, { b : 1 }, { a : 1 } ] );
  test.identical( got, true );

  test.case = 'container with arrays, fromIndex and evaluator2';
  var src = _.containerAdapter.make( new Set( [ [ 1 ], [ 2 ], [ 1 ] ] ) );
  var got = src.has( 1, 3, ( e ) => e[ 0 ] )
  test.identical( [ ... src.original ], [ [ 1 ], [ 2 ], [ 1 ] ] );
  test.identical( got, false );

  test.case = 'container with maps, fromIndex and evaluator2';
  var src = _.containerAdapter.make( new Set( [ { a : 1 }, { b : 1 }, { a : 1 } ] ) );
  var got = src.has( 1, 3, ( e ) => e.a );
  test.identical( [ ... src.original ], [ { a : 1 }, { b : 1 }, { a : 1 } ] );
  test.identical( got, false );

  test.case = 'container with arrays, equalizer';
  var src = _.containerAdapter.make( new Set( [ [ 1 ], [ 2 ], [ 1 ] ] ) );
  var got = src.has( 1, ( e, ins ) => e[ 0 ] === ins );
  test.identical( [ ... src.original ], [ [ 1 ], [ 2 ], [ 1 ] ] );
  test.identical( got, true );

  test.case = 'container with maps, equalizer';
  var src = _.containerAdapter.make( new Set( [ { a : 1 }, { b : 1 }, { a : 1 } ] ) );
  var got = src.has( 1, ( e, ins ) => e.a === ins );
  test.identical( [ ... src.original ], [ { a : 1 }, { b : 1 }, { a : 1 } ] );
  test.identical( got, true );
}

//

function setAdapterHasBugWithNullInEvaluator( test )
{
  test.case = 'null in onEvaluate1';
  var src = _.containerAdapter.make( [ 1, 2, 3, 4 ] );
  var got = src.has( 2, null );
  test.identical( got, true );
}
setAdapterHasBugWithNullInEvaluator.description =
`
 Description :
   - method has of setContainerAdapter was unable to accepts onEvaluate1 with null value.
`

//

function setAdapterCount( test )
{
  test.case = 'empty container';
  var src = _.containerAdapter.make( new Set() );
  var got = src.count();
  test.identical( got, 0 );

  var src = _.containerAdapter.make( new Set() );
  var got = src.count( 1 );
  test.identical( got, 0 );

  /* */

  test.case = 'container with primitive, no entries';
  var src = _.containerAdapter.make( new Set( [ 1, 2, 3, 4, 5 ] ) );
  var got = src.count( 6 );
  test.identical( got, 0 );

  test.case = 'container with primitive, has entry';
  var src = _.containerAdapter.make( new Set( [ 1, 2, 3, 4, 5 ] ) );
  var got = src.count( 3 );
  test.identical( got, 1 );

  test.case = 'container with maps, has entries, not count';
  var src = _.containerAdapter.make( new Set( [ { a : 1 }, { a : 1 }, { a : 1 }, { a : 1 }, { a : 1 } ] ) );
  var got = src.count( { a : 1 } );
  var exp = [ { a : 1 }, { a : 1 }, { a : 1 }, { a : 1 }, { a : 1 } ];
  test.identical( [ ... src.original ], exp );
  test.identical( got, 0 );

  test.case = 'container with arrays, has entries, not count';
  var src = _.containerAdapter.make( new Set( [ [ 1 ], [ 1 ], [ 1 ], [ 1 ], [ 1 ] ] ) );
  var got = src.count( [ 1 ] );
  var exp = [ [ 1 ], [ 1 ], [ 1 ], [ 1 ], [ 1 ] ];
  test.identical( [ ... src.original ], exp );
  test.identical( got, 0 );

  /* */

  test.case = 'container with primitive, no entries, one evaluator';
  var src = _.containerAdapter.make( new Set( [ 1, 2, 3, 4, 5 ] ) );
  var got = src.count( 6, ( e ) => 1 );
  test.identical( got, 5 );

  test.case = 'container with primitive, has entry, one evaluator';
  var src = _.containerAdapter.make( new Set( [ 1, 2, 3, 4, 5 ] ) );
  var got = src.count( 3, ( e ) => e );
  test.identical( got, 1 );

  test.case = 'container with maps, has entries, one evaluator';
  var src = _.containerAdapter.make( new Set( [ { a : 1 }, { a : 1 }, { a : 1 }, { a : 1 }, { a : 1 } ] ) );
  var got = src.count( { a : 1 }, ( e ) => e.a );
  var exp = [ { a : 1 }, { a : 1 }, { a : 1 }, { a : 1 }, { a : 1 } ];
  test.identical( [ ... src.original ], exp );
  test.identical( got, 5 );

  test.case = 'container with arrays, has entries, one evaluator';
  var src = _.containerAdapter.make( new Set( [ [ 1 ], [ 1 ], [ 1 ], [ 1 ], [ 1 ] ] ) );
  var got = src.count( [ 1 ], ( e ) => e[ 0 ] );
  var exp = [ [ 1 ], [ 1 ], [ 1 ], [ 1 ], [ 1 ] ];
  test.identical( [ ... src.original ], exp );
  test.identical( got, 5 );

  /* */

  test.case = 'container with primitive, no entries, two evaluators';
  var src = _.containerAdapter.make( new Set( [ 1, 2, 3, 4, 5 ] ) );
  var got = src.count( 6, ( e ) => e, ( ins ) => ins - 1 );
  test.identical( got, 1 );

  test.case = 'container with primitive, has entry, two evaluators';
  var src = _.containerAdapter.make( new Set( [ 1, 2, 3, 4, 5 ] ) );
  var got = src.count( 3, ( e ) => e, ( ins ) => ins + 3 );
  test.identical( got, 0 );

  test.case = 'container with maps, has entries, two evaluators';
  var src = _.containerAdapter.make( new Set( [ { a : 1 }, { a : 1 }, { a : 1 }, { a : 1 }, { a : 1 } ] ) );
  var got = src.count( { a : 1 }, ( e ) => e.a, ( ins ) => ins.a );
  var exp = [ { a : 1 }, { a : 1 }, { a : 1 }, { a : 1 }, { a : 1 } ];
  test.identical( [ ... src.original ], exp );
  test.identical( got, 5 );

  test.case = 'container with arrays, has entries, two evaluators';
  var src = _.containerAdapter.make( new Set( [ [ 1 ], [ 1 ], [ 1 ], [ 1 ], [ 1 ] ] ) );
  var got = src.count( [ 1 ], ( e ) => e[ 0 ], ( ins ) => ins[ 0 ] );
  var exp = [ [ 1 ], [ 1 ], [ 1 ], [ 1 ], [ 1 ] ];
  test.identical( [ ... src.original ], exp );
  test.identical( got, 5 );

  /* */

  test.case = 'container with primitive, no entries, equalizer';
  var src = _.containerAdapter.make( new Set( [ 1, 2, 3, 4, 5 ] ) );
  var got = src.count( 6, ( e, ins ) => e === ins - 1 );
  test.identical( got, 1 );

  test.case = 'container with primitive, has entry, equalizer';
  var src = _.containerAdapter.make( new Set( [ 1, 2, 3, 4, 5 ] ) );
  var got = src.count( 3, ( e, ins ) => e === ins + 3 );
  test.identical( got, 0 );

  test.case = 'container with maps, has entries, equalizer';
  var src = _.containerAdapter.make( new Set( [ { a : 1 }, { a : 1 }, { a : 1 }, { a : 1 }, { a : 1 } ] ) );
  var got = src.count( { a : 1 }, ( e, ins ) => e.a === ins.a );
  var exp = [ { a : 1 }, { a : 1 }, { a : 1 }, { a : 1 }, { a : 1 } ];
  test.identical( [ ... src.original ], exp );
  test.identical( got, 5 );

  test.case = 'container with arrays, has entries, equalizer';
  var src = _.containerAdapter.make( new Set( [ [ 1 ], [ 1 ], [ 1 ], [ 1 ], [ 1 ] ] ) );
  var got = src.count( [ 1 ], ( e, ins ) => e[ 0 ] === ins[ 0 ] );
  var exp = [ [ 1 ], [ 1 ], [ 1 ], [ 1 ], [ 1 ] ];
  test.identical( [ ... src.original ], exp );
  test.identical( got, 5 );
}

//

function setAdapterCopyFrom( test )
{
  test.case = 'empty container, src - empty array';
  var container = _.containerAdapter.make( new Set() );
  var src = [];
  var got = container.copyFrom( src );
  test.true( got === container );
  test.true( got !== src );
  test.identical( [ ... got.original ], [] );
  test.identical( src, [] );

  test.case = 'empty container, src - not empty array';
  var container = _.containerAdapter.make( new Set() );
  var src = [ undefined, null, [ 1 ], 2, 'str' ];
  var got = container.copyFrom( src );
  test.true( got === container );
  test.true( got !== src );
  test.identical( [ ... got.original ], [ undefined, null, [ 1 ], 2, 'str' ] );
  test.identical( src, [ undefined, null, [ 1 ], 2, 'str' ] );

  test.case = 'not empty container, src - empty array';
  var container = _.containerAdapter.make( new Set( [ true, false ] ) );
  var src = [];
  var got = container.copyFrom( src );
  test.true( got === container );
  test.true( got !== src );
  test.identical( [ ... got.original ], [ true, false ] );
  test.identical( src, [] );

  test.case = 'container.length < src.length';
  var container = _.containerAdapter.make( new Set( [ true, false ] ) );
  var src = [ undefined, null, [ 1 ], 2, 'str' ];
  var got = container.copyFrom( src );
  test.true( got === container );
  test.true( got !== src );
  test.identical( [ ... got.original ], [ undefined, null, [ 1 ], 2, 'str' ] );
  test.identical( src, [ undefined, null, [ 1 ], 2, 'str' ] );

  test.case = 'container.length === src.length';
  var container = _.containerAdapter.make( new Set( [ true, false ] ) );
  var src = [ undefined, null ];
  var got = container.copyFrom( src );
  test.true( got === container );
  test.true( got !== src );
  test.identical( [ ... got.original ], [ undefined, null ] );
  test.identical( src, [ undefined, null ] );

  test.case = 'container.length > src.length';
  var container = _.containerAdapter.make( new Set( [ true, false, 2, 'str' ] ) );
  var src = [ undefined, null, [ 1 ] ];
  var got = container.copyFrom( src );
  test.true( got === container );
  test.true( got !== src );
  test.identical( [ ... got.original ], [ undefined, null, [ 1 ], 'str' ] );
  test.identical( src, [ undefined, null, [ 1 ] ] );

  /* */

  test.case = 'empty container, src - empty Set';
  var container = _.containerAdapter.make( new Set() );
  var src = new Set();
  var got = container.copyFrom( src );
  test.true( got === container );
  test.true( got !== src );
  test.identical( [ ... got.original ], [] );
  test.identical( [ ... src ], [] );

  test.case = 'empty container, src - not empty Set';
  var container = _.containerAdapter.make( new Set() );
  var src = new Set( [ undefined, null, [ 1 ], 2, 'str' ] );
  var got = container.copyFrom( src );
  test.true( got === container );
  test.true( got !== src );
  test.identical( [ ... got.original ], [ undefined, null, [ 1 ], 2, 'str' ] );
  test.identical( [ ... src ], [ undefined, null, [ 1 ], 2, 'str' ] );

  test.case = 'not empty container, src - empty Set';
  var container = _.containerAdapter.make( new Set( [ true, false ] ) );
  var src = new Set();
  var got = container.copyFrom( src );
  test.true( got === container );
  test.true( got !== src );
  test.identical( [ ... got.original ], [ true, false ] );
  test.identical( [ ... src ], [] );

  test.case = 'container.length < src.length';
  var container = _.containerAdapter.make( new Set( [ true, false ] ) );
  var src = new Set( [ undefined, null, [ 1 ], 2, 'str' ] );
  var got = container.copyFrom( src );
  test.true( got === container );
  test.true( got !== src );
  test.identical( [ ... got.original ], [ undefined, null, [ 1 ], 2, 'str' ] );
  test.identical( [ ... src ], [ undefined, null, [ 1 ], 2, 'str' ] );

  test.case = 'container.length === src.length';
  var container = _.containerAdapter.make( new Set( [ true, false ] ) );
  var src = new Set( [ [ 1 ], 2 ] );
  var got = container.copyFrom( src );
  test.true( got === container );
  test.true( got !== src );
  test.identical( [ ... got.original ], [ [ 1 ], 2 ] );
  test.identical( [ ... src ], [ [ 1 ], 2 ] );

  test.case = 'container.length > src.length';
  var container = _.containerAdapter.make( new Set( [ true, false, 2, 'str' ] ) );
  var src = new Set( [ undefined, null, [ 1 ] ] );
  var got = container.copyFrom( src );
  test.true( got === container );
  test.true( got !== src );
  test.identical( [ ... got.original ], [ undefined, null, [ 1 ], 'str' ] );
  test.identical( [ ... src ], [ undefined, null, [ 1 ] ] );

  /* */

  test.case = 'empty container, src - empty arrayContainerAdapter';
  var container = _.containerAdapter.make( new Set() );
  var src = _.containerAdapter.make( [] );
  var got = container.copyFrom( src );
  test.true( got === container );
  test.true( got !== src );
  test.identical( [ ... got.original ], [] );
  test.identical( src.original, [] );

  test.case = 'empty container, src - not empty arrayContainerAdapter';
  var container = _.containerAdapter.make( new Set() );
  var src = _.containerAdapter.make( [ undefined, null, [ 1 ], 2, 'str' ] );
  var got = container.copyFrom( src );
  test.true( got === container );
  test.true( got !== src );
  test.identical( [ ... got.original ], [ undefined, null, [ 1 ], 2, 'str' ] );
  test.identical( src.original, [ undefined, null, [ 1 ], 2, 'str' ] );

  test.case = 'not empty container, src - empty arrayContainerAdapter';
  var container = _.containerAdapter.make( new Set( [ true, false ] ) );
  var src = _.containerAdapter.make( [] );
  var got = container.copyFrom( src );
  test.true( got === container );
  test.true( got !== src );
  test.identical([ ... got.original ], [ true, false ] );
  test.identical( src.original, [] );

  test.case = 'container.length < src.length';
  var container = _.containerAdapter.make( new Set( [ true, false ] ) );
  var src = _.containerAdapter.make( [ undefined, null, [ 1 ], 2, 'str' ] );
  var got = container.copyFrom( src );
  test.true( got === container );
  test.true( got !== src );
  test.identical( [ ... got.original ], [ undefined, null, [ 1 ], 2, 'str' ] );
  test.identical( src.original, [ undefined, null, [ 1 ], 2, 'str' ] );

  test.case = 'container.length === src.length';
  var container = _.containerAdapter.make( new Set( [ true, false ] ) );
  var src = _.containerAdapter.make( [ 2, 'str' ] );
  var got = container.copyFrom( src );
  test.true( got === container );
  test.true( got !== src );
  test.identical( [ ... got.original ], [ 2, 'str' ] );
  test.identical( src.original, [ 2, 'str' ] );

  test.case = 'container.length > src.length';
  var container = _.containerAdapter.make( new Set( [ true, false, 2, 'str' ] ) );
  var src = _.containerAdapter.make( [ undefined, null, [ 1 ] ] );
  var got = container.copyFrom( src );
  test.true( got === container );
  test.true( got !== src );
  test.identical( [ ... got.original ], [ undefined, null, [ 1 ], 'str' ] );
  test.identical( src.original, [ undefined, null, [ 1 ] ] );

  /* */

  test.case = 'empty container, src - empty setContainerAdapter';
  var container = _.containerAdapter.make( new Set() );
  var src = _.containerAdapter.make( new Set() );
  var got = container.copyFrom( src );
  test.true( got === container );
  test.true( got !== src );
  test.identical( [ ... got.original ], [] );
  test.identical( [ ... src ], [] );

  test.case = 'empty container, src - not empty Set';
  var container = _.containerAdapter.make( new Set() );
  var src = _.containerAdapter.make( new Set( [ undefined, null, [ 1 ], 2, 'str' ] ) );
  var got = container.copyFrom( src );
  test.true( got === container );
  test.true( got !== src );
  test.identical( [ ... got.original ], [ undefined, null, [ 1 ], 2, 'str' ] );
  test.identical( [ ... src ], [ undefined, null, [ 1 ], 2, 'str' ] );

  test.case = 'not empty container, src - empty Set';
  var container = _.containerAdapter.make( new Set( [ true, false ] ) );
  var src = _.containerAdapter.make( new Set() );
  var got = container.copyFrom( src );
  test.true( got === container );
  test.true( got !== src );
  test.identical( [ ... got.original ], [ true, false ] );
  test.identical( [ ... src ], [] );

  test.case = 'container.length < src.length';
  var container = _.containerAdapter.make( new Set( [ true, false ] ) );
  var src = _.containerAdapter.make( new Set( [ undefined, null, [ 1 ], 2, 'str' ] ) );
  var got = container.copyFrom( src );
  test.true( got === container );
  test.true( got !== src );
  test.identical( [ ... got.original ], [ undefined, null, [ 1 ], 2, 'str' ] );
  test.identical( [ ... src ], [ undefined, null, [ 1 ], 2, 'str' ] );

  test.case = 'container.length === src.length';
  var container = _.containerAdapter.make( new Set( [ true, false ] ) );
  var src = _.containerAdapter.make( new Set( [ 2, 'str' ] ) );
  var got = container.copyFrom( src );
  test.true( got === container );
  test.true( got !== src );
  test.identical( [ ... got.original ], [ 2, 'str' ] );
  test.identical( [ ... src ], [ 2, 'str' ] );

  test.case = 'container.length > src.length';
  var container = _.containerAdapter.make( new Set( [ true, false, 2, 'str' ] ) );
  var src = _.containerAdapter.make( new Set( [ undefined, null, [ 1 ] ] ) );
  var got = container.copyFrom( src );
  test.true( got === container );
  test.true( got !== src );
  test.identical( [ ... got.original ], [ undefined, null, [ 1 ], 'str' ] );
  test.identical( [ ... src ], [ undefined, null, [ 1 ] ] );

  /* */

  test.case = 'empty container';
  var container = _.containerAdapter.make( new Set() );
  var got = container.copyFrom( container );
  test.true( got === container );
  test.identical( [ ... got.original ], [] );

  test.case = 'container from src';
  var src = new Set( [ 1, 2 ] );
  var container = _.containerAdapter.make( src );
  var got = container.copyFrom( src );
  test.true( got === container );
  test.identical( [ ... got.original ], [ 1, 2 ] );

  test.case = 'not empty container';
  var container = _.containerAdapter.make( new Set( [ undefined, null, [ 1 ], 2, 'str' ] ) );
  var got = container.copyFrom( container );
  test.true( got === container );
  test.identical( [ ... got.original ], [ undefined, null, [ 1 ], 2, 'str' ] );

  test.case = 'not empty containers from src';
  var src = new Set( [ 1, 2 ] );
  var src2 = _.containerAdapter.make( src );
  var container = _.containerAdapter.make( src );
  var got = container.copyFrom( src2 );
  test.true( got === container );
  test.identical( [ ... got.original ], [ 1, 2 ] );
}

//

function setAdapterAppend( test )
{
  test.case = 'empty container, append primitive';
  var dst = _.containerAdapter.make( new Set() );
  var got = dst.append( 1 );
  var exp = [ 1 ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'empty container, append Long';
  var dst = _.containerAdapter.make( new Set() );
  var got = dst.append( _.unroll.make( [ 1, 2 ] ) );
  var exp = [ [ 1, 2 ] ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'empty container, append map';
  var dst = _.containerAdapter.make( new Set() );
  var got = dst.append( { a : 0 } );
  var exp = [ { a : 0 } ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  /* */

  test.case = 'container, append primitive';
  var dst = _.containerAdapter.make( new Set( [ undefined ] ) );
  var got = dst.append( 1 );
  var exp = [ undefined, 1 ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'container, append Long';
  var dst = _.containerAdapter.make( new Set( [ null ] ) );
  var got = dst.append( _.unroll.make( [ 1, 2 ] ) );
  var exp = [ null, [ 1, 2 ] ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'container, append map';
  var dst = _.containerAdapter.make( new Set( [ 0 ] ) );
  var got = dst.append( { a : 0 } );
  var exp = [ 0, { a : 0 } ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  /* */

  test.case = 'container, append primitive, duplicates';
  var dst = _.containerAdapter.make( new Set( [ 1, 2, 3 ] ) );
  var got = dst.append( 1 ).append( 2 );
  var exp = [ 1, 2, 3 ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'container, append Long, duplicates';
  var dst = _.containerAdapter.make( new Set( [ [ 1, 2 ] ] ) );
  var got = dst.append( [ 1, 2 ] ).append( [ 1, 2 ] );
  var exp = [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ] ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'container, append map, duplicates';
  var dst = _.containerAdapter.make( new Set( [ { a : 0 } ] ) );
  var got = dst.append( { a : 0 } ).append( { a : 0 } );
  var exp = [ { a : 0 }, { a : 0 }, { a : 0 } ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );
}

//

function setAdapterAppendOnceWithoutCallbacks( test )
{
  test.case = 'empty container, append primitive';
  var dst = _.containerAdapter.make( new Set() );
  var got = dst.appendOnce( 1 );
  var exp = [ 1 ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'empty container, append Long';
  var dst = _.containerAdapter.make( new Set() );
  var got = dst.appendOnce( _.unroll.make( [ 1, 2 ] ) );
  var exp = [ [ 1, 2 ] ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'empty container, append map';
  var dst = _.containerAdapter.make( new Set() );
  var got = dst.appendOnce( { a : 0 } );
  var exp = [ { a : 0 } ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  /* */

  test.case = 'container, append primitive';
  var dst = _.containerAdapter.make( new Set( [ undefined ] ) );
  var got = dst.appendOnce( 1 );
  var exp = [ undefined, 1 ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'container, append Long';
  var dst = _.containerAdapter.make( new Set( [ null ] ) );
  var got = dst.appendOnce( _.unroll.make( [ 1, 2 ] ) );
  var exp = [ null, [ 1, 2 ] ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'container, append map';
  var dst = _.containerAdapter.make( new Set( [ 0 ] ) );
  var got = dst.appendOnce( { a : 0 } );
  var exp = [ 0, { a : 0 } ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  /* */

  test.case = 'container, append primitive, duplicates';
  var dst = _.containerAdapter.make( new Set( [ 1, 2, 3 ] ) );
  var got = dst.appendOnce( 1 ).appendOnce( 2 );
  var exp = [ 1, 2, 3 ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'container, append Long, duplicates';
  var dst = _.containerAdapter.make( new Set( [ [ 1, 2 ] ] ) );
  var got = dst.appendOnce( [ 1, 2 ] ).appendOnce( [ 1, 2 ] );
  var exp = [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ] ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'container, append map, duplicates';
  var dst = _.containerAdapter.make( new Set( [ { a : 0 } ] ) );
  var got = dst.appendOnce( { a : 0 } ).appendOnce( { a : 0 } );
  var exp = [ { a : 0 }, { a : 0 }, { a : 0 } ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );
}

//

function setAdapterAppendOnceOneEvaluator( test )
{
  test.case = 'empty container, append primitive';
  var dst = _.containerAdapter.make( new Set() );
  var got = dst.appendOnce( 1, ( e ) => e );
  var exp = [ 1 ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'empty container, append Long';
  var dst = _.containerAdapter.make( new Set() );
  var got = dst.appendOnce( _.unroll.make( [ 1, 2 ] ), ( e ) => e );
  var exp = [ [ 1, 2 ] ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'empty container, append map';
  var dst = _.containerAdapter.make( new Set() );
  var got = dst.appendOnce( { a : 0 }, ( e ) => e );
  var exp = [ { a : 0 } ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  /* */

  test.case = 'container, append primitive';
  var dst = _.containerAdapter.make( new Set( [ undefined ] ) );
  var got = dst.appendOnce( 1, ( e ) => e );
  var exp = [ undefined, 1 ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'container, append Long';
  var dst = _.containerAdapter.make( new Set( [ null ] ) );
  var got = dst.appendOnce( _.unroll.make( [ 1, 2 ] ), ( e ) => e );
  var exp = [ null, [ 1, 2 ] ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'container, append map';
  var dst = _.containerAdapter.make( new Set( [ 0 ] ) );
  var got = dst.appendOnce( { a : 0 }, ( e ) => e );
  var exp = [ 0, { a : 0 } ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  /* */

  test.case = 'container, append primitive, duplicates';
  var dst = _.containerAdapter.make( new Set( [ 1, 2, 3 ] ) );
  var got = dst.appendOnce( 4, ( e ) => e === 4 ? 1 : e );
  var exp = [ 1, 2, 3 ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'container, append Long, duplicates';
  var dst = _.containerAdapter.make( new Set( [ [ 1, 2 ], [ 1, 3 ] ] ) );
  var got = dst.appendOnce( [ 1, 2 ], ( e ) => e[ 0 ] );
  var exp = [ [ 1, 2 ], [ 1, 3 ] ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'container, append Long, append duplicates';
  var dst = _.containerAdapter.make( new Set( [ [ 1, 3 ], [ 1, 3 ] ] ) );
  var got = dst.appendOnce( [ 1, 2 ], ( e ) => e[ 1 ] );
  var exp = [ [ 1, 3 ], [ 1, 3 ], [ 1, 2 ] ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'container, append map, duplicates';
  var dst = _.containerAdapter.make( new Set( [ { a : 0 }, { a : 1 } ] ) );
  var got = dst.appendOnce( { a : 0 }, ( e ) => e.a );
  var exp = [ { a : 0 }, { a : 1 } ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'container, append map, append duplicates';
  var dst = _.containerAdapter.make( new Set( [ { b : 0 }, { b : 1 } ] ) );
  var got = dst.appendOnce( { a : 0 }, ( e ) => e.b );
  var exp = [ { b : 0 }, { b : 1 }, { a : 0 } ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );
}

//

function setAdapterAppendOnceTwoEvaluators( test )
{
  test.open( 'two evaluators' );

  test.case = 'empty container, append primitive';
  var dst = _.containerAdapter.make( new Set() );
  var got = dst.appendOnce( 1, ( e ) => e, ( ins ) => ins );
  var exp = [ 1 ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'empty container, append Long';
  var dst = _.containerAdapter.make( new Set() );
  var got = dst.appendOnce( _.unroll.make( [ 1, 2 ] ), ( e ) => e, ( ins ) => ins );
  var exp = [ [ 1, 2 ] ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'empty container, append map';
  var dst = _.containerAdapter.make( new Set() );
  var got = dst.appendOnce( { a : 0 }, ( e ) => e, ( ins ) => ins );
  var exp = [ { a : 0 } ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  /* */

  test.case = 'container, append primitive';
  var dst = _.containerAdapter.make( new Set( [ undefined ] ) );
  var got = dst.appendOnce( 1, ( e ) => e, ( ins ) => ins );
  var exp = [ undefined, 1 ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'container, append Long';
  var dst = _.containerAdapter.make( new Set( [ null ] ) );
  var got = dst.appendOnce( _.unroll.make( [ 1, 2 ] ), ( e ) => e, ( ins ) => ins );
  var exp = [ null, [ 1, 2 ] ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'container, append map';
  var dst = _.containerAdapter.make( new Set( [ 0 ] ) );
  var got = dst.appendOnce( { a : 0 }, ( e ) => e, ( ins ) => ins );
  var exp = [ 0, { a : 0 } ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  /* */

  test.case = 'container, append primitive, duplicates';
  var dst = _.containerAdapter.make( new Set( [ 1, 2, 3 ] ) );
  var got = dst.appendOnce( 4, ( e ) => e, ( ins ) => 0 );
  var exp = [ 1, 2, 3, 4 ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'container, append Long, duplicates';
  var dst = _.containerAdapter.make( new Set( [ [ 1, 2 ], [ 1, 3 ] ] ) );
  var got = dst.appendOnce( [ 1, 2 ], ( e ) => e[ 0 ], ( ins ) => ins[ 0 ] );
  var exp = [ [ 1, 2 ], [ 1, 3 ] ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'container, append Long, append duplicates';
  var dst = _.containerAdapter.make( new Set( [ [ 1, 3 ], [ 1, 3 ] ] ) );
  var got = dst.appendOnce( [ 1, 2 ], ( e ) => e[ 1 ], ( ins ) => ins[ 1 ] );
  var exp = [ [ 1, 3 ], [ 1, 3 ], [ 1, 2 ] ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'container, append map, duplicates';
  var dst = _.containerAdapter.make( new Set( [ { a : 0 }, { a : 1 } ] ) );
  var got = dst.appendOnce( { a : 0 }, ( e ) => e.a, ( ins ) => ins.a );
  var exp = [ { a : 0 }, { a : 1 } ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'container, append map, append duplicates';
  var dst = _.containerAdapter.make( new Set( [ { b : 0 }, { b : 1 } ] ) );
  var got = dst.appendOnce( { a : 0 }, ( e ) => e.b, ( ins ) => ins.a );
  var exp = [ { b : 0 }, { b : 1 } ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.close( 'two evaluators' );

  /* - */

  test.open( 'fromIndex' );

  test.case = 'container, append primitive';
  var dst = _.containerAdapter.make( new Set( [ 1, undefined, 2 ] ) );
  var got = dst.appendOnce( 1, 1, ( e ) => e );
  var exp = [ 1, undefined, 2 ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'container, append Long';
  var dst = _.containerAdapter.make( new Set( [ [ 1, 2 ], [ null ] ] ) );
  var got = dst.appendOnce( _.unroll.make( [ 1, 2 ] ), 1, ( e ) => e[ 0 ] );
  var exp = [ [ 1, 2 ], [ null ], [ 1, 2 ] ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'container, append map';
  var dst = _.containerAdapter.make( new Set( [ { a : 0 }, 0 ] ) );
  var got = dst.appendOnce( { a : 0 }, 1, ( e ) => e.a );
  var exp = [ { a : 0 }, 0, { a : 0 } ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  /* */

  test.case = 'container, append primitive, duplicates';
  var dst = _.containerAdapter.make( new Set( [ 1, 2, 3 ] ) );
  var got = dst.appendOnce( 2, 2, ( e ) => e );
  var exp = [ 1, 2, 3 ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'container, append Long, duplicates';
  var dst = _.containerAdapter.make( new Set( [ [ 1, 2 ], [ 1, 3 ] ] ) );
  var got = dst.appendOnce( [ 1, 2 ], 2, ( e ) => e[ 0 ] );
  var exp = [ [ 1, 2 ], [ 1, 3 ], [ 1, 2 ] ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'container, append Long, append duplicates';
  var dst = _.containerAdapter.make( new Set( [ [ 1, 3 ], [ 1, 3 ] ] ) );
  var got = dst.appendOnce( [ 1, 3 ], 2, ( e ) => e[ 1 ] );
  var exp = [ [ 1, 3 ], [ 1, 3 ], [ 1, 3 ] ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'container, append map, duplicates';
  var dst = _.containerAdapter.make( new Set( [ { a : 0 }, { a : 1 } ] ) );
  var got = dst.appendOnce( { a : 0 }, 1, ( e ) => e.a );
  var exp = [ { a : 0 }, { a : 1 }, { a : 0 } ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'container, append map, append duplicates';
  var dst = _.containerAdapter.make( new Set( [ { b : 1 }, { a : 0 } ] ) );
  var got = dst.appendOnce( { a : 0 }, 1, ( ins ) => ins.a );
  var exp = [ { b : 1 }, { a : 0 } ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.close( 'fromIndex' );
}

//

function setAdapterAppendOnceEqualizer( test )
{
  test.case = 'empty container, append primitive';
  var dst = _.containerAdapter.make( new Set() );
  var got = dst.appendOnce( 1, ( e, ins ) => e === ins );
  var exp = [ 1 ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'empty container, append Long';
  var dst = _.containerAdapter.make( new Set() );
  var got = dst.appendOnce( _.unroll.make( [ 1, 2 ] ), ( e, ins ) => e === ins );
  var exp = [ [ 1, 2 ] ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'empty container, append map';
  var dst = _.containerAdapter.make( new Set() );
  var got = dst.appendOnce( { a : 0 }, ( e, ins ) => e === ins );
  var exp = [ { a : 0 } ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  /* */

  test.case = 'container, append primitive';
  var dst = _.containerAdapter.make( new Set( [ undefined ] ) );
  var got = dst.appendOnce( 1, ( e, ins ) => e === ins );
  var exp = [ undefined, 1 ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'container, append Long';
  var dst = _.containerAdapter.make( new Set( [ null ] ) );
  var got = dst.appendOnce( _.unroll.make( [ 1, 2 ] ), ( e, ins ) => e === ins );
  var exp = [ null, [ 1, 2 ] ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'container, append map';
  var dst = _.containerAdapter.make( new Set( [ 0 ] ) );
  var got = dst.appendOnce( { a : 0 }, ( e, ins ) => e === ins );
  var exp = [ 0, { a : 0 } ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  /* */

  test.case = 'container, append primitive, duplicates';
  var dst = _.containerAdapter.make( new Set( [ 1, 2, 3 ] ) );
  var got = dst.appendOnce( 4, ( e, ins ) => e === 0 );
  var exp = [ 1, 2, 3, 4 ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'container, append Long, duplicates';
  var dst = _.containerAdapter.make( new Set( [ [ 1, 2 ], [ 1, 3 ] ] ) );
  var got = dst.appendOnce( [ 1, 2 ], ( e, ins ) => e[ 0 ] === ins[ 0 ] );
  var exp = [ [ 1, 2 ], [ 1, 3 ] ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'container, append Long, append duplicates';
  var dst = _.containerAdapter.make( new Set( [ [ 1, 3 ], [ 1, 3 ] ] ) );
  var got = dst.appendOnce( [ 1, 2 ], ( e, ins ) => e[ 0 ] === ins[ 1 ] );
  var exp = [ [ 1, 3 ], [ 1, 3 ], [ 1, 2 ] ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'container, append map, duplicates';
  var dst = _.containerAdapter.make( new Set( [ { a : 0 }, { a : 1 } ] ) );
  var got = dst.appendOnce( { a : 0 }, ( e, ins ) => e.a === ins.a );
  var exp = [ { a : 0 }, { a : 1 } ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'container, append map, append duplicates';
  var dst = _.containerAdapter.make( new Set( [ { b : 0 }, { b : 1 } ] ) );
  var got = dst.appendOnce( { a : 0 }, ( e, ins ) => e.b === ins.a );
  var exp = [ { b : 0 }, { b : 1 } ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );
}

//

function setAdapterAppendOnceStrictly( test )
{
  test.open( 'without callbacks' );

  test.case = 'empty container, append primitive';
  var dst = _.containerAdapter.make( new Set() );
  var got = dst.appendOnceStrictly( 1 );
  var exp = [ 1 ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'empty container, append Long';
  var dst = _.containerAdapter.make( new Set() );
  var got = dst.appendOnceStrictly( _.unroll.make( [ 1, 2 ] ) );
  var exp = [ [ 1, 2 ] ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'empty container, append map';
  var dst = _.containerAdapter.make( new Set() );
  var got = dst.appendOnceStrictly( { a : 0 } );
  var exp = [ { a : 0 } ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  /* */

  test.case = 'container, append primitive';
  var dst = _.containerAdapter.make( new Set( [ undefined ] ) );
  var got = dst.appendOnceStrictly( 1 );
  var exp = [ undefined, 1 ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'container, append Long';
  var dst = _.containerAdapter.make( new Set( [ null ] ) );
  var got = dst.appendOnceStrictly( _.unroll.make( [ 1, 2 ] ) );
  var exp = [ null, [ 1, 2 ] ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'container, append map';
  var dst = _.containerAdapter.make( new Set( [ 0 ] ) );
  var got = dst.appendOnceStrictly( { a : 0 } );
  var exp = [ 0, { a : 0 } ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  /* */

  test.case = 'container, append primitive';
  var dst = _.containerAdapter.make( new Set( [ 1, 2, 3 ] ) );
  var got = dst.appendOnceStrictly( 4 ).appendOnceStrictly( 5 );
  var exp = [ 1, 2, 3, 4, 5 ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'container, append Long, duplicates';
  var dst = _.containerAdapter.make( new Set( [ [ 1, 2 ] ] ) );
  var got = dst.appendOnceStrictly( [ 1, 2 ] ).appendOnceStrictly( [ 1, 2 ] );
  var exp = [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ] ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'container, append map, duplicates';
  var dst = _.containerAdapter.make( new Set( [ { a : 0 } ] ) );
  var got = dst.appendOnceStrictly( { a : 0 } ).appendOnceStrictly( { a : 0 } );
  var exp = [ { a : 0 }, { a : 0 }, { a : 0 } ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.close( 'without callbacks' );

  /* - */

  test.case = 'container append array, one evaluator';
  var dst = _.containerAdapter.make( new Set( [ 1, [ 2 ], 3 ] ) );
  var got = dst.appendOnceStrictly( [ 1 ], ( e ) => e[ 0 ] );
  var exp = [ 1, [ 2 ], 3, [ 1 ] ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'container append array, one evaluator';
  var dst = _.containerAdapter.make( new Set( [ [ 1 ], [ 1 ], [ 2 ], 3 ] ) );
  var got = dst.appendOnceStrictly( [ 3 ], ( e ) => e[ 0 ] );
  var exp = [ [ 1 ], [ 1 ], [ 2 ], 3, [ 3 ] ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'container append array, two evaluators';
  var dst = _.containerAdapter.make( new Set( [ 1, [ 2 ], 3 ] ) );
  var got = dst.appendOnceStrictly( [ 1 ], ( e ) => e + 1, ( ins ) => ins[ 0 ] );
  var exp = [ 1, [ 2 ], 3, [ 1 ] ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'container append array, two evaluators';
  var dst = _.containerAdapter.make( new Set( [ [ 1 ], [ 1 ], [ 2 ], 3 ] ) );
  var got = dst.appendOnceStrictly( [ 1 ], ( e ) => e, ( ins ) => ins[ 0 ] );
  var exp = [ [ 1 ], [ 1 ], [ 2 ], 3, [ 1 ] ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'container append array, fromIndex and evaluator2';
  var dst = _.containerAdapter.make( new Set( [ 1, [ 2 ], 3 ] ) );
  var got = dst.appendOnceStrictly( [ 1 ], 2, ( e ) => e[ 0 ] );
  var exp = [ 1, [ 2 ], 3, [ 1 ] ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'container append array, fromIndex and evaluator2';
  var dst = _.containerAdapter.make( new Set( [ [ 1 ], [ 1 ], [ 2 ], 3 ] ) );
  var got = dst.appendOnceStrictly( [ 1 ], 2, ( e ) => e[ 0 ] );
  var exp = [ [ 1 ], [ 1 ], [ 2 ], 3, [ 1 ] ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'container append array, equalizer';
  var dst = _.containerAdapter.make( new Set( [ 1, [ 2 ], 3 ] ) );
  var got = dst.appendOnceStrictly( [ 1 ], ( e, ins ) => e + 1 === ins[ 0 ] );
  var exp = [ 1, [ 2 ], 3, [ 1 ] ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'container append array, equalizer';
  var dst = _.containerAdapter.make( new Set( [ [ 1 ], [ 1 ], [ 2 ], 3 ] ) );
  var got = dst.appendOnceStrictly( [ 1 ], ( e, ins ) => e === ins[ 0 ] );
  var exp = [ [ 1 ], [ 1 ], [ 2 ], 3, [ 1 ] ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'duplicate, no onEvaluate';
  test.shouldThrowErrorSync( () =>
  {
    var src = _.containerAdapter.make( new Set( [ 1, 2, 3 ] ) );
    src.appendOnceStrictly( 1 );
  });

  test.case = 'duplicate, onEvaluate';
  test.shouldThrowErrorSync( () =>
  {
    var src = _.containerAdapter.make( new Set( [ 1, 2, 3 ] ) );
    src.appendOnceStrictly( [ 1 ], ( e, ins ) => e === ins[ 0 ] );
  });

  test.case = 'wrong length of onEvaluate';
  test.shouldThrowErrorSync( () =>
  {
    var src = _.containerAdapter.make( new Set( [ 1, 2, 3 ] ) );
    src.appendOnceStrictly( [ 1 ], () => 1 );
  });
}

setAdapterAppendOnceStrictly.timeOut = 10000;

//

function setAdapterPush( test )
{
  test.case = 'empty container, push primitive';
  var dst = _.containerAdapter.make( new Set() );
  var got = dst.push( 1 );
  var exp = [ 1 ];
  test.identical( [ ... dst.original ], exp );
  test.identical( got, 1 );

  test.case = 'empty container, push Long';
  var dst = _.containerAdapter.make( new Set() );
  var got = dst.push( _.unroll.make( [ 1, 2 ] ) );
  var exp = [ [ 1, 2 ] ];
  test.identical( [ ... dst.original ], exp );
  test.identical( got, 1 );

  test.case = 'empty container, push map';
  var dst = _.containerAdapter.make( new Set() );
  var got = dst.push( { a : 0 } );
  var exp = [ { a : 0 } ];
  test.identical( [ ... dst.original ], exp );
  test.identical( got, 1 );

  /* */

  test.case = 'container, push primitive';
  var dst = _.containerAdapter.make( new Set( [ undefined ] ) );
  var got = dst.push( 1 );
  var exp = [ undefined, 1 ];
  test.identical( [ ... dst.original ], exp );
  test.identical( got, 2 );

  test.case = 'container, push Long';
  var dst = _.containerAdapter.make( new Set( [ null ] ) );
  var got = dst.push( _.unroll.make( [ 1, 2 ] ) );
  var exp = [ null, [ 1, 2 ] ];
  test.identical( [ ... dst.original ], exp );
  test.identical( got, 2 );

  test.case = 'container, push map';
  var dst = _.containerAdapter.make( new Set( [ 0 ] ) );
  var got = dst.push( { a : 0 } );
  var exp = [ 0, { a : 0 } ];
  test.identical( [ ... dst.original ], exp );
  test.identical( got, 2 );

  /* */

  test.case = 'container, push primitive, duplicates';
  var dst = _.containerAdapter.make( new Set( [ 1, 2, 3 ] ) );
  var got = dst.push( 1 );
  var exp = [ 1, 2, 3 ];
  test.identical( [ ... dst.original ], exp );
  test.identical( got, 3 );

  test.case = 'container, push Long, duplicates';
  var dst = _.containerAdapter.make( new Set( [ [ 1, 2 ] ] ) );
  var got = dst.push( [ 1, 2 ] );
  var exp = [ [ 1, 2 ], [ 1, 2 ] ];
  test.identical( [ ... dst.original ], exp );
  test.identical( got, 2 );

  test.case = 'container, push map, duplicates';
  var dst = _.containerAdapter.make( new Set( [ { a : 0 } ] ) );
  var got = dst.push( { a : 0 } );
  var exp = [ { a : 0 }, { a : 0 } ];
  test.identical( [ ... dst.original ], exp );
  test.identical( got, 2 );
}

setAdapterPush.timeOut = 10000;

//

function setAdapterAppendContainer( test )
{
  test.case = 'src - empty container, append empty array';
  var dst = _.containerAdapter.make( new Set() );
  var src = [];
  var got = dst.appendContainer( src );
  var exp = [];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - empty container, append empty BufferTyped';
  var dst = _.containerAdapter.make( new Set() );
  var src = new U32x();
  var got = dst.appendContainer( src );
  var exp = [];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - empty container, append empty Set';
  var dst = _.containerAdapter.make( new Set() );
  var src = new Set();
  var got = dst.appendContainer( src );
  var exp = [];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - empty container, append empty arrayContainerAdapter';
  var dst = _.containerAdapter.make( new Set() );
  var src = _.containerAdapter.make( [] );
  var got = dst.appendContainer( src );
  var exp = [];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - empty container, append empty setContainerAdapter';
  var dst = _.containerAdapter.make( new Set() );
  var src = _.containerAdapter.make( new Set() );
  var got = dst.appendContainer( src );
  var exp = [];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  /* */

  test.case = 'src - container, append unroll';
  var dst = _.containerAdapter.make( new Set( [ 1, 2, 3 ] ) );
  var src = _.unroll.make( [ 1, { a : 2 }, [ 3 ], 'str', undefined ] );
  var got = dst.appendContainer( src );
  var exp = [ 1, 2, 3, { a : 2 }, [ 3 ], 'str', undefined ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - container, append argumentsArray';
  var dst = _.containerAdapter.make( new Set( [ 1, 2, 3 ] ) );
  var src = _.argumentsArray.make( [ 1, { a : 2 }, [ 3 ], 'str', undefined ] );
  var got = dst.appendContainer( src );
  var exp = [ 1, 2, 3, { a : 2 }, [ 3 ], 'str', undefined ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - container, append Set';
  var dst = _.containerAdapter.make( new Set( [ 1, 2, 3 ] ) );
  var src = new Set( [ 1, { a : 2 }, [ 3 ], 'str', undefined ] );
  var got = dst.appendContainer( src );
  var exp = [ 1, 2, 3, { a : 2 }, [ 3 ], 'str', undefined ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - container, append arrayContainerAdapter';
  var dst = _.containerAdapter.make( new Set( [ 1, 2, 3 ] ) );
  var src = _.containerAdapter.make( [ 1, { a : 2 }, [ 3 ], 'str', undefined ] );
  var got = dst.appendContainer( src );
  var exp = [ 1, 2, 3, { a : 2 }, [ 3 ], 'str', undefined ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - container, append setContainerAdapter';
  var dst = _.containerAdapter.make( new Set( [ 1, 2, 3 ] ) );
  var src = _.containerAdapter.make( new Set( [ 1, { a : 2 }, [ 3 ], 'str', undefined ] ) );
  var got = dst.appendContainer( src );
  var exp = [ 1, 2, 3, { a : 2 }, [ 3 ], 'str', undefined ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  /* */

  test.case = 'src - container, append array, duplicates';
  var dst = _.containerAdapter.make( new Set( [ 1, 2, 3 ] ) );
  var src = [ 1, 2, 3, { a : 2 }, { a : 2 }, [ 3 ], [ 3 ], undefined, undefined, undefined ];
  var got = dst.appendContainer( src );
  var exp = [ 1, 2, 3, { a : 2 }, { a : 2 }, [ 3 ], [ 3 ], undefined ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - container, append unroll';
  var dst = _.containerAdapter.make( new Set( [ 1, 2, 3 ] ) );
  var src = _.unroll.make( [ 1, 2, 3, { a : 2 }, { a : 2 }, [ 3 ], [ 3 ], undefined, undefined, undefined ] );
  var got = dst.appendContainer( src );
  var exp = [ 1, 2, 3, { a : 2 }, { a : 2 }, [ 3 ], [ 3 ], undefined ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - container, append Set';
  var dst = _.containerAdapter.make( new Set( [ 1, 2, 3 ] ) );
  var src = new Set( [ 1, 2, 3, { a : 2 }, { a : 2 }, [ 3 ], [ 3 ], undefined, undefined, undefined ] );
  var got = dst.appendContainer( src );
  var exp = [ 1, 2, 3, { a : 2 }, { a : 2 }, [ 3 ], [ 3 ], undefined ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - container, append arrayContainerAdapter';
  var dst = _.containerAdapter.make( new Set( [ 1, 2, 3 ] ) );
  var src = _.containerAdapter.make( [ 1, 2, 3, { a : 2 }, { a : 2 }, [ 3 ], [ 3 ], undefined, undefined, undefined ] );
  var got = dst.appendContainer( src );
  var exp = [ 1, 2, 3, { a : 2 }, { a : 2 }, [ 3 ], [ 3 ], undefined ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - container, append setContainerAdapter';
  var dst = _.containerAdapter.make( new Set( [ 1, 2, 3 ] ) );
  var src = _.containerAdapter.make( new Set( [ 1, 2, 3, { a : 2 }, { a : 2 }, [ 3 ], [ 3 ], undefined, undefined, undefined ] ) );
  var got = dst.appendContainer( src );
  var exp = [ 1, 2, 3, { a : 2 }, { a : 2 }, [ 3 ], [ 3 ], undefined ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () =>
  {
    var dst = _.containerAdapter.make( [] );
    dst.appendContainer();
  });

  test.case = 'wrong type of container';
  test.shouldThrowErrorSync( () =>
  {
    var dst = _.containerAdapter.make( [] );
    dst.appendContainer( 0 );
  });

  test.shouldThrowErrorSync( () =>
  {
    var dst = _.containerAdapter.make( [] );
    dst.appendContainer( new BufferRaw() );
  });

  test.shouldThrowErrorSync( () =>
  {
    var dst = _.containerAdapter.make( [] );
    dst.appendContainer( 'wrong' );
  });
}

setAdapterAppendContainer.timeOut = 10000;

//

function setAdapterAppendContainerOnce( test )
{
  test.case = 'dst - empty container, append empty array';
  var dst = _.containerAdapter.make( new Set() );
  var src = [];
  var got = dst.appendContainerOnce( src );
  var exp = [];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'dst - empty container, append empty BufferTyped';
  var dst = _.containerAdapter.make( new Set() );
  var src = new U32x();
  var got = dst.appendContainerOnce( src );
  var exp = [];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'dst - empty container, append empty Set';
  var dst = _.containerAdapter.make( new Set() );
  var src = new Set();
  var got = dst.appendContainerOnce( src );
  var exp = [];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'dst - empty container, append empty arrayContainerAdapter';
  var dst = _.containerAdapter.make( new Set() );
  var src = _.containerAdapter.make( [] );
  var got = dst.appendContainerOnce( src );
  var exp = [];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'dst - empty container, append empty setContainerAdapter';
  var dst = _.containerAdapter.make( new Set() );
  var src = _.containerAdapter.make( new Set() );
  var got = dst.appendContainerOnce( src );
  var exp = [];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  /* */

  test.case = 'append unroll';
  var dst = _.containerAdapter.make( new Set( [ 1, 2, 3 ] ) );
  var src = _.unroll.make( [ 1, { a : 2 }, [ 3 ], 'str', undefined ] );
  var got = dst.appendContainerOnce( src );
  var exp = [ 1, 2, 3, { a : 2 }, [ 3 ], 'str', undefined ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'append argumentsArray';
  var dst = _.containerAdapter.make( new Set( [ 1, 2, 3 ] ) );
  var src = _.argumentsArray.make( [ 1, { a : 2 }, [ 3 ], 'str', undefined ] );
  var got = dst.appendContainerOnce( src );
  var exp = [ 1, 2, 3, { a : 2 }, [ 3 ], 'str', undefined ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'append Set';
  var dst = _.containerAdapter.make( new Set( [ 1, 2, 3 ] ) );
  var src = new Set( [ 1, { a : 2 }, [ 3 ], 'str', undefined ] );
  var got = dst.appendContainerOnce( src );
  var exp = [ 1, 2, 3, { a : 2 }, [ 3 ], 'str', undefined ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'append arrayContainerAdapter';
  var dst = _.containerAdapter.make( new Set( [ 1, 2, 3 ] ) );
  var src = _.containerAdapter.make( [ 1, { a : 2 }, [ 3 ], 'str', undefined ] );
  var got = dst.appendContainerOnce( src );
  var exp = [ 1, 2, 3, { a : 2 }, [ 3 ], 'str', undefined ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'append setContainerAdapter';
  var dst = _.containerAdapter.make( new Set( [ 1, 2, 3 ] ) );
  var src = _.containerAdapter.make( new Set( [ 1, { a : 2 }, [ 3 ], 'str', undefined ] ) );
  var got = dst.appendContainerOnce( src );
  var exp = [ 1, 2, 3, { a : 2 }, [ 3 ], 'str', undefined ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  /* */

  test.case = 'append array, duplicates';
  var dst = _.containerAdapter.make( new Set( [ 1, 2, 3 ] ) );
  var src = [ 1, 2, 3, { a : 2 }, { a : 2 }, [ 3 ], [ 3 ], undefined, undefined, undefined ];
  var got = dst.appendContainerOnce( src );
  var exp = [ 1, 2, 3, { a : 2 }, { a : 2 }, [ 3 ], [ 3 ], undefined ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'append unroll';
  var dst = _.containerAdapter.make( new Set( [ 1, 2, 3 ] ) );
  var src = _.unroll.make( [ 1, 2, 3, { a : 2 }, { a : 2 }, [ 3 ], [ 3 ], undefined, undefined, undefined ] );
  var got = dst.appendContainerOnce( src );
  var exp = [ 1, 2, 3, { a : 2 }, { a : 2 }, [ 3 ], [ 3 ], undefined ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'append Set';
  var dst = _.containerAdapter.make( new Set( [ 1, 2, 3 ] ) );
  var src = new Set( [ 1, 2, 3, { a : 2 }, { a : 2 }, [ 3 ], [ 3 ], undefined, undefined, undefined ] );
  var got = dst.appendContainerOnce( src );
  var exp = [ 1, 2, 3, { a : 2 }, { a : 2 }, [ 3 ], [ 3 ], undefined ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'append arrayContainerAdapter';
  var dst = _.containerAdapter.make( new Set( [ 1, 2, 3 ] ) );
  var src = _.containerAdapter.make( [ 1, 2, 3, { a : 2 }, { a : 2 }, [ 3 ], [ 3 ], undefined, undefined, undefined ] );
  var got = dst.appendContainerOnce( src );
  var exp = [ 1, 2, 3, { a : 2 }, { a : 2 }, [ 3 ], [ 3 ], undefined ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'append setContainerAdapter';
  var dst = _.containerAdapter.make( new Set( [ 1, 2, 3 ] ) );
  var src = _.containerAdapter.make( new Set( [ 1, 2, 3, { a : 2 }, { a : 2 }, [ 3 ], [ 3 ], undefined, undefined, undefined ] ) );
  var got = dst.appendContainerOnce( src );
  var exp = [ 1, 2, 3, { a : 2 }, { a : 2 }, [ 3 ], [ 3 ], undefined ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  /* */

  test.case = 'append array, one evaluator';
  var dst = _.containerAdapter.make( new Set( [ 1, 2, 3 ] ) );
  var src = _.containerAdapter.make( [ 1, 2, 3, { a : 2 }, { a : 2 }, [ 3 ], [ 3 ], [ undefined ], [ undefined ], [ undefined ] ] );
  var got = dst.appendContainerOnce( src, ( e ) => e[ 0 ] );
  var exp = [ 1, 2, 3, [ 3 ] ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'dst - container, append array, two evaluators';
  var dst = _.containerAdapter.make( new Set( [ 1, 2, 3 ] ) );
  var src = _.containerAdapter.make( [ 1, 2, 3, { a : 2 }, { a : 2 }, [ 3 ], [ 3 ], [ undefined ], [ undefined ], [ undefined ] ] );
  var got = dst.appendContainerOnce( src, ( e ) => e, ( ins ) => ins[ 0 ] );
  var exp = [ 1, 2, 3, { a : 2 }, { a : 2 }, [ undefined ], [ undefined ], [ undefined ] ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'dst - container, append array, fromIndex and evaluator2';
  var dst = _.containerAdapter.make( new Set( [ 1, 2, 3 ] ) );
  var src = _.containerAdapter.make( [ 1, 2, 3, { a : 2 }, { a : 2 }, [ 3 ], [ 3 ], [ undefined ], [ undefined ], [ undefined ] ] );
  var got = dst.appendContainerOnce( src, 2, ( e ) => e.a );
  var exp = [ 1, 2, 3, { a : 2 } ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );


  test.case = 'dst - container, append array, equalizer';
  var dst = _.containerAdapter.make( new Set( [ 1, 2, 3 ] ) );
  var src = _.containerAdapter.make( [ 1, 2, 3, { a : 2 }, { a : 2 }, [ 3 ], [ 3 ], [ undefined ], [ undefined ], [ undefined ] ] );
  var got = dst.appendContainerOnce( src, ( e, ins ) => e === ins[ 0 ] );
  var exp = [ 1, 2, 3, { a : 2 }, { a : 2 }, [ undefined ], [ undefined ], [ undefined ] ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  /* */

  test.case = 'append setAdapter, one evaluator';
  var dst = _.containerAdapter.make( new Set( [ 1, 2, 3 ] ) );
  var src = _.containerAdapter.make( new Set( [ 1, 2, 3, { a : 2 }, { a : 2 }, [ 3 ], [ 3 ], [ undefined ], [ undefined ], [ undefined ] ] ) );
  var got = dst.appendContainerOnce( src, ( e ) => e[ 0 ] );
  var exp = [ 1, 2, 3, [ 3 ] ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'dst - container, append setAdapter, two evaluators';
  var dst = _.containerAdapter.make( new Set( [ 1, 2, 3 ] ) );
  var src = _.containerAdapter.make( new Set( [ 1, 2, 3, { a : 2 }, { a : 2 }, [ 3 ], [ 3 ], [ undefined ], [ undefined ], [ undefined ] ] ) );
  var got = dst.appendContainerOnce( src, ( e ) => e, ( ins ) => ins[ 0 ] );
  var exp = [ 1, 2, 3, { a : 2 }, { a : 2 }, [ undefined ], [ undefined ], [ undefined ] ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'dst - container, append setAdapter, fromIndex and evaluator2';
  var dst = _.containerAdapter.make( new Set( [ 1, 2, 3 ] ) );
  var src = _.containerAdapter.make( new Set( [ 1, 2, 3, { a : 2 }, { a : 2 }, [ 3 ], [ 3 ], [ undefined ], [ undefined ], [ undefined ] ] ) );
  var got = dst.appendContainerOnce( src, 3, ( e ) => e.a );
  var exp = [ 1, 2, 3, { a : 2 }, [ 3 ] ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );


  test.case = 'dst - container, append setAdapter, equalizer';
  var dst = _.containerAdapter.make( new Set( [ 1, 2, 3 ] ) );
  var src = _.containerAdapter.make( new Set( [ 1, 2, 3, { a : 2 }, { a : 2 }, [ 3 ], [ 3 ], [ undefined ], [ undefined ], [ undefined ] ] ) );
  var got = dst.appendContainerOnce( src, ( e, ins ) => e === ins[ 0 ] );
  var exp = [ 1, 2, 3, { a : 2 }, { a : 2 }, [ undefined ], [ undefined ], [ undefined ] ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );
  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () =>
  {
    var dst = _.containerAdapter.make( new Set() );
    dst.appendContainerOnce();
  });

  test.case = 'wrong type of container';
  test.shouldThrowErrorSync( () =>
  {
    var dst = _.containerAdapter.make( new Set() );
    dst.appendContainerOnce( 0 );
  });

  test.shouldThrowErrorSync( () =>
  {
    var dst = _.containerAdapter.make( new Set() );
    dst.appendContainerOnce( new BufferRaw() );
  });

  test.shouldThrowErrorSync( () =>
  {
    var dst = _.containerAdapter.make( new Set() );
    dst.appendContainerOnce( 'wrong' );
  });
}

setAdapterAppendContainerOnce.timeOut = 10000;

//

function setAdapterAppendContainerOnceStrictly( test )
{
  test.case = 'dst - empty container, append empty array';
  var dst = _.containerAdapter.make( new Set() );
  var src = [];
  var got = dst.appendContainerOnceStrictly( src );
  var exp = [];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'dst - empty container, append empty BufferTyped';
  var dst = _.containerAdapter.make( new Set() );
  var src = new U32x();
  var got = dst.appendContainerOnceStrictly( src );
  var exp = [];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'dst - empty container, append empty Set';
  var dst = _.containerAdapter.make( new Set() );
  var src = new Set();
  var got = dst.appendContainerOnceStrictly( src );
  var exp = [];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'dst - empty container, append empty arrayContainerAdapter';
  var dst = _.containerAdapter.make( new Set() );
  var src = _.containerAdapter.make( [] );
  var got = dst.appendContainerOnceStrictly( src );
  var exp = [];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'dst - empty container, append empty setContainerAdapter';
  var dst = _.containerAdapter.make( new Set() );
  var src = _.containerAdapter.make( new Set() );
  var got = dst.appendContainerOnceStrictly( src );
  var exp = [];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  /* */

  test.case = 'append unroll';
  var dst = _.containerAdapter.make( new Set( [ 1, 2, 3 ] ) );
  var src = _.unroll.make( [ 4, { a : 2 }, [ 3 ], 'str', undefined ] );
  var got = dst.appendContainerOnceStrictly( src );
  var exp = [ 1, 2, 3, 4, { a : 2 }, [ 3 ], 'str', undefined ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'append argumentsArray';
  var dst = _.containerAdapter.make( new Set( [ 1, 2, 3 ] ) );
  var src = _.argumentsArray.make( [ 4, { a : 2 }, [ 3 ], 'str', undefined ] );
  var got = dst.appendContainerOnceStrictly( src );
  var exp = [ 1, 2, 3, 4, { a : 2 }, [ 3 ], 'str', undefined ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'append Set';
  var dst = _.containerAdapter.make( new Set( [ 1, 2, 3 ] ) );
  var src = new Set( [ 4, { a : 2 }, [ 3 ], 'str', undefined ] );
  var got = dst.appendContainerOnceStrictly( src );
  var exp = [ 1, 2, 3, 4, { a : 2 }, [ 3 ], 'str', undefined ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'append arrayContainerAdapter';
  var dst = _.containerAdapter.make( new Set( [ 1, 2, 3 ] ) );
  var src = _.containerAdapter.make( [ 4, { a : 2 }, [ 3 ], 'str', undefined ] );
  var got = dst.appendContainerOnceStrictly( src );
  var exp = [ 1, 2, 3, 4, { a : 2 }, [ 3 ], 'str', undefined ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'append setContainerAdapter';
  var dst = _.containerAdapter.make( new Set( [ 1, 2, 3 ] ) );
  var src = _.containerAdapter.make( new Set( [ 4, { a : 2 }, [ 3 ], 'str', undefined ] ) );
  var got = dst.appendContainerOnceStrictly( src );
  var exp = [ 1, 2, 3, 4, { a : 2 }, [ 3 ], 'str', undefined ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  /* */

  test.case = 'append arrayAdapter, one evaluator';
  var dst = _.containerAdapter.make( new Set( [ [ 1 ], [ 2 ], [ 3 ] ] ) );
  var src = _.containerAdapter.make( [ 1, [ 4 ], [ 5 ], [ null ] ] );
  var got = dst.appendContainerOnceStrictly( src, ( e ) => e[ 0 ] );
  var exp = [ [ 1 ], [ 2 ], [ 3 ], 1, [ 4 ], [ 5 ], [ null ] ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'dst - container, append arrayAdapter, two evaluators';
  var dst = _.containerAdapter.make( new Set( [ [ 1 ], [ 2 ], [ 3 ] ] ) );
  var src = _.containerAdapter.make( [ 1, 2, 3, { a : 2 }, { a : 2 }, [ 4 ], [ 5 ], [ undefined ] ] );
  var got = dst.appendContainerOnceStrictly( src, ( e ) => e, ( ins ) => ins[ 0 ] );
  var exp = [ [ 1 ], [ 2 ], [ 3 ], 1, 2, 3, { a : 2 }, { a : 2 }, [ 4 ], [ 5 ], [ undefined ] ]
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'dst - container, append arrayAdapter, fromIndex and evaluator2';
  var dst = _.containerAdapter.make( new Set( [ [ 1 ], [ 2 ], [ 3 ] ] ) );
  var src = _.containerAdapter.make( [ { a : 2 }, { a : 3 } ] );
  var got = dst.appendContainerOnceStrictly( src, 2, ( e ) => e.a );
  var exp = [ [ 1 ], [ 2 ], [ 3 ], { a : 2 }, { a : 3 } ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'dst - container, append arrayAdapter, equalizer';
  var dst = _.containerAdapter.make( new Set( [ [ 1 ], [ 2 ], [ 3 ] ] ) );
  var src = _.containerAdapter.make( [ 1, 2, 3, { a : 2 }, { a : 2 }, [ 4 ], [ 5 ], [ undefined ] ] );
  var got = dst.appendContainerOnceStrictly( src, ( e, ins ) => e === ins[ 0 ] );
  var exp = [ [ 1 ], [ 2 ], [ 3 ], 1, 2, 3, { a : 2 }, { a : 2 }, [ 4 ], [ 5 ], [ undefined ] ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  /* */

  test.case = 'append setAdapter, one evaluator';
  var dst = _.containerAdapter.make( new Set( [ [ 1 ], [ 2 ], [ 3 ] ] ) );
  var src = _.containerAdapter.make( new Set( [ 1, [ 4 ], [ 5 ], [ null ] ] ) );
  var got = dst.appendContainerOnceStrictly( src, ( e ) => e[ 0 ] );
  var exp = [ [ 1 ], [ 2 ], [ 3 ], 1, [ 4 ], [ 5 ], [ null ] ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'dst - container, append setAdapter, two evaluators';
  var dst = _.containerAdapter.make( new Set( [ [ 1 ], [ 2 ], [ 3 ] ] ) );
  var src = _.containerAdapter.make( new Set( [ 1, 2, 3, { a : 2 }, { a : 2 }, [ 4 ], [ 5 ], [ undefined ] ] ) );
  var got = dst.appendContainerOnceStrictly( src, ( e ) => e, ( ins ) => ins[ 0 ] );
  var exp = [ [ 1 ], [ 2 ], [ 3 ], 1, 2, 3, { a : 2 }, { a : 2 }, [ 4 ], [ 5 ], [ undefined ] ]
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'dst - container, append setAdapter, fromIndex and evaluator2';
  var dst = _.containerAdapter.make( new Set( [ [ 1 ], [ 2 ], [ 3 ] ] ) );
  var src = _.containerAdapter.make( new Set( [ { a : 2 }, { a : 3 } ] ) );
  var got = dst.appendContainerOnceStrictly( src, 2, ( e ) => e.a );
  var exp = [ [ 1 ], [ 2 ], [ 3 ], { a : 2 }, { a : 3 } ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'dst - container, append setAdapter, equalizer';
  var dst = _.containerAdapter.make( new Set( [ [ 1 ], [ 2 ], [ 3 ] ] ) );
  var src = _.containerAdapter.make( new Set( [ 1, 2, 3, { a : 2 }, { a : 2 }, [ 4 ], [ 5 ], [ undefined ] ] ) );
  var got = dst.appendContainerOnceStrictly( src, ( e, ins ) => e === ins[ 0 ] );
  var exp = [ [ 1 ], [ 2 ], [ 3 ], 1, 2, 3, { a : 2 }, { a : 2 }, [ 4 ], [ 5 ], [ undefined ] ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () =>
  {
    var dst = _.containerAdapter.make( new Set() );
    dst.appendContainerOnceStrictly();
  });

  test.case = 'wrong type of container';
  test.shouldThrowErrorSync( () =>
  {
    var dst = _.containerAdapter.make( new Set() );
    dst.appendContainerOnceStrictly( 0 );
  });

  test.shouldThrowErrorSync( () =>
  {
    var dst = _.containerAdapter.make( new Set() );
    dst.appendContainerOnceStrictly( new BufferRaw() );
  });

  test.shouldThrowErrorSync( () =>
  {
    var dst = _.containerAdapter.make( new Set() );
    dst.appendContainerOnceStrictly( 'wrong' );
  });

  test.case = 'duplicates in container';
  test.shouldThrowErrorSync( () =>
  {
    var dst = _.containerAdapter.make( new Set() );
    var src = [ 1, 2, 1, 2 ];
    dst.appendContainerOnceStrictly( src );
  });

  test.case = 'evaluator makes undefined from primitives';
  test.shouldThrowErrorSync( () =>
  {
    var dst = _.containerAdapter.make( new Set( [ [ 1 ] ] ) );
    var src = [ [ 3 ], [ 4 ], 1, 2 ];
    dst.appendContainerOnceStrictly( src, ( e ) => e[ 0 ] );
  });
}

setAdapterAppendContainerOnceStrictly.timeOut = 10000;

//

function setAdapterPop( test )
{
  test.case = 'container, last element is undefined, without arguments';
  var src = _.containerAdapter.make( new Set( [ null, 1, 'str', undefined ] ) );
  var got = src.pop();
  test.identical( [ ... src.original ], [ null, 1, 'str' ] );
  test.identical( got, undefined );

  test.case = 'container, last element is not undefined, without arguments';
  var src = _.containerAdapter.make( new Set( [ null, 1, 'str', [] ] ) );
  var got = src.pop();
  test.identical( [ ... src.original ], [ null, 1, 'str' ] );
  test.identical( got, [] );

  test.case = 'container, last element === searched element';
  var src = _.containerAdapter.make( new Set( [ null, 1, 'str' ] ) );
  var got = src.pop( 'str' );
  test.identical( [ ... src.original ], [ null, 1 ] );
  test.identical( got, 'str' );

  test.case = 'container contains searched element, not last';
  var src = _.containerAdapter.make( new Set( [ null, 1, 'str', undefined, [] ] ) );
  var got = src.pop( 'str' );
  test.identical( [ ... src.original ], [ null, 1, undefined, [] ] );
  test.identical( got, 'str' );

  test.case = 'container, last element - complex data, one evaluator';
  var src = _.containerAdapter.make( new Set( [ null, 1, 'str', [ 1 ] ] ) );
  var got = src.pop( [ 1 ], ( e ) => !e ? e : e[ 0 ] );
  test.identical( [ ... src.original ], [ null, 1, 'str' ] );
  test.identical( got, [ 1 ] );

  test.case = 'container, searched element is not last element, complex data, one evaluator';
  var src = _.containerAdapter.make( new Set( [ null, 1, 'str', [ 1 ], undefined, [ 1, 2 ], { a : 2 } ] ) );
  var got = src.pop( [ 1 ], ( e ) => !e ? e : e[ 0 ] );
  test.identical( [ ... src.original ], [ null, 1, 'str', [ 1 ], undefined, { a : 2 } ] );
  test.identical( got, [ 1, 2 ] );

  test.case = 'container, last element - complex data, two evaluators';
  var src = _.containerAdapter.make( new Set( [ null, 1, 'str', [ 1 ] ] ) );
  var got = src.pop( 1, ( e ) => !e ? e : e[ 0 ], ( ins ) => ins );
  test.identical( [ ... src.original ], [ null, 1, 'str' ] );
  test.identical( got, [ 1 ] );

  test.case = 'container, searched element is not last element, complex data, two evaluators';
  var src = _.containerAdapter.make( new Set( [ null, 1, 'str', [ 1 ], undefined, [ 1, 2 ], { a : 2 } ] ) );
  var got = src.pop( [ 1 ], ( e ) => !e ? e : e[ 0 ], ( ins ) => ins[ 0 ] );
  test.identical( [ ... src.original ], [ null, 1, 'str', [ 1 ], undefined, { a : 2 } ] );
  test.identical( got, [ 1, 2 ] );

  test.case = 'container, last element - complex data, equalizer';
  var src = _.containerAdapter.make( new Set( [ null, 1, 'str', [ 1 ] ] ) );
  var got = src.pop( [ 1 ], ( e, ins ) => e === ins[ 0 ] );
  test.identical( [ ... src.original ], [ null, 'str', [ 1 ] ] );
  test.identical( got, 1 );

  test.case = 'container, searched element is not last element, complex data, equalizer';
  var src = _.containerAdapter.make( new Set( [ null, 1, 'str', [ 1 ], undefined, [ 1, 2 ], { a : 2 } ] ) );
  var got = src.pop( [ 1 ], ( e, ins ) => e === ins[ 0 ] );
  test.identical( [ ... src.original ], [ null, 'str', [ 1 ], undefined, [ 1, 2 ], { a : 2 } ] );
  test.identical( got, 1 );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'empty container, pop without argument';
  test.shouldThrowErrorSync( () =>
  {
    var src = _.containerAdapter.make( new Set() );
    src.pop();
  });

  test.case = 'empty container, pop element';
  test.shouldThrowErrorSync( () =>
  {
    var src = _.containerAdapter.make( new Set() );
    src.pop( 2 );
  });

  test.case = 'complex data';
  test.shouldThrowErrorSync( () =>
  {
    var src = _.containerAdapter.make( new Set( [ [ 1 ], [ 0 ], [ 1 ] ] ) );
    src.pop( [ 1 ] );
  });
}

//

function setAdapterPopStrictly( test )
{
  test.case = 'container, last element === searched element';
  var src = _.containerAdapter.make( new Set( [ null, 1, 'str' ] ) );
  var got = src.popStrictly( 'str' );
  test.identical( [ ... src.original ], [ null, 1 ] );
  test.identical( got, 'str' );

  test.case = 'container, last element - complex data, one evaluator';
  var src = _.containerAdapter.make( new Set( [ null, 1, 'str', [ 1 ] ] ) );
  var got = src.popStrictly( [ 1 ], ( e ) => e[ 0 ] );
  test.identical( [ ... src.original ], [ null, 1, 'str' ] );
  test.identical( got, [ 1 ] );

  test.case = 'container, last element - complex data, two evaluators';
  var src = _.containerAdapter.make( new Set( [ null, 1, 'str', [ 1 ] ] ) );
  var got = src.popStrictly( 1, ( e ) => e[ 0 ], ( ins ) => ins );
  test.identical( [ ... src.original ], [ null, 1, 'str' ] );
  test.identical( got, [ 1 ] );

  test.case = 'container, last element - complex data, equalizer';
  var src = _.containerAdapter.make( new Set( [ null, 1, 'str', [ 1 ] ] ) );
  var got = src.popStrictly( 1, ( e, ins ) => e[ 0 ] === ins );
  test.identical( [ ... src.original ], [ null, 1, 'str' ] );
  test.identical( got, [ 1 ] );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without argument';
  test.shouldThrowErrorSync( () =>
  {
    var src = _.containerAdapter.make( new Set() );
    src.popStrictly();
  });

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () =>
  {
    var src = _.containerAdapter.make( new Set([ 1, 2, 3, 4 ] ) );
    src.popStrictly( 4, ( e ) => e, ( ins ) => ins, 'extra' );
  });

  test.case = 'empty container, pop element';
  test.shouldThrowErrorSync( () =>
  {
    var src = _.containerAdapter.make( new Set() );
    src.popStrictly( 2 );
  });

  test.case = 'popped element !== searched element';
  test.shouldThrowErrorSync( () =>
  {
    var src = _.containerAdapter.make( new Set( [ null, 1, 'str', undefined ] ) );
    src.popStrictly( 'str' );
  });

  test.case = 'complex data';
  test.shouldThrowErrorSync( () =>
  {
    var src = _.containerAdapter.make( new Set( [ [ 1 ], [ 0 ], [ 1 ] ] ) );
    src.popStrictly( [ 1 ] );
  });
}

setAdapterPopStrictly.timeOut = 10000;

//

function setAdapterRemoved( test )
{
  test.case = 'empty container, remove primitive';
  var dst = _.containerAdapter.make( new Set() );
  var got = dst.removed( 1 );
  var exp = [];
  test.identical( got, 0 );
  test.identical( [ ... dst.original ], exp );

  test.case = 'empty container, remove Long';
  var dst = _.containerAdapter.make( new Set() );
  var got = dst.removed( _.unroll.make( [ 1, 2 ] ) );
  var exp = [];
  test.identical( got, 0 );
  test.identical( [ ... dst.original ], exp );

  test.case = 'empty container, remove map';
  var dst = _.containerAdapter.make( new Set() );
  var got = dst.removed( { a : 0 } );
  var exp = [];
  test.identical( got, 0 );
  test.identical( [ ... dst.original ], exp );

  /* */

  test.case = 'container, remove primitive';
  var dst = _.containerAdapter.make( new Set( [ 1, 1, 2, 2, '1' ] ) );
  var got = dst.removed( 1 );
  var exp = [ 2, '1' ];
  test.identical( got, 1 );
  test.identical( [ ... dst.original ], exp );

  test.case = 'container, remove Long';
  var dst = _.containerAdapter.make( new Set( [ 1, 1, 2, 2, '1' ] ) );
  var got = dst.removed( _.unroll.make( [ 1, 2 ] ) );
  var exp = [ 1, 2, '1' ];
  test.identical( got, 0 );
  test.identical( [ ... dst.original ], exp );

  test.case = 'container, remove map';
  var dst = _.containerAdapter.make( new Set( [ 1, 1, 2, 2, '1' ] ) );
  var got = dst.removed( { a : 1 } );
  var exp = [ 1, 2, '1' ];
  test.identical( got, 0 );
  test.identical( [ ... dst.original ], exp );

  /* */

  test.case = 'container, remove primitive, duplicates';
  var dst = _.containerAdapter.make( new Set( [ 1, 2, 3 ] ) );
  var got = dst.removed( 1 );
  var exp = [ 2, 3 ];
  test.identical( got, 1 );
  test.identical( [ ... dst.original ], exp );

  test.case = 'container, remove Long, duplicates';
  var dst = _.containerAdapter.make( new Set( [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ] ) );
  var got = dst.removed( [ 1, 2 ] );
  var exp = [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ];
  test.identical( got, 0 );
  test.identical( [ ... dst.original ], exp );

  test.case = 'container, remove map, duplicates';
  var dst = _.containerAdapter.make( new Set( [ { a : 0 }, { a : 0 }, { a : 0 }, { a : 1 } ] ) );
  var got = dst.removed( { a : 0 } );
  var exp = [ { a : 0 }, { a : 0 }, { a : 0 }, { a : 1 } ];
  test.identical( got, 0 );
  test.identical( [ ... dst.original ], exp );

  /* */

  test.case = 'container, remove Long, one evaluator';
  var dst = _.containerAdapter.make( new Set( [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ] ) );
  var got = dst.removed( [ 1, 2 ], ( e ) => e[ 0 ] );
  var exp = [ [ 2, 2 ] ];
  test.identical( got, 3 );
  test.identical( [ ... dst.original ], exp );

  test.case = 'container, remove Long, two evaluators';
  var dst = _.containerAdapter.make( new Set( [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ] ) );
  var got = dst.removed( [ 1, 2 ], ( e ) => e[ 0 ], ( ins ) => ins[ 0 ] );
  var exp = [ [ 2, 2 ] ];
  test.identical( got, 3 );
  test.identical( [ ... dst.original ], exp );

  test.case = 'container, remove Long, fromIndex and evaluator2';
  var dst = _.containerAdapter.make( new Set( [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ] ) );
  var got = dst.removed( [ 1, 2 ], 2, ( e ) => e[ 0 ] );
  var exp = [ [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ];
  test.identical( got, 1 );
  test.identical( [ ... dst.original ], exp );

  test.case = 'container, remove Long, equalizer';
  var dst = _.containerAdapter.make( new Set( [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ] ) );
  var got = dst.removed( [ 1, 2 ], ( e, ins ) => e === ins[ 0 ] );
  var exp = [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ];
  test.identical( got, 0 );
  test.identical( [ ... dst.original ], exp );
}

setAdapterRemoved.timeOut = 10000;

//

function setAdapterRemovedOnce( test )
{
  test.case = 'empty container, remove primitive';
  var dst = _.containerAdapter.make( new Set() );
  var got = dst.removedOnce( 1 );
  var exp = [];
  test.identical( got, -1 );
  test.identical( [ ... dst.original ], exp );

  test.case = 'empty container, remove Long';
  var dst = _.containerAdapter.make( new Set() );
  var got = dst.removedOnce( _.unroll.make( [ 1, 2 ] ) );
  var exp = [];
  test.identical( got, -1 );
  test.identical( [ ... dst.original ], exp );

  test.case = 'empty container, remove empty Long';
  var dst = _.containerAdapter.make( new Set() );
  var got = dst.removedOnce( [] );
  var exp = [];
  test.identical( got, -1 );
  test.identical( [ ... dst.original ], exp );

  test.case = 'empty container, remove map';
  var dst = _.containerAdapter.make( new Set() );
  var got = dst.removedOnce( { a : 0 } );
  var exp = [];
  test.identical( got, -1 );
  test.identical( [ ... dst.original ], exp );

  /* */

  test.case = 'container, remove primitive';
  var dst = _.containerAdapter.make( new Set( [ 1, 1, 2, 2, '1' ] ) );
  var got = dst.removedOnce( 1 );
  var exp = [ 2, '1' ];
  test.identical( got, 0 );
  test.identical( [ ... dst.original ], exp );

  test.case = 'container, remove Long';
  var dst = _.containerAdapter.make( new Set( [ 1, 1, 2, 2, '1' ] ) );
  var got = dst.removedOnce( _.unroll.make( [ 1, 2 ] ) );
  var exp = [ 1, 2, '1' ];
  test.identical( got, -1 );
  test.identical( [ ... dst.original ], exp );

  test.case = 'container, remove map';
  var dst = _.containerAdapter.make( new Set( [ 1, 1, 2, 2, '1' ] ) );
  var got = dst.removedOnce( { a : 1 } );
  var exp = [ 1, 2, '1' ];
  test.identical( got, -1 );
  test.identical( [ ... dst.original ], exp );

  /* */

  test.case = 'container, remove primitive, duplicates';
  var dst = _.containerAdapter.make( new Set( [ 1, 1, 2, 2, 3, 3 ] ) );
  var got = dst.removedOnce( 2 );
  var exp = [ 1, 3 ];
  test.identical( got, 1 );
  test.identical( [ ... dst.original ], exp );

  test.case = 'container, remove Long, duplicates';
  var dst = _.containerAdapter.make( new Set( [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ] ) );
  var got = dst.removedOnce( [ 1, 2 ] );
  var exp = [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ];
  test.identical( got, -1 );
  test.identical( [ ... dst.original ], exp );

  test.case = 'container, remove map, duplicates';
  var dst = _.containerAdapter.make( new Set( [ { a : 0 }, { a : 0 }, { a : 0 }, { a : 1 } ] ) );
  var got = dst.removedOnce( { a : 0 } );
  var exp = [ { a : 0 }, { a : 0 }, { a : 0 }, { a : 1 } ];
  test.identical( got, -1 );
  test.identical( [ ... dst.original ], exp );

  /* */

  test.case = 'container, remove Long, one evaluator';
  var dst = _.containerAdapter.make( new Set( [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ] ) );
  var got = dst.removedOnce( [ 2, 2 ], ( e ) => e[ 0 ] );
  var exp = [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ] ];
  test.identical( got, 3 );
  test.identical( [ ... dst.original ], exp );

  test.case = 'container, remove Long, two evaluators';
  var dst = _.containerAdapter.make( new Set( [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ] ) );
  var got = dst.removedOnce( [ 2, 2 ], ( e ) => e[ 0 ], ( ins ) => ins[ 0 ] );
  var exp = [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ] ];
  test.identical( got, 3 );
  test.identical( [ ... dst.original ], exp );

  test.case = 'container, remove Long, fromIndex and evaluator2';
  var dst = _.containerAdapter.make( new Set( [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ] ) );
  var got = dst.removedOnce( [ 2, 2 ], 2, ( e ) => e[ 0 ] );
  var exp = [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ] ];
  test.identical( got, 3 );
  test.identical( [ ... dst.original ], exp );

  test.case = 'container, remove Long, equalizer';
  var dst = _.containerAdapter.make( new Set( [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ] ) );
  var got = dst.removedOnce( [ 2, 2 ], ( e, ins ) => e === ins[ 0 ] );
  var exp = [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ];
  test.identical( got, -1 );
  test.identical( [ ... dst.original ], exp );
}

setAdapterRemovedOnce.timeOut = 10000;

//

function setAdapterRemovedOnceRight( test )
{
  test.case = 'empty container, remove primitive';
  var dst = _.containerAdapter.make( new Set() );
  var got = dst.removedOnceRight( 1 );
  var exp = [];
  test.identical( got, -1 );
  test.identical( [ ... dst.original ], exp );

  test.case = 'empty container, remove Long';
  var dst = _.containerAdapter.make( new Set() );
  var got = dst.removedOnceRight( _.unroll.make( [ 1, 2 ] ) );
  var exp = [];
  test.identical( got, -1 );
  test.identical( [ ... dst.original ], exp );

  test.case = 'empty container, remove empty Long';
  var dst = _.containerAdapter.make( new Set() );
  var got = dst.removedOnceRight( [] );
  var exp = [];
  test.identical( got, -1 );
  test.identical( [ ... dst.original ], exp );

  test.case = 'empty container, remove map';
  var dst = _.containerAdapter.make( new Set() );
  var got = dst.removedOnceRight( { a : 0 } );
  var exp = [];
  test.identical( got, -1 );
  test.identical( [ ... dst.original ], exp );

  /* */

  test.case = 'container, remove primitive';
  var dst = _.containerAdapter.make( new Set( [ 1, 1, 2, 2, '1' ] ) );
  var got = dst.removedOnceRight( 1 );
  var exp = [ 2, '1' ];
  test.identical( got, 0 );
  test.identical( [ ... dst.original ], exp );

  test.case = 'container, remove Long';
  var dst = _.containerAdapter.make( new Set( [ 1, 1, 2, 2, '1' ] ) );
  var got = dst.removedOnceRight( _.unroll.make( [ 1, 2 ] ) );
  var exp = [ 1, 2, '1' ];
  test.identical( got, -1 );
  test.identical( [ ... dst.original ], exp );

  test.case = 'container, remove map';
  var dst = _.containerAdapter.make( new Set( [ 1, 1, 2, 2, '1' ] ) );
  var got = dst.removedOnceRight( { a : 1 } );
  var exp = [ 1, 2, '1' ];
  test.identical( got, -1 );
  test.identical( [ ... dst.original ], exp );

  /* */

  test.case = 'container, remove primitive, duplicates';
  var dst = _.containerAdapter.make( new Set( [ 1, 1, 2, 2, 3, 3 ] ) );
  var got = dst.removedOnceRight( 2 );
  var exp = [ 1, 3 ];
  test.identical( got, 1 );
  test.identical( [ ... dst.original ], exp );

  test.case = 'container, remove Long, duplicates';
  var dst = _.containerAdapter.make( new Set( [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ] ) );
  var got = dst.removedOnceRight( [ 1, 2 ] );
  var exp = [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ];
  test.identical( got, -1 );
  test.identical( [ ... dst.original ], exp );

  test.case = 'container, remove map, duplicates';
  var dst = _.containerAdapter.make( new Set( [ { a : 0 }, { a : 0 }, { a : 0 }, { a : 1 } ] ) );
  var got = dst.removedOnceRight( { a : 0 } );
  var exp = [ { a : 0 }, { a : 0 }, { a : 0 }, { a : 1 } ];
  test.identical( got, -1 );
  test.identical( [ ... dst.original ], exp );

  /* */

  test.case = 'container, remove Long, one evaluator';
  var dst = _.containerAdapter.make( new Set( [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ] ) );
  var got = dst.removedOnceRight( [ 1, 2 ], ( e ) => e[ 0 ] );
  var exp = [ [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ];
  test.identical( got, 2 );
  test.identical( [ ... dst.original ], exp );

  test.case = 'container, remove Long, two evaluators';
  var dst = _.containerAdapter.make( new Set( [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ] ) );
  var got = dst.removedOnceRight( [ 2, 2 ], ( e ) => e[ 0 ], ( ins ) => ins[ 0 ] );
  var exp = [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ] ];
  test.identical( got, 3 );
  test.identical( [ ... dst.original ], exp );

  test.case = 'container, remove Long, fromIndex and evaluator2';
  var dst = _.containerAdapter.make( new Set( [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ] ) );
  var got = dst.removedOnceRight( [ 1, 2 ], 2, ( e ) => e[ 0 ] );
  var exp = [ [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ];
  test.identical( got, 1 );
  test.identical( [ ... dst.original ], exp );

  test.case = 'container, remove Long, equalizer';
  var dst = _.containerAdapter.make( new Set( [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ] ) );
  var got = dst.removedOnceRight( [ 2, 2 ], ( e, ins ) => e[ 0 ] === ins[ 0 ] );
  var exp = [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ] ];
  test.identical( got, 3 );
  test.identical( [ ... dst.original ], exp );
}

//

function setAdapterRemovedOnceStrictly( test )
{
  test.case = 'container, remove primitive';
  var dst = _.containerAdapter.make( new Set( [ 1, 2, 2, '1' ] ) );
  var got = dst.removedOnceStrictly( 1 );
  var exp = [ 2, '1' ];
  test.identical( got, 0 );
  test.identical( [ ... dst.original ], exp );

  /* */

  test.case = 'container, remove Long, one evaluator';
  var dst = _.containerAdapter.make( new Set( [ [ 1, 2 ], [ 2, 2 ] ] ) );
  var got = dst.removedOnceStrictly( [ 1, 2 ], ( e ) => e[ 0 ] );
  var exp = [ [ 2, 2 ] ];
  test.identical( got, 0 );
  test.identical( [ ... dst.original ], exp );

  test.case = 'container, remove Long, two evaluators';
  var dst = _.containerAdapter.make( new Set( [ [ 3, 2 ], [ 1, 2 ], [ 2, 2 ] ] ) );
  var got = dst.removedOnceStrictly( [ 1, 2 ], ( e ) => e[ 0 ], ( ins ) => ins[ 0 ] );
  var exp = [ [ 3, 2 ], [ 2, 2 ] ];
  test.identical( got, 1 );
  test.identical( [ ... dst.original ], exp );

  test.case = 'container, remove Long, fromIndex and evaluator2';
  var dst = _.containerAdapter.make( new Set( [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ] ) );
  var got = dst.removedOnceStrictly( [ 1, 2 ], 2, ( e ) => e[ 0 ] );
  var exp = [ [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ];
  test.identical( got, 2 );
  test.identical( [ ... dst.original ], exp );

  test.case = 'container, remove Long, equalizer';
  var dst = _.containerAdapter.make( new Set( [ [ 1, 2 ], [ 2, 2 ] ] ) );
  var got = dst.removedOnceStrictly( [ 1, 2 ], ( e, ins ) => e[ 0 ] === ins[ 0 ] );
  var exp = [ [ 2, 2 ] ];
  test.identical( got, 0 );
  test.identical( [ ... dst.original ], exp );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'empty container'
  test.shouldThrowErrorSync( () =>
  {
    var dst = _.containerAdapter.make( new Set() );
    var got = dst.removedOnceStrictly( 1 );
  });

  test.case = 'container does not have element';
  test.shouldThrowErrorSync( () =>
  {
    var dst = _.containerAdapter.make( new Set( [ 1, 1, 2, 2, '1' ] ) );
    var got = dst.removedOnceStrictly( [ 1, 2 ] );
  });

  test.case = 'container has a few elements';
  test.shouldThrowErrorSync( () =>
  {
    var dst = _.containerAdapter.make( new Set( [ [ 1 ], [ 1 ], 2, 2, '1' ] ) );
    var got = dst.removedOnceStrictly( [ 1 ], ( e ) => e[ 0 ] );
  });
}

//

function setAdapterRemovedOnceStrictlyRight( test )
{
  test.case = 'container, remove primitive';
  var dst = _.containerAdapter.make( new Set( [ 1, 2, 2, '1' ] ) );
  var got = dst.removedOnceStrictlyRight( 1 );
  var exp = [ 2, '1' ];
  test.identical( got, 0 );
  test.identical( [ ... dst.original ], exp );

  /* */

  test.case = 'container, remove Long, one evaluator';
  var dst = _.containerAdapter.make( new Set( [ [ 1, 2 ], [ 2, 2 ] ] ) );
  var got = dst.removedOnceStrictlyRight( [ 1, 2 ], ( e ) => e[ 0 ] );
  var exp = [ [ 2, 2 ] ];
  test.identical( got, 0 );
  test.identical( [ ... dst.original ], exp );

  test.case = 'container, remove Long, two evaluators';
  var dst = _.containerAdapter.make( new Set( [ [ 3, 2 ], [ 1, 2 ], [ 2, 2 ] ] ) );
  var got = dst.removedOnceStrictlyRight( [ 1, 2 ], ( e ) => e[ 0 ], ( ins ) => ins[ 0 ] );
  var exp = [ [ 3, 2 ], [ 2, 2 ] ];
  test.identical( got, 1 );
  test.identical( [ ... dst.original ], exp );

  test.case = 'container, remove Long, fromIndex and evaluator2';
  var dst = _.containerAdapter.make( new Set( [ [ 2, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ] ) );
  var got = dst.removedOnceStrictlyRight( [ 1, 2 ], 2, ( e ) => e[ 0 ] );
  var exp = [ [ 2, 2 ], [ 1, 2 ], [ 2, 2 ] ];
  test.identical( got, 1 );
  test.identical( [ ... dst.original ], exp );

  test.case = 'container, remove Long, equalizer';
  var dst = _.containerAdapter.make( new Set( [ [ 1, 2 ], [ 2, 2 ] ] ) );
  var got = dst.removedOnceStrictlyRight( [ 1, 2 ], ( e, ins ) => e[ 0 ] === ins[ 0 ] );
  var exp = [ [ 2, 2 ] ];
  test.identical( got, 0 );
  test.identical( [ ... dst.original ], exp );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'empty container'
  test.shouldThrowErrorSync( () =>
  {
    var dst = _.containerAdapter.make( new Set() );
    var got = dst.removedOnceStrictlyRight( 1 );
  });

  test.case = 'container does not have element';
  test.shouldThrowErrorSync( () =>
  {
    var dst = _.containerAdapter.make( new Set( [ 1, 1, 2, 2, '1' ] ) );
    var got = dst.removedOnceStrictlyRight( [ 1, 2 ] );
  });

  test.case = 'container has a few elements';
  test.shouldThrowErrorSync( () =>
  {
    var dst = _.containerAdapter.make( new Set( [ [ 1 ], [ 1 ], 2, 2, '1' ] ) );
    var got = dst.removedOnceStrictlyRight( [ 1 ], ( e ) => e[ 0 ] );
  });
}

//

function setAdapterRemove( test )
{
  test.case = 'empty container, remove primitive';
  var dst = _.containerAdapter.make( new Set() );
  var got = dst.remove( 1 );
  var exp = [];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'empty container, remove Long';
  var dst = _.containerAdapter.make( new Set() );
  var got = dst.remove( _.unroll.make( [ 1, 2 ] ) );
  var exp = [];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'empty container, remove map';
  var dst = _.containerAdapter.make( new Set() );
  var got = dst.remove( { a : 0 } );
  var exp = [];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  /* */

  test.case = 'container, remove primitive';
  var dst = _.containerAdapter.make( new Set( [ 1, 1, 2, 2, '1' ] ) );
  var got = dst.remove( 1 );
  var exp = [ 2, '1' ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'container, remove Long';
  var dst = _.containerAdapter.make( new Set( [ 1, 1, 2, 2, '1' ] ) );
  var got = dst.remove( _.unroll.make( [ 1, 2 ] ) );
  var exp = [ 1, 2, '1' ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'container, remove map';
  var dst = _.containerAdapter.make( new Set( [ 1, 1, 2, 2, '1' ] ) );
  var got = dst.remove( { a : 1 } );
  var exp = [ 1, 2, '1' ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  /* */

  test.case = 'container, remove primitive, duplicates';
  var dst = _.containerAdapter.make( new Set( [ 1, 2, 3 ] ) );
  var got = dst.remove( 1 ).remove( 2 );
  var exp = [ 3 ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'container, remove Long, duplicates';
  var dst = _.containerAdapter.make( new Set( [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ] ) );
  var got = dst.remove( [ 1, 2 ] ).remove( [ 1, 2 ] );
  var exp = [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'container, remove map, duplicates';
  var dst = _.containerAdapter.make( new Set( [ { a : 0 }, { a : 0 }, { a : 0 }, { a : 1 } ] ) );
  var got = dst.remove( { a : 0 } ).remove( { a : 0 } );
  var exp = [ { a : 0 }, { a : 0 }, { a : 0 }, { a : 1 } ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  /* */

  test.case = 'container, remove Long, one evaluator';
  var dst = _.containerAdapter.make( new Set( [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ] ) );
  var got = dst.remove( [ 1, 2 ], ( e ) => e[ 0 ] );
  var exp = [ [ 2, 2 ] ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'container, remove Long, two evaluators';
  var dst = _.containerAdapter.make( new Set( [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ] ) );
  var got = dst.remove( [ 1, 2 ], ( e ) => e[ 0 ], ( ins ) => ins[ 0 ] );
  var exp = [ [ 2, 2 ] ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'container, remove Long, fromIndex and evaluator2';
  var dst = _.containerAdapter.make( new Set( [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ] ) );
  var got = dst.remove( [ 1, 2 ], 2, ( e ) => e[ 0 ] );
  var exp = [ [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'container, remove Long, equalizer';
  var dst = _.containerAdapter.make( new Set( [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ] ) );
  var got = dst.remove( [ 1, 2 ], ( e, ins ) => e === ins[ 0 ] );
  var exp = [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );
}

//

function setAdapterRemoveOnce( test )
{
  test.case = 'empty container, remove primitive';
  var dst = _.containerAdapter.make( new Set() );
  var got = dst.removeOnce( 1 );
  var exp = [];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'empty container, remove Long';
  var dst = _.containerAdapter.make( new Set() );
  var got = dst.removeOnce( _.unroll.make( [ 1, 2 ] ) );
  var exp = [];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'empty container, remove map';
  var dst = _.containerAdapter.make( new Set() );
  var got = dst.removeOnce( { a : 0 } );
  var exp = [];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  /* */

  test.case = 'container, remove primitive';
  var dst = _.containerAdapter.make( new Set( [ 1, 1, 2, 2, '1' ] ) );
  var got = dst.removeOnce( 1 );
  var exp = [ 2, '1' ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'container, remove Long';
  var dst = _.containerAdapter.make( new Set( [ 1, 1, 2, 2, '1' ] ) );
  var got = dst.removeOnce( _.unroll.make( [ 1, 2 ] ) );
  var exp = [ 1, 2, '1' ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'container, remove map';
  var dst = _.containerAdapter.make( new Set( [ 1, 1, 2, 2, '1' ] ) );
  var got = dst.removeOnce( { a : 1 } );
  var exp = [ 1, 2, '1' ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  /* */

  test.case = 'container, remove primitive, duplicates';
  var dst = _.containerAdapter.make( new Set( [ 1, 1, 2, 2, 3, 3 ] ) );
  var got = dst.removeOnce( 1 ).removeOnce( 2 );
  var exp = [ 3 ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'container, remove Long, duplicates';
  var dst = _.containerAdapter.make( new Set( [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ] ) );
  var got = dst.removeOnce( [ 1, 2 ] ).removeOnce( [ 1, 2 ] );
  var exp = [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'container, remove map, duplicates';
  var dst = _.containerAdapter.make( new Set( [ { a : 0 }, { a : 0 }, { a : 0 }, { a : 1 } ] ) );
  var got = dst.removeOnce( { a : 0 } ).removeOnce( { a : 0 } );
  var exp = [ { a : 0 }, { a : 0 }, { a : 0 }, { a : 1 } ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  /* */

  test.case = 'container, remove Long, one evaluator';
  var dst = _.containerAdapter.make( new Set( [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ] ) );
  var got = dst.removeOnce( [ 1, 2 ], ( e ) => e[ 0 ] );
  var exp = [ [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'container, remove Long, two evaluators';
  var dst = _.containerAdapter.make( new Set( [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ] ) );
  var got = dst.removeOnce( [ 1, 2 ], ( e ) => e[ 0 ], ( ins ) => ins[ 0 ] );
  var exp = [ [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'container, remove Long, fromIndex and evaluator2';
  var dst = _.containerAdapter.make( new Set( [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ] ) );
  var got = dst.removeOnce( [ 1, 2 ], 2, ( e ) => e[ 0 ] );
  var exp = [ [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'container, remove Long, equalizer';
  var dst = _.containerAdapter.make( new Set( [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ] ) );
  var got = dst.removeOnce( [ 1, 2 ], ( e, ins ) => e === ins[ 0 ] );
  var exp = [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );
}

//

function setAdapterRemoveOnceRight( test )
{
  test.case = 'empty container, remove primitive';
  var dst = _.containerAdapter.make( new Set() );
  var got = dst.removeOnceRight( 1 );
  var exp = [];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'empty container, remove Long';
  var dst = _.containerAdapter.make( new Set() );
  var got = dst.removeOnceRight( _.unroll.make( [ 1, 2 ] ) );
  var exp = [];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'empty container, remove empty Long';
  var dst = _.containerAdapter.make( new Set() );
  var got = dst.removeOnceRight( [] );
  var exp = [];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'empty container, remove map';
  var dst = _.containerAdapter.make( new Set() );
  var got = dst.removeOnceRight( { a : 0 } );
  var exp = [];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  /* */

  test.case = 'container, remove primitive';
  var dst = _.containerAdapter.make( new Set( [ 1, 1, 2, 2, '1' ] ) );
  var got = dst.removeOnceRight( 1 );
  var exp = [ 2, '1' ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'container, remove Long';
  var dst = _.containerAdapter.make( new Set( [ 1, 1, 2, 2, '1' ] ) );
  var got = dst.removeOnceRight( _.unroll.make( [ 1, 2 ] ) );
  var exp = [ 1, 2, '1' ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'container, remove map';
  var dst = _.containerAdapter.make( new Set( [ 1, 1, 2, 2, '1' ] ) );
  var got = dst.removeOnceRight( { a : 1 } );
  var exp = [ 1, 2, '1' ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  /* */

  test.case = 'container, remove primitive, duplicates';
  var dst = _.containerAdapter.make( new Set( [ 1, 1, 2, 2, 3, 3 ] ) );
  var got = dst.removeOnceRight( 1 ).removeOnceRight( 2 );
  var exp = [ 3 ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'container, remove Long, duplicates';
  var dst = _.containerAdapter.make( new Set( [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ] ) );
  var got = dst.removeOnceRight( [ 1, 2 ] ).removeOnceRight( [ 1, 2 ] );
  var exp = [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'container, remove map, duplicates';
  var dst = _.containerAdapter.make( new Set( [ { a : 0 }, { a : 0 }, { a : 0 }, { a : 1 } ] ) );
  var got = dst.removeOnceRight( { a : 0 } ).removeOnceRight( { a : 0 } );
  var exp = [ { a : 0 }, { a : 0 }, { a : 0 }, { a : 1 } ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  /* */

  test.case = 'container, remove Long, one evaluator';
  var dst = _.containerAdapter.make( new Set( [ [ 2, 2 ], [ 1, 3 ], [ 1, 2 ], [ 2, 2 ] ] ) );
  var got = dst.removeOnceRight( [ 1, 2 ], ( e ) => e[ 0 ] );
  var exp = [ [ 2, 2 ], [ 1, 3 ], [ 2, 2 ] ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'container, remove Long, two evaluators';
  var dst = _.containerAdapter.make( new Set( [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ] ) );
  var got = dst.removeOnceRight( [ 1, 2 ], ( e ) => e[ 0 ], ( ins ) => ins[ 0 ] );
  var exp = [ [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'container, remove Long, fromIndex and evaluator2';
  var dst = _.containerAdapter.make( new Set( [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ] ) );
  var got = dst.removeOnceRight( [ 1, 2 ], 2, ( e ) => e[ 0 ] );
  var exp = [ [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'container, remove Long, equalizer';
  var dst = _.containerAdapter.make( new Set( [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ] ) );
  var got = dst.removeOnceRight( [ 1, 2 ], ( e, ins ) => e === ins[ 0 ] );
  var exp = [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );
}

//

function setAdapterRemoveOnceStrictly( test )
{
  test.case = 'container, remove primitive';
  var dst = _.containerAdapter.make( new Set( [ 1, 2, 2, '1' ] ) );
  var got = dst.removeOnceStrictly( 1 );
  var exp = [ 2, '1' ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  /* */

  test.case = 'container, remove Long, one evaluator';
  var dst = _.containerAdapter.make( new Set( [ [ 1, 2 ], [ 2, 2 ] ] ) );
  var got = dst.removeOnceStrictly( [ 1, 2 ], ( e ) => e[ 0 ] );
  var exp = [ [ 2, 2 ] ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'container, remove Long, two evaluators';
  var dst = _.containerAdapter.make( new Set( [ [ 1, 2 ], [ 2, 2 ], [ 2, 2 ] ] ) );
  var got = dst.removeOnceStrictly( [ 1, 2 ], ( e ) => e[ 0 ], ( ins ) => ins[ 0 ] );
  var exp = [ [ 2, 2 ], [ 2, 2 ] ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'container, remove Long, fromIndex and evaluator2';
  var dst = _.containerAdapter.make( new Set( [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ] ) );
  var got = dst.removeOnceStrictly( [ 1, 2 ], 2, ( e ) => e[ 0 ] );
  var exp = [ [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'container, remove Long, equalizer';
  var dst = _.containerAdapter.make( new Set( [ [ 1, 2 ], [ 2, 2 ] ] ) );
  var got = dst.removeOnceStrictly( [ 1, 2 ], ( e, ins ) => e[ 0 ] === ins[ 0 ] );
  var exp = [ [ 2, 2 ] ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'empty container'
  test.shouldThrowErrorSync( () =>
  {
    var dst = _.containerAdapter.make( new Set() );
    var got = dst.removeOnceStrictly( 1 );
  });

  test.case = 'container does not have element';
  test.shouldThrowErrorSync( () =>
  {
    var dst = _.containerAdapter.make( new Set( [ 1, 1, 2, 2, '1' ] ) );
    var got = dst.removeOnceStrictly( [ 1, 2 ] );
  });

  test.case = 'container has a few elements';
  test.shouldThrowErrorSync( () =>
  {
    var dst = _.containerAdapter.make( new Set( [ [ 1 ], [ 1 ], 2, 2, '1' ] ) );
    var got = dst.removeOnceStrictly( [ 1 ], ( e ) => e[ 0 ] );
  });
}

//

function setAdapterRemoveOnceStrictlyRight( test )
{
  test.case = 'container, remove primitive';
  var dst = _.containerAdapter.make( new Set( [ 1, 2, 2, '1' ] ) );
  var got = dst.removeOnceStrictlyRight( 1 );
  var exp = [ 2, '1' ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  /* */

  test.case = 'container, remove Long, one evaluator';
  var dst = _.containerAdapter.make( new Set( [ [ 1, 2 ], [ 2, 2 ] ] ) );
  var got = dst.removeOnceStrictlyRight( [ 1, 2 ], ( e ) => e[ 0 ] );
  var exp = [ [ 2, 2 ] ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'container, remove Long, two evaluators';
  var dst = _.containerAdapter.make( new Set( [ [ 1, 2 ], [ 2, 2 ], [ 2, 2 ] ] ) );
  var got = dst.removeOnceStrictlyRight( [ 1, 2 ], ( e ) => e[ 0 ], ( ins ) => ins[ 0 ] );
  var exp = [ [ 2, 2 ], [ 2, 2 ] ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'container, remove Long, fromIndex and evaluator2';
  var dst = _.containerAdapter.make( new Set( [ [ 3, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ] ) );
  var got = dst.removeOnceStrictlyRight( [ 1, 2 ], 2, ( e ) => e[ 0 ] );
  var exp = [ [ 3, 2 ], [ 1, 2 ], [ 2, 2 ] ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'container, remove Long, equalizer';
  var dst = _.containerAdapter.make( new Set( [ [ 1, 2 ], [ 2, 2 ] ] ) );
  var got = dst.removeOnceStrictlyRight( [ 1, 2 ], ( e, ins ) => e[ 0 ] === ins[ 0 ] );
  var exp = [ [ 2, 2 ] ];
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'empty container'
  test.shouldThrowErrorSync( () =>
  {
    var dst = _.containerAdapter.make( new Set() );
    var got = dst.removeOnceStrictlyRight( 1 );
  });

  test.case = 'container does not have element';
  test.shouldThrowErrorSync( () =>
  {
    var dst = _.containerAdapter.make( new Set( [ 1, 1, 2, 2, '1' ] ) );
    var got = dst.removeOnceStrictlyRight( [ 1, 2 ] );
  });

  test.case = 'container has a few elements';
  test.shouldThrowErrorSync( () =>
  {
    var dst = _.containerAdapter.make( new Set( [ [ 1 ], [ 1 ], 2, 2, '1' ] ) );
    var got = dst.removeOnceStrictlyRight( [ 1 ], ( e ) => e[ 0 ] );
  });
}

//

function setAdapterEmpty( test )
{
  test.case = 'empty container';
  var src = _.containerAdapter.make( new Set() );
  var got = src.empty();
  test.identical( [ ... src.original ], [] );

  test.case = 'not empty container';
  var src = _.containerAdapter.make( new Set( [ 1, 2, 3, [ 1 ], null, undefined, { a : 0 } ] ) );
  var got = src.empty();
  test.identical( [ ... src.original ], [] );
}

//

function setAdapterMap( test )
{
  test.case = 'without arguments';
  var src = _.containerAdapter.make( new Set( [] ) );
  var exp = [];
  var got = src.map( null );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  /* - */

  test.open( 'only onEach' );

  test.case = 'from empty array, onEach returns number';
  var src = _.containerAdapter.make( new Set( [] ) );
  var exp = [];
  var got = src.map( ( e ) => 123 );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns original';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ];
  var got = src.map( ( e ) => e );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns key';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 0, 1, 2, 3, 4, 5, 6, 7, 8 ];
  var got = src.map( ( e, k ) => k );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns undefined';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ];
  var got = src.map( ( e ) => undefined );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns array';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ [ 0 ], [ 1 ], [ null ], [ true ], [ false ], [ undefined ], [ '' ], [ [ 2 ] ], [ { a : 0 } ] ];
  var got = src.map( ( e ) => [ e ] );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns element of array';
  var src = _.containerAdapter.make( new Set( [ 0, 1, NaN, true, false, [ undefined ], '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 0, 1, NaN, true, false, [ undefined ], '', 2, { a : 0 } ];
  var got = src.map( ( e ) => e[ 0 ] );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.close( 'only onEach' );

  /* - */

  test.open( 'dst === null' );

  test.case = 'from array, onEach returns container';
  var src = _.containerAdapter.make( new Set( [ 1 ] ) );
  var exp = [ 1 ];
  var got = src.map( null, ( e, k, src ) => src );
  test.true( got !== src );
  test.identical( Array.from( ... got.original ),  exp );

  test.case = 'from array, onEach returns original';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ];
  var got = src.map( null, ( e ) => e );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns key';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 0, 1, 2, 3, 4, 5, 6, 7, 8 ];
  var got = src.map( null, ( e, k ) => k );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns undefined';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ];
  var got = src.map( null, ( e ) => undefined );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns array';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ [ 0 ], [ 1 ], [ null ], [ true ], [ false ], [ undefined ], [ '' ], [ [ 2 ] ], [ { a : 0 } ] ];
  var got = src.map( null, ( e ) => [ e ] );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns element of array';
  var src = _.containerAdapter.make( new Set( [ 0, 1, NaN, true, false, [ undefined ], '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 0, 1, NaN, true, false, [ undefined ], '', 2, { a : 0 } ];
  var got = src.map( null, ( e ) => e[ 0 ] );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.close( 'dst === null' );

  /* - */

  test.open( 'dst === src' );

  test.case = 'from empty array, onEach returns container';
  var src = _.containerAdapter.make( new Set( [] ) );
  var exp = [];
  var got = src.map( src, ( e, k, src ) => src );
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns original';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ];
  var got = src.map( src, ( e ) => e );
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns key';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 0, 1, 2, 3, 4, 5, 6, 7, 8 ];
  var got = src.map( src, ( e, k ) => k );
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns undefined';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ];
  var got = src.map( src, ( e ) => undefined );
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns array';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ [ 0 ], [ 1 ], [ null ], [ true ], [ false ], [ undefined ], [ '' ], [ [ 2 ] ], [ { a : 0 } ] ];
  var got = src.map( src, ( e ) => [ e ] );
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns element of array';
  var src = _.containerAdapter.make( new Set( [ 0, 1, NaN, true, false, [ undefined ], '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 0, 1, NaN, true, false, [ undefined ], '', 2, { a : 0 } ];
  var got = src.map( src, ( e ) => e[ 0 ] );
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.close( 'dst === src' );
}

//

function setAdapterMapRight( test )
{
  test.case = 'without arguments';
  var src = _.containerAdapter.make( new Set( [] ) );
  var exp = [];
  var got = src.mapRight( null );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  /* - */

  test.open( 'only onEach' );

  test.case = 'from empty array, onEach returns number';
  var src = _.containerAdapter.make( new Set( [] ) );
  var exp = [];
  var got = src.mapRight( ( e ) => 123 );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns original';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ { a : 0 }, [ 2 ], '', undefined, false, true, null, 1, 0 ];
  var got = src.mapRight( ( e ) => e );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns key';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 8, 7, 6, 5, 4, 3, 2, 1, 0 ];
  var got = src.mapRight( ( e, k ) => k );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns undefined';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ { a : 0 }, [ 2 ], '', undefined, false, true, null, 1, 0 ];
  var got = src.mapRight( ( e ) => undefined );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns array';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ [ { a : 0 } ], [ [ 2 ] ], [ '' ], [ undefined ], [ false ], [ true ], [ null ], [ 1 ], [ 0 ] ];
  var got = src.mapRight( ( e ) => [ e ] );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns element of array';
  var src = _.containerAdapter.make( new Set( [ 0, 1, NaN, true, false, [ undefined ], '', [ 2 ], { a : 0 } ] ) );
  var exp = [ { a : 0 }, 2, '', [ undefined ], false, true, NaN, 1, 0 ];
  var got = src.mapRight( ( e ) => e[ 0 ] );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.close( 'only onEach' );

  /* - */

  test.open( 'dst === null' );

  test.case = 'from array, onEach returns container';
  var src = _.containerAdapter.make( new Set( [ 1 ] ) );
  var exp = [ 1 ];
  var got = src.mapRight( null, ( e, k, src ) => src );
  test.true( got !== src );
  test.identical( Array.from( ... got.original ),  exp );

  test.case = 'from array, onEach returns original';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ { a : 0 }, [ 2 ], '', undefined, false, true, null, 1, 0 ];
  var got = src.mapRight( null, ( e ) => e );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns key';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 8, 7, 6, 5, 4, 3, 2, 1, 0 ];
  var got = src.mapRight( null, ( e, k ) => k );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns undefined';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ { a : 0 }, [ 2 ], '', undefined, false, true, null, 1, 0 ];
  var got = src.mapRight( null, ( e ) => undefined );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns array';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ [ { a : 0 } ], [ [ 2 ] ], [ '' ], [ undefined ], [ false ], [ true ], [ null ], [ 1 ], [ 0 ] ];
  var got = src.mapRight( null, ( e ) => [ e ] );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns element of array';
  var src = _.containerAdapter.make( new Set( [ 0, 1, NaN, true, false, [ undefined ], '', [ 2 ], { a : 0 } ] ) );
  var exp = [ { a : 0 }, 2, '', [ undefined ], false, true, NaN, 1, 0 ];
  var got = src.mapRight( null, ( e ) => e[ 0 ] );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.close( 'dst === null' );

  /* - */

  test.open( 'dst === src' );

  test.case = 'from empty array, onEach returns container';
  var src = _.containerAdapter.make( new Set( [] ) );
  var exp = [];
  var got = src.mapRight( src, ( e, k, src ) => src );
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns original';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ];
  var got = src.mapRight( src, ( e ) => e );
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns key';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 0, 1, 2, 3, 4, 5, 6, 7, 8 ];
  var got = src.mapRight( src, ( e, k ) => k );
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns undefined';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ];
  var got = src.mapRight( src, ( e ) => undefined );
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns array';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ [ 0 ], [ 1 ], [ null ], [ true ], [ false ], [ undefined ], [ '' ], [ [ 2 ] ], [ { a : 0 } ] ];
  var got = src.mapRight( src, ( e ) => [ e ] );
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns element of array';
  var src = _.containerAdapter.make( new Set( [ 0, 1, NaN, true, false, [ undefined ], '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 0, 1, NaN, true, false, [ undefined ], '', 2, { a : 0 } ];
  var got = src.mapRight( src, ( e ) => e[ 0 ] );
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.close( 'dst === src' );
}

//

function setAdapterFilter( test )
{
  test.case = 'without arguments';
  var src = _.containerAdapter.make( new Set( [] ) );
  var exp = [];
  var got = src.filter( null );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  /* - */

  test.open( 'only onEach' );

  test.case = 'from empty array, onEach returns number';
  var src = _.containerAdapter.make( new Set( [] ) );
  var exp = [];
  var got = src.filter( ( e ) => 123 );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns original';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 0, 1, null, true, false, '', [ 2 ], { a : 0 } ];
  var got = src.filter( ( e ) => e );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns key';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 0, 1, 2, 3, 4, 5, 6, 7, 8 ];
  var got = src.filter( ( e, k ) => k );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns undefined';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [];
  var got = src.filter( ( e ) => undefined );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns array';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ [ 0 ], [ 1 ], [ null ], [ true ], [ false ], [ undefined ], [ '' ], [ [ 2 ] ], [ { a : 0 } ] ];
  var got = src.filter( ( e ) => [ e ] );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns element of array';
  var src = _.containerAdapter.make( new Set( [ 0, 1, NaN, true, false, [ undefined ], '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 2 ];
  var got = src.filter( ( e ) => e[ 0 ] );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.close( 'only onEach' );

  /* - */

  test.open( 'dst === null' );

  test.case = 'from array, onEach returns container';
  var src = _.containerAdapter.make( new Set( [ 1 ] ) );
  var exp = [ src ];
  var got = src.filter( null, ( e, k, src ) => src );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns original';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 0, 1, null, true, false, '', [ 2 ], { a : 0 } ];
  var got = src.filter( null, ( e ) => e );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns key';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 0, 1, 2, 3, 4, 5, 6, 7, 8 ];
  var got = src.filter( null, ( e, k ) => k );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns undefined';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [];
  var got = src.filter( null, ( e ) => undefined );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns array';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ [ 0 ], [ 1 ], [ null ], [ true ], [ false ], [ undefined ], [ '' ], [ [ 2 ] ], [ { a : 0 } ] ];
  var got = src.filter( null, ( e ) => [ e ] );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns element of array';
  var src = _.containerAdapter.make( new Set( [ 0, 1, NaN, true, false, [ undefined ], '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 2 ];
  var got = src.filter( null, ( e ) => e[ 0 ] );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.close( 'dst === null' );

  /* - */

  test.open( 'dst != src' );

  test.case = 'dst - array, from empty array, onEach returns container';
  var dst = [ 1, 2 ];
  var src = _.containerAdapter.make( new Set( [] ) );
  var exp = [ 1, 2 ];
  var got = src.filter( dst, ( e, k, src ) => src );
  test.true( got !== dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'dst - arrayAdapter, from array, onEach returns original';
  var dst = _.containerAdapter.make( [ 1, 2 ] );
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 1, 2, 0, 1, null, true, false, '', [ 2 ], { a : 0 } ];
  var got = src.filter( dst, ( e ) => e );
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'dst - arrayAdapter, from array, onEach returns key';
  var dst = _.containerAdapter.make( [ 1, 2 ] );
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 1, 2, 0, 1, 2, 3, 4, 5, 6, 7, 8 ];
  var got = src.filter( dst, ( e, k ) => k );
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'dst - Set, from array, onEach returns undefined';
  var dst = new Set( [ 1, 2 ] );
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 1, 2 ];
  var got = src.filter( dst, ( e ) => undefined );
  test.true( got.original === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'dst - setAdapter, from array, onEach returns array';
  var dst = _.containerAdapter.make( new Set( [ 1, 2 ] ) );
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 1, 2, [ 0 ], [ 1 ], [ null ], [ true ], [ false ], [ undefined ], [ '' ], [ [ 2 ] ], [ { a : 0 } ] ];
  var got = src.filter( dst, ( e ) => [ e ] );
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'dst - array, from array, onEach returns element of array';
  var dst = [ 1, 2 ];
  var src = _.containerAdapter.make( new Set( [ 0, 1, NaN, true, false, [ undefined ], '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 1, 2, 2 ];
  var got = src.filter( dst, ( e ) => e[ 0 ] );
  test.true( got.original === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.close( 'dst != src' );

  /* - */

  test.open( 'dst === src' );

  test.case = 'from empty array, onEach returns container';
  var src = _.containerAdapter.make( new Set( [] ) );
  var exp = [];
  var got = src.filter( src, ( e, k, src ) => src );
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns original';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 0, 1, null, true, false, '', [ 2 ], { a : 0 } ];
  var got = src.filter( src, ( e ) => e );
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns key';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 0, 1, 2, 3, 4, 5, 6, 7, 8 ];
  var got = src.filter( src, ( e, k ) => k );
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns undefined';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [];
  var got = src.filter( src, ( e ) => undefined );
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns array';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ [ 0 ], [ 1 ], [ null ], [ true ], [ false ], [ undefined ], [ '' ], [ [ 2 ] ], [ { a : 0 } ] ];
  var got = src.filter( src, ( e ) => [ e ] );
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns element of array';
  var src = _.containerAdapter.make( new Set( [ 0, 1, NaN, true, false, [ undefined ], '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 2 ];
  var got = src.filter( src, ( e ) => e[ 0 ] );
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.close( 'dst === src' );
}

//

function setAdapterFilterRight( test )
{
  test.case = 'without arguments';
  var src = _.containerAdapter.make( new Set( [] ) );
  var exp = [];
  var got = src.filterRight( null );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  /* - */

  test.open( 'only onEach' );

  test.case = 'from empty array, onEach returns number';
  var src = _.containerAdapter.make( new Set( [] ) );
  var exp = [];
  var got = src.filterRight( ( e ) => 123 );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns original';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ { a : 0 }, [ 2 ], '', false, true, null, 1, 0 ];
  var got = src.filterRight( ( e ) => e );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns key';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 8, 7, 6, 5, 4, 3, 2, 1, 0 ];
  var got = src.filterRight( ( e, k ) => k );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns undefined';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [];
  var got = src.filterRight( ( e ) => undefined );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns array';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ [ { a : 0 } ], [ [ 2 ] ], [ '' ], [ undefined ], [ false ], [ true ], [ null ], [ 1 ], [ 0 ] ];
  var got = src.filterRight( ( e ) => [ e ] );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns element of array';
  var src = _.containerAdapter.make( new Set( [ 0, 1, NaN, true, false, [ undefined ], '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 2 ];
  var got = src.filterRight( ( e ) => e[ 0 ] );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.close( 'only onEach' );

  /* - */

  test.open( 'dst === null' );

  test.case = 'from array, onEach returns container';
  var src = _.containerAdapter.make( new Set( [ 1 ] ) );
  var exp = [ src ];
  var got = src.filterRight( null, ( e, k, src ) => src );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns original';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ { a : 0 }, [ 2 ], '', false, true, null, 1, 0 ];
  var got = src.filterRight( null, ( e ) => e );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns key';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 8, 7, 6, 5, 4, 3, 2, 1, 0 ];
  var got = src.filterRight( null, ( e, k ) => k );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns undefined';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [];
  var got = src.filterRight( null, ( e ) => undefined );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns array';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ [ { a : 0 } ], [ [ 2 ] ], [ '' ], [ undefined ], [ false ], [ true ], [ null ], [ 1 ], [ 0 ] ];
  var got = src.filterRight( null, ( e ) => [ e ] );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns element of array';
  var src = _.containerAdapter.make( new Set( [ 0, 1, NaN, true, false, [ undefined ], '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 2 ];
  var got = src.filterRight( null, ( e ) => e[ 0 ] );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.close( 'dst === null' );

  /* - */

  test.open( 'dst != src' );

  test.case = 'dst - array, from empty array, onEach returns container';
  var dst = [ 1, 2 ];
  var src = _.containerAdapter.make( new Set( [] ) );
  var exp = [ 1, 2 ];
  var got = src.filterRight( dst, ( e, k, src ) => src );
  test.true( got !== dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'dst - arrayAdapter, from array, onEach returns original';
  var dst = _.containerAdapter.make( [ 1, 2 ] );
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 1, 2, { a : 0 }, [ 2 ], '', false, true, null, 1, 0 ];
  var got = src.filterRight( dst, ( e ) => e );
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'dst - arrayAdapter, from array, onEach returns key';
  var dst = _.containerAdapter.make( [ 1, 2 ] );
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 1, 2, 8, 7, 6, 5, 4, 3, 2, 1, 0 ];
  var got = src.filterRight( dst, ( e, k ) => k );
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'dst - Set, from array, onEach returns undefined';
  var dst = new Set( [ 1, 2 ] );
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 1, 2 ];
  var got = src.filterRight( dst, ( e ) => undefined );
  test.true( got.original === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'dst - setAdapter, from array, onEach returns array';
  var dst = _.containerAdapter.make( new Set( [ 1, 2 ] ) );
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 1, 2, [ { a : 0 } ], [ [ 2 ] ], [ '' ], [ undefined ], [ false ], [ true ], [ null ], [ 1 ], [ 0 ] ];
  var got = src.filterRight( dst, ( e ) => [ e ] );
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'dst - array, from array, onEach returns element of array';
  var dst = [ 1, 2 ];
  var src = _.containerAdapter.make( new Set( [ 0, 1, NaN, true, false, [ undefined ], '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 1, 2, 2 ];
  var got = src.filterRight( dst, ( e ) => e[ 0 ] );
  test.true( got.original === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.close( 'dst != src' );

  /* - */

  test.open( 'dst === src' );

  test.case = 'from empty array, onEach returns container';
  var src = _.containerAdapter.make( new Set( [] ) );
  var exp = [];
  var got = src.filterRight( src, ( e, k, src ) => src );
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns original';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 0, 1, null, true, false, '', [ 2 ], { a : 0 } ];
  var got = src.filterRight( src, ( e ) => e );
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns key';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 0, 1, 2, 3, 4, 5, 6, 7, 8 ];
  var got = src.filterRight( src, ( e, k ) => k );
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns undefined';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [];
  var got = src.filterRight( src, ( e ) => undefined );
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns array';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ [ 0 ], [ 1 ], [ null ], [ true ], [ false ], [ undefined ], [ '' ], [ [ 2 ] ], [ { a : 0 } ] ];
  var got = src.filterRight( src, ( e ) => [ e ] );
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns element of array';
  var src = _.containerAdapter.make( new Set( [ 0, 1, NaN, true, false, [ undefined ], '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 2 ];
  var got = src.filterRight( src, ( e ) => e[ 0 ] );
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.close( 'dst === src' );
}

//

function setAdapterFlatFilter( test )
{
  test.case = 'without arguments';
  var src = _.containerAdapter.make( new Set( [] ) );
  var exp = [];
  var got = src.flatFilter( null );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  /* - */

  test.open( 'only onEach' );

  test.case = 'from empty array, onEach returns number';
  var src = _.containerAdapter.make( new Set( [] ) );
  var exp = [];
  var got = src.flatFilter( ( e ) => 123 );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns original';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 0, 1, null, true, false, '', 2, { a : 0 } ];
  var got = src.flatFilter( ( e ) => e );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns key';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 0, 1, 2, 3, 4, 5, 6, 7, 8 ];
  var got = src.flatFilter( ( e, k ) => k );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns undefined';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [];
  var got = src.flatFilter( ( e ) => undefined );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns array';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 0, '0', 1, '1', null, 'null', true, 'true', false, 'false', undefined, 'undefined', '', [ 2 ], '2', { a : 0 }, '[object Object]' ];
  var got = src.flatFilter( ( e ) => [ e, String( e ) ] );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns element of array';
  var src = _.containerAdapter.make( new Set( [ 0, 1, NaN, true, false, [ undefined ], '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 2 ];
  var got = src.flatFilter( ( e ) => e[ 0 ] );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.close( 'only onEach' );

  /* - */

  test.open( 'dst === null' );

  test.case = 'from array, onEach returns container';
  var src = _.containerAdapter.make( new Set( [ 1 ] ) );
  var exp = [ ... src.original ];
  var got = src.flatFilter( null, ( e, k, src ) => src );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns original';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 0, 1, null, true, false, '', 2, { a : 0 } ];
  var got = src.flatFilter( null, ( e ) => e );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns key';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 0, 1, 2, 3, 4, 5, 6, 7, 8 ];
  var got = src.flatFilter( null, ( e, k ) => k );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns undefined';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [];
  var got = src.flatFilter( null, ( e ) => undefined );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns array';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ];
  var got = src.flatFilter( null, ( e ) => [ e ] );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns element of array';
  var src = _.containerAdapter.make( new Set( [ 0, 1, NaN, true, false, [ undefined ], '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 2 ];
  var got = src.flatFilter( null, ( e ) => e[ 0 ] );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.close( 'dst === null' );

  /* - */

  test.open( 'dst != src' );

  test.case = 'dst - array, from empty array, onEach returns container';
  var dst = [ 1, 2 ];
  var src = _.containerAdapter.make( new Set( [] ) );
  var exp = [ 1, 2 ];
  var got = src.flatFilter( dst, ( e, k, src ) => src );
  test.true( got !== dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'dst - arrayAdapter, from array, onEach returns original';
  var dst = _.containerAdapter.make( [ 1, 2 ] );
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 1, 2, 0, 1, null, true, false, '', 2, { a : 0 } ];
  var got = src.flatFilter( dst, ( e ) => e );
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'dst - arrayAdapter, from array, onEach returns key';
  var dst = _.containerAdapter.make( [ 1, 2 ] );
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 1, 2, 0, 1, 2, 3, 4, 5, 6, 7, 8 ];
  var got = src.flatFilter( dst, ( e, k ) => k );
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'dst - Set, from array, onEach returns undefined';
  var dst = new Set( [ 1, 2 ] );
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 1, 2 ];
  var got = src.flatFilter( dst, ( e ) => undefined );
  test.true( got.original === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'dst - setAdapter, from array, onEach returns array';
  var dst = _.containerAdapter.make( new Set( [ 1, 2 ] ) );
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 1, 2, 0, null, true, false, undefined, '', [ 2 ], { a : 0 } ];
  var got = src.flatFilter( dst, ( e ) => [ e ] );
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'dst - array, from array, onEach returns element of array';
  var dst = [ 1, 2 ];
  var src = _.containerAdapter.make( new Set( [ 0, 1, NaN, true, false, [ undefined ], '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 1, 2, 2 ];
  var got = src.flatFilter( dst, ( e ) => e[ 0 ] );
  test.true( got.original === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.close( 'dst != src' );

  /* - */

  test.open( 'dst === src' );

  test.case = 'from empty array, onEach returns container';
  var src = _.containerAdapter.make( new Set( [] ) );
  var exp = [];
  var got = src.flatFilter( src, ( e, k, src ) => src );
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns original';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 0, 1, null, true, false, '', 2, { a : 0 } ];
  var got = src.flatFilter( src, ( e ) => e );
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns key';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 0, 1, 2, 3, 4, 5, 6, 7, 8 ];
  var got = src.flatFilter( src, ( e, k ) => k );
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns undefined';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [];
  var got = src.flatFilter( src, ( e ) => undefined );
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns array';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ];
  var got = src.flatFilter( src, ( e ) => [ e ] );
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns element of array';
  var src = _.containerAdapter.make( new Set( [ 0, 1, NaN, true, false, [ undefined ], '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 2 ];
  var got = src.flatFilter( src, ( e ) => e[ 0 ] );
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.close( 'dst === src' );
}

//

function setAdapterFlatFilterRight( test )
{
  test.case = 'without arguments';
  var src = _.containerAdapter.make( new Set( [] ) );
  var exp = [];
  var got = src.flatFilterRight( null );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  /* - */

  test.open( 'only onEach' );

  test.case = 'from empty array, onEach returns number';
  var src = _.containerAdapter.make( new Set( [] ) );
  var exp = [];
  var got = src.flatFilterRight( ( e ) => 123 );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns original';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ { a : 0 }, 2, '', false, true, null, 1, 0 ];
  var got = src.flatFilterRight( ( e ) => e );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns key';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 8, 7, 6, 5, 4, 3, 2, 1, 0 ];
  var got = src.flatFilterRight( ( e, k ) => k );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns undefined';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [];
  var got = src.flatFilterRight( ( e ) => undefined );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns array';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ { a : 0 }, '[object Object]', [ 2 ], '2', '', undefined, 'undefined', false, 'false', true, 'true', null, 'null', 1, '1', 0, '0' ];
  var got = src.flatFilterRight( ( e ) => [ e, String( e ) ] );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns element of array';
  var src = _.containerAdapter.make( new Set( [ 0, 1, NaN, true, false, [ undefined ], '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 2 ];
  var got = src.flatFilterRight( ( e ) => e[ 0 ] );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.close( 'only onEach' );

  /* - */

  test.open( 'dst === null' );

  test.case = 'from array, onEach returns container';
  var src = _.containerAdapter.make( new Set( [ 1 ] ) );
  var exp = [ ... src.original ];
  var got = src.flatFilterRight( null, ( e, k, src ) => src );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns original';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ { a : 0 }, 2, '', false, true, null, 1, 0 ];
  var got = src.flatFilterRight( null, ( e ) => e );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns key';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 8, 7, 6, 5, 4, 3, 2, 1, 0 ];
  var got = src.flatFilterRight( null, ( e, k ) => k );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns undefined';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [];
  var got = src.flatFilterRight( null, ( e ) => undefined );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns array';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ { a : 0 }, [ 2 ], '', undefined, false, true, null, 1, 0 ];
  var got = src.flatFilterRight( null, ( e ) => [ e ] );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns element of array';
  var src = _.containerAdapter.make( new Set( [ 0, 1, NaN, true, false, [ undefined ], '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 2 ];
  var got = src.flatFilterRight( null, ( e ) => e[ 0 ] );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.close( 'dst === null' );

  /* - */

  test.open( 'dst != src' );

  test.case = 'dst - array, from empty array, onEach returns container';
  var dst = [ 1, 2 ];
  var src = _.containerAdapter.make( new Set( [] ) );
  var exp = [ 1, 2 ];
  var got = src.flatFilterRight( dst, ( e, k, src ) => src );
  test.true( got !== dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'dst - arrayAdapter, from array, onEach returns original';
  var dst = _.containerAdapter.make( [ 1, 2 ] );
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 1, 2, { a : 0 }, 2, '', false, true, null, 1, 0 ];
  var got = src.flatFilterRight( dst, ( e ) => e );
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'dst - arrayAdapter, from array, onEach returns key';
  var dst = _.containerAdapter.make( [ 1, 2 ] );
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 1, 2, 8, 7, 6, 5, 4, 3, 2, 1, 0 ];
  var got = src.flatFilterRight( dst, ( e, k ) => k );
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'dst - Set, from array, onEach returns undefined';
  var dst = new Set( [ 1, 2 ] );
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 1, 2 ];
  var got = src.flatFilterRight( dst, ( e ) => undefined );
  test.true( got.original === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'dst - setAdapter, from array, onEach returns array';
  var dst = _.containerAdapter.make( new Set( [ 1, 2 ] ) );
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 1, 2, { a : 0 }, [ 2 ], '', undefined, false, true, null, 0 ];
  var got = src.flatFilterRight( dst, ( e ) => [ e ] );
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'dst - array, from array, onEach returns element of array';
  var dst = [ 1, 2 ];
  var src = _.containerAdapter.make( new Set( [ 0, 1, NaN, true, false, [ undefined ], '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 1, 2, 2 ];
  var got = src.flatFilterRight( dst, ( e ) => e[ 0 ] );
  test.true( got.original === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.close( 'dst != src' );

  /* - */

  test.open( 'dst === src' );

  test.case = 'from empty array, onEach returns container';
  var src = _.containerAdapter.make( new Set( [] ) );
  var exp = [];
  var got = src.flatFilterRight( src, ( e, k, src ) => src );
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns original';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 0, 1, null, true, false, '', 2, { a : 0 } ];
  var got = src.flatFilterRight( src, ( e ) => e );
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns key';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 0, 1, 2, 3, 4, 5, 6, 7, 8 ];
  var got = src.flatFilterRight( src, ( e, k ) => k );
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns undefined';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [];
  var got = src.flatFilterRight( src, ( e ) => undefined );
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns array';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ];
  var got = src.flatFilterRight( src, ( e ) => [ e ] );
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.case = 'from array, onEach returns element of array';
  var src = _.containerAdapter.make( new Set( [ 0, 1, NaN, true, false, [ undefined ], '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 2 ];
  var got = src.flatFilterRight( src, ( e ) => e[ 0 ] );
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.close( 'dst === src' );
}

//

function setAdapterOnce( test )
{
  test.case = 'without arguments';
  var src = _.containerAdapter.make( new Set( [] ) );
  var exp = [];
  var got = src.once( null );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  /* - */

  test.open( 'only onEval' );

  test.case = 'evaluator returns element';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ];
  var got = src.once( ( e ) => e );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'two evaluators returns element';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ];
  var got = src.once( ( e ) => e, ( ins ) => ins );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'equalizer';
  var src = _.containerAdapter.make( new Set( [ 0, 0, 1, 1, null, true, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ];
  var got = src.once( ( e, e2 ) => e === e2 );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'evaluator returns undefined';
  var src = _.containerAdapter.make( new Set( [ 0, 0, 1, 1, undefined, undefined, undefined, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 0 ];
  var got = src.once( ( e ) => undefined );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'evaluator checks element of array';
  var src = _.containerAdapter.make( new Set( [ 0, 1, [ null ], [ true ], false, [ undefined ], '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 0, [ null ], [ true ], [ 2 ] ];
  var got = src.once( ( e ) => e[ 0 ] );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.close( 'only onEval' );

  /* - */

  test.open( 'only dst' );

  test.case = 'src - empty container, dst - empty array, no onEval';
  var src = _.containerAdapter.make( new Set( [] ) );
  var dst = [];
  var exp = [];
  var got = src.once( dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'src - empty container, dst - empty arrayAdapter, no onEval';
  var src = _.containerAdapter.make( new Set( [] ) );
  var dst = _.containerAdapter.make( [] );
  var exp = [];
  var got = src.once( dst );
  test.true( got !== src );
  test.true( got === dst );
  test.identical( got.original, exp );

  test.case = 'src - empty container, dst - filled array, no onEval';
  var src = _.containerAdapter.make( new Set( [] ) );
  var dst = [ 1, 2, 1, 2, null, null, undefined, false, undefined, false ];
  var exp = [ 1, 2, 1, 2, null, null, undefined, false, undefined, false ];
  var got = src.once( dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'src - filled container, dst - empty array, no onEval';
  var src = _.containerAdapter.make( new Set( [ 1, 2, 1, 2, null, null, undefined, false, undefined, false ] ) );
  var dst = [];
  var exp = [ 1, 2, null, undefined, false ];
  var got = src.once( dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'src - filled container, dst - filled array, no onEval';
  var src = _.containerAdapter.make( new Set( [ 1, 2, 1, 2, null, null, undefined, false, undefined, false, [ 2 ], [ 2 ], [ 2 ] ] ) );
  var dst = [ 1, 2, null, undefined, false ];
  var exp = [ 1, 2, null, undefined, false, [ 2 ], [ 2 ], [ 2 ] ];
  var got = src.once( dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'src - filled container, dst - filled array, same content, no onEval';
  var src = _.containerAdapter.make( new Set( [ 1, 2, 1, 2, null, null, undefined, false, undefined, false ] ) );
  var dst = [ 1, 2, 1, 2, null, null, undefined, false, undefined, false ];
  var exp = [ 1, 2, 1, 2, null, null, undefined, false, undefined, false ];
  var got = src.once( dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'src - filled container, dst - filled arrayAdapter, same content, no onEval, evaluator returns element';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var dst = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 }, [ 2 ], { a : 0 } ];
  var got = src.once( dst, ( e ) => e );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'src - filled container, dst - filled arrayAdapter, same content, equalizer';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var dst = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 }, new U8x( 2 ) ] );
  var exp = [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 }, new U8x( 2 ), [ 2 ], { a : 0 } ];
  var got = src.once( dst, ( e, e2 ) => e === e2 );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'src - filled container, dst - filled arrayAdapter, onEval returns undefined';
  var src = _.containerAdapter.make( new Set( [ 0, 0, 1, 1, undefined, undefined, undefined, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var dst = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ];
  var got = src.once( dst, ( e ) => undefined );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'src - filled container, dst - filled array, evaluator checks element of array';
  var src = _.containerAdapter.make( new Set( [ 0, 1, [ null ], [ true ], false, [ undefined ], '', [ 2 ], { a : 0 } ] ) );
  var dst = _.containerAdapter.make( [ 0, 1, [ null ], [ true ], false, [ undefined ], '', [ 2 ], { a : 0 } ] );
  var exp = [ 0, 1, [ null ], [ true ], false, [ undefined ], '', [ 2 ], { a : 0 } ];
  var got = src.once( dst, ( e ) => e[ 0 ] );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'src - filled container, dst - filled array, duplicates in src, evaluator checks element of array';
  var src = _.containerAdapter.make( new Set( [ 0, 1, [ null ], [ true ], [ true ], [ true ], false, [ undefined ], '', [ 2 ], { a : 0 } ] ) );
  var dst = _.containerAdapter.make( [ 0, 1, [ null ], [ true ], false, [ undefined ], '', [ 2 ], { a : 0 } ] );
  var exp = [ 0, 1, [ null ], [ true ], false, [ undefined ], '', [ 2 ], { a : 0 } ];
  var got = src.once( dst, ( e ) => e[ 0 ] );
  test.true( got !== src );
  test.identical( got.original, exp );

  /* */

  test.case = 'src - empty container, dst - empty Set, no onEval';
  var src = _.containerAdapter.make( new Set( [] ) );
  var dst = new Set( [] );
  var exp = [];
  var got = src.once( dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - empty container, dst - empty ContainerAdapterSet, no onEval';
  var src = _.containerAdapter.make( new Set( [] ) );
  var dst = _.containerAdapter.make( new Set( [] ) );
  var exp = [];
  var got = src.once( dst );
  test.true( got !== src );
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - empty container, dst - filled Set, no onEval';
  var src = _.containerAdapter.make( new Set( [] ) );
  var dst = new Set( [ 1, 2, 1, 2, null, null, undefined, false, undefined, false ] );
  var exp = [ 1, 2, null, undefined, false ];
  var got = src.once( dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - filled container, dst - empty Set, no onEval';
  var src = _.containerAdapter.make( new Set( [ 1, 2, 1, 2, null, null, undefined, false, undefined, false ] ) );
  var dst = new Set( [] );
  var exp = [ 1, 2, null, undefined, false ];
  var got = src.once( dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - filled container, dst - filled Set, no onEval';
  var src = _.containerAdapter.make( new Set( [ 1, 2, 1, 2, null, null, undefined, false, undefined, false, [ 2 ], [ 2 ], [ 2 ] ] ) );
  var dst = new Set( [ 1, 2, null, undefined, false ] );
  var exp = [ 1, 2, null, undefined, false, [ 2 ], [ 2 ], [ 2 ] ];
  var got = src.once( dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - filled container, dst - filled Set, same content, no onEval';
  var src = _.containerAdapter.make( new Set( [ 1, 2, 1, 2, null, null, undefined, false, undefined, false ] ) );
  var dst = new Set( [ 1, 2, 1, 2, null, null, undefined, false, undefined, false ] );
  var exp = [ 1, 2, null, undefined, false ];
  var got = src.once( dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - filled container, dst - filled setAdapter, same content, one evaluator';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var dst = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 }, [ 2 ], { a : 0 } ];
  var got = src.once( dst, ( e ) => e );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - filled container, dst - filled setAdapter, same content, equalizer';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var dst = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 }, new U8x( 2 ) ] ) );
  var exp = [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 }, new U8x( 2 ), [ 2 ], { a : 0 } ];
  var got = src.once( dst, ( e, e2 ) => e === e2 );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - filled container, dst - filled setAdapter, evaluator returns undefined';
  var src = _.containerAdapter.make( new Set( [ 0, 0, 1, 1, undefined, undefined, undefined, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var dst = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ];
  var got = src.once( dst, ( e ) => undefined );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - filled container, dst - filled setAdapter, evaluator checks element of array';
  var src = _.containerAdapter.make( new Set( [ 0, 1, [ null ], [ true ], false, [ undefined ], '', [ 2 ], { a : 0 } ] ) );
  var dst = _.containerAdapter.make( new Set( [ 0, 1, [ null ], [ true ], false, [ undefined ], '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 0, 1, [ null ], [ true ], false, [ undefined ], '', [ 2 ], { a : 0 } ];
  var got = src.once( dst, ( e ) => e[ 0 ] );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - filled container, dst - filled setAdapter, duplicates in src, evaluator checks element of array';
  var src = _.containerAdapter.make( new Set( [ 0, 1, [ null ], [ true ], [ true ], [ true ], false, [ undefined ], '', [ 2 ], { a : 0 } ] ) );
  var dst = _.containerAdapter.make( new Set( [ 0, 1, [ null ], [ true ], false, [ undefined ], '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 0, 1, [ null ], [ true ], false, [ undefined ], '', [ 2 ], { a : 0 } ];
  var got = src.once( dst, ( e ) => e[ 0 ] );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - filled container, dst - filled setAdapter, duplicates in src, two evaluators checks element of array';
  var src = _.containerAdapter.make( new Set( [ 0, 1, [ null ], [ true ], [ true ], [ true ], false, [ undefined ], '', [ 2 ], { a : 0 } ] ) );
  var dst = _.containerAdapter.make( new Set( [ 0, 1, [ null ], [ true ], false, [ undefined ], '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 0, 1, [ null ], [ true ], false, [ undefined ], '', [ 2 ], { a : 0 } ];
  var got = src.once( dst, ( e ) => e[ 0 ], ( ins ) => ins[ 0 ] );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.close( 'only dst' );

  /* - */

  test.open( 'dst === null' );

  test.case = 'src - empty container, no onEval';
  var src = _.containerAdapter.make( new Set( [] ) );
  var got = src.once( null );
  var exp = [];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - filled container, no onEval';
  var src = _.containerAdapter.make( new Set( [ 1, 2, 1, 2, null, null, undefined, false, undefined, false ] ) );
  var got = src.once( null );
  var exp = [ 1, 2, null, undefined, false ];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - filled container, complex data, no onEval';
  var src = _.containerAdapter.make( new Set( [ 1, 2, 1, 2, null, null, undefined, false, undefined, false, [ 2 ], [ 2 ], [ 2 ] ] ) );
  var got = src.once( null );
  var exp = [ 1, 2, null, undefined, false, [ 2 ], [ 2 ], [ 2 ] ];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - filled container, evaluator returns element';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var got = src.once( null, ( e ) => e );
  var exp = [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - filled container, two evaluators returns element';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var got = src.once( null, ( e ) => e, ( ins ) => ins );
  var exp = [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - filled container, equalizer';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var got = src.once( null, ( e, e2 ) => e === e2 );
  var exp = [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - filled container, evaluator returns undefined';
  var src = _.containerAdapter.make( new Set( [ 0, 0, 1, 1, undefined, undefined, undefined, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var got = src.once( null, ( e ) => undefined );
  var exp = [ 0 ];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - filled container, evaluator checks element of array, no duplicates in src';
  var src = _.containerAdapter.make( new Set( [ 0, 1, [ null ], [ true ], false, [ undefined ], '', [ 2 ], { a : 0 } ] ) );
  var got = src.once( null, ( e ) => e[ 0 ] );
  var exp = [ 0, [ null ], [ true ], [ 2 ] ];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - filled container, evaluator checks element of array, duplicates in src';
  var src = _.containerAdapter.make( new Set( [ 0, 1, [ null ], [ true ], [ true ], [ true ], false, [ undefined ], '', [ 2 ], { a : 0 } ] ) );
  var got = src.once( null, ( e ) => e[ 0 ] );
  var exp = [ 0, [ null ], [ true ], [ 2 ] ];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - filled container, two evaluators checks element of array, duplicates in src';
  var src = _.containerAdapter.make( new Set( [ 0, 1, [ null ], [ true ], [ true ], [ true ], false, [ undefined ], '', [ 2 ], { a : 0 } ] ) );
  var got = src.once( null, ( e ) => e[ 0 ], ( ins ) => ins[ 0 ] );
  var exp = [ 0, [ null ], [ true ], [ 2 ] ];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.close( 'dst === null' );

  /* - */

  test.open( 'dst === src' );

  test.case = 'src - empty container, no onEval';
  var src = _.containerAdapter.make( new Set( [] ) );
  var got = src.once( src );
  var exp = [];
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - filled container, no onEval';
  var src = _.containerAdapter.make( new Set( [ 1, 2, 1, 2, null, null, undefined, false, undefined, false ] ) );
  var got = src.once( src );
  var exp = [ 1, 2, null, undefined, false ];
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - filled container, complex data, no onEval';
  var src = _.containerAdapter.make( new Set( [ 1, 2, 1, 2, null, null, undefined, false, undefined, false, [ 2 ], [ 2 ], [ 2 ] ] ) );
  var got = src.once( src );
  var exp = [ 1, 2, null, undefined, false, [ 2 ], [ 2 ], [ 2 ] ];
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - filled container, complex data, evaluator returns element';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var got = src.once( src, ( e ) => e );
  var exp = [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ];
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - filled container, complex data, equalizer';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var got = src.once( src, ( e, e2 ) => e === e2 );
  var exp = [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ];
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - filled container, complex data, evaluator returns undefined';
  var src = _.containerAdapter.make( new Set( [ 0, 0, 1, 1, undefined, undefined, undefined, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var got = src.once( src, ( e ) => undefined );
  var exp = [ 0 ];
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - filled container, complex data, evaluator checks element of array, no duplicates in src';
  var src = _.containerAdapter.make( new Set( [ 0, 1, [ null ], [ true ], false, [ undefined ], '', [ 2 ], { a : 0 } ] ) );
  var got = src.once( src, ( e ) => e[ 0 ] );
  var exp = [ 0, [ null ], [ true ], [ 2 ] ];
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - filled container, complex data, two evaluators checks element of array, no duplicates in src';
  var src = _.containerAdapter.make( new Set( [ 0, 1, [ null ], [ true ], false, [ undefined ], '', [ 2 ], { a : 0 } ] ) );
  var got = src.once( src, ( e ) => e[ 0 ] );
  var exp = [ 0, [ null ], [ true ], [ 2 ] ];
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.close( 'dst === src' );
}

//

function setAdapterOnceRight( test )
{
  test.case = 'without arguments';
  var src = _.containerAdapter.make( new Set( [] ) );
  var exp = [];
  var got = src.onceRight( null );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  /* - */

  test.open( 'only onEval' );

  test.case = 'evaluator returns element';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ { a : 0 }, [ 2 ], '', undefined, false, true, null, 1, 0 ];
  var got = src.onceRight( ( e ) => e );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'two evaluators returns element';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ { a : 0 }, [ 2 ], '', undefined, false, true, null, 1, 0 ];
  var got = src.onceRight( ( e ) => e, ( ins ) => ins );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'equalizer';
  var src = _.containerAdapter.make( new Set( [ 0, 0, 1, 1, null, true, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ { a : 0 }, [ 2 ], '', undefined, false, true, null, 1, 0 ];
  var got = src.onceRight( ( e, e2 ) => e === e2 );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'evaluator returns undefined';
  var src = _.containerAdapter.make( new Set( [ 0, 0, 1, 1, undefined, undefined, undefined, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ { a : 0 } ];
  var got = src.onceRight( ( e ) => undefined );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'evaluator checks element of array';
  var src = _.containerAdapter.make( new Set( [ 0, 1, [ null ], [ true ], false, [ undefined ], '', [ 2 ], { a : 0 } ] ) );
  var exp = [ { a : 0 }, [ 2 ], [ true ], [ null ] ];
  var got = src.onceRight( ( e ) => e[ 0 ] );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.close( 'only onEval' );

  /* - */

  test.open( 'only dst' );

  test.case = 'src - empty container, dst - empty array, no onEval';
  var src = _.containerAdapter.make( new Set( [] ) );
  var dst = [];
  var exp = [];
  var got = src.onceRight( dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'src - empty container, dst - empty arrayAdapter, no onEval';
  var src = _.containerAdapter.make( new Set( [] ) );
  var dst = _.containerAdapter.make( [] );
  var exp = [];
  var got = src.onceRight( dst );
  test.true( got !== src );
  test.true( got === dst );
  test.identical( got.original, exp );

  test.case = 'src - empty container, dst - filled array, no onEval';
  var src = _.containerAdapter.make( new Set( [] ) );
  var dst = [ 1, 2, 1, 2, null, null, undefined, false, undefined, false ];
  var exp = [ 1, 2, 1, 2, null, null, undefined, false, undefined, false ];
  var got = src.onceRight( dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'src - filled container, dst - empty array, no onEval';
  var src = _.containerAdapter.make( new Set( [ 1, 2, 1, 2, null, null, undefined, false, undefined, false ] ) );
  var dst = [];
  var exp = [ false, undefined, null, 2, 1 ];
  var got = src.onceRight( dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'src - filled container, dst - filled array, no onEval';
  var src = _.containerAdapter.make( new Set( [ 1, 2, 1, 2, null, null, undefined, false, undefined, false, [ 2 ], [ 2 ], [ 2 ] ] ) );
  var dst = [ 1, 2, null, undefined, false ];
  var exp = [ 1, 2, null, undefined, false, [ 2 ], [ 2 ], [ 2 ] ];
  var got = src.onceRight( dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'src - filled container, dst - filled array, same content, no onEval';
  var src = _.containerAdapter.make( new Set( [ 1, 2, 1, 2, null, null, undefined, false, undefined, false ] ) );
  var dst = [ 1, 2, 1, 2, null, null, undefined, false, undefined, false ];
  var exp = [ 1, 2, 1, 2, null, null, undefined, false, undefined, false ];
  var got = src.onceRight( dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'src - filled container, dst - filled arrayAdapter, same content, no onEval, evaluator returns element';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var dst = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 }, { a : 0 }, [ 2 ] ];
  var got = src.onceRight( dst, ( e ) => e );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'src - filled container, dst - filled arrayAdapter, same content, equalizer';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var dst = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 }, new U8x( 2 ) ] );
  var exp = [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 }, new U8x( 2 ), { a : 0 }, [ 2 ] ];
  var got = src.onceRight( dst, ( e, e2 ) => e === e2 );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'src - filled container, dst - filled arrayAdapter, onEval returns undefined';
  var src = _.containerAdapter.make( new Set( [ 0, 0, 1, 1, undefined, undefined, undefined, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var dst = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ];
  var got = src.onceRight( dst, ( e ) => undefined );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'src - filled container, dst - filled array, evaluator checks element of array';
  var src = _.containerAdapter.make( new Set( [ 0, 1, [ null ], [ true ], false, [ undefined ], '', [ 2 ], { a : 0 } ] ) );
  var dst = _.containerAdapter.make( [ 0, 1, [ null ], [ true ], false, [ undefined ], '', [ 2 ], { a : 0 } ] );
  var exp = [ 0, 1, [ null ], [ true ], false, [ undefined ], '', [ 2 ], { a : 0 } ];
  var got = src.onceRight( dst, ( e ) => e[ 0 ] );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'src - filled container, dst - filled array, duplicates in src, evaluator checks element of array';
  var src = _.containerAdapter.make( new Set( [ 0, 1, [ null ], [ true ], [ true ], [ true ], false, [ undefined ], '', [ 2 ], { a : 0 } ] ) );
  var dst = _.containerAdapter.make( [ 0, 1, [ null ], [ true ], false, [ undefined ], '', [ 2 ], { a : 0 } ] );
  var exp = [ 0, 1, [ null ], [ true ], false, [ undefined ], '', [ 2 ], { a : 0 } ];
  var got = src.onceRight( dst, ( e ) => e[ 0 ] );
  test.true( got !== src );
  test.identical( got.original, exp );

  /* */

  test.case = 'src - empty container, dst - empty Set, no onEval';
  var src = _.containerAdapter.make( new Set( [] ) );
  var dst = new Set( [] );
  var exp = [];
  var got = src.onceRight( dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - empty container, dst - empty ContainerAdapterSet, no onEval';
  var src = _.containerAdapter.make( new Set( [] ) );
  var dst = _.containerAdapter.make( new Set( [] ) );
  var exp = [];
  var got = src.onceRight( dst );
  test.true( got !== src );
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - empty container, dst - filled Set, no onEval';
  var src = _.containerAdapter.make( new Set( [] ) );
  var dst = new Set( [ 1, 2, 1, 2, null, null, undefined, false, undefined, false ] );
  var exp = [ 1, 2, null, undefined, false ];
  var got = src.onceRight( dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - filled container, dst - empty Set, no onEval';
  var src = _.containerAdapter.make( new Set( [ 1, 2, 1, 2, null, null, undefined, false, undefined, false ] ) );
  var dst = new Set( [] );
  var exp = [ false, undefined, null, 2, 1 ];
  var got = src.onceRight( dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - filled container, dst - filled Set, no onEval';
  var src = _.containerAdapter.make( new Set( [ 1, 2, 1, 2, null, null, undefined, false, undefined, false, [ 2 ], [ 2 ], [ 2 ] ] ) );
  var dst = new Set( [ 1, 2, null, undefined, false ] );
  var exp = [ 1, 2, null, undefined, false, [ 2 ], [ 2 ], [ 2 ] ];
  var got = src.onceRight( dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - filled container, dst - filled Set, same content, no onEval';
  var src = _.containerAdapter.make( new Set( [ 1, 2, 1, 2, null, null, undefined, false, undefined, false ] ) );
  var dst = new Set( [ 1, 2, 1, 2, null, null, undefined, false, undefined, false ] );
  var exp = [ 1, 2, null, undefined, false ];
  var got = src.onceRight( dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - filled container, dst - filled setAdapter, same content, one evaluator';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var dst = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 }, { a : 0 }, [ 2 ] ];
  var got = src.onceRight( dst, ( e ) => e );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - filled container, dst - filled setAdapter, same content, equalizer';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var dst = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 }, new U8x( 2 ) ] ) );
  var exp = [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 }, new U8x( 2 ), { a : 0 }, [ 2 ] ];
  var got = src.onceRight( dst, ( e, e2 ) => e === e2 );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - filled container, dst - filled setAdapter, evaluator returns undefined';
  var src = _.containerAdapter.make( new Set( [ 0, 0, 1, 1, undefined, undefined, undefined, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var dst = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ];
  var got = src.onceRight( dst, ( e ) => undefined );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - filled container, dst - filled setAdapter, evaluator checks element of array';
  var src = _.containerAdapter.make( new Set( [ 0, 1, [ null ], [ true ], false, [ undefined ], '', [ 2 ], { a : 0 } ] ) );
  var dst = _.containerAdapter.make( new Set( [ 0, 1, [ null ], [ true ], false, [ undefined ], '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 0, 1, [ null ], [ true ], false, [ undefined ], '', [ 2 ], { a : 0 } ];
  var got = src.onceRight( dst, ( e ) => e[ 0 ] );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - filled container, dst - filled setAdapter, duplicates in src, evaluator checks element of array';
  var src = _.containerAdapter.make( new Set( [ 0, 1, [ null ], [ true ], [ true ], [ true ], false, [ undefined ], '', [ 2 ], { a : 0 } ] ) );
  var dst = _.containerAdapter.make( new Set( [ 0, 1, [ null ], [ true ], false, [ undefined ], '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 0, 1, [ null ], [ true ], false, [ undefined ], '', [ 2 ], { a : 0 } ];
  var got = src.onceRight( dst, ( e ) => e[ 0 ] );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - filled container, dst - filled setAdapter, duplicates in src, two evaluators checks element of array';
  var src = _.containerAdapter.make( new Set( [ 0, 1, [ null ], [ true ], [ true ], [ true ], false, [ undefined ], '', [ 2 ], { a : 0 } ] ) );
  var dst = _.containerAdapter.make( new Set( [ 0, 1, [ null ], [ true ], false, [ undefined ], '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 0, 1, [ null ], [ true ], false, [ undefined ], '', [ 2 ], { a : 0 } ];
  var got = src.onceRight( dst, ( e ) => e[ 0 ], ( ins ) => ins[ 0 ] );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.close( 'only dst' );

  /* - */

  test.open( 'dst === null' );

  test.case = 'src - empty container, no onEval';
  var src = _.containerAdapter.make( new Set( [] ) );
  var got = src.onceRight( null );
  var exp = [];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - filled container, no onEval';
  var src = _.containerAdapter.make( new Set( [ 1, 2, 1, 2, null, null, undefined, false, undefined, false ] ) );
  var got = src.onceRight( null );
  var exp = [ false, undefined, null, 2, 1 ];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - filled container, complex data, no onEval';
  var src = _.containerAdapter.make( new Set( [ 1, 2, 1, 2, null, null, undefined, false, undefined, false, [ 2 ], [ 2 ], [ 2 ] ] ) );
  var got = src.onceRight( null );
  var exp = [ [ 2 ], [ 2 ], [ 2 ], false, undefined, null, 2, 1 ];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - filled container, evaluator returns element';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var got = src.onceRight( null, ( e ) => e );
  var exp = [ { a : 0 }, [ 2 ], '', undefined, false, true, null, 1, 0 ];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - filled container, two evaluators returns element';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var got = src.onceRight( null, ( e ) => e, ( ins ) => ins );
  var exp = [ { a : 0 }, [ 2 ], '', undefined, false, true, null, 1, 0 ];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - filled container, equalizer';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var got = src.onceRight( null, ( e, e2 ) => e === e2 );
  var exp = [ { a : 0 }, [ 2 ], '', undefined, false, true, null, 1, 0 ];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - filled container, evaluator returns undefined';
  var src = _.containerAdapter.make( new Set( [ 0, 0, 1, 1, undefined, undefined, undefined, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var got = src.onceRight( null, ( e ) => undefined );
  var exp = [ { a : 0 } ];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - filled container, evaluator checks element of array, no duplicates in src';
  var src = _.containerAdapter.make( new Set( [ 0, 1, [ null ], [ true ], false, [ undefined ], '', [ 2 ], { a : 0 } ] ) );
  var got = src.onceRight( null, ( e ) => e[ 0 ] );
  var exp = [ { a : 0 }, [ 2 ], [ true ], [ null ] ];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - filled container, evaluator checks element of array, duplicates in src';
  var src = _.containerAdapter.make( new Set( [ 0, 1, [ null ], [ true ], [ true ], [ true ], false, [ undefined ], '', [ 2 ], { a : 0 } ] ) );
  var got = src.onceRight( null, ( e ) => e[ 0 ] );
  var exp = [ { a : 0 }, [ 2 ], [ true ], [ null ] ];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - filled container, two evaluators checks element of array, duplicates in src';
  var src = _.containerAdapter.make( new Set( [ 0, 1, [ null ], [ true ], [ true ], [ true ], false, [ undefined ], '', [ 2 ], { a : 0 } ] ) );
  var got = src.onceRight( null, ( e ) => e[ 0 ], ( ins ) => ins[ 0 ] );
  var exp = [ { a : 0 }, [ 2 ], [ true ], [ null ] ];
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.close( 'dst === null' );

  /* - */

  test.open( 'dst === src' );

  test.case = 'src - empty container, no onEval';
  var src = _.containerAdapter.make( new Set( [] ) );
  var got = src.onceRight( src );
  var exp = [];
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - filled container, no onEval';
  var src = _.containerAdapter.make( new Set( [ 1, 2, 1, 2, null, null, undefined, false, undefined, false ] ) );
  var got = src.onceRight( src );
  var exp = [ 1, 2, null, undefined, false ];
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - filled container, complex data, no onEval';
  var src = _.containerAdapter.make( new Set( [ 1, 2, 1, 2, null, null, undefined, false, undefined, false, [ 2 ], [ 2 ], [ 2 ] ] ) );
  var got = src.onceRight( src );
  var exp = [ 1, 2, null, undefined, false, [ 2 ], [ 2 ], [ 2 ] ];
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - filled container, complex data, evaluator returns element';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var got = src.onceRight( src, ( e ) => e );
  var exp = [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ];
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - filled container, complex data, equalizer';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var got = src.onceRight( src, ( e, e2 ) => e === e2 );
  var exp = [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ];
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - filled container, complex data, evaluator returns undefined';
  var src = _.containerAdapter.make( new Set( [ 0, 0, 1, 1, undefined, undefined, undefined, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var got = src.onceRight( src, ( e ) => undefined );
  var exp = [ 0 ];
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - filled container, complex data, evaluator checks element of array, no duplicates in src';
  var src = _.containerAdapter.make( new Set( [ 0, 1, [ null ], [ true ], false, [ undefined ], '', [ 2 ], { a : 0 } ] ) );
  var got = src.onceRight( src, ( e ) => e[ 0 ] );
  var exp = [ 0, [ null ], [ true ], [ 2 ] ];
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - filled container, complex data, two evaluators checks element of array, no duplicates in src';
  var src = _.containerAdapter.make( new Set( [ 0, 1, [ null ], [ true ], false, [ undefined ], '', [ 2 ], { a : 0 } ] ) );
  var got = src.onceRight( src, ( e ) => e[ 0 ] );
  var exp = [ 0, [ null ], [ true ], [ 2 ] ];
  test.true( got === src );
  test.identical( [ ... got.original ], exp );

  test.close( 'dst === src' );
}

//

function setAdapterFirst( test )
{
  test.case = 'empty container';
  var src = _.containerAdapter.make( new Set() );
  var got = src.first();
  var exp = undefined;
  test.identical( got, exp );

  test.case = 'empty container, onEach returns e';
  var src = _.containerAdapter.make( new Set() );
  var got = src.first( ( e ) => e );
  var exp = undefined;
  test.identical( got, exp );

  test.case = 'container, without onEach';
  var src = _.containerAdapter.make( new Set( [ 'first', 2, 3 ] ) );
  var got = src.first();
  var exp = 'first';
  test.identical( got, exp );

  test.case = 'container, onEach returns boolLike value of element';
  var src = _.containerAdapter.make( new Set( [ 'first', 2, 3 ] ) );
  var got = src.first( ( e ) => e ? 1 : 0 );
  var exp = 1;
  test.identical( got, exp );

  test.case = 'container, onEach add key to element';
  var src = _.containerAdapter.make( new Set( [ 'first', 2, 3 ] ) );
  var got = src.first( ( e, k ) => e + k );
  var exp = 'first0';
  test.identical( got, exp );

  test.case = 'container, onEach check length of container';
  var src = _.containerAdapter.make( new Set( [ 'first', 2, 3 ] ) );
  var got = src.first( ( e, k, c ) => c.length > 2 ? e + 1 : e );
  var exp = 'first1';
  test.identical( got, exp );
}

//

function setAdapterLast( test )
{
  test.case = 'empty container';
  var src = _.containerAdapter.make( new Set() );
  var got = src.last();
  var exp = undefined;
  test.identical( got, exp );

  test.case = 'empty container, onEach returns element';
  var src = _.containerAdapter.make( new Set() );
  var got = src.last( ( e ) => e );
  var exp = undefined;
  test.identical( got, exp );

  test.case = 'container, without onEach';
  var src = _.containerAdapter.make( new Set( [ 'first', 2, 3 ] ) );
  var got = src.last();
  var exp = 3;
  test.identical( got, exp );

  test.case = 'container, onEach returns boolLike value of element';
  var src = _.containerAdapter.make( new Set( [ 'first', 2, 3 ] ) );
  var got = src.last( ( e ) => e ? 1 : 0 );
  var exp = 1;
  test.identical( got, exp );

  test.case = 'container, onEach returns sum of element and key';
  var src = _.containerAdapter.make( new Set( [ 'first', 2, 3 ] ) );
  var got = src.last( ( e, k ) => e + k );
  var exp = 5;
  test.identical( got, exp );

  test.case = 'container, onEach check container';
  var src = _.containerAdapter.make( new Set( [ 'first', 2, 3 ] ) );
  var got = src.last( ( e, k, c ) => c.length > 2 ? e + k : e );
  var exp = 5;
  test.identical( got, exp );
}

//

function setAdapterLastTimeExperiment( test )
{
  var array = new Array( 100000000 );
  array[ 0 ] = 0
  for( let i = 0; i < array.length; i++ )
  array[ i ] = array[ i - 1 ];

  var container = _.containerAdapter.make( new Set( array ) );
  let result1, result2;

  var start = _.time.now();
  for( let i = 0; i < 100000; i++ )
  result1 = container.last();
  var end = _.time.now();

  console.log( end - start );

  // var start = _.time.now();
  // for( let i = 0; i < 100000; i++ )
  // result2 = container.reduce( ( a, e ) => e );
  // var end = _.time.now();
  //
  // console.log( end - start );

  test.identical( result1, result2 );

  /*|         |   Running time     |
    |         |---------|----------|
    |  NodeJS | copy    | reduce   |
    |---------|---------|----------|
    |  v8     | 0.03 s  | 0.02 s   |
    |         | 0.03 s  | 0.02 s   |
    |---------|---------|----------|
    |  v9     | 0.03 s  | 0.02 s   |
    |         | 0.03 s  | 0.02 s   |
    |---------|---------|----------|
    |  v10    | 0.08 s  | 0.42 s   |
    |         | 0.26 s  | 0.06 s   |
    |---------|---------|----------|
    |  v12    | 2.19 s  | 1,42 s   |
    |         | 1.43 s  | 1.12 s   |
  */
  /*
  Dmytro : better time has routine with reduce call ( iterations ). New version of node is slower then older. */
}
setAdapterLastTimeExperiment.experimental = 1;
setAdapterLastTimeExperiment.timeOut = 120000;

//

function setAdapterEach( test )
{
  test.case = 'without arguments';
  var src = _.containerAdapter.make( new Set( [] ) );
  var exp = [];
  var got = src.each( ( e ) => exp.push( e ) );
  test.true( got === src );
  test.identical( [ ... got.original ], exp );
  test.identical( exp, [] );

  test.case = 'from empty array, onEach returns number';
  var src = _.containerAdapter.make( new Set( [] ) );
  var exp = [];
  var got = src.each( ( e ) => exp.push( 123 ) );
  test.true( got === src );
  test.identical( [ ... got.original ], exp );
  test.identical( exp, [] );

  test.case = 'from array, onEach returns original';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [];
  var got = src.each( ( e ) => exp.push( e ) );
  test.true( got === src );
  test.identical( [ ... got.original ], exp );
  test.identical( exp, [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );

  test.case = 'from array, onEach returns key';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [];
  var got = src.each( ( e, k ) => exp.push( k ) );
  test.true( got === src );
  test.identical( [ ... got.original ], [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  test.identical( exp, [ 0, 1, 2, 3, 4, 5, 6, 7, 8 ] );

  test.case = 'from array, onEach check container';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [];
  var got = src.each( ( e, k, c ) => c.length > 10 ? exp.push( k ) : exp.push( c.length - 1 - k ) );
  test.true( got === src );
  test.identical( [ ... got.original ], [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  test.identical( exp, [ 8, 7, 6, 5, 4, 3, 2, 1, 0 ] );

  test.case = 'from array, onEach returns undefined';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [];
  var got = src.each( ( e ) => exp.push( undefined ) );
  test.true( got === src );
  test.identical( [ ... got.original ], [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  test.identical( exp, [ undefined, undefined, undefined, undefined, undefined, undefined, undefined, undefined, undefined ] );

  test.case = 'from array, onEach returns array';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [];
  var got = src.each( ( e ) => exp.push( [ e ] ) );
  test.true( got === src );
  test.identical( [ ... got.original ], [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  test.identical( exp, [ [ 0 ], [ 1 ], [ null ], [ true ], [ false ], [ undefined ], [ '' ], [ [ 2 ] ], [ { a : 0 } ] ] );

  test.case = 'from array, onEach returns element of array';
  var src = _.containerAdapter.make( new Set( [ 0, 1, NaN, true, false, [ undefined ], '', [ 2 ], { a : 0 } ] ) );
  var exp = [];
  var got = src.each( ( e ) => exp.push( e[ 0 ] ) );
  test.true( got === src );
  test.identical( [ ... got.original ], [ 0, 1, NaN, true, false, [ undefined ], '', [ 2 ], { a : 0 } ] );
  test.identical( exp, [ undefined, undefined, undefined, undefined, undefined, undefined, undefined, 2, undefined ] );
}

//

function setAdapterEachRight( test )
{
  test.case = 'without arguments';
  var src = _.containerAdapter.make( new Set( [] ) );
  var exp = [];
  var got = src.eachRight( ( e ) => exp.push( e ) );
  test.true( got === src );
  test.identical( [ ... got.original ], exp );
  test.identical( exp, [] );

  test.case = 'from empty array, onEach returns number';
  var src = _.containerAdapter.make( new Set( [] ) );
  var exp = [];
  var got = src.eachRight( ( e ) => exp.push( 123 ) );
  test.true( got === src );
  test.identical( [ ... got.original ], exp );
  test.identical( exp, [] );

  test.case = 'from array, onEach returns original';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [];
  var got = src.eachRight( ( e ) => exp.push( e ) );
  test.true( got === src );
  test.identical( [ ... got.original ], [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  test.identical( exp, [ { a : 0 }, [ 2 ], '', undefined, false, true, null, 1, 0 ] );

  test.case = 'from array, onEach returns key';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [];
  var got = src.eachRight( ( e, k ) => exp.push( k ) );
  test.true( got === src );
  test.identical( [ ... got.original ], [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  test.identical( exp, [ 8, 7, 6, 5, 4, 3, 2, 1, 0 ] );

  test.case = 'from array, onEach check container';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [];
  var got = src.eachRight( ( e, k, c ) => c.length > 10 ? exp.push( k ) : exp.push( c.length - 1 - k ) );
  test.true( got === src );
  test.identical( [ ... got.original ], [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  test.identical( exp, [ 0, 1, 2, 3, 4, 5, 6, 7, 8 ] );

  test.case = 'from array, onEach returns undefined';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [];
  var got = src.eachRight( ( e ) => exp.push( undefined ) );
  test.true( got === src );
  test.identical( [ ... got.original ], [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  test.identical( exp, [ undefined, undefined, undefined, undefined, undefined, undefined, undefined, undefined, undefined ] );

  test.case = 'from array, onEach returns array';
  var src = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [];
  var got = src.eachRight( ( e ) => exp.push( [ e ] ) );
  test.true( got === src );
  test.identical( [ ... got.original ], [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  test.identical( exp, [ [ { a : 0 } ], [ [ 2 ] ], [ '' ], [ undefined ], [ false ], [ true ], [ null ], [ 1 ], [ 0 ] ] );

  test.case = 'from array, onEach returns element of array';
  var src = _.containerAdapter.make( new Set( [ 0, 1, NaN, true, false, [ undefined ], '', [ 2 ], { a : 0 } ] ) );
  var exp = [];
  var got = src.eachRight( ( e ) => exp.push( e[ 0 ] ) );
  test.true( got === src );
  test.identical( [ ... got.original ], [ 0, 1, NaN, true, false, [ undefined ], '', [ 2 ], { a : 0 } ] );
  test.identical( exp, [ undefined, 2, undefined, undefined, undefined, undefined, undefined, undefined, undefined ] );
}

//

function setAdapterReduce( test )
{
  test.case = 'without accumulator, empty src';
  var src = _.containerAdapter.make( new Set( [] ) );
  var got = src.reduce( ( a, e ) => Math.max( a, e ) );
  test.identical( [ ... src.original ], [] );
  test.identical( got, undefined );

  test.case = 'without accumulator, not empty src';
  var src = _.containerAdapter.make( new Set( [ 1, 2, 3, 0, -2 ] ) );
  var got = src.reduce( ( a, e ) => a === undefined ? Math.max( 0, e ) : Math.max( a, e ) );
  test.identical( [ ... src.original ], [ 1, 2, 3, 0, -2 ] );
  test.identical( got, 3 );

  test.case = 'without accumulator, not empty src, use key';
  var src = _.containerAdapter.make( new Set( [ 1, 2, 3, undefined, -2 ] ) );
  var got = src.reduce( ( a, e, k ) => e === undefined ? Math.max( 0, k ) : Math.max( a, e ) );
  test.identical( [ ... src.original ], [ 1, 2, 3, undefined, -2 ] );
  test.identical( got, 3 );

  test.case = 'without accumulator, not empty src, use container';
  var src = _.containerAdapter.make( new Set( [ 1, 2, 3, undefined, -2 ] ) );
  var got = src.reduce( ( a, e, k, c ) => c.length > 2 ? Math.max( 0, e ) : Math.max( a, e ) );
  test.identical( [ ... src.original ], [ 1, 2, 3, undefined, -2 ] );
  test.identical( got, 0 );

  /* */

  test.case = 'with accumulator, empty src';
  var accumulator = [ 0 ];
  var src = _.containerAdapter.make( new Set( [] ) );
  var got = src.reduce( accumulator, ( a, e ) => a[ a.length - 1 ] > e ? a[ a.length - 1 ] : a.push( e ) );
  test.identical( [ ... src.original ], [] );
  test.true( got === accumulator );
  test.identical( got, [ 0 ] );

  test.case = 'with accumulator, not empty src';
  var accumulator = [ 0 ];
  var src = _.containerAdapter.make( new Set( [ 1, 2, 3, 0, -2 ] ) );
  var got = src.reduce( accumulator, ( a, e ) => a[ a.length - 1 ] < e ? _.arrayAppend( a, e ) : a );
  test.identical( [ ... src.original ], [ 1, 2, 3, 0, -2 ] );
  test.true( got === accumulator );
  test.identical( got, [ 0, 1, 2, 3 ] );

  test.case = 'with accumulator, not empty src, use key';
  var accumulator = [ 0 ];
  var src = _.containerAdapter.make( new Set( [ 1, 2, 3, undefined, -2 ] ) );
  var got = src.reduce( accumulator, ( a, e, k ) => e === undefined ? _.arrayAppend( a, k ) : _.arrayAppend( a, Math.max( a[ a.length - 1 ], e ) ) );
  test.identical( [ ... src.original ], [ 1, 2, 3, undefined, -2 ] );
  test.identical( got, [ 0, 1, 2, 3, 3, 3 ] );

  test.case = 'with accumulator, not empty src, use container';
  var accumulator = [];
  var src = _.containerAdapter.make( new Set( [ 1, 2, 3, undefined, -2 ] ) );
  var got = src.reduce( accumulator, ( a, e, k, c ) => c.length > 2 ? _.arrayAppend( a, k ) : _.arrayAppend( a, e ) );
  test.identical( [ ... src.original ], [ 1, 2, 3, undefined, -2 ] );
  test.identical( got, [ 0, 1, 2, 3, 4 ] );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'onEach has wrong type';
  test.shouldThrowErrorSync( () =>
  {
    let src = _.containerAdapter.make( new Set( [ 1, 2, 3 ] ) );
    src.reduce( 'wrong' );
  });
  test.shouldThrowErrorSync( () =>
  {
    let accumulator = 0;
    let src = _.containerAdapter.make( new Set( [ 1, 2, 3 ] ) );
    src.reduce( accumulator, 'wrong' );
  });
}

//

function setAdapterReduceRight( test )
{
  test.case = 'without accumulator, empty src';
  var src = _.containerAdapter.make( new Set( [] ) );
  var got = src.reduceRight( ( a, e ) => Math.max( a, e ) );
  test.identical( [ ... src.original ], [] );
  test.identical( got, undefined );

  test.case = 'without accumulator, not empty src';
  var src = _.containerAdapter.make( new Set( [ 1, 2, 3, 0, -2 ] ) );
  var got = src.reduceRight( ( a, e ) => a === undefined ? Math.max( 0, e ) : Math.max( a, e ) );
  test.identical( [ ... src.original ], [ 1, 2, 3, 0, -2 ] );
  test.identical( got, 3 );

  test.case = 'without accumulator, not empty src, use key';
  var src = _.containerAdapter.make( new Set( [ 1, 2, 3, undefined, -2 ] ) );
  var got = src.reduceRight( ( a, e, k ) => e === undefined ? Math.max( 0, k ) : Math.max( a, e ) );
  test.identical( [ ... src.original ], [ 1, 2, 3, undefined, -2 ] );
  test.identical( got, 3 );

  test.case = 'without accumulator, not empty src, use container';
  var src = _.containerAdapter.make( new Set( [ 1, 2, 3, undefined, -2 ] ) );
  var got = src.reduceRight( ( a, e, k, c ) => c.length > 2 ? Math.max( 0, e ) : Math.max( a, e ) );
  test.identical( [ ... src.original ], [ 1, 2, 3, undefined, -2 ] );
  test.identical( got, 1 );

  /* */

  test.case = 'with accumulator, empty src';
  var accumulator = [ 0 ];
  var src = _.containerAdapter.make( new Set( [] ) );
  var got = src.reduceRight( accumulator, ( a, e ) => a[ a.length - 1 ] > e ? a[ a.length - 1 ] : a.push( e ) );
  test.identical( [ ... src.original ], [] );
  test.true( got === accumulator );
  test.identical( got, [ 0 ] );

  test.case = 'with accumulator, not empty src';
  var accumulator = [ 0 ];
  var src = _.containerAdapter.make( new Set( [ 1, 2, 3, 0, -2 ] ) );
  var got = src.reduceRight( accumulator, ( a, e ) => a[ a.length - 1 ] < e ? _.arrayAppend( a, e ) : a );
  test.identical( [ ... src.original ], [ 1, 2, 3, 0, -2 ] );
  test.true( got === accumulator );
  test.identical( got, [ 0, 3 ] );

  test.case = 'with accumulator, not empty src, use key';
  var accumulator = [ 0 ];
  var src = _.containerAdapter.make( new Set( [ 1, 2, 3, undefined, -2 ] ) );
  var got = src.reduceRight( accumulator, ( a, e, k ) => e === undefined ? _.arrayAppend( a, k ) : _.arrayAppend( a, Math.max( a[ a.length - 1 ], e ) ) );
  test.identical( [ ... src.original ], [ 1, 2, 3, undefined, -2 ] );
  test.identical( got, [ 0, 0, 3, 3, 3, 3 ] );

  test.case = 'with accumulator, not empty src, use container';
  var accumulator = [];
  var src = _.containerAdapter.make( new Set( [ 1, 2, 3, undefined, -2 ] ) );
  var got = src.reduceRight( accumulator, ( a, e, k, c ) => c.length > 2 ? _.arrayAppend( a, k ) : _.arrayAppend( a, e ) );
  test.identical( [ ... src.original ], [ 1, 2, 3, undefined, -2 ] );
  test.identical( got, [ 4, 3, 2, 1, 0 ] );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'onEach has wrong type';
  test.shouldThrowErrorSync( () =>
  {
    let src = _.containerAdapter.make( new Set( [ 1, 2, 3 ] ) );
    src.reduceRight( 'wrong' );
  });
  test.shouldThrowErrorSync( () =>
  {
    let accumulator = 0;
    let src = _.containerAdapter.make( new Set( [ 1, 2, 3 ] ) );
    src.reduceRight( accumulator, 'wrong' );
  });
}

//

function setAdapterAll( test )
{

  test.case = 'src - empty container, onEach returns element';
  var src = _.containerAdapter.make( new Set() );
  var got = src.all( ( e ) => e );
  test.identical( got, true );

  test.case = 'src - empty container, onEach';
  var src = _.containerAdapter.make( new Set() );
  var got = src.all( ( e ) => true );
  test.identical( got, true );

  test.case = 'all elements is defined, onEach returns element';
  var src = _.containerAdapter.make( new Set( [ 1, 'str', [ 0 ], { a : 0 }, true ] ) );
  var got = src.all( ( e ) => e );
  test.identical( got, true );

  test.case = 'all elements is defined, onEach returns incremented key';
  var src = _.containerAdapter.make( new Set( [ 1, 'str', [ 0 ], { a : 0 }, true ] ) );
  var got = src.all( ( e, i ) => ++i );
  test.identical( got, true );

  test.case = 'all elements is defined, onEach checks container';
  var src = _.containerAdapter.make( new Set( [ 1, 'str', [ 0 ], { a : 0 }, true ] ) );
  var got = src.all( ( e, i, c ) => c.length > 5  );
  test.identical( got, false );

  test.case = 'all elements is undefines, onEach returns element';
  var src = _.containerAdapter.make( new Set( [ false, null, 0, '', undefined ] ) );
  var got = src.all( ( e ) => e );
  test.identical( got, false );

  test.case = 'all elements is undefines, onEach';
  var src = _.containerAdapter.make( new Set( [ false, null, 0, '', undefined ] ) );
  var got = src.all( ( e, i, c ) => c.length > 2  );
  test.identical( got, true );

  test.case = 'all elements is undefines, onEach returns undefined';
  var src = _.containerAdapter.make( new Set( [ false, null, 0, '', undefined ] ) );
  var got = src.all( ( e ) => undefined  );
  test.identical( got, false );

}

//

function setAdapterAllRight( test )
{
  test.case = 'src - empty container, onEach returns element';
  var src = _.containerAdapter.make( new Set() );
  var got = src.allRight( ( e ) => e );
  test.identical( got, true );

  test.case = 'src - empty container, onEach';
  var src = _.containerAdapter.make( new Set() );
  var got = src.allRight( ( e ) => true );
  test.identical( got, true );

  test.case = 'all elements is defined, onEach returns element';
  var src = _.containerAdapter.make( new Set( [ 1, 'str', [ 0 ], { a : 0 }, true ] ) );
  var got = src.allRight( ( e ) => e );
  test.identical( got, true );

  test.case = 'all elements is defined, onEach returns incremented key';
  var src = _.containerAdapter.make( new Set( [ 1, 'str', [ 0 ], { a : 0 }, true ] ) );
  var got = src.allRight( ( e, i ) => ++i );
  test.identical( got, true );

  test.case = 'all elements is defined, onEach checks container';
  var src = _.containerAdapter.make( new Set( [ 1, 'str', [ 0 ], { a : 0 }, true ] ) );
  var got = src.allRight( ( e, i, c ) => c.length > 5  );
  test.identical( got, false );

  test.case = 'all elements is undefines, onEach returns element';
  var src = _.containerAdapter.make( new Set( [ false, null, 0, '', undefined ] ) );
  var got = src.allRight( ( e ) => e );
  test.identical( got, false );

  test.case = 'all elements is undefines, onEach';
  var src = _.containerAdapter.make( new Set( [ false, null, 0, '', undefined ] ) );
  var got = src.allRight( ( e, i, c ) => c.length > 2  );
  test.identical( got, true );

  test.case = 'all elements is undefines, onEach returns undefined';
  var src = _.containerAdapter.make( new Set( [ false, null, 0, '', undefined ] ) );
  var got = src.allRight( ( e ) => undefined  );
  test.identical( got, false );
}

//

function setAdapterAny( test )
{
  test.case = 'src - empty container, onEach returns element';
  var src = _.containerAdapter.make( new Set() );
  var got = src.any( ( e ) => e );
  test.identical( got, false );

  test.case = 'src - empty container, onEach returns true';
  var src = _.containerAdapter.make( new Set() );
  var got = src.any( ( e ) => true );
  test.identical( got, false );

  test.case = 'all elements is defined, onEach returns element';
  var src = _.containerAdapter.make( new Set( [ 1, 'str', [ 0 ], { a : 0 }, true ] ) );
  var got = src.any( ( e ) => e );
  test.identical( got, true );

  test.case = 'one elements is defined, onEach returns incremented key';
  var src = _.containerAdapter.make( new Set( [ null, 0, '', false, undefined, true ] ) );
  var got = src.any( ( e, i ) => ++i );
  test.identical( got, true );

  test.case = 'all elements is defined, onEach checks container';
  var src = _.containerAdapter.make( new Set( [ 1, 'str', [ 0 ], { a : 0 }, true ] ) );
  var got = src.any( ( e, i, c ) => c.length > 5 );
  test.identical( got, false );

  test.case = 'all elements is undefines, onEach return element';
  var src = _.containerAdapter.make( new Set( [ false, null, 0, '', undefined ] ) );
  var got = src.any( ( e ) => e );
  test.identical( got, false );

  test.case = 'all elements defined, onEach';
  var src = _.containerAdapter.make( new Set( [ false, null, 0, '', undefined ] ) );
  var got = src.any( ( e, i, c ) => e === undefined  );
  test.identical( got, true );
}

//

function setAdapterAnyRight( test )
{
  test.case = 'src - empty container, onEach returns element';
  var src = _.containerAdapter.make( new Set() );
  var got = src.anyRight( ( e ) => e );
  test.identical( got, false );

  test.case = 'src - empty container, onEach returns true';
  var src = _.containerAdapter.make( new Set() );
  var got = src.anyRight( ( e ) => true );
  test.identical( got, false );

  test.case = 'all elements is defined, onEach returns element';
  var src = _.containerAdapter.make( new Set( [ 1, 'str', [ 0 ], { a : 0 }, true ] ) );
  var got = src.anyRight( ( e ) => e );
  test.identical( got, true );

  test.case = 'one elements is defined, onEach returns key';
  var src = _.containerAdapter.make( new Set( [ null, 0, '', false, undefined, true ] ) );
  var got = src.anyRight( ( e, i ) => i );
  test.identical( got, true );

  test.case = 'all elements is defined, onEach checks container';
  var src = _.containerAdapter.make( new Set( [ 1, 'str', [ 0 ], { a : 0 }, true ] ) );
  var got = src.anyRight( ( e, i, c ) => c.length > 5 );
  test.identical( got, false );

  test.case = 'all elements is undefines, onEach return element';
  var src = _.containerAdapter.make( new Set( [ false, null, 0, '', undefined ] ) );
  var got = src.anyRight( ( e ) => e );
  test.identical( got, false );

  test.case = 'all elements defined, onEach';
  var src = _.containerAdapter.make( new Set( [ false, null, 0, '', undefined ] ) );
  var got = src.anyRight( ( e, i, c ) => e === undefined  );
  test.identical( got, true );
}

//

function setAdapterNone( test )
{
  test.case = 'src - empty container, onEach returns element';
  var src = _.containerAdapter.make( new Set() );
  var got = src.none( ( e ) => e );
  test.identical( got, true );

  test.case = 'src - empty container, onEach returns false';
  var src = _.containerAdapter.make( new Set() );
  var got = src.none( ( e ) => false );
  test.identical( got, true );

  test.case = 'all elements is defined, onEach returns element';
  var src = _.containerAdapter.make( new Set( [ 1, 'str', [ 0 ], { a : 0 }, true ] ) );
  var got = src.none( ( e ) => e );
  test.identical( got, false );

  test.case = 'one elements is defined, onEach returns element';
  var src = _.containerAdapter.make( new Set( [ null, 0, '', false, undefined, true ] ) );
  var got = src.none( ( e ) => e );
  test.identical( got, false );

  test.case = 'one elements is defined, onEach returns key';
  var src = _.containerAdapter.make( new Set( [ null, 0, '', false, undefined, true ] ) );
  var got = src.none( ( e, i ) => i > 8 ? i : undefined );
  test.identical( got, true );

  test.case = 'all elements is defined, onEach checks container';
  var src = _.containerAdapter.make( new Set( [ 1, 'str', [ 0 ], { a : 0 }, true ] ) );
  var got = src.none( ( e, i, c ) => c.length > 5  );
  test.identical( got, true );

  test.case = 'all elements is undefines, onEach returns element';
  var src = _.containerAdapter.make( new Set( [ false, null, 0, '', undefined ] ) );
  var got = src.none( ( e ) => e );
  test.identical( got, true );

  test.case = 'all elements defined, onEach checks undefined';
  var src = _.containerAdapter.make( new Set( [ false, null, 0, '', undefined ] ) );
  var got = src.none( ( e, i, c ) => e === undefined  );
  test.identical( got, false );
}

//

function setAdapterNoneRight( test )
{
  test.case = 'src - empty container, onEach returns element';
  var src = _.containerAdapter.make( new Set() );
  var got = src.noneRight( ( e ) => e );
  test.identical( got, true );

  test.case = 'src - empty container, onEach returns false';
  var src = _.containerAdapter.make( new Set() );
  var got = src.noneRight( ( e ) => false );
  test.identical( got, true );

  test.case = 'all elements is defined, onEach returns element';
  var src = _.containerAdapter.make( new Set( [ 1, 'str', [ 0 ], { a : 0 }, true ] ) );
  var got = src.noneRight( ( e ) => e );
  test.identical( got, false );

  test.case = 'one elements is defined, onEach returns element';
  var src = _.containerAdapter.make( new Set( [ null, 0, '', false, undefined, true ] ) );
  var got = src.noneRight( ( e ) => e );
  test.identical( got, false );

  test.case = 'one elements is defined, onEach returns key';
  var src = _.containerAdapter.make( new Set( [ null, 0, '', false, undefined, true ] ) );
  var got = src.noneRight( ( e, i ) => i );
  test.identical( got, false );

  test.case = 'all elements is defined, onEach checks container';
  var src = _.containerAdapter.make( new Set( [ 1, 'str', [ 0 ], { a : 0 }, true ] ) );
  var got = src.noneRight( ( e, i, c ) => c.length > 5  );
  test.identical( got, true );

  test.case = 'all elements is undefines, onEach returns element';
  var src = _.containerAdapter.make( new Set( [ false, null, 0, '', undefined ] ) );
  var got = src.noneRight( ( e ) => e );
  test.identical( got, true );

  test.case = 'all elements defined, onEach checks undefined';
  var src = _.containerAdapter.make( new Set( [ false, null, 0, '', undefined ] ) );
  var got = src.noneRight( ( e, i, c ) => e === undefined  );
  test.identical( got, false );
}

//

function setAdapterLeft( test )
{
  test.case = 'empty container';
  var src = _.containerAdapter.make( new Set( [] ) );
  var got = src.left( 1 );
  test.identical( got, { index : -1 } );

  test.case = 'container has not searched element';
  var src = _.containerAdapter.make( new Set( [ 1, 2, 'str', [ 3 ], { a : 2 } ] ) );
  var got = src.left( 3 );
  test.identical( got, { index : -1 } );

  test.case = 'container has duplicated searched element';
  var src = _.containerAdapter.make( new Set( [ 1, 2, 3, 'str', [ 3 ], 3, { a : 2 } ] ) );
  var got = src.left( 3 );
  test.identical( got, { index : 2, element : 3 } );

  test.case = 'searches complex data without evaluators';
  var src = _.containerAdapter.make( new Set( [ 1, 2, 'str', [ 3 ], { a : 2 } ] ) );
  var got = src.left( [ 3 ] );
  test.identical( got, { index : -1 } );

  /* */

  test.case = 'container has not searched element, fromIndex';
  var src = _.containerAdapter.make( new Set( [ 1, 2, 3, 'str', [ 3 ], { a : 2 } ] ) );
  var got = src.left( 3, 4 );
  test.identical( got, { index : -1 } );

  test.case = 'container has duplicated searched element, fromIndex';
  var src = _.containerAdapter.make( new Set( [ 1, 2, 3, 'str', [ 3 ], 3, 'str', { a : 2 } ] ) );
  var got = src.left( 'str', 4 );
  test.identical( got, { index : -1 } );

  test.case = 'searches complex data, fromIndex';
  var src = _.containerAdapter.make( new Set( [ 1, 2, 'str', [ 3 ], { a : 2 } ] ) );
  var got = src.left( [ 3 ], 2 );
  test.identical( got, { index : -1 } );

  /* */

  test.case = 'container has not searched element, onEvaluate1';
  var src = _.containerAdapter.make( new Set( [ 1, 2, 'str', [ 3 ], { a : 2 } ] ) );
  var got = src.left( 3, ( e ) => typeof e );
  test.identical( got, { index : 0, element : 1 } );

  test.case = 'container has duplicated searched element, onEvaluate1';
  var src = _.containerAdapter.make( new Set( [ 1, 2, 3, 'str', [ 3 ], 3, 'str', { a : 2 } ] ) );
  var got = src.left( 'str', ( e ) => e );
  test.identical( got, { index : 3, element : 'str' } );

  test.case = 'searches complex data, onEvaluate1';
  var src = _.containerAdapter.make( new Set( [ 1, 2, 'str', [ 3 ], { a : 2 } ] ) );
  var got = src.left( [ 3 ], ( e ) => e[ 0 ] );
  test.identical( got, { index : 3, element : [ 3 ] } );

  /* */

  test.case = 'container has not searched element, fromIndex, onEvaluate1';
  var src = _.containerAdapter.make( new Set( [ 1, 2, 'str', [ 3 ], { a : 2 } ] ) );
  var got = src.left( 3, 2, ( e ) => typeof e );
  test.identical( got, { index : -1 } );

  test.case = 'container has duplicated searched element, fromIndex, onEvaluate1';
  var src = _.containerAdapter.make( new Set( [ 1, 2, 3, 'str', [ 3 ], 3, 'str', { a : 2 } ] ) );
  var got = src.left( 'str', 4, ( e ) => typeof e );
  test.identical( got, { index : -1 } );

  test.case = 'searches complex data, onEvaluate1';
  var src = _.containerAdapter.make( new Set( [ 1, 2, 'str', [ 3 ], { a : 2 } ] ) );
  var got = src.left( [ 3 ], 4, ( e ) => e[ 0 ] );
  test.identical( got, { index : -1 } );

  /* */

  test.case = 'container has not searched element, onEvaluate1, onEvaluate2';
  var src = _.containerAdapter.make( new Set( [ 1, 2, 'str', [ 3 ], { a : 2 } ] ) );
  var got = src.left( 3, ( e ) => e[ 0 ], ( ins ) => ins );
  test.identical( got, { index : 3, element : [ 3 ] } );

  test.case = 'container has duplicated searched element, onEvaluate1, onEvaluate2';
  var src = _.containerAdapter.make( new Set( [ 1, 2, 3, 'str', [ 3 ], 3, 'str', { a : 2 } ] ) );
  var got = src.left( 2, ( e ) => e.a, ( ins ) => ins );
  test.identical( got, { index : 5, element : { a : 2 } } );

  test.case = 'searches complex data, onEvaluate, onEvaluate2';
  var src = _.containerAdapter.make( new Set( [ 1, 2, 'str', [ 3 ], { a : 2 } ] ) );
  var got = src.left( 3, ( e ) => e[ 0 ], ( ins ) => ins );
  test.identical( got, { index : 3, element : [ 3 ] } );

  /* */

  test.case = 'container has not searched element, fromIndex, onEvaluate1, onEvaluate2';
  var src = _.containerAdapter.make( new Set( [ 1, 2, 'str', [ 3 ], { a : 2 } ] ) );
  var got = src.left( 3, 2, ( e ) => e[ 0 ], ( ins ) => ins );
  test.identical( got, { index : 3, element : [ 3 ] } );

  test.case = 'container has duplicated searched element, fromIndex, onEvaluate1, onEvaluate2';
  var src = _.containerAdapter.make( new Set( [ 1, 2, 3, 'str', [ 3 ], 3, 'str', { a : 2 } ] ) );
  var got = src.left( 2, 5, ( e ) => e.a, ( ins ) => ins );
  test.identical( got, { index : 5, element : { a : 2 } } );

  test.case = 'searches complex data, fromIndex, onEvaluate, onEvaluate2';
  var src = _.containerAdapter.make( new Set( [ 1, 2, 'str', [ 3 ], { a : 2 } ] ) );
  var got = src.left( 3, 4, ( e ) => e[ 0 ], ( ins ) => ins );
  test.identical( got, { index : -1 } );

  /* */

  test.case = 'container has not searched element, equalizer';
  var src = _.containerAdapter.make( new Set( [ 1, 2, 'str', [ 3 ], { a : 2 } ] ) );
  var got = src.left( 3, ( e, ins ) => e[ 0 ] === ins );
  test.identical( got, { index : 3, element : [ 3 ] } );

  test.case = 'container has duplicated searched element, equalizer';
  var src = _.containerAdapter.make( new Set( [ 1, 2, 3, 'str', [ 3 ], 3, 'str', { a : 2 } ] ) );
  var got = src.left( 2, ( e, ins ) => e.a === ins );
  test.identical( got, { index : 5, element : { a : 2 } } );

  test.case = 'searches complex data, equalizer';
  var src = _.containerAdapter.make( new Set( [ 1, 2, 'str', [ 3 ], { a : 2 } ] ) );
  var got = src.left( 3, ( e, ins ) => e[ 0 ] ===  ins );
  test.identical( got, { index : 3, element : [ 3 ] } );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () =>
  {
    var src = _.containerAdapter.make( new Set( [] ) );
    src.left();
  });

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () =>
  {
    var src = _.containerAdapter.make( new Set( [ 1, 2 ] ) );
    src.left( 1, 0, ( e ) => e, ( ins ) => ins, 'extra' );
  });

  test.case = 'onEvaluate1 is not a routine, not a number';
  test.shouldThrowErrorSync( () =>
  {
    var src = _.containerAdapter.make( new Set( [ 1, 2 ] ) );
    src.left( 1, 0, 'wrong' );
  });

  test.case = 'onEvaluate1 has wrong length';
  test.shouldThrowErrorSync( () =>
  {
    var src = _.containerAdapter.make( new Set( [ 1, 2 ] ) );
    src.left( 1, 0, () => 1 );
  });

  test.case = 'onEvaluate2 has wrong length';
  test.shouldThrowErrorSync( () =>
  {
    var src = _.containerAdapter.make( new Set( [ 1, 2 ] ) );
    src.left( 1, 0, ( e ) => e, () => 1 );
  });
}

//

function setAdapterRight( test )
{
  test.case = 'empty container';
  var src = _.containerAdapter.make( new Set( [] ) );
  var got = src.right( 1 );
  test.identical( got, { index : -1 } );

  test.case = 'container has not searched element';
  var src = _.containerAdapter.make( new Set( [ 1, 2, 'str', [ 3 ], { a : 2 } ] ) );
  var got = src.right( 3 );
  test.identical( got, { index : -1 } );

  test.case = 'container has duplicated searched element';
  var src = _.containerAdapter.make( new Set( [ 1, 2, 3, 'str', [ 3 ], 3, { a : 2 } ] ) );
  var got = src.right( 3 );
  test.identical( got, { index : 2, element : 3 } );

  test.case = 'searches complex data without evaluators';
  var src = _.containerAdapter.make( new Set( [ 1, 2, 'str', [ 3 ], { a : 2 } ] ) );
  var got = src.right( [ 3 ] );
  test.identical( got, { index : -1 } );

  /* */

  test.case = 'container has not searched element, fromIndex';
  var src = _.containerAdapter.make( new Set( [ 1, 2, 3, 'str', [ 3 ], { a : 2 } ] ) );
  var got = src.right( 3, 4 );
  test.identical( got, { index : 2, element : 3 } );

  test.case = 'container has duplicated searched element, fromIndex';
  var src = _.containerAdapter.make( new Set( [ 1, 2, 3, 'str', [ 3 ], 3, 'str', { a : 2 } ] ) );
  var got = src.right( 'str', 4 );
  test.identical( got, { index : 3, element : 'str' } );

  test.case = 'searches complex data, fromIndex';
  var src = _.containerAdapter.make( new Set( [ 1, 2, 'str', [ 3 ], { a : 2 } ] ) );
  var got = src.right( [ 3 ], 2 );
  test.identical( got, { index : -1 } );

  /* */

  test.case = 'container has not searched element, onEvaluate1';
  var src = _.containerAdapter.make( new Set( [ 1, 2, 'str', [ 3 ], { a : 2 } ] ) );
  var got = src.right( 3, ( e ) => typeof e );
  test.identical( got, { index : 1, element : 2 } );

  test.case = 'container has duplicated searched element, onEvaluate1';
  var src = _.containerAdapter.make( new Set( [ 1, 2, 3, 'str', [ 3 ], 3, 'str', { a : 2 } ] ) );
  var got = src.right( 'str', ( e ) => e );
  test.identical( got, { index : 3, element : 'str' } );

  test.case = 'searches complex data, onEvaluate1';
  var src = _.containerAdapter.make( new Set( [ 1, 2, 'str', [ 3 ], { a : 2 } ] ) );
  var got = src.right( [ 3 ], ( e ) => e[ 0 ] );
  test.identical( got, { index : 3, element : [ 3 ] } );

  /* */

  test.case = 'container has not searched element, fromIndex, onEvaluate1';
  var src = _.containerAdapter.make( new Set( [ 1, 2, 'str', [ 3 ], { a : 2 } ] ) );
  var got = src.right( 3, 2, ( e ) => typeof e );
  test.identical( got, { index : 1, element : 2 } );

  test.case = 'container has duplicated searched element, fromIndex, onEvaluate1';
  var src = _.containerAdapter.make( new Set( [ 1, 2, 3, 'str', [ 3 ], 3, 'str', { a : 2 } ] ) );
  var got = src.right( 'str', 4, ( e ) => typeof e );
  test.identical( got, { index : 3, element : 'str' } );

  test.case = 'searches complex data, onEvaluate1';
  var src = _.containerAdapter.make( new Set( [ 1, 2, 'str', [ 3 ], { a : 2 } ] ) );
  var got = src.right( [ 3 ], 4, ( e ) => e[ 0 ] );
  test.identical( got, { index : 3, element : [ 3 ] } );

  /* */

  test.case = 'container has not searched element, onEvaluate1, onEvaluate2';
  var src = _.containerAdapter.make( new Set( [ 1, 2, 'str', [ 3 ], { a : 2 } ] ) );
  var got = src.right( 3, ( e ) => e[ 0 ], ( ins ) => ins );
  test.identical( got, { index : 3, element : [ 3 ] } );

  test.case = 'container has duplicated searched element, onEvaluate1, onEvaluate2';
  var src = _.containerAdapter.make( new Set( [ 1, 2, 3, 'str', [ 3 ], 3, 'str', { a : 2 } ] ) );
  var got = src.right( 2, ( e ) => e.a, ( ins ) => ins );
  test.identical( got, { index : 5, element : { a : 2 } } );

  test.case = 'searches complex data, onEvaluate, onEvaluate2';
  var src = _.containerAdapter.make( new Set( [ 1, 2, 'str', [ 3 ], { a : 2 } ] ) );
  var got = src.right( 3, ( e ) => e[ 0 ], ( ins ) => ins );
  test.identical( got, { index : 3, element : [ 3 ] } );

  /* */

  test.case = 'container has not searched element, fromIndex, onEvaluate1, onEvaluate2';
  var src = _.containerAdapter.make( new Set( [ 1, 2, 'str', [ 3 ], { a : 2 } ] ) );
  var got = src.right( 3, 2, ( e ) => e[ 0 ], ( ins ) => ins );
  test.identical( got, { index : -1 } );

  test.case = 'container has duplicated searched element, fromIndex, onEvaluate1, onEvaluate2';
  var src = _.containerAdapter.make( new Set( [ 1, 2, 3, 'str', [ 3 ], 3, 'str', { a : 2 } ] ) );
  var got = src.right( 2, 5, ( e ) => e.a, ( ins ) => ins );
  test.identical( got, { index : -1 } );

  test.case = 'searches complex data, fromIndex, onEvaluate, onEvaluate2';
  var src = _.containerAdapter.make( new Set( [ 1, 2, 'str', [ 3 ], { a : 2 } ] ) );
  var got = src.right( 3, 4, ( e ) => e[ 0 ], ( ins ) => ins );
  test.identical( got, { index : 3, element : [ 3 ] } );

  /* */

  test.case = 'container has not searched element, equalizer';
  var src = _.containerAdapter.make( new Set( [ 1, 2, 'str', [ 3 ], { a : 2 } ] ) );
  var got = src.right( 3, ( e, ins ) => e[ 0 ] === ins );
  test.identical( got, { index : 3, element : [ 3 ] } );

  test.case = 'container has duplicated searched element, equalizer';
  var src = _.containerAdapter.make( new Set( [ 1, 2, 3, 'str', [ 3 ], 3, 'str', { a : 2 } ] ) );
  var got = src.right( 2, ( e, ins ) => e.a === ins );
  test.identical( got, { index : 5, element : { a : 2 } } );

  test.case = 'searches complex data, equalizer';
  var src = _.containerAdapter.make( new Set( [ 1, 2, 'str', [ 3 ], { a : 2 } ] ) );
  var got = src.right( 3, ( e, ins ) => e[ 0 ] ===  ins );
  test.identical( got, { index : 3, element : [ 3 ] } );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () =>
  {
    var src = _.containerAdapter.make( new Set( [] ) );
    src.right();
  });

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () =>
  {
    var src = _.containerAdapter.make( new Set( [ 1, 2 ] ) );
    src.right( 1, 0, ( e ) => e, ( ins ) => ins, 'extra' );
  });

  test.case = 'onEvaluate1 is not a routine, not a number';
  test.shouldThrowErrorSync( () =>
  {
    var src = _.containerAdapter.make( new Set( [ 1, 2 ] ) );
    src.right( 1, 2, 'wrong' );
  });

  test.case = 'onEvaluate1 has wrong length';
  test.shouldThrowErrorSync( () =>
  {
    var src = _.containerAdapter.make( new Set( [ 1, 2 ] ) );
    src.left( 1, 0, () => 1 );
  });

  test.case = 'onEvaluate2 has wrong length';
  test.shouldThrowErrorSync( () =>
  {
    var src = _.containerAdapter.make( new Set( [ 1, 2 ] ) );
    src.left( 1, 0, ( e ) => e, () => 1 );
  });
}


//

function setAdapterReverse( test )
{
  test.case = 'without arguments';
  var src = _.containerAdapter.make( new Set( [ undefined, null, [ 0 ], 'str', { a : false } ] ) );
  var got = src.reverse();
  test.true( got !== src );
  test.identical( [ ... got.original ], [ { a : false }, 'str', [ 0 ], null, undefined ] );

  test.case = 'dst - empty array';
  var src = _.containerAdapter.make( new Set( [ undefined, null, [ 0 ], 'str', { a : false } ] ) );
  var dst = [];
  var got = src.reverse( [] );
  test.true( got !== src );
  test.true( got !== dst );
  test.identical( got.original, [ { a : false }, 'str', [ 0 ], null, undefined ] );

  test.case = 'dst - not empty array';
  var src = _.containerAdapter.make( new Set( [ undefined, null, [ 0 ], 'str', { a : false } ] ) );
  var dst = [ true, '' ];
  var got = src.reverse( dst );
  test.true( got !== src );
  test.true( got !== dst );
  test.identical( got.original, [ true, '', { a : false }, 'str', [ 0 ], null, undefined ] );

  test.case = 'dst - empty Set';
  var src = _.containerAdapter.make( new Set( [ undefined, null, [ 0 ], 'str', { a : false } ] ) );
  var dst = new Set();
  var got = src.reverse( dst );
  test.true( got !== src );
  test.true( got !== dst );
  test.identical( [ ... got.original ], [ { a : false }, 'str', [ 0 ], null, undefined ] );

  test.case = 'dst - not empty Set';
  var src = _.containerAdapter.make( new Set( [ undefined, null, [ 0 ], 'str', { a : false } ] ) );
  var dst = new Set( [ true, '' ] );
  var got = src.reverse( dst );
  test.true( got !== src );
  test.true( got !== dst );
  test.identical( [ ... got.original ], [ true, '', { a : false }, 'str', [ 0 ], null, undefined ] );

  test.case = 'dst - empty arrayContainerAdapter';
  var src = _.containerAdapter.make( new Set( [ undefined, null, [ 0 ], 'str', { a : false } ] ) );
  var dst = _.containerAdapter.make( [] );
  var got = src.reverse( dst );
  test.true( got !== src );
  test.true( got === dst );
  test.identical( got.original, [ { a : false }, 'str', [ 0 ], null, undefined ] );

  test.case = 'dst - not empty arrayContainerAdapter';
  var src = _.containerAdapter.make( new Set( [ undefined, null, [ 0 ], 'str', { a : false } ] ) );
  var dst = _.containerAdapter.make( [ true, '' ] );
  var got = src.reverse( dst );
  test.true( got !== src );
  test.true( got === dst );
  test.identical( got.original, [ true, '', { a : false }, 'str', [ 0 ], null, undefined ] );

  test.case = 'dst - empty setContainerAdapter';
  var src = _.containerAdapter.make( new Set( [ undefined, null, [ 0 ], 'str', { a : false } ] ) );
  var dst = _.containerAdapter.make( new Set() );
  var got = src.reverse( dst );
  test.true( got !== src );
  test.true( got === dst );
  test.identical( [ ... got.original ], [ { a : false }, 'str', [ 0 ], null, undefined ] );

  test.case = 'dst - not empty setContainerAdapter';
  var src = _.containerAdapter.make( new Set( [ undefined, null, [ 0 ], 'str', { a : false } ] ) );
  var dst = _.containerAdapter.make( new Set( [ true, '' ] ) );
  var got = src.reverse( dst );
  test.true( got !== src );
  test.true( got === dst );
  test.identical( [ ... got.original ], [ true, '', { a : false }, 'str', [ 0 ], null, undefined ] );

  test.case = 'dst === src, odd number of elements';
  var src = _.containerAdapter.make( new Set( [ undefined, null, [ 0 ], 'str', { a : false } ] ) );
  var got = src.reverse( src );
  test.true( got === src );
  test.identical( [ ... got.original ], [ { a : false }, 'str', [ 0 ], null, undefined ] );

  test.case = 'dst === src, even number of elements';
  var src = _.containerAdapter.make( new Set( [ undefined, null, [ 0 ], 'str', { a : false }, true ] ) );
  var got = src.reverse( src );
  test.true( got === src );
  test.identical( [ ... got.original ], [ true, { a : false }, 'str', [ 0 ], null, undefined ] );
}

//

function setAdapterJoin( test )
{
  test.case = 'empty container, without delimeter';
  var src = _.containerAdapter.make( new Set() );
  var got = src.join();
  test.identical( got, '' );

  test.case = 'empty container, delimeter';
  var src = _.containerAdapter.make( new Set() );
  var got = src.join( '/' );
  test.identical( got, '' );

  test.case = 'not empty container, delimeter';
  var src = _.containerAdapter.make( new Set( [ 1, 2, [ 3, 4 ] ] ) );
  var got = src.join( '/' );
  test.identical( got, '1/2/3,4' );
}

//

function setAdapterToArray( test )
{
  test.case = 'empty container';
  var src = _.containerAdapter.make( new Set() );
  var got = src.toArray();
  test.true( got !== src );
  test.true( got.original !== src.original );
  test.identical( got.original, [ ... src.original ] );

  test.case = 'not empty container';
  var src = _.containerAdapter.make( new Set( [ 1, 2, 3 ] ) );
  var got = src.toArray();
  test.true( got !== src );
  test.true( got.original !== src.original );
  test.identical( got.original, [ ... src.original ] );
}

//

function setAdapterToSet( test )
{
  test.case = 'empty container';
  var src = _.containerAdapter.make( new Set() );
  var got = src.toSet();
  test.true( got === src );
  test.true( got.original === src.original );
  test.identical( [ ... got.original ], [ ... src.original ] );

  test.case = 'not empty container';
  var src = _.containerAdapter.make( new Set( [ 1, 2, 3 ] ) );
  var got = src.toSet();
  test.true( got === src );
  test.true( got.original === src.original );
  test.identical( [ ... got.original ], [ ... src.original ] );
}

//

function setAdapterSymbolIterator( test )
{
  test.case = 'empty container';
  var src = _.containerAdapter.make( new Set( [] ) );
  var exp = [];
  for( let e of src )
  exp.push( e + 1 );
  test.identical( [ ... src.original ], [] );
  test.identical( exp, [] );

  test.case = 'not empty container';
  var src = _.containerAdapter.make( new Set( [ 1, 2, 3 ] ) );
  var exp = [];
  for( let e of src )
  exp.push( e + 1 );
  test.identical( [ ... src.original ], [ 1, 2, 3 ] );
  test.identical( exp, [ 2, 3, 4 ] );
}

//

function setAdapterLength( test )
{
  test.case = 'empty container';
  var src = _.containerAdapter.make( new Set() );
  var got = src.length;
  test.identical( got, 0 )

  test.case = 'not empty container';
  var src = _.containerAdapter.make( new Set( [ 1, 2, 3, undefined ] ) );
  var got = src.length;
  test.identical( got, 4 )
}

//--
// ContainerAdapterArray
//--

function arrayAdapterMake_( test )
{
  test.case = 'from array';
  var src = _.containerAdapter.make( [ 1, { a : 2 }, [ 0 ], undefined ] );
  var got = src.make();
  var exp = [ 1, { a : 2 }, [ 0 ], undefined ];
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'from unroll';
  var src = _.containerAdapter.make(  _.unroll.make( [ 1, { a : 2 }, [ 0 ], undefined ] ) );
  var got = src.make();
  var exp = [ 1, { a : 2 }, [ 0 ], undefined ];
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'from arrayContainerAdapter';
  var src = _.containerAdapter.make( _.containerAdapter.make( [ 1, { a : 2 }, [ 0 ], undefined ] ) );
  var got = src.make();
  var exp = [ 1, { a : 2 }, [ 0 ], undefined ];
  test.true( got !== src );
  test.identical( got.original, exp );

  if( !Config.debug )
  return;

  test.case = 'argument is provided';
  test.shouldThrowErrorSync( () =>
  {
    var src = _.containerAdapter.make( [] );
    src.make( [ 1 ] );
  });
}

//

function arrayAdapterMakeEmpty( test )
{
  test.case = 'from array';
  var src = _.containerAdapter.make( [ 1, { a : 2 }, [ 0 ], undefined ] );
  var got = src.MakeEmpty();
  var exp = [];
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'from unroll';
  var src = _.containerAdapter.make(  _.unroll.make( [ 1, { a : 2 }, [ 0 ], undefined ] ) );
  var got = src.MakeEmpty();
  var exp = [];
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'from arrayContainerAdapter';
  var src = _.containerAdapter.make( _.containerAdapter.make( [ 1, { a : 2 }, [ 0 ], undefined ] ) );
  var got = src.MakeEmpty();
  var exp = [];
  test.true( got !== src );
  test.identical( got.original, exp );

  if( !Config.debug )
  return;

  test.case = 'argument is provided';
  test.shouldThrowErrorSync( () =>
  {
    var src = _.containerAdapter.MakeEmpty( [] );
    src.make( [ 1 ] );
  });
}

//

function arrayAdapterMake( test )
{
  test.case = 'without arguments';
  var src = _.containerAdapter.make( [] );
  var got = src.Make();
  test.true( got !== src );
  test.identical( got.original, [] );

  test.case = 'from undefined';
  var src = _.containerAdapter.make( [] );
  var got = src.Make( undefined );
  test.true( got !== src );
  test.identical( got.original, [] );

  test.case = 'from null';
  var src = _.containerAdapter.make( [] );
  var got = src.Make( null );
  test.true( got !== src );
  test.identical( got.original, [] );

  test.case = 'from zero';
  var src = _.containerAdapter.make( [] );
  var got = src.Make( 0 );
  test.true( got !== src );
  test.identical( got.original, [] );

  test.case = 'from number';
  var src = _.containerAdapter.make( [] );
  var got = src.Make( 100 );
  test.true( got !== src );
  test.identical( got.original, new Array( 100 ) );

  test.case = 'from array';
  var container = [ 1, 2, 3 ];
  var src = _.containerAdapter.make( [] );
  var got = src.Make( container );
  test.true( got !== container );
  test.true( got !== src );
  test.identical( got.original, [ 1, 2, 3 ] );

  test.case = 'from argumentsArray';
  var container = _.argumentsArray.make( [ 1, 2, 3 ] );
  var src = _.containerAdapter.make( [] );
  var got = src.Make( container );
  test.true( got !== container );
  test.true( got !== src );
  test.identical( got.original, [ 1, 2, 3 ] );

  test.case = 'from BufferTyped';
  var container = new F32x( [ 1, 2, 3 ] );
  var src = _.containerAdapter.make( [] );
  var got = src.Make( container );
  test.true( got !== container );
  test.true( got !== src );
  test.identical( got.original, [ 1, 2, 3 ] );

  test.case = 'from arrayContainerAdapter';
  var container = _.containerAdapter.make( [ 1, 2, 3 ] );
  var src = _.containerAdapter.make( [] );
  var got = src.Make( container );
  test.true( got !== container );
  test.true( got !== src );
  test.identical( got.original, [ 1, 2, 3 ] );

  test.case = 'from setContainerAdapter';
  var container = _.containerAdapter.make( new Set( [ 1, 2, 3 ] ) );
  var src = _.containerAdapter.make( [] );
  var got = src.Make( container );
  test.true( got !== container );
  test.true( got !== src );
  test.identical( got.original, [ 1, 2, 3 ] );

  test.case = 'from self';
  var src = _.containerAdapter.make( [] );
  var got = src.Make( src );
  test.true( got !== container );
  test.true( got !== src );
  test.identical( got.original, [] );

  if( !Config.debug )
  return;

  test.case = 'from string';
  var src = _.containerAdapter.make( [] );
  var got = src.Make( 'str' );
  test.identical( got, undefined );

  test.case = 'from BufferRaw';
  var src = _.containerAdapter.make( [] );
  var got = src.Make( new BufferRaw( 10 ) );
  test.identical( got, undefined );
}

//

function arrayAdapterHas( test )
{
  test.case = 'empty container';
  var src = _.containerAdapter.make( [] );
  var got = src.has( 1 );
  test.identical( got, false );

  test.case = 'container, no entries';
  var src = _.containerAdapter.make( [ 1, 2, 3, 4 ] );
  var got = src.has( 0 );
  test.identical( got, false );

  test.case = 'container, entry';
  var src = _.containerAdapter.make( [ 1, 2, 3, 4 ] );
  var got = src.has( 3 );
  test.identical( got, true );

  test.case = 'container with arrays, no entries';
  var src = _.containerAdapter.make( [ [ 1 ], [ 2 ], [ 1 ] ] );
  var got = src.has( [ 1 ] );
  test.identical( [ ... src.original ], [ [ 1 ], [ 2 ], [ 1 ] ] );
  test.identical( got, false );

  test.case = 'container with maps, no entries';
  var src = _.containerAdapter.make( [ { a : 1 }, { b : 1 }, { a : 1 } ] );
  var got = src.has( { a : 1 } );
  test.identical( [ ... src.original ], [ { a : 1 }, { b : 1 }, { a : 1 } ] );
  test.identical( got, false );

  /* */

  test.case = 'container with arrays, one evaluator';
  var src = _.containerAdapter.make( [ [ 1 ], [ 2 ], [ 1 ] ] );
  var got = src.has( [ 1 ], ( e ) => e[ 0 ] );
  test.identical( [ ... src.original ], [ [ 1 ], [ 2 ], [ 1 ] ] );
  test.identical( got, true );

  test.case = 'container with maps, one evaluator';
  var src = _.containerAdapter.make( [ { a : 1 }, { b : 1 }, { a : 1 } ] );
  var got = src.has( { a : 1 }, ( e ) => e.a );
  test.identical( [ ... src.original ], [ { a : 1 }, { b : 1 }, { a : 1 } ] );
  test.identical( got, true );

  test.case = 'container with arrays, two evaluators';
  var src = _.containerAdapter.make( [ [ 1 ], [ 2 ], [ 1 ] ] );
  var got = src.has( 1, ( e ) => e[ 0 ], ( ins ) => ins );
  test.identical( [ ... src.original ], [ [ 1 ], [ 2 ], [ 1 ] ] );
  test.identical( got, true );

  test.case = 'container with maps, two evaluators';
  var src = _.containerAdapter.make( [ { a : 1 }, { b : 1 }, { a : 1 } ] );
  var got = src.has( 1, ( e ) => e.a, ( ins ) => ins );
  test.identical( [ ... src.original ], [ { a : 1 }, { b : 1 }, { a : 1 } ] );
  test.identical( got, true );

  test.case = 'container with arrays, fromIndex and evaluator2';
  var src = _.containerAdapter.make( [ [ 1 ], [ 2 ], [ 1 ] ] );
  var got = src.has( 1, 3, ( e ) => e[ 0 ] )
  test.identical( [ ... src.original ], [ [ 1 ], [ 2 ], [ 1 ] ] );
  test.identical( got, false );

  test.case = 'container with maps, fromIndex and evaluator2';
  var src = _.containerAdapter.make( [ { a : 1 }, { b : 1 }, { a : 1 } ] );
  var got = src.has( 1, 3, ( e ) => e.a );
  test.identical( [ ... src.original ], [ { a : 1 }, { b : 1 }, { a : 1 } ] );
  test.identical( got, false );

  test.case = 'container with arrays, equalizer';
  var src = _.containerAdapter.make( [ [ 1 ], [ 2 ], [ 1 ] ] );
  var got = src.has( 1, ( e, ins ) => e[ 0 ] === ins );
  test.identical( [ ... src.original ], [ [ 1 ], [ 2 ], [ 1 ] ] );
  test.identical( got, true );

  test.case = 'container with maps, equalizer';
  var src = _.containerAdapter.make( [ { a : 1 }, { b : 1 }, { a : 1 } ] );
  var got = src.has( 1, ( e, ins ) => e.a === ins );
  test.identical( [ ... src.original ], [ { a : 1 }, { b : 1 }, { a : 1 } ] );
  test.identical( got, true );
}

//

function arrayAdapterCopyFrom( test )
{
  test.case = 'empty container, src - empty array';
  var container = _.containerAdapter.make( [] );
  var src = [];
  var got = container.copyFrom( src );
  test.true( got === container );
  test.true( got !== src );
  test.identical( got.original, [] );
  test.identical( src, [] );

  test.case = 'empty container, src - not empty array';
  var container = _.containerAdapter.make( [] );
  var src = [ undefined, null, [ 1 ], 2, 'str' ];
  var got = container.copyFrom( src );
  test.true( got === container );
  test.true( got !== src );
  test.identical( got.original, [ undefined, null, [ 1 ], 2, 'str' ] );
  test.identical( src, [ undefined, null, [ 1 ], 2, 'str' ] );

  test.case = 'not empty container, src - empty array';
  var container = _.containerAdapter.make( [ true, false ] );
  var src = [];
  var got = container.copyFrom( src );
  test.true( got === container );
  test.true( got !== src );
  test.identical( got.original, [ true, false ] );
  test.identical( src, [] );

  test.case = 'container.length < src.length';
  var container = _.containerAdapter.make( [ true, false ] );
  var src = [ undefined, null, [ 1 ], 2, 'str' ];
  var got = container.copyFrom( src );
  test.true( got === container );
  test.true( got !== src );
  test.identical( got.original, [ undefined, null, [ 1 ], 2, 'str' ] );
  test.identical( src, [ undefined, null, [ 1 ], 2, 'str' ] );

  test.case = 'container.length === src.length';
  var container = _.containerAdapter.make( [ true, false ] );
  var src = [ 2, 'str' ];
  var got = container.copyFrom( src );
  test.true( got === container );
  test.true( got !== src );
  test.identical( got.original, [ 2, 'str' ] );
  test.identical( src, [ 2, 'str' ] );

  test.case = 'container.length > src.length';
  var container = _.containerAdapter.make( [ true, false, 2, 'str' ] );
  var src = [ undefined, null, [ 1 ] ];
  var got = container.copyFrom( src );
  test.true( got === container );
  test.true( got !== src );
  test.identical( got.original, [ undefined, null, [ 1 ], 'str' ] );
  test.identical( src, [ undefined, null, [ 1 ] ] );

  /* */

  test.case = 'empty container, src - empty Set';
  var container = _.containerAdapter.make( [] );
  var src = new Set();
  var got = container.copyFrom( src );
  test.true( got === container );
  test.true( got !== src );
  test.identical( got.original, [] );
  test.identical( [ ... src ], [] );

  test.case = 'empty container, src - not empty Set';
  var container = _.containerAdapter.make( [] );
  var src = new Set( [ undefined, null, [ 1 ], 2, 'str' ] );
  var got = container.copyFrom( src );
  test.true( got === container );
  test.true( got !== src );
  test.identical( got.original, [ undefined, null, [ 1 ], 2, 'str' ] );
  test.identical( [ ... src ], [ undefined, null, [ 1 ], 2, 'str' ] );

  test.case = 'not empty container, src - empty Set';
  var container = _.containerAdapter.make( [ true, false ] );
  var src = new Set();
  var got = container.copyFrom( src );
  test.true( got === container );
  test.true( got !== src );
  test.identical( got.original, [ true, false ] );
  test.identical( [ ... src ], [] );

  test.case = 'container.length < src.length';
  var container = _.containerAdapter.make( [ true, false ] );
  var src = new Set( [ undefined, null, [ 1 ], 2, 'str' ] );
  var got = container.copyFrom( src );
  test.true( got === container );
  test.true( got !== src );
  test.identical( got.original, [ undefined, null, [ 1 ], 2, 'str' ] );
  test.identical( [ ... src ], [ undefined, null, [ 1 ], 2, 'str' ] );

  test.case = 'container.length === src.length';
  var container = _.containerAdapter.make( [ true, false ] );
  var src = new Set( [ 2, 'str' ] );
  var got = container.copyFrom( src );
  test.true( got === container );
  test.true( got !== src );
  test.identical( got.original, [ 2, 'str' ] );
  test.identical( [ ... src ], [ 2, 'str' ] );

  test.case = 'container.length > src.length';
  var container = _.containerAdapter.make( [ true, false, 2, 'str' ] );
  var src = new Set( [ undefined, null, [ 1 ] ] );
  var got = container.copyFrom( src );
  test.true( got === container );
  test.true( got !== src );
  test.identical( got.original, [ undefined, null, [ 1 ], 'str' ] );
  test.identical( [ ... src ], [ undefined, null, [ 1 ] ] );

  /* */

  test.case = 'empty container, src - empty arrayContainerAdapter';
  var container = _.containerAdapter.make( [] );
  var src = _.containerAdapter.make( [] );
  var got = container.copyFrom( src );
  test.true( got === container );
  test.true( got !== src );
  test.identical( got.original, [] );
  test.identical( src.original, [] );

  test.case = 'empty container, src - not empty arrayContainerAdapter';
  var container = _.containerAdapter.make( [] );
  var src = _.containerAdapter.make( [ undefined, null, [ 1 ], 2, 'str' ] );
  var got = container.copyFrom( src );
  test.true( got === container );
  test.true( got !== src );
  test.identical( got.original, [ undefined, null, [ 1 ], 2, 'str' ] );
  test.identical( src.original, [ undefined, null, [ 1 ], 2, 'str' ] );

  test.case = 'not empty container, src - empty arrayContainerAdapter';
  var container = _.containerAdapter.make( [ true, false ] );
  var src = _.containerAdapter.make( [] );
  var got = container.copyFrom( src );
  test.true( got === container );
  test.true( got !== src );
  test.identical( got.original, [ true, false ] );
  test.identical( src.original, [] );

  test.case = 'container.length < src.length';
  var container = _.containerAdapter.make( [ true, false ] );
  var src = _.containerAdapter.make( [ undefined, null, [ 1 ], 2, 'str' ] );
  var got = container.copyFrom( src );
  test.true( got === container );
  test.true( got !== src );
  test.identical( got.original, [ undefined, null, [ 1 ], 2, 'str' ] );
  test.identical( src.original, [ undefined, null, [ 1 ], 2, 'str' ] );

  test.case = 'container.length === src.length';
  var container = _.containerAdapter.make( [ true, false, null ] );
  var src = _.containerAdapter.make( [ [ 1 ], 2, 'str' ] );
  var got = container.copyFrom( src );
  test.true( got === container );
  test.true( got !== src );
  test.identical( got.original, [ [ 1 ], 2, 'str' ] );
  test.identical( src.original, [ [ 1 ], 2, 'str' ] );

  test.case = 'container.length > src.length';
  var container = _.containerAdapter.make( [ true, false, 2, 'str' ] );
  var src = _.containerAdapter.make( [ undefined, null, [ 1 ] ] );
  var got = container.copyFrom( src );
  test.true( got === container );
  test.true( got !== src );
  test.identical( got.original, [ undefined, null, [ 1 ], 'str' ] );
  test.identical( src.original, [ undefined, null, [ 1 ] ] );

  /* */

  test.case = 'empty container, src - empty setContainerAdapter';
  var container = _.containerAdapter.make( [] );
  var src = _.containerAdapter.make( new Set() );
  var got = container.copyFrom( src );
  test.true( got === container );
  test.true( got !== src );
  test.identical( got.original, [] );
  test.identical( [ ... src ], [] );

  test.case = 'empty container, src - not empty Set';
  var container = _.containerAdapter.make( [] );
  var src = _.containerAdapter.make( new Set( [ undefined, null, [ 1 ], 2, 'str' ] ) );
  var got = container.copyFrom( src );
  test.true( got === container );
  test.true( got !== src );
  test.identical( got.original, [ undefined, null, [ 1 ], 2, 'str' ] );
  test.identical( [ ... src ], [ undefined, null, [ 1 ], 2, 'str' ] );

  test.case = 'not empty container, src - empty Set';
  var container = _.containerAdapter.make( [ true, false ] );
  var src = _.containerAdapter.make( new Set() );
  var got = container.copyFrom( src );
  test.true( got === container );
  test.true( got !== src );
  test.identical( got.original, [ true, false ] );
  test.identical( [ ... src ], [] );

  test.case = 'container.length < src.length';
  var container = _.containerAdapter.make( [ true, false ] );
  var src = _.containerAdapter.make( new Set( [ undefined, null, [ 1 ], 2, 'str' ] ) );
  var got = container.copyFrom( src );
  test.true( got === container );
  test.true( got !== src );
  test.identical( got.original, [ undefined, null, [ 1 ], 2, 'str' ] );
  test.identical( [ ... src ], [ undefined, null, [ 1 ], 2, 'str' ] );

  test.case = 'container.length === src.length';
  var container = _.containerAdapter.make( [ true, false ] );
  var src = _.containerAdapter.make( new Set( [ 2, 'str' ] ) );
  var got = container.copyFrom( src );
  test.true( got === container );
  test.true( got !== src );
  test.identical( got.original, [ 2, 'str' ] );
  test.identical( [ ... src ], [ 2, 'str' ] );

  test.case = 'container.length > src.length';
  var container = _.containerAdapter.make( [ true, false, 2, 'str' ] );
  var src = _.containerAdapter.make( new Set( [ undefined, null, [ 1 ] ] ) );
  var got = container.copyFrom( src );
  test.true( got === container );
  test.true( got !== src );
  test.identical( got.original, [ undefined, null, [ 1 ], 'str' ] );
  test.identical( [ ... src ], [ undefined, null, [ 1 ] ] );

  /* */

  /* */

  test.case = 'empty container';
  var container = _.containerAdapter.make( [] );
  var got = container.copyFrom( container );
  test.true( got === container );
  test.identical( got.original, [] );

  test.case = 'container from src';
  var src = [ 1, 2 ];
  var container = _.containerAdapter.make( src );
  var got = container.copyFrom( src );
  test.true( got === container );
  test.identical( [ ... got.original ], [ 1, 2 ] );

  test.case = 'not empty container';
  var container = _.containerAdapter.make( [ undefined, null, [ 1 ], 2, 'str' ] );
  var got = container.copyFrom( container );
  test.true( got === container );
  test.identical( [ ... got.original ], [ undefined, null, [ 1 ], 2, 'str' ] );

  test.case = 'not empty containers from src';
  var src = [ 1, 2 ];
  var src2 = _.containerAdapter.make( src );
  var container = _.containerAdapter.make( src );
  var got = container.copyFrom( src2 );
  test.true( got === container );
  test.identical( [ ... got.original ], [ 1, 2 ] );
}

//

function arrayAdapterCount( test )
{
  test.case = 'empty container';
  var src = _.containerAdapter.make( [] );
  var got = src.count();
  test.identical( got, 0 );

  var src = _.containerAdapter.make( [] );
  var got = src.count( 1 );
  test.identical( got, 0 );

  /* */

  test.case = 'container with primitive, no entries';
  var src = _.containerAdapter.make( [ 1, 2, 3, 4, 5 ] );
  var got = src.count( 6 );
  test.identical( got, 0 );

  test.case = 'container with primitive, has entry';
  var src = _.containerAdapter.make( [ 1, 2, 3, 4, 5 ] );
  var got = src.count( 3 );
  test.identical( got, 1 );

  test.case = 'container with maps, has entries, not count';
  var src = _.containerAdapter.make( [ { a : 1 }, { a : 1 }, { a : 1 }, { a : 1 }, { a : 1 } ] );
  var got = src.count( { a : 1 } );
  var exp = [ { a : 1 }, { a : 1 }, { a : 1 }, { a : 1 }, { a : 1 } ];
  test.identical( src.original, exp );
  test.identical( got, 0 );

  test.case = 'container with arrays, has entries, not count';
  var src = _.containerAdapter.make( [ [ 1 ], [ 1 ], [ 1 ], [ 1 ], [ 1 ] ] );
  var got = src.count( [ 1 ] );
  var exp = [ [ 1 ], [ 1 ], [ 1 ], [ 1 ], [ 1 ] ];
  test.identical( src.original, exp );
  test.identical( got, 0 );

  /* */

  test.case = 'container with primitive, no entries, one evaluator';
  var src = _.containerAdapter.make( [ 1, 2, 3, 4, 5 ] );
  var got = src.count( 6, ( e ) => 1 );
  test.identical( got, 5 );

  test.case = 'container with primitive, has entry, one evaluator';
  var src = _.containerAdapter.make( [ 1, 2, 3, 4, 5 ] );
  var got = src.count( 3, ( e ) => e );
  test.identical( got, 1 );

  test.case = 'container with maps, has entries, one evaluator';
  var src = _.containerAdapter.make( [ { a : 1 }, { a : 1 }, { a : 1 }, { a : 1 }, { a : 1 } ] );
  var got = src.count( { a : 1 }, ( e ) => e.a );
  var exp = [ { a : 1 }, { a : 1 }, { a : 1 }, { a : 1 }, { a : 1 } ];
  test.identical( src.original, exp );
  test.identical( got, 5 );

  test.case = 'container with arrays, has entries, one evaluator';
  var src = _.containerAdapter.make( [ [ 1 ], [ 1 ], [ 1 ], [ 1 ], [ 1 ] ] );
  var got = src.count( [ 1 ], ( e ) => e[ 0 ] );
  var exp = [ [ 1 ], [ 1 ], [ 1 ], [ 1 ], [ 1 ] ];
  test.identical( src.original, exp );
  test.identical( got, 5 );

  /* */

  test.case = 'container with primitive, no entries, two evaluators';
  var src = _.containerAdapter.make( [ 1, 2, 3, 4, 5 ] );
  var got = src.count( 6, ( e ) => e, ( ins ) => ins - 1 );
  test.identical( got, 1 );

  test.case = 'container with primitive, has entry, two evaluators';
  var src = _.containerAdapter.make( [ 1, 2, 3, 4, 5 ] );
  var got = src.count( 3, ( e ) => e, ( ins ) => ins + 3 );
  test.identical( got, 0 );

  test.case = 'container with maps, has entries, two evaluators';
  var src = _.containerAdapter.make( [ { a : 1 }, { a : 1 }, { a : 1 }, { a : 1 }, { a : 1 } ] );
  var got = src.count( { a : 1 }, ( e ) => e.a, ( ins ) => ins.a );
  var exp = [ { a : 1 }, { a : 1 }, { a : 1 }, { a : 1 }, { a : 1 } ];
  test.identical( src.original, exp );
  test.identical( got, 5 );

  test.case = 'container with arrays, has entries, two evaluators';
  var src = _.containerAdapter.make( [ [ 1 ], [ 1 ], [ 1 ], [ 1 ], [ 1 ] ] );
  var got = src.count( [ 1 ], ( e ) => e[ 0 ], ( ins ) => ins[ 0 ] );
  var exp = [ [ 1 ], [ 1 ], [ 1 ], [ 1 ], [ 1 ] ];
  test.identical( src.original, exp );
  test.identical( got, 5 );

  /* */

  test.case = 'container with primitive, no entries, equalizer';
  var src = _.containerAdapter.make( [ 1, 2, 3, 4, 5 ] );
  var got = src.count( 6, ( e, ins ) => e === ins - 1 );
  test.identical( got, 1 );

  test.case = 'container with primitive, has entry, equalizer';
  var src = _.containerAdapter.make( [ 1, 2, 3, 4, 5 ] );
  var got = src.count( 3, ( e, ins ) => e === ins + 3 );
  test.identical( got, 0 );

  test.case = 'container with maps, has entries, equalizer';
  var src = _.containerAdapter.make( [ { a : 1 }, { a : 1 }, { a : 1 }, { a : 1 }, { a : 1 } ] );
  var got = src.count( { a : 1 }, ( e, ins ) => e.a === ins.a );
  var exp = [ { a : 1 }, { a : 1 }, { a : 1 }, { a : 1 }, { a : 1 } ];
  test.identical( src.original, exp );
  test.identical( got, 5 );

  test.case = 'container with arrays, has entries, equalizer';
  var src = _.containerAdapter.make( [ [ 1 ], [ 1 ], [ 1 ], [ 1 ], [ 1 ] ] );
  var got = src.count( [ 1 ], ( e, ins ) => e[ 0 ] === ins[ 0 ] );
  var exp = [ [ 1 ], [ 1 ], [ 1 ], [ 1 ], [ 1 ] ];
  test.identical( src.original, exp );
  test.identical( got, 5 );
}

//

function arrayAdapterAppend( test )
{
  test.case = 'empty container, append primitive';
  var dst = _.containerAdapter.make( [] );
  var got = dst.append( 1 );
  var exp = [ 1 ];
  test.true( got === dst );
  test.identical( got.original, exp );

  test.case = 'empty container, append Long';
  var dst = _.containerAdapter.make( [] );
  var got = dst.append( _.unroll.make( [ 1, 2 ] ) );
  var exp = [ [ 1, 2 ] ];
  test.true( got === dst );
  test.identical( got.original, exp );

  test.case = 'empty container, append map';
  var dst = _.containerAdapter.make( [] );
  var got = dst.append( { a : 0 } );
  var exp = [ { a : 0 } ];
  test.true( got === dst );
  test.identical( got.original, exp );

  /* */

  test.case = 'container, append primitive';
  var dst = _.containerAdapter.make( [ undefined ] );
  var got = dst.append( 1 );
  var exp = [ undefined, 1 ];
  test.true( got === dst );
  test.identical( got.original, exp );

  test.case = 'container, append Long';
  var dst = _.containerAdapter.make( [ null ] );
  var got = dst.append( _.unroll.make( [ 1, 2 ] ) );
  var exp = [ null, [ 1, 2 ] ];
  test.true( got === dst );
  test.identical( got.original, exp );

  test.case = 'container, append map';
  var dst = _.containerAdapter.make( [ 0 ] );
  var got = dst.append( { a : 0 } );
  var exp = [ 0, { a : 0 } ];
  test.true( got === dst );
  test.identical( got.original, exp );

  /* */

  test.case = 'container, append primitive, duplicates';
  var dst = _.containerAdapter.make( [ 1, 2, 3 ] );
  var got = dst.append( 1 ).append( 2 );
  var exp = [ 1, 2, 3, 1, 2 ];
  test.true( got === dst );
  test.identical( got.original, exp );

  test.case = 'container, append Long, duplicates';
  var dst = _.containerAdapter.make( [ [ 1, 2 ] ] );
  var got = dst.append( [ 1, 2 ] ).append( [ 1, 2 ] );
  var exp = [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ] ];
  test.true( got === dst );
  test.identical( got.original, exp );

  test.case = 'container, append map, duplicates';
  var dst = _.containerAdapter.make( [ { a : 0 } ] );
  var got = dst.append( { a : 0 } ).append( { a : 0 } );
  var exp = [ { a : 0 }, { a : 0 }, { a : 0 } ];
  test.true( got === dst );
  test.identical( got.original, exp );
}

//

function arrayAdapterAppendOnce( test )
{
  test.open( 'without callbacks' );

  test.case = 'empty container, append primitive';
  var dst = _.containerAdapter.make( [] );
  var got = dst.appendOnce( 1 );
  var exp = [ 1 ];
  test.true( got === dst );
  test.identical( got.original, exp );

  test.case = 'empty container, append Long';
  var dst = _.containerAdapter.make( [] );
  var got = dst.appendOnce( _.unroll.make( [ 1, 2 ] ) );
  var exp = [ [ 1, 2 ] ];
  test.true( got === dst );
  test.identical( got.original, exp );

  test.case = 'empty container, append map';
  var dst = _.containerAdapter.make( [] );
  var got = dst.appendOnce( { a : 0 } );
  var exp = [ { a : 0 } ];
  test.true( got === dst );
  test.identical( got.original, exp );

  /* */

  test.case = 'container, append primitive';
  var dst = _.containerAdapter.make( [ undefined ] );
  var got = dst.appendOnce( 1 );
  var exp = [ undefined, 1 ];
  test.true( got === dst );
  test.identical( got.original, exp );

  test.case = 'container, append Long';
  var dst = _.containerAdapter.make( [ null ] );
  var got = dst.appendOnce( _.unroll.make( [ 1, 2 ] ) );
  var exp = [ null, [ 1, 2 ] ];
  test.true( got === dst );
  test.identical( got.original, exp );

  test.case = 'container, append map';
  var dst = _.containerAdapter.make( [ 0 ] );
  var got = dst.appendOnce( { a : 0 } );
  var exp = [ 0, { a : 0 } ];
  test.true( got === dst );
  test.identical( got.original, exp );

  /* */

  test.case = 'container, append primitive, duplicates';
  var dst = _.containerAdapter.make( [ 1, 2, 3 ] );
  var got = dst.appendOnce( 1 ).appendOnce( 2 );
  var exp = [ 1, 2, 3 ];
  test.true( got === dst );
  test.identical( got.original, exp );

  test.case = 'container, append Long, duplicates';
  var dst = _.containerAdapter.make( [ [ 1, 2 ] ] );
  var got = dst.appendOnce( [ 1, 2 ] ).appendOnce( [ 1, 2 ] );
  var exp = [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ] ];
  test.true( got === dst );
  test.identical( got.original, exp );

  test.case = 'container, append map, duplicates';
  var dst = _.containerAdapter.make( [ { a : 0 } ] );
  var got = dst.appendOnce( { a : 0 } ).appendOnce( { a : 0 } );
  var exp = [ { a : 0 }, { a : 0 }, { a : 0 } ];
  test.true( got === dst );
  test.identical( got.original, exp );

  test.close( 'without callbacks' );

  /* - */

  test.case = 'container append array, one evaluator, appends';
  var dst = _.containerAdapter.make( [ 1, [ 2 ], 3 ] );
  var got = dst.appendOnce( [ 1 ], ( e ) => e[ 0 ] );
  var exp = [ 1, [ 2 ], 3, [ 1 ] ];
  test.true( got === dst );
  test.identical( got.original, exp );

  test.case = 'container append array, one evaluator';
  var dst = _.containerAdapter.make( [ [ 1 ], [ 1 ], [ 2 ], 3 ] );
  var got = dst.appendOnce( [ 1 ], ( e ) => e[ 0 ] );
  var exp = [ [ 1 ], [ 1 ], [ 2 ], 3 ];
  test.true( got === dst );
  test.identical( got.original, exp );

  test.case = 'container append array, two evaluators';
  var dst = _.containerAdapter.make( [ 1, [ 2 ], 3 ] );
  var got = dst.appendOnce( [ 1 ], ( e ) => e, ( ins ) => ins[ 0 ] );
  var exp = [ 1, [ 2 ], 3 ];
  test.true( got === dst );
  test.identical( got.original, exp );

  test.case = 'container append array, two evaluators, appends';
  var dst = _.containerAdapter.make( [ [ 1 ], [ 1 ], [ 2 ], 3 ] );
  var got = dst.appendOnce( [ 1 ], ( e ) => e, ( ins ) => ins[ 0 ] );
  var exp = [ [ 1 ], [ 1 ], [ 2 ], 3, [ 1 ] ];
  test.true( got === dst );
  test.identical( got.original, exp );

  test.case = 'container append array, fromIndex and evaluator2, appends';
  var dst = _.containerAdapter.make( [ 1, [ 2 ], 3 ] );
  var got = dst.appendOnce( [ 1 ], 2, ( e ) => e[ 0 ] );
  var exp = [ 1, [ 2 ], 3, [ 1 ] ];
  test.true( got === dst );
  test.identical( got.original, exp );

  test.case = 'container append array, fromIndex and evaluator2';
  var dst = _.containerAdapter.make( [ [ 1 ], [ 1 ], [ 2 ], 3 ] );
  var got = dst.appendOnce( [ 1 ], 1, ( e ) => e[ 0 ] );
  var exp = [ [ 1 ], [ 1 ], [ 2 ], 3 ];
  test.true( got === dst );
  test.identical( got.original, exp );

  test.case = 'container append array, equalizer';
  var dst = _.containerAdapter.make( [ 1, [ 2 ], 3 ] );
  var got = dst.appendOnce( [ 1 ], ( e, ins ) => e === ins[ 0 ] );
  var exp = [ 1, [ 2 ], 3 ];
  test.true( got === dst );
  test.identical( got.original, exp );

  test.case = 'container append array, two evaluators, appends';
  var dst = _.containerAdapter.make( [ [ 1 ], [ 1 ], [ 2 ], 3 ] );
  var got = dst.appendOnce( [ 1 ], ( e, ins ) => e === ins[ 0 ] );
  var exp = [ [ 1 ], [ 1 ], [ 2 ], 3, [ 1 ] ];
  test.true( got === dst );
  test.identical( got.original, exp );
}

//

function arrayAdapterAppendOnceStrictly( test )
{
  test.open( 'without callbacks' );

  test.case = 'empty container, append primitive';
  var dst = _.containerAdapter.make( [] );
  var got = dst.appendOnceStrictly( 1 );
  var exp = [ 1 ];
  test.true( got === dst );
  test.identical( got.original, exp );

  test.case = 'empty container, append Long';
  var dst = _.containerAdapter.make( [] );
  var got = dst.appendOnceStrictly( _.unroll.make( [ 1, 2 ] ) );
  var exp = [ [ 1, 2 ] ];
  test.true( got === dst );
  test.identical( got.original, exp );

  test.case = 'empty container, append map';
  var dst = _.containerAdapter.make( [] );
  var got = dst.appendOnceStrictly( { a : 0 } );
  var exp = [ { a : 0 } ];
  test.true( got === dst );
  test.identical( got.original, exp );

  /* */

  test.case = 'container, append primitive';
  var dst = _.containerAdapter.make( [ undefined ] );
  var got = dst.appendOnceStrictly( 1 );
  var exp = [ undefined, 1 ];
  test.true( got === dst );
  test.identical( got.original, exp );

  test.case = 'container, append Long';
  var dst = _.containerAdapter.make( [ null ] );
  var got = dst.appendOnceStrictly( _.unroll.make( [ 1, 2 ] ) );
  var exp = [ null, [ 1, 2 ] ];
  test.true( got === dst );
  test.identical( got.original, exp );

  test.case = 'container, append map';
  var dst = _.containerAdapter.make( [ 0 ] );
  var got = dst.appendOnceStrictly( { a : 0 } );
  var exp = [ 0, { a : 0 } ];
  test.true( got === dst );
  test.identical( got.original, exp );

  /* */

  test.case = 'container, append primitive';
  var dst = _.containerAdapter.make( [ 1, 2, 3 ] );
  var got = dst.appendOnceStrictly( 4 ).appendOnceStrictly( 5 );
  var exp = [ 1, 2, 3, 4, 5 ];
  test.true( got === dst );
  test.identical( got.original, exp );

  test.case = 'container, append Long, duplicates';
  var dst = _.containerAdapter.make( [ [ 1, 2 ] ] );
  var got = dst.appendOnceStrictly( [ 1, 2 ] ).appendOnceStrictly( [ 1, 2 ] );
  var exp = [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ] ];
  test.true( got === dst );
  test.identical( got.original, exp );

  test.case = 'container, append map, duplicates';
  var dst = _.containerAdapter.make( [ { a : 0 } ] );
  var got = dst.appendOnceStrictly( { a : 0 } ).appendOnceStrictly( { a : 0 } );
  var exp = [ { a : 0 }, { a : 0 }, { a : 0 } ];
  test.true( got === dst );
  test.identical( got.original, exp );

  test.close( 'without callbacks' );

  /* - */

  test.case = 'container append array, one evaluator';
  var dst = _.containerAdapter.make( [ 1, [ 2 ], 3 ] );
  var got = dst.appendOnceStrictly( [ 1 ], ( e ) => e[ 0 ] );
  var exp = [ 1, [ 2 ], 3, [ 1 ] ];
  test.true( got === dst );
  test.identical( got.original, exp );

  test.case = 'container append array, one evaluator';
  var dst = _.containerAdapter.make( [ [ 1 ], [ 1 ], [ 2 ], 3 ] );
  var got = dst.appendOnceStrictly( [ 3 ], ( e ) => e[ 0 ] );
  var exp = [ [ 1 ], [ 1 ], [ 2 ], 3, [ 3 ] ];
  test.true( got === dst );
  test.identical( got.original, exp );

  test.case = 'container append array, two evaluators';
  var dst = _.containerAdapter.make( [ 1, [ 2 ], 3 ] );
  var got = dst.appendOnceStrictly( [ 1 ], ( e ) => e + 1, ( ins ) => ins[ 0 ] );
  var exp = [ 1, [ 2 ], 3, [ 1 ] ];
  test.true( got === dst );
  test.identical( got.original, exp );

  test.case = 'container append array, two evaluators';
  var dst = _.containerAdapter.make( [ [ 1 ], [ 1 ], [ 2 ], 3 ] );
  var got = dst.appendOnceStrictly( [ 1 ], ( e ) => e, ( ins ) => ins[ 0 ] );
  var exp = [ [ 1 ], [ 1 ], [ 2 ], 3, [ 1 ] ];
  test.true( got === dst );
  test.identical( got.original, exp );

  test.case = 'container append array, fromIndex and evaluator2';
  var dst = _.containerAdapter.make( [ 1, [ 2 ], 3 ] );
  var got = dst.appendOnceStrictly( [ 1 ], 2, ( e ) => e[ 0 ] );
  var exp = [ 1, [ 2 ], 3, [ 1 ] ];
  test.true( got === dst );
  test.identical( got.original, exp );

  test.case = 'container append array, fromIndex and evaluator2';
  var dst = _.containerAdapter.make( [ [ 1 ], [ 1 ], [ 2 ], 3 ] );
  var got = dst.appendOnceStrictly( [ 1 ], 2, ( e ) => e[ 0 ] );
  var exp = [ [ 1 ], [ 1 ], [ 2 ], 3, [ 1 ] ];
  test.true( got === dst );
  test.identical( got.original, exp );

  test.case = 'container append array, equalizer';
  var dst = _.containerAdapter.make( [ 1, [ 2 ], 3 ] );
  var got = dst.appendOnceStrictly( [ 1 ], ( e, ins ) => e + 1 === ins[ 0 ] );
  var exp = [ 1, [ 2 ], 3, [ 1 ] ];
  test.true( got === dst );
  test.identical( got.original, exp );

  test.case = 'container append array, equalizer';
  var dst = _.containerAdapter.make( [ [ 1 ], [ 1 ], [ 2 ], 3 ] );
  var got = dst.appendOnceStrictly( [ 1 ], ( e, ins ) => e === ins[ 0 ] );
  var exp = [ [ 1 ], [ 1 ], [ 2 ], 3, [ 1 ] ];
  test.true( got === dst );
  test.identical( got.original, exp );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'duplicate, no onEvaluate';
  test.shouldThrowErrorSync( () =>
  {
    var src = _.containerAdapter.make( [ 1, 2, 3 ] );
    src.appendOnceStrictly( 1 );
  });

  test.case = 'duplicate, onEvaluate';
  test.shouldThrowErrorSync( () =>
  {
    var src = _.containerAdapter.make( [ 1, 2, 3 ] );
    src.appendOnceStrictly( [ 1 ], ( e, ins ) => e === ins[ 0 ] );
  });

  test.case = 'wrong length of onEvaluate';
  test.shouldThrowErrorSync( () =>
  {
    var src = _.containerAdapter.make( [ 1, 2, 3 ] );
    src.appendOnceStrictly( [ 1 ], () => 1 );
  });
}

//

function arrayAdapterPush( test )
{
  test.case = 'empty container, push primitive';
  var dst = _.containerAdapter.make( [] );
  var got = dst.push( 1 );
  var exp = [ 1 ];
  test.identical( dst.original, exp );
  test.identical( got, 1 );

  test.case = 'empty container, push Long';
  var dst = _.containerAdapter.make( [] );
  var got = dst.push( _.unroll.make( [ 1, 2 ] ) );
  var exp = [ [ 1, 2 ] ];
  test.identical( dst.original, exp );
  test.identical( got, 1 );

  test.case = 'empty container, push map';
  var dst = _.containerAdapter.make( [] );
  var got = dst.push( { a : 0 } );
  var exp = [ { a : 0 } ];
  test.identical( dst.original, exp );
  test.identical( got, 1 );

  /* */

  test.case = 'container, push primitive';
  var dst = _.containerAdapter.make( [ undefined ] );
  var got = dst.push( 1 );
  var exp = [ undefined, 1 ];
  test.identical( dst.original, exp );
  test.identical( got, 2 );

  test.case = 'container, push Long';
  var dst = _.containerAdapter.make( [ null ] );
  var got = dst.push( _.unroll.make( [ 1, 2 ] ) );
  var exp = [ null, [ 1, 2 ] ];
  test.identical( dst.original, exp );
  test.identical( got, 2 );

  test.case = 'container, push map';
  var dst = _.containerAdapter.make( [ 0 ] );
  var got = dst.push( { a : 0 } );
  var exp = [ 0, { a : 0 } ];
  test.identical( dst.original, exp );
  test.identical( got, 2 );

  /* */

  test.case = 'container, push primitive, duplicates';
  var dst = _.containerAdapter.make( [ 1, 2, 3 ] );
  var got = dst.push( 1 );
  var exp = [ 1, 2, 3, 1 ];
  test.identical( dst.original, exp );
  test.identical( got, 4 );

  test.case = 'container, push Long, duplicates';
  var dst = _.containerAdapter.make( [ [ 1, 2 ] ] );
  var got = dst.push( [ 1, 2 ] );
  var exp = [ [ 1, 2 ], [ 1, 2 ] ];
  test.identical( dst.original, exp );
  test.identical( got, 2 );

  test.case = 'container, push map, duplicates';
  var dst = _.containerAdapter.make( [ { a : 0 } ] );
  var got = dst.push( { a : 0 } );
  var exp = [ { a : 0 }, { a : 0 } ];
  test.identical( dst.original, exp );
  test.identical( got, 2 );
}

//

function arrayAdapterAppendContainer( test )
{
  test.case = 'dst - empty container, append empty array';
  var dst = _.containerAdapter.make( [] );
  var src = [];
  var got = dst.appendContainer( src );
  var exp = [];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'dst - empty container, append empty BufferTyped';
  var dst = _.containerAdapter.make( [] );
  var src = new U32x();
  var got = dst.appendContainer( src );
  var exp = [];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'dst - empty container, append empty Set';
  var dst = _.containerAdapter.make( [] );
  var src = new Set();
  var got = dst.appendContainer( src );
  var exp = [];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'dst - empty container, append empty arrayContainerAdapter';
  var dst = _.containerAdapter.make( [] );
  var src = _.containerAdapter.make( [] );
  var got = dst.appendContainer( src );
  var exp = [];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'dst - empty container, append empty setContainerAdapter';
  var dst = _.containerAdapter.make( [] );
  var src = _.containerAdapter.make( new Set() );
  var got = dst.appendContainer( src );
  var exp = [];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  /* */

  test.case = 'append unroll';
  var dst = _.containerAdapter.make( [ 1, 2, 3 ] );
  var src = _.unroll.make( [ 1, { a : 2 }, [ 3 ], 'str', undefined ] );
  var got = dst.appendContainer( src );
  var exp = [ 1, 2, 3, 1, { a : 2 }, [ 3 ], 'str', undefined ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'append argumentsArray';
  var dst = _.containerAdapter.make( [ 1, 2, 3 ] );
  var src = _.argumentsArray.make( [ 1, { a : 2 }, [ 3 ], 'str', undefined ] );
  var got = dst.appendContainer( src );
  var exp = [ 1, 2, 3, 1, { a : 2 }, [ 3 ], 'str', undefined ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'append Set';
  var dst = _.containerAdapter.make( [ 1, 2, 3 ] );
  var src = new Set( [ 1, { a : 2 }, [ 3 ], 'str', undefined ] );
  var got = dst.appendContainer( src );
  var exp = [ 1, 2, 3, 1, { a : 2 }, [ 3 ], 'str', undefined ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'append arrayContainerAdapter';
  var dst = _.containerAdapter.make( [ 1, 2, 3 ] );
  var src = _.containerAdapter.make( [ 1, { a : 2 }, [ 3 ], 'str', undefined ] );
  var got = dst.appendContainer( src );
  var exp = [ 1, 2, 3, 1, { a : 2 }, [ 3 ], 'str', undefined ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'append setContainerAdapter';
  var dst = _.containerAdapter.make( [ 1, 2, 3 ] );
  var src = _.containerAdapter.make( new Set( [ 1, { a : 2 }, [ 3 ], 'str', undefined ] ) );
  var got = dst.appendContainer( src );
  var exp = [ 1, 2, 3, 1, { a : 2 }, [ 3 ], 'str', undefined ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  /* */

  test.case = 'append array, duplicates';
  var dst = _.containerAdapter.make( [ 1, 2, 3 ] );
  var src = [ 1, 2, 3, { a : 2 }, { a : 2 }, [ 3 ], [ 3 ], undefined, undefined, undefined ];
  var got = dst.appendContainer( src );
  var exp = [ 1, 2, 3, 1, 2, 3, { a : 2 }, { a : 2 }, [ 3 ], [ 3 ], undefined, undefined, undefined ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'append unroll';
  var dst = _.containerAdapter.make( [ 1, 2, 3 ] );
  var src = _.unroll.make( [ 1, 2, 3, { a : 2 }, { a : 2 }, [ 3 ], [ 3 ], undefined, undefined, undefined ] );
  var got = dst.appendContainer( src );
  var exp = [ 1, 2, 3, 1, 2, 3, { a : 2 }, { a : 2 }, [ 3 ], [ 3 ], undefined, undefined, undefined ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'append Set';
  var dst = _.containerAdapter.make( [ 1, 2, 3 ] );
  var src = new Set( [ 1, 2, 3, { a : 2 }, { a : 2 }, [ 3 ], [ 3 ], undefined, undefined, undefined ] );
  var got = dst.appendContainer( src );
  var exp = [ 1, 2, 3, 1, 2, 3, { a : 2 }, { a : 2 }, [ 3 ], [ 3 ], undefined ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'append arrayContainerAdapter';
  var dst = _.containerAdapter.make( [ 1, 2, 3 ] );
  var src = _.containerAdapter.make( [ 1, 2, 3, { a : 2 }, { a : 2 }, [ 3 ], [ 3 ], undefined, undefined, undefined ] );
  var got = dst.appendContainer( src );
  var exp = [ 1, 2, 3, 1, 2, 3, { a : 2 }, { a : 2 }, [ 3 ], [ 3 ], undefined, undefined, undefined ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'append setContainerAdapter';
  var dst = _.containerAdapter.make( [ 1, 2, 3 ] );
  var src = _.containerAdapter.make( new Set( [ 1, 2, 3, { a : 2 }, { a : 2 }, [ 3 ], [ 3 ], undefined, undefined, undefined ] ) );
  var got = dst.appendContainer( src );
  var exp = [ 1, 2, 3, 1, 2, 3, { a : 2 }, { a : 2 }, [ 3 ], [ 3 ], undefined ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () =>
  {
    var dst = _.containerAdapter.make( [] );
    dst.appendContainer();
  });

  test.case = 'wrong type of container';
  test.shouldThrowErrorSync( () =>
  {
    var dst = _.containerAdapter.make( [] );
    dst.appendContainer( 0 );
  });

  test.shouldThrowErrorSync( () =>
  {
    var dst = _.containerAdapter.make( [] );
    dst.appendContainer( new BufferRaw() );
  });

  test.shouldThrowErrorSync( () =>
  {
    var dst = _.containerAdapter.make( [] );
    dst.appendContainer( 'wrong' );
  });
}

//

function arrayAdapterAppendContainerOnce( test )
{
  test.case = 'dst - empty container, append empty array';
  var dst = _.containerAdapter.make( [] );
  var src = [];
  var got = dst.appendContainerOnce( src );
  var exp = [];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'dst - empty container, append empty BufferTyped';
  var dst = _.containerAdapter.make( [] );
  var src = new U32x();
  var got = dst.appendContainerOnce( src );
  var exp = [];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'dst - empty container, append empty Set';
  var dst = _.containerAdapter.make( [] );
  var src = new Set();
  var got = dst.appendContainerOnce( src );
  var exp = [];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'dst - empty container, append empty arrayContainerAdapter';
  var dst = _.containerAdapter.make( [] );
  var src = _.containerAdapter.make( [] );
  var got = dst.appendContainerOnce( src );
  var exp = [];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'dst - empty container, append empty setContainerAdapter';
  var dst = _.containerAdapter.make( [] );
  var src = _.containerAdapter.make( new Set() );
  var got = dst.appendContainerOnce( src );
  var exp = [];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  /* */

  test.case = 'append unroll';
  var dst = _.containerAdapter.make( [ 1, 2, 3 ] );
  var src = _.unroll.make( [ 1, { a : 2 }, [ 3 ], 'str', undefined ] );
  var got = dst.appendContainerOnce( src );
  var exp = [ 1, 2, 3, { a : 2 }, [ 3 ], 'str', undefined ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'append argumentsArray';
  var dst = _.containerAdapter.make( [ 1, 2, 3 ] );
  var src = _.argumentsArray.make( [ 1, { a : 2 }, [ 3 ], 'str', undefined ] );
  var got = dst.appendContainerOnce( src );
  var exp = [ 1, 2, 3, { a : 2 }, [ 3 ], 'str', undefined ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'append Set';
  var dst = _.containerAdapter.make( [ 1, 2, 3 ] );
  var src = new Set( [ 1, { a : 2 }, [ 3 ], 'str', undefined ] );
  var got = dst.appendContainerOnce( src );
  var exp = [ 1, 2, 3, { a : 2 }, [ 3 ], 'str', undefined ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'append arrayContainerAdapter';
  var dst = _.containerAdapter.make( [ 1, 2, 3 ] );
  var src = _.containerAdapter.make( [ 1, { a : 2 }, [ 3 ], 'str', undefined ] );
  var got = dst.appendContainerOnce( src );
  var exp = [ 1, 2, 3, { a : 2 }, [ 3 ], 'str', undefined ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'append setContainerAdapter';
  var dst = _.containerAdapter.make( [ 1, 2, 3 ] );
  var src = _.containerAdapter.make( new Set( [ 1, { a : 2 }, [ 3 ], 'str', undefined ] ) );
  var got = dst.appendContainerOnce( src );
  var exp = [ 1, 2, 3, { a : 2 }, [ 3 ], 'str', undefined ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  /* */

  test.case = 'append array, duplicates';
  var dst = _.containerAdapter.make( [ 1, 2, 3 ] );
  var src = [ 1, 2, 3, { a : 2 }, { a : 2 }, [ 3 ], [ 3 ], undefined, undefined, undefined ];
  var got = dst.appendContainerOnce( src );
  var exp = [ 1, 2, 3, { a : 2 }, { a : 2 }, [ 3 ], [ 3 ], undefined ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'append unroll';
  var dst = _.containerAdapter.make( [ 1, 2, 3 ] );
  var src = _.unroll.make( [ 1, 2, 3, { a : 2 }, { a : 2 }, [ 3 ], [ 3 ], undefined, undefined, undefined ] );
  var got = dst.appendContainerOnce( src );
  var exp = [ 1, 2, 3, { a : 2 }, { a : 2 }, [ 3 ], [ 3 ], undefined ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'append Set';
  var dst = _.containerAdapter.make( [ 1, 2, 3 ] );
  var src = new Set( [ 1, 2, 3, { a : 2 }, { a : 2 }, [ 3 ], [ 3 ], undefined, undefined, undefined ] );
  var got = dst.appendContainerOnce( src );
  var exp = [ 1, 2, 3, { a : 2 }, { a : 2 }, [ 3 ], [ 3 ], undefined ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'append arrayContainerAdapter';
  var dst = _.containerAdapter.make( [ 1, 2, 3 ] );
  var src = _.containerAdapter.make( [ 1, 2, 3, { a : 2 }, { a : 2 }, [ 3 ], [ 3 ], undefined, undefined, undefined ] );
  var got = dst.appendContainerOnce( src );
  var exp = [ 1, 2, 3, { a : 2 }, { a : 2 }, [ 3 ], [ 3 ], undefined ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'append setContainerAdapter';
  var dst = _.containerAdapter.make( [ 1, 2, 3 ] );
  var src = _.containerAdapter.make( new Set( [ 1, 2, 3, { a : 2 }, { a : 2 }, [ 3 ], [ 3 ], undefined, undefined, undefined ] ) );
  var got = dst.appendContainerOnce( src );
  var exp = [ 1, 2, 3, { a : 2 }, { a : 2 }, [ 3 ], [ 3 ], undefined ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  /* */

  test.case = 'append array, one evaluator';
  var dst = _.containerAdapter.make( [ 1, 2, 3 ] );
  var src = _.containerAdapter.make( [ 1, 2, 3, { a : 2 }, { a : 2 }, [ 3 ], [ 3 ], [ undefined ], [ undefined ], [ undefined ] ] );
  var got = dst.appendContainerOnce( src, ( e ) => e[ 0 ] );
  var exp = [ 1, 2, 3, [ 3 ] ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'dst - container, append array, two evaluators';
  var dst = _.containerAdapter.make( [ 1, 2, 3 ] );
  var src = _.containerAdapter.make( [ 1, 2, 3, { a : 2 }, { a : 2 }, [ 3 ], [ 3 ], [ undefined ], [ undefined ], [ undefined ] ] );
  var got = dst.appendContainerOnce( src, ( e ) => e, ( ins ) => ins[ 0 ] );
  var exp = [ 1, 2, 3, 1, 2, 3, { a : 2 }, { a : 2 }, [ undefined ], [ undefined ], [ undefined ] ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'dst - container, append array, fromIndex and evaluator2';
  var dst = _.containerAdapter.make( [ 1, 2, 3 ] );
  var src = _.containerAdapter.make( [ 1, 2, 3, { a : 2 }, { a : 2 }, [ 3 ], [ 3 ], [ undefined ], [ undefined ], [ undefined ] ] );
  var got = dst.appendContainerOnce( src, 3, ( e ) => e.a );
  var exp = [ 1, 2, 3, 1, { a : 2 } ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );


  test.case = 'dst - container, append array, equalizer';
  var dst = _.containerAdapter.make( [ 1, 2, 3 ] );
  var src = _.containerAdapter.make( [ 1, 2, 3, { a : 2 }, { a : 2 }, [ 3 ], [ 3 ], [ undefined ], [ undefined ], [ undefined ] ] );
  var got = dst.appendContainerOnce( src, ( e, ins ) => e === ins[ 0 ] );
  var exp = [ 1, 2, 3, 1, 2, 3, { a : 2 }, { a : 2 }, [ undefined ], [ undefined ], [ undefined ] ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  /* */

  test.case = 'append setAdapter, one evaluator';
  var dst = _.containerAdapter.make( [ 1, 2, 3 ] );
  var src = _.containerAdapter.make( new Set( [ 1, 2, 3, { a : 2 }, { a : 2 }, [ 3 ], [ 3 ], [ undefined ], [ undefined ], [ undefined ] ] ) );
  var got = dst.appendContainerOnce( src, ( e ) => e[ 0 ] );
  var exp = [ 1, 2, 3, [ 3 ] ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'dst - container, append setAdapter, two evaluators';
  var dst = _.containerAdapter.make( [ 1, 2, 3 ] );
  var src = _.containerAdapter.make( new Set( [ 1, 2, 3, { a : 2 }, { a : 2 }, [ 3 ], [ 3 ], [ undefined ], [ undefined ], [ undefined ] ] ) );
  var got = dst.appendContainerOnce( src, ( e ) => e, ( ins ) => ins[ 0 ] );
  var exp = [ 1, 2, 3, 1, 2, 3, { a : 2 }, { a : 2 }, [ undefined ], [ undefined ], [ undefined ] ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'dst - container, append setAdapter, fromIndex and evaluator2';
  var dst = _.containerAdapter.make( [ 1, 2, 3 ] );
  var src = _.containerAdapter.make( new Set( [ 1, 2, 3, { a : 2 }, { a : 2 }, [ 3 ], [ 3 ], [ undefined ], [ undefined ], [ undefined ] ] ) );
  var got = dst.appendContainerOnce( src, 3, ( e ) => e.a );
  var exp = [ 1, 2, 3, 1, { a : 2 } ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );


  test.case = 'dst - container, append setAdapter, equalizer';
  var dst = _.containerAdapter.make( [ 1, 2, 3 ] );
  var src = _.containerAdapter.make( new Set( [ 1, 2, 3, { a : 2 }, { a : 2 }, [ 3 ], [ 3 ], [ undefined ], [ undefined ], [ undefined ] ] ) );
  var got = dst.appendContainerOnce( src, ( e, ins ) => e === ins[ 0 ] );
  var exp = [ 1, 2, 3, 1, 2, 3, { a : 2 }, { a : 2 }, [ undefined ], [ undefined ], [ undefined ] ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () =>
  {
    var dst = _.containerAdapter.make( [] );
    dst.appendContainerOnce();
  });

  test.case = 'wrong type of container';
  test.shouldThrowErrorSync( () =>
  {
    var dst = _.containerAdapter.make( [] );
    dst.appendContainerOnce( 0 );
  });

  test.shouldThrowErrorSync( () =>
  {
    var dst = _.containerAdapter.make( [] );
    dst.appendContainerOnce( new BufferRaw() );
  });

  test.shouldThrowErrorSync( () =>
  {
    var dst = _.containerAdapter.make( [] );
    dst.appendContainerOnce( 'wrong' );
  });
}

//

function arrayAdapterAppendContainerOnceStrictly( test )
{
  test.case = 'dst - empty container, append empty array';
  var dst = _.containerAdapter.make( [] );
  var src = [];
  var got = dst.appendContainerOnceStrictly( src );
  var exp = [];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'dst - empty container, append empty BufferTyped';
  var dst = _.containerAdapter.make( [] );
  var src = new U32x();
  var got = dst.appendContainerOnceStrictly( src );
  var exp = [];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'dst - empty container, append empty Set';
  var dst = _.containerAdapter.make( [] );
  var src = new Set();
  var got = dst.appendContainerOnceStrictly( src );
  var exp = [];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'dst - empty container, append empty arrayContainerAdapter';
  var dst = _.containerAdapter.make( [] );
  var src = _.containerAdapter.make( [] );
  var got = dst.appendContainerOnceStrictly( src );
  var exp = [];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'dst - empty container, append empty setContainerAdapter';
  var dst = _.containerAdapter.make( [] );
  var src = _.containerAdapter.make( new Set() );
  var got = dst.appendContainerOnceStrictly( src );
  var exp = [];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  /* */

  test.case = 'append unroll';
  var dst = _.containerAdapter.make( [ 1, 2, 3 ] );
  var src = _.unroll.make( [ 4, { a : 2 }, [ 3 ], 'str', undefined ] );
  var got = dst.appendContainerOnceStrictly( src );
  var exp = [ 1, 2, 3, 4, { a : 2 }, [ 3 ], 'str', undefined ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'append argumentsArray';
  var dst = _.containerAdapter.make( [ 1, 2, 3 ] );
  var src = _.argumentsArray.make( [ 4, { a : 2 }, [ 3 ], 'str', undefined ] );
  var got = dst.appendContainerOnceStrictly( src );
  var exp = [ 1, 2, 3, 4, { a : 2 }, [ 3 ], 'str', undefined ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'append Set';
  var dst = _.containerAdapter.make( [ 1, 2, 3 ] );
  var src = new Set( [ 4, { a : 2 }, [ 3 ], 'str', undefined ] );
  var got = dst.appendContainerOnceStrictly( src );
  var exp = [ 1, 2, 3, 4, { a : 2 }, [ 3 ], 'str', undefined ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'append arrayContainerAdapter';
  var dst = _.containerAdapter.make( [ 1, 2, 3 ] );
  var src = _.containerAdapter.make( [ 4, { a : 2 }, [ 3 ], 'str', undefined ] );
  var got = dst.appendContainerOnceStrictly( src );
  var exp = [ 1, 2, 3, 4, { a : 2 }, [ 3 ], 'str', undefined ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'append setContainerAdapter';
  var dst = _.containerAdapter.make( [ 1, 2, 3 ] );
  var src = _.containerAdapter.make( new Set( [ 4, { a : 2 }, [ 3 ], 'str', undefined ] ) );
  var got = dst.appendContainerOnceStrictly( src );
  var exp = [ 1, 2, 3, 4, { a : 2 }, [ 3 ], 'str', undefined ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  /* */

  test.case = 'append arrayAdapter, one evaluator';
  var dst = _.containerAdapter.make( [ [ 1 ], [ 2 ], [ 3 ] ] );
  var src = _.containerAdapter.make( [ 1, [ 4 ], [ 5 ], [ null ] ] );
  var got = dst.appendContainerOnceStrictly( src, ( e ) => e[ 0 ] );
  var exp = [ [ 1 ], [ 2 ], [ 3 ], 1, [ 4 ], [ 5 ], [ null ] ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'dst - container, append arrayAdapter, two evaluators';
  var dst = _.containerAdapter.make( [ [ 1 ], [ 2 ], [ 3 ] ] );
  var src = _.containerAdapter.make( [ 1, 2, 3, { a : 2 }, { a : 2 }, [ 4 ], [ 5 ], [ undefined ] ] );
  var got = dst.appendContainerOnceStrictly( src, ( e ) => e, ( ins ) => ins[ 0 ] );
  var exp = [ [ 1 ], [ 2 ], [ 3 ], 1, 2, 3, { a : 2 }, { a : 2 }, [ 4 ], [ 5 ], [ undefined ] ]
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'dst - container, append arrayAdapter, fromIndex and evaluator2';
  var dst = _.containerAdapter.make( [ [ 1 ], [ 2 ], [ 3 ] ] );
  var src = _.containerAdapter.make( [ { a : 2 }, { a : 3 } ] );
  var got = dst.appendContainerOnceStrictly( src, 2, ( e ) => e.a );
  var exp = [ [ 1 ], [ 2 ], [ 3 ], { a : 2 }, { a : 3 } ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'dst - container, append arrayAdapter, equalizer';
  var dst = _.containerAdapter.make( [ [ 1 ], [ 2 ], [ 3 ] ] );
  var src = _.containerAdapter.make( [ 1, 2, 3, { a : 2 }, { a : 2 }, [ 4 ], [ 5 ], [ undefined ] ] );
  var got = dst.appendContainerOnceStrictly( src, ( e, ins ) => e === ins[ 0 ] );
  var exp = [ [ 1 ], [ 2 ], [ 3 ], 1, 2, 3, { a : 2 }, { a : 2 }, [ 4 ], [ 5 ], [ undefined ] ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  /* */

  test.case = 'append setAdapter, one evaluator';
  var dst = _.containerAdapter.make( [ [ 1 ], [ 2 ], [ 3 ] ] );
  var src = _.containerAdapter.make( new Set( [ 1, [ 4 ], [ 5 ], [ null ] ] ) );
  var got = dst.appendContainerOnceStrictly( src, ( e ) => e[ 0 ] );
  var exp = [ [ 1 ], [ 2 ], [ 3 ], 1, [ 4 ], [ 5 ], [ null ] ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'dst - container, append setAdapter, two evaluators';
  var dst = _.containerAdapter.make( [ [ 1 ], [ 2 ], [ 3 ] ] );
  var src = _.containerAdapter.make( new Set( [ 1, 2, 3, { a : 2 }, { a : 2 }, [ 4 ], [ 5 ], [ undefined ] ] ) );
  var got = dst.appendContainerOnceStrictly( src, ( e ) => e, ( ins ) => ins[ 0 ] );
  var exp = [ [ 1 ], [ 2 ], [ 3 ], 1, 2, 3, { a : 2 }, { a : 2 }, [ 4 ], [ 5 ], [ undefined ] ]
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'dst - container, append setAdapter, fromIndex and evaluator2';
  var dst = _.containerAdapter.make( [ [ 1 ], [ 2 ], [ 3 ] ] );
  var src = _.containerAdapter.make( new Set( [ { a : 2 }, { a : 3 } ] ) );
  var got = dst.appendContainerOnceStrictly( src, 2, ( e ) => e.a );
  var exp = [ [ 1 ], [ 2 ], [ 3 ], { a : 2 }, { a : 3 } ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'dst - container, append setAdapter, equalizer';
  var dst = _.containerAdapter.make( [ [ 1 ], [ 2 ], [ 3 ] ] );
  var src = _.containerAdapter.make( new Set( [ 1, 2, 3, { a : 2 }, { a : 2 }, [ 4 ], [ 5 ], [ undefined ] ] ) );
  var got = dst.appendContainerOnceStrictly( src, ( e, ins ) => e === ins[ 0 ] );
  var exp = [ [ 1 ], [ 2 ], [ 3 ], 1, 2, 3, { a : 2 }, { a : 2 }, [ 4 ], [ 5 ], [ undefined ] ];
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () =>
  {
    var dst = _.containerAdapter.make( [] );
    dst.appendContainerOnceStrictly();
  });

  test.case = 'wrong type of container';
  test.shouldThrowErrorSync( () =>
  {
    var dst = _.containerAdapter.make( [] );
    dst.appendContainerOnceStrictly( 0 );
  });

  test.shouldThrowErrorSync( () =>
  {
    var dst = _.containerAdapter.make( [] );
    dst.appendContainerOnceStrictly( new BufferRaw() );
  });

  test.shouldThrowErrorSync( () =>
  {
    var dst = _.containerAdapter.make( [] );
    dst.appendContainerOnceStrictly( 'wrong' );
  });

  test.case = 'duplicates in container';
  test.shouldThrowErrorSync( () =>
  {
    var dst = _.containerAdapter.make( [] );
    var src = [ 1, 2, 1, 2 ];
    dst.appendContainerOnceStrictly( src );
  });

  test.case = 'evaluator makes undefined from primitives';
  test.shouldThrowErrorSync( () =>
  {
    var dst = _.containerAdapter.make([ [ 1 ] ] );
    var src = [ [ 3 ], [ 4 ], 1, 2 ];
    dst.appendContainerOnceStrictly( src, ( e ) => e[ 0 ] );
  });
}

//

function arrayAdapterPop( test )
{
  test.case = 'empty container, without argument';
  var src = _.containerAdapter.make( [] );
  var got = src.pop();
  test.identical( src.original, [] );
  test.identical( got, undefined );

  test.case = 'container, last element is undefined, without arguments';
  var src = _.containerAdapter.make( [ null, 1, 'str', undefined ] );
  var got = src.pop();
  test.identical( src.original, [ null, 1, 'str' ] );
  test.identical( got, undefined );

  test.case = 'container, last element === searched element';
  var src = _.containerAdapter.make( [ null, 1, 'str' ] );
  var got = src.pop( 'str' );
  test.identical( src.original, [ null, 1 ] );
  test.identical( got, 'str' );

  test.case = 'container, last element - complex data, one evaluator';
  var src = _.containerAdapter.make( [ null, 1, 'str', [ 1 ] ] );
  var got = src.pop( [ 1 ], ( e ) => e[ 0 ] );
  test.identical( src.original, [ null, 1, 'str' ] );
  test.identical( got, [ 1 ] );

  test.case = 'container, last element - complex data, two evaluators';
  var src = _.containerAdapter.make( [ null, 1, 'str', [ 1 ] ] );
  var got = src.pop( 1, ( e ) => e[ 0 ], ( ins ) => ins );
  test.identical( src.original, [ null, 1, 'str' ] );
  test.identical( got, [ 1 ] );

  test.case = 'container, last element - complex data, equalizer';
  var src = _.containerAdapter.make( [ null, 1, 'str', [ 1 ] ] );
  var got = src.pop( 1, ( e, ins ) => e[ 0 ] === ins );
  test.identical( src.original, [ null, 1, 'str' ] );
  test.identical( got, [ 1 ] );

  if( !Config.debug )
  return;

  test.case = 'empty container, pop element';
  test.shouldThrowErrorSync( () =>
  {
    var src = _.containerAdapter.make( [] );
    src.pop( 2 );
  });

  test.case = 'popped element !== searched element';
  test.shouldThrowErrorSync( () =>
  {
    var src = _.containerAdapter.make( [ null, 1, 'str', undefined ] );
    src.pop( 'str' );
  });

  test.case = 'complex data';
  test.shouldThrowErrorSync( () =>
  {
    var src = _.containerAdapter.make( [ [ 1 ], [ 0 ], [ 1 ] ] );
    src.pop( [ 1 ] );
  });
}

//

function arrayAdapterPopStrictly( test )
{
  test.case = 'container, last element === searched element';
  var src = _.containerAdapter.make( [ null, 1, 'str' ] );
  var got = src.popStrictly( 'str' );
  test.identical( src.original, [ null, 1 ] );
  test.identical( got, 'str' );

  test.case = 'container, last element - complex data, one evaluator';
  var src = _.containerAdapter.make( [ null, 1, 'str', [ 1 ] ] );
  var got = src.popStrictly( [ 1 ], ( e ) => e[ 0 ] );
  test.identical( src.original, [ null, 1, 'str' ] );
  test.identical( got, [ 1 ] );

  test.case = 'container, last element - complex data, two evaluators';
  var src = _.containerAdapter.make( [ null, 1, 'str', [ 1 ] ] );
  var got = src.popStrictly( 1, ( e ) => e[ 0 ], ( ins ) => ins );
  test.identical( src.original, [ null, 1, 'str' ] );
  test.identical( got, [ 1 ] );

  test.case = 'container, last element - complex data, equalizer';
  var src = _.containerAdapter.make( [ null, 1, 'str', [ 1 ] ] );
  var got = src.popStrictly( 1, ( e, ins ) => e[ 0 ] === ins );
  test.identical( src.original, [ null, 1, 'str' ] );
  test.identical( got, [ 1 ] );

  if( !Config.debug )
  return;

  test.case = 'without argument';
  test.shouldThrowErrorSync( () =>
  {
    var src = _.containerAdapter.make( [] );
    src.popStrictly();
  });

  test.case = 'empty container, pop element';
  test.shouldThrowErrorSync( () =>
  {
    var src = _.containerAdapter.make( [] );
    src.popStrictly( 2 );
  });

  test.case = 'popped element !== searched element';
  test.shouldThrowErrorSync( () =>
  {
    var src = _.containerAdapter.make( [ null, 1, 'str', undefined ] );
    src.popStrictly( 'str' );
  });

  test.case = 'complex data';
  test.shouldThrowErrorSync( () =>
  {
    var src = _.containerAdapter.make( [ [ 1 ], [ 0 ], [ 1 ] ] );
    src.popStrictly( [ 1 ] );
  });
}

//

function arrayAdapterRemoved( test )
{
  test.case = 'empty container, remove primitive';
  var dst = _.containerAdapter.make( [] );
  var got = dst.removed( 1 );
  var exp = [];
  test.identical( got, 0 );
  test.identical( dst.original, exp );

  test.case = 'empty container, remove Long';
  var dst = _.containerAdapter.make( [] );
  var got = dst.removed( _.unroll.make( [ 1, 2 ] ) );
  var exp = [];
  test.identical( got, 0 );
  test.identical( dst.original, exp );

  test.case = 'empty container, remove map';
  var dst = _.containerAdapter.make( [] );
  var got = dst.removed( { a : 0 } );
  var exp = [];
  test.identical( got, 0 );
  test.identical( dst.original, exp );

  /* */

  test.case = 'container, remove primitive';
  var dst = _.containerAdapter.make( [ 1, 1, 2, 2, '1' ] );
  var got = dst.removed( 1 );
  var exp = [ 2, 2, '1' ];
  test.identical( got, 2 );
  test.identical( dst.original, exp );

  test.case = 'container, remove Long';
  var dst = _.containerAdapter.make( [ 1, 1, 2, 2, '1' ] );
  var got = dst.removed( _.unroll.make( [ 1, 2 ] ) );
  var exp = [ 1, 1, 2, 2, '1' ];
  test.identical( got, 0 );
  test.identical( dst.original, exp );

  test.case = 'container, remove map';
  var dst = _.containerAdapter.make( [ 1, 1, 2, 2, '1' ] );
  var got = dst.removed( { a : 1 } );
  var exp = [ 1, 1, 2, 2, '1' ];
  test.identical( got, 0 );
  test.identical( dst.original, exp );

  /* */

  test.case = 'container, remove primitive, duplicates';
  var dst = _.containerAdapter.make( [ 1, 2, 3 ] );
  var got = dst.removed( 1 );
  var exp = [ 2, 3 ];
  test.identical( got, 1 );
  test.identical( dst.original, exp );

  test.case = 'container, remove Long, duplicates';
  var dst = _.containerAdapter.make( [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ] );
  var got = dst.removed( [ 1, 2 ] );
  var exp = [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ];
  test.identical( got, 0 );
  test.identical( dst.original, exp );

  test.case = 'container, remove map, duplicates';
  var dst = _.containerAdapter.make( [ { a : 0 }, { a : 0 }, { a : 0 }, { a : 1 } ] );
  var got = dst.removed( { a : 0 } );
  var exp = [ { a : 0 }, { a : 0 }, { a : 0 }, { a : 1 } ];
  test.identical( got, 0 );
  test.identical( dst.original, exp );

  /* */

  test.case = 'container, remove Long, one evaluator';
  var dst = _.containerAdapter.make( [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ] );
  var got = dst.removed( [ 1, 2 ], ( e ) => e[ 0 ] );
  var exp = [ [ 2, 2 ] ];
  test.identical( got, 3 );
  test.identical( dst.original, exp );

  test.case = 'container, remove Long, two evaluators';
  var dst = _.containerAdapter.make( [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ] );
  var got = dst.removed( [ 1, 2 ], ( e ) => e[ 0 ], ( ins ) => ins[ 0 ] );
  var exp = [ [ 2, 2 ] ];
  test.identical( got, 3 );
  test.identical( dst.original, exp );

  test.case = 'container, remove Long, fromIndex and evaluator2';
  var dst = _.containerAdapter.make( [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ] );
  var got = dst.removed( [ 1, 2 ], 2, ( e ) => e[ 0 ] );
  var exp = [ [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ];
  test.identical( got, 1 );
  test.identical( dst.original, exp );

  test.case = 'container, remove Long, equalizer';
  var dst = _.containerAdapter.make( [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ] );
  var got = dst.removed( [ 1, 2 ], ( e, ins ) => e === ins[ 0 ] );
  var exp = [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ];
  test.identical( got, 0 );
  test.identical( dst.original, exp );
}

//

function arrayAdapterRemovedOnce( test )
{
  test.case = 'empty container, remove primitive';
  var dst = _.containerAdapter.make( [] );
  var got = dst.removedOnce( 1 );
  var exp = [];
  test.identical( got, -1 );
  test.identical( dst.original, exp );

  test.case = 'empty container, remove Long';
  var dst = _.containerAdapter.make( [] );
  var got = dst.removedOnce( _.unroll.make( [ 1, 2 ] ) );
  var exp = [];
  test.identical( got, -1 );
  test.identical( dst.original, exp );

  test.case = 'empty container, remove map';
  var dst = _.containerAdapter.make( [] );
  var got = dst.removedOnce( { a : 0 } );
  var exp = [];
  test.identical( got, -1 );
  test.identical( dst.original, exp );

  /* */

  test.case = 'container, remove primitive';
  var dst = _.containerAdapter.make( [ 1, 1, 2, 2, '1' ] );
  var got = dst.removedOnce( 1 );
  var exp = [ 1, 2, 2, '1' ];
  test.identical( got, 0 );
  test.identical( dst.original, exp );

  test.case = 'container, remove Long';
  var dst = _.containerAdapter.make( [ 1, 1, 2, 2, '1' ] );
  var got = dst.removedOnce( _.unroll.make( [ 1, 2 ] ) );
  var exp = [ 1, 1, 2, 2, '1' ];
  test.identical( got, -1 );
  test.identical( dst.original, exp );

  test.case = 'container, remove map';
  var dst = _.containerAdapter.make( [ 1, 1, 2, 2, '1' ] );
  var got = dst.removedOnce( { a : 1 } );
  var exp = [ 1, 1, 2, 2, '1' ];
  test.identical( got, -1 );
  test.identical( dst.original, exp );

  /* */

  test.case = 'container, remove primitive, duplicates';
  var dst = _.containerAdapter.make( [ 1, 1, 2, 2, 3, 3 ] );
  var got = dst.removedOnce( 1 );
  var exp = [ 1, 2, 2, 3, 3 ];
  test.identical( got, 0 );
  test.identical( dst.original, exp );

  test.case = 'container, remove Long, duplicates';
  var dst = _.containerAdapter.make( [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ] );
  var got = dst.removedOnce( [ 1, 2 ] );
  var exp = [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ];
  test.identical( got, -1 );
  test.identical( dst.original, exp );

  test.case = 'container, remove map, duplicates';
  var dst = _.containerAdapter.make( [ { a : 0 }, { a : 0 }, { a : 0 }, { a : 1 } ] );
  var got = dst.removedOnce( { a : 0 } );
  var exp = [ { a : 0 }, { a : 0 }, { a : 0 }, { a : 1 } ];
  test.identical( got, -1 );
  test.identical( dst.original, exp );

  /* */

  test.case = 'container, remove Long, one evaluator';
  var dst = _.containerAdapter.make( [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ] );
  var got = dst.removedOnce( [ 1, 2 ], ( e ) => e[ 0 ] );
  var exp = [ [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ];
  test.identical( got, 0 );
  test.identical( dst.original, exp );

  test.case = 'container, remove Long, two evaluators';
  var dst = _.containerAdapter.make( [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ] );
  var got = dst.removedOnce( [ 1, 2 ], ( e ) => e[ 0 ], ( ins ) => ins[ 0 ] );
  var exp = [ [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ];
  test.identical( got, 0 );
  test.identical( dst.original, exp );

  test.case = 'container, remove Long, fromIndex and evaluator2';
  var dst = _.containerAdapter.make( [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ] );
  var got = dst.removedOnce( [ 1, 2 ], 2, ( e ) => e[ 0 ] );
  var exp = [ [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ];
  test.identical( got, 2 );
  test.identical( dst.original, exp );

  test.case = 'container, remove Long, equalizer';
  var dst = _.containerAdapter.make( [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ] );
  var got = dst.removedOnce( [ 1, 2 ], ( e, ins ) => e === ins[ 0 ] );
  var exp = [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ];
  test.identical( got, -1 );
  test.identical( dst.original, exp );
}

//

function arrayAdapterRemovedOnceRight( test )
{
  test.case = 'empty container, remove primitive';
  var dst = _.containerAdapter.make( [] );
  var got = dst.removedOnceRight( 1 );
  var exp = [];
  test.identical( got, -1 );
  test.identical( dst.original, exp );

  test.case = 'empty container, remove Long';
  var dst = _.containerAdapter.make( [] );
  var got = dst.removedOnceRight( _.unroll.make( [ 1, 2 ] ) );
  var exp = [];
  test.identical( got, -1 );
  test.identical( dst.original, exp );

  test.case = 'empty container, remove empty Long';
  var dst = _.containerAdapter.make( [] );
  var got = dst.removedOnceRight( [] );
  var exp = [];
  test.identical( got, -1 );
  test.identical( dst.original, exp );

  test.case = 'empty container, remove map';
  var dst = _.containerAdapter.make( [] );
  var got = dst.removedOnceRight( { a : 0 } );
  var exp = [];
  test.identical( got, -1 );
  test.identical( dst.original, exp );

  /* */

  test.case = 'container, remove primitive';
  var dst = _.containerAdapter.make( [ 1, 1, 2, 2, '1' ] );
  var got = dst.removedOnceRight( 1 );
  var exp = [ 1, 2, 2, '1' ];
  test.identical( got, 1 );
  test.identical( dst.original, exp );

  test.case = 'container, remove Long';
  var dst = _.containerAdapter.make( [ 1, 1, 2, 2, '1' ] );
  var got = dst.removedOnceRight( _.unroll.make( [ 1, 2 ] ) );
  var exp = [ 1, 1, 2, 2, '1' ];
  test.identical( got, -1 );
  test.identical( dst.original, exp );

  test.case = 'container, remove map';
  var dst = _.containerAdapter.make( [ 1, 1, 2, 2, '1' ] );
  var got = dst.removedOnceRight( { a : 1 } );
  var exp = [ 1, 1, 2, 2, '1' ];
  test.identical( got, -1 );
  test.identical( dst.original, exp );

  /* */

  test.case = 'container, remove primitive, duplicates';
  var dst = _.containerAdapter.make( [ 1, 1, 2, 2, 3, 3 ] );
  var got = dst.removedOnceRight( 1 );
  var exp = [ 1, 2, 2, 3, 3 ];
  test.identical( got, 1 );
  test.identical( dst.original, exp );

  test.case = 'container, remove Long, duplicates';
  var dst = _.containerAdapter.make( [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ] );
  var got = dst.removedOnceRight( [ 1, 2 ] );
  var exp = [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ];
  test.identical( got, -1 );
  test.identical( dst.original, exp );

  test.case = 'container, remove map, duplicates';
  var dst = _.containerAdapter.make( [ { a : 0 }, { a : 0 }, { a : 0 }, { a : 1 } ] );
  var got = dst.removedOnceRight( { a : 0 } );
  var exp = [ { a : 0 }, { a : 0 }, { a : 0 }, { a : 1 } ];
  test.identical( got, -1 );
  test.identical( dst.original, exp );

  /* */

  test.case = 'container, remove Long, one evaluator';
  var dst = _.containerAdapter.make( [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ] );
  var got = dst.removedOnceRight( [ 1, 2 ], ( e ) => e[ 0 ] );
  var exp = [ [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ];
  test.identical( got, 2 );
  test.identical( dst.original, exp );

  test.case = 'container, remove Long, two evaluators';
  var dst = _.containerAdapter.make( [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ] );
  var got = dst.removedOnceRight( [ 1, 2 ], ( e ) => e[ 0 ], ( ins ) => ins[ 0 ] );
  var exp = [ [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ];
  test.identical( got, 2 );
  test.identical( dst.original, exp );

  test.case = 'container, remove Long, fromIndex and evaluator2';
  var dst = _.containerAdapter.make( [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ] );
  var got = dst.removedOnceRight( [ 1, 2 ], 2, ( e ) => e[ 0 ] );
  var exp = [ [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ];
  test.identical( got, 2 );
  test.identical( dst.original, exp );

  test.case = 'container, remove Long, equalizer';
  var dst = _.containerAdapter.make( [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ] );
  var got = dst.removedOnceRight( [ 1, 2 ], ( e, ins ) => e === ins[ 0 ] );
  var exp = [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ];
  test.identical( got, -1 );
  test.identical( dst.original, exp );
}

//

function arrayAdapterRemovedOnceStrictly( test )
{
  test.case = 'container, remove primitive';
  var dst = _.containerAdapter.make( [ 1, 2, 2, '1' ] );
  var got = dst.removedOnceStrictly( 1 );
  var exp = [ 2, 2, '1' ];
  test.identical( got, 0 );
  test.identical( dst.original, exp );

  /* */

  test.case = 'container, remove Long, one evaluator';
  var dst = _.containerAdapter.make( [ [ 1, 2 ], [ 2, 2 ] ] );
  var got = dst.removedOnceStrictly( [ 1, 2 ], ( e ) => e[ 0 ] );
  var exp = [ [ 2, 2 ] ];
  test.identical( got, 0 );
  test.identical( dst.original, exp );

  test.case = 'container, remove Long, two evaluators';
  var dst = _.containerAdapter.make( [ [ 1, 2 ], [ 2, 2 ], [ 2, 2 ] ] );
  var got = dst.removedOnceStrictly( [ 1, 2 ], ( e ) => e[ 0 ], ( ins ) => ins[ 0 ] );
  var exp = [ [ 2, 2 ], [ 2, 2 ] ];
  test.identical( got, 0 );
  test.identical( dst.original, exp );

  test.case = 'container, remove Long, fromIndex and evaluator2';
  var dst = _.containerAdapter.make( [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ] );
  var got = dst.removedOnceStrictly( [ 1, 2 ], 2, ( e ) => e[ 0 ] );
  var exp = [ [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ];
  test.identical( got, 2 );
  test.identical( dst.original, exp );

  test.case = 'container, remove Long, equalizer';
  var dst = _.containerAdapter.make( [ [ 1, 2 ], [ 2, 2 ] ] );
  var got = dst.removedOnceStrictly( [ 1, 2 ], ( e, ins ) => e[ 0 ] === ins[ 0 ] );
  var exp = [ [ 2, 2 ] ];
  test.identical( got, 0 );
  test.identical( dst.original, exp );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'empty container'
  test.shouldThrowErrorSync( () =>
  {
    var dst = _.containerAdapter.make( [] );
    var got = dst.removedOnceStrictly( 1 );
  });

  test.case = 'container does not have element';
  test.shouldThrowErrorSync( () =>
  {
    var dst = _.containerAdapter.make( [ 1, 1, 2, 2, '1' ] );
    var got = dst.removedOnceStrictly( [ 1, 2 ] );
  });

  test.case = 'container has a few elements';
  test.shouldThrowErrorSync( () =>
  {
    var dst = _.containerAdapter.make( [ 1, 1, 2, 2, '1' ] );
    var got = dst.removedOnceStrictly( 1 );
  });
}

//

function arrayAdapterRemovedOnceStrictlyRight( test )
{
  test.case = 'container, remove primitive';
  var dst = _.containerAdapter.make( [ 1, 2, 2, '1' ] );
  var got = dst.removedOnceStrictlyRight( 1 );
  var exp = [ 2, 2, '1' ];
  test.identical( got, 0 );
  test.identical( dst.original, exp );

  /* */

  test.case = 'container, remove Long, one evaluator';
  var dst = _.containerAdapter.make( [ [ 1, 2 ], [ 2, 2 ] ] );
  var got = dst.removedOnceStrictlyRight( [ 1, 2 ], ( e ) => e[ 0 ] );
  var exp = [ [ 2, 2 ] ];
  test.identical( got, 0 );
  test.identical( dst.original, exp );

  test.case = 'container, remove Long, two evaluators';
  var dst = _.containerAdapter.make( [ [ 1, 2 ], [ 2, 2 ], [ 2, 2 ] ] );
  var got = dst.removedOnceStrictlyRight( [ 1, 2 ], ( e ) => e[ 0 ], ( ins ) => ins[ 0 ] );
  var exp = [ [ 2, 2 ], [ 2, 2 ] ];
  test.identical( got, 0 );
  test.identical( dst.original, exp );

  test.case = 'container, remove Long, fromIndex and evaluator2';
  var dst = _.containerAdapter.make( [ [ 2, 2 ], [ 1, 2 ], [ 2, 2 ], [ 1, 2 ] ] );
  var got = dst.removedOnceStrictlyRight( [ 1, 2 ], 2, ( e ) => e[ 0 ] );
  var exp = [ [ 2, 2 ], [ 2, 2 ], [ 1, 2 ] ];
  test.identical( got, 1 );
  test.identical( dst.original, exp );

  test.case = 'container, remove Long, equalizer';
  var dst = _.containerAdapter.make( [ [ 1, 2 ], [ 2, 2 ] ] );
  var got = dst.removedOnceStrictlyRight( [ 1, 2 ], ( e, ins ) => e[ 0 ] === ins[ 0 ] );
  var exp = [ [ 2, 2 ] ];
  test.identical( got, 0 );
  test.identical( dst.original, exp );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'empty container'
  test.shouldThrowErrorSync( () =>
  {
    var dst = _.containerAdapter.make( [] );
    var got = dst.removedOnceStrictlyRight( 1 );
  });

  test.case = 'container does not have element';
  test.shouldThrowErrorSync( () =>
  {
    var dst = _.containerAdapter.make( [ 1, 1, 2, 2, '1' ] );
    var got = dst.removedOnceStrictlyRight( [ 1, 2 ] );
  });

  test.case = 'container has a few elements';
  test.shouldThrowErrorSync( () =>
  {
    var dst = _.containerAdapter.make( [ 1, 1, 2, 2, '1' ] );
    var got = dst.removedOnceStrictlyRight( 1 );
  });
}

//

function arrayAdapterRemove( test )
{
  test.case = 'empty container, remove primitive';
  var dst = _.containerAdapter.make( [] );
  var got = dst.remove( 1 );
  var exp = [];
  test.true( got === dst );
  test.identical( got.original, exp );

  test.case = 'empty container, remove Long';
  var dst = _.containerAdapter.make( [] );
  var got = dst.remove( _.unroll.make( [ 1, 2 ] ) );
  var exp = [];
  test.true( got === dst );
  test.identical( got.original, exp );

  test.case = 'empty container, remove map';
  var dst = _.containerAdapter.make( [] );
  var got = dst.remove( { a : 0 } );
  var exp = [];
  test.true( got === dst );
  test.identical( got.original, exp );

  /* */

  test.case = 'container, remove primitive';
  var dst = _.containerAdapter.make( [ 1, 1, 2, 2, '1' ] );
  var got = dst.remove( 1 );
  var exp = [ 2, 2, '1' ];
  test.true( got === dst );
  test.identical( got.original, exp );

  test.case = 'container, remove Long';
  var dst = _.containerAdapter.make( [ 1, 1, 2, 2, '1' ] );
  var got = dst.remove( _.unroll.make( [ 1, 2 ] ) );
  var exp = [ 1, 1, 2, 2, '1' ];
  test.true( got === dst );
  test.identical( got.original, exp );

  test.case = 'container, remove map';
  var dst = _.containerAdapter.make( [ 1, 1, 2, 2, '1' ] );
  var got = dst.remove( { a : 1 } );
  var exp = [ 1, 1, 2, 2, '1' ];
  test.true( got === dst );
  test.identical( got.original, exp );

  /* */

  test.case = 'container, remove primitive, duplicates';
  var dst = _.containerAdapter.make( [ 1, 2, 3 ] );
  var got = dst.remove( 1 ).remove( 2 );
  var exp = [ 3 ];
  test.true( got === dst );
  test.identical( got.original, exp );

  test.case = 'container, remove Long, duplicates';
  var dst = _.containerAdapter.make( [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ] );
  var got = dst.remove( [ 1, 2 ] ).remove( [ 1, 2 ] );
  var exp = [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ];
  test.true( got === dst );
  test.identical( got.original, exp );

  test.case = 'container, remove map, duplicates';
  var dst = _.containerAdapter.make( [ { a : 0 }, { a : 0 }, { a : 0 }, { a : 1 } ] );
  var got = dst.remove( { a : 0 } ).remove( { a : 0 } );
  var exp = [ { a : 0 }, { a : 0 }, { a : 0 }, { a : 1 } ];
  test.true( got === dst );
  test.identical( got.original, exp );

  /* */

  test.case = 'container, remove Long, one evaluator';
  var dst = _.containerAdapter.make( [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ] );
  var got = dst.remove( [ 1, 2 ], ( e ) => e[ 0 ] );
  var exp = [ [ 2, 2 ] ];
  test.true( got === dst );
  test.identical( got.original, exp );

  test.case = 'container, remove Long, two evaluators';
  var dst = _.containerAdapter.make( [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ] );
  var got = dst.remove( [ 1, 2 ], ( e ) => e[ 0 ], ( ins ) => ins[ 0 ] );
  var exp = [ [ 2, 2 ] ];
  test.true( got === dst );
  test.identical( got.original, exp );

  test.case = 'container, remove Long, fromIndex and evaluator2';
  var dst = _.containerAdapter.make( [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ] );
  var got = dst.remove( [ 1, 2 ], 2, ( e ) => e[ 0 ] );
  var exp = [ [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ];
  test.true( got === dst );
  test.identical( got.original, exp );

  test.case = 'container, remove Long, equalizer';
  var dst = _.containerAdapter.make( [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ] );
  var got = dst.remove( [ 1, 2 ], ( e, ins ) => e === ins[ 0 ] );
  var exp = [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ];
  test.true( got === dst );
  test.identical( got.original, exp );
}

//

function arrayAdapterRemoveOnce( test )
{
  test.case = 'empty container, remove primitive';
  var dst = _.containerAdapter.make( [] );
  var got = dst.removeOnce( 1 );
  var exp = [];
  test.true( got === dst );
  test.identical( got.original, exp );

  test.case = 'empty container, remove Long';
  var dst = _.containerAdapter.make( [] );
  var got = dst.removeOnce( _.unroll.make( [ 1, 2 ] ) );
  var exp = [];
  test.true( got === dst );
  test.identical( got.original, exp );

  test.case = 'empty container, remove map';
  var dst = _.containerAdapter.make( [] );
  var got = dst.removeOnce( { a : 0 } );
  var exp = [];
  test.true( got === dst );
  test.identical( got.original, exp );

  /* */

  test.case = 'container, remove primitive';
  var dst = _.containerAdapter.make( [ 1, 1, 2, 2, '1' ] );
  var got = dst.removeOnce( 1 );
  var exp = [ 1, 2, 2, '1' ];
  test.true( got === dst );
  test.identical( got.original, exp );

  test.case = 'container, remove Long';
  var dst = _.containerAdapter.make( [ 1, 1, 2, 2, '1' ] );
  var got = dst.removeOnce( _.unroll.make( [ 1, 2 ] ) );
  var exp = [ 1, 1, 2, 2, '1' ];
  test.true( got === dst );
  test.identical( got.original, exp );

  test.case = 'container, remove map';
  var dst = _.containerAdapter.make( [ 1, 1, 2, 2, '1' ] );
  var got = dst.removeOnce( { a : 1 } );
  var exp = [ 1, 1, 2, 2, '1' ];
  test.true( got === dst );
  test.identical( got.original, exp );

  /* */

  test.case = 'container, remove primitive, duplicates';
  var dst = _.containerAdapter.make( [ 1, 1, 2, 2, 3, 3 ] );
  var got = dst.removeOnce( 1 ).removeOnce( 2 );
  var exp = [ 1, 2, 3, 3 ];
  test.true( got === dst );
  test.identical( got.original, exp );

  test.case = 'container, remove Long, duplicates';
  var dst = _.containerAdapter.make( [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ] );
  var got = dst.removeOnce( [ 1, 2 ] ).removeOnce( [ 1, 2 ] );
  var exp = [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ];
  test.true( got === dst );
  test.identical( got.original, exp );

  test.case = 'container, remove map, duplicates';
  var dst = _.containerAdapter.make( [ { a : 0 }, { a : 0 }, { a : 0 }, { a : 1 } ] );
  var got = dst.removeOnce( { a : 0 } ).removeOnce( { a : 0 } );
  var exp = [ { a : 0 }, { a : 0 }, { a : 0 }, { a : 1 } ];
  test.true( got === dst );
  test.identical( got.original, exp );

  /* */

  test.case = 'container, remove Long, one evaluator';
  var dst = _.containerAdapter.make( [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ] );
  var got = dst.removeOnce( [ 1, 2 ], ( e ) => e[ 0 ] );
  var exp = [ [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ];
  test.true( got === dst );
  test.identical( got.original, exp );

  test.case = 'container, remove Long, two evaluators';
  var dst = _.containerAdapter.make( [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ] );
  var got = dst.removeOnce( [ 1, 2 ], ( e ) => e[ 0 ], ( ins ) => ins[ 0 ] );
  var exp = [ [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ];
  test.true( got === dst );
  test.identical( got.original, exp );

  test.case = 'container, remove Long, fromIndex and evaluator2';
  var dst = _.containerAdapter.make( [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ] );
  var got = dst.removeOnce( [ 1, 2 ], 2, ( e ) => e[ 0 ] );
  var exp = [ [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ];
  test.true( got === dst );
  test.identical( got.original, exp );

  test.case = 'container, remove Long, equalizer';
  var dst = _.containerAdapter.make( [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ] );
  var got = dst.removeOnce( [ 1, 2 ], ( e, ins ) => e === ins[ 0 ] );
  var exp = [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ];
  test.true( got === dst );
  test.identical( got.original, exp );
}

//

function arrayAdapterRemoveOnceRight( test )
{
  test.case = 'empty container, remove primitive';
  var dst = _.containerAdapter.make( [] );
  var got = dst.removeOnceRight( 1 );
  var exp = [];
  test.true( got === dst );
  test.identical( got.original, exp );

  test.case = 'empty container, remove Long';
  var dst = _.containerAdapter.make( [] );
  var got = dst.removeOnceRight( _.unroll.make( [ 1, 2 ] ) );
  var exp = [];
  test.true( got === dst );
  test.identical( got.original, exp );

  test.case = 'empty container, remove empty Long';
  var dst = _.containerAdapter.make( [] );
  var got = dst.removeOnceRight( [] );
  var exp = [];
  test.true( got === dst );
  test.identical( got.original, exp );

  test.case = 'empty container, remove map';
  var dst = _.containerAdapter.make( [] );
  var got = dst.removeOnceRight( { a : 0 } );
  var exp = [];
  test.true( got === dst );
  test.identical( got.original, exp );

  /* */

  test.case = 'container, remove primitive';
  var dst = _.containerAdapter.make( [ 1, 1, 2, 2, '1' ] );
  var got = dst.removeOnceRight( 1 );
  var exp = [ 1, 2, 2, '1' ];
  test.true( got === dst );
  test.identical( got.original, exp );

  test.case = 'container, remove Long';
  var dst = _.containerAdapter.make( [ 1, 1, 2, 2, '1' ] );
  var got = dst.removeOnceRight( _.unroll.make( [ 1, 2 ] ) );
  var exp = [ 1, 1, 2, 2, '1' ];
  test.true( got === dst );
  test.identical( got.original, exp );

  test.case = 'container, remove map';
  var dst = _.containerAdapter.make( [ 1, 1, 2, 2, '1' ] );
  var got = dst.removeOnceRight( { a : 1 } );
  var exp = [ 1, 1, 2, 2, '1' ];
  test.true( got === dst );
  test.identical( got.original, exp );

  /* */

  test.case = 'container, remove primitive, duplicates';
  var dst = _.containerAdapter.make( [ 1, 1, 2, 2, 3, 3 ] );
  var got = dst.removeOnceRight( 1 ).removeOnceRight( 2 );
  var exp = [ 1, 2, 3, 3 ];
  test.true( got === dst );
  test.identical( got.original, exp );

  test.case = 'container, remove Long, duplicates';
  var dst = _.containerAdapter.make( [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ] );
  var got = dst.removeOnceRight( [ 1, 2 ] ).removeOnceRight( [ 1, 2 ] );
  var exp = [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ];
  test.true( got === dst );
  test.identical( got.original, exp );

  test.case = 'container, remove map, duplicates';
  var dst = _.containerAdapter.make( [ { a : 0 }, { a : 0 }, { a : 0 }, { a : 1 } ] );
  var got = dst.removeOnceRight( { a : 0 } ).removeOnceRight( { a : 0 } );
  var exp = [ { a : 0 }, { a : 0 }, { a : 0 }, { a : 1 } ];
  test.true( got === dst );
  test.identical( got.original, exp );

  /* */

  test.case = 'container, remove Long, one evaluator';
  var dst = _.containerAdapter.make( [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ] );
  var got = dst.removeOnceRight( [ 1, 2 ], ( e ) => e[ 0 ] );
  var exp = [ [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ];
  test.true( got === dst );
  test.identical( got.original, exp );

  test.case = 'container, remove Long, two evaluators';
  var dst = _.containerAdapter.make( [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ] );
  var got = dst.removeOnceRight( [ 1, 2 ], ( e ) => e[ 0 ], ( ins ) => ins[ 0 ] );
  var exp = [ [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ];
  test.true( got === dst );
  test.identical( got.original, exp );

  test.case = 'container, remove Long, fromIndex and evaluator2';
  var dst = _.containerAdapter.make( [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ] );
  var got = dst.removeOnceRight( [ 1, 2 ], 2, ( e ) => e[ 0 ] );
  var exp = [ [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ];
  test.true( got === dst );
  test.identical( got.original, exp );

  test.case = 'container, remove Long, equalizer';
  var dst = _.containerAdapter.make( [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ] );
  var got = dst.removeOnceRight( [ 1, 2 ], ( e, ins ) => e === ins[ 0 ] );
  var exp = [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ];
  test.true( got === dst );
  test.identical( got.original, exp );
}

//

function arrayAdapterRemoveOnceStrictly( test )
{
  test.case = 'container, remove primitive';
  var dst = _.containerAdapter.make( [ 1, 2, 2, '1' ] );
  var got = dst.removeOnceStrictly( 1 );
  var exp = [ 2, 2, '1' ];
  test.true( got === dst );
  test.identical( got.original, exp );

  /* */

  test.case = 'container, remove Long, one evaluator';
  var dst = _.containerAdapter.make( [ [ 1, 2 ], [ 2, 2 ] ] );
  var got = dst.removeOnceStrictly( [ 1, 2 ], ( e ) => e[ 0 ] );
  var exp = [ [ 2, 2 ] ];
  test.true( got === dst );
  test.identical( got.original, exp );

  test.case = 'container, remove Long, two evaluators';
  var dst = _.containerAdapter.make( [ [ 1, 2 ], [ 2, 2 ], [ 2, 2 ] ] );
  var got = dst.removeOnceStrictly( [ 1, 2 ], ( e ) => e[ 0 ], ( ins ) => ins[ 0 ] );
  var exp = [ [ 2, 2 ], [ 2, 2 ] ];
  test.true( got === dst );
  test.identical( got.original, exp );

  test.case = 'container, remove Long, fromIndex and evaluator2';
  var dst = _.containerAdapter.make( [ [ 1, 2 ], [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ] );
  var got = dst.removeOnceStrictly( [ 1, 2 ], 2, ( e ) => e[ 0 ] );
  var exp = [ [ 1, 2 ], [ 1, 2 ], [ 2, 2 ] ];
  test.true( got === dst );
  test.identical( got.original, exp );

  test.case = 'container, remove Long, equalizer';
  var dst = _.containerAdapter.make( [ [ 1, 2 ], [ 2, 2 ] ] );
  var got = dst.removeOnceStrictly( [ 1, 2 ], ( e, ins ) => e[ 0 ] === ins[ 0 ] );
  var exp = [ [ 2, 2 ] ];
  test.true( got === dst );
  test.identical( got.original, exp );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'empty container'
  test.shouldThrowErrorSync( () =>
  {
    var dst = _.containerAdapter.make( [] );
    var got = dst.removeOnceStrictly( 1 );
  });

  test.case = 'container does not have element';
  test.shouldThrowErrorSync( () =>
  {
    var dst = _.containerAdapter.make( [ 1, 1, 2, 2, '1' ] );
    var got = dst.removeOnceStrictly( [ 1, 2 ] );
  });

  test.case = 'container has a few elements';
  test.shouldThrowErrorSync( () =>
  {
    var dst = _.containerAdapter.make( [ 1, 1, 2, 2, '1' ] );
    var got = dst.removeOnceStrictly( 1 );
  });
}

//

function arrayAdapterRemoveOnceStrictlyRight( test )
{
  test.case = 'container, remove primitive';
  var dst = _.containerAdapter.make( [ 1, 2, 2, '1' ] );
  var got = dst.removeOnceStrictlyRight( 1 );
  var exp = [ 2, 2, '1' ];
  test.true( got === dst );
  test.identical( got.original, exp );

  /* */

  test.case = 'container, remove Long, one evaluator';
  var dst = _.containerAdapter.make( [ [ 1, 2 ], [ 2, 2 ] ] );
  var got = dst.removeOnceStrictlyRight( [ 1, 2 ], ( e ) => e[ 0 ] );
  var exp = [ [ 2, 2 ] ];
  test.true( got === dst );
  test.identical( got.original, exp );

  test.case = 'container, remove Long, two evaluators';
  var dst = _.containerAdapter.make( [ [ 1, 2 ], [ 2, 2 ], [ 2, 2 ] ] );
  var got = dst.removeOnceStrictlyRight( [ 1, 2 ], ( e ) => e[ 0 ], ( ins ) => ins[ 0 ] );
  var exp = [ [ 2, 2 ], [ 2, 2 ] ];
  test.true( got === dst );
  test.identical( got.original, exp );

  test.case = 'container, remove Long, fromIndex and evaluator2';
  var dst = _.containerAdapter.make( [ [ 2, 2 ], [ 1, 2 ], [ 2, 2 ], [ 1, 2 ] ] );
  var got = dst.removeOnceStrictlyRight( [ 1, 2 ], 2, ( e ) => e[ 0 ] );
  var exp = [ [ 2, 2 ], [ 2, 2 ], [ 1, 2 ] ];
  test.true( got === dst );
  test.identical( got.original, exp );

  test.case = 'container, remove Long, equalizer';
  var dst = _.containerAdapter.make( [ [ 1, 2 ], [ 2, 2 ] ] );
  var got = dst.removeOnceStrictlyRight( [ 1, 2 ], ( e, ins ) => e[ 0 ] === ins[ 0 ] );
  var exp = [ [ 2, 2 ] ];
  test.true( got === dst );
  test.identical( got.original, exp );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'empty container'
  test.shouldThrowErrorSync( () =>
  {
    var dst = _.containerAdapter.make( [] );
    var got = dst.removeOnceStrictlyRight( 1 );
  });

  test.case = 'container does not have element';
  test.shouldThrowErrorSync( () =>
  {
    var dst = _.containerAdapter.make( [ 1, 1, 2, 2, '1' ] );
    var got = dst.removeOnceStrictlyRight( [ 1, 2 ] );
  });

  test.case = 'container has a few elements';
  test.shouldThrowErrorSync( () =>
  {
    var dst = _.containerAdapter.make( [ 1, 1, 2, 2, '1' ] );
    var got = dst.removeOnceStrictlyRight( 1 );
  });
}

//

function arrayAdapterEmpty( test )
{
  test.case = 'empty container';
  var src = _.containerAdapter.make( [] );
  var got = src.empty();
  test.identical( src.original, [] );

  test.case = 'not empty container';
  var src = _.containerAdapter.make( [ 1, 2, 3, [ 1 ], null, undefined, { a : 0 } ] );
  var got = src.empty();
  test.identical( src.original, [] );
}

//

function arrayAdapterMap( test )
{
  test.case = 'without arguments';
  var src = _.containerAdapter.make( [] );
  var exp = [];
  var got = src.map( null );
  test.true( got !== src );
  test.identical( got.original, exp );

  /* - */

  test.open( 'only onEach' );

  test.case = 'from empty array, onEach returns number';
  var src = _.containerAdapter.make( [] );
  var exp = [];
  var got = src.map( ( e ) => 123 );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns original';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ];
  var got = src.map( ( e ) => e );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns key';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ 0, 1, 2, 3, 4, 5, 6, 7, 8 ];
  var got = src.map( ( e, k ) => k );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns undefined';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ];
  var got = src.map( ( e ) => undefined );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns array';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ [ 0 ], [ 1 ], [ null ], [ true ], [ false ], [ undefined ], [ '' ], [ [ 2 ] ], [ { a : 0 } ] ];
  var got = src.map( ( e ) => [ e ] );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns element of array';
  var src = _.containerAdapter.make( [ 0, 1, NaN, true, false, [ undefined ], '', [ 2 ], { a : 0 } ] );
  var exp = [ 0, 1, NaN, true, false, [ undefined ], '', 2, { a : 0 } ];
  var got = src.map( ( e ) => e[ 0 ] );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.close( 'only onEach' );

  /* - */

  test.open( 'dst === null' );

  test.case = 'from array, onEach returns container';
  var src = _.containerAdapter.make( [ 1 ] );
  var exp = [ src ];
  var got = src.map( null, ( e, k, src ) => src );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns original';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ];
  var got = src.map( null, ( e ) => e );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns key';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ 0, 1, 2, 3, 4, 5, 6, 7, 8 ];
  var got = src.map( null, ( e, k ) => k );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns undefined';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ];
  var got = src.map( null, ( e ) => undefined );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns array';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ [ 0 ], [ 1 ], [ null ], [ true ], [ false ], [ undefined ], [ '' ], [ [ 2 ] ], [ { a : 0 } ] ];
  var got = src.map( null, ( e ) => [ e ] );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns element of array';
  var src = _.containerAdapter.make( [ 0, 1, NaN, true, false, [ undefined ], '', [ 2 ], { a : 0 } ] );
  var exp = [ 0, 1, NaN, true, false, [ undefined ], '', 2, { a : 0 } ];
  var got = src.map( null, ( e ) => e[ 0 ] );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.close( 'dst === null' );

  /* - */

  test.open( 'dst === src' );

  test.case = 'from empty array, onEach returns container';
  var src = _.containerAdapter.make( [] );
  var exp = [];
  var got = src.map( src, ( e, k, src ) => src );
  test.true( got === src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns original';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ];
  var got = src.map( src, ( e ) => e );
  test.true( got === src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns key';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ 0, 1, 2, 3, 4, 5, 6, 7, 8 ];
  var got = src.map( src, ( e, k ) => k );
  test.true( got === src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns undefined';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ];
  var got = src.map( src, ( e ) => undefined );
  test.true( got === src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns array';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ [ 0 ], [ 1 ], [ null ], [ true ], [ false ], [ undefined ], [ '' ], [ [ 2 ] ], [ { a : 0 } ] ];
  var got = src.map( src, ( e ) => [ e ] );
  test.true( got === src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns element of array';
  var src = _.containerAdapter.make( [ 0, 1, NaN, true, false, [ undefined ], '', [ 2 ], { a : 0 } ] );
  var exp = [ 0, 1, NaN, true, false, [ undefined ], '', 2, { a : 0 } ];
  var got = src.map( src, ( e ) => e[ 0 ] );
  test.true( got === src );
  test.identical( got.original, exp );

  test.close( 'dst === src' );
}

//

function arrayAdapterMapRight( test )
{
  test.case = 'without arguments';
  var src = _.containerAdapter.make( [] );
  var exp = [];
  var got = src.mapRight( null );
  test.true( got !== src );
  test.identical( got.original, exp );

  /* - */

  test.open( 'only onEach' );

  test.case = 'from empty array, onEach returns number';
  var src = _.containerAdapter.make( [] );
  var exp = [];
  var got = src.mapRight( ( e ) => 123 );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns original';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ { a : 0 }, [ 2 ], '', undefined, false, true, null, 1, 0 ];
  var got = src.mapRight( ( e ) => e );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns key';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ 8, 7, 6, 5, 4, 3, 2, 1, 0 ];
  var got = src.mapRight( ( e, k ) => k );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns undefined';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ { a : 0 }, [ 2 ], '', undefined, false, true, null, 1, 0 ];
  var got = src.mapRight( ( e ) => undefined );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns array';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ [ { a : 0 } ], [ [ 2 ] ], [ '' ], [ undefined ], [ false ], [ true ], [ null ], [ 1 ], [ 0 ] ];
  var got = src.mapRight( ( e ) => [ e ] );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns element of array';
  var src = _.containerAdapter.make( [ 0, 1, NaN, true, false, [ undefined ], '', [ 2 ], { a : 0 } ] );
  var exp = [ { a : 0 }, 2, '', [ undefined ], false, true, NaN, 1, 0 ];
  var got = src.mapRight( ( e ) => e[ 0 ] );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.close( 'only onEach' );

  /* - */

  test.open( 'dst === null' );

  test.case = 'from array, onEach returns container';
  var src = _.containerAdapter.make( [ 1 ] );
  var exp = [ src ];
  var got = src.mapRight( null, ( e, k, src ) => src );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns original';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ { a : 0 }, [ 2 ], '', undefined, false, true, null, 1, 0 ];
  var got = src.mapRight( null, ( e ) => e );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns key';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ 8, 7, 6, 5, 4, 3, 2, 1, 0 ];
  var got = src.mapRight( null, ( e, k ) => k );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns undefined';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ { a : 0 }, [ 2 ], '', undefined, false, true, null, 1, 0 ];
  var got = src.mapRight( null, ( e ) => undefined );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns array';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ [ { a : 0 } ], [ [ 2 ] ], [ '' ], [ undefined ], [ false ], [ true ], [ null ], [ 1 ], [ 0 ] ];
  var got = src.mapRight( null, ( e ) => [ e ] );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns element of array';
  var src = _.containerAdapter.make( [ 0, 1, NaN, true, false, [ undefined ], '', [ 2 ], { a : 0 } ] );
  var exp = [ { a : 0 }, 2, '', [ undefined ], false, true, NaN, 1, 0 ];
  var got = src.mapRight( null, ( e ) => e[ 0 ] );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.close( 'dst === null' );

  /* - */

  test.open( 'dst === src' );

  test.case = 'from empty array, onEach returns container';
  var src = _.containerAdapter.make( [] );
  var exp = [];
  var got = src.mapRight( src, ( e, k, src ) => src );
  test.true( got === src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns original';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ];
  var got = src.mapRight( src, ( e ) => e );
  test.true( got === src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns key';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ 0, 1, 2, 3, 4, 5, 6, 7, 8 ];
  var got = src.mapRight( src, ( e, k ) => k );
  test.true( got === src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns undefined';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ];
  var got = src.mapRight( src, ( e ) => undefined );
  test.true( got === src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns array';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ [ 0 ], [ 1 ], [ null ], [ true ], [ false ], [ undefined ], [ '' ], [ [ 2 ] ], [ { a : 0 } ] ];
  var got = src.mapRight( src, ( e ) => [ e ] );
  test.true( got === src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns element of array';
  var src = _.containerAdapter.make( [ 0, 1, NaN, true, false, [ undefined ], '', [ 2 ], { a : 0 } ] );
  var exp = [ 0, 1, NaN, true, false, [ undefined ], '', 2, { a : 0 } ];
  var got = src.mapRight( src, ( e ) => e[ 0 ] );
  test.true( got === src );
  test.identical( got.original, exp );

  test.close( 'dst === src' );
}

//

function arrayAdapterFilter( test )
{
  test.case = 'without arguments';
  var src = _.containerAdapter.make( [] );
  var exp = [];
  var got = src.filter( null );
  test.true( got !== src );
  test.identical( got.original, exp );

  /* - */

  test.open( 'only onEach' );

  test.case = 'from empty array, onEach returns number';
  var src = _.containerAdapter.make( [] );
  var exp = [];
  var got = src.filter( ( e ) => 123 );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns original';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ 0, 1, null, true, false, '', [ 2 ], { a : 0 } ];
  var got = src.filter( ( e ) => e );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns key';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ 0, 1, 2, 3, 4, 5, 6, 7, 8 ];
  var got = src.filter( ( e, k ) => k );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns undefined';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [];
  var got = src.filter( ( e ) => undefined );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns array';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ [ 0 ], [ 1 ], [ null ], [ true ], [ false ], [ undefined ], [ '' ], [ [ 2 ] ], [ { a : 0 } ] ];
  var got = src.filter( ( e ) => [ e ] );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns element of array';
  var src = _.containerAdapter.make( [ 0, 1, NaN, true, false, [ undefined ], '', [ 2 ], { a : 0 } ] );
  var exp = [ 2 ];
  var got = src.filter( ( e ) => e[ 0 ] );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.close( 'only onEach' );

  /* - */

  test.open( 'dst === null' );

  test.case = 'from array, onEach returns container';
  var src = _.containerAdapter.make( [ 1 ] );
  var exp = [ src.original ];
  var got = src.filter( null, ( e, k, src ) => src );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns original';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ 0, 1, null, true, false, '', [ 2 ], { a : 0 } ];
  var got = src.filter( null, ( e ) => e );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns key';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ 0, 1, 2, 3, 4, 5, 6, 7, 8 ];
  var got = src.filter( null, ( e, k ) => k );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns undefined';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [];
  var got = src.filter( null, ( e ) => undefined );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns array';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ [ 0 ], [ 1 ], [ null ], [ true ], [ false ], [ undefined ], [ '' ], [ [ 2 ] ], [ { a : 0 } ] ];
  var got = src.filter( null, ( e ) => [ e ] );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns element of array';
  var src = _.containerAdapter.make( [ 0, 1, NaN, true, false, [ undefined ], '', [ 2 ], { a : 0 } ] );
  var exp = [ 2 ];
  var got = src.filter( null, ( e ) => e[ 0 ] );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.close( 'dst === null' );

  /* - */

  test.open( 'dst != src' );

  test.case = 'dst - array, from empty array, onEach returns container';
  var dst = [ 1, 2 ];
  var src = _.containerAdapter.make( [] );
  var exp = [ 1, 2 ];
  var got = src.filter( dst, ( e, k, src ) => src );
  test.true( got !== dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'dst - arrayAdapter, from array, onEach returns original';
  var dst = _.containerAdapter.make( [ 1, 2 ] );
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ 1, 2, 0, 1, null, true, false, '', [ 2 ], { a : 0 } ];
  var got = src.filter( dst, ( e ) => e );
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'dst - arrayAdapter, from array, onEach returns key';
  var dst = _.containerAdapter.make( [ 1, 2 ] );
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ 1, 2, 0, 1, 2, 3, 4, 5, 6, 7, 8 ];
  var got = src.filter( dst, ( e, k ) => k );
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'dst - Set, from array, onEach returns undefined';
  var dst = new Set( [ 1, 2 ] );
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ 1, 2 ];
  var got = src.filter( dst, ( e ) => undefined );
  test.true( got.original === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'dst - setAdapter, from array, onEach returns array';
  var dst = _.containerAdapter.make( new Set( [ 1, 2 ] ) );
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ 1, 2, [ 0 ], [ 1 ], [ null ], [ true ], [ false ], [ undefined ], [ '' ], [ [ 2 ] ], [ { a : 0 } ] ];
  var got = src.filter( dst, ( e ) => [ e ] );
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'dst - array, from array, onEach returns element of array';
  var dst = [ 1, 2 ];
  var src = _.containerAdapter.make( [ 0, 1, NaN, true, false, [ undefined ], '', [ 2 ], { a : 0 } ] );
  var exp = [ 1, 2, 2 ];
  var got = src.filter( dst, ( e ) => e[ 0 ] );
  test.true( got.original === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.close( 'dst != src' );

  /* - */

  test.open( 'dst === src' );

  test.case = 'from empty array, onEach returns container';
  var src = _.containerAdapter.make( [] );
  var exp = [];
  var got = src.filter( src, ( e, k, src ) => src );
  test.true( got === src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns original';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ 0, 1, null, true, false, '', [ 2 ], { a : 0 } ];
  var got = src.filter( src, ( e ) => e );
  test.true( got === src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns key';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ 0, 1, 2, 3, 4, 5, 6, 7, 8 ];
  var got = src.filter( src, ( e, k ) => k );
  test.true( got === src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns undefined';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [];
  var got = src.filter( src, ( e ) => undefined );
  test.true( got === src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns array';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ [ 0 ], [ 1 ], [ null ], [ true ], [ false ], [ undefined ], [ '' ], [ [ 2 ] ], [ { a : 0 } ] ];
  var got = src.filter( src, ( e ) => [ e ] );
  test.true( got === src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns element of array';
  var src = _.containerAdapter.make( [ 0, 1, NaN, true, false, [ undefined ], '', [ 2 ], { a : 0 } ] );
  var exp = [ 2 ];
  var got = src.filter( src, ( e ) => e[ 0 ] );
  test.true( got === src );
  test.identical( got.original, exp );

  test.close( 'dst === src' );
}

//

function arrayAdapterFilterRight( test )
{
  test.case = 'without arguments';
  var src = _.containerAdapter.make( [] );
  var exp = [];
  var got = src.filterRight( null );
  test.true( got !== src );
  test.identical( got.original, exp );

  /* - */

  test.open( 'only onEach' );

  test.case = 'from empty array, onEach returns number';
  var src = _.containerAdapter.make( [] );
  var exp = [];
  var got = src.filterRight( ( e ) => 123 );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns original';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ { a : 0 }, [ 2 ], '', false, true, null, 1, 0 ];
  var got = src.filterRight( ( e ) => e );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns key';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ 8, 7, 6, 5, 4, 3, 2, 1, 0 ];
  var got = src.filterRight( ( e, k ) => k );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns undefined';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [];
  var got = src.filterRight( ( e ) => undefined );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns array';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ [ { a : 0 } ], [ [ 2 ] ], [ '' ], [ undefined ], [ false ], [ true ], [ null ], [ 1 ], [ 0 ] ];
  var got = src.filterRight( ( e ) => [ e ] );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns element of array';
  var src = _.containerAdapter.make( [ 0, 1, NaN, true, false, [ undefined ], '', [ 2 ], { a : 0 } ] );
  var exp = [ 2 ];
  var got = src.filterRight( ( e ) => e[ 0 ] );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.close( 'only onEach' );

  /* - */

  test.open( 'dst === null' );

  test.case = 'from array, onEach returns container';
  var src = _.containerAdapter.make( [ 1 ] );
  var exp = [ src.original ];
  var got = src.filterRight( null, ( e, k, src ) => src );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns original';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ { a : 0 }, [ 2 ], '', false, true, null, 1, 0 ];
  var got = src.filterRight( null, ( e ) => e );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns key';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ 8, 7, 6, 5, 4, 3, 2, 1, 0 ];
  var got = src.filterRight( null, ( e, k ) => k );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns undefined';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [];
  var got = src.filterRight( null, ( e ) => undefined );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns array';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ [ { a : 0 } ], [ [ 2 ] ], [ '' ], [ undefined ], [ false ], [ true ], [ null ], [ 1 ], [ 0 ] ];
  var got = src.filterRight( null, ( e ) => [ e ] );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns element of array';
  var src = _.containerAdapter.make( [ 0, 1, NaN, true, false, [ undefined ], '', [ 2 ], { a : 0 } ] );
  var exp = [ 2 ];
  var got = src.filterRight( null, ( e ) => e[ 0 ] );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.close( 'dst === null' );

  /* - */

  test.open( 'dst != src' );

  test.case = 'dst - array, from empty array, onEach returns container';
  var dst = [ 1, 2 ];
  var src = _.containerAdapter.make( [] );
  var exp = [ 1, 2 ];
  var got = src.filterRight( dst, ( e, k, src ) => src );
  test.true( got !== dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'dst - arrayAdapter, from array, onEach returns original';
  var dst = _.containerAdapter.make( [ 1, 2 ] );
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ 1, 2, { a : 0 }, [ 2 ], '', false, true, null, 1, 0 ];
  var got = src.filterRight( dst, ( e ) => e );
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'dst - arrayAdapter, from array, onEach returns key';
  var dst = _.containerAdapter.make( [ 1, 2 ] );
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ 1, 2, 8, 7, 6, 5, 4, 3, 2, 1, 0 ];
  var got = src.filterRight( dst, ( e, k ) => k );
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'dst - Set, from array, onEach returns undefined';
  var dst = new Set( [ 1, 2 ] );
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ 1, 2 ];
  var got = src.filterRight( dst, ( e ) => undefined );
  test.true( got.original === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'dst - setAdapter, from array, onEach returns array';
  var dst = _.containerAdapter.make( new Set( [ 1, 2 ] ) );
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ 1, 2, [ { a : 0 } ], [ [ 2 ] ], [ '' ], [ undefined ], [ false ], [ true ], [ null ], [ 1 ], [ 0 ] ];
  var got = src.filterRight( dst, ( e ) => [ e ] );
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'dst - array, from array, onEach returns element of array';
  var dst = [ 1, 2 ];
  var src = _.containerAdapter.make( [ 0, 1, NaN, true, false, [ undefined ], '', [ 2 ], { a : 0 } ] );
  var exp = [ 1, 2, 2 ];
  var got = src.filterRight( dst, ( e ) => e[ 0 ] );
  test.true( got.original === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.close( 'dst != src' );

  /* - */

  test.open( 'dst === src' );

  test.case = 'from empty array, onEach returns container';
  var src = _.containerAdapter.make( [] );
  var exp = [];
  var got = src.filterRight( src, ( e, k, src ) => src );
  test.true( got === src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns original';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ 0, 1, null, true, false, '', [ 2 ], { a : 0 } ];
  var got = src.filterRight( src, ( e ) => e );
  test.true( got === src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns key';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ 0, 1, 2, 3, 4, 5, 6, 7, 8 ];
  var got = src.filterRight( src, ( e, k ) => k );
  test.true( got === src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns undefined';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [];
  var got = src.filterRight( src, ( e ) => undefined );
  test.true( got === src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns array';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ [ 0 ], [ 1 ], [ null ], [ true ], [ false ], [ undefined ], [ '' ], [ [ 2 ] ], [ { a : 0 } ] ];
  var got = src.filterRight( src, ( e ) => [ e ] );
  test.true( got === src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns element of array';
  var src = _.containerAdapter.make( [ 0, 1, NaN, true, false, [ undefined ], '', [ 2 ], { a : 0 } ] );
  var exp = [ 2 ];
  var got = src.filterRight( src, ( e ) => e[ 0 ] );
  test.true( got === src );
  test.identical( got.original, exp );

  test.close( 'dst === src' );
}

//

function arrayAdapterFlatFilter( test )
{
  test.case = 'without arguments';
  var src = _.containerAdapter.make( [] );
  var exp = [];
  var got = src.flatFilter( null );
  test.true( got !== src );
  test.identical( got.original, exp );

  /* - */

  test.open( 'only onEach' );

  test.case = 'from empty array, onEach returns number';
  var src = _.containerAdapter.make( [] );
  var exp = [];
  var got = src.flatFilter( ( e ) => 123 );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns original';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ 0, 1, null, true, false, '', 2, { a : 0 } ];
  var got = src.flatFilter( ( e ) => e );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns key';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ 0, 1, 2, 3, 4, 5, 6, 7, 8 ];
  var got = src.flatFilter( ( e, k ) => [ k ] );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns undefined';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [];
  var got = src.flatFilter( ( e ) => undefined );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns array';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ];
  var got = src.flatFilter( ( e ) => [ e ] );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns element of array';
  var src = _.containerAdapter.make( [ 0, 1, NaN, true, false, [ undefined ], '', [ 2 ], { a : 0 } ] );
  var exp = [ 2 ];
  var got = src.flatFilter( ( e ) => e[ 0 ] );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.close( 'only onEach' );

  /* - */

  test.open( 'dst === null' );

  test.case = 'from array, onEach returns container';
  var src = _.containerAdapter.make( [ 1 ] );
  var exp = src.original;
  var got = src.flatFilter( null, ( e, k, src ) => src );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns original';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ 0, 1, null, true, false, '', 2, { a : 0 } ];
  var got = src.flatFilter( null, ( e ) => e );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns key';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ 0, 1, 2, 3, 4, 5, 6, 7, 8 ];
  var got = src.flatFilter( null, ( e, k ) => [ k ] );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns undefined';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [];
  var got = src.flatFilter( null, ( e ) => undefined );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns array';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ 0, '0', 1, '1', null, 'null', true, 'true', false, 'false', undefined, 'undefined', '', '', [ 2 ], '2', { a : 0 }, '[object Object]' ];
  var got = src.flatFilter( null, ( e ) => [ e, String( e ) ] );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns element of array';
  var src = _.containerAdapter.make( [ 0, 1, NaN, true, false, [ undefined ], '', [ 2 ], { a : 0 } ] );
  var exp = [ 2 ];
  var got = src.flatFilter( null, ( e ) => e[ 0 ] );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.close( 'dst === null' );

  /* - */

  test.open( 'dst != src' );

  test.case = 'dst - array, from empty array, onEach returns container';
  var dst = [ 1, 2 ];
  var src = _.containerAdapter.make( [] );
  var exp = [ 1, 2 ];
  var got = src.flatFilter( dst, ( e, k, src ) => src );
  test.true( got !== dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'dst - arrayAdapter, from array, onEach returns original';
  var dst = _.containerAdapter.make( [ 1, 2 ] );
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ 1, 2, 0, 1, null, true, false, '', 2, { a : 0 } ];
  var got = src.flatFilter( dst, ( e ) => e );
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'dst - arrayAdapter, from array, onEach returns key';
  var dst = _.containerAdapter.make( [ 1, 2 ] );
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ 1, 2, 0, 1, 2, 3, 4, 5, 6, 7, 8 ];
  var got = src.flatFilter( dst, ( e, k ) => [ k ] );
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'dst - Set, from array, onEach returns undefined';
  var dst = new Set( [ 1, 2 ] );
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ 1, 2 ];
  var got = src.flatFilter( dst, ( e ) => undefined );
  test.true( got.original === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'dst - setAdapter, from array, onEach returns array';
  var dst = _.containerAdapter.make( new Set( [ 1, 2 ] ) );
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ 1, 2, 0, null, true, false, undefined, '', [ 2 ], { a : 0 } ];
  var got = src.flatFilter( dst, ( e ) => [ e ] );
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'dst - array, from array, onEach returns element of array';
  var dst = [ 1, 2 ];
  var src = _.containerAdapter.make( [ 0, 1, NaN, true, false, [ undefined ], '', [ 2 ], { a : 0 } ] );
  var exp = [ 1, 2, 2 ];
  var got = src.flatFilter( dst, ( e ) => e[ 0 ] );
  test.true( got.original === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.close( 'dst != src' );

  /* - */

  test.open( 'dst === src' );

  test.case = 'from empty array, onEach returns container';
  var src = _.containerAdapter.make( [] );
  var exp = [];
  var got = src.flatFilter( src, ( e, k, src ) => src );
  test.true( got === src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns original';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ 0, 1, null, true, false, '', 2, { a : 0 } ];
  var got = src.flatFilter( src, ( e ) => e );
  test.true( got === src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns key';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ 0, 1, 2, 3, 4, 5, 6, 7, 8 ];
  var got = src.flatFilter( src, ( e, k ) => [ k ] );
  test.true( got === src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns undefined';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [];
  var got = src.flatFilter( src, ( e ) => undefined );
  test.true( got === src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns array';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ];
  var got = src.flatFilter( src, ( e ) => [ e ] );
  test.true( got === src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns element of array';
  var src = _.containerAdapter.make( [ 0, 1, NaN, true, false, [ undefined ], '', [ 2 ], { a : 0 } ] );
  var exp = [ 2 ];
  var got = src.flatFilter( src, ( e ) => e[ 0 ] );
  test.true( got === src );
  test.identical( got.original, exp );

  test.close( 'dst === src' );
}

//

function arrayAdapterFlatFilterRight( test )
{
  test.case = 'without arguments';
  var src = _.containerAdapter.make( [] );
  var exp = [];
  var got = src.flatFilterRight( null );
  test.true( got !== src );
  test.identical( got.original, exp );

  /* - */

  test.open( 'only onEach' );

  test.case = 'from empty array, onEach returns number';
  var src = _.containerAdapter.make( [] );
  var exp = [];
  var got = src.flatFilterRight( ( e ) => 123 );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns original';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ { a : 0 }, 2, '', false, true, null, 1, 0 ];
  var got = src.flatFilterRight( ( e ) => e );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns key';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ 8, 7, 6, 5, 4, 3, 2, 1, 0 ];
  var got = src.flatFilterRight( ( e, k ) => [ k ] );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns undefined';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [];
  var got = src.flatFilterRight( ( e ) => undefined );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns array';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ { a : 0 }, [ 2 ], '', undefined, false, true, null, 1, 0 ];
  var got = src.flatFilterRight( ( e ) => [ e ] );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns element of array';
  var src = _.containerAdapter.make( [ 0, 1, NaN, true, false, [ undefined ], '', [ 2 ], { a : 0 } ] );
  var exp = [ 2 ];
  var got = src.flatFilterRight( ( e ) => e[ 0 ] );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.close( 'only onEach' );

  /* - */

  test.open( 'dst === null' );

  test.case = 'from array, onEach returns container';
  var src = _.containerAdapter.make( [ 1 ] );
  var exp = src.original;
  var got = src.flatFilterRight( null, ( e, k, src ) => src );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns original';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ { a : 0 }, 2, '', false, true, null, 1, 0 ];
  var got = src.flatFilterRight( null, ( e ) => e );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns key';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ 8, 7, 6, 5, 4, 3, 2, 1, 0 ];
  var got = src.flatFilterRight( null, ( e, k ) => [ k ] );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns undefined';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [];
  var got = src.flatFilterRight( null, ( e ) => undefined );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns array';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ { a : 0 }, '[object Object]', [ 2 ], '2', '', '', undefined, 'undefined', false, 'false', true, 'true', null, 'null', 1, '1', 0, '0' ];
  var got = src.flatFilterRight( null, ( e ) => [ e, String( e ) ] );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns element of array';
  var src = _.containerAdapter.make( [ 0, 1, NaN, true, false, [ undefined ], '', [ 2 ], { a : 0 } ] );
  var exp = [ 2 ];
  var got = src.flatFilterRight( null, ( e ) => e[ 0 ] );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.close( 'dst === null' );

  /* - */

  test.open( 'dst != src' );

  test.case = 'dst - array, from empty array, onEach returns container';
  var dst = [ 1, 2 ];
  var src = _.containerAdapter.make( [] );
  var exp = [ 1, 2 ];
  var got = src.flatFilterRight( dst, ( e, k, src ) => src );
  test.true( got !== dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'dst - arrayAdapter, from array, onEach returns original';
  var dst = _.containerAdapter.make( [ 1, 2 ] );
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ 1, 2, { a : 0 }, 2, '', false, true, null, 1, 0 ];
  var got = src.flatFilterRight( dst, ( e ) => e );
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'dst - arrayAdapter, from array, onEach returns key';
  var dst = _.containerAdapter.make( [ 1, 2 ] );
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ 1, 2, 8, 7, 6, 5, 4, 3, 2, 1, 0 ];
  var got = src.flatFilterRight( dst, ( e, k ) => [ k ] );
  test.true( got === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'dst - Set, from array, onEach returns undefined';
  var dst = new Set( [ 1, 2 ] );
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ 1, 2 ];
  var got = src.flatFilterRight( dst, ( e ) => undefined );
  test.true( got.original === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'dst - setAdapter, from array, onEach returns array';
  var dst = _.containerAdapter.make( new Set( [ 1, 2 ] ) );
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ 1, 2, { a : 0 }, [ 2 ], '', undefined, false, true, null, 0 ];
  var got = src.flatFilterRight( dst, ( e ) => [ e ] );
  test.true( got === dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'dst - array, from array, onEach returns element of array';
  var dst = [ 1, 2 ];
  var src = _.containerAdapter.make( [ 0, 1, NaN, true, false, [ undefined ], '', [ 2 ], { a : 0 } ] );
  var exp = [ 1, 2, 2 ];
  var got = src.flatFilterRight( dst, ( e ) => e[ 0 ] );
  test.true( got.original === dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.close( 'dst != src' );

  /* - */

  test.open( 'dst === src' );

  test.case = 'from empty array, onEach returns container';
  var src = _.containerAdapter.make( [] );
  var exp = [];
  var got = src.flatFilterRight( src, ( e, k, src ) => src );
  test.true( got === src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns original';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ 0, 1, null, true, false, '', 2, { a : 0 } ];
  var got = src.flatFilterRight( src, ( e ) => e );
  test.true( got === src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns key';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ 0, 1, 2, 3, 4, 5, 6, 7, 8 ];
  var got = src.flatFilterRight( src, ( e, k ) => [ k ] );
  test.true( got === src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns undefined';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [];
  var got = src.flatFilterRight( src, ( e ) => undefined );
  test.true( got === src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns array';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ];
  var got = src.flatFilterRight( src, ( e ) => [ e ] );
  test.true( got === src );
  test.identical( got.original, exp );

  test.case = 'from array, onEach returns element of array';
  var src = _.containerAdapter.make( [ 0, 1, NaN, true, false, [ undefined ], '', [ 2 ], { a : 0 } ] );
  var exp = [ 2 ];
  var got = src.flatFilterRight( src, ( e ) => e[ 0 ] );
  test.true( got === src );
  test.identical( got.original, exp );

  test.close( 'dst === src' );
}

//

function arrayAdapterOnce( test )
{
  test.case = 'without arguments';
  var src = _.containerAdapter.make( [] );
  var exp = [];
  var got = src.once( null );
  test.true( got !== src );
  test.identical( got.original, exp );

  /* - */

  test.open( 'only onEval' );

  test.case = 'evaluator returns element';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ];
  var got = src.once( ( e ) => e );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'two evaluators returns element';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ];
  var got = src.once( ( e ) => e, ( ins ) => ins );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'equalizer';
  var src = _.containerAdapter.make( [ 0, 0, 1, 1, null, true, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ];
  var got = src.once( ( e, e2 ) => e === e2 );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'evaluator returns undefined';
  var src = _.containerAdapter.make( [ 0, 0, 1, 1, undefined, undefined, undefined, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ 0 ];
  var got = src.once( ( e ) => undefined );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'evaluator checks element of array';
  var src = _.containerAdapter.make( [ 0, 1, [ null ], [ true ], false, [ undefined ], '', [ 2 ], { a : 0 } ] );
  var exp = [ 0, [ null ], [ true ], [ 2 ] ];
  var got = src.once( ( e ) => e[ 0 ] );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.close( 'only onEval' );

  /* - */

  test.open( 'only dst' );

  test.case = 'src - empty container, dst - empty array, no onEval';
  var src = _.containerAdapter.make( [] );
  var dst = [];
  var exp = [];
  var got = src.once( dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'src - empty container, dst - empty arrayAdapter, no onEval';
  var src = _.containerAdapter.make( [] );
  var dst = _.containerAdapter.make( [] );
  var exp = [];
  var got = src.once( dst );
  test.true( got !== src );
  test.true( got === dst );
  test.identical( got.original, exp );

  test.case = 'src - empty container, dst - filled array, no onEval';
  var src = _.containerAdapter.make( [] );
  var dst = [ 1, 2, 1, 2, null, null, undefined, false, undefined, false ];
  var exp = [ 1, 2, 1, 2, null, null, undefined, false, undefined, false ];
  var got = src.once( dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'src - filled container, dst - empty array, no onEval';
  var src = _.containerAdapter.make( [ 1, 2, 1, 2, null, null, undefined, false, undefined, false ] );
  var dst = [];
  var exp = [ 1, 2, null, undefined, false ];
  var got = src.once( dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'src - filled container, dst - filled array, no onEval';
  var src = _.containerAdapter.make( [ 1, 2, 1, 2, null, null, undefined, false, undefined, false, [ 2 ], [ 2 ], [ 2 ] ] );
  var dst = [ 1, 2, null, undefined, false ];
  var exp = [ 1, 2, null, undefined, false, [ 2 ], [ 2 ], [ 2 ] ];
  var got = src.once( dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'src - filled container, dst - filled array, same content, no onEval';
  var src = _.containerAdapter.make( [ 1, 2, 1, 2, null, null, undefined, false, undefined, false ] );
  var dst = [ 1, 2, 1, 2, null, null, undefined, false, undefined, false ];
  var exp = [ 1, 2, 1, 2, null, null, undefined, false, undefined, false ];
  var got = src.once( dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'src - filled container, dst - filled arrayAdapter, same content, no onEval, evaluator returns element';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var dst = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 }, [ 2 ], { a : 0 } ];
  var got = src.once( dst, ( e ) => e );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'src - filled container, dst - filled arrayAdapter, same content, equalizer';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var dst = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 }, new U8x( 2 ) ] );
  var exp = [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 }, new U8x( 2 ), [ 2 ], { a : 0 } ];
  var got = src.once( dst, ( e, e2 ) => e === e2 );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'src - filled container, dst - filled arrayAdapter, onEval returns undefined';
  var src = _.containerAdapter.make( [ 0, 0, 1, 1, undefined, undefined, undefined, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var dst = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ];
  var got = src.once( dst, ( e ) => undefined );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'src - filled container, dst - filled array, evaluator checks element of array';
  var src = _.containerAdapter.make( [ 0, 1, [ null ], [ true ], false, [ undefined ], '', [ 2 ], { a : 0 } ] );
  var dst = _.containerAdapter.make( [ 0, 1, [ null ], [ true ], false, [ undefined ], '', [ 2 ], { a : 0 } ] );
  var exp = [ 0, 1, [ null ], [ true ], false, [ undefined ], '', [ 2 ], { a : 0 } ];
  var got = src.once( dst, ( e ) => e[ 0 ] );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'src - filled container, dst - filled array, duplicates in src, evaluator checks element of array';
  var src = _.containerAdapter.make( [ 0, 1, [ null ], [ true ], [ true ], [ true ], false, [ undefined ], '', [ 2 ], { a : 0 } ] );
  var dst = _.containerAdapter.make( [ 0, 1, [ null ], [ true ], false, [ undefined ], '', [ 2 ], { a : 0 } ] );
  var exp = [ 0, 1, [ null ], [ true ], false, [ undefined ], '', [ 2 ], { a : 0 } ];
  var got = src.once( dst, ( e ) => e[ 0 ] );
  test.true( got !== src );
  test.identical( got.original, exp );

  /* */

  test.case = 'src - empty container, dst - empty Set, no onEval';
  var src = _.containerAdapter.make( [] );
  var dst = new Set( [] );
  var exp = [];
  var got = src.once( dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - empty container, dst - empty ContainerAdapterSet, no onEval';
  var src = _.containerAdapter.make( [] );
  var dst = _.containerAdapter.make( new Set( [] ) );
  var exp = [];
  var got = src.once( dst );
  test.true( got !== src );
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - empty container, dst - filled Set, no onEval';
  var src = _.containerAdapter.make( [] );
  var dst = new Set( [ 1, 2, 1, 2, null, null, undefined, false, undefined, false ] );
  var exp = [ 1, 2, null, undefined, false ];
  var got = src.once( dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - filled container, dst - empty Set, no onEval';
  var src = _.containerAdapter.make( [ 1, 2, 1, 2, null, null, undefined, false, undefined, false ] );
  var dst = new Set( [] );
  var exp = [ 1, 2, null, undefined, false ];
  var got = src.once( dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - filled container, dst - filled Set, no onEval';
  var src = _.containerAdapter.make( [ 1, 2, 1, 2, null, null, undefined, false, undefined, false, [ 2 ], [ 2 ], [ 2 ] ] );
  var dst = new Set( [ 1, 2, null, undefined, false ] );
  var exp = [ 1, 2, null, undefined, false, [ 2 ], [ 2 ], [ 2 ] ];
  var got = src.once( dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - filled container, dst - filled Set, same content, no onEval';
  var src = _.containerAdapter.make( [ 1, 2, 1, 2, null, null, undefined, false, undefined, false ] );
  var dst = new Set( [ 1, 2, 1, 2, null, null, undefined, false, undefined, false ] );
  var exp = [ 1, 2, null, undefined, false ];
  var got = src.once( dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - filled container, dst - filled setAdapter, same content, one evaluator';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var dst = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 }, [ 2 ], { a : 0 } ];
  var got = src.once( dst, ( e ) => e );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - filled container, dst - filled setAdapter, same content, equalizer';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var dst = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 }, new U8x( 2 ) ] ) );
  var exp = [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 }, new U8x( 2 ), [ 2 ], { a : 0 } ];
  var got = src.once( dst, ( e, e2 ) => e === e2 );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - filled container, dst - filled setAdapter, evaluator returns undefined';
  var src = _.containerAdapter.make( [ 0, 0, 1, 1, undefined, undefined, undefined, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var dst = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ];
  var got = src.once( dst, ( e ) => undefined );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - filled container, dst - filled setAdapter, evaluator checks element of array';
  var src = _.containerAdapter.make( [ 0, 1, [ null ], [ true ], false, [ undefined ], '', [ 2 ], { a : 0 } ] );
  var dst = _.containerAdapter.make( new Set( [ 0, 1, [ null ], [ true ], false, [ undefined ], '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 0, 1, [ null ], [ true ], false, [ undefined ], '', [ 2 ], { a : 0 } ];
  var got = src.once( dst, ( e ) => e[ 0 ] );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - filled container, dst - filled setAdapter, duplicates in src, evaluator checks element of array';
  var src = _.containerAdapter.make( [ 0, 1, [ null ], [ true ], [ true ], [ true ], false, [ undefined ], '', [ 2 ], { a : 0 } ] );
  var dst = _.containerAdapter.make( new Set( [ 0, 1, [ null ], [ true ], false, [ undefined ], '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 0, 1, [ null ], [ true ], false, [ undefined ], '', [ 2 ], { a : 0 } ];
  var got = src.once( dst, ( e ) => e[ 0 ] );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - filled container, dst - filled setAdapter, duplicates in src, two evaluators checks element of array';
  var src = _.containerAdapter.make( [ 0, 1, [ null ], [ true ], [ true ], [ true ], false, [ undefined ], '', [ 2 ], { a : 0 } ] );
  var dst = _.containerAdapter.make( new Set( [ 0, 1, [ null ], [ true ], false, [ undefined ], '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 0, 1, [ null ], [ true ], false, [ undefined ], '', [ 2 ], { a : 0 } ];
  var got = src.once( dst, ( e ) => e[ 0 ], ( ins ) => ins[ 0 ] );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.close( 'only dst' );

  /* - */

  test.open( 'dst === null' );

  test.case = 'src - empty container, no onEval';
  var src = _.containerAdapter.make( [] );
  var got = src.once( null );
  var exp = [];
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'src - filled container, no onEval';
  var src = _.containerAdapter.make( [ 1, 2, 1, 2, null, null, undefined, false, undefined, false ] );
  var got = src.once( null );
  var exp = [ 1, 2, null, undefined, false ];
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'src - filled container, complex data, no onEval';
  var src = _.containerAdapter.make( [ 1, 2, 1, 2, null, null, undefined, false, undefined, false, [ 2 ], [ 2 ], [ 2 ] ] );
  var got = src.once( null );
  var exp = [ 1, 2, null, undefined, false, [ 2 ], [ 2 ], [ 2 ] ];
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'src - filled container, evaluator returns element';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var got = src.once( null, ( e ) => e );
  var exp = [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ];
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'src - filled container, two evaluators returns element';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var got = src.once( null, ( e ) => e, ( ins ) => ins );
  var exp = [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ];
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'src - filled container, equalizer';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var got = src.once( null, ( e, e2 ) => e === e2 );
  var exp = [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ];
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'src - filled container, evaluator returns undefined';
  var src = _.containerAdapter.make( [ 0, 0, 1, 1, undefined, undefined, undefined, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var got = src.once( null, ( e ) => undefined );
  var exp = [ 0 ];
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'src - filled container, evaluator checks element of array, no duplicates in src';
  var src = _.containerAdapter.make( [ 0, 1, [ null ], [ true ], false, [ undefined ], '', [ 2 ], { a : 0 } ] );
  var got = src.once( null, ( e ) => e[ 0 ] );
  var exp = [ 0, [ null ], [ true ], [ 2 ] ];
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'src - filled container, evaluator checks element of array, duplicates in src';
  var src = _.containerAdapter.make( [ 0, 1, [ null ], [ true ], [ true ], [ true ], false, [ undefined ], '', [ 2 ], { a : 0 } ] );
  var got = src.once( null, ( e ) => e[ 0 ] );
  var exp = [ 0, [ null ], [ true ], [ 2 ] ];
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'src - filled container, two evaluators checks element of array, duplicates in src';
  var src = _.containerAdapter.make( [ 0, 1, [ null ], [ true ], [ true ], [ true ], false, [ undefined ], '', [ 2 ], { a : 0 } ] );
  var got = src.once( null, ( e ) => e[ 0 ], ( ins ) => ins[ 0 ] );
  var exp = [ 0, [ null ], [ true ], [ 2 ] ];
  test.true( got !== src );
  test.identical( got.original, exp );

  test.close( 'dst === null' );

  /* - */

  test.open( 'dst === src' );

  test.case = 'src - empty container, no onEval';
  var src = _.containerAdapter.make( [] );
  var got = src.once( src );
  var exp = [];
  test.true( got === src );
  test.identical( got.original, exp );

  test.case = 'src - filled container, no onEval';
  var src = _.containerAdapter.make( [ 1, 2, 1, 2, null, null, undefined, false, undefined, false ] );
  var got = src.once( src );
  var exp = [ 1, 2, null, undefined, false ];
  test.true( got === src );
  test.identical( got.original, exp );

  test.case = 'src - filled container, complex data, no onEval';
  var src = _.containerAdapter.make( [ 1, 2, 1, 2, null, null, undefined, false, undefined, false, [ 2 ], [ 2 ], [ 2 ] ] );
  var got = src.once( src );
  var exp = [ 1, 2, null, undefined, false, [ 2 ], [ 2 ], [ 2 ] ];
  test.true( got === src );
  test.identical( got.original, exp );

  test.case = 'src - filled container, complex data, evaluator returns element';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var got = src.once( src, ( e ) => e );
  var exp = [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ];
  test.true( got === src );
  test.identical( got.original, exp );

  test.case = 'src - filled container, complex data, equalizer';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var got = src.once( src, ( e, e2 ) => e === e2 );
  var exp = [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ];
  test.true( got === src );
  test.identical( got.original, exp );

  test.case = 'src - filled container, complex data, evaluator returns undefined';
  var src = _.containerAdapter.make( [ 0, 0, 1, 1, undefined, undefined, undefined, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var got = src.once( src, ( e ) => undefined );
  var exp = [ 0 ];
  test.true( got === src );
  test.identical( got.original, exp );

  test.case = 'src - filled container, complex data, evaluator checks element of array, no duplicates in src';
  var src = _.containerAdapter.make( [ 0, 1, [ null ], [ true ], false, [ undefined ], '', [ 2 ], { a : 0 } ] );
  var got = src.once( src, ( e ) => e[ 0 ] );
  var exp = [ 0, [ null ], [ true ], [ 2 ] ];
  test.true( got === src );
  test.identical( got.original, exp );

  test.case = 'src - filled container, complex data, two evaluators checks element of array, no duplicates in src';
  var src = _.containerAdapter.make( [ 0, 1, [ null ], [ true ], false, [ undefined ], '', [ 2 ], { a : 0 } ] );
  var got = src.once( src, ( e ) => e[ 0 ] );
  var exp = [ 0, [ null ], [ true ], [ 2 ] ];
  test.true( got === src );
  test.identical( got.original, exp );

  test.close( 'dst === src' );
}

//

function arrayAdapterOnceRight( test )
{
  test.case = 'without arguments';
  var src = _.containerAdapter.make( [] );
  var exp = [];
  var got = src.onceRight( null );
  test.true( got !== src );
  test.identical( got.original, exp );

  /* - */

  test.open( 'only onEval' );

  test.case = 'evaluator returns element';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ { 'a' : 0 }, [ 2 ], '', undefined, false, true, null, 1, 0 ];
  var got = src.onceRight( ( e ) => e );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'two evaluators returns element';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ { 'a' : 0 }, [ 2 ], '', undefined, false, true, null, 1, 0 ];
  var got = src.onceRight( ( e ) => e, ( ins ) => ins );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'equalizer';
  var src = _.containerAdapter.make( [ 0, 0, 1, 1, null, true, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ { 'a' : 0 }, [ 2 ], '', undefined, false, true, null, 1, 0 ];
  var got = src.onceRight( ( e, e2 ) => e === e2 );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'evaluator returns undefined';
  var src = _.containerAdapter.make( [ 0, 0, 1, 1, undefined, undefined, undefined, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ { a : 0 } ];
  var got = src.onceRight( ( e ) => undefined );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'evaluator checks element of array';
  var src = _.containerAdapter.make( [ 0, 1, [ null ], [ true ], false, [ undefined ], '', [ 2 ], { a : 0 } ] );
  var exp = [ { a : 0 }, [ 2 ], [ true ], [ null ] ];
  var got = src.onceRight( ( e ) => e[ 0 ] );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.close( 'only onEval' );

  /* - */

  test.open( 'only dst' );

  test.case = 'src - empty container, dst - empty array, no onEval';
  var src = _.containerAdapter.make( [] );
  var dst = [];
  var exp = [];
  var got = src.onceRight( dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'src - empty container, dst - empty arrayAdapter, no onEval';
  var src = _.containerAdapter.make( [] );
  var dst = _.containerAdapter.make( [] );
  var exp = [];
  var got = src.onceRight( dst );
  test.true( got !== src );
  test.true( got === dst );
  test.identical( got.original, exp );

  test.case = 'src - empty container, dst - filled array, no onEval';
  var src = _.containerAdapter.make( [] );
  var dst = [ 1, 2, 1, 2, null, null, undefined, false, undefined, false ];
  var exp = [ 1, 2, 1, 2, null, null, undefined, false, undefined, false ];
  var got = src.onceRight( dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'src - filled container, dst - empty array, no onEval';
  var src = _.containerAdapter.make( [ 1, 2, 1, 2, null, null, undefined, false, undefined, false ] );
  var dst = [];
  var exp = [ false, undefined, null, 2, 1 ];
  var got = src.onceRight( dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'src - filled container, dst - filled array, no onEval';
  var src = _.containerAdapter.make( [ 1, 2, 1, 2, null, null, undefined, false, undefined, false, [ 2 ], [ 2 ], [ 2 ] ] );
  var dst = [ 1, 2, null, undefined, false ];
  var exp = [ 1, 2, null, undefined, false, [ 2 ], [ 2 ], [ 2 ] ];
  var got = src.onceRight( dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'src - filled container, dst - filled array, same content, no onEval';
  var src = _.containerAdapter.make( [ 1, 2, 1, 2, null, null, undefined, false, undefined, false ] );
  var dst = [ 1, 2, 1, 2, null, null, undefined, false, undefined, false ];
  var exp = [ 1, 2, 1, 2, null, null, undefined, false, undefined, false ];
  var got = src.onceRight( dst );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'src - filled container, dst - filled arrayAdapter, same content, no onEval, evaluator returns element';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var dst = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 }, { a : 0 }, [ 2 ] ];
  var got = src.onceRight( dst, ( e ) => e );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'src - filled container, dst - filled arrayAdapter, same content, equalizer';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var dst = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 }, new U8x( 2 ) ] );
  var exp = [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 }, new U8x( 2 ), { a : 0 }, [ 2 ] ];
  var got = src.onceRight( dst, ( e, e2 ) => e === e2 );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'src - filled container, dst - filled arrayAdapter, onEval returns undefined';
  var src = _.containerAdapter.make( [ 0, 0, 1, 1, undefined, undefined, undefined, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var dst = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ];
  var got = src.onceRight( dst, ( e ) => undefined );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'src - filled container, dst - filled array, evaluator checks element of array';
  var src = _.containerAdapter.make( [ 0, 1, [ null ], [ true ], false, [ undefined ], '', [ 2 ], { a : 0 } ] );
  var dst = _.containerAdapter.make( [ 0, 1, [ null ], [ true ], false, [ undefined ], '', [ 2 ], { a : 0 } ] );
  var exp = [ 0, 1, [ null ], [ true ], false, [ undefined ], '', [ 2 ], { a : 0 } ];
  var got = src.onceRight( dst, ( e ) => e[ 0 ] );
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'src - filled container, dst - filled array, duplicates in src, evaluator checks element of array';
  var src = _.containerAdapter.make( [ 0, 1, [ null ], [ true ], [ true ], [ true ], false, [ undefined ], '', [ 2 ], { a : 0 } ] );
  var dst = _.containerAdapter.make( [ 0, 1, [ null ], [ true ], false, [ undefined ], '', [ 2 ], { a : 0 } ] );
  var exp = [ 0, 1, [ null ], [ true ], false, [ undefined ], '', [ 2 ], { a : 0 } ];
  var got = src.onceRight( dst, ( e ) => e[ 0 ] );
  test.true( got !== src );
  test.identical( got.original, exp );

  /* */

  test.case = 'src - empty container, dst - empty Set, no onEval';
  var src = _.containerAdapter.make( [] );
  var dst = new Set( [] );
  var exp = [];
  var got = src.onceRight( dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - empty container, dst - empty ContainerAdapterSet, no onEval';
  var src = _.containerAdapter.make( [] );
  var dst = _.containerAdapter.make( new Set( [] ) );
  var exp = [];
  var got = src.onceRight( dst );
  test.true( got !== src );
  test.true( got === dst );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - empty container, dst - filled Set, no onEval';
  var src = _.containerAdapter.make( [] );
  var dst = new Set( [ 1, 2, 1, 2, null, null, undefined, false, undefined, false ] );
  var exp = [ 1, 2, null, undefined, false ];
  var got = src.onceRight( dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - filled container, dst - empty Set, no onEval';
  var src = _.containerAdapter.make( [ 1, 2, 1, 2, null, null, undefined, false, undefined, false ] );
  var dst = new Set( [] );
  var exp = [ false, undefined, null, 2, 1 ];
  var got = src.onceRight( dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - filled container, dst - filled Set, no onEval';
  var src = _.containerAdapter.make( [ 1, 2, 1, 2, null, null, undefined, false, undefined, false, [ 2 ], [ 2 ], [ 2 ] ] );
  var dst = new Set( [ 1, 2, null, undefined, false ] );
  var exp = [ 1, 2, null, undefined, false, [ 2 ], [ 2 ], [ 2 ] ];
  var got = src.onceRight( dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - filled container, dst - filled Set, same content, no onEval';
  var src = _.containerAdapter.make( [ 1, 2, 1, 2, null, null, undefined, false, undefined, false ] );
  var dst = new Set( [ 1, 2, 1, 2, null, null, undefined, false, undefined, false ] );
  var exp = [ 1, 2, null, undefined, false ];
  var got = src.onceRight( dst );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - filled container, dst - filled setAdapter, same content, one evaluator';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var dst = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 }, { a : 0 }, [ 2 ] ];
  var got = src.onceRight( dst, ( e ) => e );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - filled container, dst - filled setAdapter, same content, equalizer';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var dst = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 }, new U8x( 2 ) ] ) );
  var exp = [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 }, new U8x( 2 ), { a : 0 }, [ 2 ] ];
  var got = src.onceRight( dst, ( e, e2 ) => e === e2 );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - filled container, dst - filled setAdapter, evaluator returns undefined';
  var src = _.containerAdapter.make( [ 0, 0, 1, 1, undefined, undefined, undefined, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var dst = _.containerAdapter.make( new Set( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ];
  var got = src.onceRight( dst, ( e ) => undefined );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - filled container, dst - filled setAdapter, evaluator checks element of array';
  var src = _.containerAdapter.make( [ 0, 1, [ null ], [ true ], false, [ undefined ], '', [ 2 ], { a : 0 } ] );
  var dst = _.containerAdapter.make( new Set( [ 0, 1, [ null ], [ true ], false, [ undefined ], '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 0, 1, [ null ], [ true ], false, [ undefined ], '', [ 2 ], { a : 0 } ];
  var got = src.onceRight( dst, ( e ) => e[ 0 ] );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - filled container, dst - filled setAdapter, duplicates in src, evaluator checks element of array';
  var src = _.containerAdapter.make( [ 0, 1, [ null ], [ true ], [ true ], [ true ], false, [ undefined ], '', [ 2 ], { a : 0 } ] );
  var dst = _.containerAdapter.make( new Set( [ 0, 1, [ null ], [ true ], false, [ undefined ], '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 0, 1, [ null ], [ true ], false, [ undefined ], '', [ 2 ], { a : 0 } ];
  var got = src.onceRight( dst, ( e ) => e[ 0 ] );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.case = 'src - filled container, dst - filled setAdapter, duplicates in src, two evaluators checks element of array';
  var src = _.containerAdapter.make( [ 0, 1, [ null ], [ true ], [ true ], [ true ], false, [ undefined ], '', [ 2 ], { a : 0 } ] );
  var dst = _.containerAdapter.make( new Set( [ 0, 1, [ null ], [ true ], false, [ undefined ], '', [ 2 ], { a : 0 } ] ) );
  var exp = [ 0, 1, [ null ], [ true ], false, [ undefined ], '', [ 2 ], { a : 0 } ];
  var got = src.onceRight( dst, ( e ) => e[ 0 ], ( ins ) => ins[ 0 ] );
  test.true( got !== src );
  test.identical( [ ... got.original ], exp );

  test.close( 'only dst' );

  /* - */

  test.open( 'dst === null' );

  test.case = 'src - empty container, no onEval';
  var src = _.containerAdapter.make( [] );
  var got = src.onceRight( null );
  var exp = [];
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'src - filled container, no onEval';
  var src = _.containerAdapter.make( [ 1, 2, 1, 2, null, null, undefined, false, undefined, false ] );
  var got = src.onceRight( null );
  var exp = [ false, undefined, null, 2, 1 ];
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'src - filled container, complex data, no onEval';
  var src = _.containerAdapter.make( [ 1, 2, 1, 2, null, null, undefined, false, undefined, false, [ 2 ], [ 2 ], [ 2 ] ] );
  var got = src.onceRight( null );
  var exp = [ [ 2 ], [ 2 ], [ 2 ], false, undefined, null, 2, 1 ];
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'src - filled container, evaluator returns element';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var got = src.onceRight( null, ( e ) => e );
  var exp = [ { 'a' : 0 }, [ 2 ], '', undefined, false, true, null, 1, 0 ];
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'src - filled container, two evaluators returns element';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var got = src.onceRight( null, ( e ) => e, ( ins ) => ins );
  var exp = [ { 'a' : 0 }, [ 2 ], '', undefined, false, true, null, 1, 0 ];
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'src - filled container, equalizer';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var got = src.onceRight( null, ( e, e2 ) => e === e2 );
  var exp = [ { 'a' : 0 }, [ 2 ], '', undefined, false, true, null, 1, 0 ];
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'src - filled container, evaluator returns undefined';
  var src = _.containerAdapter.make( [ 0, 0, 1, 1, undefined, undefined, undefined, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var got = src.onceRight( null, ( e ) => undefined );
  var exp = [ { a : 0 } ];
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'src - filled container, evaluator checks element of array, no duplicates in src';
  var src = _.containerAdapter.make( [ 0, 1, [ null ], [ true ], false, [ undefined ], '', [ 2 ], { a : 0 } ] );
  var got = src.onceRight( null, ( e ) => e[ 0 ] );
  var exp = [ { a : 0 }, [ 2 ], [ true ], [ null ] ];
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'src - filled container, evaluator checks element of array, duplicates in src';
  var src = _.containerAdapter.make( [ 0, 1, [ null ], [ true ], [ true ], [ true ], false, [ undefined ], '', [ 2 ], { a : 0 } ] );
  var got = src.onceRight( null, ( e ) => e[ 0 ] );
  var exp = [ { a : 0 }, [ 2 ], [ true ], [ null ] ];
  test.true( got !== src );
  test.identical( got.original, exp );

  test.case = 'src - filled container, two evaluators checks element of array, duplicates in src';
  var src = _.containerAdapter.make( [ 0, 1, [ null ], [ true ], [ true ], [ true ], false, [ undefined ], '', [ 2 ], { a : 0 } ] );
  var got = src.onceRight( null, ( e ) => e[ 0 ], ( ins ) => ins[ 0 ] );
  var exp = [ { a : 0 }, [ 2 ], [ true ], [ null ] ];
  test.true( got !== src );
  test.identical( got.original, exp );

  test.close( 'dst === null' );

  /* - */

  test.open( 'dst === src' );

  test.case = 'src - empty container, no onEval';
  var src = _.containerAdapter.make( [] );
  var got = src.onceRight( src );
  var exp = [];
  test.true( got === src );
  test.identical( got.original, exp );

  test.case = 'src - filled container, no onEval';
  var src = _.containerAdapter.make( [ 1, 2, 1, 2, null, null, undefined, false, undefined, false ] );
  var got = src.onceRight( src );
  var exp = [ 1, 2, null, undefined, false ];
  test.true( got === src );
  test.identical( got.original, exp );

  test.case = 'src - filled container, complex data, no onEval';
  var src = _.containerAdapter.make( [ 1, 2, 1, 2, null, null, undefined, false, undefined, false, [ 2 ], [ 2 ], [ 2 ] ] );
  var got = src.onceRight( src );
  var exp = [ 1, 2, null, undefined, false, [ 2 ], [ 2 ], [ 2 ] ];
  test.true( got === src );
  test.identical( got.original, exp );

  test.case = 'src - filled container, complex data, evaluator returns element';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var got = src.onceRight( src, ( e ) => e );
  var exp = [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ];
  test.true( got === src );
  test.identical( got.original, exp );

  test.case = 'src - filled container, complex data, equalizer';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var got = src.onceRight( src, ( e, e2 ) => e === e2 );
  var exp = [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ];
  test.true( got === src );
  test.identical( got.original, exp );

  test.case = 'src - filled container, complex data, evaluator returns undefined';
  var src = _.containerAdapter.make( [ 0, 0, 1, 1, undefined, undefined, undefined, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var got = src.onceRight( src, ( e ) => undefined );
  var exp = [ 0 ];
  test.true( got === src );
  test.identical( got.original, exp );

  test.case = 'src - filled container, complex data, evaluator checks element of array, no duplicates in src';
  var src = _.containerAdapter.make( [ 0, 1, [ null ], [ true ], false, [ undefined ], '', [ 2 ], { a : 0 } ] );
  var got = src.onceRight( src, ( e ) => e[ 0 ] );
  var exp = [ 0, [ null ], [ true ], [ 2 ] ];
  test.true( got === src );
  test.identical( got.original, exp );

  test.case = 'src - filled container, complex data, two evaluators checks element of array, no duplicates in src';
  var src = _.containerAdapter.make( [ 0, 1, [ null ], [ true ], false, [ undefined ], '', [ 2 ], { a : 0 } ] );
  var got = src.onceRight( src, ( e ) => e[ 0 ] );
  var exp = [ 0, [ null ], [ true ], [ 2 ] ];
  test.true( got === src );
  test.identical( got.original, exp );

  test.close( 'dst === src' );
}

//

function arrayAdapterFirst( test )
{
  test.case = 'empty container';
  var src = _.containerAdapter.make( [] );
  var got = src.first();
  var exp = undefined;
  test.identical( got, exp );

  test.case = 'empty container, onEach';
  var src = _.containerAdapter.make( [] );
  var got = src.first( ( e ) => e );
  var exp = undefined;
  test.identical( got, exp );

  test.case = 'container, without onEach';
  var src = _.containerAdapter.make( [ 'first', 2, 3 ] );
  var got = src.first();
  var exp = 'first';
  test.identical( got, exp );

  test.case = 'container, onEach';
  var src = _.containerAdapter.make( [ 'first', 2, 3 ] );
  var got = src.first( ( e ) => e ? 1 : 0 );
  var exp = 1;
  test.identical( got, exp );

  test.case = 'container, onEach add key to element';
  var src = _.containerAdapter.make( [ 'first', 2, 3 ] );
  var got = src.first( ( e, k ) => e + k );
  var exp = 'first0';
  test.identical( got, exp );

  test.case = 'container, onEach check length of container';
  var src = _.containerAdapter.make( [ 'first', 2, 3 ] );
  var got = src.first( ( e, k, c ) => c.length > 2 ? e + 1 : e );
  var exp = 'first1';
  test.identical( got, exp );
}

//

function arrayAdapterLast( test )
{
  test.case = 'empty container';
  var src = _.containerAdapter.make( [] );
  var got = src.last();
  var exp = undefined;
  test.identical( got, exp );

  test.case = 'empty container, onEach';
  var src = _.containerAdapter.make( [] );
  var got = src.last( ( e ) => e );
  var exp = undefined;
  test.identical( got, exp );

  test.case = 'container, without onEach';
  var src = _.containerAdapter.make( [ 'first', 2, 3 ] );
  var got = src.last();
  var exp = 3;
  test.identical( got, exp );

  test.case = 'container, onEach';
  var src = _.containerAdapter.make( [ 'first', 2, 3 ] );
  var got = src.last( ( e ) => e ? 1 : 0 );
  var exp = 1;
  test.identical( got, exp );

  test.case = 'container, onEach returns sum of element and key';
  var src = _.containerAdapter.make( [ 'first', 2, 3 ] );
  var got = src.last( ( e, k ) => e + k );
  var exp = 5;
  test.identical( got, exp );

  test.case = 'container, onEach check container';
  var src = _.containerAdapter.make( [ 'first', 2, 3 ] );
  var got = src.last( ( e, k, c ) => c.length > 2 ? e + k : e );
  var exp = 5;
  test.identical( got, exp );
}

//

function arrayAdapterEach( test )
{
  test.case = 'without arguments';
  var src = _.containerAdapter.make( [] );
  var exp = [];
  var got = src.each( ( e ) => exp.push( e ) );
  test.true( got === src );
  test.identical( got.original, exp );
  test.identical( exp, [] );

  test.case = 'from empty array, onEach returns number';
  var src = _.containerAdapter.make( [] );
  var exp = [];
  var got = src.each( ( e ) => exp.push( 123 ) );
  test.true( got === src );
  test.identical( got.original, exp );
  test.identical( exp, [] );

  test.case = 'from array, onEach returns original';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [];
  var got = src.each( ( e ) => exp.push( e ) );
  test.true( got === src );
  test.identical( got.original, exp );
  test.identical( exp, [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );

  test.case = 'from array, onEach returns key';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [];
  var got = src.each( ( e, k ) => exp.push( k ) );
  test.true( got === src );
  test.identical( got.original, [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  test.identical( exp, [ 0, 1, 2, 3, 4, 5, 6, 7, 8 ] );

  test.case = 'from array, onEach check container';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [];
  var got = src.each( ( e, k, c ) => c.length > 10 ? exp.push( k ) : exp.push( c.length - 1 - k ) );
  test.true( got === src );
  test.identical( got.original, [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  test.identical( exp, [ 8, 7, 6, 5, 4, 3, 2, 1, 0 ] );

  test.case = 'from array, onEach returns undefined';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [];
  var got = src.each( ( e ) => exp.push( undefined ) );
  test.true( got === src );
  test.identical( got.original, [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  test.identical( exp, [ undefined, undefined, undefined, undefined, undefined, undefined, undefined, undefined, undefined ] );

  test.case = 'from array, onEach returns array';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [];
  var got = src.each( ( e ) => exp.push( [ e ] ) );
  test.true( got === src );
  test.identical( got.original, [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  test.identical( exp, [ [ 0 ], [ 1 ], [ null ], [ true ], [ false ], [ undefined ], [ '' ], [ [ 2 ] ], [ { a : 0 } ] ] );

  test.case = 'from array, onEach returns element of array';
  var src = _.containerAdapter.make( [ 0, 1, NaN, true, false, [ undefined ], '', [ 2 ], { a : 0 } ] );
  var exp = [];
  var got = src.each( ( e ) => exp.push( e[ 0 ] ) );
  test.true( got === src );
  test.identical( got.original, [ 0, 1, NaN, true, false, [ undefined ], '', [ 2 ], { a : 0 } ] );
  test.identical( exp, [ undefined, undefined, undefined, undefined, undefined, undefined, undefined, 2, undefined ] );
}

//

function arrayAdapterEachRight( test )
{
  test.case = 'without arguments';
  var src = _.containerAdapter.make( [] );
  var exp = [];
  var got = src.eachRight( ( e ) => exp.push( e ) );
  test.true( got === src );
  test.identical( got.original, exp );
  test.identical( exp, [] );

  test.case = 'from empty array, onEach returns number';
  var src = _.containerAdapter.make( [] );
  var exp = [];
  var got = src.eachRight( ( e ) => exp.push( 123 ) );
  test.true( got === src );
  test.identical( got.original, exp );
  test.identical( exp, [] );

  test.case = 'from array, onEach returns original';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [];
  var got = src.eachRight( ( e ) => exp.push( e ) );
  test.true( got === src );
  test.identical( got.original, src.original );
  test.identical( exp, [ { a : 0 }, [ 2 ], '', undefined, false, true, null, 1, 0 ] );

  test.case = 'from array, onEach returns key';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [];
  var got = src.eachRight( ( e, k ) => exp.push( k ) );
  test.true( got === src );
  test.identical( got.original, [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  test.identical( exp, [ 8, 7, 6, 5, 4, 3, 2, 1, 0 ] );

  test.case = 'from array, onEach check container';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [];
  var got = src.eachRight( ( e, k, c ) => c.length > 10 ? exp.push( k ) : exp.push( c.length - 1 - k ) );
  test.true( got === src );
  test.identical( got.original, [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  test.identical( exp, [ 0, 1, 2, 3, 4, 5, 6, 7, 8 ] );

  test.case = 'from array, onEach returns undefined';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [];
  var got = src.eachRight( ( e ) => exp.push( undefined ) );
  test.true( got === src );
  test.identical( got.original, [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  test.identical( exp, [ undefined, undefined, undefined, undefined, undefined, undefined, undefined, undefined, undefined ] );

  test.case = 'from array, onEach returns array';
  var src = _.containerAdapter.make( [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  var exp = [];
  var got = src.eachRight( ( e ) => exp.push( [ e ] ) );
  test.true( got === src );
  test.identical( got.original, [ 0, 1, null, true, false, undefined, '', [ 2 ], { a : 0 } ] );
  test.identical( exp, [ [ { a : 0 } ], [ [ 2 ] ], [ '' ], [ undefined ], [ false ], [ true ], [ null ], [ 1 ], [ 0 ] ] );

  test.case = 'from array, onEach returns element of array';
  var src = _.containerAdapter.make( [ 0, 1, NaN, true, false, [ undefined ], '', [ 2 ], { a : 0 } ] );
  var exp = [];
  var got = src.eachRight( ( e ) => exp.push( e[ 0 ] ) );
  test.true( got === src );
  test.identical( got.original, [ 0, 1, NaN, true, false, [ undefined ], '', [ 2 ], { a : 0 } ] );
  test.identical( exp, [ undefined, 2, undefined, undefined, undefined, undefined, undefined, undefined, undefined ] );
}

//

function arrayAdapterReduce( test )
{
  test.case = 'without accumulator, empty src';
  var src = _.containerAdapter.make( [] );
  var got = src.reduce( ( a, e ) => Math.max( a, e ) );
  test.identical( src.original, [] );
  test.identical( got, undefined );

  test.case = 'without accumulator, not empty src';
  var src = _.containerAdapter.make( [ 1, 2, 3, 0, -2 ] );
  var got = src.reduce( ( a, e ) => a === undefined ? Math.max( 0, e ) : Math.max( a, e ) );
  test.identical( src.original, [ 1, 2, 3, 0, -2 ] );
  test.identical( got, 3 );

  test.case = 'without accumulator, not empty src, use key';
  var src = _.containerAdapter.make( [ 1, 2, 3, undefined, -2 ] );
  var got = src.reduce( ( a, e, k ) => e === undefined ? Math.max( 0, k ) : Math.max( a, e ) );
  test.identical( src.original, [ 1, 2, 3, undefined, -2 ] );
  test.identical( got, 3 );

  test.case = 'without accumulator, not empty src, use container';
  var src = _.containerAdapter.make( [ 1, 2, 3, undefined, -2 ] );
  var got = src.reduce( ( a, e, k, c ) => c.length > 2 ? Math.max( 0, e ) : Math.max( a, e ) );
  test.identical( src.original, [ 1, 2, 3, undefined, -2 ] );
  test.identical( got, 0 );

  /* */

  test.case = 'with accumulator, empty src';
  var accumulator = [ 0 ];
  var src = _.containerAdapter.make( [] );
  var got = src.reduce( accumulator, ( a, e ) => a[ a.length - 1 ] > e ? a[ a.length - 1 ] : a.push( e ) );
  test.identical( src.original, [] );
  test.true( got === accumulator );
  test.identical( got, [ 0 ] );

  test.case = 'with accumulator, not empty src';
  var accumulator = [ 0 ];
  var src = _.containerAdapter.make( [ 1, 2, 3, 0, -2 ] );
  var got = src.reduce( accumulator, ( a, e ) => a[ a.length - 1 ] < e ? _.arrayAppend( a, e ) : a );
  test.identical( src.original, [ 1, 2, 3, 0, -2 ] );
  test.true( got === accumulator );
  test.identical( got, [ 0, 1, 2, 3 ] );

  test.case = 'with accumulator, not empty src, use key';
  var accumulator = [ 0 ];
  var src = _.containerAdapter.make( [ 1, 2, 3, undefined, -2 ] );
  var got = src.reduce( accumulator, ( a, e, k ) => e === undefined ? _.arrayAppend( a, k ) : _.arrayAppend( a, Math.max( a[ a.length - 1 ], e ) ) );
  test.identical( src.original, [ 1, 2, 3, undefined, -2 ] );
  test.identical( got, [ 0, 1, 2, 3, 3, 3 ] );

  test.case = 'with accumulator, not empty src, use container';
  var accumulator = [];
  var src = _.containerAdapter.make( [ 1, 2, 3, undefined, -2 ] );
  var got = src.reduce( accumulator, ( a, e, k, c ) => c.length > 2 ? _.arrayAppend( a, k ) : _.arrayAppend( a, e ) );
  test.identical( src.original, [ 1, 2, 3, undefined, -2 ] );
  test.identical( got, [ 0, 1, 2, 3, 4 ] );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'onEach has wrong type';
  test.shouldThrowErrorSync( () =>
  {
    let src = _.containerAdapter.make( [ 1, 2, 3 ] );
    src.reduce( 'wrong' );
  });
  test.shouldThrowErrorSync( () =>
  {
    let accumulator = 0;
    let src = _.containerAdapter.make( [ 1, 2, 3 ] );
    src.reduce( accumulator, 'wrong' );
  });
}

//

function arrayAdapterReduceRight( test )
{
  test.case = 'without accumulator, empty src';
  var src = _.containerAdapter.make( [] );
  var got = src.reduceRight( ( a, e ) => Math.max( a, e ) );
  test.identical( src.original, [] );
  test.identical( got, undefined );

  test.case = 'without accumulator, not empty src';
  var src = _.containerAdapter.make( [ 1, 2, 3, 0, -2 ] );
  var got = src.reduceRight( ( a, e ) => a === undefined ? Math.max( 0, e ) : Math.max( a, e ) );
  test.identical( src.original, [ 1, 2, 3, 0, -2 ] );
  test.identical( got, 3 );

  test.case = 'without accumulator, not empty src, use key';
  var src = _.containerAdapter.make( [ 1, 2, 3, undefined, -2 ] );
  var got = src.reduceRight( ( a, e, k ) => e === undefined ? Math.max( 0, k ) : Math.max( a, e ) );
  test.identical( src.original, [ 1, 2, 3, undefined, -2 ] );
  test.identical( got, 3 );

  test.case = 'without accumulator, not empty src, use container';
  var src = _.containerAdapter.make( [ 1, 2, 3, undefined, -2 ] );
  var got = src.reduceRight( ( a, e, k, c ) => c.length > 2 ? Math.max( 0, e ) : Math.max( a, e ) );
  test.identical( src.original, [ 1, 2, 3, undefined, -2 ] );
  test.identical( got, 1 );

  /* */

  test.case = 'with accumulator, empty src';
  var accumulator = [ 0 ];
  var src = _.containerAdapter.make( [] );
  var got = src.reduceRight( accumulator, ( a, e ) => a[ a.length - 1 ] > e ? a[ a.length - 1 ] : a.push( e ) );
  test.identical( src.original, [] );
  test.true( got === accumulator );
  test.identical( got, [ 0 ] );

  test.case = 'with accumulator, not empty src';
  var accumulator = [ 0 ];
  var src = _.containerAdapter.make( [ 1, 2, 3, 0, -2 ] );
  var got = src.reduceRight( accumulator, ( a, e ) => a[ a.length - 1 ] < e ? _.arrayAppend( a, e ) : a );
  test.identical( src.original, [ 1, 2, 3, 0, -2 ] );
  test.true( got === accumulator );
  test.identical( got, [ 0, 3 ] );

  test.case = 'with accumulator, not empty src, use key';
  var accumulator = [ 0 ];
  var src = _.containerAdapter.make( [ 1, 2, 3, undefined, -2 ] );
  var got = src.reduceRight( accumulator, ( a, e, k ) => e === undefined ? _.arrayAppend( a, k ) : _.arrayAppend( a, Math.max( a[ a.length - 1 ], e ) ) );
  test.identical( src.original, [ 1, 2, 3, undefined, -2 ] );
  test.identical( got, [ 0, 0, 3, 3, 3, 3 ] );

  test.case = 'with accumulator, not empty src, use container';
  var accumulator = [];
  var src = _.containerAdapter.make( [ 1, 2, 3, undefined, -2 ] );
  var got = src.reduceRight( accumulator, ( a, e, k, c ) => c.length > 2 ? _.arrayAppend( a, k ) : _.arrayAppend( a, e ) );
  test.identical( src.original, [ 1, 2, 3, undefined, -2 ] );
  test.identical( got, [ 4, 3, 2, 1, 0 ] );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'onEach has wrong type';
  test.shouldThrowErrorSync( () =>
  {
    let src = _.containerAdapter.make( [ 1, 2, 3 ] );
    src.reduceRight( 'wrong' );
  });
  test.shouldThrowErrorSync( () =>
  {
    let accumulator = 0;
    let src = _.containerAdapter.make( [ 1, 2, 3 ] );
    src.reduceRight( accumulator, 'wrong' );
  });
}

//

function arrayAdapterAll( test )
{
  test.case = 'src - empty container, onEach returns element';
  var src = _.containerAdapter.make( [] );
  var got = src.all( ( e ) => e );
  test.identical( got, true );

  test.case = 'src - empty container, onEach';
  var src = _.containerAdapter.make( [] );
  var got = src.all( ( e ) => true );
  test.identical( got, true );

  test.case = 'all elements is defined, onEach returns element';
  var src = _.containerAdapter.make( [ 1, 'str', [ 0 ], { a : 0 }, true ] );
  var got = src.all( ( e ) => e );
  test.identical( got, true );

  test.case = 'all elements is defined, onEach returns incremented key';
  var src = _.containerAdapter.make( [ 1, 'str', [ 0 ], { a : 0 }, true ] );
  var got = src.all( ( e, i ) => ++i );
  test.identical( got, true );

  test.case = 'all elements is defined, onEach checks container';
  var src = _.containerAdapter.make( [ 1, 'str', [ 0 ], { a : 0 }, true ] );
  var got = src.all( ( e, i, c ) => c.length > 5  );
  test.identical( got, false );

  test.case = 'all elements is undefines, onEach returns element';
  var src = _.containerAdapter.make( [ false, null, 0, '', undefined ] );
  var got = src.all( ( e ) => e );
  test.identical( got, false );

  test.case = 'all elements is undefines, onEach';
  var src = _.containerAdapter.make( [ false, null, 0, '', undefined ] );
  var got = src.all( ( e, i, c ) => c.length > 2  );
  test.identical( got, true );

  test.case = 'all elements is undefines, onEach returns undefined';
  var src = _.containerAdapter.make( [ false, null, 0, '', undefined ] );
  var got = src.all( ( e ) => undefined  );
  test.identical( got, false );
}

//

function arrayAdapterAllRight( test )
{
  test.case = 'src - empty container, onEach returns element';
  var src = _.containerAdapter.make( [] );
  var got = src.allRight( ( e ) => e );
  test.identical( got, true );

  test.case = 'src - empty container, onEach';
  var src = _.containerAdapter.make( [] );
  var got = src.allRight( ( e ) => true );
  test.identical( got, true );

  test.case = 'all elements is defined, onEach returns element';
  var src = _.containerAdapter.make( [ 1, 'str', [ 0 ], { a : 0 }, true ] );
  var got = src.allRight( ( e ) => e );
  test.identical( got, true );

  test.case = 'all elements is defined, onEach returns incremented key';
  var src = _.containerAdapter.make( [ 1, 'str', [ 0 ], { a : 0 }, true ] );
  var got = src.allRight( ( e, i ) => ++i );
  test.identical( got, true );

  test.case = 'all elements is defined, onEach checks container';
  var src = _.containerAdapter.make( [ 1, 'str', [ 0 ], { a : 0 }, true ] );
  var got = src.allRight( ( e, i, c ) => c.length > 5  );
  test.identical( got, false );

  test.case = 'all elements is undefines, onEach returns element';
  var src = _.containerAdapter.make( [ false, null, 0, '', undefined ] );
  var got = src.allRight( ( e ) => e );
  test.identical( got, false );

  test.case = 'all elements is undefines, onEach';
  var src = _.containerAdapter.make( [ false, null, 0, '', undefined ] );
  var got = src.allRight( ( e, i, c ) => c.length > 2  );
  test.identical( got, true );

  test.case = 'all elements is undefines, onEach returns undefined';
  var src = _.containerAdapter.make( [ false, null, 0, '', undefined ] );
  var got = src.allRight( ( e ) => undefined  );
  test.identical( got, false );
}

//

function arrayAdapterAllLeftRightCompare( test )
{
  test.case = 'compare by indexes';
  var src = _.containerAdapter.make( [ 0, 1, 2, 3, 4, '', null, undefined ] );
  var temp1 = [];
  var got1 = src.allLeft( ( e, i ) => temp.push( [ e, i ] ) );
  var temp2 = [];
  var got2 = src.allRight( ( e, i ) => temp2.push( [ e, i ] ) );
}

//

function arrayAdapterAny( test )
{
  test.case = 'src - empty container, onEach returns element';
  var src = _.containerAdapter.make( [] );
  var got = src.any( ( e ) => e );
  test.identical( got, false );

  test.case = 'src - empty container, onEach returns true';
  var src = _.containerAdapter.make( [] );
  var got = src.any( ( e ) => true );
  test.identical( got, false );

  test.case = 'all elements is defined, onEach returns element';
  var src = _.containerAdapter.make( [ 1, 'str', [ 0 ], { a : 0 }, true ] );
  var got = src.any( ( e ) => e );
  test.identical( got, true );

  test.case = 'one elements is defined, onEach returns element';
  var src = _.containerAdapter.make( [ null, 0, '', false, undefined, true ] );
  var got = src.any( ( e ) => e );
  test.identical( got, true );

  test.case = 'one elements is defined, onEach returns key';
  var src = _.containerAdapter.make( [ null, 0, '', false, undefined, true ] );
  var got = src.any( ( e, i ) => i );
  test.identical( got, true );

  test.case = 'all elements is defined, onEach checks container';
  var src = _.containerAdapter.make( [ 1, 'str', [ 0 ], { a : 0 }, true ] );
  var got = src.any( ( e, i, c ) => c.length > 5  );
  test.identical( got, false );

  test.case = 'all elements is undefines, onEach returns element';
  var src = _.containerAdapter.make( [ false, null, 0, '', undefined ] );
  var got = src.any( ( e ) => e );
  test.identical( got, false );

  test.case = 'all elements defined, onEach checks undefined';
  var src = _.containerAdapter.make( [ false, null, 0, '', undefined ] );
  var got = src.any( ( e, i, c ) => e === undefined  );
  test.identical( got, true );
}

//

function arrayAdapterAnyRight( test )
{
  test.case = 'src - empty container, onEach returns element';
  var src = _.containerAdapter.make( [] );
  var got = src.anyRight( ( e ) => e );
  test.identical( got, false );

  test.case = 'src - empty container, onEach returns true';
  var src = _.containerAdapter.make( [] );
  var got = src.anyRight( ( e ) => true );
  test.identical( got, false );

  test.case = 'all elements is defined, onEach returns element';
  var src = _.containerAdapter.make( [ 1, 'str', [ 0 ], { a : 0 }, true ] );
  var got = src.anyRight( ( e ) => e );
  test.identical( got, true );

  test.case = 'one elements is defined, onEach returns element';
  var src = _.containerAdapter.make( [ null, 0, '', false, undefined, true ] );
  var got = src.anyRight( ( e ) => e );
  test.identical( got, true );

  test.case = 'one elements is defined, onEach returns key';
  var src = _.containerAdapter.make( [ null, 0, '', false, undefined, true ] );
  var got = src.anyRight( ( e, i ) => i );
  test.identical( got, true );

  test.case = 'all elements is defined, onEach checks container';
  var src = _.containerAdapter.make( [ 1, 'str', [ 0 ], { a : 0 }, true ] );
  var got = src.anyRight( ( e, i, c ) => c.length > 5  );
  test.identical( got, false );

  test.case = 'all elements is undefines, onEach returns element';
  var src = _.containerAdapter.make( [ false, null, 0, '', undefined ] );
  var got = src.anyRight( ( e ) => e );
  test.identical( got, false );

  test.case = 'all elements defined, onEach checks undefined';
  var src = _.containerAdapter.make( [ false, null, 0, '', undefined ] );
  var got = src.anyRight( ( e, i, c ) => e === undefined  );
  test.identical( got, true );

  test.case = 'all elements defined, onEach returns false if e === 0';
  var src = _.containerAdapter.make( [ false, null, 0, '', undefined ] );
  var exp = [];
  var got = src.anyRight( ( e ) =>
  {
    if( e === 0 )
    return true;
    else
    exp.push( e );
    return false;
  } );
  test.identical( exp, [ undefined, '' ] );
  test.identical( got, true );
}

//

function arrayAdapterNone( test )
{
  test.case = 'src - empty container, onEach returns element';
  var src = _.containerAdapter.make( [] );
  var got = src.none( ( e ) => e );
  test.identical( got, true );

  test.case = 'src - empty container, onEach returns false';
  var src = _.containerAdapter.make( [] );
  var got = src.none( ( e ) => false );
  test.identical( got, true );

  test.case = 'all elements is defined, onEach returns element';
  var src = _.containerAdapter.make( [ 1, 'str', [ 0 ], { a : 0 }, true ] );
  var got = src.none( ( e ) => e );
  test.identical( got, false );

  test.case = 'one elements is defined, onEach returns element';
  var src = _.containerAdapter.make( [ null, 0, '', false, undefined, true ] );
  var got = src.none( ( e ) => e );
  test.identical( got, false );

  test.case = 'one elements is defined, onEach returns key';
  var src = _.containerAdapter.make( [ null, 0, '', false, undefined, true ] );
  var got = src.none( ( e, i ) => i > 8 ? i : undefined );
  test.identical( got, true );

  test.case = 'all elements is defined, onEach checks container';
  var src = _.containerAdapter.make( [ 1, 'str', [ 0 ], { a : 0 }, true ] );
  var got = src.none( ( e, i, c ) => c.length > 5  );
  test.identical( got, true );

  test.case = 'all elements is undefines, onEach returns element';
  var src = _.containerAdapter.make( [ false, null, 0, '', undefined ] );
  var got = src.none( ( e ) => e );
  test.identical( got, true );

  test.case = 'all elements defined, onEach checks undefined';
  var src = _.containerAdapter.make( [ false, null, 0, '', undefined ] );
  var got = src.none( ( e, i, c ) => e === undefined  );
  test.identical( got, false );
}

//

function arrayAdapterNoneRight( test )
{
  test.case = 'src - empty container, onEach returns element';
  var src = _.containerAdapter.make( [] );
  var got = src.noneRight( ( e ) => e );
  test.identical( got, true );

  test.case = 'src - empty container, onEach returns false';
  var src = _.containerAdapter.make( [] );
  var got = src.noneRight( ( e ) => false );
  test.identical( got, true );

  test.case = 'all elements is defined, onEach returns element';
  var src = _.containerAdapter.make( [ 1, 'str', [ 0 ], { a : 0 }, true ] );
  var got = src.noneRight( ( e ) => e );
  test.identical( got, false );

  test.case = 'one elements is defined, onEach returns element';
  var src = _.containerAdapter.make( [ null, 0, '', false, undefined, true ] );
  var got = src.noneRight( ( e ) => e );
  test.identical( got, false );

  test.case = 'one elements is defined, onEach returns key';
  var src = _.containerAdapter.make( [ null, 0, '', false, undefined, true ] );
  var got = src.noneRight( ( e, i ) => i );
  test.identical( got, false );

  test.case = 'all elements is defined, onEach checks container';
  var src = _.containerAdapter.make( [ 1, 'str', [ 0 ], { a : 0 }, true ] );
  var got = src.noneRight( ( e, i, c ) => c.length > 5  );
  test.identical( got, true );

  test.case = 'all elements is undefines, onEach returns element';
  var src = _.containerAdapter.make( [ false, null, 0, '', undefined ] );
  var got = src.noneRight( ( e ) => e );
  test.identical( got, true );

  test.case = 'all elements is undefines, onEach checks undefined';
  var src = _.containerAdapter.make( [ false, null, 0, '', undefined ] );
  var got = src.noneRight( ( e ) => e === undefined  );
  test.identical( got, false );

  test.case = 'all elements is undefines, onEach makes copy';
  var src = _.containerAdapter.make( [ false, null, 0, '', undefined ] );
  var exp = [];
  var got = src.noneRight( ( e ) =>
  {
    if( e === null )
    return true;
    else
    exp.push( e );
    return false;
  });
  test.identical( exp, [ undefined, '', 0 ] );
  test.identical( got, false );
}

//

function arrayAdapterLeft( test )
{
  test.case = 'empty container';
  var src = _.containerAdapter.make( [] );
  var got = src.left( 1 );
  test.identical( got, { index : -1 } );

  test.case = 'container has not searched element';
  var src = _.containerAdapter.make( [ 1, 2, 'str', [ 3 ], { a : 2 } ] );
  var got = src.left( 3 );
  test.identical( got, { index : -1 } );

  test.case = 'container has duplicated searched element';
  var src = _.containerAdapter.make( [ 1, 2, 3, 'str', [ 3 ], 3, { a : 2 } ] );
  var got = src.left( 3 );
  test.identical( got, { index : 2, element : 3 } );

  test.case = 'searches complex data without evaluators';
  var src = _.containerAdapter.make( [ 1, 2, 'str', [ 3 ], { a : 2 } ] );
  var got = src.left( [ 3 ] );
  test.identical( got, { index : -1 } );

  /* */

  test.case = 'container has not searched element, fromIndex';
  var src = _.containerAdapter.make( [ 1, 2, 3, 'str', [ 3 ], { a : 2 } ] );
  var got = src.left( 3, 4 );
  test.identical( got, { index : -1 } );

  test.case = 'container has duplicated searched element, fromIndex';
  var src = _.containerAdapter.make( [ 1, 2, 3, 'str', [ 3 ], 3, 'str', { a : 2 } ] );
  var got = src.left( 'str', 4 );
  test.identical( got, { index : 6, element : 'str' } );

  test.case = 'searches complex data, fromIndex';
  var src = _.containerAdapter.make( [ 1, 2, 'str', [ 3 ], { a : 2 } ] );
  var got = src.left( [ 3 ], 2 );
  test.identical( got, { index : -1 } );

  /* */

  test.case = 'container has not searched element, onEvaluate1';
  var src = _.containerAdapter.make( [ 1, 2, 'str', [ 3 ], { a : 2 } ] );
  var got = src.left( 3, ( e ) => typeof e );
  test.identical( got, { index : 0, element : 1 } );

  test.case = 'container has duplicated searched element, onEvaluate1';
  var src = _.containerAdapter.make( [ 1, 2, 3, 'str', [ 3 ], 3, 'str', { a : 2 } ] );
  var got = src.left( 'str', ( e ) => e );
  test.identical( got, { index : 3, element : 'str' } );

  test.case = 'searches complex data, onEvaluate1';
  var src = _.containerAdapter.make( [ 1, 2, 'str', [ 3 ], { a : 2 } ] );
  var got = src.left( [ 3 ], ( e ) => e[ 0 ] );
  test.identical( got, { index : 3, element : [ 3 ] } );

  /* */

  test.case = 'container has not searched element, fromIndex, onEvaluate1';
  var src = _.containerAdapter.make( [ 1, 2, 'str', [ 3 ], { a : 2 } ] );
  var got = src.left( 3, 2, ( e ) => typeof e );
  test.identical( got, { index : -1 } );

  test.case = 'container has duplicated searched element, fromIndex, onEvaluate1';
  var src = _.containerAdapter.make( [ 1, 2, 3, 'str', [ 3 ], 3, 'str', { a : 2 } ] );
  var got = src.left( 'str', 4, ( e ) => typeof e );
  test.identical( got, { index : 6, element : 'str' } );

  test.case = 'searches complex data, onEvaluate1';
  var src = _.containerAdapter.make( [ 1, 2, 'str', [ 3 ], { a : 2 } ] );
  var got = src.left( [ 3 ], 4, ( e ) => e[ 0 ] );
  test.identical( got, { index : -1 } );

  /* */

  test.case = 'container has not searched element, onEvaluate1, onEvaluate2';
  var src = _.containerAdapter.make( [ 1, 2, 'str', [ 3 ], { a : 2 } ] );
  var got = src.left( 3, ( e ) => e[ 0 ], ( ins ) => ins );
  test.identical( got, { index : 3, element : [ 3 ] } );

  test.case = 'container has duplicated searched element, onEvaluate1, onEvaluate2';
  var src = _.containerAdapter.make( [ 1, 2, 3, 'str', [ 3 ], 3, 'str', { a : 2 } ] );
  var got = src.left( 2, ( e ) => e.a, ( ins ) => ins );
  test.identical( got, { index : 7, element : { a : 2 } } );

  test.case = 'searches complex data, onEvaluate, onEvaluate2';
  var src = _.containerAdapter.make( [ 1, 2, 'str', [ 3 ], { a : 2 } ] );
  var got = src.left( 3, ( e ) => e[ 0 ], ( ins ) => ins );
  test.identical( got, { index : 3, element : [ 3 ] } );

  /* */

  test.case = 'container has not searched element, fromIndex, onEvaluate1, onEvaluate2';
  var src = _.containerAdapter.make( [ 1, 2, 'str', [ 3 ], { a : 2 } ] );
  var got = src.left( 3, 2, ( e ) => e[ 0 ], ( ins ) => ins );
  test.identical( got, { index : 3, element : [ 3 ] } );

  test.case = 'container has duplicated searched element, fromIndex, onEvaluate1, onEvaluate2';
  var src = _.containerAdapter.make( [ 1, 2, 3, 'str', [ 3 ], 3, 'str', { a : 2 } ] );
  var got = src.left( 2, 7, ( e ) => e.a, ( ins ) => ins );
  test.identical( got, { index : 7, element : { a : 2 } } );

  test.case = 'searches complex data, fromIndex, onEvaluate, onEvaluate2';
  var src = _.containerAdapter.make( [ 1, 2, 'str', [ 3 ], { a : 2 } ] );
  var got = src.left( 3, 4, ( e ) => e[ 0 ], ( ins ) => ins );
  test.identical( got, { index : -1 } );

  /* */

  test.case = 'container has not searched element, equalizer';
  var src = _.containerAdapter.make( [ 1, 2, 'str', [ 3 ], { a : 2 } ] );
  var got = src.left( 3, ( e, ins ) => e[ 0 ] === ins );
  test.identical( got, { index : 3, element : [ 3 ] } );

  test.case = 'container has duplicated searched element, equalizer';
  var src = _.containerAdapter.make( [ 1, 2, 3, 'str', [ 3 ], 3, 'str', { a : 2 } ] );
  var got = src.left( 2, ( e, ins ) => e.a === ins );
  test.identical( got, { index : 7, element : { a : 2 } } );

  test.case = 'searches complex data, equalizer';
  var src = _.containerAdapter.make( [ 1, 2, 'str', [ 3 ], { a : 2 } ] );
  var got = src.left( 3, ( e, ins ) => e[ 0 ] ===  ins );
  test.identical( got, { index : 3, element : [ 3 ] } );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () =>
  {
    var src = _.containerAdapter.make( [] );
    src.left();
  });

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () =>
  {
    var src = _.containerAdapter.make( [ 1, 2 ] );
    src.left( 1, 0, ( e ) => e, ( ins ) => ins, 'extra' );
  });

  test.case = 'onEvaluate1 is not a routine, not a number';
  test.shouldThrowErrorSync( () =>
  {
    var src = _.containerAdapter.make( [ 1, 2 ] );
    src.left( 1, 0, 'wrong' );
  });

  test.case = 'onEvaluate1 has wrong length';
  test.shouldThrowErrorSync( () =>
  {
    var src = _.containerAdapter.make( [ 1, 2 ] );
    src.left( 1, 0, () => 1 );
  });

  test.case = 'onEvaluate2 has wrong length';
  test.shouldThrowErrorSync( () =>
  {
    var src = _.containerAdapter.make( [ 1, 2 ] );
    src.left( 1, 0, ( e ) => e, () => 1 );
  });
}

//

function arrayAdapterRight( test )
{
  test.case = 'empty container';
  var src = _.containerAdapter.make( [] );
  var got = src.right( 1 );
  test.identical( got, { index : -1 } );

  test.case = 'container has not searched element';
  var src = _.containerAdapter.make( [ 1, 2, 'str', [ 3 ], { a : 2 } ] );
  var got = src.right( 3 );
  test.identical( got, { index : -1 } );

  test.case = 'container has duplicated searched element';
  var src = _.containerAdapter.make( [ 1, 2, 3, 'str', [ 3 ], 3, { a : 2 } ] );
  var got = src.right( 3 );
  test.identical( got, { index : 5, element : 3 } );

  test.case = 'searches complex data without evaluators';
  var src = _.containerAdapter.make( [ 1, 2, 'str', [ 3 ], { a : 2 } ] );
  var got = src.right( [ 3 ] );
  test.identical( got, { index : -1 } );

  /* */

  test.case = 'container has not searched element, fromIndex';
  var src = _.containerAdapter.make( [ 1, 2, 3, 'str', [ 3 ], { a : 2 } ] );
  var got = src.right( 3, 4 );
  test.identical( got, { index : 2, element : 3 } );

  test.case = 'container has duplicated searched element, fromIndex';
  var src = _.containerAdapter.make( [ 1, 2, 3, 'str', [ 3 ], 3, 'str', { a : 2 } ] );
  var got = src.right( 'str', 4 );
  test.identical( got, { index : 3, element : 'str' } );

  test.case = 'searches complex data, fromIndex';
  var src = _.containerAdapter.make( [ 1, 2, 'str', [ 3 ], { a : 2 } ] );
  var got = src.right( [ 3 ], 2 );
  test.identical( got, { index : -1 } );

  /* */

  test.case = 'container has not searched element, onEvaluate1';
  var src = _.containerAdapter.make( [ 1, 2, 'str', [ 3 ], { a : 2 } ] );
  var got = src.right( 3, ( e ) => typeof e );
  test.identical( got, { index : 1, element : 2 } );

  test.case = 'container has duplicated searched element, onEvaluate1';
  var src = _.containerAdapter.make( [ 1, 2, 3, 'str', [ 3 ], 3, 'str', { a : 2 } ] );
  var got = src.right( 'str', ( e ) => e );
  test.identical( got, { index : 6, element : 'str' } );

  test.case = 'searches complex data, onEvaluate1';
  var src = _.containerAdapter.make( [ 1, 2, 'str', [ 3 ], { a : 2 } ] );
  var got = src.right( [ 3 ], ( e ) => e[ 0 ] );
  test.identical( got, { index : 3, element : [ 3 ] } );

  /* */

  test.case = 'container has not searched element, fromIndex, onEvaluate1';
  var src = _.containerAdapter.make( [ 1, 2, 'str', [ 3 ], { a : 2 } ] );
  var got = src.right( 3, 2, ( e ) => typeof e );
  test.identical( got, { index : 1, element : 2 } );

  test.case = 'container has duplicated searched element, fromIndex, onEvaluate1';
  var src = _.containerAdapter.make( [ 1, 2, 3, 'str', [ 3 ], 3, 'str', { a : 2 } ] );
  var got = src.right( 'str', 4, ( e ) => typeof e );
  test.identical( got, { index : 3, element : 'str' } );

  test.case = 'searches complex data, onEvaluate1';
  var src = _.containerAdapter.make( [ 1, 2, 'str', [ 3 ], { a : 2 } ] );
  var got = src.right( [ 3 ], 4, ( e ) => e[ 0 ] );
  test.identical( got, { index : 3, element : [ 3 ] } );

  /* */

  test.case = 'container has not searched element, onEvaluate1, onEvaluate2';
  var src = _.containerAdapter.make( [ 1, 2, 'str', [ 3 ], { a : 2 } ] );
  var got = src.right( 3, ( e ) => e[ 0 ], ( ins ) => ins );
  test.identical( got, { index : 3, element : [ 3 ] } );

  test.case = 'container has duplicated searched element, onEvaluate1, onEvaluate2';
  var src = _.containerAdapter.make( [ 1, 2, 3, 'str', [ 3 ], 3, 'str', { a : 2 } ] );
  var got = src.right( 2, ( e ) => e.a, ( ins ) => ins );
  test.identical( got, { index : 7, element : { a : 2 } } );

  test.case = 'searches complex data, onEvaluate, onEvaluate2';
  var src = _.containerAdapter.make( [ 1, 2, 'str', [ 3 ], { a : 2 } ] );
  var got = src.right( 3, ( e ) => e[ 0 ], ( ins ) => ins );
  test.identical( got, { index : 3, element : [ 3 ] } );

  /* */

  test.case = 'container has not searched element, fromIndex, onEvaluate1, onEvaluate2';
  var src = _.containerAdapter.make( [ 1, 2, 'str', [ 3 ], { a : 2 } ] );
  var got = src.right( 3, 2, ( e ) => e[ 0 ], ( ins ) => ins );
  test.identical( got, { index : -1 } );

  test.case = 'container has duplicated searched element, fromIndex, onEvaluate1, onEvaluate2';
  var src = _.containerAdapter.make( [ 1, 2, 3, 'str', [ 3 ], 3, 'str', { a : 2 } ] );
  var got = src.right( 2, 7, ( e ) => e.a, ( ins ) => ins );
  test.identical( got, { index : 7, element : { a : 2 } } );

  test.case = 'searches complex data, fromIndex, onEvaluate, onEvaluate2';
  var src = _.containerAdapter.make( [ 1, 2, 'str', [ 3 ], { a : 2 } ] );
  var got = src.right( 3, 4, ( e ) => e[ 0 ], ( ins ) => ins );
  test.identical( got, { index : 3, element : [ 3 ] } );

  /* */

  test.case = 'container has not searched element, equalizer';
  var src = _.containerAdapter.make( [ 1, 2, 'str', [ 3 ], { a : 2 } ] );
  var got = src.right( 3, ( e, ins ) => e[ 0 ] === ins );
  test.identical( got, { index : 3, element : [ 3 ] } );

  test.case = 'container has duplicated searched element, equalizer';
  var src = _.containerAdapter.make( [ 1, 2, 3, 'str', [ 3 ], 3, 'str', { a : 2 } ] );
  var got = src.right( 2, ( e, ins ) => e.a === ins );
  test.identical( got, { index : 7, element : { a : 2 } } );

  test.case = 'searches complex data, equalizer';
  var src = _.containerAdapter.make( [ 1, 2, 'str', [ 3 ], { a : 2 } ] );
  var got = src.right( 3, ( e, ins ) => e[ 0 ] ===  ins );
  test.identical( got, { index : 3, element : [ 3 ] } );

  /* - */

  if( !Config.debug )
  return;

  test.case = 'without arguments';
  test.shouldThrowErrorSync( () =>
  {
    var src = _.containerAdapter.make( [] );
    src.right();
  });

  test.case = 'extra arguments';
  test.shouldThrowErrorSync( () =>
  {
    var src = _.containerAdapter.make( [ 1, 2 ] );
    src.right( 1, 0, ( e ) => e, ( ins ) => ins, 'extra' );
  });

  test.case = 'onEvaluate1 is not a routine, not a number';
  test.shouldThrowErrorSync( () =>
  {
    var src = _.containerAdapter.make( [ 1, 2 ] );
    src.right( 1, 0, 'wrong' );
  });

  test.case = 'onEvaluate1 has wrong length';
  test.shouldThrowErrorSync( () =>
  {
    var src = _.containerAdapter.make( [ 1, 2 ] );
    src.right( 1, 0, () => 1 );
  });

  test.case = 'onEvaluate2 has wrong length';
  test.shouldThrowErrorSync( () =>
  {
    var src = _.containerAdapter.make( [ 1, 2 ] );
    src.right( 1, 0, ( e ) => e, () => 1 );
  });
}

//

function arrayAdapterReverse( test )
{
  test.case = 'without arguments';
  var src = _.containerAdapter.make( [ undefined, null, [ 0 ], 'str', { a : false } ] );
  var got = src.reverse();
  test.true( got !== src );
  test.identical( got.original, [ { a : false }, 'str', [ 0 ], null, undefined ] );

  test.case = 'dst - empty array';
  var src = _.containerAdapter.make( [ undefined, null, [ 0 ], 'str', { a : false } ] );
  var dst = [];
  var got = src.reverse( [] );
  test.true( got !== src );
  test.true( got !== dst );
  test.identical( got.original, [ { a : false }, 'str', [ 0 ], null, undefined ] );

  test.case = 'dst - not empty array';
  var src = _.containerAdapter.make( [ undefined, null, [ 0 ], 'str', { a : false } ] );
  var dst = [ true, '' ];
  var got = src.reverse( dst );
  test.true( got !== src );
  test.true( got !== dst );
  test.identical( got.original, [ true, '', { a : false }, 'str', [ 0 ], null, undefined ] );

  test.case = 'dst - empty Set';
  var src = _.containerAdapter.make( [ undefined, null, [ 0 ], 'str', { a : false } ] );
  var dst = new Set();
  var got = src.reverse( dst );
  test.true( got !== src );
  test.true( got !== dst );
  test.identical( [ ... got.original ], [ { a : false }, 'str', [ 0 ], null, undefined ] );

  test.case = 'dst - not empty Set';
  var src = _.containerAdapter.make( [ undefined, null, [ 0 ], 'str', { a : false } ] );
  var dst = new Set( [ true, '' ] );
  var got = src.reverse( dst );
  test.true( got !== src );
  test.true( got !== dst );
  test.identical( [ ... got.original ], [ true, '', { a : false }, 'str', [ 0 ], null, undefined ] );

  test.case = 'dst - empty arrayContainerAdapter';
  var src = _.containerAdapter.make( [ undefined, null, [ 0 ], 'str', { a : false } ] );
  var dst = _.containerAdapter.make( [] );
  var got = src.reverse( dst );
  test.true( got !== src );
  test.true( got === dst );
  test.identical( got.original, [ { a : false }, 'str', [ 0 ], null, undefined ] );

  test.case = 'dst - not empty arrayContainerAdapter';
  var src = _.containerAdapter.make( [ undefined, null, [ 0 ], 'str', { a : false } ] );
  var dst = _.containerAdapter.make( [ true, '' ] );
  var got = src.reverse( dst );
  test.true( got !== src );
  test.true( got === dst );
  test.identical( got.original, [ true, '', { a : false }, 'str', [ 0 ], null, undefined ] );

  test.case = 'dst - empty setContainerAdapter';
  var src = _.containerAdapter.make( [ undefined, null, [ 0 ], 'str', { a : false } ] );
  var dst = _.containerAdapter.make( new Set() );
  var got = src.reverse( dst );
  test.true( got !== src );
  test.true( got === dst );
  test.identical( [ ... got.original ], [ { a : false }, 'str', [ 0 ], null, undefined ] );

  test.case = 'dst - not empty setContainerAdapter';
  var src = _.containerAdapter.make( [ undefined, null, [ 0 ], 'str', { a : false } ] );
  var dst = _.containerAdapter.make( new Set( [ true, '' ] ) );
  var got = src.reverse( dst );
  test.true( got !== src );
  test.true( got === dst );
  test.identical( [ ... got.original ], [ true, '', { a : false }, 'str', [ 0 ], null, undefined ] );

  test.case = 'dst === src, odd number of elements';
  var src = _.containerAdapter.make( [ undefined, null, [ 0 ], 'str', { a : false } ] );
  var got = src.reverse( src );
  test.true( got === src );
  test.identical( [ ... got.original ], [ { a : false }, 'str', [ 0 ], null, undefined ] );

  test.case = 'dst === src, even number of elements';
  var src = _.containerAdapter.make( [ undefined, null, [ 0 ], 'str', { a : false }, true ] );
  var got = src.reverse( src );
  test.true( got === src );
  test.identical( [ ... got.original ], [ true, { a : false }, 'str', [ 0 ], null, undefined ] );
}

//

function arrayAdapterJoin( test )
{
  test.case = 'empty container, without delimeter';
  var src = _.containerAdapter.make( [] );
  var got = src.join();
  test.identical( got, '' );

  test.case = 'empty container, delimeter';
  var src = _.containerAdapter.make( [] );
  var got = src.join( '/' );
  test.identical( got, '' );

  test.case = 'not empty container, delimeter';
  var src = _.containerAdapter.make( [ 1, 2, [ 3, 4 ] ] );
  var got = src.join( '/' );
  test.identical( got, '1/2/3,4' );
}

//

function arrayAdapterToArray( test )
{
  test.case = 'empty container';
  var src = _.containerAdapter.make( [] );
  var got = src.toArray();
  test.true( got === src );
  test.true( got.original === src.original );
  test.identical( got.original, src.original );

  test.case = 'not empty container';
  var src = _.containerAdapter.make( [ 1, 2, 3 ] );
  var got = src.toArray();
  test.true( got === src );
  test.true( got.original === src.original );
  test.identical( got.original, src.original );
}
//

function arrayAdapterToSet( test )
{
  test.case = 'empty container';
  var src = _.containerAdapter.make( [] );
  var got = src.toSet();
  test.true( got !== src );
  test.true( got.original !== src.original );
  test.identical( [ ... got.original ], src.original );

  test.case = 'not empty container';
  var src = _.containerAdapter.make( [ 1, 2, 3 ] );
  var got = src.toSet();
  test.true( got !== src );
  test.true( got.original !== src.original );
  test.identical( [ ... got.original ], src.original );
}

//

function arrayAdapterSymbolIterator( test )
{
  test.case = 'empty container';
  var src = _.containerAdapter.make( [] );
  var exp = [];
  for( let e of src )
  exp.push( e + 1 );
  test.identical( src.original, [] );
  test.identical( src.original, [] );

  test.case = 'not empty container';
  var src = _.containerAdapter.make( [ 1, 2, 3 ] );
  var exp = [];
  for( let e of src )
  exp.push( e + 1 );
  test.identical( src.original, [ 1, 2, 3 ] );
  test.identical( exp, [ 2, 3, 4 ] );
}

//

function arrayAdapterLength( test )
{
  test.case = 'empty container';
  var src = _.containerAdapter.make( [] );
  var got = src.length;
  test.identical( got, 0 )

  test.case = 'not empty container';
  var src = _.containerAdapter.make( [ 1, 2, 3, undefined ] );
  var got = src.length;
  test.identical( got, 4 )
}

// --
// ContainerAdapter related routines
// --

function longMakeEmpty( test )
{
  test.case = 'src - containerAdapter, empty array';
  var got = _.long.makeEmpty( _.containerAdapter.make( [] ) );
  test.true( _.long.default.is( got ) );
  test.identical( got, _.long.default.make( [] ) );

  test.case = 'src - containerAdapter, filled array';
  var got = _.long.makeEmpty( _.containerAdapter.make([ 1, 2, 3 ]) );
  test.true( _.long.default.is( got ) );
  test.identical( got, _.long.default.make( [] ) );

  test.case = 'src - containerAdapter, empty array';
  var got = _.long.makeEmpty( _.containerAdapter.make( new Set( [] ) ) );
  test.true( _.long.default.is( got ) );
  test.identical( got, _.long.default.make( [] ) );

  test.case = 'src - containerAdapter, filled array';
  var got = _.long.makeEmpty( _.containerAdapter.make( new Set([ 1, 2, 3 ]) ) );
  test.true( _.long.default.is( got ) );
  test.identical( got, _.long.default.make( [] ) );
}

//

function vectorizeVectorizeArray( test )
{
  var srcRoutine = function()
  {
    if( arguments.length === 0 )
    return null;
    return _.longSlice( arguments );
  };

  var o =
  {
    vectorizingArray : 1,
    vectorizingMapVals : 0,
    select : 1,
  };
  o.routine = srcRoutine;
  var routine = _.vectorize( o );

  /* - */

  var src = _.containerAdapter.make( [ 1, 2 ] );
  var got = routine( src, [ 3, 4 ] )
  test.identical( [ ... got[ 0 ].original ], [ 1, 2 ] );
  test.identical( got[ 1 ], [ 3, 4 ] );
  test.true( got !== src );

  var src = _.containerAdapter.make( new Set( [ 1, 2, 3 ] ) );
  var got = routine( src, 2 );
  test.identical( [ ... got[ 0 ].original ], [ 1, 2, 3 ] );
  test.identical( got[ 1 ], 2 );
  test.true( got !== src );

  /* - */

  test.open( 'containerAdapter, no unwraping' );

  var o =
  {
    vectorizingArray : 1,
    vectorizingMapVals : 0,
    select : 1,
    vectorizingContainerAdapter : 1,
  }
  o.routine = srcRoutine;
  var routine = _.vectorize( o );

  test.case = 'single argument';

  var src = _.containerAdapter.make([ 1 ]);
  var got = routine( src );
  test.identical( got.original, [ [ 1 ] ] );
  test.true( got !== src );

  var src = _.containerAdapter.make([ 1, 2, 3 ]);
  var got = routine( src );
  test.identical( got.original, [ [ 1 ], [ 2 ], [ 3 ] ] );
  test.true( got !== src );

  test.case = 'multiple argument';

  var src = _.containerAdapter.make([ 1, 2, 3 ]);
  var got = routine( src, 2 );
  test.identical( got.original, [ [ 1, 2 ], [ 2, 2 ], [ 3, 2 ] ] );
  test.true( got !== src );

  var src = _.containerAdapter.make([ 2 ]);
  var got = routine( src, [ 1, 2, 3 ] );
  test.identical( got.original, [ [ 2, [ 1, 2, 3 ] ] ] );
  test.true( got !== src );

  var src = _.containerAdapter.make([ 1, 2 ]);
  var got = routine( src, [ 3, 4 ] )
  test.identical( got.original, [ [ 1, [ 3, 4 ] ], [ 2, [ 3, 4 ] ] ] );
  test.true( got !== src );

  /* */

  test.case = 'single argument';

  var src = _.containerAdapter.make( new Set([ 1 ]) );
  var got = routine( src );
  test.identical( [ ... got.original ], [ [ 1 ] ] );
  test.true( got !== src );

  var src = _.containerAdapter.make( new Set([ 1, 2, 3 ]) );
  var got = routine( src );
  test.identical( [ ... got.original ], [ [ 1 ], [ 2 ], [ 3 ] ] );
  test.true( got !== src );

  test.case = 'multiple argument';

  var src = _.containerAdapter.make( new Set([ 1, 2, 3 ]) );
  var got = routine( src, 2 );
  test.identical( [ ... got.original ], [ [ 1, 2 ], [ 2, 2 ], [ 3, 2 ] ] );
  test.true( got !== src );

  var src = _.containerAdapter.make( new Set([ 2 ]) );
  var got = routine( src, [ 1, 2, 3 ] );
  test.identical( [ ... got.original ], [ [ 2, [ 1, 2, 3 ] ] ] );
  test.true( got !== src );

  var src = _.containerAdapter.make( new Set([ 1, 2 ]) );
  var got = routine( src, [ 3, 4 ] )
  test.identical( [ ... got.original ], [ [ 1, [ 3, 4 ] ], [ 2, [ 3, 4 ] ] ] );
  test.true( got !== src );

  test.close( 'containerAdapter, no unwraping' );

  /* - */

  test.open( 'containerAdapter, unwraping' );

  var o =
  {
    vectorizingArray : 1,
    vectorizingMapVals : 0,
    select : 1,
    vectorizingContainerAdapter : 1,
    unwrapingContainerAdapter : 1,
  }
  o.routine = srcRoutine;
  var routine = _.vectorize( o );

  test.case = 'single argument';

  var src = _.containerAdapter.make([ 1 ]);
  var got = routine( src );
  test.identical( got, [ [ 1 ] ] );
  test.true( got !== src );

  var src = _.containerAdapter.make([ 1, 2, 3 ]);
  var got = routine( _.containerAdapter.make( [ 1, 2, 3 ] ) );
  test.identical( got, [ [ 1 ], [ 2 ], [ 3 ] ] );
  test.true( got !== src );

  test.case = 'multiple argument';

  var src = _.containerAdapter.make([ 1, 2, 3 ]);
  var got = routine( _.containerAdapter.make( [ 1, 2, 3 ] ), 2 );
  test.identical( got, [ [ 1, 2 ], [ 2, 2 ], [ 3, 2 ] ] );
  test.true( got !== src );

  var src = _.containerAdapter.make([ 2 ]);
  var got = routine( _.containerAdapter.make( [ 2 ] ), [ 1, 2, 3 ] );
  test.identical( got, [ [ 2, [ 1, 2, 3 ] ] ] );
  test.true( got !== src );

  var src = _.containerAdapter.make([ 1, 2 ]);
  var got = routine( _.containerAdapter.make( [ 1, 2 ] ), [ 3, 4 ] )
  test.identical( got, [ [ 1, [ 3, 4 ] ], [ 2, [ 3, 4 ] ] ] );
  test.true( got !== src );

  /* */

  test.case = 'single argument';

  var src = _.containerAdapter.make( new Set([ 1 ]) );
  var got = routine( src );
  test.identical( [ ... got ], [ [ 1 ] ] );
  test.true( got !== src );

  var src = _.containerAdapter.make( new Set([ 1, 2, 3 ]) );
  var got = routine( _.containerAdapter.make( new Set( [ 1, 2, 3 ] ) ) );
  test.identical( [ ... got ], [ [ 1 ], [ 2 ], [ 3 ] ] );
  test.true( got !== src );

  test.case = 'multiple argument';

  var src = _.containerAdapter.make( new Set([ 1, 2, 3 ]) );
  var got = routine( _.containerAdapter.make( new Set( [ 1, 2, 3 ] ) ), 2 );
  test.identical( [ ... got ], [ [ 1, 2 ], [ 2, 2 ], [ 3, 2 ] ] );
  test.true( got !== src );

  var src = _.containerAdapter.make( new Set([ 2 ]) );
  var got = routine( _.containerAdapter.make( new Set( [ 2 ] ) ), [ 1, 2, 3 ] );
  test.identical( [ ... got ], [ [ 2, [ 1, 2, 3 ] ] ] );
  test.true( got !== src );

  var src = _.containerAdapter.make( new Set([ 1, 2 ]) );
  var got = routine( _.containerAdapter.make( new Set( [ 1, 2 ] ) ), [ 3, 4 ] )
  test.identical( [ ... got ], [ [ 1, [ 3, 4 ] ], [ 2, [ 3, 4 ] ] ] );
  test.true( got !== src );

  test.close( 'containerAdapter, unwraping' );
}

// --
// declaration
// --

const Proto =
{

  name : 'Tools.ContainerAdapter.l1',
  silencing : 1,
  routineTimeOut : 10000,

  tests :
  {

    // containerAdapter

    is,
    make,
    makeByConstructor,
    from,
    toOriginal,
    toOriginals,

    // ContainerAdapterAbstract

    ToOriginal,
    is_,
    isContainer,
    from_,
    same,

    removedContainer,
    removedContainerOnce,
    removedContainerOnceStrictly,
    removeContainer,
    removeContainerOnce,
    removeContainerOnceStrictly,

    min,
    max,
    least,
    most,

    onlyWithoutCallbacks,
    onlyOneEvaluator,
    onlyTwoEvaluators,
    onlyEqualizer,
    onlyBugWithReturnedContainer,

    butWithoutCallbacks,
    butOneEvaluator,
    butTwoEvaluators,
    butEqualizer,
    butBugWithReturnedContainer,

    select,

    // ContainerAdapterSet

    setAdapterMake_,
    setAdapterMakeEmpty,
    setAdapterMake,
    setAdapterHas,
    setAdapterHasBugWithNullInEvaluator,
    setAdapterCount,
    setAdapterCopyFrom,
    setAdapterAppend,

    setAdapterAppendOnceWithoutCallbacks,
    setAdapterAppendOnceOneEvaluator,
    setAdapterAppendOnceTwoEvaluators,
    setAdapterAppendOnceEqualizer,

    setAdapterAppendOnceStrictly,
    setAdapterPush,
    setAdapterAppendContainer,
    setAdapterAppendContainerOnce,
    setAdapterAppendContainerOnceStrictly,
    setAdapterPop,
    setAdapterPopStrictly,
    setAdapterRemoved,
    setAdapterRemovedOnce,
    setAdapterRemovedOnceRight,
    setAdapterRemovedOnceStrictly,
    setAdapterRemovedOnceStrictlyRight,
    setAdapterRemove,
    setAdapterRemoveOnce,
    setAdapterRemoveOnceRight,
    setAdapterRemoveOnceStrictly,
    setAdapterRemoveOnceStrictlyRight,
    setAdapterEmpty,
    setAdapterMap,
    setAdapterMapRight,
    setAdapterFilter,
    setAdapterFilterRight,
    setAdapterFlatFilter,
    setAdapterFlatFilterRight,
    setAdapterOnce,
    setAdapterOnceRight,
    setAdapterFirst,
    setAdapterLast,
    setAdapterLastTimeExperiment,
    setAdapterEach,
    setAdapterEachRight,
    setAdapterReduce,
    setAdapterReduceRight,
    setAdapterAll,
    setAdapterAllRight,
    setAdapterAny,
    setAdapterAnyRight,
    setAdapterNone,
    setAdapterNoneRight,
    setAdapterLeft,
    setAdapterRight,
    setAdapterReverse,
    setAdapterJoin,
    setAdapterToArray,
    setAdapterToSet,
    setAdapterSymbolIterator,
    setAdapterLength,

    // ContainerAdapterArray

    arrayAdapterMake_,
    arrayAdapterMakeEmpty,
    arrayAdapterMake,
    arrayAdapterHas,
    arrayAdapterCount,
    arrayAdapterCopyFrom,
    arrayAdapterAppend,
    arrayAdapterAppendOnce,
    arrayAdapterAppendOnceStrictly,
    arrayAdapterPush,
    arrayAdapterAppendContainer,
    arrayAdapterAppendContainerOnce,
    arrayAdapterAppendContainerOnceStrictly,
    arrayAdapterPop,
    arrayAdapterPopStrictly,
    arrayAdapterRemoved,
    arrayAdapterRemovedOnce,
    arrayAdapterRemovedOnceRight,
    arrayAdapterRemovedOnceStrictly,
    arrayAdapterRemovedOnceStrictlyRight,
    arrayAdapterRemove,
    arrayAdapterRemoveOnce,
    arrayAdapterRemoveOnceRight,
    arrayAdapterRemoveOnceStrictly,
    arrayAdapterRemoveOnceStrictlyRight,
    arrayAdapterEmpty,
    arrayAdapterMap,
    arrayAdapterMapRight,
    arrayAdapterFilter,
    arrayAdapterFilterRight,
    arrayAdapterFlatFilter,
    arrayAdapterFlatFilterRight,
    arrayAdapterOnce,
    arrayAdapterOnceRight,
    arrayAdapterFirst,
    arrayAdapterLast,
    arrayAdapterEach,
    arrayAdapterEachRight,
    arrayAdapterReduce,
    arrayAdapterReduceRight,
    arrayAdapterAll,
    arrayAdapterAllRight,
    arrayAdapterAny,
    arrayAdapterAnyRight,
    arrayAdapterNone,
    arrayAdapterNoneRight,
    arrayAdapterLeft,
    arrayAdapterRight,
    arrayAdapterReverse,
    arrayAdapterJoin,
    arrayAdapterToArray,
    arrayAdapterToSet,
    arrayAdapterSymbolIterator,
    arrayAdapterLength,

    // ContainerAdapter related routines

    longMakeEmpty,
    vectorizeVectorizeArray,

  }

}

const Self = wTestSuite( Proto );
if( typeof module !== 'undefined' && !module.parent )
wTester.test( Self.name );

})();
