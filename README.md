# todo

An anticlimactic proc-macro for generating concrete types for traits.

## Usage

```toml
[dependencies]
todo-trait = { git = "https://github.com/csh/todo.git" }
```

Attach the todo macro to your trait.

```rust
#[todo]
trait Greeter {
    fn greet(name: &str);
}
```

Generated output:

```rust
#[cfg(debug_assertions)]
struct TodoGreeter;
impl Greeter for TodoGreeter {
    fn greet(name: &str) { ::core::panicking::panic("not yet implemented") }
}
```

The cfg guard exists to stop people using this in release mode, it's mostly meant to speed up prototyping.

If you wish to override this behaviour (ie compiling for a debug-unfriendly environment) then use this instead:

```rust
#[todo(enable_in_release = true)]
trait Greeter {
    fn greet(name: &str);
}
```