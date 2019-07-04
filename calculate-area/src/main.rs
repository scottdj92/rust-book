#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    // let width1 = 30;
    // let height1 = 20;

    // let rect = (30, 20);
    let rect = Rectangle { width: 30, height: 20 };
    println!("the rectangle is {:#?}", rect);

    println!(
        "the area of the rectangle is {} square pixels",
        // area(width1, height1),
        // area(rect)
        // area(&rect)
        &rect.area()
    );
}

// basic func
// fn area(w: u32, h:u32) -> u32 {
//     w * h
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// fn area(rect: &Rectangle) -> u32 {
//     rect.width * rect.height
// }
