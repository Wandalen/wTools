let objectSetField = require( './0_Implementation.s' );

let obj = {};

objectSetField( obj, 'field1', 1 );
objectSetField( obj, 'field2', 2 );
objectSetField( obj, 'field3', null );

console.log( obj );
/* log : { field1: 1, field2: 2 } */
