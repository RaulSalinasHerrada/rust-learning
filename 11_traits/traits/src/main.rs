use std::{fmt::{Debug, Display}, iter::Sum};


pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

pub struct TV{
    pub content: String
}

// ie: For every implementation of trait Summary, it
//should have this implementation giving String

pub trait Summary {
    
    fn summarise_author(&self) -> String{
        String::from ("Unknown author")
    }    // we create the signature, not body
    //default implementation for structs implementing trait
    fn summarise_content(&self) -> String{
        String::from("lorem ipsum")
    }
    
    fn summarise(&self) -> String{
        format!("{}: {}", self.summarise_author(), self.summarise_content())
    }
}

impl Summary for NewsArticle{
    
    fn summarise_author(&self) -> String {
        format!("{}", &self.author)
    }
    fn summarise_content(&self) -> String {
        format!("{}", &self.headline)
    }
    fn summarise(&self) -> String {
        format!("{}, by {}", &self.headline, &self.author)
    }
}

impl Summary for Tweet {
    fn summarise_content(&self) -> String {
        String::from(&self.content)
    }

    fn summarise_author(&self) -> String {
        format!("@{}",String::from(&self.username))
    }
    fn summarise(&self) -> String {
        format!("{}: {}", &self.username, &self.content)
    }
}

impl Summary for TV {}

//methods giving traits (implemented on strucs that implement certain trait)


//trait bound. T implements summary, gives more livery
pub fn notify <T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarise());
}

//both times
pub fn notify_two <T: Summary + Display>(item1: &T, item2: &T) {
    println!("Breaking news! {} and {}", item1.summarise(), item2.summarise());
}

//don't go to far, use where

fn get_me<T,U>(t: &T, u: &U)-> ()
where T: Display + Clone, U: Clone + Debug
{
    
}

//specity function output on Structs that implement Traits
//can only return ONLY one type (not any type that implements Summary)
fn return_summarisable() -> impl Summary {
    TV{content : String::from("hola")}

}

// Trait bounds to conditionally implement methods


struct Pair<T> {
    x:T,
    y:T
}
//implemented for all types
impl<T> Pair<T> {
    fn new(x: T, y:T) -> Self {
        Self{x,y}
    }
}

//
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display (&self) {
        if self.x >= self.y {
            println!("Largest member is x: {}", self.x);
        } else{
            println!("Largest member is y: {}", self.y)
        } 
    }
}

//linking implementation, implementing trait from other traits!

// impl <S: Display> ToString for S 


//task. create summaries. create method shared bet
fn main() {

    let news = NewsArticle{
        author: String::from("Alice"),
        headline: String::from("Dog eats treats"),
        content: String::from("Dog likes good treats")

    };

    let tweet = Tweet{
        username : String::from("@alice"),
        content: String::from("we wanna get busy"),
        reply : false,
        retweet: false,
    };

    let tv = TV{content : String::new()};
    println!("{}",news.summarise());
    println!("{}",tweet.summarise());
    println!("{}",tv.summarise());

    notify(&news); //pass by reference

    let summarisable = return_summarisable();
    println!("{}",summarisable.summarise());

    let pair = Pair {
        x : String::from("hello"), y:String::from("world")
    };

    pair.cmp_display()

}

// dhis does not create overhead, as the compiler specialises for you