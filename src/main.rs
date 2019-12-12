use std::string::String;
use std::fmt::{Debug, Error};

fn main() {
    // println! is a macro that can accept a string literal ("" -> &str)
    println!("Hello, world!");

    // see some basic math
    basic_math();

    // a primary allure of Rust is to avoid run-time errors that can be avoided by safe memory management and threading. 
    // just trying various things with variables, macros, errors, types, traits, conversions, etc.
    can_we_fail_runtime();
}

// basic math with type inference
fn basic_math() {
    let x = 3;
    let y = 4;
    let z = x + y;
    println!("z is {}", z);

    let float3 = 5.0;
    let int2 = i32::fromFloat(float3).expect("shoulda been an ok result");
    println!("int2 is {}", int2);

    let float = 4.1;
    // need to explicitly convert types
    let float2 = float + f64::from(x);
    println!("float2 is {}", float2);

    // this will cause a panic
//    let int = i32::fromFloat(float2).expect("shoulda been an ok result");
//    println!("int is {}", int);

    let res = i32::fromFloat(float2);
    match res {
        Ok(i) => println!("we got an ok result"),
        Err(e) => println!("we got an err result ... pure sadness")
    }
}

fn can_we_fail_runtime() {
        // test method will return a Some and print the result below
    let string = test(&String::from("me"))
        .expect("got an unexpected None value"); // doesn't fire or we crash
    println!("{}", string);

    let string = test(&String::from("mesmerizing"))
        .expect("got an unexpected None value");  // doesn't fire or we crash
    println!("{}", string);

    // this will return a None, which will cause expect to crash the app
    // let string = test(&String::from("")) // empty String is a String - no nulls here
    //      .expect("got a None value");

    // let's handle the Option that was returned differently
    let default = String::from("default for None");
    let another: &str = default.as_str();

    // this will return a None, so we print the default value
    let res = test(&String::from(""));
    println!("Got a None? {}", res.is_none());
    println!("{}", res.unwrap_or(default.clone())); // have to clone our default String to pass as an arg

    // this will return a Some, so we print the result
    let res = test(&String::from("me"));
    println!("Got a None? {}", res.is_none());
    println!("{}", res.unwrap_or(default.clone())); // have to clone our default String to pass as an arg
}


// This is a simple String comparison - successes return a transformed String, else we return a None
// Testing how Option and its Some/None types function in leui of a null pointer
fn test(thing: &String) -> Option<String> {
    // simple condition - return result as a Some<String> with additional text
    if thing.starts_with("me") {
        return Some(String::from("you said ") + thing);
    }

    println!("you did not start with me so returning a None");
    return None;
}

// I want to convert a float value to an integer in a special way, using a new Train to do so
pub trait FromFloat {
    fn fromFloat(float_val: f64) -> Result<i32, String>;
}

// here's the implementation - if the float is already an integer value, use a simple cast; else return an Err as a String
// - trying the Result type as a way to simulate how a recoverable error would occur in Rust
impl FromFloat for i32 {
    fn fromFloat(float_val: f64) -> Result<i32, String> {
        if float_val.round() != float_val {
            println!("value {:.64} is not an integer", float_val);
            return Err("hey i'm an error".to_string());
        }
        return Ok(float_val as i32); // 
    }
}

// Creating my own error type - TODO
struct MyError(Error);
