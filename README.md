# `stack-debug`

An experimental Rust crate for instrumenting functions to print stack sizes to debug stack overflows.

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

## Feature Flags

- `tracing`: Switches to using `tracing` to log frame sizes, instead of `println!()`.