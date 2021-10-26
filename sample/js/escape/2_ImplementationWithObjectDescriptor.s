/*
  A map can be provided as a first argument whether to assign value or not
*/

console.log( '2_ImplementationWithObjectDesriptor.s' );

function objectSetFieldWithObjectDescriptor( objectDescriptor, field, value )
{
  if( objectDescriptor.skip === true )
  return;
  else
  objectDescriptor.src[ field ] = value;
}

module.exports = objectSetFieldWithObjectDescriptor;
