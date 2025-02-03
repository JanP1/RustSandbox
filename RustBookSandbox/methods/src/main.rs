use methods::rectangle::Rectangle;

fn main() {

    let rect1 = Rectangle{
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels",
        rect1.area()
    );

    let rect2 = Rectangle { width: 10, height: 40, };
    let rect3 = Rectangle { width: 60, height:
        45, };   

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));


}
