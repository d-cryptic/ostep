use std::alloc::{alloc, Layout};

fn main() {
    // Get function pointer to main (code location)
    let main_ptr = main as *const fn();
    println!("location of code: {:p}", main_ptr);

    // allocate heap memory (equivalent to malloc(100MB))
    unsafe {
        let layout = Layout::from_size_align(100_000_000, 8).unwrap();
        let heap_ptr = alloc(layout);
        println!("location of heap: {:p}", heap_ptr);
    }

    // stack variable
    let x = 3;
    println!("location of stack: {:p}", &x);
}
