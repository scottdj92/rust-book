fn main() {
    println!("Hello, world!");

    another_function(5);
    flow_control(3);
    flow_control(7);
}

fn another_function(x: u32) {
    println!("Another function");
    println!("the value of x is {}", x);
}

fn flow_control(num: u32) {
    if num < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

