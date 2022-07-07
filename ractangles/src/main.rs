use core::prelude;


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // let width1 = 30;
    // let height1 = 50;
    // println!("area is {}", area(width1, height1));

    // let rect1 = (30, 50);
    // println!("area is {}", area(rect1));

    // let rect1 = Rectangle {
    //     width: dbg!(30),
    //     height: 50
    // };
    // println!("area is {}", area(&rect1));
    // println!("rect is {:#?}", rect1);
    // dbg!(&rect1);

    let rect1 = Rectangle {
        width: 30,
        height: 50
    };

    println!("area is {}", rect1.area());
}

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}