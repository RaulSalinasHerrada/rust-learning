use num_traits::Num;

pub fn add<T: std::ops::Add<Output = T>>(left: T, right: T) -> T {
    left + right
}

#[derive(Debug, PartialEq)]
struct Rectangle<T:  Num + PartialOrd>{
    width : T,
    height: T
}

impl<T> Rectangle<T> 
where T: Num + PartialOrd{
    fn can_hold(&self, other: &Rectangle<T>) -> bool {
        self.width > other.width && self.height > other.height
    }
}

//custom fail message

pub fn greeting(name:&str) -> String {
    format!("Hello {}", name)
}


//we can assert code panics
pub fn greeting_no_name(_name:&str) -> String {
    format!("Hello")
}

pub struct Guess(i8);

impl Guess {
    pub fn new(value: i8) -> Guess {
        if value < 1 {
            panic!("Value must be greater than 0")
        }
        else if value > 100 {
            panic!("Value must be less than 101")
        }
        Guess(value)
    }
}

//test fails iif panic
#[cfg(test)]
mod tests {
    use core::time;
    use std::thread::sleep;

    use super::*; //get everything ))

    #[test]
    fn larger_hold_smaller() {
        let large = Rectangle {width: 8, height: 3};
        let small = Rectangle {width: 2, height: 2};

        assert!(large.can_hold(&small));
    }

    #[test]
    fn smaller_dont_hold_larger(){
        let large = Rectangle {width: 8.2, height: 3.2};
        let small = Rectangle {width: 2.2, height: 2.2};
        assert_eq!(small.can_hold(&large), false);
    }
    #[test]
    fn compare_2_rect(){
        let r1 = Rectangle {width: 2, height: 3};
        let r2 = Rectangle {width: 4, height: 4};
        assert_ne!(r1, r2);
    }

    #[test]
    fn greeting_contains_name () {
        let name = "Carol";
        let result  = greeting(name);
        assert!(result.contains(name))
    }

    #[test]
    #[should_panic]
    fn greeting_no_name_contains_name () {
        let name = "Carol";
        let result  = greeting_no_name(name);
        assert!(result.contains(name))
    }

    #[test]
    #[should_panic  = "Value must be less than 101"]
    fn greater_than_100() {
        Guess::new(120);
    }

    //test should give results, does not need assert

    #[test]
    fn it_just_works() -> Result<(), String> {
        if 2+2 == 4 {Ok(())}
        else {
            Err(String::from("nope"))
        }
    }

// cargo test -- --help bin options
// cargo test -- --test-threads=1 : serial test
// cargo test -- --show-output : show output if successful test
// cargo test my_func : run test for my_func
// cargo test my :fun test for functions starting with my
// cargo test tests:: : run test for tests module

#[test]
#[ignore] //for expensive tests
fn expensive_test(){
    let duration = time::Duration::from_secs(5);
    sleep(duration);
    assert!(true)
}


}
