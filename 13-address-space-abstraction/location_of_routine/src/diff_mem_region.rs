use std::alloc::{alloc, Layout};

static GLOBAL_VAR: i32 = 42;

fn main() {
    // Code section
    println!("location of code : {:p}", main as *const fn());

    // Global/static data section
    println!("location of global: {:p}", &GLOBAL_VAR);

    // Heap allocation
    let heap_box = Box::new(vec![0u8; 1_000_000]); // 1MB
    println!("location of heap : {:p}", heap_box.as_ptr());

    unsafe {
        let layout = Layout::from_size_align(100_000_000, 8).unwrap();
        let raw_heap = alloc(layout);
        println!("location of raw heap: {:p}", raw_heap);
    }

    // stack variables
    // Stack variables
    let x = 3;
    let y = [0u8; 1024]; // Stack array
    println!("location of stack var: {:p}", &x);
    println!("location of stack array: {:p}", y.as_ptr());

    // String literals (usually in read-only data section)
    let s = "Hello, World!";
    println!("location of string literal: {:p}", s.as_ptr());
}
