use std::{cell::{Ref, RefCell}, fmt::Display, rc::{Rc, Weak}};


#[derive(Debug)]
enum List<T> {
    Cons(i32, RefCell<Rc<List<T>>>),
    Nil,
}

impl<T> List<T>{
    fn tail(&self) -> Option<&RefCell<Rc<List<T>>>> {
        match self {
            List::Cons(_, item) => Some(item),
            List::Nil => None,
        }
    }
}

//creating trees
#[derive(Debug)]
struct Node<T> {
    value: T,
    parent: RefCell<Weak<Node<T>>>,
    children:
    RefCell<
        Vec<
            Rc<
                Node<T>
            >
        >
    >,
}


fn main() {
    use List::*;
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil::<i32>))));

    println!("a initial rc count: {} ", Rc::strong_count(&a));
    println!("a next item: {:?} ", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count: {} ", Rc::strong_count(&a));
    println!("b rc count: {:?} ", Rc::strong_count(&b));
    println!("b next item: {:?} ", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
        
    }
    println!("a rc count: {} ", Rc::strong_count(&a));
    println!("b rc count: {:?} ", Rc::strong_count(&b));
    //create a ref cycle

    let leaf = Rc::new(Node{
        value : 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![])
    });

    println!("leaf parent : {:?}", leaf.parent.borrow().upgrade());
    println!("leaf strong : {}, weak: {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));

    let branch = Rc::new(Node{
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)])
    });
    
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent : {:?}", leaf.parent.borrow().upgrade());
    println!("leaf strong : {}, weak: {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
    println!("leaf strong : {}, weak: {}", Rc::strong_count(&branch), Rc::weak_count(&branch));



}
