fn main() {
    
    // Vectors --------
    
    let _a = [1, 2, 3];
    let mut v:Vec<i32> = Vec::new();
    
    v.push(1);
    v.push(2);
    v.push(3);
    
    {
        // only visible inside the scope
        let _v2 = vec![1, 2, 3]; 
    }
    
    let mut v1 = vec![1, 2, 3, 4, 5];

    let third_element = &v1[2];


    println!("The third element is {}", third_element);

    match v1.get(30) {
        Some(third) => println!("The third element {}", third),
        None => println!("There is no third element"),
    }
    
    for i in &mut v1 {
        *i += 50;
    }


    for i in &v1{
        println!("{}", i);
        
    }

    // Vectors --------


    //
    //// Structs --------
    //
    //struct MyStruct {
    //    name: String,   
    //}
    //
    //
    //let mut struct1 = MyStruct{ name:String::from("Jan")};
    //
    //let num_taking_ownership = struct1.name;
    //
    //struct1.name = String::from("Jan2");
    //
    //println!("{num_taking_ownership}");
    //println!("{}", struct1.name);
    //
    //println!("{num_taking_ownership}");
    //// Structs --------
}


