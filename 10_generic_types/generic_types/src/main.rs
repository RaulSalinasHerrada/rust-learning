

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &U {
        &self.y
    }
}

impl Point<f64, f64> { //implementation only for f64 points
    fn norm_squared(&self) -> f64 {
        self.x*self.x + self.y*self.y
    }
}   

impl<T,U> Point<T,U> {

    fn mixup<V,W>(self, other: Point<V,W>) -> Point<T,W> {
        Point {
            x : self.x,
            y : other.y
        }
    }
}

fn main() {

    //generic, traits and lifetimes: avoid code duplication!

    let number_list = vec![1,2,5,25,50, 100, 40];
    
    
    let largest = get_largest(number_list);
    
    println!("Largest: {}", largest);
    
    let number_list = vec![1,2];
    let largest = get_largest(number_list);
    println!("Largest: {}", largest);

    //logic on chars
    //solution: duplication (for Vec<char> ->char). BAD
    // create generic type function

    let char_list = vec!['a', 'z', '0', 'B'];

    let largest = get_largest(char_list);
    println!("Largest: {}", largest);

    //enums with generics -> Option<Type> and Result <Type,Error>

    let p1 = Point{ x :5 , y: 10};
    let p2 = Point{x:1.0, y:2.0}; //over specialisation!
    let p3 = Point{ x:1, y:1.0}; //diferent types!
    let p4 = Point{ x:1.0, y:1.0};

    p4.norm_squared(); //we get norm squared as method! for exact type

    let p5 = Point {x :'c', y: 'h'};

    let p6 = p2.mixup(p5); //guess types from implementation

//performance:  DOES NOT PRODUCE OVERHEAD!!
    
    let some_integer = Option::Some(5);
    let some_float = Option::Some('c');


}
//set for an especific type of type (vec i32)

//solution, use generic type (function for type). T = Type


//restrict generic type T with order and copy (like primitive)
fn get_largest<T: PartialOrd + Copy>(number_list: Vec<T>) -> T {
    let mut largest = number_list[0];
    // how can we put this on a function for multiple elements?
    // share logic
    for number in number_list {
        //what if types can't be compared? -> ERROR
        // generic is any type that can be ordered and copy
        if number > largest {
            largest = number;
        }
    }
    largest
}


