fn main() {
    //for n in 1..=43 {
    //    println!("{}", n)
    //}
    //

    println!("Program starts");

    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter() {
        match name {
            &"Ferris" => println!("Works for Ferris"),
            _ => println!("Hello {}", name),
        }
    }

    println!("Just placing some text");
    println!("Names: {:?}", names);
}
