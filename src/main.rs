use std::string::String;
use std::fmt::{Debug, Error};

fn main() {
    // println! is a macro that can accept a string literal ("" -> &str)
    println!("Hello, world!");

    // see some basic math
    basic_math();

    // test method will return a Some and print the result below
    let string = test(&String::from("me"))
        .expect("got an unexpected None value");
    println!("{}", string);

    let string = test(&String::from("mesmerizing"))
        .expect("got an unexpected None value");
    println!("{}", string);

    // this will return a None, which will cause expect to crash the app
    // let string = test(&String::from("")).expect("got a None value");

    // let's handle the Option differently
    let default = String::from("default for None");
    let another: &str = default.as_str();

    let res = test(&String::from(""));
    println!("Got a None? {}", res.is_none());
    println!("{}", res.unwrap_or(default.clone()));

    let res = test(&String::from("me"));
    println!("Got a None? {}", res.is_none());
    println!("{}", res.unwrap_or(default.clone()));
}

fn test(thing: &String) -> Option<String> {
    // simple condition - return result as a Some<String> with additional text
    if thing.starts_with("me") {
        return Some(String::from("you said ") + thing);
    }

    println!("you did not start with me so returning a None");
    return None;
}

pub trait FromFloat<T: num> {
    fn fromFloat(float_val: f64) -> Result<T, String>;
}

struct MyError(Error);

impl FromFloat<T> for T {
    fn fromFloat(float_val: f64) -> Result<T, String> {
        if float_val.round() != float_val {
            println!("value {:.64} is not an integer", float_val);
            return Err("hey i'm an error".to_string());
        }
        return Ok(float_val as T);
    }
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