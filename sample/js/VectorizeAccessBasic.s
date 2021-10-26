if( typeof module !== undefined )
require( 'wTools' );

let _ = wTools;

var cb1 = ( e ) => e;
var cb2 = ( e ) => e ** 2;
var cb3 = ( e ) => Math.sqrt( e );

/* in objects */

var elements = [ { routine : cb1, name : 'first' }, { routine : cb2, name : 'second' }, { routine : cb3, name : 'third' } ];
var routines = _.vectorizeAccess( elements );

/* get property */

console.log( routines[ '$' ] );
// log [
//       { routine: [Function: cb1], name: 'first' },
//       { routine: [Function: cb2], name: 'second' },
//       { routine: [Function: cb3], name: 'third' }
//     ]
console.log( routines.routine ); // log [Function]
console.log( routines.name.$ ); // log [ 'first', 'second', 'third' ]

/* set property */

routines.name = 'new element';

console.log( routines[ '$' ] );
// log [
//       { routine: [Function: cb1], name: 'new element' },
//       { routine: [Function: cb2], name: 'new element' },
//       { routine: [Function: cb3], name: 'new element' }
//     ]
console.log( routines.routine ); // log [Function]
console.log( routines.name.$ ); // log [ 'new element', 'new element', 'new element' ]

/* use callback */

var resultOperations = routines.routine( 4 );
console.log( resultOperations[ '$' ] ); // log [ 4, 16, 2 ]
