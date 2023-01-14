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
//     let age:&str="36";
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


// fn main(){
//     let num_3:u32=5;
//     let num_4:u32=4;

//     println!("5 + 4 = {}",num_3+num_4);
//     println!("5 - 4 = {}",num_3-num_4);
//     println!("5 X 4 = {}",num_3*num_4);
//     println!("5 / 4 = {}",num_3/num_4);
//     println!("5 % 4 = {}",num_3%num_4);
    
// }
//function ke 5 operasi  matematik

// fn main(){
//     let random_num=rand::thread_rng().gen_range(1..101);
//     println!("Random:{}",random_num)
// }

//function cari angka random 1 sampai 100

// fn main(){
//     let age:i32=8;
//     if (age>=1)&&(age<=18){
//         println!("Important birthday");
//     }else if (age==21)||(age==50) {
//         println!("Important Birthday")        
//     }else if age>=65 {
//         println!("important Birthday")
        
//     }else {
//         println!("Not an important birthday")
//     }
// }
// function if

// fn main(){
//     let mut my_age:i32=47;
//     let can_vote=if my_age>=18{
//         true
//     }else{
//         false
//     };
//     println!("can vote:{}",can_vote)
// }
//functional conditional if

// fn main(){
//     let age2=30;
//     match age2 {
//         1..=18=>println!("important birtday"),
//         21|50=>println!("important birthday"),
//         65..=i32::MAX=>println!("important birthday"),
//         _ =>println!("not an important birthday"),

//     };
// }

//functional match conditional

// fn main(){
//     let my_age=20;
//     let voting_age=18;
//     match my_age.cmp(&voting_age){
//         Ordering::Less=>println!("Anda Belum Bisa Memilih !"),
//         Ordering::Greater=>println!("Anda Bisa Memilih"),
//         Ordering::Equal=>println!("Waktu yang mantap untuk pilih"),
//     };
// }
// functional match compare

// fn main(){
//     let arr_1=[1,2,3,4,5,6,7,8,9];
//     println!("1st:{}",arr_1[0]);
//     println!("2nd:{}",arr_1[1]);
//     println!("lenght:{}",arr_1.len());

// }
// functional call the array

// fn main(){
//     let arr_2=[1,2,3,4,5,6,7,8,9];
//     let mut loop_idx=0;
//     loop{
//         if arr_2[loop_idx] %2 == 0{
//             loop_idx += 1;
//             continue;
//         }
//         if arr_2[loop_idx] == 9{
//             break;
//         }
//         println!("val : {}",arr_2[loop_idx]);
//         loop_idx += 1;
//     }
// }
// function loop increment

// fn main(){
//     let arr_2=[1,2,3,4,5,6,7,8,9];
//     let mut loop_idx=0;
//     for val in arr_2.iter(){
//         println!("Val : {}",val)
//     }
// }
// loop for

// fn main(){
//     let arr_2=[1,2,3,4,5,6,7,8,9];
//     let mut loop_idx=0; 
//     while loop_idx<arr_2.len(){
//         println!("Arr : {}",arr_2[loop_idx]);
//         loop_idx +=1;
//     }
// }
//while loop

// tuple

// fn main(){
//    let my_yuple:(u8,String,f64)=(37,"Gerry".to_string(),50_000.00);
//    println!("Name : {}",my_yuple.1);
//    let (v1,v2,v3)=my_yuple;
//    println!("Age : {}",v1);
//    println!("Expend : {}",v3);
// }

// tuple


fn main(){
    let mut st1=String::new();
    st1.push('A');
    st1.push_str("word");
    for word in st1.split_whitespace(){
        println!("{}",word);
    }
    let st2=st1.replace("A", "Another One"); 
    println!("{}",st2)

}