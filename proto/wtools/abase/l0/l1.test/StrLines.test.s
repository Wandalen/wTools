( function _l0_l1_StrLines_test_s_()
{

'use strict';

if( typeof module !== 'undefined' )
{
  require( '../Include1.s' );
  require( 'wTesting' );
}

const _global = _global_;
const _ = _global_.wTools;
const __ = _globals_.testing.wTools;

// --
//
// --

function assumptions( test )
{
  let its;

  /* */

  test.case = 'assumption';
  test.identical( '\n\r'.length, 2 );
  test.identical( '\n'.charCodeAt( 0 ), 10 );
  test.identical( '\r'.charCodeAt( 0 ), 13 );

}

//

function eachLeftSingle( test )
{
  let its;

  /* */

  test.case = 'empty';
  clean();
  var src = ``;
  _.str.lines.eachLeft({ src, onEach, eol : _.str.lines.Eol.any });
  var exp =
  [
    {
      'src' : '',
      onEach,
      'withLine' : true,
      'eol' : [],
      'lineIndex' : 0,
      'nl' : '',
      'charInterval' : [ 0, -1 ],
      'line' : ''
    }
  ]
  test.identical( its, exp );
  test.identical( src.length, 0 );

  /* */

  test.case = 'single line';
  clean();
  var src = `abc`;
  _.str.lines.eachLeft({ src, onEach, eol : _.str.lines.Eol.any });
  var exp =
  [
    {
      'src' : 'abc',
      onEach,
      'withLine' : true,
      'eol' : [],
      'lineIndex' : 0,
      'charInterval' : [ 0, 2 ],
      'nl' : '',
      'line' : 'abc',
    }
  ]
  test.identical( its, exp );
  test.identical( src.length, 3 );

  /* */

  test.case = '\\n';
  clean();
  var src = `\n`;
  _.str.lines.eachLeft({ src, onEach, eol : _.str.lines.Eol.any });
  var exp =
  [
    {
      src,
      onEach,
      'eol' : '\n',
      'charInterval' : [ 0, 0 ],
      'lineIndex' : 0,
      'withLine' : true,
      'nl' : '\n',
      'line' : '',
    },
    {
      src,
      onEach,
      'eol' : '\n',
      'charInterval' : [ 1, 0 ],
      'lineIndex' : 1,
      'withLine' : true,
      'nl' : '',
      'line' : '',
    }
  ]
  test.identical( its, exp );
  test.identical( src.length, 1 );

  /* */

  test.case = 'ab\\nabc\\nabcd';
  clean();
  var src = `ab\nabc\nabcd`;
  _.str.lines.eachLeft({ src, onEach, eol : _.str.lines.Eol.any });
  var exp =
  [
    {
      src,
      onEach,
      'withLine' : true,
      'eol' : '\n',
      'charInterval' : [ 0, 2 ],
      'lineIndex' : 0,
      'nl' : '\n',
      'line' : 'ab',
    },
    {
      src,
      onEach,
      'withLine' : true,
      'eol' : '\n',
      'charInterval' : [ 3, 6 ],
      'lineIndex' : 1,
      'nl' : '\n',
      'line' : 'abc',
    },
    {
      src,
      onEach,
      'withLine' : true,
      'eol' : '\n',
      'charInterval' : [ 7, 10 ],
      'lineIndex' : 2,
      'nl' : '',
      'line' : 'abcd',
    }
  ]
  test.identical( its, exp );
  test.identical( src.length, 11 );

  /* */

  test.case = '\\nab\\nabc\\nabcd\\n';
  clean();
  var src = `\nab\nabc\nabcd\n`;
  _.str.lines.eachLeft({ src, onEach, eol : _.str.lines.Eol.any });
  var exp =
  [
    {
      src,
      onEach,
      'withLine' : true,
      'eol' : '\n',
      'charInterval' : [ 0, 0 ],
      'lineIndex' : 0,
      'nl' : '\n',
      'line' : '',
    },
    {
      src,
      onEach,
      'withLine' : true,
      'eol' : '\n',
      'charInterval' : [ 1, 3 ],
      'lineIndex' : 1,
      'nl' : '\n',
      'line' : 'ab',
    },
    {
      src,
      onEach,
      'withLine' : true,
      'eol' : '\n',
      'charInterval' : [ 4, 7 ],
      'lineIndex' : 2,
      'nl' : '\n',
      'line' : 'abc',
    },
    {
      src,
      onEach,
      'withLine' : true,
      'eol' : '\n',
      'charInterval' : [ 8, 12 ],
      'lineIndex' : 3,
      'nl' : '\n',
      'line' : 'abcd',
    },
    {
      src,
      onEach,
      'withLine' : true,
      'eol' : '\n',
      'charInterval' : [ 13, 12 ],
      'lineIndex' : 4,
      'nl' : '',
      'line' : '',
    }
  ]
  test.identical( its, exp );
  test.identical( src.length, 13 );

  /* */

  function onEach( it )
  {
    it = _.props.extend( null, it );
    it.charInterval = it.charInterval.slice();
    it.eol = it.eol.slice();
    its.push( it );
  }

  /* */

  function clean()
  {
    its = [];
  }

  /* */

}

//

function eachLeftCustomToken( test )
{
  let its;

  /* */

  test.case = '\\n';
  clean();
  var eol = '\n';
  var src = `\n`;
  _.str.lines.eachLeft({ src, onEach, eol });
  var exp =
  [
    {
      src,
      onEach,
      'withLine' : true,
      'eol' : '\n',
      'charInterval' : [ 0, 0 ],
      'lineIndex' : 0,
      'nl' : '\n',
      'line' : '',
    },
    {
      src,
      onEach,
      'withLine' : true,
      'eol' : '\n',
      'charInterval' : [ 1, 0 ],
      'lineIndex' : 1,
      'nl' : '',
      'line' : '',
    }
  ]
  test.identical( its, exp );
  test.identical( src.length, 1 );

  /* */

  test.case = 'ab\\nabc\\nabcd';
  clean();
  var eol = '\r\n';
  var src = `ab\r\nabc\r\nabcd`;
  _.str.lines.eachLeft({ src, onEach, eol });
  var exp =
  [
    {
      src,
      onEach,
      'withLine' : true,
      'eol' : '\r\n',
      'charInterval' : [ 0, 3 ],
      'lineIndex' : 0,
      'nl' : '\r\n',
      'line' : 'ab',
    },
    {
      src,
      onEach,
      'withLine' : true,
      'eol' : '\r\n',
      'charInterval' : [ 4, 8 ],
      'lineIndex' : 1,
      'nl' : '\r\n',
      'line' : 'abc',
    },
    {
      src,
      onEach,
      'withLine' : true,
      'eol' : '\r\n',
      'charInterval' : [ 9, 12 ],
      'lineIndex' : 2,
      'nl' : '',
      'line' : 'abcd',
    }
  ]
  test.identical( its, exp );
  test.identical( src.length, 13 );

  /* */

  test.case = '\\r\\nab\\r\\nabc\\r\\nabcd\\r\\n';
  clean();
  var src = `\r\nab\r\nabc\r\nabcd\r\n`;
  _.str.lines.eachLeft({ src, onEach, eol });
  var exp =
  [
    {
      src,
      onEach,
      'withLine' : true,
      'eol' : '\r\n',
      'charInterval' : [ 0, 1 ],
      'lineIndex' : 0,
      'nl' : '\r\n',
      'line' : '',
    },
    {
      src,
      onEach,
      'withLine' : true,
      'eol' : '\r\n',
      'charInterval' : [ 2, 5 ],
      'lineIndex' : 1,
      'nl' : '\r\n',
      'line' : 'ab',
    },
    {
      src,
      onEach,
      'withLine' : true,
      'eol' : '\r\n',
      'charInterval' : [ 6, 10 ],
      'lineIndex' : 2,
      'nl' : '\r\n',
      'line' : 'abc',
    },
    {
      src,
      onEach,
      'withLine' : true,
      'eol' : '\r\n',
      'charInterval' : [ 11, 16 ],
      'lineIndex' : 3,
      'nl' : '\r\n',
      'line' : 'abcd',
    },
    {
      src,
      onEach,
      'withLine' : true,
      'eol' : '\r\n',
      'charInterval' : [ 17, 16 ],
      'lineIndex' : 4,
      'nl' : '',
      'line' : '',
    }
  ]
  test.identical( its, exp );
  test.identical( src.length, 17 );

  /* */

  function onEach( it )
  {
    it = _.props.extend( null, it );
    it.charInterval = it.charInterval.slice();
    it.eol = it.eol.slice();
    its.push( it );
  }

  /* */

  function clean()
  {
    its = [];
  }

  /* */

}

//

function eachLeftMultiple( test )
{
  let its;

  /* */

  test.case = '\\nab\\nabc\\nabcd\\r\\n';
  clean();
  var src = `\nab\nabc\nabcd\r\n`;
  _.str.lines.eachLeft({ src, onEach, eol : _.str.lines.Eol.any });
  var exp =
  [
    {
      src,
      onEach,
      'withLine' : true,
      'eol' : [ '\r\n', '\n', ],
      'charInterval' : [ 0, 0 ],
      'lineIndex' : 0,
      'nl' : '\n',
      'line' : '',
    },
    {
      src,
      onEach,
      'withLine' : true,
      'eol' : [ '\r\n', '\n', ],
      'charInterval' : [ 1, 3 ],
      'lineIndex' : 1,
      'nl' : '\n',
      'line' : 'ab',
    },
    {
      src,
      onEach,
      'withLine' : true,
      'eol' : [ '\r\n', '\n', ],
      'charInterval' : [ 4, 7 ],
      'lineIndex' : 2,
      'nl' : '\n',
      'line' : 'abc',
    },
    {
      src,
      onEach,
      'withLine' : true,
      'eol' : [ '\r\n', '\n', ],
      'charInterval' : [ 8, 13 ],
      'lineIndex' : 3,
      'nl' : '\r\n',
      'line' : 'abcd',
    },
    {
      src,
      onEach,
      'withLine' : true,
      'eol' : [],
      'charInterval' : [ 14, 13 ],
      'lineIndex' : 4,
      'nl' : '',
      'line' : '',
    }
  ]
  test.identical( its, exp );
  test.identical( src.length, 14 );

  /* */

  test.case = '\\r\\nab\\nabc\\nabcd\\n';
  clean();
  var src = `\r\nab\nabc\nabcd\n`;
  _.str.lines.eachLeft({ src, onEach, eol : _.str.lines.Eol.any });
  var exp =
  [
    {
      src,
      onEach,
      'withLine' : true,
      'eol' : [ '\r\n', '\n', ],
      'charInterval' : [ 0, 1 ],
      'lineIndex' : 0,
      'nl' : '\r\n',
      'line' : '',
    },
    {
      src,
      onEach,
      'withLine' : true,
      'eol' : [ '\n' ],
      'charInterval' : [ 2, 4 ],
      'lineIndex' : 1,
      'nl' : '\n',
      'line' : 'ab',
    },
    {
      src,
      onEach,
      'withLine' : true,
      'eol' : [ '\n' ],
      'charInterval' : [ 5, 8 ],
      'lineIndex' : 2,
      'nl' : '\n',
      'line' : 'abc',
    },
    {
      src,
      onEach,
      'withLine' : true,
      'eol' : [ '\n' ],
      'charInterval' : [ 9, 13 ],
      'lineIndex' : 3,
      'nl' : '\n',
      'line' : 'abcd',
    },
    {
      src,
      onEach,
      'withLine' : true,
      'eol' : [],
      'charInterval' : [ 14, 13 ],
      'lineIndex' : 4,
      'nl' : '',
      'line' : '',
    }
  ]
  test.identical( its, exp );
  test.identical( src.length, 14 );

  /* */

  test.case = '\\r\\nab\\n\\rabc\\nabcd\\n';
  clean();
  var src = `\r\nab\n\rabc\nabcd\n`;
  _.str.lines.eachLeft({ src, onEach, eol : _.str.lines.Eol.any });
  var exp =
  [
    {
      src,
      onEach,
      'withLine' : true,
      'eol' : [ '\r\n', '\n\r', '\n' ],
      'charInterval' : [ 0, 1 ],
      'lineIndex' : 0,
      'nl' : '\r\n',
      'line' : '',
    },
    {
      src,
      onEach,
      'withLine' : true,
      'eol' : [ '\n\r', '\n' ],
      'charInterval' : [ 2, 5 ],
      'lineIndex' : 1,
      'nl' : '\n\r',
      'line' : 'ab',
    },
    {
      src,
      onEach,
      'withLine' : true,
      'eol' : [ '\n' ],
      'charInterval' : [ 6, 9 ],
      'lineIndex' : 2,
      'nl' : '\n',
      'line' : 'abc',
    },
    {
      src,
      onEach,
      'withLine' : true,
      'eol' : [ '\n' ],
      'charInterval' : [ 10, 14 ],
      'lineIndex' : 3,
      'nl' : '\n',
      'line' : 'abcd',
    },
    {
      src,
      onEach,
      'withLine' : true,
      'eol' : [],
      'charInterval' : [ 15, 14 ],
      'lineIndex' : 4,
      'nl' : '',
      'line' : '',
    }
  ]
  test.identical( its, exp );
  test.identical( src.length, 15 );

  /* */

  function onEach( it )
  {
    it = _.props.extend( null, it );
    it.charInterval = it.charInterval.slice();
    it.eol = it.eol.slice();
    its.push( it );
  }

  /* */

  function clean()
  {
    its = [];
  }

  /* */

}

//

function eachLeftMultipleInInterval( test )
{
  let its;

  /* */

  test.case = 'empty, -2, -2';
  clean();
  var src = ``;
  var interval = [ -2, -2 ];
  _.str.lines.eachLeft({ src, onEach, interval,  });
  var exp = [];
  test.identical( its, exp );
  test.identical( src.length, 0 );

  /* */

  test.case = 'empty, -1, -1';
  clean();
  var src = ``;
  var interval = [ -1, -1 ];
  _.str.lines.eachLeft({ src, onEach, interval, eol : _.str.lines.Eol.any });
  var exp = [];
  test.identical( its, exp );
  test.identical( src.length, 0 );

  /* */

  test.case = 'empty, 1, 1';
  clean();
  var src = ``;
  var interval = [ 1, 1 ];
  _.str.lines.eachLeft({ src, onEach, interval, eol : _.str.lines.Eol.any });
  var exp = [];
  test.identical( its, exp );
  test.identical( src.length, 0 );

  /* */

  test.case = 'empty, 2, 2';
  clean();
  var src = ``;
  var interval = [ 2, 2 ];
  _.str.lines.eachLeft({ src, onEach, interval, eol : _.str.lines.Eol.any });
  var exp = [];
  test.identical( its, exp );
  test.identical( src.length, 0 );

  /* */

  test.case = 'empty, 0, 0';
  clean();
  var src = ``;
  var interval = [ 0, 0 ];
  _.str.lines.eachLeft({ src, onEach, interval, eol : _.str.lines.Eol.any });
  var exp =
  [
    {
      'src' : '',
      onEach,
      interval,
      'withLine' : true,
      'eol' : [],
      'lineIndex' : 0,
      'charInterval' : [ 0, -1 ],
      'nl' : '',
      'line' : '',
    }
  ]
  test.identical( its, exp );
  test.identical( src.length, 0 );

  /* */

  test.case = 'empty, -1, 0';
  clean();
  var src = ``;
  var interval = [ -1, 0 ];
  _.str.lines.eachLeft({ src, onEach, interval, eol : _.str.lines.Eol.any });
  var exp =
  [
    {
      'src' : '',
      onEach,
      interval,
      'withLine' : true,
      'eol' : [],
      'lineIndex' : 0,
      'charInterval' : [ 0, -1 ],
      'nl' : '',
      'line' : '',
    }
  ]
  test.identical( its, exp );
  test.identical( src.length, 0 );

  /* */

  test.case = 'empty, 0, 1';
  clean();
  var src = ``;
  var interval = [ 0, 1 ];
  _.str.lines.eachLeft({ src, onEach, interval, eol : _.str.lines.Eol.any });
  var exp =
  [
    {
      'src' : '',
      onEach,
      interval,
      'withLine' : true,
      'eol' : [],
      'lineIndex' : 0,
      'charInterval' : [ 0, -1 ],
      'nl' : '',
      'line' : '',
    }
  ]
  test.identical( its, exp );
  test.identical( src.length, 0 );

  /* */

  test.case = 'empty, -1, 1';
  clean();
  var src = ``;
  var interval = [ -1, 1 ];
  _.str.lines.eachLeft({ src, onEach, interval, eol : _.str.lines.Eol.any });
  var exp =
  [
    {
      'src' : '',
      onEach,
      interval,
      'withLine' : true,
      'eol' : [],
      'lineIndex' : 0,
      'charInterval' : [ 0, -1 ],
      'nl' : '',
      'line' : '',
    }
  ]
  test.identical( its, exp );
  test.identical( src.length, 0 );

  /* */

  test.case = 'empty, 1, 1';
  clean();
  var src = ``;
  var interval = [ 1, 1 ];
  _.str.lines.eachLeft({ src, onEach, interval, eol : _.str.lines.Eol.any });
  var exp = [];
  test.identical( its, exp );
  test.identical( src.length, 0 );

  /* */

  test.case = 'aa\\nbb, -Infinity, +Infinity';
  clean();
  var src = `aa\nbb`;
  var interval = [ -Infinity, +Infinity ];
  _.str.lines.eachLeft({ src, onEach, interval, eol : _.str.lines.Eol.any });
  var exp =
  [
    {
      src,
      onEach,
      interval,
      'withLine' : true,
      'eol' : '\n',
      'charInterval' : [ 0, 2 ],
      'lineIndex' : 0,
      'nl' : '\n',
      'line' : 'aa',
    },
    {
      src,
      onEach,
      interval,
      'withLine' : true,
      'eol' : '\n',
      'charInterval' : [ 3, 4 ],
      'lineIndex' : 1,
      'nl' : '',
      'line' : 'bb',
    }
  ]
  test.identical( its, exp );
  test.identical( src.length, 5 );

  /* */

  test.case = 'aa\\nbb, 0, 1';
  clean();
  var src = `aa\nbb`;
  var interval = [ 0, 1 ];
  _.str.lines.eachLeft({ src, onEach, interval, eol : _.str.lines.Eol.any });
  var exp =
  [
    {
      src,
      onEach,
      interval,
      'withLine' : true,
      'eol' : '\n',
      'charInterval' : [ 0, 2 ],
      'lineIndex' : 0,
      'nl' : '\n',
      'line' : 'aa',
    },
    {
      src,
      onEach,
      interval,
      'withLine' : true,
      'eol' : '\n',
      'charInterval' : [ 3, 4 ],
      'lineIndex' : 1,
      'nl' : '',
      'line' : 'bb',
    }
  ]
  test.identical( its, exp );
  test.identical( src.length, 5 );

  /* */

  test.case = 'aa\\nbb, 0, 0';
  clean();
  var src = `aa\nbb`;
  var interval = [ 0, 0 ];
  _.str.lines.eachLeft({ src, onEach, interval, eol : _.str.lines.Eol.any });
  var exp =
  [
    {
      src,
      onEach,
      interval,
      'withLine' : true,
      'eol' : '\n',
      'charInterval' : [ 0, 2 ],
      'lineIndex' : 0,
      'nl' : '\n',
      'line' : 'aa',
    }
  ]
  test.identical( its, exp );
  test.identical( src.length, 5 );

  /* */

  test.case = 'aa\\nbb, 1, 1';
  clean();
  var src = `aa\nbb`;
  var interval = [ 1, 1 ];
  _.str.lines.eachLeft({ src, onEach, interval, eol : _.str.lines.Eol.any });
  var exp =
  [
    {
      src,
      onEach,
      interval,
      'withLine' : true,
      'eol' : '\n',
      'charInterval' : [ 3, 4 ],
      'lineIndex' : 1,
      'nl' : '',
      'line' : 'bb',
    }
  ]
  test.identical( its, exp );
  test.identical( src.length, 5 );

  /* */

  test.case = 'aa\\nbb, -2, -2';
  clean();
  var src = `aa\nbb`;
  var interval = [ -2, -2 ];
  _.str.lines.eachLeft({ src, onEach, interval, eol : _.str.lines.Eol.any });
  var exp = [];
  test.identical( its, exp );
  test.identical( src.length, 5 );

  /* */

  test.case = 'aa\\nbb, -1, -1';
  clean();
  var src = `aa\nbb`;
  var interval = [ -1, -1 ];
  _.str.lines.eachLeft({ src, onEach, interval, eol : _.str.lines.Eol.any });
  var exp = [];
  test.identical( its, exp );
  test.identical( src.length, 5 );

  /* */

  test.case = 'aa\\nbb, 0, -1';
  clean();
  var src = `aa\nbb`;
  var interval = [ 0, -1 ];
  _.str.lines.eachLeft({ src, onEach, interval, eol : _.str.lines.Eol.any });
  var exp = [];
  test.identical( its, exp );
  test.identical( src.length, 5 );

  /* */

  test.case = 'aa\\nbb, 2, 1';
  clean();
  var src = `aa\nbb`;
  var interval = [ 2, 1 ];
  _.str.lines.eachLeft({ src, onEach, interval, eol : _.str.lines.Eol.any });
  var exp = [];
  test.identical( its, exp );
  test.identical( src.length, 5 );

  /* */

  test.case = 'aa\\nbb, 2, 2';
  clean();
  var src = `aa\nbb`;
  var interval = [ 2, 2 ];
  _.str.lines.eachLeft({ src, onEach, interval, eol : _.str.lines.Eol.any });
  var exp = [];
  test.identical( its, exp );
  test.identical( src.length, 5 );

  /* */

  test.case = '0, 4';
  clean();
  var interval = [ 0, 4 ];
  var src = `\r\nab\n\rabc\nabcd\n`;
  _.str.lines.eachLeft({ src, onEach, interval, eol : _.str.lines.Eol.any });
  var exp =
  [
    {
      src,
      onEach,
      interval,
      'withLine' : true,
      'eol' : [ '\r\n', '\n\r', '\n' ],
      'charInterval' : [ 0, 1 ],
      'lineIndex' : 0,
      'nl' : '\r\n',
      'line' : '',
    },
    {
      src,
      onEach,
      interval,
      'withLine' : true,
      'eol' : [ '\n\r', '\n' ],
      'charInterval' : [ 2, 5 ],
      'lineIndex' : 1,
      'nl' : '\n\r',
      'line' : 'ab',
    },
    {
      src,
      onEach,
      interval,
      'withLine' : true,
      'eol' : [ '\n' ],
      'charInterval' : [ 6, 9 ],
      'lineIndex' : 2,
      'nl' : '\n',
      'line' : 'abc',
    },
    {
      src,
      onEach,
      interval,
      'withLine' : true,
      'eol' : [ '\n' ],
      'charInterval' : [ 10, 14 ],
      'lineIndex' : 3,
      'nl' : '\n',
      'line' : 'abcd',
    },
    {
      src,
      onEach,
      interval,
      'withLine' : true,
      'eol' : [],
      'charInterval' : [ 15, 14 ],
      'lineIndex' : 4,
      'nl' : '',
      'line' : '',
    }
  ]
  test.identical( its, exp );
  test.identical( src.length, 15 );

  /* */

  test.case = '-1, 5';
  clean();
  var interval = [ -1, 5 ];
  var src = `\r\nab\n\rabc\nabcd\n`;
  _.str.lines.eachLeft({ src, onEach, interval, eol : _.str.lines.Eol.any });
  var exp =
  [
    {
      src,
      onEach,
      interval,
      'withLine' : true,
      'eol' : [ '\r\n', '\n\r', '\n' ],
      'charInterval' : [ 0, 1 ],
      'lineIndex' : 0,
      'nl' : '\r\n',
      'line' : '',
    },
    {
      src,
      onEach,
      interval,
      'withLine' : true,
      'eol' : [ '\n\r', '\n' ],
      'charInterval' : [ 2, 5 ],
      'lineIndex' : 1,
      'nl' : '\n\r',
      'line' : 'ab',
    },
    {
      src,
      onEach,
      interval,
      'withLine' : true,
      'eol' : [ '\n' ],
      'charInterval' : [ 6, 9 ],
      'lineIndex' : 2,
      'nl' : '\n',
      'line' : 'abc',
    },
    {
      src,
      onEach,
      interval,
      'withLine' : true,
      'eol' : [ '\n' ],
      'charInterval' : [ 10, 14 ],
      'lineIndex' : 3,
      'nl' : '\n',
      'line' : 'abcd',
    },
    {
      src,
      onEach,
      interval,
      'withLine' : true,
      'eol' : [],
      'charInterval' : [ 15, 14 ],
      'lineIndex' : 4,
      'nl' : '',
      'line' : '',
    }
  ]
  test.identical( its, exp );
  test.identical( src.length, 15 );

  /* */

  test.case = '1, 3';
  clean();
  var interval = [ 1, 3 ];
  var src = `\r\nab\n\rabc\nabcd\n`;
  _.str.lines.eachLeft({ src, onEach, interval, eol : _.str.lines.Eol.any });
  var exp =
  [
    {
      src,
      onEach,
      interval,
      'withLine' : true,
      'eol' : [ '\n\r', '\n' ],
      'charInterval' : [ 2, 5 ],
      'lineIndex' : 1,
      'nl' : '\n\r',
      'line' : 'ab',
    },
    {
      src,
      onEach,
      interval,
      'withLine' : true,
      'eol' : [ '\n' ],
      'charInterval' : [ 6, 9 ],
      'lineIndex' : 2,
      'nl' : '\n',
      'line' : 'abc',
    },
    {
      src,
      onEach,
      interval,
      'withLine' : true,
      'eol' : [ '\n' ],
      'charInterval' : [ 10, 14 ],
      'lineIndex' : 3,
      'nl' : '\n',
      'line' : 'abcd',
    },
  ]
  test.identical( its, exp );
  test.identical( src.length, 15 );

  /* */

  test.case = '2, 2';
  clean();
  var interval = [ 2, 2 ];
  var src = `\r\nab\n\rabc\nabcd\n`;
  _.str.lines.eachLeft({ src, onEach, interval, eol : _.str.lines.Eol.any });
  var exp =
  [
    {
      src,
      onEach,
      interval,
      'withLine' : true,
      'eol' : [ '\n' ],
      'charInterval' : [ 6, 9 ],
      'lineIndex' : 2,
      'nl' : '\n',
      'line' : 'abc',
    },
  ]
  test.identical( its, exp );
  test.identical( src.length, 15 );

  /* */

  test.case = '0, 0';
  clean();
  var interval = [ -2, 0 ];
  var src = `\r\nab\n\rabc\nabcd\n`;
  _.str.lines.eachLeft({ src, onEach, interval, eol : _.str.lines.Eol.any });
  var exp =
  [
    {
      src,
      onEach,
      interval,
      'withLine' : true,
      'eol' : [ '\r\n', '\n\r', '\n' ],
      'charInterval' : [ 0, 1 ],
      'lineIndex' : 0,
      'nl' : '\r\n',
      'line' : '',
    },
  ]
  test.identical( its, exp );
  test.identical( src.length, 15 );

  /* */

  test.case = '-2, 0';
  clean();
  var interval = [ -2, 0 ];
  var src = `\r\nab\n\rabc\nabcd\n`;
  _.str.lines.eachLeft({ src, onEach, interval, eol : _.str.lines.Eol.any });
  var exp =
  [
    {
      src,
      onEach,
      interval,
      'withLine' : true,
      'eol' : [ '\r\n', '\n\r', '\n' ],
      'charInterval' : [ 0, 1 ],
      'lineIndex' : 0,
      'nl' : '\r\n',
      'line' : '',
    },
  ]
  test.identical( its, exp );
  test.identical( src.length, 15 );

  /* */

  test.case = '4, 4';
  clean();
  var interval = [ 4, 4 ];
  var src = `\r\nab\n\rabc\nabcd\n`;
  _.str.lines.eachLeft({ src, onEach, interval, eol : _.str.lines.Eol.any });
  var exp =
  [
    {
      src,
      onEach,
      interval,
      'withLine' : true,
      'eol' : [],
      'charInterval' : [ 15, 14 ],
      'lineIndex' : 4,
      'nl' : '',
      'line' : '',
    }
  ]
  test.identical( its, exp );
  test.identical( src.length, 15 );

  /* */

  test.case = '4, 6';
  clean();
  var interval = [ 4, 6 ];
  var src = `\r\nab\n\rabc\nabcd\n`;
  _.str.lines.eachLeft({ src, onEach, interval, eol : _.str.lines.Eol.any });
  var exp =
  [
    {
      src,
      onEach,
      interval,
      'withLine' : true,
      'eol' : [],
      'charInterval' : [ 15, 14 ],
      'lineIndex' : 4,
      'nl' : '',
      'line' : '',
    }
  ]
  test.identical( its, exp );
  test.identical( src.length, 15 );

  /* */

  test.case = '3, 2';
  clean();
  var interval = [ 3, 2 ];
  var src = `\r\nab\n\rabc\nabcd\n`;
  _.str.lines.eachLeft({ src, onEach, interval, eol : _.str.lines.Eol.any });
  var exp = [];
  test.identical( its, exp );
  test.identical( src.length, 15 );

  /* */

  test.case = '2, 1';
  clean();
  var interval = [ 2, 1 ];
  var src = `\r\nab\n\rabc\nabcd\n`;
  _.str.lines.eachLeft({ src, onEach, interval, eol : _.str.lines.Eol.any });
  var exp = [];
  test.identical( its, exp );
  test.identical( src.length, 15 );

  /* */

  test.case = '-1, -1';
  clean();
  var interval = [ -1, -1 ];
  var src = `\r\nab\n\rabc\nabcd\n`;
  _.str.lines.eachLeft({ src, onEach, interval, eol : _.str.lines.Eol.any });
  var exp = [];
  test.identical( its, exp );
  test.identical( src.length, 15 );

  /* */

  test.case = '0, -1';
  clean();
  var interval = [ 0, -1 ];
  var src = `\r\nab\n\rabc\nabcd\n`;
  _.str.lines.eachLeft({ src, onEach, interval, eol : _.str.lines.Eol.any });
  var exp = [];
  test.identical( its, exp );
  test.identical( src.length, 15 );

  /* */

  test.case = '-2, -1';
  clean();
  var interval = [ -2, -1 ];
  var src = `\r\nab\n\rabc\nabcd\n`;
  _.str.lines.eachLeft({ src, onEach, interval, eol : _.str.lines.Eol.any });
  var exp = [];
  test.identical( its, exp );
  test.identical( src.length, 15 );

  /* */

  test.case = '5, 6';
  clean();
  var interval = [ 5, 6 ];
  var src = `\r\nab\n\rabc\nabcd\n`;
  _.str.lines.eachLeft({ src, onEach, interval, eol : _.str.lines.Eol.any });
  var exp = [];
  test.identical( its, exp );
  test.identical( src.length, 15 );

  /* */

  function onEach( it )
  {
    it = _.props.extend( null, it );
    it.charInterval = it.charInterval.slice();
    it.eol = it.eol.slice();
    it.interval = it.interval.slice();
    its.push( it );
  }

  /* */

  function clean()
  {
    its = [];
  }

  /* */

}

//

function eachLeftCallbacks( test )
{
  let its;

  act({ interval : 0, withLine : 0 });
  act({ interval : 0, withLine : 1 });
  act({ interval : 1, withLine : 0 });
  act({ interval : 1, withLine : 1 });

  /* */

  function act( env )
  {
    test.case = `${__.entity.exportStringSolo( env )}`;

    clean();
    var src = `aa\nbb\r\ncc`;
    var o = { src, onEach, withLine : env.withLine, eol : _.str.lines.Eol.any };
    if( env.interval )
    o.interval = [ 0, 2 ];
    var got = _.str.lines.eachLeft( o );
    test.true( o === got );
    var exp =
    {
      src,
      onEach,
      'withLine' : env.withLine,
      'eol' : [],
      'charInterval' : [ 9, 8 ],
      'lineIndex' : 3,
      'nl' : '',
    }
    if( env.withLine )
    exp.line = '';
    if( env.interval )
    {
      exp.interval = [ 0, 2 ];
      exp.charInterval = [ 7, 8 ];
      exp.lineIndex = 2;
      if( env.withLine )
      exp.line = 'cc';
    }
    test.identical( o, exp );

    var exp =
    [
      {
        src,
        onEach,
        'withLine' : env.withLine,
        'eol' : [ '\r\n', '\n' ],
        'charInterval' : [ 0, 2 ],
        'lineIndex' : 0,
        'nl' : '\n',
      },
      {
        src,
        onEach,
        'withLine' : env.withLine,
        'eol' : [ '\r\n', '\n' ],
        'charInterval' : [ 3, 6 ],
        'lineIndex' : 1,
        'nl' : '\r\n',
      },
      {
        src,
        onEach,
        'withLine' : env.withLine,
        'eol' : [],
        'charInterval' : [ 7, 8 ],
        'lineIndex' : 2,
        'nl' : '',
      },
    ]
    if( env.withLine )
    {
      exp[ 0 ].line = 'aa';
      exp[ 1 ].line = 'bb';
      exp[ 2 ].line = 'cc';
    }
    if( env.interval )
    {
      exp[ 0 ].interval = [ 0, 2 ];
      exp[ 1 ].interval = [ 0, 2 ];
      exp[ 2 ].interval = [ 0, 2 ];
    }
    test.identical( its, exp );
    test.identical( src.length, 9 );

  }

  /* */

  function onEach( it )
  {
    it = _.props.extend( null, it );
    it.charInterval = it.charInterval.slice();
    it.eol = it.eol.slice();
    if( it.interval )
    it.interval = it.interval.slice();
    its.push( it );
  }

  /* */

  function clean()
  {
    its = [];
  }

  /* */

}

//

function atLeftSingle( test )
{
  test.case = 'empty';
  var src = ``;
  var exp  =
  {
    src,
    lineIndex : -2,
    charInterval : [ 0, -1 ],
  };
  var got = _.str.lines.atLeft( src, -2 );
  test.identical( got, exp );
  var exp  =
  {
    src,
    lineIndex : -1,
    charInterval : [ 0, -1 ],
  };
  var got = _.str.lines.atLeft( src, -1 );
  test.identical( got, exp );
  var exp  =
  {
    src,
    lineIndex : 0,
    charInterval : [ 0, -1 ],
    line : '',
    nl : '',
  }
  var got = _.str.lines.atLeft( src, 0 );
  test.identical( got, exp );
  var exp  =
  {
    src,
    lineIndex : 1,
    charInterval : [ 0, -1 ],
  }
  var got = _.str.lines.atLeft( src, 1 );
  test.identical( got, exp );
  var exp  =
  {
    src,
    lineIndex : 2,
    charInterval : [ 0, -1 ],
  }
  var got = _.str.lines.atLeft( src, 2 );
  test.identical( got, exp );
  test.identical( src.length, 0 );

  /* */

  test.case = '\\n';
  var src = `\n`;
  var exp  =
  {
    src,
    lineIndex : -2,
    charInterval : [ 0, -1 ],
  }
  var got = _.str.lines.atLeft( src, -2 );
  test.identical( got, exp );

  var exp  =
  {
    src,
    lineIndex : -1,
    charInterval : [ 0, -1 ],
  }
  var got = _.str.lines.atLeft( src, -1 );
  test.identical( got, exp );
  var exp  =
  {
    src,
    lineIndex : 0,
    charInterval : [ 0, 0 ],
    line : '',
    nl : '\n',
  }
  var got = _.str.lines.atLeft( src, 0 );
  test.identical( got, exp );
  var exp  =
  {
    src,
    lineIndex : 1,
    charInterval : [ 1, 0 ],
    line : '',
    nl : '',
  }
  var got = _.str.lines.atLeft( src, 1 );
  test.identical( got, exp );
  var exp  =
  {
    src,
    lineIndex : 2,
    charInterval : [ 1, 0 ],
  }
  var got = _.str.lines.atLeft( src, 2 );
  test.identical( got, exp );
  var exp  =
  {
    src,
    lineIndex : 3,
    charInterval : [ 1, 0 ],
  }
  var got = _.str.lines.atLeft( src, 3 );
  test.identical( got, exp );
  test.identical( src.length, 1 );

  /* */

  test.case = 'a\\nb';
  var src = `a\nb`;
  var exp  =
  {
    src,
    lineIndex : -2,
    charInterval : [ 0, -1 ],
  }
  var got = _.str.lines.atLeft( src, -2 );
  test.identical( got, exp );
  var exp  =
  {
    src,
    lineIndex : -1,
    charInterval : [ 0, -1 ],
  }
  var got = _.str.lines.atLeft( src, -1 );
  test.identical( got, exp );
  var exp  =
  {
    src,
    lineIndex : 0,
    charInterval : [ 0, 1 ],
    line : 'a',
    nl : '\n',
  }
  var got = _.str.lines.atLeft( src, 0 );
  test.identical( got, exp );
  var exp  =
  {
    src,
    lineIndex : 1,
    charInterval : [ 2, 2 ],
    line : 'b',
    nl : '',
  }
  var got = _.str.lines.atLeft( src, 1 );
  test.identical( got, exp );
  var exp  =
  {
    src,
    lineIndex : 2,
    charInterval : [ 3, 2 ],
  }
  var got = _.str.lines.atLeft( src, 2 );
  test.identical( got, exp );
  var exp  =
  {
    src,
    lineIndex : 3,
    charInterval : [ 3, 2 ],
  }
  test.identical( src.length, 3 );

  /* */

  test.case = 'ab\\nabc\\nabcd';
  var src = `ab\nabc\nabcd`;
  var exp  =
  {
    src,
    lineIndex : -2,
    charInterval : [ 0, -1 ],
  }
  var got = _.str.lines.atLeft( src, -2 );
  test.identical( got, exp );
  var exp  =
  {
    src,
    lineIndex : -1,
    charInterval : [ 0, -1 ],
  }
  var got = _.str.lines.atLeft( src, -1 );
  test.identical( got, exp );
  var exp  =
  {
    src,
    lineIndex : 0,
    charInterval : [ 0, 2 ],
    line : 'ab',
    nl : '\n',
  }
  var got = _.str.lines.atLeft( src, 0 );
  test.identical( got, exp );
  var exp  =
  {
    src,
    lineIndex : 1,
    charInterval : [ 3, 6 ],
    line : 'abc',
    nl : '\n',
  }
  var got = _.str.lines.atLeft( src, 1 );
  test.identical( got, exp );
  var exp  =
  {
    src,
    lineIndex : 2,
    charInterval : [ 7, 10 ],
    line : 'abcd',
    nl : '',
  }
  var got = _.str.lines.atLeft( src, 2 );
  test.identical( got, exp );
  var exp  =
  {
    src,
    lineIndex : 3,
    charInterval : [ 11, 10 ],
  }
  var got = _.str.lines.atLeft( src, 3 );
  test.identical( got, exp );
  var exp  =
  {
    src,
    lineIndex : 4,
    charInterval : [ 11, 10 ],
  }
  var got = _.str.lines.atLeft( src, 4 );
  test.identical( got, exp );
  test.identical( src.length, 11 );

  /* */

  test.case = '\\nab\\nabc\\nabcd\\n';
  var src = `\nab\nabc\nabcd\n`;
  var exp  =
  {
    src,
    lineIndex : -2,
    charInterval : [ 0, -1 ],
  }
  var got = _.str.lines.atLeft( src, -2 );
  test.identical( got, exp );
  var exp  =
  {
    src,
    lineIndex : -1,
    charInterval : [ 0, -1 ],
  }
  var got = _.str.lines.atLeft( src, -1 );
  test.identical( got, exp );
  var exp  =
  {
    src,
    lineIndex : 0,
    charInterval : [ 0, 0 ],
    line : '',
    nl : '\n',
  }
  var got = _.str.lines.atLeft( src, 0 );
  test.identical( got, exp );
  var exp  =
  {
    src,
    lineIndex : 1,
    charInterval : [ 1, 3 ],
    line : 'ab',
    nl : '\n',
  }
  var got = _.str.lines.atLeft( src, 1 );
  test.identical( got, exp );
  var exp  =
  {
    src,
    lineIndex : 2,
    charInterval : [ 4, 7 ],
    line : 'abc',
    nl : '\n',
  }
  var got = _.str.lines.atLeft( src, 2 );
  test.identical( got, exp );
  var exp  =
  {
    src,
    lineIndex : 3,
    charInterval : [ 8, 12 ],
    line : 'abcd',
    nl : '\n',
  }
  var got = _.str.lines.atLeft( src, 3 );
  test.identical( got, exp );
  var exp  =
  {
    src,
    lineIndex : 4,
    charInterval : [ 13, 12 ],
    line : '',
    nl : '',
  }
  var got = _.str.lines.atLeft( src, 4 );
  test.identical( got, exp );
  var exp  =
  {
    src,
    lineIndex : 5,
    charInterval : [ 13, 12 ],
  }
  var got = _.str.lines.atLeft( src, 5 );
  test.identical( got, exp );
  test.identical( src.length, 13 );
}

atLeftSingle.timeOut = 10000;

//

function atLeftCustomToken( test )
{

  /* */

  test.case = 'a\\nb';
  var src = `a\nb`;
  var exp  =
  {
    'src' : src,
    'lineIndex' : -2,
    'charInterval' : [ 0, -1 ],
  }
  var got = _.str.lines.atLeft( src, -2, '\n' );
  test.identical( got, exp );
  var exp  =
  {
    'src' : src,
    'lineIndex' : -1,
    'charInterval' : [ 0, -1 ],
  }
  var got = _.str.lines.atLeft( src, -1, '\n' );
  test.identical( got, exp );
  var exp  =
  {
    'src' : src,
    'lineIndex' : 0,
    'charInterval' : [ 0, 1 ],
    'line' : 'a',
    'nl' : '\n',
  }
  var got = _.str.lines.atLeft( src, 0, '\n' );
  test.identical( got, exp );
  var exp  =
  {
    'src' : src,
    'lineIndex' : 1,
    'charInterval' : [ 2, 2 ],
    'line' : 'b',
    'nl' : '',
  }
  var got = _.str.lines.atLeft( src, 1, '\n' );
  test.identical( got, exp );
  var exp  =
  {
    'src' : src,
    'lineIndex' : 2,
    'charInterval' : [ 3, 2 ],
  }
  var got = _.str.lines.atLeft( src, 2, '\n' );
  test.identical( got, exp );
  var exp  =
  {
    'src' : src,
    'lineIndex' : 3,
    'charInterval' : [ 3, 3 ],
  }
  test.identical( src.length, 3 );

  /* */

  test.case = 'a\\rb - custom token';
  var src = `a\rb`;
  var exp  =
  {
    'src' : src,
    'lineIndex' : -2,
    'charInterval' : [ 0, -1 ],
  }
  var got = _.str.lines.atLeft( src, -2, '\r' );
  test.identical( got, exp );
  var exp  =
  {
    'src' : src,
    'lineIndex' : -1,
    'charInterval' : [ 0, -1 ],
  }
  var got = _.str.lines.atLeft( src, -1, '\r' );
  test.identical( got, exp );
  var exp  =
  {
    'src' : src,
    'lineIndex' : 0,
    'charInterval' : [ 0, 1 ],
    'line' : 'a',
    'nl' : '\r',
  }
  var got = _.str.lines.atLeft( src, 0, '\r' );
  test.identical( got, exp );
  var exp  =
  {
    'src' : src,
    'lineIndex' : 1,
    'charInterval' : [ 2, 2 ],
    'line' : 'b',
    'nl' : '',
  }
  var got = _.str.lines.atLeft( src, 1, '\r' );
  test.identical( got, exp );
  var exp  =
  {
    'src' : src,
    'lineIndex' : 2,
    'charInterval' : [ 3, 2 ],
  }
  var got = _.str.lines.atLeft( src, 2, '\r' );
  test.identical( got, exp );
  var exp  =
  {
    'src' : src,
    'lineIndex' : 3,
    'charInterval' : [ 3, 2 ],
  }
  var got = _.str.lines.atLeft( src, 3, '\r' );
  test.identical( got, exp );
  test.identical( src.length, 3 );

  /* */

}

//

function atLeftMultiple( test )
{

  /* */

  test.case = '\\nab\\nabc\\nabcd\\r\\n';
  var src = `\nab\nabc\nabcd\r\n`;
  var exp  =
  {
    'src' : src,
    'lineIndex' : -1,
    'charInterval' : [ 0, -1 ],
  }
  var got = _.str.lines.atLeft( src, -1, _.str.lines.Eol.any );
  test.identical( got, exp );
  var exp  =
  {
    'src' : src,
    'lineIndex' : 0,
    'charInterval' : [ 0, 0 ],
    'line' : '',
    'nl' : '\n',
  }
  var got = _.str.lines.atLeft( src, 0, _.str.lines.Eol.any );
  test.identical( got, exp );
  var exp  =
  {
    'src' : src,
    'lineIndex' : 1,
    'charInterval' : [ 1, 3 ],
    'line' : 'ab',
    'nl' : '\n',
  }
  var got = _.str.lines.atLeft( src, 1, _.str.lines.Eol.any );
  test.identical( got, exp );
  var exp  =
  {
    'src' : src,
    'lineIndex' : 2,
    'charInterval' : [ 4, 7 ],
    'line' : 'abc',
    'nl' : '\n',
  }
  var got = _.str.lines.atLeft( src, 2, _.str.lines.Eol.any );
  test.identical( got, exp );
  var exp  =
  {
    'src' : src,
    'lineIndex' : 3,
    'charInterval' : [ 8, 13 ],
    'line' : 'abcd',
    'nl' : '\r\n',
  }
  var got = _.str.lines.atLeft( src, 3, _.str.lines.Eol.any );
  test.identical( got, exp );
  var exp  =
  {
    'src' : src,
    'lineIndex' : 4,
    'charInterval' : [ 14, 13 ],
    'line' : '',
    'nl' : '',
  }
  var got = _.str.lines.atLeft( src, 4, _.str.lines.Eol.any );
  test.identical( got, exp );
  var exp  =
  {
    'src' : src,
    'lineIndex' : 5,
    'charInterval' : [ 14, 13 ],
  }
  var got = _.str.lines.atLeft( src, 5, _.str.lines.Eol.any );
  test.identical( got, exp );
  test.identical( src.length, 14 );

  /* */

  test.case = '\\r\\nab\\nabc\\nabcd\\n';
  var src = `\r\nab\nabc\nabcd\n`;
  var exp  =
  {
    'src' : src,
    'lineIndex' : -1,
    'charInterval' : [ 0, -1 ],
  }
  var got = _.str.lines.atLeft( src, -1, _.str.lines.Eol.any );
  test.identical( got, exp );
  var exp  =
  {
    'src' : src,
    'lineIndex' : 0,
    'charInterval' : [ 0, 1 ],
    'line' : '',
    'nl' : '\r\n',
  }
  var got = _.str.lines.atLeft( src, 0, _.str.lines.Eol.any );
  test.identical( got, exp );
  var exp  =
  {
    'src' : src,
    'lineIndex' : 1,
    'charInterval' : [ 2, 4 ],
    'line' : 'ab',
    'nl' : '\n',
  }
  var got = _.str.lines.atLeft( src, 1, _.str.lines.Eol.any );
  test.identical( got, exp );
  var exp  =
  {
    'src' : src,
    'lineIndex' : 2,
    'charInterval' : [ 5, 8 ],
    'line' : 'abc',
    'nl' : '\n',
  }
  var got = _.str.lines.atLeft( src, 2, _.str.lines.Eol.any );
  test.identical( got, exp );
  var exp  =
  {
    'src' : src,
    'lineIndex' : 3,
    'charInterval' : [ 9, 13 ],
    'line' : 'abcd',
    'nl' : '\n',
  }
  var got = _.str.lines.atLeft( src, 3, _.str.lines.Eol.any );
  test.identical( got, exp );
  var exp  =
  {
    'src' : src,
    'lineIndex' : 4,
    'charInterval' : [ 14, 13 ],
    'line' : '',
    'nl' : '',
  }
  var got = _.str.lines.atLeft( src, 4, _.str.lines.Eol.any );
  test.identical( got, exp );
  var exp  =
  {
    'src' : src,
    'lineIndex' : 5,
    'charInterval' : [ 14, 13 ],
  }
  var got = _.str.lines.atLeft( src, 5, _.str.lines.Eol.any );
  test.identical( got, exp );
  test.identical( src.length, 14 );

  /* */

  test.case = '\\r\\nab\\n\\rabc\\nabcd\\n';
  var src = `\r\nab\n\rabc\nabcd\n`;
  var exp  =
  {
    'src' : src,
    'lineIndex' : -1,
    'charInterval' : [ 0, -1 ],
  }
  var got = _.str.lines.atLeft( src, -1, _.str.lines.Eol.any );
  test.identical( got, exp );
  var exp  =
  {
    'src' : src,
    'lineIndex' : 0,
    'charInterval' : [ 0, 1 ],
    'line' : '',
    'nl' : '\r\n',
  }
  var got = _.str.lines.atLeft( src, 0, _.str.lines.Eol.any );
  test.identical( got, exp );
  var exp  =
  {
    'src' : src,
    'lineIndex' : 1,
    'charInterval' : [ 2, 5 ],
    'line' : 'ab',
    'nl' : '\n\r',
  }
  var got = _.str.lines.atLeft( src, 1, _.str.lines.Eol.any );
  test.identical( got, exp );
  var exp  =
  {
    'src' : src,
    'lineIndex' : 2,
    'charInterval' : [ 6, 9 ],
    'line' : 'abc',
    'nl' : '\n',
  }
  var got = _.str.lines.atLeft( src, 2, _.str.lines.Eol.any );
  test.identical( got, exp );
  var exp  =
  {
    'src' : src,
    'lineIndex' : 3,
    'charInterval' : [ 10, 14 ],
    'line' : 'abcd',
    'nl' : '\n',
  }
  var got = _.str.lines.atLeft( src, 3, _.str.lines.Eol.any );
  test.identical( got, exp );
  var exp  =
  {
    'src' : src,
    'lineIndex' : 4,
    'charInterval' : [ 15, 14 ],
    'line' : '',
    'nl' : '',
  }
  var got = _.str.lines.atLeft( src, 4, _.str.lines.Eol.any );
  test.identical( got, exp );
  var exp  =
  {
    'src' : src,
    'lineIndex' : 5,
    'charInterval' : [ 15, 14 ],
  }
  var got = _.str.lines.atLeft( src, 5, _.str.lines.Eol.any );
  test.identical( got, exp );
  test.identical( src.length, 15 );

  /* */

}

//

function eachRightSingle( test )
{
  let its;

  /* */

  test.case = 'empty';
  clean();
  var src = ``;
  _.str.lines.eachRight({ src, onEach, eol : _.str.lines.Eol.any });
  var exp =
  [
    {
      'src' : '',
      onEach,
      'withLine' : true,
      'withLine' : true,
      'eol' : [],
      'lineIndex' : 0,
      'charInterval' : [ 0, -1 ],
      'nl' : '',
      'line' : '',
    }
  ]
  test.identical( its, exp );
  test.identical( src.length, 0 );

  /* */

  test.case = 'single line';
  clean();
  var src = `abc`;
  _.str.lines.eachRight({ src, onEach, eol : _.str.lines.Eol.any });
  var exp =
  [
    {
      'src' : 'abc',
      onEach,
      'withLine' : true,
      'withLine' : true,
      'eol' : [],
      'lineIndex' : 0,
      'charInterval' : [ 0, 2 ],
      'nl' : '',
      'line' : 'abc',
    }
  ]
  test.identical( its, exp );
  test.identical( src.length, 3 );

  /* */

  test.case = '\\n';
  clean();
  var src = `\n`;
  _.str.lines.eachRight({ src, onEach, eol : _.str.lines.Eol.any });
  var exp =
  [
    {
      src,
      onEach,
      'withLine' : true,
      'eol' : '\n',
      'charInterval' : [ 1, 0 ],
      'lineIndex' : 0,
      'nl' : '',
      'line' : '',
    },
    {
      src,
      onEach,
      'withLine' : true,
      'eol' : '\n',
      'charInterval' : [ 0, 0 ],
      'lineIndex' : 1,
      'nl' : '\n',
      'line' : '',
    }
  ]
  test.identical( its, exp );
  test.identical( src.length, 1 );

  /* */

  test.case = 'aa\\nbb';
  clean();
  var src = `aa\nbb`;
  _.str.lines.eachRight({ src, onEach, eol : _.str.lines.Eol.any });
  var exp =
  [
    {
      src,
      onEach,
      'withLine' : true,
      'eol' : '\n',
      'charInterval' : [ 3, 4 ],
      'lineIndex' : 0,
      'nl' : '',
      'line' : 'bb',
    },
    {
      src,
      onEach,
      'withLine' : true,
      'eol' : '\n',
      'charInterval' : [ 0, 2 ],
      'lineIndex' : 1,
      'nl' : '\n',
      'line' : 'aa',
    },
  ]
  test.identical( its, exp );
  test.identical( src.length, 5 );

  /* */

  test.case = 'ab\\nabc\\nabcd';
  clean();
  var src = `ab\nabc\nabcd`;
  _.str.lines.eachRight({ src, onEach, eol : _.str.lines.Eol.any });
  var exp =
  [
    {
      src,
      onEach,
      'withLine' : true,
      'eol' : '\n',
      'charInterval' : [ 7, 10 ],
      'lineIndex' : 0,
      'nl' : '',
      'line' : 'abcd',
    },
    {
      src,
      onEach,
      'withLine' : true,
      'eol' : '\n',
      'charInterval' : [ 3, 6 ],
      'lineIndex' : 1,
      'nl' : '\n',
      'line' : 'abc',
    },
    {
      src,
      onEach,
      'withLine' : true,
      'eol' : '\n',
      'charInterval' : [ 0, 2 ],
      'lineIndex' : 2,
      'nl' : '\n',
      'line' : 'ab',
    },
  ]
  test.identical( its, exp );
  test.identical( src.length, 11 );

  /* */

  test.case = '\\nab\\nabc\\nabcd\\n';
  clean();
  var src = `\nab\nabc\nabcd\n`;
  _.str.lines.eachRight({ src, onEach, eol : _.str.lines.Eol.any });
  var exp =
  [
    {
      src,
      onEach,
      'withLine' : true,
      'eol' : '\n',
      'charInterval' : [ 13, 12 ],
      'lineIndex' : 0,
      'nl' : '',
      'line' : '',
    },
    {
      src,
      onEach,
      'withLine' : true,
      'eol' : '\n',
      'charInterval' : [ 8, 12 ],
      'lineIndex' : 1,
      'nl' : '\n',
      'line' : 'abcd',
    },
    {
      src,
      onEach,
      'withLine' : true,
      'eol' : '\n',
      'charInterval' : [ 4, 7 ],
      'lineIndex' : 2,
      'nl' : '\n',
      'line' : 'abc',
    },
    {
      src,
      onEach,
      'withLine' : true,
      'eol' : '\n',
      'charInterval' : [ 1, 3 ],
      'lineIndex' : 3,
      'nl' : '\n',
      'line' : 'ab',
    },
    {
      src,
      onEach,
      'withLine' : true,
      'eol' : '\n',
      'charInterval' : [ 0, 0 ],
      'lineIndex' : 4,
      'nl' : '\n',
      'line' : '',
    },
  ]
  test.identical( its, exp );
  test.identical( src.length, 13 );

  /* */

  function onEach( it )
  {
    it = _.props.extend( null, it );
    it.charInterval = it.charInterval.slice();
    it.eol = it.eol.slice();
    its.push( it );
  }

  /* */

  function clean()
  {
    its = [];
  }

  /* */

}

//

function eachRightCustomToken( test )
{
  let its;

  /* */

  test.case = '\\n';
  clean();
  var eol = '\n';
  var src = `\n`;
  _.str.lines.eachRight({ src, onEach, eol });
  var exp =
  [
    {
      src,
      onEach,
      'withLine' : true,
      'eol' : '\n',
      'charInterval' : [ 1, 0 ],
      'lineIndex' : 0,
      'nl' : '',
      'line' : '',
    },
    {
      src,
      onEach,
      'withLine' : true,
      'eol' : '\n',
      'charInterval' : [ 0, 0 ],
      'lineIndex' : 1,
      'nl' : '\n',
      'line' : '',
    },
  ]
  test.identical( its, exp );
  test.identical( src.length, 1 );

  /* */

  test.case = 'ab\\nabc\\nabcd';
  clean();
  var eol = '\r\n';
  var src = `ab\r\nabc\r\nabcd`;
  _.str.lines.eachRight({ src, onEach, eol });
  var exp =
  [
    {
      src,
      onEach,
      'withLine' : true,
      'eol' : '\r\n',
      'charInterval' : [ 9, 12 ],
      'lineIndex' : 0,
      'nl' : '',
      'line' : 'abcd',
    },
    {
      src,
      onEach,
      'withLine' : true,
      'eol' : '\r\n',
      'charInterval' : [ 4, 8 ],
      'lineIndex' : 1,
      'nl' : '\r\n',
      'line' : 'abc',
    },
    {
      src,
      onEach,
      'withLine' : true,
      'eol' : '\r\n',
      'charInterval' : [ 0, 3 ],
      'lineIndex' : 2,
      'nl' : '\r\n',
      'line' : 'ab',
    },
  ]
  test.identical( its, exp );
  test.identical( src.length, 13 );

  /* */

  test.case = '\\r\\nab\\r\\nabc\\r\\nabcd\\r\\n';
  clean();
  var src = `\r\nab\r\nabc\r\nabcd\r\n`;
  _.str.lines.eachRight({ src, onEach, eol });
  var exp =
  [
    {
      src,
      onEach,
      'withLine' : true,
      'eol' : '\r\n',
      'charInterval' : [ 17, 16 ],
      'lineIndex' : 0,
      'nl' : '',
      'line' : '',
    },
    {
      src,
      onEach,
      'withLine' : true,
      'eol' : '\r\n',
      'charInterval' : [ 11, 16 ],
      'lineIndex' : 1,
      'nl' : '\r\n',
      'line' : 'abcd',
    },
    {
      src,
      onEach,
      'withLine' : true,
      'eol' : '\r\n',
      'charInterval' : [ 6, 10 ],
      'lineIndex' : 2,
      'nl' : '\r\n',
      'line' : 'abc',
    },
    {
      src,
      onEach,
      'withLine' : true,
      'eol' : '\r\n',
      'charInterval' : [ 2, 5 ],
      'lineIndex' : 3,
      'nl' : '\r\n',
      'line' : 'ab',
    },
    {
      src,
      onEach,
      'withLine' : true,
      'eol' : '\r\n',
      'charInterval' : [ 0, 1 ],
      'lineIndex' : 4,
      'nl' : '\r\n',
      'line' : '',
    },
  ]
  test.identical( its, exp );
  test.identical( src.length, 17 );

  /* */

  function onEach( it )
  {
    it = _.props.extend( null, it );
    it.charInterval = it.charInterval.slice();
    it.eol = it.eol.slice();
    its.push( it );
  }

  /* */

  function clean()
  {
    its = [];
  }

  /* */

}

//

function eachRightMultiple( test )
{
  let its;

  /* */

  test.case = 'aa\\nbb\\r\\ncc';
  clean();
  var src = `aa\nbb\r\ncc`;
  _.str.lines.eachRight({ src, onEach, eol : _.str.lines.Eol.any });
  var exp =
  [
    {
      src,
      onEach,
      'withLine' : true,
      'eol' : [ '\r\n', '\n' ],
      'charInterval' : [ 7, 8 ],
      'lineIndex' : 0,
      'nl' : '',
      'line' : 'cc',
    },
    {
      src,
      onEach,
      'withLine' : true,
      'eol' : [ '\n' ],
      'charInterval' : [ 3, 6 ],
      'lineIndex' : 1,
      'nl' : '\r\n',
      'line' : 'bb',
    },
    {
      src,
      onEach,
      'withLine' : true,
      'eol' : [],
      'charInterval' : [ 0, 2 ],
      'lineIndex' : 2,
      'nl' : '\n',
      'line' : 'aa',
    },
  ]
  test.identical( its, exp );
  test.identical( src.length, 9 );

  /* */

  test.case = '\\nab\\nabc\\nabcd\\r\\n';
  clean();
  var src = `\nab\nabc\nabcd\r\n`;
  _.str.lines.eachRight({ src, onEach, eol : _.str.lines.Eol.any });
  var exp =
  [
    {
      src,
      onEach,
      'withLine' : true,
      'eol' : [ '\r\n', '\n', ],
      'charInterval' : [ 14, 13 ],
      'lineIndex' : 0,
      'nl' : '',
      'line' : '',
    },
    {
      src,
      onEach,
      'withLine' : true,
      'eol' : [ '\n', ],
      'charInterval' : [ 8, 13 ],
      'lineIndex' : 1,
      'nl' : '\r\n',
      'line' : 'abcd',
    },
    {
      src,
      onEach,
      'withLine' : true,
      'eol' : [ '\n', ],
      'charInterval' : [ 4, 7 ],
      'lineIndex' : 2,
      'nl' : '\n',
      'line' : 'abc',
    },
    {
      src,
      onEach,
      'withLine' : true,
      'eol' : [ '\n', ],
      'charInterval' : [ 1, 3 ],
      'lineIndex' : 3,
      'nl' : '\n',
      'line' : 'ab',
    },
    {
      src,
      onEach,
      'withLine' : true,
      'eol' : [],
      'charInterval' : [ 0, 0 ],
      'lineIndex' : 4,
      'nl' : '\n',
      'line' : '',
    }
  ]
  test.identical( its, exp );
  test.identical( src.length, 14 );

  /* */

  test.case = '\\r\\nab\\nabc\\nabcd\\n';
  clean();
  var src = `\r\nab\nabc\nabcd\n`;
  _.str.lines.eachRight({ src, onEach, eol : _.str.lines.Eol.any });
  var exp =
  [
    {
      src,
      onEach,
      'withLine' : true,
      'eol' : [ '\r\n', '\n', ],
      'charInterval' : [ 14, 13 ],
      'lineIndex' : 0,
      'nl' : '',
      'line' : '',
    },
    {
      src,
      onEach,
      'withLine' : true,
      'eol' : [ '\r\n', '\n', ],
      'charInterval' : [ 9, 13 ],
      'lineIndex' : 1,
      'nl' : '\n',
      'line' : 'abcd',
    },
    {
      src,
      onEach,
      'withLine' : true,
      'eol' : [ '\r\n', '\n', ],
      'charInterval' : [ 5, 8 ],
      'lineIndex' : 2,
      'nl' : '\n',
      'line' : 'abc',
    },
    {
      src,
      onEach,
      'withLine' : true,
      'eol' : [ '\r\n', '\n', ],
      'charInterval' : [ 2, 4 ],
      'lineIndex' : 3,
      'nl' : '\n',
      'line' : 'ab',
    },
    {
      src,
      onEach,
      'withLine' : true,
      'eol' : [],
      'charInterval' : [ 0, 1 ],
      'lineIndex' : 4,
      'nl' : '\r\n',
      'line' : '',
    }
  ]
  test.identical( its, exp );
  test.identical( src.length, 14 );

  /* */

  test.case = '\\r\\nab\\n\\rabc\\nabcd\\n';
  clean();
  var src = `\r\nab\n\rabc\nabcd\n`;
  _.str.lines.eachRight({ src, onEach, eol : _.str.lines.Eol.any });
  var exp =
  [
    {
      src,
      onEach,
      'withLine' : true,
      'eol' : [ '\r\n', '\n\r', '\n' ],
      'charInterval' : [ 15, 14 ],
      'lineIndex' : 0,
      'nl' : '',
      'line' : '',
    },
    {
      src,
      onEach,
      'withLine' : true,
      'eol' : [ '\r\n', '\n\r', '\n' ],
      'charInterval' : [ 10, 14 ],
      'lineIndex' : 1,
      'nl' : '\n',
      'line' : 'abcd',
    },
    {
      src,
      onEach,
      'withLine' : true,
      'eol' : [ '\r\n', '\n\r', '\n' ],
      'charInterval' : [ 6, 9 ],
      'lineIndex' : 2,
      'nl' : '\n',
      'line' : 'abc',
    },
    {
      src,
      onEach,
      'withLine' : true,
      'eol' : [ '\r\n', '\n' ],
      'charInterval' : [ 2, 5 ],
      'lineIndex' : 3,
      'nl' : '\n\r',
      'line' : 'ab',
    },
    {
      src,
      onEach,
      'withLine' : true,
      'eol' : [],
      'charInterval' : [ 0, 1 ],
      'lineIndex' : 4,
      'nl' : '\r\n',
      'line' : '',
    }
  ]
  test.identical( its, exp );
  test.identical( src.length, 15 );

  /* */

  function onEach( it )
  {
    it = _.props.extend( null, it );
    it.charInterval = it.charInterval.slice();
    it.eol = it.eol.slice();
    its.push( it );
    debugger;
  }

  /* */

  function clean()
  {
    its = [];
  }

  /* */

}

//

function eachRightMultipleInInterval( test )
{
  let its;

  /* */

  test.case = 'empty, -2, -2';
  clean();
  var src = ``;
  var interval = [ -2, -2 ];
  _.str.lines.eachRight({ src, onEach, interval, eol : _.str.lines.Eol.any });
  var exp = [];
  test.identical( its, exp );
  test.identical( src.length, 0 );

  /* */

  test.case = 'empty, -1, -1';
  clean();
  var src = ``;
  var interval = [ -1, -1 ];
  _.str.lines.eachRight({ src, onEach, interval, eol : _.str.lines.Eol.any });
  var exp = [];
  test.identical( its, exp );
  test.identical( src.length, 0 );

  /* */

  test.case = 'empty, 1, 1';
  clean();
  var src = ``;
  var interval = [ 1, 1 ];
  _.str.lines.eachRight({ src, onEach, interval, eol : _.str.lines.Eol.any });
  var exp = [];
  test.identical( its, exp );
  test.identical( src.length, 0 );

  /* */

  test.case = 'empty, 2, 2';
  clean();
  var src = ``;
  var interval = [ 2, 2 ];
  _.str.lines.eachRight({ src, onEach, interval, eol : _.str.lines.Eol.any });
  var exp = [];
  test.identical( its, exp );
  test.identical( src.length, 0 );

  /* */

  test.case = 'empty, 0, 0';
  clean();
  var src = ``;
  var interval = [ 0, 0 ];
  _.str.lines.eachRight({ src, onEach, interval, eol : _.str.lines.Eol.any });
  var exp =
  [
    {
      'src' : '',
      onEach,
      interval,
      'withLine' : true,
      'eol' : [],
      'lineIndex' : 0,
      'nl' : '',
      'charInterval' : [ 0, -1 ],
      'line' : '',
    }
  ]
  test.identical( its, exp );
  test.identical( src.length, 0 );

  /* */

  test.case = 'empty, -1, 0';
  clean();
  var src = ``;
  var interval = [ -1, 0 ];
  _.str.lines.eachRight({ src, onEach, interval, eol : _.str.lines.Eol.any });
  var exp =
  [
    {
      'src' : '',
      onEach,
      interval,
      'withLine' : true,
      'eol' : [],
      'lineIndex' : 0,
      'nl' : '',
      'charInterval' : [ 0, -1 ],
      'line' : '',
    }
  ]
  test.identical( its, exp );
  test.identical( src.length, 0 );

  /* */

  test.case = 'empty, 0, 1';
  clean();
  var src = ``;
  var interval = [ 0, 1 ];
  _.str.lines.eachRight({ src, onEach, interval, eol : _.str.lines.Eol.any });
  var exp =
  [
    {
      'src' : '',
      onEach,
      interval,
      'withLine' : true,
      'eol' : [],
      'lineIndex' : 0,
      'nl' : '',
      'charInterval' : [ 0, -1 ],
      'line' : '',
    }
  ]
  test.identical( its, exp );
  test.identical( src.length, 0 );

  /* */

  test.case = 'empty, -1, 1';
  clean();
  var src = ``;
  var interval = [ -1, 1 ];
  _.str.lines.eachRight({ src, onEach, interval, eol : _.str.lines.Eol.any });
  var exp =
  [
    {
      'src' : '',
      onEach,
      interval,
      'withLine' : true,
      'eol' : [],
      'lineIndex' : 0,
      'nl' : '',
      'charInterval' : [ 0, -1 ],
      'line' : '',
    }
  ]
  test.identical( its, exp );
  test.identical( src.length, 0 );

  /* */

  test.case = 'empty, 1, 1';
  clean();
  var src = ``;
  var interval = [ 1, 1 ];
  _.str.lines.eachRight({ src, onEach, interval, eol : _.str.lines.Eol.any });
  var exp = [];
  test.identical( its, exp );
  test.identical( src.length, 0 );

  /* */

  test.case = 'aa\\nbb, -Infinity, +Infinity';
  clean();
  var src = `aa\nbb`;
  var interval = [ -Infinity, +Infinity ];
  _.str.lines.eachRight({ src, onEach, interval, eol : _.str.lines.Eol.any });
  var exp =
  [
    {
      src,
      onEach,
      interval,
      'withLine' : true,
      'eol' : '\n',
      'charInterval' : [ 3, 4 ],
      'lineIndex' : 0,
      'nl' : '',
      'line' : 'bb',
    },
    {
      src,
      onEach,
      interval,
      'withLine' : true,
      'eol' : '\n',
      'charInterval' : [ 0, 2 ],
      'lineIndex' : 1,
      'nl' : '\n',
      'line' : 'aa',
    }
  ]
  test.identical( its, exp );
  test.identical( src.length, 5 );

  /* */

  test.case = 'aa\\nbb, 0, 1';
  clean();
  var src = `aa\nbb`;
  var interval = [ 0, 1 ];
  _.str.lines.eachRight({ src, onEach, interval, eol : _.str.lines.Eol.any });
  var exp =
  [
    {
      src,
      onEach,
      interval,
      'withLine' : true,
      'eol' : '\n',
      'charInterval' : [ 3, 4 ],
      'lineIndex' : 0,
      'nl' : '',
      'line' : 'bb',
    },
    {
      src,
      onEach,
      interval,
      'withLine' : true,
      'eol' : '\n',
      'charInterval' : [ 0, 2 ],
      'lineIndex' : 1,
      'nl' : '\n',
      'line' : 'aa',
    },
  ]
  test.identical( its, exp );
  test.identical( src.length, 5 );

  /* */

  test.case = 'aa\\nbb, 0, 0';
  clean();
  var src = `aa\nbb`;
  var interval = [ 0, 0 ];
  _.str.lines.eachRight({ src, onEach, interval, eol : _.str.lines.Eol.any });
  var exp =
  [
    {
      src,
      onEach,
      interval,
      'withLine' : true,
      'eol' : '\n',
      'charInterval' : [ 3, 4 ],
      'lineIndex' : 0,
      'nl' : '',
      'line' : 'bb',
    }
  ]
  test.identical( its, exp );
  test.identical( src.length, 5 );

  /* */

  test.case = 'aa\\nbb, 1, 1';
  clean();
  var src = `aa\nbb`;
  var interval = [ 1, 1 ];
  _.str.lines.eachRight({ src, onEach, interval, eol : _.str.lines.Eol.any });
  var exp =
  [
    {
      src,
      onEach,
      interval,
      'withLine' : true,
      'eol' : '\n',
      'charInterval' : [ 0, 2 ],
      'lineIndex' : 1,
      'nl' : '\n',
      'line' : 'aa',
    },
  ]
  test.identical( its, exp );
  test.identical( src.length, 5 );

  /* */

  test.case = 'aa\\nbb, -2, -2';
  clean();
  var src = `aa\nbb`;
  var interval = [ -2, -2 ];
  _.str.lines.eachRight({ src, onEach, interval, eol : _.str.lines.Eol.any });
  var exp = [];
  test.identical( its, exp );
  test.identical( src.length, 5 );

  /* */

  test.case = 'aa\\nbb, -1, -1';
  clean();
  var src = `aa\nbb`;
  var interval = [ -1, -1 ];
  _.str.lines.eachRight({ src, onEach, interval, eol : _.str.lines.Eol.any });
  var exp = [];
  test.identical( its, exp );
  test.identical( src.length, 5 );

  /* */

  test.case = 'aa\\nbb, 0, -1';
  clean();
  var src = `aa\nbb`;
  var interval = [ 0, -1 ];
  _.str.lines.eachRight({ src, onEach, interval, eol : _.str.lines.Eol.any });
  var exp = [];
  test.identical( its, exp );
  test.identical( src.length, 5 );

  /* */

  test.case = 'aa\\nbb, 2, 1';
  clean();
  var src = `aa\nbb`;
  var interval = [ 2, 1 ];
  _.str.lines.eachRight({ src, onEach, interval, eol : _.str.lines.Eol.any });
  var exp = [];
  test.identical( its, exp );
  test.identical( src.length, 5 );

  /* */

  test.case = 'aa\\nbb, 2, 2';
  clean();
  var src = `aa\nbb`;
  var interval = [ 2, 2 ];
  _.str.lines.eachRight({ src, onEach, interval, eol : _.str.lines.Eol.any });
  var exp = [];
  test.identical( its, exp );
  test.identical( src.length, 5 );

  /* */

  test.case = '0, 4';
  clean();
  var interval = [ 0, 4 ];
  var src = `\r\nab\n\rabc\nabcd\n`;
  _.str.lines.eachRight({ src, onEach, interval, eol : _.str.lines.Eol.any });
  var exp =
  [
    {
      src,
      onEach,
      interval,
      'withLine' : true,
      'eol' : [ '\r\n', '\n\r', '\n' ],
      'charInterval' : [ 15, 14 ],
      'lineIndex' : 0,
      'nl' : '',
      'line' : '',
    },
    {
      src,
      onEach,
      interval,
      'withLine' : true,
      'eol' : [ '\r\n', '\n\r', '\n' ],
      'charInterval' : [ 10, 14 ],
      'lineIndex' : 1,
      'nl' : '\n',
      'line' : 'abcd',
    },
    {
      src,
      onEach,
      interval,
      'withLine' : true,
      'eol' : [ '\r\n', '\n\r', '\n' ],
      'charInterval' : [ 6, 9 ],
      'lineIndex' : 2,
      'nl' : '\n',
      'line' : 'abc',
    },
    {
      src,
      onEach,
      interval,
      'withLine' : true,
      'eol' : [ '\r\n', '\n' ],
      'charInterval' : [ 2, 5 ],
      'lineIndex' : 3,
      'nl' : '\n\r',
      'line' : 'ab',
    },
    {
      src,
      onEach,
      interval,
      'withLine' : true,
      'eol' : [],
      'charInterval' : [ 0, 1 ],
      'lineIndex' : 4,
      'nl' : '\r\n',
      'line' : '',
    },
  ]
  test.identical( its, exp );
  test.identical( src.length, 15 );

  /* */

  test.case = '-1, 5';
  clean();
  var interval = [ -1, 5 ];
  var src = `\r\nab\n\rabc\nabcd\n`;
  _.str.lines.eachRight({ src, onEach, interval, eol : _.str.lines.Eol.any });
  var exp =
  [
    {
      src,
      onEach,
      interval,
      'withLine' : true,
      'eol' : [ '\r\n', '\n\r', '\n' ],
      'charInterval' : [ 15, 14 ],
      'lineIndex' : 0,
      'nl' : '',
      'line' : '',
    },
    {
      src,
      onEach,
      interval,
      'withLine' : true,
      'eol' : [ '\r\n', '\n\r', '\n' ],
      'charInterval' : [ 10, 14 ],
      'lineIndex' : 1,
      'nl' : '\n',
      'line' : 'abcd',
    },
    {
      src,
      onEach,
      interval,
      'withLine' : true,
      'eol' : [ '\r\n', '\n\r', '\n' ],
      'charInterval' : [ 6, 9 ],
      'lineIndex' : 2,
      'nl' : '\n',
      'line' : 'abc',
    },
    {
      src,
      onEach,
      interval,
      'withLine' : true,
      'eol' : [ '\r\n', '\n' ],
      'charInterval' : [ 2, 5 ],
      'lineIndex' : 3,
      'nl' : '\n\r',
      'line' : 'ab',
    },
    {
      src,
      onEach,
      interval,
      'withLine' : true,
      'eol' : [],
      'charInterval' : [ 0, 1 ],
      'lineIndex' : 4,
      'nl' : '\r\n',
      'line' : '',
    },
  ]
  test.identical( its, exp );
  test.identical( src.length, 15 );

  /* */

  test.case = '1, 3';
  clean();
  var interval = [ 1, 3 ];
  var src = `\r\nab\n\rabc\nabcd\n`;
  _.str.lines.eachRight({ src, onEach, interval, eol : _.str.lines.Eol.any });
  var exp =
  [
    {
      src,
      onEach,
      interval,
      'withLine' : true,
      'eol' : [ '\r\n', '\n\r', '\n' ],
      'charInterval' : [ 10, 14 ],
      'lineIndex' : 1,
      'nl' : '\n',
      'line' : 'abcd',
    },
    {
      src,
      onEach,
      interval,
      'withLine' : true,
      'eol' : [ '\r\n', '\n\r', '\n' ],
      'charInterval' : [ 6, 9 ],
      'lineIndex' : 2,
      'nl' : '\n',
      'line' : 'abc',
    },
    {
      src,
      onEach,
      interval,
      'withLine' : true,
      'eol' : [ '\r\n', '\n' ],
      'charInterval' : [ 2, 5 ],
      'lineIndex' : 3,
      'nl' : '\n\r',
      'line' : 'ab',
    },
  ]
  test.identical( its, exp );
  test.identical( src.length, 15 );

  /* */

  test.case = '2, 2';
  clean();
  var interval = [ 2, 2 ];
  var src = `\r\nab\n\rabc\nabcd\n`;
  _.str.lines.eachRight({ src, onEach, interval, eol : _.str.lines.Eol.any });
  var exp =
  [
    {
      src,
      onEach,
      interval,
      'withLine' : true,
      'eol' : [ '\r\n', '\n\r', '\n' ],
      'charInterval' : [ 6, 9 ],
      'lineIndex' : 2,
      'nl' : '\n',
      'line' : 'abc',
    },
  ]
  test.identical( its, exp );
  test.identical( src.length, 15 );

  /* */

  test.case = '0, 0';
  clean();
  var interval = [ -2, 0 ];
  var src = `\r\nab\n\rabc\nabcd\n`;
  _.str.lines.eachRight({ src, onEach, interval, eol : _.str.lines.Eol.any });
  var exp =
  [
    {
      src,
      onEach,
      interval,
      'withLine' : true,
      'eol' : [ '\r\n', '\n\r', '\n' ],
      'charInterval' : [ 15, 14 ],
      'lineIndex' : 0,
      'nl' : '',
      'line' : '',
    },
  ]
  test.identical( its, exp );
  test.identical( src.length, 15 );

  /* */

  test.case = '-2, 0';
  clean();
  var interval = [ -2, 0 ];
  var src = `\r\nab\n\rabc\nabcd\n`;
  _.str.lines.eachRight({ src, onEach, interval, eol : _.str.lines.Eol.any });
  var exp =
  [
    {
      src,
      onEach,
      interval,
      'withLine' : true,
      'eol' : [ '\r\n', '\n\r', '\n' ],
      'charInterval' : [ 15, 14 ],
      'lineIndex' : 0,
      'nl' : '',
      'line' : '',
    },
  ]
  test.identical( its, exp );
  test.identical( src.length, 15 );

  /* */

  test.case = '4, 4';
  clean();
  var interval = [ 4, 4 ];
  var src = `\r\nab\n\rabc\nabcd\n`;
  _.str.lines.eachRight({ src, onEach, interval, eol : _.str.lines.Eol.any });
  var exp =
  [
    {
      src,
      onEach,
      interval,
      'withLine' : true,
      'eol' : [],
      'charInterval' : [ 0, 1 ],
      'lineIndex' : 4,
      'nl' : '\r\n',
      'line' : '',
    }
  ]
  test.identical( its, exp );
  test.identical( src.length, 15 );

  /* */

  test.case = '4, 6';
  clean();
  var interval = [ 4, 6 ];
  var src = `\r\nab\n\rabc\nabcd\n`;
  _.str.lines.eachRight({ src, onEach, interval, eol : _.str.lines.Eol.any });
  var exp =
  [
    {
      src,
      onEach,
      interval,
      'withLine' : true,
      'eol' : [],
      'charInterval' : [ 0, 1 ],
      'lineIndex' : 4,
      'nl' : '\r\n',
      'line' : '',
    }
  ]
  test.identical( its, exp );
  test.identical( src.length, 15 );

  /* */

  test.case = '3, 2';
  clean();
  var interval = [ 3, 2 ];
  var src = `\r\nab\n\rabc\nabcd\n`;
  _.str.lines.eachRight({ src, onEach, interval, eol : _.str.lines.Eol.any });
  var exp = [];
  test.identical( its, exp );
  test.identical( src.length, 15 );

  /* */

  test.case = '2, 1';
  clean();
  var interval = [ 2, 1 ];
  var src = `\r\nab\n\rabc\nabcd\n`;
  _.str.lines.eachRight({ src, onEach, interval, eol : _.str.lines.Eol.any });
  var exp = [];
  test.identical( its, exp );
  test.identical( src.length, 15 );

  /* */

  test.case = '-1, -1';
  clean();
  var interval = [ -1, -1 ];
  var src = `\r\nab\n\rabc\nabcd\n`;
  _.str.lines.eachRight({ src, onEach, interval, eol : _.str.lines.Eol.any });
  var exp = [];
  test.identical( its, exp );
  test.identical( src.length, 15 );

  /* */

  test.case = '0, -1';
  clean();
  var interval = [ 0, -1 ];
  var src = `\r\nab\n\rabc\nabcd\n`;
  _.str.lines.eachRight({ src, onEach, interval, eol : _.str.lines.Eol.any });
  var exp = [];
  test.identical( its, exp );
  test.identical( src.length, 15 );

  /* */

  test.case = '-2, -1';
  clean();
  var interval = [ -2, -1 ];
  var src = `\r\nab\n\rabc\nabcd\n`;
  _.str.lines.eachRight({ src, onEach, interval, eol : _.str.lines.Eol.any });
  var exp = [];
  test.identical( its, exp );
  test.identical( src.length, 15 );

  /* */

  test.case = '5, 6';
  clean();
  var interval = [ 5, 6 ];
  var src = `\r\nab\n\rabc\nabcd\n`;
  _.str.lines.eachRight({ src, onEach, interval, eol : _.str.lines.Eol.any });
  var exp = [];
  test.identical( its, exp );
  test.identical( src.length, 15 );

  /* */

  function onEach( it )
  {
    it = _.props.extend( null, it );
    it.charInterval = it.charInterval.slice();
    it.eol = it.eol.slice();
    it.interval = it.interval.slice();
    its.push( it );
  }

  /* */

  function clean()
  {
    its = [];
  }

  /* */

}

//

function eachRightCallbacks( test )
{
  let its;

  act({ interval : 0, withLine : 0 });
  act({ interval : 0, withLine : 1 });
  act({ interval : 1, withLine : 0 });
  act({ interval : 1, withLine : 1 });

  /* */

  function act( env )
  {
    test.case = `${__.entity.exportStringSolo( env )}`;

    clean();
    var src = `aa\nbb\r\ncc`;
    var o = { src, onEach, withLine : env.withLine, eol : _.str.lines.Eol.any };
    if( env.interval )
    o.interval = [ 0, 2 ];

    var got = _.str.lines.eachRight( o );
    test.true( o === got );
    var exp =
    {
      src,
      onEach,
      'withLine' : env.withLine,
      'eol' : [],
      'charInterval' : [ 0, -1 ],
      'lineIndex' : 3,
      'nl' : '\n',
    }
    if( env.withLine )
    exp.line = '';
    if( env.interval )
    {
      exp.interval = [ 0, 2 ];
      exp.charInterval = [ 0, 2 ];
      exp.lineIndex = 2;
      if( env.withLine )
      exp.line = 'aa';
    }
    test.identical( o, exp );

    var exp =
    [
      {
        src,
        onEach,
        'withLine' : env.withLine,
        'eol' : [ '\r\n', '\n' ],
        'charInterval' : [ 7, 8 ],
        'lineIndex' : 0,
        'nl' : '',
      },
      {
        src,
        onEach,
        'withLine' : env.withLine,
        'eol' : [ '\n' ],
        'charInterval' : [ 3, 6 ],
        'lineIndex' : 1,
        'nl' : '\r\n',
      },
      {
        src,
        onEach,
        'withLine' : env.withLine,
        'eol' : [],
        'charInterval' : [ 0, 2 ],
        'lineIndex' : 2,
        'nl' : '\n',
      },
    ]
    if( env.withLine )
    {
      exp[ 0 ].line = 'cc';
      exp[ 1 ].line = 'bb';
      exp[ 2 ].line = 'aa';
    }
    if( env.interval )
    {
      exp[ 0 ].interval = [ 0, 2 ];
      exp[ 1 ].interval = [ 0, 2 ];
      exp[ 2 ].interval = [ 0, 2 ];
    }
    test.identical( its, exp );
    test.identical( src.length, 9 );

  }

  /* */

  function onEach( it )
  {
    it = _.props.extend( null, it );
    it.charInterval = it.charInterval.slice();
    it.eol = it.eol.slice();
    if( it.interval )
    it.interval = it.interval.slice();
    its.push( it );
  }

  /* */

  function clean()
  {
    its = [];
  }

  /* */

}

//

function atRightSingle( test )
{

  /* */

  test.case = 'empty';
  var src = ``;
  var exp  =
  {
    'src' : src,
    'lineIndex' : -2,
    'charInterval' : [ 0, -1 ],
  }
  var got = _.str.lines.atRight( src, -2 );
  test.identical( got, exp );
  var exp  =
  {
    'src' : src,
    'lineIndex' : -1,
    'charInterval' : [ 0, -1 ],
  }
  var got = _.str.lines.atRight( src, -1 );
  test.identical( got, exp );
  var exp  =
  {
    'src' : src,
    'lineIndex' : 0,
    'charInterval' : [ 0, -1 ],
    'line' : '',
    'nl' : '',
  }
  var got = _.str.lines.atRight( src, 0 );
  test.identical( got, exp );
  var exp  =
  {
    'src' : src,
    'lineIndex' : 1,
    'charInterval' : [ 0, -1 ],
  }
  var got = _.str.lines.atRight( src, 1 );
  test.identical( got, exp );
  var exp  =
  {
    'src' : src,
    'lineIndex' : 2,
    'charInterval' : [ 0, -1 ],
  }
  var got = _.str.lines.atRight( src, 2 );
  test.identical( got, exp );
  test.identical( src.length, 0 );

  /* */

  test.case = '\\n';
  var src = `\n`;
  var exp  =
  {
    'src' : src,
    'lineIndex' : -2,
    'charInterval' : [ 1, 0 ],
  }
  var got = _.str.lines.atRight( src, -2 );
  test.identical( got, exp );

  var exp  =
  {
    'src' : src,
    'lineIndex' : -1,
    'charInterval' : [ 1, 0 ],
  }
  var got = _.str.lines.atRight( src, -1 );
  test.identical( got, exp );
  var exp  =
  {
    'src' : src,
    'lineIndex' : 0,
    'charInterval' : [ 1, 0 ],
    'line' : '',
    'nl' : '',
  }
  var got = _.str.lines.atRight( src, 0 );
  test.identical( got, exp );
  var exp  =
  {
    'src' : src,
    'lineIndex' : 1,
    'charInterval' : [ 0, 0 ],
    'line' : '',
    'nl' : '\n',
  }
  var got = _.str.lines.atRight( src, 1 );
  test.identical( got, exp );
  var exp  =
  {
    'src' : src,
    'lineIndex' : 2,
    'charInterval' : [ 0, -1 ],
  }
  var got = _.str.lines.atRight( src, 2 );
  test.identical( got, exp );
  var exp  =
  {
    'src' : src,
    'lineIndex' : 3,
    'charInterval' : [ 0, -1 ],
  }
  var got = _.str.lines.atRight( src, 3 );
  test.identical( got, exp );
  test.identical( src.length, 1 );

  /* */

  test.case = 'a\\nb';
  var src = `a\nb`;
  var exp  =
  {
    'src' : src,
    'lineIndex' : -2,
    'charInterval' : [ 3, 2 ],
  }
  var got = _.str.lines.atRight( src, -2 );
  test.identical( got, exp );
  var exp  =
  {
    'src' : src,
    'lineIndex' : -1,
    'charInterval' : [ 3, 2 ],
  }
  var got = _.str.lines.atRight( src, -1 );
  test.identical( got, exp );
  var exp  =
  {
    'src' : src,
    'lineIndex' : 0,
    'charInterval' : [ 2, 2 ],
    'line' : 'b',
    'nl' : '',
  }
  var got = _.str.lines.atRight( src, 0 );
  test.identical( got, exp );
  var exp  =
  {
    'src' : src,
    'lineIndex' : 1,
    'charInterval' : [ 0, 1 ],
    'line' : 'a',
    'nl' : '\n',
  }
  var got = _.str.lines.atRight( src, 1 );
  test.identical( got, exp );
  var exp  =
  {
    'src' : src,
    'lineIndex' : 2,
    'charInterval' : [ 0, -1 ],
  }
  var got = _.str.lines.atRight( src, 2 );
  test.identical( got, exp );
  var exp  =
  {
    'src' : src,
    'lineIndex' : 3,
    'charInterval' : [ 3, 2 ],
  }
  test.identical( src.length, 3 );

  /* */

  test.case = 'ab\\nabc\\nabcd';
  var src = `ab\nabc\nabcd`;
  var exp  =
  {
    'src' : src,
    'lineIndex' : -2,
    'charInterval' : [ 11, 10 ],
  }
  var got = _.str.lines.atRight( src, -2 );
  test.identical( got, exp );
  var exp  =
  {
    'src' : src,
    'lineIndex' : -1,
    'charInterval' : [ 11, 10 ],
  }
  var got = _.str.lines.atRight( src, -1 );
  test.identical( got, exp );
  var exp  =
  {
    'src' : src,
    'lineIndex' : 0,
    'charInterval' : [ 7, 10 ],
    'line' : 'abcd',
    'nl' : '',
  }
  var got = _.str.lines.atRight( src, 0 );
  test.identical( got, exp );
  var exp  =
  {
    'src' : src,
    'lineIndex' : 1,
    'charInterval' : [ 3, 6 ],
    'line' : 'abc',
    'nl' : '\n',
  }
  var got = _.str.lines.atRight( src, 1 );
  test.identical( got, exp );
  var exp  =
  {
    'src' : src,
    'lineIndex' : 2,
    'charInterval' : [ 0, 2 ],
    'line' : 'ab',
    'nl' : '\n',
  }
  var got = _.str.lines.atRight( src, 2 );
  test.identical( got, exp );
  var exp  =
  {
    'src' : src,
    'lineIndex' : 3,
    'charInterval' : [ 0, -1 ],
  }
  var got = _.str.lines.atRight( src, 3 );
  test.identical( got, exp );
  var exp  =
  {
    'src' : src,
    'lineIndex' : 4,
    'charInterval' : [ 0, -1 ],
  }
  var got = _.str.lines.atRight( src, 4 );
  test.identical( got, exp );
  test.identical( src.length, 11 );

  /* */

  test.case = '\\nab\\nabc\\nabcd\\n';
  var src = `\nab\nabc\nabcd\n`;
  var exp  =
  {
    'src' : src,
    'lineIndex' : -2,
    'charInterval' : [ 13, 12 ],
  }
  var got = _.str.lines.atRight( src, -2 );
  test.identical( got, exp );
  var exp  =
  {
    'src' : src,
    'lineIndex' : -1,
    'charInterval' : [ 13, 12 ],
  }
  var got = _.str.lines.atRight( src, -1 );
  test.identical( got, exp );
  var exp  =
  {
    'src' : src,
    'lineIndex' : 0,
    'charInterval' : [ 13, 12 ],
    'line' : '',
    'nl' : '',
  }
  var got = _.str.lines.atRight( src, 0 );
  test.identical( got, exp );
  var exp  =
  {
    'src' : src,
    'lineIndex' : 1,
    'charInterval' : [ 8, 12 ],
    'line' : 'abcd',
    'nl' : '\n',
  }
  var got = _.str.lines.atRight( src, 1 );
  test.identical( got, exp );
  var exp  =
  {
    'src' : src,
    'lineIndex' : 2,
    'charInterval' : [ 4, 7 ],
    'line' : 'abc',
    'nl' : '\n',
  }
  var got = _.str.lines.atRight( src, 2 );
  test.identical( got, exp );
  var exp  =
  {
    'src' : src,
    'lineIndex' : 3,
    'charInterval' : [ 1, 3 ],
    'line' : 'ab',
    'nl' : '\n',
  }
  var got = _.str.lines.atRight( src, 3 );
  test.identical( got, exp );
  var exp  =
  {
    'src' : src,
    'lineIndex' : 4,
    'charInterval' : [ 0, 0 ],
    'line' : '',
    'nl' : '\n',
  }
  var got = _.str.lines.atRight( src, 4 );
  test.identical( got, exp );
  var exp  =
  {
    'src' : src,
    'lineIndex' : 5,
    'charInterval' : [ 0, -1 ],
  }
  var got = _.str.lines.atRight( src, 5 );
  test.identical( got, exp );
  test.identical( src.length, 13 );

  /* */

}

//

function atRightCustomToken( test )
{

  /* */

  test.case = 'a\\nb';
  var src = `a\nb`;
  var exp  =
  {
    'src' : src,
    'lineIndex' : -2,
    'charInterval' : [ 3, 2 ],
  }
  var got = _.str.lines.atRight( src, -2, '\n' );
  test.identical( got, exp );
  var exp  =
  {
    'src' : src,
    'lineIndex' : -1,
    'charInterval' : [ 3, 2 ],
  }
  var got = _.str.lines.atRight( src, -1, '\n' );
  test.identical( got, exp );
  var exp  =
  {
    'src' : src,
    'lineIndex' : 0,
    'charInterval' : [ 2, 2 ],
    'line' : 'b',
    'nl' : '',
  }
  var got = _.str.lines.atRight( src, 0, '\n' );
  test.identical( got, exp );
  var exp  =
  {
    'src' : src,
    'lineIndex' : 1,
    'charInterval' : [ 0, 1 ],
    'line' : 'a',
    'nl' : '\n',
  }
  var got = _.str.lines.atRight( src, 1, '\n' );
  test.identical( got, exp );
  var exp  =
  {
    'src' : src,
    'lineIndex' : 2,
    'charInterval' : [ 0, -1 ],
  }
  var got = _.str.lines.atRight( src, 2, '\n' );
  test.identical( got, exp );
  var exp  =
  {
    'src' : src,
    'lineIndex' : 3,
    'charInterval' : [ 3, 3 ],
  }
  test.identical( src.length, 3 );

  /* */

  test.case = 'a\\rb - custom token';
  var src = `a\rb`;
  var exp  =
  {
    'src' : src,
    'lineIndex' : -2,
    'charInterval' : [ 3, 2 ],
  }
  var got = _.str.lines.atRight( src, -2, '\r' );
  test.identical( got, exp );
  var exp  =
  {
    'src' : src,
    'lineIndex' : -1,
    'charInterval' : [ 3, 2 ],
  }
  var got = _.str.lines.atRight( src, -1, '\r' );
  test.identical( got, exp );
  var exp  =
  {
    'src' : src,
    'lineIndex' : 0,
    'charInterval' : [ 2, 2 ],
    'line' : 'b',
    'nl' : '',
  }
  var got = _.str.lines.atRight( src, 0, '\r' );
  test.identical( got, exp );
  var exp  =
  {
    'src' : src,
    'lineIndex' : 1,
    'charInterval' : [ 0, 1 ],
    'line' : 'a',
    'nl' : '\r',
  }
  var got = _.str.lines.atRight( src, 1, '\r' );
  test.identical( got, exp );
  var exp  =
  {
    'src' : src,
    'lineIndex' : 2,
    'charInterval' : [ 0, -1 ],
  }
  var got = _.str.lines.atRight( src, 2, '\r' );
  test.identical( got, exp );
  var exp  =
  {
    'src' : src,
    'lineIndex' : 3,
    'charInterval' : [ 0, -1 ],
  }
  var got = _.str.lines.atRight( src, 3, '\r' );
  test.identical( got, exp );
  test.identical( src.length, 3 );

  /* */

}

//

function atRightMultiple( test )
{

  /* */

  test.case = '\\nab\\nabc\\nabcd\\r\\n';
  var src = `\nab\nabc\nabcd\r\n`;
  var exp  =
  {
    'src' : src,
    'lineIndex' : -1,
    'charInterval' : [ 14, 13 ],
  }
  var got = _.str.lines.atRight( src, -1, _.str.lines.Eol.any );
  test.identical( got, exp );
  var exp  =
  {
    'src' : src,
    'lineIndex' : 0,
    'charInterval' : [ 14, 13 ],
    'line' : '',
    'nl' : '',
  }
  var got = _.str.lines.atRight( src, 0, _.str.lines.Eol.any );
  test.identical( got, exp );
  var exp  =
  {
    'src' : src,
    'lineIndex' : 1,
    'charInterval' : [ 8, 13 ],
    'line' : 'abcd',
    'nl' : '\r\n',
  }
  var got = _.str.lines.atRight( src, 1, _.str.lines.Eol.any );
  test.identical( got, exp );
  var exp  =
  {
    'src' : src,
    'lineIndex' : 2,
    'charInterval' : [ 4, 7 ],
    'line' : 'abc',
    'nl' : '\n',
  }
  var got = _.str.lines.atRight( src, 2, _.str.lines.Eol.any );
  test.identical( got, exp );
  var exp  =
  {
    'src' : src,
    'lineIndex' : 3,
    'charInterval' : [ 1, 3 ],
    'line' : 'ab',
    'nl' : '\n',
  }
  var got = _.str.lines.atRight( src, 3, _.str.lines.Eol.any );
  test.identical( got, exp );
  var exp  =
  {
    'src' : src,
    'lineIndex' : 4,
    'charInterval' : [ 0, 0 ],
    'line' : '',
    'nl' : '\n',
  }
  var got = _.str.lines.atRight( src, 4, _.str.lines.Eol.any );
  test.identical( got, exp );
  var exp  =
  {
    'src' : src,
    'lineIndex' : 5,
    'charInterval' : [ 0, -1 ],
  }
  var got = _.str.lines.atRight( src, 5, _.str.lines.Eol.any );
  test.identical( got, exp );
  test.identical( src.length, 14 );

  /* */

  test.case = '\\r\\nab\\nabc\\nabcd\\n';
  var src = `\r\nab\nabc\nabcd\n`;
  var exp  =
  {
    'src' : src,
    'lineIndex' : -1,
    'charInterval' : [ 14, 13 ],
  }
  var got = _.str.lines.atRight( src, -1, _.str.lines.Eol.any );
  test.identical( got, exp );
  var exp  =
  {
    'src' : src,
    'lineIndex' : 0,
    'charInterval' : [ 14, 13 ],
    'line' : '',
    'nl' : '',
  }
  var got = _.str.lines.atRight( src, 0, _.str.lines.Eol.any );
  test.identical( got, exp );
  var exp  =
  {
    'src' : src,
    'lineIndex' : 1,
    'charInterval' : [ 9, 13 ],
    'line' : 'abcd',
    'nl' : '\n',
  }
  var got = _.str.lines.atRight( src, 1, _.str.lines.Eol.any );
  test.identical( got, exp );
  var exp  =
  {
    'src' : src,
    'lineIndex' : 2,
    'charInterval' : [ 5, 8 ],
    'line' : 'abc',
    'nl' : '\n',
  }
  var got = _.str.lines.atRight( src, 2, _.str.lines.Eol.any );
  test.identical( got, exp );
  var exp  =
  {
    'src' : src,
    'lineIndex' : 3,
    'charInterval' : [ 2, 4 ],
    'line' : 'ab',
    'nl' : '\n',
  }
  var got = _.str.lines.atRight( src, 3, _.str.lines.Eol.any );
  test.identical( got, exp );
  var exp  =
  {
    'src' : src,
    'lineIndex' : 4,
    'charInterval' : [ 0, 1 ],
    'line' : '',
    'nl' : '\r\n',
  }
  var got = _.str.lines.atRight( src, 4, _.str.lines.Eol.any );
  test.identical( got, exp );
  var exp  =
  {
    'src' : src,
    'lineIndex' : 5,
    'charInterval' : [ 0, -1 ],
  }
  var got = _.str.lines.atRight( src, 5, _.str.lines.Eol.any );
  test.identical( got, exp );
  test.identical( src.length, 14 );

  /* */

  test.case = '\\r\\nab\\n\\rabc\\nabcd\\n';
  var src = `\r\nab\n\rabc\nabcd\n`;
  var exp  =
  {
    'src' : src,
    'lineIndex' : -1,
    'charInterval' : [ 15, 14 ],
  }
  var got = _.str.lines.atRight( src, -1, _.str.lines.Eol.any );
  test.identical( got, exp );
  var exp  =
  {
    'src' : src,
    'lineIndex' : 0,
    'charInterval' : [ 15, 14 ],
    'line' : '',
    'nl' : '',
  }
  var got = _.str.lines.atRight( src, 0, _.str.lines.Eol.any );
  test.identical( got, exp );
  var exp  =
  {
    'src' : src,
    'lineIndex' : 1,
    'charInterval' : [ 10, 14 ],
    'line' : 'abcd',
    'nl' : '\n',
  }
  var got = _.str.lines.atRight( src, 1, _.str.lines.Eol.any );
  test.identical( got, exp );
  var exp  =
  {
    'src' : src,
    'lineIndex' : 2,
    'charInterval' : [ 6, 9 ],
    'line' : 'abc',
    'nl' : '\n',
  }
  var got = _.str.lines.atRight( src, 2, _.str.lines.Eol.any );
  test.identical( got, exp );
  var exp  =
  {
    'src' : src,
    'lineIndex' : 3,
    'charInterval' : [ 2, 5 ],
    'line' : 'ab',
    'nl' : '\n\r',
  }
  var got = _.str.lines.atRight( src, 3, _.str.lines.Eol.any );
  test.identical( got, exp );
  var exp  =
  {
    'src' : src,
    'lineIndex' : 4,
    'charInterval' : [ 0, 1 ],
    'line' : '',
    'nl' : '\r\n',
  }
  var got = _.str.lines.atRight( src, 4, _.str.lines.Eol.any );
  test.identical( got, exp );
  var exp  =
  {
    'src' : src,
    'lineIndex' : 5,
    'charInterval' : [ 0, -1 ],
  }
  var got = _.str.lines.atRight( src, 5, _.str.lines.Eol.any );
  test.identical( got, exp );
  test.identical( src.length, 15 );

  /* */

}

// --
//
// --

const Proto =
{

  name : 'Tools.str.lines.l0.l1',
  silencing : 1,

  tests :
  {

    assumptions,

    eachLeftSingle,
    eachLeftCustomToken,
    eachLeftMultiple,
    eachLeftMultipleInInterval,
    eachLeftCallbacks,
    atLeftSingle,
    atLeftCustomToken,
    atLeftMultiple,

    eachRightSingle,
    eachRightCustomToken,
    eachRightMultiple,
    eachRightMultipleInInterval,
    eachRightCallbacks,
    atRightSingle,
    atRightCustomToken,
    atRightMultiple,

  }

}

const Self = wTestSuite( Proto );
if( typeof module !== 'undefined' && !module.parent )
wTester.test( Self.name );

})();
