fn main() {
    println!("hi!, tell me about yourself");

    //memory safety: dumb and conservative
    //error free but hard to learn

    // fn a() {
    //     let x = "hello"; //pointer on stack, value in heap
    //     let y = 22;
    //     b()
    // }

    // fn b(){
    //     let x = String::from("world");
    // }

    // Ownership

    //I   Every value has a variable (owner)
    //II  There's one and only one owner
    //III If owner if not in scope -> drop

    {
        let s = String::from("hello");
        println!("{}",s);

    } //dropping s as no scope

    // let s1 = String::from("this is s1");
    // let s2 = s1; // TRASPASS OWNERSHIP!! (rule 2), it's a move

    // println!("{}, really", s2); //error, s2 is the owner.

    //solution, pass reference (&) or clone

    // let s = String::from("hello");
    // own(s);
    // println!("{}?", s); //ERROR -> some_str owns and out of scope

    // OWNERSHIP:: TEDIOUS, references!

    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    // println!("{} Length: {}", s1, len); // error, s in calculate lengh owns and dropped

    //PASSING REFERENCE: BORROWING

    println!("{} Length: {}", s1, len); // reference, not ownership passed, ok.

    let mut s2 = String::from("HI!");

    //only one mut ref

    let r_mut = &mut s2;

    // BORROWING RULES (on scope):

    // I   Many inmutable references
    // II  One Mutable reference
    // III Mutable xor inmutable

    change(r_mut);     // ENDS SCOPE OF MUT REFERENCE!!

    let r1 = &s2;
    let r2 = &s2;

    println!("{} {}", r1, r2);

    // Dangling: Pass reference of out of scope data

    // let ref_to_nothing = dangle();


    // let hello = &s[..5];
    // let world = &s[5..];

    // let word = first_word_bad(&s);


    let s5 = String::from("hello world this");
    let s6 = "hello world";

    let word = first_word(&s6);

    println!("{}", word)

    // s5.clear();
    // println!("{}", word);

}

fn calculate_length(s: &String) -> usize { //pass REFERENCE!
    let length = s.len();
    length
}
fn change(s: &mut String) { //pass mutable REFERENCE
    s.push_str(", hola");
}

// fn first_word_bad(s: &String) -> usize{
// //waht about second word?
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate(){
//         if item == b' ' {
//             return i //chops
//         }
//     }

//     s.len()
// }

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[..i] //chops
        }
    }
    
    // nothing to chop
    &s[..]
    }
    


// dangle gives oos data
// fn dangle() -> &String {
//     let s = String::from("h");
//     &s
// }

// fn own(some_str:String){
//     println!("{}", some_str)
// }