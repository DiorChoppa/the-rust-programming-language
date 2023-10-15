#[derive(Clone, Copy)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn max(self, other: Rectangle) -> Rectangle {
        let w = self.width.max(other.width);
        let h = self.height.max(other.height);
        Rectangle { 
            width: w,
            height: h,
        }
    }

    fn set_to_max(&mut self, other: Rectangle) {
        *self = self.max(other);
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let mut rect = Rectangle {
        width: 30,
        height: 50
    };

    let other_rect = Rectangle {
        width: 25,
        height: 15
    };

    rect.set_to_max(other_rect);
}
