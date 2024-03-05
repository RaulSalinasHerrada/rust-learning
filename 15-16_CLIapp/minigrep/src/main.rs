use std::env; // env module to get arguments given
use std::process;
use minigrep::Config; 

fn main() {
    //parsing arguments from CLI
    //reads file. 2 things! bad

    //error is unsufficient. What about args?
    let args: Vec<String> = env::args().collect(); // what do we need

    println!("args: {:?}", &args);

    let config = Config::new(&args)
    .unwrap_or_else(|err| {
        eprintln!("Problems with args: {}", err);
        process::exit(1);
    }); //in ok, returns value, of not execute closure

    if let Err(e) = minigrep::run(config){
        eprintln!("App error: {}", e);
        process::exit(1);


    }; //contains logic outside
}
