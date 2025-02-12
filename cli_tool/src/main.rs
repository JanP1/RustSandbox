use std::env;
use std::process;
use cli_tool::Config;


fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments {err}");
        process::exit(1);
    });

    if let Err(e) = cli_tool::run(config) {
        println!("Aplication error: {e}");
        process::exit(1);
    }
    
}


