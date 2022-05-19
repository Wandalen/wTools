# Type <code>BufferNode</code>

Non-standard implementation of untyped buffer with built-in access to buffer data.

[Type `BufferNode`](https://nodejs.org/dist/latest-v12.x/docs/api/buffer.html) presented by one class `BufferNode` ( `Buffer` ). Class `BufferNode` exists only in the `NodeJS` interpreter environment. Since the type `BufferNode` is designed for the `NodeJS` interpreter environment and is not standard, it has limited application.

### Examples

The `from`, `alloc`, `allocUnsafe` methods are used to create the buffer instance. Creating an instance of the class `BufferNode` using `new` [not recommended](https://nodejs.org/dist/latest-v12.x/docs/api/buffer.html).

```js
var buffer = BufferNode.alloc( 5 );
console.log( buffer );
// log <Buffer 00 00 00 00 00>
```
The method `alloc` allocates an initialized section of memory with the specified length.

```js
var buffer = BufferNode.alloc( 5, 1 );
console.log( buffer );
// log <Buffer 01 01 01 01 01>
```

The method `alloc` can fills buffer. For this purpose, the necessary value is passed to the second argument.

```js
var buffer = BufferNode.allocUnsafe( 5 );
console.log( buffer );
// log <Buffer 20 29 0a 20 20>
```

The method `allocUnsafe` also allocates section of memory but does not clear it from previous data.

```js
var buffer = BufferNode.from( [ 1, 2, 3 ] );
console.log( buffer );
// returns <Buffer 01 02 03>
```

The method `from` can accept a value of type [`Long`](TypeIndexed.md). The output buffer will have a length equal to the length of passed `Long`.

```js
var buffer = BufferNode.from( 'hello, world' );
console.log( buffer );
// returns <Buffer 68 65 6c 6c 6f 2c 20 77 6f 72 6c 64>
```

If the string is passed to the `from` method, it converts to the buffer. Each character of the string converts to a corresponding buffer byte with the `ASCII` encoding.

```js
var buffer = BufferNode.from( 'hello, world', 'base64' );
console.log( buffer );
//returns <Buffer 85 e9 65 a3 0a 2b 95>
```

The encoding can be specified in the second argument. The example above uses `base64` encoding, the length of the output buffer `buffer` is reduced to 7 bytes from 12 with `ASCII` encoding.

### Summary

- Type `BufferNode` implements dynamic typing.
- An instance of type `BufferNode` has data.
- Type `BufferNode` combines both data container and view ( representation ) in one class.
- Type `BufferNode` is non-standard.
- Type `BufferNode` has disadvantages.

[Back to content](../README.md#Concepts)
