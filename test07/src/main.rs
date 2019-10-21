// fn main() {
//     let mut s = String::from("Hello World");
//     println!("{}",s);
//     let s_change = change_value(s);
//     println!("{}",s_change);
//     println!("{}",s);
    
// }
// fn change_value(mut s: String) -> String{
//     s.push_str("From IOT Class");
//     s
// }

//////////////////////////////////////////////////////////////////////////////////////////////////////////
//make a function that compare two strings length and the print the higher length string

// fn main(){
//     let s_one = String::from("Hello");
//     let s_two = String::from("World!");
// //    let comp_value = comp_string(s_one, s_two);
// //    println!("Greater length String is {}",comp_value);
//     println!("{}",comp_string(s_one,s_two));
// }
// fn comp_string(s_one:String , s_two:String) -> String {
//     if s_one.len() > s_two.len() {
//         s_one
//     } else {
//         s_two
//     }
// }
////////////////////////////////////////////////////////////////////////////////////////////////////

// use std::io;

// fn main(){
//   //  let s_one = String::from("Hello");
//   //  let s_two = String::from("World!");
//     let mut s_one = String::new();
//     let mut s_two = String::new();
//     println!("Enter the 1st String");
//     io::stdin().read_line(&mut s_one);
//     println!("Enter the 2nd String");
//     io::stdin().read_line(&mut s_two);
    
// //    let comp_value = comp_string(s_one, s_two);
// //    println!("Greater length String is {}",comp_value);
//     println!("The max length string is {}",comp_string(s_one,s_two));
// }
// fn comp_string(s_one:String , s_two:String) -> String {
//     if s_one.len() > s_two.len() {
//         s_one
//     } else {
//         s_two
//     }
// }

///////////////////////////////////////////////////////////////////////////////////////
//Take two string as dynamic input and merge into one string
// use std::io;

// fn main(){
//   //  let s_one = String::from("Hello");
//   //  let s_two = String::from("World!");
//     let mut s_one = String::new();
//     let mut s_two = String::new();
//     println!("Enter the 1st String");
//     io::stdin().read_line(&mut s_one);
//     println!("Enter the 2nd String");
//     io::stdin().read_line(&mut s_two);
//     s_one.push_str(&s_two);
    
// //    let comp_value = comp_string(s_one, s_two);
// //    println!("Greater length String is {}",comp_value);
//     println!("Full Name is {}",s_one);
// }

/////////////////////////////////////////////////////////////
// write a program that takes one string as parameter and print
// first half and second half of the string that you given as input

// use std::io;

// fn main(){
//   let mut s_one = String::new();
//   println!("Enter the 1st String");
//   io::stdin().read_line(&mut s_one);
//   let x = s_one.len() / 2;
//   let slice_one = &s_one[..x];
//   let slice_two = &s_one[x..];
//   println!("First half is: {}  , Second half is: {}",slice_one,slice_two);
// }
use std::io;

fn main(){
  let mut s_one = String::new();
  println!("Enter the 1st String");
  io::stdin().read_line(&mut s_one);
  let result = break_into_two_strings(s_one);
  println!("This is 1st half: {}, This is 2nd half: {}",result.0,result.1);
}

fn break_into_two_strings(s:String)->(String,String)
{
  let length = s.len();
  let first_half = &s[0..(length/2)-1];
  let second_half = &s[(length/2)-1..];
  (first_half.to_string(),second_half.to_string())
}
