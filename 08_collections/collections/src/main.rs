use unicode_segmentation::UnicodeSegmentation;
use std::collections::HashMap;
fn main() {
    let a = [1,2,3]; //array fixed length
    let mut v:Vec<i32> = Vec::new(); //creates mt vector, specify type. can grow

    //vectors are in heap

    v.push(1);
    v.push(4);
    v.push(2);
    {
        let v2 = vec![1,2,3]; //type can be infered from macro
    } //will be drpped from scope

    //access, index

    let mut w = vec![1,2,4,8];
    let rd = &w[2];

    // w.push(16); //rd is non-mutable reference (no mutable and inmutable at the same) 
    //mutable because we allocate memory and move memory, pointing to something else

    //RUNTIME Error by bounds
    println!("Third elemnt is: {}", rd);

    match w.get(20) {
        Some(rd)  => println!("3rd is: {}", rd),
        _ => println!("Nope") //_ is placeholder 
    }
    match w.get(2) {
        Some(rd)  => println!("3rd is: {}", rd),
        _ => println!("Nope") //_ is placeholder 
    }

    let mut v = vec![0,1,4,16];

    for i in &mut v{
        *i += 50; //de-referencing op, to get the underlying value
        println!("big numbers!, {}", i)
    }

    //can store enums, helps to store more complex, rich data

    enum SpreadsheetCell{
        Int(i32),
        Float(f64),
        Text(String)
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.2)
    ];

    match row.get(0) {
        Some(SpreadsheetCell::Int(i)) => println!("number {}", i),
        None => println!("Out of bounds"),
        _ => println!("Not a number")
    } //match with placeholders and None :D

    // strings are tough in rust!! =(
    //strings are utf-8 bytes, we see it as letters,
    // when they are 0,1s

    //ASCII: fixed width (byte), not many things. Problems for languages
    //UTF-8: languages, emoji, backwards to ASCII (256 are ASCII)
    //variable width, different size!

    let s2= "initial contents";
    let s3 = s2.to_string();
    let s4= String::from("initial contents");
    let eq24 = s2 == s4; //FALSE!!
    println!("{}", eq24);

    //String can grow as vecs

    let mut smut = String::from("foo");
    smut.push_str("bar");
    smut.push('!');

    println!("{}",smut);

    // let s_rule = ;
    smut = smut + &" rules!"; //&str or String ))
    println!("{}",smut);

    let s_format = format!("{}{}", smut, s3);
    println!("with formats {}",s_format);

    let hello = String::from("Hello");
    // can't index by byte, as can be more than a 2 byte char
    //rep, bytes, scalar, graphemes (what do we want?)

    for (i, b) in "नमस्कार".bytes().enumerate(){
        println!("{} as bytes, {}", i,b) //20 bytes!
    }

    for (i, c) in "नमस्कार".chars().enumerate(){
        println!("{} as chars, {}", i,c) //7 chars
    }

    //to iterate over graphemes, need unicde segmentation

    for (i, g) in "नमस्कार".graphemes(true).enumerate(){
        println!("{} as graphemes, {}", i,g) //5 graphemes
    }

    //hashmaps: key val pairs

    enum Teams {
        Blue,
        Red
    }

    let blue = String::from("blue");
    let yellow = String::from("yellow");

    let mut scores = HashMap::new();

    scores.insert(blue, 10); //moves ownership!!
    scores.insert(yellow, 50);

    let home = String::from("blue");

    let home_score =scores.get(&home); //option as we can't say if there's

    for (key, val) in &scores {
        println!("{}: {}", key, val);
    }
    // println!("{}", blue); //nope

    scores.insert(String::from("blue"), 80); //overrides
    scores.entry(String::from("yellow")).or_insert(30); //if there's do nothing. else put on 30 
    scores.entry(String::from("red")).or_insert(30); //if there's do nothing. else put on 30 

    println!("{:?}", scores);

    let text = "hello world dear world";

    let mut counters = HashMap::new();

    for word in text.split_whitespace() {
        // or insert returns a mutable ref to the value!
        let count = counters.entry(word).or_insert(0); //populates, start with 0
        *count += 1; // the value itself is updated per word via dereferencing
    }

    println!("See the words: {:?}", counters);





















}
