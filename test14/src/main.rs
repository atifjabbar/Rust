// // fn main() {
    
// //     let mut arr = [3;5];
// //     arr[2] = 6;
// //     println!("{:?}",arr);
    
// //     let mut vector_01 : Vec<i32> = Vec::new();
// //     let vector_02 = vec!["Faheem","Ibad","Zain"];
// //     let mut vector_03 = vec![121,3243,534,232,456];

// //     vector_01.push(25);
// //     vector_01.push(27);
// //     vector_01.push(29);

// // // not use this method
// //     // vector_01[0] = 23;
// //     // vector_01[1] = 25;
// //     // vector_01[2] = 27;
// //     // vector_01[3] = 29;

// //     vector_03[0] = 23;
// //     vector_03[1] = 25;
// //     vector_03[2] = 27;
// //     vector_03[3] = 29;

// //     println!("{:?}",vector_01);
// //     println!("{:?}",vector_02);
// //     println!("{:?}",vector_03);
       
// // }


// //vector is a dynamic type but it have a same type of variables




// #![allow(unused_variables)]
// fn main() {
// let mut v = vec![100, 32, 57];
// println!("{:?}",v);
// for i in &mut v {
//     *i *= *i;
//     println!("{}",i);
// }
// println!("{:?}",v);

// //stack fixed and fast and limited strings
// //head dynamic and slow and unlimited strings


// // // dangling  reference or ownership transfer

// // let mut s = String::from("Hello World");

// // let s1 = &mut s;
// // let s2 = &mut s;

// // println!("{}-{}",s1,s2);

// let mut s = String::from("Hello World");

// let s1 = &mut s;
// println!("{:?}",s1);

// let s2 = &mut s;
// println!("{:?}",s2);


// // let s1 = String::from("tic");
// // let s2 = String::from("tac");
// // let s3 = String::from("toe");

// // let s = format!("{} - {} - {}",&s1,&s2,&s3);
// // //let s = s1 + "-" + &s2 + "-" + &s3;  // OK
// // //let s = &s1 + "-" + &s2 + "-" + &s3;  // through error
// // println!("{}",s);

// }
/////////////////////////////////////////////////////////////////////////////////////////////////////////

// Write a program in which you can make a vector and generate its cube without using the dereference method
#![allow(unused_variables)]
fn main() {
let mut v = vec![100, 32, 57];
//println!("{:?}",v);
for &mut i in &mut v {
    
    let i = i * i * i;
    println!("{}",i);
}
println!("{:?}",v);
}


// #![allow(unused_variables)]
// fn main() {
// let mut v = vec![100, 32, 57];
// for index in 0..v.len() {
//     v[index] = v[index]*v[index]*v[index];
// }
// println!("{:?}",v);
// }

///////////////////////////////////////////////////////////////////////////////////////////////////

// [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164,
//224, 165, 135]
//serialized data