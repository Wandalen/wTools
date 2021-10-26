if( typeof module !== undefined )
require( 'wTools' );

let _ = wTools;

var cb1 = ( e ) => e;
var cb2 = ( e ) => e ** 2;
var cb3 = ( e ) => Math.sqrt( e );

/* in vectors */

var elements = [ [ cb1, 'first' ], [ cb2, 'second' ], [ cb3, 'third' ] ];
var routines = _.vectorizeAccess( elements );

/* get property */

console.log( routines[ '$' ] );
// log [
//       [ [Function: cb1], 'first' ],
//       [ [Function: cb2], 'second' ],
//       [ [Function: cb3], 'third' ]
//     ]
console.log( routines[ 0 ] ); // log [Function]
console.log( routines[ 1 ].$ ); // log [ 'first', 'second', 'third' ]

/* set property */

routines[ 1 ] = 'new element';

console.log( routines[ '$' ] );
// log
//     [
//       [ [Function: cb1], 'new element' ],
//       [ [Function: cb2], 'new element' ],
//       [ [Function: cb3], 'new element' ]
//     ]
console.log( routines[ 0 ] ); // log [Function]
console.log( routines[ 1 ].$ ); // log [ 'new element', 'new element', 'new element' ]

/* use callback */

var resultOperations = routines[ 0 ]( 4 );
console.log( resultOperations[ '$' ] ); // log [ 4, 16, 2 ]
