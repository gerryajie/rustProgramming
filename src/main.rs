#![allow(unused)]


use std::io;
use rand::Rng;
use std::io::{Write,BufReader,BufRead,ErrorKind};
use std::fs::File;
use std::cmp::Ordering;


// fn main() {
//     println!("What is your name");
//     let mut name=String::new();
//     let greeting:&str="Nice to see u";
//     io::stdin().read_line(&mut name)
//         .expect("Didnot Receive Input");                 

//     println!("One Rust for everybody enjoy {}!{}",name.trim_end(),greeting);
// }
// Function ke 1

fn main(){
    const ONE_MIL:u32=1_000_000;
    const PI:f32=3.1415;
    let age:&str="47";
    let mut age:u32=age.trim().parse()
        .expect("Age wasnt assigned number");
    age=age+1;
    println!("I'm {} and I want ${}",age,ONE_MIL);
}
// function ke 2