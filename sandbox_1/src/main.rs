fn main() {
    struct MyStruct {
        name: String,   
    }


    let mut struct1 = MyStruct{ name:String::from("Jan")};

    let num_taking_ownership = struct1.name;
    
    struct1.name = String::from("Jan2");
    
    println!("{num_taking_ownership}");
    println!("{}", struct1.name);

    println!("{num_taking_ownership}");
}
