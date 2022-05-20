# Type <code>BufferTyped</code>

Standard typed buffers.

[Type `BufferTyped`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray) represents binary data of `BufferRaw` in form of numerical primitives such as `Int8`, `Ui32`, `Float64` and other. The typed buffers do not contain real data but are a representation of the real data in `BufferRaw`. Typed buffers implement strategies for accessing to `BufferRaw` data.

The list of typed buffers:

- `I8x` ( `Int8Array` );
- `U8x` ( `Ui8Array` );
- `U8ClampedX` ( `Ui8ClampedArray` );
- `I16x` ( `Int16Array` );
- `U16x` ( `Ui16Array` );
- `I32x`, similar to the name `Ix` ( `Int32Array` );
- `U32x`, similar to the name `Ux` ( `Ui32Array` );
- `F32x`, similar to the name `Fx` ( `Float32Array` );
- `F64x` ( `Float64Array` );
- `I64x` ( `BigInt64Array` );
- `U64x` ( `BigUint64Array` ).

The type `BufferTyped`, unlike [`BufferView`](./ BufferView.md), types elements statically.

### Examples

```js
var u8 = new U8x();
console.log( u8.length ) // log 0
console.log( u8.byteLength ) // log 0
```

To create an instance of a typed buffer call the constructor of the appropriate class and pass an argument to construct the buffer. If the constructor is called without arguments, then an empty typed buffer will be created.
The `u8` variable contains an instance of the buffer` U8x`. The constructor called without arguments, and its length is zero.

```js
var i8 = new I8x( 20 );
console.log( i8.length ); // log 20
console.log( i8.byteLength ); // log 20
```

If a buffer length is passed, a typed buffer is created with a specified number of elements. The  type `Int8` is a single-byte signed integer number, so the size corresponds to the length of the buffer.

```js
var i16 = new I16x( [ 1, 2, 10 ] );
console.log( i16.length ); // log 3
console.log( i16.byteLength ); // log 6
```

If an array is passed, the constructor creates a typed buffer equal to the length of the passed buffer. Each element of the array converts to the type and size of the constructor. For the `i16` buffer, each element of the input array is converted to a two-byte signed number.

```js
var raw = new BufferRaw( 32 );

var float32 = new F32x( raw );
console.log( float32.length ); // log 8
console.log( float32.byteLength ); // log 32

var ui64 = new U64x( raw );
console.log( ui64.length ); // log 4
console.log( ui64.byteLength ); // log 32
```

If a `raw` buffer representation is created using a typed buffer, the byte sequence of the`raw` buffer divides by the size of the typed buffer element. Therefore, the typed buffer `float32` has 8 elements. And when `ui64` representation is used, the typed buffer has half the length because the size of one element of type`U64x` is eight bytes.

```js
var raw = new BufferRaw( 32 );
var u8 = new U8x( raw );
u8[ 0 ] = 20;
console.log( u8[ 5 ] ); // log 20
```

In the example, `u8` was created from`raw` buffer. To access individual elements of a typed buffer, indexes are used as in a regular array.

```js
var raw = new BufferRaw( 32 );

var u8First = new U8x( raw );
var u8Second = new U8x( raw );

u8First[ 0 ] = 20;
u8Second[ 5 ] = 3;

console.log( u8First[ 5 ] ); // log 3
console.log( u8Second[ 0 ] ); // log 20
```

If a typed buffer is created from a buffer of type `BufferRaw`, `BufferRaw` is passed to the typed buffer. No copying of buffer `BufferRaw` is performed when creating a representation. On the other hand, the representation does not have data but is a kind of link to the data. Therefore, when an element of one typed buffer changes, then an element in another typed buffer also changes.

### Summary

- Typed buffers represent binary data in the form of numeric primitives.
- Indexes are used to access elements of typed buffers.
- The length of the typed buffer is determined by the number of elements.
- The size of the typed buffer is determined by the number of bytes occupying all the elements.
- Typed buffers do not have data.
- Typed buffers are representations of `BufferRaw` buffers that contain real data.
- Therefore, changing the data in one representation is reflected in the other `BufferTyped` with the same` BufferRaw`.

[Back to content](../README.md#Concepts)
