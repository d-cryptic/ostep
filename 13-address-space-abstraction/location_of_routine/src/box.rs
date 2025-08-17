fn main() {
    // function pointer(code location)
    println!("location of code: {:p}", main as *const fn());

    // heap allocation using Box (safer than raw allocation)
    let heap_data = vec![0u8; 100_000_000];
    println!("location of heap: {:p}", heap_data.as_ptr());

    //stack variable
    let x = 3;
    println!("location of stack: {:p}", &x);
}
