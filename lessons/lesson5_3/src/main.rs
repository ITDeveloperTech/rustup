#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, another: &Self) -> bool {
        let hold_width = another.width <= self.width;
        let hold_height = another.height <= self.height;
        hold_width && hold_height
    }

    fn can_hold_with_rotate(&self, another: &Self) -> bool {
        let hold_width = another.width <= self.width;
        let hold_height = another.height <= self.height;
        let can_hold = hold_width && hold_height;

        let hold_width = another.width <= self.height;
        let hold_height = another.height <= self.width;
        let can_hold_rotate = hold_width && hold_height;

        can_hold || can_hold_rotate
    }
}

// This is second impl, this way it's too accept
impl Rectangle {
    //Assoc function
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    //Now we have a special function for only Rectangle structure
    let rect = Rectangle {
        width: 30,
        height: 50
    };
    println!(
        "The area of rectangle is {}",
        rect.area()
    );
    println!(
        "Width is non zero? - {}",
        rect.width()
    );

    //can_hold fn realization
    let rect1 = Rectangle {
        width: 30,
        height: 50
    };
    let rect2 = Rectangle {
        width: 20,
        height: 50,
    };
    let rect3 = Rectangle {
        width: 40,
        height: 10,
    };
    println!("\n{rect1:?}, {rect2:?}, {rect3:?}");
    println!("Hold rect1 and rect2? - {}", rect1.can_hold(&rect2));
    println!("Hold rect1 and rect3? - {}", rect1.can_hold(&rect3));
    println!("Hold rect2 and rect3? - {}", rect2.can_hold(&rect3));
    println!("Hold rect1 and rect2 with rotate? - {}", rect1.can_hold_with_rotate(&rect2));
    println!("Hold rect1 and rect3 with rotate? - {}", rect1.can_hold_with_rotate(&rect3));
    println!("Hold rect2 and rect3 with rotate? - {}", rect2.can_hold_with_rotate(&rect3));

    //Use assoc function
    println!("\n{:?}", Rectangle::square(32));

}
