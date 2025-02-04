fn main() {

    let config_max = Some(3u8);

//    The syntax if let takes a pattern and an expression separated by an equal sign. It works the same
//way as a match , where the expression is given to the match and the pattern is its ﬁrst arm. In this
//case, the pattern is Some(max) , and the max binds to the value inside the Some . We can then use
//max in the body of the if let block in the same way we used max in the corresponding match
//arm. The code in the if let block isn’t run if the value doesn’t match the pattern.


    if let Some(max) = config_max {
        println!("The maximum is {max}");
    }




}
