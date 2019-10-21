// #![allow(unused_variables)]

// struct User {
//     username: String,
//     email: String,
//     sign_in_count: u64,
//     active: bool,
// }

// fn build_user(email: String, username: String) -> User {
//     User {
//         email: email,
//         username: username,
//         active: true,
//         sign_in_count: 1,
//     }
// }

// fn main() {

// let fback = build_user(String::from("abc@xyz.com"), String::from("Atif Jabbar"));
// println!("{:?} ,\n{:?} ,\n{:?} ,\n{:?}",fback.email,fback.username,fback.sign_in_count,fback.active);

// }




// #![allow(unused_variables)]

// struct User {
//     username: String,
//     email: String,
//     sign_in_count: u64,
//     active: bool,
// }

// fn build_user(email: String, username: String) -> User {
//     User {
//         email,
//         username,
//         active: true,
//         sign_in_count: 1,
//     }
// }

// fn main() {

// let fback = build_user(String::from("abc@xyz.com"), String::from("Atif Jabbar"));
// println!("{:?} ,\n{:?} ,\n{:?} ,\n{:?}",fback.email,fback.username,fback.sign_in_count,fback.active);

// }



// #![allow(unused_variables)]
// struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);

// fn main() {

// let black = Color(0, 0, 0);
// let origin = Point(0, 0, 0);

// println!("{:?}",black.0);
// }



// fn main() {
//     let width1 = 30;
//     let height1 = 50;

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(width1, height1)
//     );
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }



// fn main() {
//     let rect1 = (30, 50);

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(rect1)
//     );
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }



// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50
//         };

//     println!("The area of the rectangle is {} square pixels.",area(&rect1));
// }

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }



// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let rect1 = Rectangle { width: 30, height: 50 };

//     println!("rect1 is {:#?}", rect1);
// }



// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }

// fn main() {
//     let rect1 = Rectangle { width: 30, height: 50 };

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         rect1.area()
//     );
// }



// #![allow(unused_variables)]
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }

//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }
// }

// fn main() {
//     let rect1 = Rectangle { width: 30, height: 50 };
//     let rect2 = Rectangle { width: 10, height: 40 };
//     let rect3 = Rectangle { width: 60, height: 45 };

//     println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
//     println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
// }




#![allow(unused_variables)]
fn main() {

pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}


let article1 = NewsArticle {
    headline: String::from("Hello World!"),
    location: String::from("Pakistan"),
    author: String::from("Atif Jabbar"),
    content: String::from("hkjhdkjshkjshkdj hsajkhd kjshdk kjshd kasjhd kajhs dkjash dkjahsd kjashd"),
};

let tweet1 = Tweet {
    username: String::from("Abdul Jabbar"),
    content: String::from("yiquwyeiuy 218798127 817239871yeq 9qwue989172 nowqeoquy89ueoiq8eu1uqwo "),
    reply: true,
    retweet: true,
};

let article = NewsArticle {
    headline: String::from("Penguins win the Stanley Cup Championship!"),
    location: String::from("Pittsburgh, PA, USA"),
    author: String::from("Iceburgh"),
    content: String::from("The Pittsburgh Penguins once again are the best
    hockey team in the NHL."),
};

println!("New article available! {:#?}", article.summarize());
println!("{:#?}",article1.summarize());
println!("{:#?}",tweet1.summarize());
}