use adder;

mod common;

#[test]
fn it_adds(){
    assert_eq!(adder::add(2,3), 5);
}