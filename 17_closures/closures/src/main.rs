
//closures: annonymous functions, stored as var, passed as params

use std::collections::HashMap;
use std::{thread};
use std::time::Duration;
use std::hash::Hash;

// fn simulated_expensive_calculation(intensity: u32) -> u32 {
//     println!("calculating slowly...");
//     thread::sleep(Duration::from_secs(1));
//     intensity
// }

//create memoisation (caching) structure to cache elements

struct Cacher<F, U>
where F: Fn(U) -> U, U: Hash + Eq + Copy{
    calculation: F,
    hashmap: HashMap<U,  U>,
}

impl <F,U> Cacher<F,U>
where
    F: Fn(U) -> U, U: Hash + Eq + Copy
{
    fn new(calculation: F) -> Cacher<F, U> {
        Cacher {
            calculation,
            hashmap : HashMap::new(),
        }
    }

    fn value(&mut self, arg: U) -> U {
        match self.hashmap.get(&arg) {
            Some(v) => *v, //caching
            None => {
                        let v = (self.calculation)(arg);
                        self.hashmap.insert(arg, v);
                        v
                    }
        }
    }
}

fn generate_workout(
    intensity: u32,
    random_number: u32) {
    
    // how can we not call simulated_expensive_funct multiple times?
    // innecesarily, we want to limit the amount of calls


    //other problem, it's called even if number is 3,
    // where we can omit the calculation

    //define code 1 place and calculate only if needed
    //let's define a closure

    //if it's passed to a parameter, it guess what type it  takes!

    let mut cached_result = Cacher::new(
    
    |num| {
        println! ("calculating slowly...");
        thread::sleep(Duration::from_secs(1));
        num
    });

    if intensity < 25 {

        println!(
            "Do {} pushups",
            cached_result.value(intensity)
        );

        println!(
            "Next, do {} situps",
            cached_result.value(intensity)
    );
    } else {
        if random_number == 3 {
            println!("Take a break");
        } else {
            println!(
                "Today, run for {} minutes",
                cached_result.value(intensity + 2)
            );
        }
    }
    
    
}

fn main() {
    let simulated_intensity = 10;
    let random_number = 7;

    generate_workout(
        simulated_intensity,
        random_number);


    //closured for dynamic environment

    let x = 4;
    // if it's a function , they can't capture dynamic environment
    let equal_to_x = |z| z == x ;
    // closure capture values as functions, produces overhead (borrowing, ownership, passing)
    // encoded on function traits
    // * FnOnce: Takes ownership
    // * FnMut: mutable borrows values
    // * Fn: Unmutably borrows values

    let y = 4;

    assert!(equal_to_x(y));


    let c = vec![1,2,4];

    let equal_to_c = |z| z == c ;

    println!("can't use c here: {:?}", x);

    let k = c.clone();

    assert!(equal_to_c(k));

}
