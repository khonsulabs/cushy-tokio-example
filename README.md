# Cushy + Tokio Example

This repository is a temporary example that shows the currently recommended way
to integrate [Tokio](https://tokio.rs/) and
[Cushy](https://github.com/khonsulabs/cushy).

Cushy uses [`winit`](https://github.com/rust-windowing/winit) for its windowing
and event loop handling. On the platforms Cushy currently supports, `winit`
takes over the main thread when executing. This can cause problems with code
like this:

```rust
#[tokio::main]
async fn main() {
    tokio::spawn(/* do something */);

    // Run a Cushy app
    "Hello World".run().unwrap();
}
```

This is problematic because the spawn call may place that task on the current
thread's work queue, and once `winit` takes over the thread, the spawned task
will never be yielded to.

This example shows how to reliablly execute Cushy with a Tokio runtime, ensuring
tasks can be spawned from any Cushy code.

## Long Term Solution

Cushy will add a mechanism to easily enable tokio or be able to prepare each
Cushy thread to support any executor. See
[khonsulabs/cushy#147](https://github.com/khonsulabs/cushy/issues/147) to see
progress updates.
