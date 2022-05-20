## Concepts

<details>
  <summary><a href="./concept/Vectorization.md#vectorization">
    Vectorization
  </a></summary>
    Vectorization is the process of converting an algorithm from operating on a single input value at a time
    to operating on a set of input values(<a href="./concept/Vector.md#vector">vector</a>) at one time.
</details>

<details>
  <summary><a href="./concept/Vectorization.md#automatic-vectorization">
    Automatic vectorization
  </a></summary>
  Vectorization can be performed using the routine <a href="./tutorial/Vectorize.md">vectorize</a>.
</details>

<details>
  <summary><a href="./concept/Vectorization.md#manual-vectorization">
    Manual vectorization
  </a></summary>
  Data operations can be complex and not be amenable to automatic vectorization or make it inappropriate.
  In this case, you need to vectorize the routines manually.
</details>

<details>
  <summary><a href="./concept/Evaluator.md#evaluator">
    Evaluator
  </a></summary>
    This is a routine or pair of routines that allow you to arbitrarily convert array elements before comparing them. If the obtained values match - the evaluator returns <code>true</code>, otherwise - <code>false</code>. The evaluator as a callback function is passed to the routines intended for working with [Long](./concept/Long.md) types.
</details>

<details>
  <summary><a href="./concept/Equalizer.md#equalizer">
    Equalizer
  </a></summary>
  This is a routine that compares two elements of an array and returns <code>true</code> if the comparison condition is met, otherwise - <code>false</code>. The `equalizer` as a callback function is passed to the routines intended for working with [Long](./concept/Long.md) types.
</details>

<details>
  <summary><a href="./concept/TypePrimitive.md">
    Type <code>primitive</code>
  </a></summary>
    In <code>JavaScript</code>, all data types that are integral and have no changeable elements are called primitives.
</details>

<details>
  <summary><a href="./concept/TypePrimitive.md#Type-string">
    Type <code>string</code>
  </a></summary>
    Primitive data type to represent text data.
</details>

<details>
  <summary><a href="./concept/TypeUnroll.md">
    Type <code>unroll</code>
  </a></summary>
    Data type <code>unroll</code> - a special type of array that can be unrolled in another array when performing an operation on it.
</details>

<details>
  <summary><a href="./concept/Buffer.md">
    Buffer types
  </a></summary>
    Entities for storing and processing binary data.
</details>

<details>
  <summary><a href="./concept/BufferRaw.md">
    Type <code>BufferRaw</code>
  </a></summary>
    Standard untyped buffer, a raw data buffer that is not intended to access data by this object directly but to be used by other buffers as a data container.
</details>

<details>
  <summary><a href="./concept/BufferTyped.md">
    Type <code>BufferTyped</code>
  </a></summary>
    Standard typed buffers.
</details>

<details>
  <summary><a href="./concept/BufferNode.md">
    Type <code>BufferNode</code>
  </a></summary>
    Non-standard implementation of untyped buffer with built-in access to buffer data.
</details>

<details>
  <summary><a href="./concept/BufferView.md">
    Type <code>BufferView</code>
  </a></summary>
    Standard untyped buffers that type data of `BufferRaw` dynamically.
</details>

## Tutorials

<details>
  <summary><a href="./tutorial/RoutineUnite.md">
    Routine routine.unite
  </a></summary>
    Automatically combining of the data preparation routine and routine for data processing.
</details>

<details>
  <summary><a href="./tutorial/TimeMeasurement.md">
    How to measure time
  </a></summary>
    Measurement of algorithms performance to choose the best solution.
</details>

## Research

<details>
  <summary><a href="./research/Modularity.md#The-system-and-its-components">
    The system and its components
  </a></summary>
    A system is a group of interacting or interrelated entities that form a unified whole.
</details>

<details>
  <summary><a href="./research/Modularity.md#Interface">
    Interface
  </a></summary>
    Interface is a shared boundary between system components.
</details>

<details>
  <summary><a href="./research/Modularity.md#Documentation">
    Documentation
  </a></summary>
    Documentation is a set of documents to describe interface, its components or concepts behind the system.
</details>

<details>
  <summary><a href="./research/Modularity.md#Test">
    Test
  </a></summary>
    Test is code or procedures which ensure that variation of input of the interface of the system produces expected output.
</details>

<details>
  <summary><a href="./research/Modularity.md#Modularization">
    Modularization
  </a></summary>
    Modularization is the architectural design process of replacing a monolithic system by several modules which are interrelated somehow.
</details>

<details>
  <summary><a href="./research/ErrorHandling.md">
    Errors handling
  </a></summary>
    The state of the art techniques of handling errors.
</details>

<details>
  <summary><a href="./research/ErrorHandling.md">
    Mechanisms of catching errors
  </a></summary>
    Describe mechanisms of handling errors, depending on their synchronicity, build, and other circumstances.
</details>

<details>
  <summary><a href="./research/Modularity.md">
    Modularity of the system
  </a></summary>
    This article describes differences between modular and monolithic systems, concepts behind it and effects of modularization on the workflow.
</details>

<details>
  <summary><a href="./research/Modularity.md#Effects-of-modularization-on-the-workflow">
    Effects of modularization on the workflow
  </a></summary>
    Modularization of the system has several effects on the workflow.
</details>
