use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};
use std::error::Error;
use std::net::IpAddr:


//return a unit (basically nothing) and Trait object (any error)
fn main() -> Result<(),Box<dyn Error>> {

    // panic!("Crash, world!"); //crash ungracefully

    //common Result Enum. Ok on abstract type T and Error E

    // enum Result<T,E> {
    //     Ok(T),
    //     Err(E)
    // }

    let file_result = File::open("hello.txt");

    //handling, instead of panic, create file

    let f = match file_result {
        Ok(file) => file,
        Err(error) => match error.kind() { //create match giving error
            ErrorKind::NotFound => match File::create("hello.txt") { //what if we cant create?
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating file: {:?}", e)
            },
            other_error => {
                panic! ("Problem opening file: {:?}", other_error)
            }
        }
        
    }; //hard to read. Use closures (anonymous functions, lambdas?)

    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating file");
            })
        } else {
            panic!("Problem opening file");
        }
    });

    //good practices

    let f = File::open("hello.txt").unwrap(); //equivalent as below
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => panic!("Not possible to open"),
    // };

    let f = File::open("hello.txt").expect("Failed to open");
    
    // File::open("hello.txt")?; //error, as function needs to return Option or result

// Never panic, always try to get error.
// panic: example, prototype (fast errors), test (you wanna fail!)

    let home: IpAddr = "127.0.0.1".parse().unwrap(); //we know is not gonna fail

Ok(())
}

//set $env:RUST_BACKTRACE=1 in powershell for backtrace

//good practice, if something can fail give error propagation (return result)

fn read_username_from_file() -> Result<String, io::Error>{

    // let mut s = String::new();
    //return error in both instances 
    // File::open("hello.txt")?.read_to_string(&mut s)?; 


    fs::read_to_string("hello.txt") //return result
    //? if succeded, return the element, if not return error
    //equivalent to what is below

    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e) //gives back the error!
    // };

    //if not, return success and strng
    
    // Ok(s)

    // match f.read_to_string(&mut s){
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e)
    // }
} 