
// #[derive(Debug)]
// struct ImportantExcerpt<'a> {
//     part: &'a str,
// }

// fn main() {
//     let novel = String::from("Call me Ishmael. Some years ago...");
//     let first_sentence = novel.split('.') // split function broke at point .
//         .next()
//         .expect("Could not find a '.'");
//     let i = ImportantExcerpt { part: first_sentence };
//     println!(" {} \n {} \n {:?} ", novel, first_sentence, i);
// }




// #[derive(Debug)]
// struct Parts_of_novel<'a> {
//     part: &'a str,
// }

// use std::io;
// fn main() {
//     // let novel = String::from("Call me Ishmael. Some years ago...");
//     // let first_sentence = novel.split("years") // split function broke at point .
//     //     .next()
//     //     .expect("Could not find a '.'");
//     // let i = Parts_of_novel { part: first_sentence };
//     // println!(" {} \n {} \n {:?} ", novel, first_sentence, i);


// }



// #![allow(unused_variables)]
// fn main() {
// fn first_word(s: &String) -> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }

//     &s[..]
// }

// let data = "Hello World.".to_string();
// println!("{}",first_word(&data));
// }

// use std::thread;
// use std::time::Duration;

// fn simulated_expensive_calculation(intensity: u32) -> u32 {
//     println!("calculating slowly...");
//     thread::sleep(Duration::from_secs(2));
//     intensity
// }

// fn generate_workout(intensity: u32, random_number: u32) {
//     if intensity < 25 {
//         println!(
//             "Today, do {} pushups!",
//             simulated_expensive_calculation(intensity)
//         );
//         println!(
//             "Next, do {} situps!",
//             simulated_expensive_calculation(intensity)
//         );
//     } else {
//         if random_number == 3 {
//             println!("Take a break today! Remember to stay hydrated!");
//         } else {
//             println!(
//                 "Today, run for {} minutes!",
//                 simulated_expensive_calculation(intensity)
//             );
//         }
//     }
// }

// fn main() {
//     let simulated_user_specified_value = 10;
//     let simulated_random_number = 7;

//     generate_workout(
//         simulated_user_specified_value,
//         simulated_random_number
//     );
// }



/////////////////////////////////////////////////////////////////////////////////////////////

// use std::thread;
// use std::time::Duration;

// fn simulated_expensive_calculation(intensity: u32) -> u32 {
//     println!("calculating slowly...");
//     thread::sleep(Duration::from_secs(2));
//     intensity
// }

// fn generate_workout(intensity: u32, random_number: u32) {

// let expensive_closure = |num| {
//     println!("calculating slowly...");
//     thread::sleep(Duration::from_secs(2));
//     num
// };

//     if intensity < 25 {
//         println!(
//             "Today, do {} pushups!",
//             expensive_closure(intensity)
//         );
//         println!(
//             "Next, do {} situps!",
//             expensive_closure(intensity)
//         );
//     } else {
//         if random_number == 3 {
//             println!("Take a break today! Remember to stay hydrated!");
//         } else {
//             println!(
//                 "Today, run for {} minutes!",
//                 expensive_closure(intensity)
//             );
//         }
//     }
// }

// fn main() {
//     let simulated_user_specified_value = 10;
//     let simulated_random_number = 7;

//     generate_workout(
//         simulated_user_specified_value,
//         simulated_random_number
//     );
// }



// fn main(){
//     let add_numbers = |num_01,num_02|{
//         num_01+num_02
//     };
//     println!("{}",add_numbers(24,24));
// }

///////////////////////////////////////Task////////////////////////////////////////////

// Write a program in which the closure name is calculation and having three parameters 
// num_01, num_02 and operator

// use std::io;
// use std::io::Read;
// use std::ops::Sub;
// //use std::fs::{self, File};
// fn main(){

//             println!("Enter first number \n ");
//             let mut first_number = String::new();
//             io::stdin().read_line(&mut first_number);
//             let mut first_number = first_number.trim();
//             let my_int = first_number.parse::<i32>().unwrap();
        
//             println!("Enter file data \n");
//             let mut second_number = String::new();
//             io::stdin().read_line(&mut second_number);
//             let mut second_number = second_number.trim();
//     let my_int = first_number.parse::<i32>().unwrap();
        
//             println!("{:?}", first_number.to_owned()-second_number);
// }


// extern crate text_io;
// fn main(){
//     println!("Enter the first number");
//     let num_01 = read!();
//         println!("Enter the second number");
//     let num_02 = read!();
//         println!("Enter the operator");
//     let operator = read!();
    
//     let calculation = |num_01,num_02,operator|{
//         if let Some(
            
//             else

//         ) = 
        
//         else
//          {
            
            
//             else
//             .
//         }
//     }
//     let result = calculation(num_01,num_02,operator);
//     println!("{}", result);
// }



// #[macro_use]
// extern crate text_io;
// fn main(){
//     println!("Enter the first number");
//     let num_01 = read!();
//         println!("Enter the second number");
//     let num_02 = read!();
//         println!("Enter the operator");
//     let operator = read!();
    
//     let calculation = |num_01,num_02,operator|{
//         match operator {
//             Some("+") => num_01+num_02,
//             Some("-") => num_01-num_02,
//             Some("*") => num_01*num_02,
//             Some("/") => num_01/num_02,
//             None => 0,
//         }
    
//     };
//     let result = calculation(num_01,num_02,operator);
//     println!("{}", result);
// }



////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
    

struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            },
        }
    }
}

fn main() {
let mut c = Cacher::new(|a| a);
let v = c.value(20);
println!("{}",v);
}