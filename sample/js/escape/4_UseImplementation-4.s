let objectSetFieldWithValueDescriptor = require( './4_ImplementationWithValueDescriptor.s' );

let obj = {};

objectSetFieldWithValueDescriptor( obj, 'field1', { value : 1, skip : false } );
objectSetFieldWithValueDescriptor( obj, 'field2', { value : 2, skip : false } );
objectSetFieldWithValueDescriptor( obj, 'field3', { value : null, skip : false } );
objectSetFieldWithValueDescriptor( obj, 'field4', { value : null, skip : true } );

console.log( obj );
/* log : { field1: 1, field2: 2, field3: null } */
