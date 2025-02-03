use sub1::func1;

mod sub1;


pub fn first_submodule_function() {
    println!("First submodule");
    func1();
}
