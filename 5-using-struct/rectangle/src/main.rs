#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }
}

fn main() {
    let scale = 1;

    let mut rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    let area1 = rect1.area();
    let area2: u32 = Rectangle::area(&rect1);
    assert_eq!(area1, area2);

    // println!("rec1 is {:#?}", rec1);
    dbg!(&rect1);
    rect1.set_width(90);
    Rectangle::set_width(&mut rect1, 96);

    println!("{} * {} = {}", rect1.width, rect1.height, area1);

    let r = &mut Box::new(Rectangle {
        width: 1,
        height: 2
    });
    let ar1 = r.area();
    let ar2 = Rectangle::area(&**r);
    assert_eq!(ar1, ar2);
}
