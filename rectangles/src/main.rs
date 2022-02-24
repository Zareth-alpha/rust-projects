//Rust includes functionality to print out debugging information.
//This can help println!("{}", struct_name)
//can format with println!("{:?}", struct_name)
//can format with println!("{:#?}", struct_name)
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

/*
In the signature for area, instead of rectangle: &rectangle, the &self is actually short for self: &Self.
Methods must have a parameter named self of type Self for their first parameter.
Be careful of ownership. Giving ownership is usually reserved for transforming self into another type.

*/
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }
    /*
    All functions defined within an impl block are called associated functions because they’re associated with the type named after the impl. We can define associated functions that don’t have self as their first parameter (and thus are not methods) because they don’t need an instance of the type to work with. We’ve already used one function like this, the String::from function, that’s defined on the String type.
    Associated functions that aren’t methods are often used for constructors that will return a new instance of the struct. For example, we could provide an associated function that would have one dimension parameter and use that as both width and height, thus making it easier to create a square Rectangle rather than having to specify the same value twice:
    */

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

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

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