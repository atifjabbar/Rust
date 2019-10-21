//////////////////////////////// Task doing by yourself ////////////////////////////////////

// use std::cmp::PartialOrd;
// use std::ops::Add;
// use std::ops::Div;

// fn largest<T: PartialOrd + Copy + Add<Output=T> + Div<Output=T>>(list: &[T]) -> T {
//     let largest = list[0];
//     let mut sum;
//     let count = list.len();
    
    
//     for &item in list.iter() {
//         sum = sum + item;
//     }

//     largest = sum / count;
// }

// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];

//     let result = largest(&number_list);
//     println!("The largest number is {}", result);

//     let float_list = vec![2.1, 4.5, 3.2, 5.5, 5.4];

//     let result = largest(&float_list);
//     println!("The largest char is {}", result);
// }


////////////////////////////////// Task done from the internet////////////////////////////////

// use std::iter::Sum;

// fn main() {
//     let err = "Slice is empty.";

//     let numbers = vec![34, 50, 25, 100, 65];
//     println!("Mean is {:.3}", mean(numbers.iter()).expect(err));

//     let numbers: [i32; 0] = [];
//     match mean(numbers.iter()) {
//         Some(mean_) => println!("Mean is {:.3}", mean_),
//         None => println!(""),
//     }
// }

// fn mean<'a, T, I>(iter: I) -> Option<f64>
// where
//     T: Into<f64> + Sum<&'a T> + 'a,
//     I: Iterator<Item = &'a T>,
// {
//     let mut len = 0;
//     let sum = iter
//         .map(|t| {
//             len += 1;
//             t
//         })
//         .sum::<T>();

//     match len {
//         0 => None,
//         _ => Some(sum.into() / len as f64),
//     }
// }

//////////////////////////////////// Tak done by teacher ///////////////////////////////////////////

// use std::cmp::PartialOrd;
// use std::ops::Add;
// use std::ops::Div;

// fn largest<T: Add<Output=T> + Copy + Clone + PartialOrd + Div<Output=T> + From<i32>>(list: &[T]) -> T
// {       
//     let mut sum = list[0];
//     let count = T::from(list.len()as i32);
    
//     for x in 0..list.len() {        
//         sum = sum + list[x];        
//     }
//     sum/count
// }

// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];

//     let result = largest(&number_list);
//     println!("The largest number is {}", result);

//     let float_list = vec![2.1, 4.5, 3.2, 5.5, 5.4];

//     let result = largest(&float_list);
//     println!("The largest char is {}", result);
// }




