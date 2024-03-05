//iterators can be implemented on every struct!

// pub trait Iterator{
//     type Item; //need to define Item type

//     fn next(&mut self) -> Option<Self::Item>; //calling next change it
// }

#[test]
fn iterator_demo(){

    let v1 = vec![2,4,6];

    let mut v1_iter = v1.iter(); //next its mutable ref

    assert_eq!(v1_iter.next(), Some(&2)); //next pass option?
    assert_eq!(v1_iter.next(), Some(&4));
    assert_eq!(v1_iter.next(), Some(&6));
    assert_eq!(v1_iter.next(), None); //end of seq

    //iter gives inmutable refs
    //iter_mut() mutable references
    //into_iter() owned types

}
//default implemetations: sum, etc

#[test]
fn iterator_impl(){

    let v1 = vec![2,4,6];

    let v1_iter = v1.iter(); //next its mutable ref

    assert_eq!(v1_iter.sum::<i32>(), 12);

    let v2: Vec<i32>= v1.iter().map(|x| x+1).collect();

    //iterators are lazy, it will run when we use it

    assert_eq!(v2, vec![3,5,7]);
}
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u8, 
    style: String
}

fn shoes_in_my_size(
    shoes:Vec<Shoe>,

    shoe_size: u8) -> Vec<Shoe> {
        shoes
        .into_iter()
        .filter(|s| s.size == shoe_size) //capture environment!
        .collect()
    
}
#[test]
fn filter_by_size(){

    let shoes = vec![
        Shoe {
            size : 10,
            style: String::from("sneaker")
        },
        Shoe {
            size : 13,
            style: String::from("sandal")
        },
        Shoe {
            size : 10,
            style: String::from("boot")
        },
    ];

    let good_shoes = vec![
        Shoe {
            size : 13,
            style: String::from("sandal")
        }
    ];

    assert_eq!(shoes_in_my_size(shoes, 13), good_shoes)

}


//implement interator


struct Counter {
    count:u8,
    pub max_value: u8
}

impl Counter{
    pub fn new(max_value: u8) -> Counter {
        Counter{
            count: 0,
            max_value
        }
    }
}

impl Iterator for Counter {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 { //goes until 5
            self.count += 1;
            Some(self.count)
        } else { //stop iteration
            None
        }
    }
}

#[test]
fn calling_next_counter(){
    let mut counter = Counter::new(5);

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

#[test]
fn using_chains() {
    let max_value = 5;
    let sum:u8  = Counter::new(max_value)
    .zip(Counter::new(max_value).skip(1))
    .map(|(a,b)| a*b)
    .filter(|x| x % 3 == 0)
    .sum()
    ;

    assert_eq!(sum, 18);
}



fn main() {
    let v1 = vec![1,2,4];

    let v1_iter = v1.iter(); //lazy

    for v in v1_iter {
        println!("Got {}", v);
    } 
}
