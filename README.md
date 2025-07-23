# `stack-debug`

An experimental Rust crate for instrumenting functions to print stack sizes to debug stack overflows.

The motivation to create this crate came from a situation where I wanted to debug a stack overflow in an application and I wanted to see which functions were taking up the most amount of stack space.

## Usage

In your `.cargo/config.toml` we need to enable frame pointers, as the calculations rely on them:

```toml
[build]
rustflags = ["-C", "force-frame-pointers=y"]
```

In your `Cargo.toml`:

```toml
[dependencies]
stack-debug = "<VERSION>"
```

In your code:

```rust
#[stack_debug::instrument]
fn my_function() {
    // ...
}
```

## Example Output

Running the example in [examples/example](./examples/example), the logged frame size output looks like this:

```
example::function_with_small_stack_frame(): stack frame size: 0
example::function_with_large_stack_frame(): stack frame size: 4176
example::nested(): stack frame size: 208
```

With the `tracing` flag enabled:

```
2025-07-23T06:23:22.461172Z  INFO function_with_small_stack_frame: example: stack frame size: 0
2025-07-23T06:23:22.461249Z  INFO function_with_large_stack_frame: example: stack frame size: 4176
2025-07-23T06:23:22.461518Z  INFO function_with_large_stack_frame:nested: example: stack frame size: 224
```

## Feature Flags

- `tracing`: Switches to using `tracing` to log frame sizes, instead of `println!()`.