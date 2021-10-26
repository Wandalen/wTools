let _ = require( '../..' );

/*
null serves as an identifier, that we should not change object property's value, but make it constant.
*/

console.log( '6_ImplementationAddConstantFieldWithEscape.s' );

function objectSetFieldConstantWithEscape( object, field, value )
{
  if( _.escape.is( value ) )
  {
    Object.defineProperty
    (
      object,
      field,
      {
        enumerable : true,
        configurable : false,
        writable : false,
        value : _.escape.right( value )
      }
    );
  }
  else if( value === null )
  {
    Object.defineProperty
    (
      object,
      field,
      {
        enumerable : true,
        configurable : false,
        writable : false,
        value : object[ field ]
      }
    );
  }
  else
  {
    Object.defineProperty
    (
      object,
      field,
      {
        enumerable : true,
        configurable : false,
        writable : false,
        value
      }
    );
  }
  return object;
}

module.exports = objectSetFieldConstantWithEscape;
