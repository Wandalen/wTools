( function _l3_Set_s_()
{

'use strict';

let _global = _global_;
let _ = _global_.wTools;
let Self = _.set = _.set || Object.create( null );
_.set.s = _.set.s || Object.create( null );

// --
// implementation
// --

function setIs( src )
{
  if( !src )
  return false;
  return src instanceof Set || src instanceof WeakSet;
}

//

function setLike( src )
{
  return _.set.is( src );
}

//

function setAdapterLike( src )
{
  if( !src )
  return false;
  if( _.set.like( src ) )
  return true;
  if( src instanceof _.containerAdapter.Set )
  return true;
  return false;
}

//

function setIsEmpty()
{
  return !src.size;
}

//

function setIsPopulated()
{
  return !!src.size;
}

// --
// set
// --

function setFrom( src )
{
  _.assert( arguments.length === 1 );
  if( _.set.adapterLike( src ) )
  return src;
  if( src === null )
  return new Set();
  if( _.containerAdapter.is( src ) )
  src = src.toArray().original;
  _.assert( _.longIs( src ) );
  return new Set([ ... src ]);
}

//

function setsFrom( srcs )
{
  _.assert( arguments.length === 1 );
  _.assert( _.longIs( srcs ) );
  let result = [];
  for( let s = 0, l = srcs.length ; s < l ; s++ )
  result[ s ] = _.set.from( srcs[ s ] );
  return result;
}

//

function setToArray( src )
{
  _.assert( arguments.length === 1 );
  _.assert( _.set.like( src ) );
  return [ ... src ];
}

//

function setsToArrays( srcs )
{
  _.assert( arguments.length === 1 );
  _.assert( _.longIs( srcs ) );
  let result = [];
  for( let s = 0, l = srcs.length ; s < l ; s++ )
  result[ s ] = _.set.toArray( srcs[ s ] );
  return result;
}

// --
// extension
// --

let ToolsExtension =
{

  // checker

  setIs,
  setLike,
  setAdapterLike,
  setIsEmpty,
  setIsPopulated,

  // set

  setFrom,
  setsFrom,
  setToArray,
  setsToArrays,

}

//

let Extension =
{

  // checker

  is : setIs,
  like : setLike,
  adapterLike : setAdapterLike,
  isEmpty : setIsEmpty,
  isPopulated : setIsPopulated,

  // set

  from : setFrom,
  toArray : setToArray,

}

//

let ExtensionS =
{

  // set

  from : setsFrom,
  toArrays : setsToArrays,
}

//

Object.assign( _, ToolsExtension );
Object.assign( Self, Extension );
Object.assign( _.set.s, ExtensionS );

// --
// export
// --

if( typeof module !== 'undefined' )
module[ 'exports' ] = _;

})();
