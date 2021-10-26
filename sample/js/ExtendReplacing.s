'use strict'

if( typeof module !== 'undefined' )
require( 'wTools' );
let _ = wTools;


/* array, imperative form */

var dst = [ 1, 2, 3 ];
var src = [ 4, 5, 6, 7 ];

Object.preventExtensions( dst );

try
{
  for( let i = 0; i < src.length; i++ )
  dst[ i ] = src[ i ];
}
catch( e )
{
  console.log( e ); // log error
}
console.log( dst ); // log [ 4, 5, 6 ]

/* array, object assigning */

var dst = [ 1, 2, 3 ];
var src = [ 4, 5, 6, 7 ];

Object.preventExtensions( dst );

try
{
  Object.assign( dst, src );
}
catch( e )
{
  console.log( e ); // log error
}

console.log( dst ); // log [ 4, 5, 6 ]

/* array, extendReplacing with check of length and check of extension possibility */

var dst = [ 1, 2, 3 ];
var src = [ 4, 5, 6, 7 ];

Object.preventExtensions( dst );

try
{
  _.container.extendReplacing( dst, src )
}
catch( e )
{
  console.log( e ); // log error
}

console.log( dst ); // log [ 1, 2, 3 ]
