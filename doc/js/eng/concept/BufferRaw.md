# Type <code>BufferRaw</code>

Standard untyped buffer, a raw data buffer that is not intended to access data by this object directly but to be used by other buffers as a data container.

This type allows allocating memory for the data, but it does not provide the instruments to access the data. For processing data contained in an instance of type `BufferRaw`, the mechanism of data representation is used. It is representation of data by any instance of [typed buffers](./ BufferTyped.md) or by [`BufferView` buffer](./ BufferView.md).


Type `BufferRaw` has two implementations:

- [`BufferRaw`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/ArrayBuffer) ( `ArrayBuffer` );
- [`BufferRawShared`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/SharedArrayBuffer) ( `SharedArrayBuffer` ).

The difference between `BufferRaw` and` BufferRawShared` is that `BufferRawShared` can give access to data for multiple procedures ( workers/processes/threads ) at a time, and `BufferRaw` only one. In this case, both `BufferRaw` and` BufferRawShared` can have multiple representation in one thread. Changing the buffer made in one representation is immediately reflected in the other because the representations do not contain real data, but only talk about how to interpret data from an instance of `BufferRaw`.

### Examples

```js
var raw = new BufferRaw( 15 );
console.log( raw.byteLength ); // log 15
```

A class constructor of the same name is used to create an instance of type `BufferRaw`. The required buffer size in bytes is passed to the constructor. The size of the `raw` buffer is 15 bytes.

```js
var sharedRaw = new BufferRawShared( 10 );
console.log( sharedRaw.byteLength ); // log 10
```

A class constructor of the same name is used to create an instance of type `BufferRawShared`. The required buffer size in bytes is passed to the constructor. The size of the `sharedRaw` buffer is 10 bytes.

### Summary

- Type `BufferRaw` is used to work with binary data of any size.
- Type `BufferRaw` does not implement methods for accessing data.
- To access data in an instance of `BufferRaw`, representations ( views ) are used. It is representation of data by instances of types` BufferTyped` and `BufferView`.
- One `BufferRaw` may have more than one representation; changes made in one representation are immediately reflected in other representation.

[Back to content](../README.md#Concepts)
