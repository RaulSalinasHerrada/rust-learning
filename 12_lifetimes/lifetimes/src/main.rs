use std::fmt::Display;
// if we want to used references on struct, we have to annotate references
struct ImportantExcerpt<'a> {
    part: &'a str,
}

trait Publish {
    fn publish(&self) -> ();
}

impl Publish for ImportantExcerpt<'_> {
    fn publish(&self) -> () {
        println!("This is an excerpt: {}",self.part)
    }
}

impl<'a> ImportantExcerpt<'a> {
    fn return_part(&self, announcement: &str) -> &str {
        println!("Important: {}", announcement);
        self.part
    }
}

fn main() {

    //dangling references (out of scope... sort of)

    let x = 5;
    let r =&x;

    // {
    //     let x = 5;
    //     r = &x; //the element x get dropped
    // }

    println!("r: {}", r);

    let string1 = String::from("abc");
    let string2 = String::from("wxyz");

    let result = longest(string1.as_str(), string2.as_str());
    println!("Longest: {}", result); //life time to the smallest passed


    let string3 = String::from("abc");
    // let result_too_long: &str;

    {
        let string4 = String::from("ijkl");
        let result = longest(string3.as_str(), string4.as_str());
        // result_too_long = longest(string3.as_str(), string4.as_str()); //as result will be used after it is error
        println!("longest 34 {}", result); //lifetime is string4 (valid)
    }

    // println!("Longest {}", result_too_long);
    //if result is called after, the lifetime is not long enough

    //what about elements created within the function itself. Return owned type

    let hello = return_hello();

    let novel = String::from("Call me Isha. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("not message");

    let i = ImportantExcerpt{ part : first_sentence};
    i.publish();
    i.return_part("this is my part");

    let fw = first_word("my world is here");

    //special lifetime: static: lives forever

    let s: &'static str = "I will love forever";

    let long = longest_with_announcement(
        "hello", "world", "anny");




}
//String is an owned type, which, instead of &str "hello", transfer ownership
fn return_hello<'a> () -> String{
    let result = String::from("hello");
    result
}
//not neccesary the annotate the lifetimes as it can be infered given these rules


// I    Each parameter gets own lifetime
// II   If there's 1 and only 1 input, that lifetime is assigned to all output

// III  If there's multiple input parameters, but one is &(mut)self, the self
//      lifetime is assigned!

//checks II rule, annotation not needed
fn first_word<'a> (s: &'a str) -> &'a str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}


fn longest<'a, T>(x : &'a T, y: &'a T) -> &'a T 
where T: PartialOrd +?Sized { 
    if x > y {
        x
    } else {
        y
    }
}

//put everything together! 

fn longest_with_announcement<'a,U,T>(
    x: &'a U,
    y: &'a U,
    ann: T) -> &'a U
    where T: Display, U: PartialOrd + Display + ?Sized{
        println! ("Announcement {}", ann);
    
    if x > y {
        x
    } else {y}

    }
