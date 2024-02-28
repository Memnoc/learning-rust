## Variables and Mutability

Variables are immutable by default - this means once a value is bound to a name, that value canot be changed anymore.

The compiler flags this example as error:

```rust
fn main() {
  let x = 5;
  println("The value of x is {x}");
  x = 6;
  println("The value of x is {x}");
}
```
