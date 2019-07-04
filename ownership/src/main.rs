fn main() {
    let s = String::from("hello");
    // let str = s.clone();

    takes_ownership(s);
    // println!("{}", str);

    let x = 5;

    makes_copy(x);

    println!("{}", x);
}

fn takes_ownership(str: String) {
    println!("{}", str);
}

fn makes_copy(num: i32) {
    println!("{}", num);
}
