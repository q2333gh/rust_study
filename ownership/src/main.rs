fn main() {
    stack_example();
    heap_move_pointer_example();
    heap_clone_example();
    shallow_copy();
}
// stack data is *fixed* on *compile time*,and faster fetch than heap
fn stack_example() {
    let x = "hello";
    let y = x; // here = means simple copy; only simple data do copy behaviour,such as int,bool,char;
    println!("stack example:\nx: {}  \ny copy from x: {}\n", x, y);
}
// heap is *dynamic* grow on *running time*.need a bit more dereference to heap-space than stack-ref
fn heap_move_pointer_example() {
    let x = String::from("hello"); //this statement function is like C-malloc,so data on the heap
    let y = x; //*ownership transfer*, and this = behaviour here is called *move*; is NOT called shallow copy in rust-lang,
    println!(
        "heap_move_pointer_example:\nx transfer-its-ownership to y: {}\n",
        y
    );
}

fn heap_clone_example() {
    let x = String::from("hello"); //this statement function is like C-malloc,so data on the heap
    let y = x.clone(); //*ownership transfer*, and this = behaviour here is called *move*; is NOT called shallow copy in rust-lang,
    println!(
        "heap_clone_example:\nx on the heap: {}\ny clone heap data(repeat) from x: {}\n",
        x, y
    );
}
fn shallow_copy() {
    let s1 = String::from("hello");
    let s2 = &s1;

    println!("s1: {}", s1);
    println!("s2: {}", s2);
    drop(s1);
}
