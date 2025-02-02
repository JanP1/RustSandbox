//// Derive the `fmt::Debug` implementation for `Structure`. `Structure`
//// is a structure which contains a single `i32`.
//#[derive(Debug)]
//struct Structure(i32);
//// Put a `Structure` inside of the structure `Deep`. Make it printable
//// also.
//#[derive(Debug)]
//struct Deep(Structure);
//fn main() {
//
//
//    let deep = Deep(Structure(7));
//    let structure = Structure(8);
//    let deep_0 = deep.0;
//
//    // Printing with `{:?}` is similar to with `{}`.
//    println!("{:?} months in a year.", 12);
//    println!("{1} {0:?} is the {actor} name.",
//    "Slater",
//    "Christian",
//    actor="actor's");
//
//
//    // `Structure` is printable!
//    println!("Now {:?} will print!", structure.0);
//
//
//    // The problem with `derive` is there is no control over how
//    // the results look. What if I want this to just show a `7`?
//    println!("Now {:?} will print!", deep_0);
//}
//
//
//use std::fmt; // Import `fmt`
//// A structure holding two numbers. `Debug` will be derived so the results can
//// be contrasted with `Display`.
//#[derive(Debug)]
//struct MinMax(i64, i64);
//// Implement `Display` for `MinMax`.
//impl fmt::Display for MinMax {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        // Use `self.number` to refer to each positional data point.
//        write!(f, "({}, {})", self.0, self.1)
//    }
//}
//// Define a structure where the fields are nameable for comparison.
//#[derive(Debug)]
//struct Point2D {
//    x: f64,
//    y: f64,
//}
//// Similarly, implement `Display` for `Point2D`.
//impl fmt::Display for Point2D {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        // Customize so only `x` and `y` are denoted.
//        write!(f, "x: {}, y: {}", self.x, self.y)
//    }
//}
//fn main() {
//    let minmax = MinMax(0, 14);
//    println!("Compare structures:");
//    println!("Display: {}", minmax);
//    println!("Debug: {:?}", minmax);
//    let big_range =
//    MinMax(-300, 300);
//    let small_range = MinMax(-3, 3);
//    println!("The big range is {big} and the small is {small}",
//    small = small_range,
//    big = big_range);
//    let point = Point2D { x: 3.3, y: 7.2 };
//    println!("Compare points:");
//    println!("Display: {}", point);
//    println!("Debug: {:?}", point);
//    // Error. Both `Debug` and `Display` were implemented, but `{:b}`
//    // requires `fmt::Binary` to be implemented. This will not work.
//    // println!("What does Point2D look like in binary: {:b}?", point);
//}

//
//fn main() {
//    // Variables can be type annotated.
//    let logical: bool = true;
//    let a_float: f64 = 1.0; // Regular annotation
//    let an_integer = 5i32; // Suffix annotation
//    // Or a default will be used.
//    let default_float
//    = 3.0; // `f64`
//    let default_integer = 7;
//    // `i32`
//    // A type can also be inferred from context.
//    let mut inferred_type = 12; // Type i64 is inferred from another line.
//    inferred_type = 4294967296i64;
//    // A mutable variable's value can be changed.
//    let mut mutable = 12; // Mutable `i32`
//    mutable = 21;
//    // Error! The type of a variable can't be changed.
//    //mutable = true;
//    // Variables can be overwritten with shadowing.
//    let mutable = true;
//    /* Compound types - Array and Tuple */
//    // Array signature consists of Type T and length as [T; length].
//    let my_array: [i32; 5] = [1, 2, 3, 4, 5];
//    // Tuple is a collection of values of different types
//    // and is constructed using parentheses ().
//    let my_tuple = (5u32, 1u8, true, -5.04f32);
//}
//
//
// `NanoSecond`, `Inch`, and `U64` are new names for `u64`.
//type NanoSecond = u64;
//type Inch = u64;
//type U64 = u64;
//fn main() {
//    // `NanoSecond` = `Inch` = `U64` = `u64`.
//    let nanoseconds: NanoSecond = 5 as u64;
//    let inches: Inch = 2 as U64;
//    // Note that type aliases *don't* provide any extra type safety, because
//    // aliases are *not* new types
//    println!("{} nanoseconds + {} inches = {} unit?",
//        nanoseconds,
//        inches,
//        nanoseconds + inches);
//}
//
//
//fn main() {
//    let mut counter = 0;
//
//    let result = loop {
//        counter += 1;
//        if counter == 10 {
//            break counter * 2;
//        }
//    };
//    assert_eq!(result, 20);
//}
//
//
//fn main() {
//    // A counter variable
//    let mut n = 1;
//    // Loop while `n` is less than 101
//    while n < 101 {
//        if n % 15 == 0 {
//            println!("fizzbuzz");
//        } else if n % 3 == 0 {
//            println!("fizz");
//        } else if n % 5 == 0 {
//            println!("buzz");
//        } else {
//            println!("{}", n);
//        }
//        // Increment counter
//        n += 1;
//    }
//}

use sandbox_2::adding;

fn main() {
   println!("{}", adding(32, 34)) 
}
