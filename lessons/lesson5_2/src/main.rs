#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

fn main() {
    //Example 1
    //We have a two numbers for a calculate
    let width = 30;
    let height = 50;
    println!(
        "The area of the rectangles is {} square pixels.",
        area(width, height)
    );

    //Example 2.
    //We have a two numbers together
    let rect = (30, 50);
    println!(
        "The area of the rectangles is {} square pixels.",
        area_2(rect)
    );

    //Example 3.
    //We have a named structure
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangles is {} square pixels.",
        area_3(&rect)
    );

    //Can try println!
    println!("{rect:?}");
    println!("{rect:#?}");

    //Use new macros !dbg (stream dbg)
    let scale = 2;
    let rect = Rectangle {
        width: dbg!(30 * scale),
        height: 50
    };
    dbg!(&rect);
}

fn area(width:u32, height:u32) -> u32 {
    width * height
}

fn area_2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_3(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}