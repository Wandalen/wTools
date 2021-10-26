let objectSetFieldWithObjectDescriptor = require( './2_ImplementationWithObjectDescriptor.s' );

let obj = {};

objectSetFieldWithObjectDescriptor( { src : obj, skip : false }, 'field1', 1 );
objectSetFieldWithObjectDescriptor( { src : obj, skip : false }, 'field2', 2 );
objectSetFieldWithObjectDescriptor( { src : obj, skip : false }, 'field3', null );
objectSetFieldWithObjectDescriptor( { src : obj, skip : true }, 'field4', null );

console.log( obj );
/* log : { field1: 1, field2: 2, field3: null } */
