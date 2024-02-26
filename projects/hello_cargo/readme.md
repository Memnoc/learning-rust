## Cargo

**Summary**
Is the package manager of Rust, much like Node is for JavaScript

`cargo new project_name` creates a new project with all the files you need in it

`Carego.toml` is the equivalent of package.json where dependecies are declared along with package info

`cargo run` runs the project and also builds the project creating the executables in `target/debug/project_name`

`cargo check` checks that the project compiles without running it. This is useful when the project is large and you need something to build much faster than with `cargo build`

**Convention on existing projects**

```rust
$ git clone example.org/someproject
$ cd someproject
$ cargo build
```

**Recap**

- We can create a project using `cargo new`
- We can build a project using `cargo build`
- We can build and run a project in one step using `cargo run`
- We can build a project without producing a binary to check for errors using `cargo check`
- Instead of saving the result of the build in the same directory as our code, Cargo stores it in the target/debug directory.
- `cargo build --release` is the flag you want to use to build the project with all the optimization you might want for production
