#[stack_debug::instrument]
fn function_with_large_stack_frame() {
    // Allocate 4KB on the stack to make the frame size obvious.
    let my_local_data = [0u8; 4096];
    println!("{my_local_data:?}");
    nested();
}

#[stack_debug::instrument]
fn nested() {
    let my_local_data = [0u8; 128];
    println!("{my_local_data:?}");
}

#[stack_debug::instrument]
fn function_with_small_stack_frame() {}

fn main() {
    tracing_subscriber::fmt().init();
    function_with_small_stack_frame();
    function_with_large_stack_frame();
}
