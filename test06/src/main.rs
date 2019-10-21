// fn main() {
//     for number in (1..4).rev() {
//         println!("{}!", number);
//     }
//     println!("LIFTOFF!!!");
// }


// fn main() {
//     let a = [10, 20, 30, 40, 50, 60];

//     for element in a.iter() {
//         println!("the value is: {}", element);
//     }
// }


// fn main() {
//     let a = [10, 20, 30, 40, 50];
//     let mut index = 0;

//     while index < 5 {
//         println!("the value is: {}", a[index]);

//         index += 1;
//     }
// }



// fn main() {       // 1st rule of ownership (only 1 owner of single variable)
//     let x=5;
//     // x is the owner of 5
//     let y=x;
//     // x is blank and now owner of 5 is y
//     println!("{}",y);
// }

/////////////////////////////question 1////////////////
// fn main() {
    
//     another_function(5,5);
// }

// fn another_function(x: i32, y: i32) {
//     let z = x + y;
//     println!("The Total is x + y is: {}", z);
// }

fn main() {
let a = [10, 20, 30, 40, 50, 60, 70];
another_function(&a);
}

fn another_function(x: &[i32]){
    let mut el = 0;
    for element in x.iter() {
        el = el + element;
        println!("the value is: {}", element);
    }
    println!("value: {}",el)
}

/////////////////////////////user input/////////////////////////////
/*use simple_user_input::get_input;

fn main(){
    let input: String = get_input("Please type something...");
    println!("{}",input);
}

mod simple_user_input {
    use std::io;
    pub fn get_input(prompt: &str) -> String{
        println!("{}",prompt);
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_goes_into_input_above) => {},
            Err(_no_updates_is_fine) => {},
        }
        input.trim().to_string()
    }
}*/