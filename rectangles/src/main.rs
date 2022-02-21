//Rust includes functionality to print out debugging information.
//This can help println!("{}", struct_name)
//can format with println!("{:?}", struct_name)
//can format with println!("{:#?}", struct_name)
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
//    let width1 = 30;
//    let height1 = 50;
//    let rect1 = (30,50);
    let rect1 = Rectangle{
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
//        area(rect1)
//        area(width1,height1)
//        area(&rect1)
        rect1.area()
    );

    //takes ownership of expression, prints the file and line #
    //and the result of the expression.
    //Can pass references.
    //if there was an operation, it would show results.
    //writes to stderr rather than stdout.
    dbg!(&rect1);
}
/*
not quite clear enough.
fn area(width: u32, height: u32) -> u32 {
    width * height
}
*/
/*
//even more confusing.
fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
*/
/*
moving to a Rectangle method.
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
*/