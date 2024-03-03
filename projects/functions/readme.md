## Functions

**functions**

- snake case naming convention for function and variable names
- you can define the function anywhere as long as they are in scope for the caller

```rust
fn main() {
  println("Hello world");
  another_function();
}

fn another_function() {
  println("Another function");
}
```

**parameters**

- Parameters are special variables part of the function signature
- Arguments are the values given to those variables when calling the function
- You must annotate the type of the parameter
  - seems a severe requirements, but it makes the compiler sure of how to use that function anywhere else in the codebase

**statements and expressions**
Rust is an expression-based language. It's important to make the distinction between **statement** and **expression**

- _statements_ are instructions that perform some action and do not return a value
  - `let y = 6;`
  - function definitions are also statements
  - variable declaration using `let` is a statement
  - statements do not return values, so you cannot assign a `let` statements to another variable, as there is nothing to bind the new variable to
- _expressions_ evaluate to a resultant value
  - `5+6` or any other math operation
  - the `6` in the statement is an expression
  - expressions can be part of statements
  - calling a function is an expression
  - calling a macro is an expression
  - scope block `{}` are expressions
  - expressions do not need semicolons - if you add semicolons, you turn them into statements

```rust
fn main() {
  let y = { // y is the statement
    let x = 3; // x is the expression
    x + 1 // expressions do not need semicolons
  };
  println("The value of y is: {y}");
}
```

**functions with return values**
Functions can return values to the code that calls them.

- return values **are not named** but they are declared with a `->`
- the return value is synonymous with the value of the final expression in the block of the body of the function
- you can return early with the `return` keyword and specify a value
- most functions return the last expression implicitly

```rust
fn five() -> i32 { // return type specified here
  5 // perfectly valid return
}

fn main() {
  let x = five();
  println("The value of x is: {x}");
}
```

- another example of the difference between statements and expressions

```rust
fn main() {
  let x = plus_one(5);

  println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
  x + 1
}
```

- if you add a `;` to `x + 1` you turn it into a statement and you get the error
  - `expected`i32`, found `()`
  - statements do not evaluate to a value, which is expressed with the `()`
