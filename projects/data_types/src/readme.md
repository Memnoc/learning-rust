## Data types

Two kinds: **scalar** and **compound**

Rust is statically typed, and usually the compiler can infer the type of data by the value we use. When things get uncertain, like in type conversion, we need to annotate the type.

**Scalar type**
Represents a single value. Four types:

- integers
  - signed or unsigned, from 8 to 128-bits
  - So an i8 can store numbers from -(27) to 27 - 1, which equals -128 to 127
  - i32 is the default and usually a good place to start
- floating-point numbers
  - two primitve types: f32 and f64
  - f32 is a single precision float
  - f64 is a double precision float
  - numeric operations: (all the common ones)
- Booleans
  - true or false, nothing special here
- characters
  - char (like in C)
  - used for single characters or even emojis
  - use single quotes for this

**Compound types**
Group multiple values into one type. We have **tuples** and **arrays**

- tuple type
  - fixed length
  - group values of different types
  - we can use . notatio to access single values
  - a tuple without any values is a _unit_

```rust
let tup :(i32, f64, u8) = (500, 6.4, 1);
```

To get individual values:

```rust
let tup = (500, 6.4, 1);

let (x,y,z) = tup;

println("The value of y is {y}");
```

- array type
  - must be all of the same type
  - fixed length
  - they are allocated on the stack instead of the heap
  - useful when you know the number of elements won't change (e.g. months, days of weeks, etc)

```rust
let a: [i32, 5] = [1,2,3,4,5];
// but can also omit the type as usual
let a = [1,2,3,4,5];

```
