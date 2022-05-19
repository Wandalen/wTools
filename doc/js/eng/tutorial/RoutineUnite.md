# Routine routine.unite

Automatically combining of the data preparation routine and routine for data processing.

The routine `routine.unite` returns routine that is combined from two routines. One of them is routine `head` and the other is routine `body`.

The routine `head` is intended to prepare the [map of options](../concept/RoutineInput.md) that is passed to the routine `body`. The routine `body` performs processing of data, it receives map of options. The access to any option in the map of options performs by a name. So, the map of options is more convenient to use than an array of arguments because the access to the option in the array of arguments is carried out by index number. So if the array of arguments is used, all options must go consistently and the default values changing becomes more complicated.

This separation of functions is convenient for several reasons:

- each of the routines provides only one functionality;
- in the routine `head`, a map of options is being prepared, in particular, the routine `head` checks for extra and unnecessary options;
- the routine `head` can be reused in similar routines;
- it is possible to create new routines based on the routine `body` by changing default options;
- the behavior of the routine `body` can depend heavily on the map of options prepared by routine `head`.

### Create a routine using `routine.unite`

<details>
  <summary><u>Module structure</u></summary>

```
routine.unite
        ├── Name.js
        └── package.json
```

</details>

Create the file structure above to test the routine `routine.unite`.

<details>
  <summary><u>The routines <code>name_head</code> and <code>name_body</code> in file <code>Name.js</code></u></summary>

```js
function name_head( routine, args )
{
  let o = args[ 0 ];
  if( _.strIs( o ) )
  o = { path : o };

  _.routine.options( routine, o );
  _.assert( o && _.strIs( o.path ), 'Expects strings {-o.path-}' );
  _.assert( args.length === 1 );
  _.assert( arguments.length === 2 );

  return o;
}

function name_body( o )
{
  let i = o.path.lastIndexOf( '/' );
  if( i !== -1 )
  o.path = o.path.substr( i+1 );

  if( !o.full )
  {
    let i = o.path.lastIndexOf( '.' );
    if( i !== -1 ) o.path = o.path.substr( 0, i );
  }

  return o.path;
}

name_body.defaults =
{
  path : null,
  full : 0,
}
```

</details>

See the code of routines `name_head` and `name_body`.

As already mentioned, the routine `head` prepares a map of options for the routine `body`. One of the arguments that are passed to it is `routine` - the current routine, its setting. The second argument is `args` - an array of arguments passed to the routine. The `name_head` routine accepts only one argument, which is immediately placed in the map of options with the key `path`

```js
let o = args[ 0 ];
if( _.strIs( o ) )
o = { path : o };
```

Each routine constructed using `routine.unite` should have default options that are placed in the routine `body`. The routine `routineOptions` uses to create a map of options that contains options created by routine `name_head` and options settled in routine `routine_body`. The routine `routineOptions` checks the map of options created from an array of arguments `args` and, if necessary, adds options from the default settings `name_body.defaults`.

Another important component of the routine `head` is an assertion. Assertions check the input data so that the formed map of options contains valid values. For example, this assertion

```js
_.assert( args.length === 1 );
```

checks that the passed array of arguments `args` contains only one element. If this value is different from `1`, the routine will complete its execution by throwing an error.

The routine `name_body` consists of two parts. The first is routine itself, and the second is its settings. The routine `name_body` accepts the map of options and returns the filename from the passed path. For the correct output, the path must be formed in the format of `posix`-systems, that is, with the slash `/` (the path conversion is not considered here). The routine allocates a full name from the last `/` character to the end of the line using the `substr` method. In addition, the body of the routine contains the `full` option. If this option is not settled, then the routine separates filename excluded the last extension.

The default settings are defined by the map of option `name_body.defaults`, which contains all options that the routine accepts. So, in the routine `name_body`, this is `path` and `full`.

The routine `routine.unite` can combine the` name_head` and `name_body` routines into one that will accept the path and return the filename.

<details>
  <summary><u>Code of file <code>Name.js</code></u></summary>

```js
let _ = require( 'wTools' );

//

function name_head( routine, args )
{
  let o = args[ 0 ];
  if( _.strIs( o ) )
  o = { path : o };

  _.routine.options( routine, o );
  _.assert( o && _.strIs( o.path ), 'Expects strings {-o.path-}' );
  _.assert( args.length === 1 );
  _.assert( arguments.length === 2 );

  return o;
}

function name_body( o )
{
  let i = o.path.lastIndexOf( '/' );
  if( i !== -1 )
  o.path = o.path.substr( i+1 );

  if( !o.full )
  {
    let i = o.path.lastIndexOf( '.' );
    if( i !== -1 ) o.path = o.path.substr( 0, i );
  }

  return o.path;
}

name_body.defaults =
{
  path : null,
  full : 0,
}

let name = _.routine.unite( name_head, name_body );

let nameFull = _.routine.unite( name_head, name_body );
nameFull.defaults.full = 1;

console.log( name( './a/b/c/File.js' ) );
console.log( nameFull( './a/b/c/File.js' ) );
```

</details>

Enter the code above in the file `Name.js`.

To use the routine `routine.unite`, you need to install the module `Tools`. Copy the next code into the `package.json` file.

<details>
    <summary><u>Code of file <code>package.json</code></u></summary>

```json
{
  "dependencies": {
    "wTools": ""
  }
}
```

</details>

Use the `npm install` command to install dependencies. After installing, the module is ready for operation.

You will find lines with a definition of routines `name` and `nameFull` at the end of the code in file `Name.js`. They specified as ordinary variables assigned to the result of the `routine.unite` execution:

```js
let name = _.routine.unite( name_head, name_body );

let nameFull = _.routine.unite( name_head, name_body );
```

The first argument of routine `routine.unite` is the routine `head` and the other - `body`.

The routine `name` uses the default settings that are settled in the routine `name_body`, so it does not override these options. The routine `nameFull` should return the full filename, so it needs to set the `full` option:

```js
nameFull.defaults.full = 1;
```

The lines with console output, for example, `console.log( name( './a/b/c/File.js' ) )`, show that to call the routines created by the routine `routine.unite` you need to specify the name of the variable and pass arguments in brackets.

Run `Name.js` in the` NodeJS` interpreter, enter command `node Name.js`. Compare the output with the following:

<details>
  <summary><u>Command output <code>node Name.js</code></u></summary>

```
$ node Name.js
File
File.js
```

</details>

The routines `name` and `nameFull` have performed operations with the string `./A/b/c/File.js` and separate the filename from path. Each of these routines has its own implementation from the components `head` and `body`. That is, routines are independent of each other, so they are more reliable.

### The name and the call of the routine created using `routine.unite`

The previous example shows that the call of routine created using `routine.unite` is carried out by the name of the variable. Nevertheless, the name of this routine forms in a different way. By default, the name of the created routine is determined by the name of the second argument, that is, the argument `body`. If the name of this routine contains the `_body` postfix, the postfix discards.

Replace the last two lines of the file `Name.js` with the next:

```js
console.log( nameFull.name );
```

After saving the file, repeat the command `node Name.js`.

<details>
  <summary><u>Command output <code>node Name.js</code></u></summary>

```
$ node Name.js
name
```

</details>

The console displays a string created by dropping a postfix from `name_body`. If the matching of the routine name and its actual call is important, then you can pass the third argument to the routine `routine.unite` - the name of routine.

Replace line

```js
let nameFull = _.routine.unite( name_head, name_body);
```

by

```js
let nameFull = _.routine.unite({ head : name_head, body : name_body, name : 'nameFull' });
```

After saving the file, run the execution of file `Name.js`. Compare the console output with the next

<details>
  <summary><u>Console output <code>node Name.js</code></u></summary>

```
$ node Name.js
nameFull
```

</details>

Now the routine name and the variable name, which marks routine, are identical. This will avoid errors in the code that uses the name of the routine.

### Summary

- Routine `routine.unite` returns a routine that consists of two components. It is routine `head` that prepares the map of options, and it is routine `body` that processes data in the map of option.
- The separation of functions increases the reliability and safety of the use of created routine.
- Routine `routine.unite` facilitates the process of expanding functionality.
- Routine `routine.unite` allows to reuse code many times.
- The name of the created routine is formed by the name of the routine from the argument `body`, or it can be passed to the third argument in the routine` routine.unite`.

[Back to content](../README.md#Tutorials)
