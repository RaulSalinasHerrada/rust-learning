
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)] //trait implementation
struct Rectangle {
    height: u32,
    width: u32
}
//method syntax
impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    } 
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn is_square(&self) -> bool {
        self.width == self.height
    }


}

impl Rectangle {

    fn square(size: u32) -> Rectangle{
        Rectangle {
            width: size, height: size
        }
    }
}



fn main() {
    let mut user = User {
        email: String::from("ab@gmail.com"),
        username: String::from("holis"),
        active: true,
        sign_in_count : 1, 
    };

    let name= user.username;
    user.username = String::from("cd112"); //ownership?
    let user_new = build_user(
        String::from("email"),
        String::from("soyG112"));
    
    let user_from_u = User {
        email: String::from("dkadj@uc.cl"),
        username: String::from("chachacha232"),
        ..user_new //fields from instance
    };

    //tuple structs
    // struct Color(i32,i32,i32);
    // struct Point(i32, i32, i32); //useful for implementation

    let width: u32 = 30;
    let height: u32 = 50;

    let rect = Rectangle {
        height,
        width,
    };

    println!("{:#?}", rect); //debug trait

    println!(
        "The area of the rect is {} pixels, from function",
        area(&rect)
    );

    println!(
        "The area of the rect {:?} is {} pixels, from method",
        rect,
        rect.area()
    );

    let other_rect = Rectangle {height: 10, width: 10};

    if rect.can_hold(&other_rect){
        println!("Rectangle {:?} can hold rectangle {:?}", rect, other_rect) 
    }

    let size = 20;

    let sq = Rectangle::square(size);

    println!("{:?} rectangle is a square? {}", sq, sq.is_square());






}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

fn build_user(email:String, username: String) -> User {
    User {
        email, //struct shorthand syntax trick
        username,
        sign_in_count: 1,
        active: true,
    }

}
