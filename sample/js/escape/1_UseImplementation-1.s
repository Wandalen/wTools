let objectSetFieldWith4Params = require( './1_ImplementationWith4Arguments.s' );

let obj = {};

objectSetFieldWith4Params( obj, 'field1', 1 );
objectSetFieldWith4Params( obj, 'field2', 2 );
objectSetFieldWith4Params( obj, 'field3', null );
objectSetFieldWith4Params( obj, 'field4', null, true );

console.log( obj );
/* log : { field1: 1, field2: 2, field3: null } */
