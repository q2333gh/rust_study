fn main() {
    let mut sum = adder(5, 6);
    println!("{}", sum);
    sum = adder2(5, 6);
    println!("{}", sum);
    statement1();
    // infinite_loop();
    stack_heap();
}
fn adder(x: i32, y: i32) -> i32 {
    // -> means return type
    println!("adder() is called , this is an expression : means with return,diff from statement.");
    x + y
}
fn adder2(x: i32, y: i32) -> i32 {
    return x + y;
}
fn statement1() {
    println!("just a statement with no return");
}
fn infinite_loop() {
    loop {
        println!("loops!")
    }
}
fn stack_heap() {
    fn a() {
        let x = "hello";
        let y = 22;
    }
    fn b() {
        let x = String::from("world");
    }
}
