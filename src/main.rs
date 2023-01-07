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

// fn main(){
//     const ONE_MIL:u32=1_000_000;
//     const PI:f32=3.1415;
//     let age:&str="47";
//     let mut age:u32=age.trim().parse()
//         .expect("Age wasnt assigned number");
//     age=age+1;
//     println!("I'm {} and I want ${}",age,ONE_MIL);
// }
// function ke 2


// fn main(){
//     println!("Max u32:{}",u32::MAX);
//     println!("Max u64:{}",u64::MAX);
//     println!("Max usize:{}",usize::MAX);
//     println!("Max u128:{}",u128::MAX);
//     println!("Max f32:{}",f32::MAX);
//     println!("Max f64:{}",f64::MAX);
// }
//function ke 3

// fn main(){
//     let num_1:f32=1.111111111111111;
//     println!("f32:{}",num_1+0.111111111111111);
//     let num_2:f64=1.111111111111111;
//     println!("f64:{}",num_2+0.111111111111111);
// }
//function ke 4


fn main(){
    let num_3:u32=5;
    let num_4:u32=4;

    println!("5 + 4 = {}",num_3+num_4);
    println!("5 - 4 = {}",num_3-num_4);
    println!("5 X 4 = {}",num_3*num_4);
    println!("5 / 4 = {}",num_3/num_4);
    println!("5 % 4 = {}",num_3%num_4);
    
}
//function ke 5