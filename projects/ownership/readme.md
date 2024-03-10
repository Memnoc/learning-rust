## Ownership

How Rust manages memory.

**Stack and Heaps**
Both are parts of memory available to use at runtime. The problems _ownership_ addresses are: keeping track of what parts of code are stored are using what data on the heap, minimising the amount of duplicate data on the heap, cleaning up unused data on the heap so you don't run out of space.

- _Stack_: **all data stored on the stack must have a fixed size**

  - _last in, first out_ stores values in the order it gets them and removes them in the opposite order. So basically the stack removes values from the top of the stack and so on.
  - Adding data is _pushing onto the stack_ to the stack and removing data is _poppig off the stack_.
  - Pushing to the stack is faster than allocating to the heap (allocator does not have to search for a place to store memory, it's always at the top of the stack)
  - Modern processors are faster if they jump around less.
  - When the code calls a function, the values passed into the function (including any pointers to data on the heap) and the function's local variables are pushed onto the stack.
  - When the function is over, the values are popped off the stack.

- _Heap_: **all data stored on the stack must have a fixed size**

  - The heap is less organised. Data can be of any size, and the memory allocator takes care of finding a big enough spot. Once found, it returns a pointer, and because the pointer's size is known at compile time, **that pointer is stored in the stack** To retrieve the actual data, however, you must follow the pointer.
    _allocating on the heap_ is the process of requesting memory from the memory allocator.

**Ownership rules**
To keep in mind:

- Each value in Rust had an owner
- There can only be one owner at a time
- When the owner goes out of scope, the value will be dropped

**The String Type**
It's easier to look at the string type when it comes to explain ownership, as that is data that has to be stored in the heap and it related to more complex data types than integer and booleans.

- `String` can be mutated. Its content it's allocated by the memory allocator so it can change - in other words, it's allocated on the heap.

```rust
let mut s = String::from("hello");
s.push_str(", world");
println!("{}", s);
```

- `string literals` cannot be mutated (immutable), as the content is known and allocated at compile time - in other words, it's stored in the stack.

```rust
let s = "Hello";
```

**Why**

When we do `String::from` we are asking for the memory. In Rust, we do not have the classic garbage collection approach (like in JavaScript and Java) and we do not have the manual way either, like in C.
**In Rust, memory is automatically returned once the variable that owns it goes out of sscope**

```rust
{
  let s = String::from("hello"); // s is valid from this point forward

  // do stuff with s
                                // the scope is now over
}                               // no longer valid, memory returned
```

There is a natural point in which we can return the memory, and Rust makes use of that under the hood calling a `drop` function automatically at the closing curly bracket
