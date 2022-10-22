#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

// Similar to a class in python
/* All functions defined within an impl block are called associated functions because 
they're associated with the type(struct) named after the impl */

impl Rectangle {
    // Area is a method, a methods first parameter must always be self
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width > rect.width && self.height > rect.height
    }

}

fn main() {
    
    let rect1 = Rectangle {
        width: 30, 
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

}
