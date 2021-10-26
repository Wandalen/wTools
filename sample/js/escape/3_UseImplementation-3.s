let objectSetFieldWithFieldDescriptor = require( './3_ImplementationWithFieldDescriptor.s' );

let obj = {};

objectSetFieldWithFieldDescriptor( obj, { name : 'field1', skip : false }, 1 );
objectSetFieldWithFieldDescriptor( obj, { name : 'field2', skip : false }, 2 );
objectSetFieldWithFieldDescriptor( obj, { name : 'field3', skip : false }, null );
objectSetFieldWithFieldDescriptor( obj, { name : 'field4', skip : true }, null );

console.log( obj );
/* log : { field1: 1, field2: 2, field3: null } */
