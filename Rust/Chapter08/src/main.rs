// fn main() {
//     // let mut vector: Vec<i32> = Vec::new();
//     // vector.push(32);
//     // vector.push(52);

//     // let mut v = vec![12,32,453];
//     // v[2] = 40;
//     // println!("{:?}",v);
//     // println!("{}",v[1]);
    
//     // println!("{}",v.get(2).unwrap());

//     // let mut v = vec![1, 2, 3, 4, 5];

//     // let first = v[0];
//     // // println!("The first element is: {}", first);
//     // v.push(6);

//     // println!("The first element is: {:?}", first);
//     // for item in v.iter(){
//     //     println!("{}", item);
//     // }
//     // let mut i = 0;
//     // while i < v.len() {
//     //     println!("{}", v[i]);
//     //     i+=1;
//     // }

//     // let mut v = vec![100, 32, 57];
//     // for i in &mut v {
//     //     *i += 50;
//     // }

//     // let mut v = vec![100, 32, 57];
//     // for i in (0..v.len()){
//     //     v[i] += 50;
//     // }
//     // println!("{:?}",v);
//     // #[derive(Debug)]
//     // enum CarEngineType{
//     //     V6,
//     //     V8,
//     //     V10, 
//     // }
//     // let op = CarEngineType::V6;
//     // match op {
//     //     CarEngineType::V6 => println!("The Car have 6 Pistons"),
//     //     CarEngineType::V8 => println!("The Car have 8 Pistons"),
//     //     CarEngineType::V10 => println!("The Car have 10 Pistons"),
//     // }
//     // #[derive(Debug)]
//     // enum SpreadsheetCell {
//     // Int(i32),
//     // Float(f64),
//     // Text(String),
//     // }

//     // let row = vec![
//     //     SpreadsheetCell::Int(3),
//     //     SpreadsheetCell::Text(String::from("blue")),
//     //     SpreadsheetCell::Float(10.12),
//     // ];
//     // println!("{:?}",row);

//     // let hello = String::from("Здравствуйте");
//     // // for i in hello.chars(){  
//     // //     println!("{}",i);
//     // // }
//     // println!("{:?}", hello.chars());
//     // let len = String::from("Hola").len();
//     // println!("{}", len);
//     // let len = String::from("Здра").len();
//     // println!("{}", len);

//     // use std::collections::HashMap;

//     // let mut scores = HashMap::new();

//     // scores.insert(String::from("Blue"), 10);
//     // scores.insert(String::from("Yellow"), 50);
//     // let mut scores2 = HashMap::new();
//     // scores2.insert(10 ,String::from("Ye Dass Hai"));
//     // println!("{:?}", scores);
//     // println!("{:?}", scores2);

//     // use std::collections::HashMap;
//     // let teams  = vec![
//     // String::from("Blue"), 
//     // String::from("Yellow"),
//     // String::from("Pink"),
//     // String::from("Blue")
//     // ];
    
//     // let initial_scores = vec![10, 50, 30, 30];
//     // let teams  = vec![String::from("Blue"), String::from("Yellow")];
//     // let initial_scores = vec![10, 50];
// // println!("{:?}",[10, 50, 30, 30].iter());
// // println!("{:?}",teams.iter().zip([10, 50, 30, 30].iter()));
// // let scores: HashMap<_, _> = teams.iter().zip([10, 50, 30, 30].iter()).collect();
// // let score = scores.get(&"Blue".to_string());

// //     println!("{:?}", scores);
// //     println!("{:?}", score);

// // use std::collections::HashMap;
// //     let text = "hello world wonderful world peacefull world";

// //     let mut map = HashMap::new();

// //     for word in text.split_whitespace() {
// //         let count = map.entry(word).or_insert(0);
// //         *count += 1;
// //     }

// //     // println!("{:?}", map);
// //     println!("{:?}", text.split_whitespace());

// // *Today's Task*

// // ```Write a program in which you can take Employee details from the user
// //  like Name, Email, age then set into a vector using a tuple. Each tuple 
// //  indicates the single employee. Also, you can create two functions- One 
// //  for creating the employee and second for updating the employee.```    


// }

#[macro_use] 
extern crate text_io;
fn main(){    
    let mut emp_list: Vec<(String,String,i32)> = Vec::new();
    // emp_list[0] = ("faheem".to_string(),"faheem@yahoo.com".to_string(),23);
    // emp_list[1] = ("ibad".to_string(),"ibad@yahoo.com".to_string(),22);
    // emp_list[2] = ("waleed".to_string(),"waleed@yahoo.com".to_string(),24);
    loop{
        println!("Enter the name");
        let mut name:String = read!();
        println!("Enter the email");
        let mut email:String = read!();
        println!("Enter the age");
        let mut age:i32 = read!();
        emp_list.push((name,email,age));
        println!("{:?}", emp_list);   
        println!("You Enter the new record press Y/n");
        let mut flag:String = read!();
        if flag == "n".to_string(){
            break;
        }
        else if flag == "show".to_string(){
            for emp in emp_list.clone(){
                println!("Name: {} Email: {} Age: {}",emp.0,emp.1,emp.2);
            }
        }
    }    
}
