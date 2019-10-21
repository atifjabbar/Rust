// fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
//     let mut largest = list[0];

//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];

//     let result = largest(&number_list);
//     println!("The largest number is {}", result);

//     let char_list = vec!['y', 'm', 'a', 'q'];

//     let result = largest(&char_list);
//     println!("The largest char is {}", result);
// }



// #![allow(unused_variables)]
// #[derive(Debug)]
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// impl Message {
//     fn call(&self) {
//         println!("{:?}",&self);
//         // method body would be defined here
//     }
// }

// fn main() {
// let m = Message::Write(String::from("hello"));
// m.call();

// }






// #![allow(unused_variables)]
// #[derive(Debug)]
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// impl Message {
//     fn call(&self) -> &Message{
//         &self
//         // method body would be defined here
//     }
// }

// fn main() {
// let m = Message::Write(String::from("hello"));
// let n = Message::ChangeColor(45,67,54);
// let o = Message::Move{x:45 , y:76};
// // println!("{:?}",m.call());
// // println!("{:?}",n.call());
// // println!("{:?}",o.call());
// let tup = (m,n,o);
// println!("{:?}",tup.1);

// }




// #![allow(unused_variables)]
// #[derive(Debug)]
// enum dayOfWeeks {
//     One,
//     Two,
//     Three,
//     Four,
//     Five,
//     Six,
//     Seven
// }

// fn main() {
//     let today = dayOfWeeks::Six;
//     match today {
//         dayOfWeeks::One => println!("Monday"),
//         dayOfWeeks::Two => println!("Tuesday"),
//         dayOfWeeks::Three => println!("Wednesday"),
//         dayOfWeeks::Four => println!("Thursday"),
//         dayOfWeeks::Five => println!("Friday"),
//         dayOfWeeks::Six => println!("Saturday"),
//         dayOfWeeks::Seven => println!("Sunday"),
//         _=> println!("Invalid Selection")
//     }
// }


// Write a program in which you make a enum with car type (Auto, Manual) and having 
// String value of car vendor

// #![allow(unused_variables)]
// #[derive(Debug)]
// enum carNames {
//     Civic,
//     City,
//     Corolla,
//     Prius,
//     Alto,
//     Swift
// }

// fn main() {
//     let carVendor = carNames::Civic;
//     match carVendor {
//         carNames::Civic => println!("Brand of Civic is HONDA"),
//         carNames::City => println!("Brand of Civic is HONDA"),
//         carNames::Corolla => println!("Brand of Civic is TOYOTA"),
//         carNames::Prius => println!("Brand of Civic is TOYOTA"),
//         carNames::Alto => println!("Brand of Civic is SUZUKI"),
//         carNames::Swift => println!("Brand of Civic is SUZUKI"),
//         _=> println!("Invalid Selection")
//     }
// }





// #![allow(unused_variables)]
// #[derive(Debug)]
// enum carType {
//     Manual,
//     Auto
// }

// fn main() {

// let car = carType::Manual;
// match car {
//         carType::Auto => println!("HONDA"),
//         carType::Manual => println!("TOYOTA"),
//         _=> println!("Invalid Selection")
// }
// }








// #![allow(unused_variables)]
// #[derive(Debug)]
// enum carType {
//     Manual,
//     Auto,
//     Vendor(String),
// }
// impl carType {
//     fn make(&self) -> &carType
//     {
//         &self
//     }
// }
// fn main() {
// let corolla = carType::Vendor(String::from("TOYOTA"));
// println!("{:?}",corolla.make());
// let corolla = carType::Auto;
// match corolla {
//     carType::Auto => println!("Car is Auto"),
//     carType::Manual => println!("Car is Manual"),
//     _=> println!("Car not defined")
// }
// }




// #![allow(unused_variables)]
// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         None => None,
//         Some(i) => Some(i + 1),
//     }
// }
// fn main() {
// let five = Some(5);
// let six = plus_one(five);
// let none = plus_one(None);
// println!("{:?}",five.unwrap());
// println!("{:?}",six.unwrap());
// println!("{:?}",none.unwrap());
// }





// #![allow(unused_variables)]
// fn plus_one(x: Result<i32,i32>) -> Result<i32,i32> {
//     match x {
//         Ok(i) => Ok(i + 1),
//         Err(j) => Err(j),
//     }
// }
// fn main() {

// let five = Ok(5);
// let six = plus_one(five);

// println!("{:?}",five.unwrap());
// println!("{:?}",six.unwrap());

// }