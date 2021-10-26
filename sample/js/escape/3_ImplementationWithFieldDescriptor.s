/*
  A map can be provided as a second argument whether to assign value or not
*/

console.log( '3_ImplementationWithFieldDescriptor.s' );

function objectSetFieldWithFieldDescriptor( obj, fieldDescriptor, value )
{
  if( fieldDescriptor.skip === true )
  return;
  else
  obj[ fieldDescriptor.name ] = value;
}

module.exports = objectSetFieldWithFieldDescriptor;
