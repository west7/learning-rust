#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn area(&self) -> usize {
        (self.height * self.width) as usize
    }

    fn can_hold(&self, rec: &Rectangle) -> bool {
        self.width > rec.width && self.height > rec.height
    }

    fn square(size: i32) -> Self {
        Self {
            height: size,
            width: size,
        }
    }
}

fn area_with_variables(width: i32, height: i32) -> usize {
    (width * height) as usize
}

fn area_with_tuple(dimensions: (i32, i32)) -> usize {
    (dimensions.0 * dimensions.1) as usize
}

fn main() {
    let rec3 = Rectangle {
        width: 30,
        height: 50,
    };

    let rec2 = (rec3.width, rec3.height);

    println!(
        "The area with variables of the rectangle 1 is {} square pixels",
        area_with_variables(rec3.width, rec3.height)
    );
    println!(
        "The area with tuple of the rectangle 2 is {} square pixels",
        area_with_tuple(rec2)
    );
    println!(
        "The area with struct of the rectangle 3: {rec3:#?} \n is {} square pixels",
        Rectangle::area(&rec3)
    );

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
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

    let sq = Rectangle::square(50);
    println!("Square: {sq:?}");
}
