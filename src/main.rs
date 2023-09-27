#![allow(dead_code, unused_imports)]

use std::cmp::Ordering;
use std::error::Error as ErrorTrait;
use std::fs::{ File, self };
use std::io::{ Error as IOError, Read };
use std::process;
use colored::Colorize;
use thiserror::Error;

fn first() {
    println!("Hello, world!");
}

fn main() {
    //first();
    //second();
    match outer() {
        Ok(_) => {
            let msg = format!("Yay: success!!");
            println!("\n{}\n", msg.green().bold())
        }
        Err(e) => {
            let msg = format!("Failure: {}", e);
            eprintln!("\n{}\n", msg.red());
            process::exit(1);
        }
    }
}

fn outer() -> Result<(), Box<dyn ErrorTrait>> {
    // panics
    //let _a = step1();
    //let _a = step1_1();
    //let _a = step1_2();

    //results
    let _a = step2();
    //let _a = step3()?;

    //options
    let _a = step4();    
    let _a = step4_1();
    match _a {
        Some(_) => Ok(()),
        None => panic!("no file"),
    }

}

//------------------------------------------------------------------------------
//                          1. non-recoverable errors
//------------------------------------------------------------------------------

fn second() {
    panic!("crash and burn");
}

// function signature useless when panic is used
fn step1() -> File {
    let greeting_file_result = File::open("hello_not_there.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    greeting_file
}

// syntactic sugar for step1, using unwrap
fn step1_1() -> File {
    File::open("hello_not_there.txt").unwrap()
}

// syntactic sugar for step1, using expect to map to a different string
fn step1_2() -> File {
    File::open("hello_not_there.txt").expect("crashed here")
}

//------------------------------------------------------------------------------
//                              2. recoverable errors
//------------------------------------------------------------------------------

/* this is the definition of a recoverable error, but we don't need to define it as its part of the standard library 
    
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }

*/

// this is a recoverable error
fn step2() -> Result<File, IOError> {
    let greeting_file_result = File::open("hello.txt");
    //println!("this is the file content: {:?}", greeting_file_result);
    greeting_file_result
}

//------------------------------------------------------------------------------
//                    3. multiple ways to fail & error mapping
//------------------------------------------------------------------------------

#[derive(Error, Debug)]
enum Failure {
    #[error("Content not found")] ContentNotFound,
    #[error("To few lines, found {0} lines")] ToFewLines(usize),
    #[error("To many lines, found {0} lines")] ToManyLines(usize),
    #[error("Invalid line: {0} at {1}")] InvalidLine(usize, String),
}

fn step3() -> Result<String, Failure> {
    // this demonstrates mapping errors between layers of abstraction
    let expected = 3;
    let contents = fs::read_to_string("hello.txt").map_err(|_| Failure::ContentNotFound)?;
    let count = contents.lines().count();
    // here we use shadowing we are know to have a valid value now
    let contents = match count.cmp(&expected) {
        Ordering::Less => {
            return Err(Failure::ToFewLines(count));
        }
        Ordering::Greater => {
            return Err(Failure::ToManyLines(count));
        }
        Ordering::Equal => contents,
    };

    for entry in contents.lines().enumerate() {
        let (index, line) = entry;
        if line.contains("bad") {
            return Err(Failure::InvalidLine(index, line.to_string()));
        }
    }

    Ok(contents)
}

//------------------------------------------------------------------------------
//                  4. squashing errors with optional results
//------------------------------------------------------------------------------

/* this is rust's answer to forcing you to handle null/undefined values. Option is part of standard library/prelude

    enum Option<T> {
        Some(T),
        None,
    }

*/
// just ignore the error, but forces the caller to handle the unpacking, so they are forced to deal with all branches
fn step4() -> Option<File> {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => Some(file),
        Err(_) => None,
    };

    greeting_file
}

// return the option and force the caller to deal with it as they see fit
fn step4_1() -> Option<File> {
    File::open("hello.txt").ok()
}
