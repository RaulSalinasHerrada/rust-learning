// pointer: More general than reference, goes to memory access
//smart pointer: data structure. Pointer + metadata (overhead)
// ej: reference counting. Cleans. 
// s pnt owned data
// Strings and vec are smart pointers. Extra capacibities.
// Deref: instances to be treated as ref or.
//drop: what to do when it's out of scope.

//EXAMPLE

//using box, instead of point to an arbitrary large list,
//it stores a pointer into the heap
enum List {
    Cons(i32,  Box<List>),
    Nil, //end of list
}

//memory is bounded by the biggest element on the enum
enum Message{
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    Change(i32, i32, i32)
}


use std::{cell::RefCell, fmt::Debug, ops::Deref, rc::Rc};

struct MyBox<T>(T);

impl <T> MyBox<T> {
    fn new(x: T) -> MyBox<T>{
        MyBox(x)
    }
}


impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize, 
    max: usize,
}

impl <'a,T> LimitTracker<'a,T> where T: Messenger {
    
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value : 0,
            max
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let max_percent = self.value as f64 / self.max as f64;

        if max_percent >= 1.0 {
            self.messenger.send("Error. Quota reached")
        } else if max_percent >= 0.9 {
            self.messenger.send("WARNING, quota over 90%!")
        } else if max_percent >= 0.75 {
            self.messenger.send("Warning, quota over 75%")
        }
    }
}


//Rc and refcels 
#[derive(Debug)]
enum SmartList<T> {
    Cons(Rc<RefCell<T>>, Rc<SmartList<T>>),
    Nil,
}


fn main() {
    
    // pass what it's going into the heap. stack is b
    // put some bounds onto to the memory
    {
    let b = Box::new(5); 
    println!("b = {}", b);

    //after this, the scack and heap will be dealocated
    }

    {
        let list = List::Cons(1, Box::new(
            List::Cons(2, Box::new(
                List::Cons(4, Box::new(List::Nil))))));
    }

    {

        let x = 5;
        let y = MyBox::new(x);

        assert_eq!(x, *y); //takes away the the reference
        // y now have a copy of x, as x is primitive
        //for box need the deref trait
        //return reference for not taking ownership

        //deref coercion convert from a type to another type
        let m = MyBox::new(String::from("Rust"));
        hello(&m) ;//works, even if we passing ref Mybox instead of str
        //&MyBox<String> -> &String -> &str
        hello(&(*m)[..]) ; //equivalent
    }
    {
        struct CustomPointer {
            data: String
        }
        
        impl CustomPointer{
            fn new(data:String) -> CustomPointer {CustomPointer{data}}
        }

        impl Drop for CustomPointer {
            fn drop(&mut self) {
                println!("Dropping CustomPointer. data: {}", self.data);
            }
        }

        let c = CustomPointer::new(String::from("data 1"));

        let d = CustomPointer {data: String::from("data 2")};
        drop(d);

        println!("CustomPointers, created.")
        
    }

    {
    //reference counters: Single value has many owners, when is dropped?
    // tv in a living room. Many come and start watching
    // the last turns it of
    // useful on singlethread

     //cons list with shared tail

    //----\
    //     +--------
    //----/

        //solution lifetimes. but nil will be problem
        
        use std::rc::Rc;

        enum ListShared<T> {
            Cons(T, Rc<ListShared<T>>),
            Nil,
        }

        
        let tail = Rc::new(ListShared::Cons(
            5, Rc::new(ListShared::Cons(
                10, Rc::new(ListShared::Nil)))));

        let head1 = ListShared::Cons(1, Rc::clone(&tail));
                println!("References couting tail: {}", Rc::strong_count(&tail)); //2;
        //head1 moves value, with problems of ownership on head2
        let head2 = ListShared::Cons(1, Rc::clone(&tail));
        //here cloens DOES NOT DEEP COPY, but +1 the references
        println!("References counting tail: {}", Rc::strong_count(&tail)) //3;

    
    }

    {
        //how enforce burrowing rules on runtime and not compile time
        //RefCell: enforces at runtime. (errors are caught later, slower to dev)
        //some problems can't be solved on compiletime
        // RefCell cna mutate value insude even if the refcell is immutable. 
        //interior mutability patern

        let a = 5;
        //can't be mutated s the value it references is inmutable.
        /*
        let b = &mut a;
        */
        let mut c = 10;
        let d = &c;
        //the reference is inmutable even if the c itself is mutable
        /*
        *d = 20;
        */
    


        //the faculty of sending message


    }
    {

    let value = Rc::new(RefCell::new(5));
    let tl = Rc::new(
        SmartList::Cons(Rc::clone(&value),Rc::new(SmartList::Nil)));

    let b = SmartList::Cons(
        Rc::new(RefCell::new(4)), Rc::clone(&tl));
    
    let c = SmartList::Cons(
        Rc::new(RefCell::new(13)), Rc::clone(&tl));

    *value.borrow_mut() += 10;

    println!("tl after = {:?}", tl);
    println!("a after = {:?}", b);
    println!("b after = {:?}", c);

}


}

fn hello(name: &str)  {
    println!("Hello ,{}", name)
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use super::*;
    

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>
    }

    impl MockMessenger {
        fn new() -> Self {
            MockMessenger {
                sent_messages : RefCell::new(vec![])
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message))
        }
    }

    #[test]
    fn it_sends_75() {
        let mut mock_msg = MockMessenger::new();
        let mut limit_trk = LimitTracker::new(&mock_msg, 100);

        limit_trk.set_value(100);

        assert_eq!(mock_msg.sent_messages.borrow().len(), 1);
    }



}
