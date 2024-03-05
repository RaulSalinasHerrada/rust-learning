fn main() {
    let x:i8 = 5;
    println!("Value: {}", x);
    let x: u8 = 6; //shadowing of variable. Change types, 
    println!("Value: {}", x);

    // const LUCAS: u32 = 1_000; //cant mut, can't be from result

    // let a = 91_110;
    // let a = 0x1fa; //hex
    // let a = 0o77; //octal
    // let a = 0b1000101; //bin
    // let a = b'A'; //bytes (u8)

    // let ch = 'z';
    // let ch: char = 'ᗢ';

    // let tup = ("let's get rusty", 10);
    // let (ch, sub) = tup;
    // let subc = tup.1;
    // let error_codes = [200, 404, 500];
    // let not_found = error_codes[1];
    // let byte_array = [0;8]; //0, 8 times

    my_func(1, 2);
    let condition = true;
    let number: i8 = if condition {5} else {0};
    println!("{}", number);

    let mut counter = 0;

    let result = loop{
        counter += 1;
        if counter == 10{
            break counter;
        }
    };

    println!("nice! {}", result);

    let arr = [1,2,3];

    for element in arr.iter()
    {
        println!("the value in range is {}", element);
    }
}

fn my_func(x:i32, y:i32) -> i32 {
    println!("value x: {}", x);
    println!("value y: {}", y);
    x + y //implicit return
}