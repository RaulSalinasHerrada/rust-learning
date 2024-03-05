//enum allows to enlist 
#[derive(Debug)]
enum IpAddrKind {
    // V4, 
    // V6
    V4(u8, u8, u8, u8),
    V6(String),

}
#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind
}

enum Message {
    Quit, // simple enum
    Move {x:i32, y: i32}, //enum as struct
    Write(String), //constructor on enum
    ChangeColor(u8,u8,u8), //non annotated struct?
}

impl Message {
    fn function () {
        println!("This is a message!")
    }
}


fn main() {
    
    
    //implementation with problems: not elegant, add struct on enums
    // let localhost = IpAddr {
    //     kind: IpAddrKind::V4(String::from("127,0,0,1"))};

    // let localhost = IpAddr {
    //     kind: IpAddrKind::V4(String::from("127.0.0.1"))
    // }; 

    //can we do better, params with u8s

    let localhost = IpAddr {
        kind: IpAddrKind::V4(127,0,0,1)
    }; 
    println!("this is my IP address: {:?}", localhost);

    Message::function();

    // enum Option<T> {
    //     Some(T),
    //     None,
    // }
    //important optional, gives 2 things, the element if there's or none.
    //it obliges deal with the none case if neccesary

    let some_nomber = Some(5);
    let some_string = Some(String::from("a string"));

    let not_number: Option<i32> = None; //MUST ANNONATE TYPE OF NONE!

    // options and types don't mix

    let x = 5;
    let y = Some(4);
    let y: Option<i32> = None;
    //how to add?

    let x_y = x + y.unwrap_or(0); //deals with cases


    //pattern matching, application given enums

    #[derive(Debug)]
    enum UsState {
        Carolina,
        California,
        //...
    }


    #[derive(Debug)]
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState)
    }

    fn cent_value(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {println!("Lucky penny"); 1},
            Coin::Dime => 10,
            Coin::Nickel => 5,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}", state); 25
            },
        }
    }

    let value = cent_value(Coin::Dime);

    println!("Value of {:?}: {}", Coin::Dime, value);

    cent_value(Coin::Quarter(UsState::Carolina));

    let z = Some(1);

    let zp1 = p1(z);
    let z_none: Option<i32> = None;

    let nonep1 = p1(z_none);
    println!("Adding plus 1 on {:?} is: {:?}", z, zp1);
    println!("Adding plus 1 on {:?} is: {:?}", z_none, nonep1);

    //if let syntax for pattern matching

    let some_value = Some(3);

    if let Some(3) = some_value { // READ AT THE INVERSE!!! ()
        println!("Three!")
    }



    
}

fn p1(x:Option<i32>) -> Option<i32> {
    match x {
        //match is exhausitive (all options)
        Some(i) => Some(i+1),
        _ => None, //all other cases (_ is placeholder)
    }


}