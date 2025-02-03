use sandbox_2::some_mode;

fn main() {
    //for n in 1..=43 {
    //    println!("{}", n)
    //}
    //
    some_mode::first_submodule::first_submodule_function();
    println!("Program starts");
    

    some_mode::second_submodule::print_sum(43, 65);

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
