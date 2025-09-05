static mut COUNTER: u32 = 0;

fn function_that_counts() {
    unsafe {
        COUNTER += 1;
    }
}

fn return_counter() -> u32 {
    unsafe { COUNTER }
}

fn main() {
    for _ in 0..10 {
        function_that_counts();
    }
    println!("Contador {}", return_counter());
}
